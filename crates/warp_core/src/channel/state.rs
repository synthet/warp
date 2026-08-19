use std::borrow::Cow;
use std::collections::HashSet;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use url::{Origin, ParseError, Url};

use super::Channel;
use crate::AppId;
use crate::channel::config::{
    ChannelConfig, IapConfig, McpOAuthProviderConfig, OzConfig, RudderStackDestination,
    WarpServerConfig, is_disabled_root_url,
};
use crate::features::FeatureFlag;
use crate::telemetry::TELEMETRY_POLICY;

lazy_static! {
    static ref CHANNEL_STATE: Mutex<ChannelState> = Mutex::new(ChannelState::init());
}

#[cfg(feature = "test-util")]
lazy_static! {
    static ref MOCK_SERVER: Mutex<mockito::ServerGuard> = Mutex::new(mockito::Server::new());
    static ref MOCK_SERVER_URL: String = MOCK_SERVER.lock().url();
    static ref APP_VERSION: Mutex<Option<&'static str>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct ChannelState {
    channel: Channel,

    /// The set of additional features to enable (on top of default-enabled ones).
    additional_features: HashSet<FeatureFlag>,

    config: ChannelConfig,
}

impl ChannelState {
    pub fn init() -> Self {
        let channel = Channel::Oss;
        let app_id = AppId::new("io", "github", "synthet.Warp");
        Self {
            channel,
            additional_features: Default::default(),
            config: ChannelConfig {
                app_id,
                logfile_name: "".into(),
                server_config: WarpServerConfig::disabled(),
                oz_config: OzConfig::disabled(),
                telemetry_config: None,
                autoupdate_config: None,
                crash_reporting_config: None,
                mcp_static_config: None,
            },
        }
    }

