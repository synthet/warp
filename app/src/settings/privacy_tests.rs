use settings::schema::SettingSchemaEntry;
use settings::{Setting, SettingSurfaces, SettingsMode};
use warp_core::features::FeatureFlag;

use super::{
    IsCloudConversationStorageEnabled, IsCrashReportingEnabled, IsTelemetryEnabled,
    PrivacySettingsSnapshot,
};

#[test]
fn privacy_settings_apply_to_gui_and_tui() {
    for storage_key in [
        IsTelemetryEnabled::toml_key(),
        IsCrashReportingEnabled::toml_key(),
        IsCloudConversationStorageEnabled::toml_key(),
    ] {
        let entry = inventory::iter::<SettingSchemaEntry>
            .into_iter()
            .find(|entry| entry.storage_key == storage_key)
            .unwrap_or_else(|| panic!("missing schema entry for {storage_key}"));
        let surfaces = (entry.surfaces_fn)();

        assert_eq!(surfaces, SettingSurfaces::ALL, "{storage_key}");
        assert!(surfaces.includes(SettingsMode::Gui), "{storage_key}");
        assert!(surfaces.includes(SettingsMode::Tui), "{storage_key}");
    }
}

#[test]
fn opted_out_disables_telemetry_even_with_agent_mode_analytics() {
    let _guard = FeatureFlag::AgentModeAnalytics.override_enabled(true);
    assert!(PrivacySettingsSnapshot::mock().should_disable_telemetry());
}
