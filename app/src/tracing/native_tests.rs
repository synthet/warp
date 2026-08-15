use super::{endpoint_host_is_loopback, traces_endpoint};
use url::Url;

#[test]
fn traces_endpoint_rejects_remote_https() {
    let err = traces_endpoint("https://example.com").unwrap_err();
    assert!(
        err.to_string().contains("loopback"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn traces_endpoint_rejects_remote_http() {
    assert!(traces_endpoint("http://example.com").is_err());
}

#[test]
fn traces_endpoint_accepts_http_loopback() {
    let endpoint = traces_endpoint("http://127.0.0.1:4318").unwrap();
    assert_eq!(endpoint, "http://127.0.0.1:4318/v1/traces");
}

#[test]
fn traces_endpoint_accepts_localhost() {
    let endpoint = traces_endpoint("http://localhost:4318").unwrap();
    assert_eq!(endpoint, "http://localhost:4318/v1/traces");
}

#[test]
fn traces_endpoint_accepts_https_loopback() {
    let endpoint = traces_endpoint("https://127.0.0.1:4318").unwrap();
    assert_eq!(endpoint, "https://127.0.0.1:4318/v1/traces");
}

#[test]
fn endpoint_host_is_loopback_for_ipv6() {
    let endpoint = Url::parse("http://[::1]:4318").unwrap();
    assert!(endpoint_host_is_loopback(&endpoint));
}