    /// Returns the server used by test-only URL routing so downstream tests can install mocks.
    #[cfg(feature = "test-util")]
    pub fn mock_server() -> parking_lot::MutexGuard<'static, mockito::ServerGuard> {
        lazy_static::initialize(&MOCK_SERVER_URL);
        MOCK_SERVER.lock()
    }

    pub fn new(channel: Channel, mut config: ChannelConfig) -> Self {
        config = config.without_remote_telemetry();
        if let Some(app_id) = app_id_from_bundle() {
            config.app_id = app_id;
        }
        Self {
            channel,
            additional_features: Default::default(),
            config,
        }
    }

    pub fn with_additional_features(mut self, overrides: &[FeatureFlag]) -> Self {
        self.additional_features.extend(overrides);
        self
    }

    pub fn set(state: ChannelState) {
        *CHANNEL_STATE.lock() = state;
    }

    pub fn is_release_bundle() -> bool {
        cfg!(feature = "release_bundle")
    }

    pub fn enable_debug_features() -> bool {
        cfg!(debug_assertions) || matches!(Self::channel(), Channel::Local | Channel::Dev)
    }

    pub fn override_server_root_url(url: impl Into<Cow<'static, str>>) -> Result<(), ParseError> {
        let url = url.into();
        Url::parse(&url)?;
        CHANNEL_STATE.lock().config.server_config.server_root_url = url;
        Ok(())
    }

    pub fn override_ws_server_url(url: impl Into<Cow<'static, str>>) -> Result<(), ParseError> {
        let url = url.into();
        Url::parse(&url)?;
        CHANNEL_STATE.lock().config.server_config.rtc_server_url = url;
        Ok(())
    }

    pub fn override_session_sharing_server_url(
        url: impl Into<Cow<'static, str>>,
    ) -> Result<(), ParseError> {
        let url = url.into();
        Url::parse(&url)?;
        CHANNEL_STATE
            .lock()
            .config
            .server_config
            .session_sharing_server_url = Some(url);
        Ok(())
    }

    pub fn uses_staging_server() -> bool {
        let Ok(url) = Url::parse(Self::server_root_url().as_ref()) else {
            return false;
        };
        url.host_str() == Some("staging.warp.dev")
    }

    /// Whether this process may contact Warp-hosted backends (`app.warp.dev`,
    /// Drive/RTC, Oz, Warp Firebase). False for OSS/Integration, and for
    /// Local/Dev unless `server_root_url` is a non-production origin.
    pub fn warp_cloud_enabled() -> bool {
        let state = CHANNEL_STATE.lock();
        warp_cloud_enabled_for(
            state.channel,
            state.config.server_config.server_root_url.as_ref(),
        )
    }

    /// Returns the canonical identifier for the application.
    ///
    /// This should not be used for namespacing persisted data - such use cases
    /// should make use of [`Self::data_domain`] instead.
    pub fn app_id() -> AppId {
        CHANNEL_STATE.lock().config.app_id.clone()
    }

    /// Returns a profile name for isolating user data. This should be used to
    /// sandbox how user data is stored.
    ///
    /// This is a debugging tool for isolating development instances of Warp, and is not
    /// supported in release builds.
    pub fn data_profile() -> Option<String> {
        if cfg!(debug_assertions) {
            std::env::var("WARP_DATA_PROFILE").ok()
        } else {
            None
        }
    }

    /// Returns a value that should be used for namespacing persisted data.
    ///
    /// In release builds, this is identical to the app ID; in debug builds,
    /// it optionally includes a suffix derived from the `WARP_DATA_PROFILE`
    /// environment variable.
    pub fn data_domain() -> String {
        match Self::data_profile() {
            Some(profile) => format!("{}-{profile}", Self::app_id()),
            None => Self::app_id().to_string(),
        }
    }

    /// Returns the data domain if overridden from the default, otherwise None.
    pub fn data_domain_if_not_default() -> Option<String> {
        Self::data_profile().map(|_| Self::data_domain())
    }

    pub fn additional_features() -> HashSet<FeatureFlag> {
        CHANNEL_STATE
            .lock()
            .additional_features
            .iter()
            .cloned()
            .collect()
    }

    pub fn debug_str() -> String {
        format!("{:?}", *CHANNEL_STATE.lock())
    }

    pub fn logfile_name() -> Cow<'static, str> {
        CHANNEL_STATE.lock().config.logfile_name.clone()
    }

    pub fn telemetry_file_name() -> Cow<'static, str> {
        CHANNEL_STATE
            .lock()
            .config
            .telemetry_config
            .as_ref()
            .map(|tc| tc.telemetry_file_name.clone())
            .unwrap_or_default()
    }

    /// Returns whether this build has a telemetry config and can therefore ship
    /// telemetry events. Builds like OpenWarp intentionally ship with
    /// `telemetry_config: None`, in which case UI that controls telemetry
    /// should be hidden since the toggle has no effect.
    pub fn is_telemetry_available() -> bool {
        CHANNEL_STATE.lock().config.telemetry_config.is_some()
    }

    /// Whether this process may POST usage telemetry or crash reports to a
    /// remote collector (Rudderstack, Sentry, or similar).
    ///
    /// Synth Warp is local-first: this is always `false`. In-process event
    /// recording and optional local file logs remain available.
    pub fn telemetry_remote_export_enabled() -> bool {
        TELEMETRY_POLICY.remote_export_allowed()
    }

    /// Returns whether this build has a crash reporting config and can therefore
    /// ship crash reports. Builds like OpenWarp intentionally ship with
    /// `crash_reporting_config: None`, in which case UI that controls crash
    /// reporting should be hidden since the toggle has no effect.
    pub fn is_crash_reporting_available() -> bool {
        CHANNEL_STATE.lock().config.crash_reporting_config.is_some()
    }

    pub fn releases_base_url() -> Cow<'static, str> {
        CHANNEL_STATE
            .lock()
            .config
            .autoupdate_config
            .as_ref()
            .map(|ac| ac.releases_base_url.clone())
            .unwrap_or_default()
    }

    pub fn firebase_api_key() -> Cow<'static, str> {
        CHANNEL_STATE
            .lock()
            .config
            .server_config
            .firebase_auth_api_key
            .clone()
    }

    pub fn iap_config() -> Option<IapConfig> {
        CHANNEL_STATE.lock().config.server_config.iap_config.clone()
    }

    pub fn ws_server_url() -> Cow<'static, str> {
        CHANNEL_STATE
            .lock()
            .config
            .server_config
            .rtc_server_url
            .clone()
    }

    /// Returns the HTTP(S) root URL for the RTC server. Used for HTTP endpoints
    /// served by warp-server-rtc (e.g. the agent event SSE stream).
    ///
    /// Derived from [`ws_server_url`] by rewriting the scheme (`wss`→`https`,
    /// `ws`→`http`) and stripping the path. Falls back to [`server_root_url`]
    /// when the WS URL cannot be parsed or uses an unexpected scheme — this
    /// keeps override paths (e.g. `WARP_WS_SERVER_URL=...`) working without a
    /// separate override for the HTTP variant.
    pub fn rtc_http_url() -> Cow<'static, str> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "test-util")] {
                Cow::Owned(MOCK_SERVER_URL.clone())
            } else {
                match derive_http_origin_from_ws_url(&Self::ws_server_url()) {
                    Some(origin) => Cow::Owned(origin),
                    None => Self::server_root_url(),
                }
            }
        }
    }

    pub fn session_sharing_server_url() -> Option<Cow<'static, str>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "test-util")] {
                Some(Cow::Borrowed("fake_session_sharing_url"))
            } else {
                CHANNEL_STATE.lock().config.server_config.session_sharing_server_url.clone()
            }
        }
    }

    pub fn oz_root_url() -> Cow<'static, str> {
        CHANNEL_STATE.lock().config.oz_config.oz_root_url.clone()
    }

    /// Whether Warp-hosted cloud agents (Oz) are reachable. Synth Warp ships
    /// [`OzConfig::disabled`], so this is `false` for OSS: Oz is a paid
    /// warp.dev service with no local equivalent, and unlike BYOK inference it
    /// cannot be repointed at a self-hosted backend.
    pub fn oz_enabled() -> bool {
        let state = CHANNEL_STATE.lock();
        !is_disabled_root_url(state.config.oz_config.oz_root_url.as_ref())
    }

    /// Whether this process may start Warp-hosted cloud agents.
    ///
    /// A self-hosted inference backend does not provide the Oz control plane,
    /// and OSS builds never opt back into Warp-hosted cloud agents.
    pub fn cloud_agents_enabled() -> bool {
        let state = CHANNEL_STATE.lock();
        cloud_agents_enabled_for(
            state.channel,
            state.config.server_config.server_root_url.as_ref(),
            state.config.oz_config.oz_root_url.as_ref(),
        )
    }

    pub fn filter_unsupported_features(flags: &mut HashSet<FeatureFlag>) {
        let state = CHANNEL_STATE.lock();
        filter_unsupported_features_for(
            flags,
            state.channel,
            cloud_agents_enabled_for(
                state.channel,
                state.config.server_config.server_root_url.as_ref(),
                state.config.oz_config.oz_root_url.as_ref(),
            ),
        );
    }

    pub fn server_root_url() -> Cow<'static, str> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "test-util")] {
                Cow::Owned(MOCK_SERVER_URL.clone())
            } else {
                CHANNEL_STATE.lock().config.server_config.server_root_url.clone()
            }
        }
    }

    pub fn workload_audience_url() -> Cow<'static, str> {
        let state = CHANNEL_STATE.lock();
        match &state.config.oz_config.workload_audience_url {
            Some(url) => url.clone(),
            None => {
                drop(state);
                Self::server_root_url()
            }
        }
    }

    // Returns the origin url, with scheme, domain, and ports (if any)
    pub fn server_root_domain() -> Origin {
        Url::parse(&Self::server_root_url())
            .expect("Server root URL should be valid")
            .origin()
    }

    /// Returns the rudderstack destination for all events that don't contain user-generated content.
    pub fn rudderstack_non_ugc_destination() -> RudderStackDestination {
        let state = CHANNEL_STATE.lock();

        state
            .config
            .telemetry_config
            .as_ref()
            .and_then(|tc| tc.rudderstack_config.as_ref())
            .map(|rs| rs.non_ugc_destination())
            .unwrap_or_default()
    }

    /// Returns the rudderstack destination for all events that contain user-generated content.
    pub fn rudderstack_ugc_destination() -> RudderStackDestination {
        let state = CHANNEL_STATE.lock();

        state
            .config
            .telemetry_config
            .as_ref()
            .and_then(|tc| tc.rudderstack_config.as_ref())
            .map(|rs| rs.ugc_destination())
            .unwrap_or_default()
    }

    pub fn channel() -> Channel {
        CHANNEL_STATE.lock().channel
    }

    #[cfg(feature = "test-util")]
    pub fn app_version() -> Option<&'static str> {
        let version = APP_VERSION.lock();

        version.or_else(|| option_env!("GIT_RELEASE_TAG"))
    }

    #[cfg(feature = "test-util")]
    pub fn set_app_version(version: Option<&'static str>) {
        *APP_VERSION.lock() = version;
    }

    #[cfg(not(feature = "test-util"))]
    pub fn app_version() -> Option<&'static str> {
        option_env!("GIT_RELEASE_TAG")
    }

    pub fn sentry_url() -> Cow<'static, str> {
        CHANNEL_STATE
            .lock()
            .config
            .crash_reporting_config
            .as_ref()
            .map(|crc| crc.sentry_url.clone())
            .unwrap_or_default()
    }

    pub fn show_autoupdate_menu_items() -> bool {
        CHANNEL_STATE
            .lock()
            .config
            .autoupdate_config
            .as_ref()
            .map(|ac| ac.show_autoupdate_menu_items)
            .unwrap_or_default()
    }

    /// Returns the MCP OAuth provider config matching the given client ID, if any.
    pub fn mcp_oauth_provider_by_client_id(client_id: &str) -> Option<McpOAuthProviderConfig> {
        CHANNEL_STATE
            .lock()
            .config
            .mcp_static_config
            .as_ref()
            .and_then(|c| c.providers.iter().find(|p| p.client_id == client_id))
            .cloned()
    }

    /// Returns the MCP OAuth provider config matching the given issuer URL, if any.
    pub fn mcp_oauth_provider_by_issuer(issuer: &str) -> Option<McpOAuthProviderConfig> {
        CHANNEL_STATE
            .lock()
            .config
            .mcp_static_config
            .as_ref()
            .and_then(|c| c.providers.iter().find(|p| p.issuer == issuer))
            .cloned()
    }

    pub fn url_scheme() -> &'static str {
        match Self::channel() {
            Channel::Stable => "warp",
            Channel::Preview => "warppreview",
            Channel::Dev => "warpdev",
            // Dummy value--integration tests shouldn't support URL schemes.
            Channel::Integration => "warpintegration",
            Channel::Local => "warplocal",
            Channel::Oss => "warposs",
        }
    }
}

