use super::{derive_http_origin_from_ws_url, is_hosted_warp_cloud_host, warp_cloud_enabled_for};
use crate::channel::{Channel, is_disabled_root_url};

/// Mirrors `WarpServerConfig::disabled`'s blackhole root.
const DISABLED_ROOT: &str = "http://192.0.2.0:9";

#[test]
fn wss_becomes_https_and_strips_path() {
    let got = derive_http_origin_from_ws_url("wss://rtc.app.warp.dev/graphql/v2");
    assert_eq!(got.as_deref(), Some("https://rtc.app.warp.dev"));
}

#[test]
fn ws_becomes_http_and_preserves_port() {
    let got = derive_http_origin_from_ws_url("ws://localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn unparseable_input_returns_none() {
    assert!(derive_http_origin_from_ws_url("not a url").is_none());
    assert!(derive_http_origin_from_ws_url("https://app.warp.dev").is_none());
}

#[test]
fn hosted_warp_cloud_hosts_include_app_rtc_oz_and_firebase() {
    assert!(is_hosted_warp_cloud_host("app.warp.dev"));
    assert!(is_hosted_warp_cloud_host("rtc.app.warp.dev"));
    assert!(is_hosted_warp_cloud_host("sessions.app.warp.dev"));
    assert!(is_hosted_warp_cloud_host("oz.warp.dev"));
    assert!(is_hosted_warp_cloud_host("identitytoolkit.googleapis.com"));
    assert!(is_hosted_warp_cloud_host("securetoken.googleapis.com"));
    assert!(is_hosted_warp_cloud_host("APP.WARP.DEV"));
}

#[test]
fn hosted_warp_cloud_hosts_exclude_byok_and_localhost() {
    assert!(!is_hosted_warp_cloud_host("api.openai.com"));
    assert!(!is_hosted_warp_cloud_host("api.anthropic.com"));
    assert!(!is_hosted_warp_cloud_host(
        "generativelanguage.googleapis.com"
    ));
    assert!(!is_hosted_warp_cloud_host("localhost"));
    assert!(!is_hosted_warp_cloud_host("127.0.0.1"));
    assert!(!is_hosted_warp_cloud_host("192.0.2.0"));
}

#[test]
fn warp_cloud_is_always_off_for_integration() {
    assert!(!warp_cloud_enabled_for(
        Channel::Integration,
        "https://app.warp.dev"
    ));
    assert!(!warp_cloud_enabled_for(
        Channel::Integration,
        "http://localhost:8080"
    ));
}

#[test]
fn warp_cloud_for_oss_requires_a_self_hosted_root() {
    // Shipped default: `WarpServerConfig::disabled()`, i.e. no backend at all.
    assert!(!warp_cloud_enabled_for(Channel::Oss, DISABLED_ROOT));
    // Warp production is never re-enabled, even if configured explicitly.
    assert!(!warp_cloud_enabled_for(
        Channel::Oss,
        "https://app.warp.dev"
    ));
    // A user-supplied backend (see `SYNTH_WARP_SERVER_ROOT_URL`) opts back in.
    assert!(warp_cloud_enabled_for(
        Channel::Oss,
        "http://localhost:8080"
    ));
}

#[test]
fn disabled_root_url_matches_only_the_blackhole_root() {
    assert!(is_disabled_root_url(DISABLED_ROOT));
    assert!(!is_disabled_root_url("http://localhost:8080"));
    assert!(!is_disabled_root_url("https://app.warp.dev"));
}

#[test]
fn warp_cloud_stays_on_for_stable_and_preview() {
    assert!(warp_cloud_enabled_for(
        Channel::Stable,
        "https://app.warp.dev"
    ));
    assert!(warp_cloud_enabled_for(
        Channel::Preview,
        "https://app.warp.dev"
    ));
}

#[test]
fn warp_cloud_for_local_and_dev_follows_server_root() {
    assert!(!warp_cloud_enabled_for(
        Channel::Local,
        "https://app.warp.dev"
    ));
    assert!(warp_cloud_enabled_for(
        Channel::Local,
        "http://localhost:8080"
    ));
    assert!(!warp_cloud_enabled_for(
        Channel::Dev,
        "https://app.warp.dev"
    ));
    assert!(warp_cloud_enabled_for(
        Channel::Dev,
        "http://127.0.0.1:8080"
    ));
}
