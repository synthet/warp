use super::*;

#[test]
fn test_parse_valid_app_id() {
    let app_id_string = "com.example.App";
    let app_id = AppId::parse(app_id_string).expect("should not fail to parse");
    assert_eq!(app_id.qualifier(), "com");
    assert_eq!(app_id.organization(), "example");
    assert_eq!(app_id.application_name(), "App");
    assert_eq!(app_id_string, &app_id.to_string());
}

#[test]
fn test_parse_app_id_with_dotted_application_name() {
    let app_id_string = "io.github.synthet.Warp";
    let app_id = AppId::parse(app_id_string).expect("should not fail to parse");
    assert_eq!(app_id.qualifier(), "io");
    assert_eq!(app_id.organization(), "github");
    assert_eq!(app_id.application_name(), "synthet.Warp");
    assert_eq!(app_id_string, &app_id.to_string());

    let local = AppId::parse("io.github.synthet.Warp-Local").expect("should parse");
    assert_eq!(local.application_name(), "synthet.Warp-Local");
    assert_eq!(local.to_string(), "io.github.synthet.Warp-Local");
}

#[test]
fn test_parse_invalid_app_id() {
    assert!(
        AppId::parse("com.example").is_err(),
        "should fail to parse two-part app ID string"
    );
    assert!(
        AppId::parse("com").is_err(),
        "should fail to parse one-part app ID string"
    );
}