/// Whether Warp-hosted backends should be contacted for this channel and
/// configured server root. See [`ChannelState::warp_cloud_enabled`].
pub fn warp_cloud_enabled_for(channel: Channel, server_root_url: &str) -> bool {
    match channel {
        Channel::Integration => false,
        Channel::Stable | Channel::Preview => true,
        // Synth Warp ships `WarpServerConfig::disabled()`, so OSS stays offline
        // unless the user points the client at their own backend (see
        // `SYNTH_WARP_SERVER_ROOT_URL`). Warp production is never re-enabled.
        Channel::Oss => {
            !is_disabled_root_url(server_root_url)
                && !is_hosted_warp_production_url(server_root_url)
        }
        Channel::Local | Channel::Dev => !is_hosted_warp_production_url(server_root_url),
    }
}

pub fn cloud_agents_enabled_for(
    channel: Channel,
    server_root_url: &str,
    oz_root_url: &str,
) -> bool {
    channel != Channel::Oss
        && warp_cloud_enabled_for(channel, server_root_url)
        && !is_disabled_root_url(oz_root_url)
}

const CLOUD_AGENT_FEATURES: &[FeatureFlag] = &[
    FeatureFlag::AmbientAgentsCommandLine,
    FeatureFlag::AmbientAgentsImageUpload,
    FeatureFlag::ScheduledAmbientAgents,
    FeatureFlag::SyncAmbientPlans,
    FeatureFlag::APIKeyManagement,
    FeatureFlag::CreateEnvironmentSlashCommand,
    FeatureFlag::CloudEnvironments,
    FeatureFlag::CloudRunners,
    FeatureFlag::CloudAgentRunners,
    FeatureFlag::WarpManagedSecrets,
    FeatureFlag::TeamApiKeys,
    FeatureFlag::NamedAgents,
    FeatureFlag::AmbientAgentsRTC,
    FeatureFlag::CloudMode,
    FeatureFlag::CloudModeFromLocalSession,
    FeatureFlag::CloudModeImageContext,
    FeatureFlag::OzPlatformSkills,
    FeatureFlag::OzIdentityFederation,
    FeatureFlag::OzChangelogUpdates,
    FeatureFlag::OzLaunchModal,
    FeatureFlag::OzHandoff,
    FeatureFlag::HandoffLocalCloud,
    FeatureFlag::CloudModeSetupV2,
    FeatureFlag::CloudModeInputV2,
    FeatureFlag::HandoffCloudCloud,
];

