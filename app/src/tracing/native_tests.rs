use url::Url;

use super::{endpoint_host_is_loopback, traces_endpoint};

#[test]
fn traces_endpoint_rejects_remote_https() {
    // Needs an explicit port: the missing-port check runs before the loopback
    // check, so a portless URL is rejected for the wrong reason.
    let err = traces_endpoint("https://example.com:4318").unwrap_err();
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
fn traces_endpoint_rejects_hostname_even_when_named_localhost() {
    let err = traces_endpoint("http://localhost:4318").unwrap_err();

    assert!(
        err.to_string().contains("literal loopback IP"),
        "unexpected error: {err:#}"
    );
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

#[test]
fn traces_endpoint_rejects_userinfo() {
    let err = traces_endpoint("http://user:password@127.0.0.1:4318").unwrap_err();

    assert!(
        err.to_string().contains("userinfo"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn traces_endpoint_rejects_missing_explicit_port() {
    let err = traces_endpoint("http://127.0.0.1").unwrap_err();

    assert!(
        err.to_string().contains("explicit port"),
        "unexpected error: {err:#}"
    );
}
