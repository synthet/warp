// On Windows, we don't want to display a console window when the application is running in release
// builds. See https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute.
#![cfg_attr(feature = "release_bundle", windows_subsystem = "windows")]

use anyhow::Result;
use warp_core::AppId;
use warp_core::channel::{Channel, ChannelConfig, ChannelState, OzConfig, WarpServerConfig};

// Simple wrapper around warp::run() for Warp OSS builds.
fn main() -> Result<()> {
    let mut state = ChannelState::new(
        Channel::Oss,
        ChannelConfig {
            app_id: AppId::new("io", "github", "synthet.Warp"),
            logfile_name: "warp-oss.log".into(),
            server_config: WarpServerConfig::disabled(),
            oz_config: OzConfig::disabled(),
            telemetry_config: None,
            crash_reporting_config: None,
            autoupdate_config: None,
            mcp_static_config: None,
        },
    );
    if cfg!(debug_assertions) {
        state = state.with_additional_features(warp_core::features::DEBUG_FLAGS);
    }
    ChannelState::set(state);
    apply_server_root_override();

    warp::run()
}

/// Environment variable used to point Synth Warp at a self-hosted agent backend.
///
/// Synth Warp ships no inference server: BYOK provider keys and custom endpoints
/// are relayed by whatever backend `server_root_url` names, so without this the
/// agent has nothing to talk to. Warp production hosts are rejected — this fork
/// does not re-enable paid warp.dev services.
const SERVER_ROOT_URL_ENV: &str = "SYNTH_WARP_SERVER_ROOT_URL";

fn apply_server_root_override() {
    let Ok(url) = std::env::var(SERVER_ROOT_URL_ENV) else {
        return;
    };
    let url = url.trim();
    if url.is_empty() {
        return;
    }
    if warp_core::channel::is_hosted_warp_production_url(url) {
        eprintln!(
            "{SERVER_ROOT_URL_ENV} points at Warp's hosted cloud ({url}); ignoring it. \
             Synth Warp only talks to a self-hosted backend."
        );
        return;
    }
    if let Err(e) = ChannelState::override_server_root_url(url.to_owned()) {
        eprintln!("{SERVER_ROOT_URL_ENV} is not a valid URL ({url}): {e}");
    }
}

// If we're not using an external plist, embed the following as the Info.plist.
#[cfg(all(not(feature = "extern_plist"), target_os = "macos"))]
embed_plist::embed_info_plist_bytes!(r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleDisplayName</key>
    <string>Synth Warp</string>
    <key>CFBundleExecutable</key>
    <string>warp-oss</string>
    <key>CFBundleIdentifier</key>
    <string>io.github.synthet.Warp</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Synth Warp</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>UIDesignRequiresCompatibility</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array><dict><key>CFBundleURLName</key><string>Custom App</string><key>CFBundleURLSchemes</key><array><string>warposs</string></array></dict></array>
    <key>NSHumanReadableCopyright</key>
    <string>© 2026 Synth Warp</string>
    </dict>
    </plist>
"#.as_bytes());