const OSS_CONTROL_PLANE_FEATURES: &[FeatureFlag] = &[
    FeatureFlag::AgentSharedSessions,
    FeatureFlag::CloudConversations,
    FeatureFlag::HOARemoteControl,
];

pub fn filter_unsupported_features_for(
    flags: &mut HashSet<FeatureFlag>,
    channel: Channel,
    cloud_agents_enabled: bool,
) {
    if !cloud_agents_enabled {
        for flag in CLOUD_AGENT_FEATURES {
            flags.remove(flag);
        }
    }

    if channel == Channel::Oss {
        for flag in OSS_CONTROL_PLANE_FEATURES {
            flags.remove(flag);
        }
    }
}

/// Whether `url`'s host belongs to Warp's hosted cloud. See [`is_hosted_warp_cloud_host`].
pub fn is_hosted_warp_production_url(url: &str) -> bool {
    Url::parse(url)
        .ok()
        .and_then(|parsed| parsed.host_str().map(is_hosted_warp_cloud_host))
        .unwrap_or(false)
}

/// Hosts that belong to Warp's cloud (app, RTC, Oz, Drive) or Warp Firebase auth.
pub fn is_hosted_warp_cloud_host(host: &str) -> bool {
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    host == "warp.dev"
        || host.ends_with(".warp.dev")
        || host == "identitytoolkit.googleapis.com"
        || host == "securetoken.googleapis.com"
}

