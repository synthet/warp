use base64::Engine as _;
use chrono::TimeDelta;
use std::ffi::OsString;
use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};

use super::*;

fn jwt_with_payload(payload: serde_json::Value) -> String {
    let encoder = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let header = encoder.encode(br#"{"alg":"none"}"#);
    let payload = encoder.encode(serde_json::to_vec(&payload).unwrap());
    format!("{header}.{payload}.test-signature")
}

fn client_with_expiry(token: &str, expires_at: DateTime<Utc>) -> AuthenticatedHttpClient {
    let (refresh_hint_sender, _) = async_channel::bounded(1);
    AuthenticatedHttpClient {
        inner: reqwest::Client::new(),
        token_store: TokenStore::new(token.to_owned(), expires_at).unwrap(),
        refresh_hint_sender,
    }
}

fn auth_context_with_expiry(token: &str, expires_at: DateTime<Utc>) -> AuthContext {
    let (refresh_hint_sender, refresh_hint_receiver) = async_channel::bounded(1);
    AuthContext {
        token_store: TokenStore::new(token.to_owned(), expires_at).unwrap(),
        expected_run_id: None,
        refresh_hint_sender,
        refresh_hint_receiver: Arc::new(Mutex::new(Some(refresh_hint_receiver))),
    }
}

struct EnvironmentGuard {
    name: &'static str,
    previous: Option<OsString>,
}

impl EnvironmentGuard {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var_os(name);
        unsafe { std::env::set_var(name, value) };
        Self { name, previous }
    }
}

impl Drop for EnvironmentGuard {
    fn drop(&mut self) {
        unsafe {
            match self.previous.take() {
                Some(value) => std::env::set_var(self.name, value),
                None => std::env::remove_var(self.name),
            }
        }
    }
}

#[test]
fn authorization_overwrites_supplied_header() {
    let client = client_with_expiry(
        "current-test-token",
        Utc::now() + TimeDelta::try_minutes(5).unwrap(),
    );
    let mut request = Request::builder()
        .header(AUTHORIZATION, "Bearer stale-test-token")
        .body(Bytes::new())
        .unwrap();

    client.authorize_request(&mut request).unwrap();

    assert_eq!(
        request.headers().get(AUTHORIZATION).unwrap(),
        "Bearer current-test-token"
    );
}

#[test]
fn expired_token_is_refused_and_supplied_header_is_removed() {
    let client = client_with_expiry(
        "expired-test-token",
        Utc::now() - TimeDelta::try_minutes(5).unwrap(),
    );
    let mut request = Request::builder()
        .header(AUTHORIZATION, "Bearer stale-test-token")
        .body(Bytes::new())
        .unwrap();

    assert!(matches!(
        client.authorize_request(&mut request),
        Err(AuthenticatedHttpError::NoValidToken)
    ));
    assert!(!request.headers().contains_key(AUTHORIZATION));
}

#[test]
fn debug_output_redacts_token() {
    let client = client_with_expiry(
        "secret-test-token",
        Utc::now() + TimeDelta::try_minutes(5).unwrap(),
    );

    let debug_output = format!("{client:?}");

    assert!(!debug_output.contains("secret-test-token"));
    assert!(debug_output.contains("expires_at"));
}

#[test]
fn authorized_request_debug_redacts_token() {
    let client = client_with_expiry(
        "secret-request-test-token",
        Utc::now() + TimeDelta::try_minutes(5).unwrap(),
    );
    let mut request = Request::builder().body(Bytes::new()).unwrap();

    client.authorize_request(&mut request).unwrap();
    let request_debug = format!("{request:?}");
    let headers_debug = format!("{:?}", request.headers());

    assert!(!request_debug.contains("secret-request-test-token"));
    assert!(!headers_debug.contains("secret-request-test-token"));
    assert!(request_debug.contains("Sensitive"));
    assert!(headers_debug.contains("Sensitive"));
}

#[test]
fn refreshed_token_run_id_exactly_matches() {
    let token = jwt_with_payload(serde_json::json!({ "run_id": "expected-run-id" }));

    validate_refreshed_token_run_id(&token, Some("expected-run-id")).unwrap();
}

#[test]
fn refreshed_token_run_id_is_required() {
    let token = jwt_with_payload(serde_json::json!({}));
    assert!(validate_refreshed_token_run_id(&token, Some("expected-run-id")).is_err());
}

#[test]
fn expected_run_id_is_required() {
    let token = jwt_with_payload(serde_json::json!({ "run_id": "expected-run-id" }));

    assert!(validate_refreshed_token_run_id(&token, None).is_err());
    assert!(validate_refreshed_token_run_id(&token, Some("")).is_err());
}

