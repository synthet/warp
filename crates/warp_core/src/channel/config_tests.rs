use super::*;
use crate::AppId;

#[test]
fn without_remote_telemetry_clears_destinations() {
    let config = ChannelConfig {
        app_id: AppId::new("io", "github", "synthet.Warp"),
        logfile_name: "warp.log".into(),
        server_config: WarpServerConfig::disabled(),
        oz_config: OzConfig::disabled(),
        telemetry_config: Some(TelemetryConfig {
            telemetry_file_name: "telemetry.json".into(),
            rudderstack_config: Some(RudderStackConfig {
                write_key: "remote-write-key".into(),
                root_url: "https://telemetry.example.invalid".into(),
                ugc_write_key: "remote-ugc-write-key".into(),
            }),
        }),
        autoupdate_config: None,
        crash_reporting_config: Some(CrashReportingConfig {
            sentry_url: "https://example.invalid/dsn".into(),
        }),
        mcp_static_config: None,
    }
    .without_remote_telemetry();

    let telemetry_config = config.telemetry_config.unwrap();
    assert_eq!(telemetry_config.telemetry_file_name, "telemetry.json");
    assert!(telemetry_config.rudderstack_config.is_none());
    assert!(config.crash_reporting_config.is_none());
}