/// Derives an HTTP(S) origin URL from a WebSocket URL by rewriting the scheme
/// (`wss`→`https`, `ws`→`http`) and stripping the path, query, and fragment.
/// Returns [`None`] when the input cannot be parsed as a URL or uses a scheme
/// other than `ws` or `wss`.
#[cfg(not(feature = "test-util"))]
fn derive_http_origin_from_ws_url(ws_url: &str) -> Option<String> {
    let url = Url::parse(ws_url).ok()?;
    let http_scheme = match url.scheme() {
        "wss" => "https",
        "ws" => "http",
        _ => return None,
    };
    let host = url.host_str()?;
    let mut origin = format!("{http_scheme}://{host}");
    if let Some(port) = url.port() {
        origin.push_str(&format!(":{port}"));
    }
    Some(origin)
}

fn app_id_from_bundle() -> Option<AppId> {
    // On macOS, attempt to determine the app ID from the containing bundle,
    // falling back to the channel-keyed "default" ID if we cannot retrieve
    // bundle information.
    //
    // We skip this for tests, as the call to `mainBundle` can take 30+ms,
    // which is a significant portion of the total test runtime.
    #[cfg(all(target_os = "macos", not(feature = "test-util")))]
    {
        use objc2_foundation::NSBundle;

        let bundle = NSBundle::mainBundle();
        if let Some(bundle_identifier) = bundle.bundleIdentifier() {
            let app_id = bundle_identifier.to_string();
            if !app_id.is_empty() {
                return Some(
                    AppId::parse(&app_id)
                        .expect("macOS bundle identifier has an unexpected format"),
                );
            }
        }
    }

    None
}

#[cfg(all(test, not(feature = "test-util")))]
#[path = "state_tests.rs"]
mod tests;

#[cfg(test)]
mod remote_export_tests {
    use super::ChannelState;

    #[test]
    fn telemetry_remote_export_is_disabled() {
        assert!(!ChannelState::telemetry_remote_export_enabled());
    }
}