#[test]
fn refreshed_token_run_id_must_match() {
    let token = jwt_with_payload(serde_json::json!({ "run_id": "wrong-run-id" }));

    assert!(validate_refreshed_token_run_id(&token, Some("expected-run-id")).is_err());
}

#[test]
fn refreshed_token_run_id_must_be_a_string() {
    let token = jwt_with_payload(serde_json::json!({ "run_id": 123 }));
    assert!(validate_refreshed_token_run_id(&token, Some("expected-run-id")).is_err());
}

#[test]
fn malformed_refreshed_tokens_are_rejected() {
    let invalid_json = {
        let encoder = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let payload = encoder.encode(b"not-json");
        format!("header.{payload}.signature")
    };

    for token in ["not-a-jwt", "header.!!!.signature", &invalid_json] {
        assert!(validate_refreshed_token_run_id(token, Some("expected-run-id")).is_err());
    }
}

#[test]
fn rejected_refreshed_token_preserves_previous_token() {
    let token_store = TokenStore::new(
        "current-test-token".to_owned(),
        Utc::now() + TimeDelta::try_minutes(5).unwrap(),
    )
    .unwrap();
    let wrong_run_token = jwt_with_payload(serde_json::json!({ "run_id": "wrong-run-id" }));

    assert!(
        token_store
            .replace_refreshed(
                wrong_run_token,
                Utc::now() + TimeDelta::try_minutes(5).unwrap(),
                Some("expected-run-id"),
            )
            .is_err()
    );
    assert_eq!(
        token_store.valid_authorization_header().unwrap(),
        "Bearer current-test-token"
    );
}

#[tokio::test]
async fn local_only_http_client_does_not_follow_redirects() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0; 1024];
        let _ = stream.read(&mut request).unwrap();
        stream
            .write_all(
                b"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/collect\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .unwrap();
    });
    let auth_context = auth_context_with_expiry(
        "current-test-token",
        Utc::now() + TimeDelta::try_minutes(5).unwrap(),
    );
    let client = auth_context.http_client().unwrap();
    let request = Request::builder()
        .method("POST")
        .uri(format!("http://{address}/v1/traces"))
        .body(Bytes::new())
        .unwrap();

    let error = client.send_bytes(request).await.unwrap_err();

    assert!(
        error.to_string().contains("HTTP status 302"),
        "unexpected error: {error:#}"
    );
    server.join().unwrap();
}

#[tokio::test]
#[serial_test::serial]
async fn local_only_http_client_ignores_proxy_environment() {
    let destination = TcpListener::bind("127.0.0.1:0").unwrap();
    let destination_address = destination.local_addr().unwrap();
    let destination_server = std::thread::spawn(move || {
        let (mut stream, _) = destination.accept().unwrap();
        let mut request = [0; 1024];
        let _ = stream.read(&mut request).unwrap();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
    });
    let proxy = TcpListener::bind("127.0.0.1:0").unwrap();
    let proxy_address = proxy.local_addr().unwrap();
    let (captured_request_sender, captured_request_receiver) = std::sync::mpsc::channel();
    let proxy_server = std::thread::spawn(move || {
        let (mut stream, _) = proxy.accept().unwrap();
        let mut request = [0; 1024];
        let request_length = stream.read(&mut request).unwrap();
        captured_request_sender
            .send(String::from_utf8_lossy(&request[..request_length]).into_owned())
            .unwrap();
        stream
            .write_all(
                b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .unwrap();
    });
    let proxy_url = format!("http://{proxy_address}");
    let _http_proxy = EnvironmentGuard::set("HTTP_PROXY", &proxy_url);
    let _https_proxy = EnvironmentGuard::set("HTTPS_PROXY", &proxy_url);
    let _all_proxy = EnvironmentGuard::set("ALL_PROXY", &proxy_url);
    let _no_proxy = EnvironmentGuard::set("NO_PROXY", "");
    let auth_context = auth_context_with_expiry(
        "current-test-token",
        Utc::now() + TimeDelta::try_minutes(5).unwrap(),
    );
    let client = auth_context.http_client().unwrap();
    let request = Request::builder()
        .method("POST")
        .uri(format!("http://{destination_address}/v1/traces"))
        .body(Bytes::new())
        .unwrap();

    client.send_bytes(request).await.unwrap();

    assert!(captured_request_receiver.try_recv().is_err());
    let mut unblock = TcpStream::connect(proxy_address).unwrap();
    unblock
        .write_all(b"GET /test-unblock HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .unwrap();
    let captured_request = captured_request_receiver.recv().unwrap();
    assert!(captured_request.starts_with("GET /test-unblock "));
    destination_server.join().unwrap();
    proxy_server.join().unwrap();
}
