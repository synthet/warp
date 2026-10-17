use super::{env_flag, web_origin};

#[test]
fn web_origin_maps_websocket_schemes_and_keeps_ports() {
    assert_eq!(
        web_origin("wss://rc.example.com").as_deref(),
        Some("https://rc.example.com")
    );
    assert_eq!(
        web_origin("ws://192.168.1.20:8787/").as_deref(),
        Some("http://192.168.1.20:8787")
    );
    assert_eq!(web_origin("https://rc.example.com"), None);
    assert_eq!(web_origin("not a url"), None);
}

#[test]
fn env_flag_accepts_common_truthy_spellings_only() {
    for value in ["1", "true", "TRUE", " yes ", "on"] {
        assert!(env_flag(Some(value)), "{value:?} should enable");
    }
    for value in ["", "0", "false", "off", "no"] {
        assert!(!env_flag(Some(value)), "{value:?} should not enable");
    }
    assert!(!env_flag(None));
}
