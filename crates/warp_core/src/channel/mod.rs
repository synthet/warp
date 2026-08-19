mod config;
mod state;

use std::fmt;

pub use config::*;
pub use state::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    /// The official/first-party stable release.
    Stable,
    /// The official/first-party feature preview release.
    Preview,

    /// The internal-only nightly build.
    Dev,
    /// The internal-only HEAD build.
    Local,

    /// The open-source build of Warp.
    Oss,

    /// The integration test build.
    Integration,
}

impl Channel {
    /// Whether or not this channel is for internal use only
    pub fn is_dogfood(&self) -> bool {
        match self {
            Channel::Dev | Channel::Local => true,
            Channel::Stable | Channel::Preview | Channel::Integration | Channel::Oss => false,
        }
    }

    /// Whether this channel honors the `--server-root-url` / `--ws-server-url` /
    /// `--session-sharing-server-url` flags (and their `WARP_*` env-var equivalents).
    ///
    /// Release channels (`Stable`, `Preview`, `Oss`) ignore these overrides so shipped
    /// builds can't be redirected away from their baked-in server URLs. Internal-only channels
    /// (`Dev`, `Local`, `Integration`) continue to honor them for local development and testing.
    pub fn allows_server_url_overrides(&self) -> bool {
        match self {
            Channel::Dev | Channel::Local | Channel::Integration => true,
            Channel::Stable | Channel::Preview | Channel::Oss => false,
        }
    }

    /// Returns the CLI command name corresponding to this channel.
    pub fn cli_command_name(&self) -> &'static str {
        match self {
            Channel::Stable => "oz",
            Channel::Dev => "oz-dev",
            Channel::Preview => "oz-preview",
            Channel::Local => "oz-local",
            Channel::Integration => "oz-integration",
            Channel::Oss => "warp-oss",
        }
    }

    /// Whether this channel may point users at Warp Inc.'s own community, support,
    /// and commercial destinations (the upstream issue tracker, the Warp Slack,
    /// `support@warp.dev`, contact-sales, `warp.dev/privacy`).
    ///
    /// False for `Oss`: Synth Warp is an independent fork that did not ship those
    /// binaries and cannot service that traffic, so linking to them misdirects users
    /// to a vendor who will not answer.
    ///
    /// This is deliberately *not* [`ChannelState::warp_cloud_enabled`], which asks
    /// whether Warp-hosted backends may be contacted and therefore varies with
    /// `server_root_url`. Pointing an OSS build at your own backend does not make you
    /// an upstream Warp customer, so this answer must not depend on that URL.
    pub fn shows_warp_inc_links(&self) -> bool {
        match self {
            Channel::Stable | Channel::Preview | Channel::Dev | Channel::Local => true,
            // Integration keeps the upstream surface so GUI integration tests continue
            // to exercise the same menus and actions as a first-party build.
            Channel::Integration => true,
            Channel::Oss => false,
        }
    }

    /// Returns the Warp Control CLI command name corresponding to this channel.
    pub fn warpctrl_command_name(&self) -> &'static str {
        match self {
            Channel::Stable => "warpctrl",
            Channel::Dev => "warpctrl-dev",
            Channel::Preview => "warpctrl-preview",
            Channel::Local => "warpctrl-local",
            Channel::Integration => "warpctrl-integration",
            Channel::Oss => "warpctrl-oss",
        }
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Channel::Stable => "stable",
            Channel::Preview => "preview",
            Channel::Dev => "dev",
            Channel::Integration => "integration",
            Channel::Local => "local",
            Channel::Oss => "warp-oss",
        })
    }
}
