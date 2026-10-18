//! Synth Warp is local-first: remote control through a user-run server.
//!
//! Upstream "Remote control" shares a terminal through Warp's hosted (paid) session-sharing
//! service. Synth Warp instead points the same sharer code at `crates/remote_control_server`,
//! which speaks the public session-sharing protocol, authenticates with one shared token rather
//! than a Warp account, and serves its own web/mobile app.
//!
//! Configured through the environment (read by `bin/oss.rs` and here):
//! - `SYNTH_WARP_REMOTE_CONTROL_URL`: the server's websocket origin, e.g. `wss://rc.example.com`.
//! - `SYNTH_WARP_REMOTE_CONTROL_TOKEN`: the server's shared token.
//! - `SYNTH_WARP_REMOTE_CONTROL_ALL_TERMINALS=1`: share every terminal as soon as it starts.
//! - `SYNTH_WARP_REMOTE_CONTROL_HOST_LABEL`: the machine name shown on the dashboard.

use session_sharing_protocol::common::SessionId;
use warp_core::channel::ChannelState;

pub const TOKEN_ENV: &str = "SYNTH_WARP_REMOTE_CONTROL_TOKEN";
pub const ALL_TERMINALS_ENV: &str = "SYNTH_WARP_REMOTE_CONTROL_ALL_TERMINALS";
pub const HOST_LABEL_ENV: &str = "SYNTH_WARP_REMOTE_CONTROL_HOST_LABEL";

/// Header the server uses to label which machine a shared terminal runs on.
const HOST_LABEL_HEADER: &str = "x-synth-host";

/// Whether sharing goes to a self-hosted remote-control server.
pub fn is_enabled() -> bool {
    ChannelState::self_hosted_remote_control_enabled()
}

/// Whether every new terminal should be shared automatically.
pub fn share_all_terminals() -> bool {
    is_enabled() && env_flag(std::env::var(ALL_TERMINALS_ENV).ok().as_deref())
}

fn env_flag(value: Option<&str>) -> bool {
    value.is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

/// Extra headers for sharer and viewer websocket connections. Empty unless a self-hosted server
/// is configured, so the token is never sent anywhere else.
pub fn connect_headers() -> Vec<(&'static str, String)> {
    if !is_enabled() {
        return Vec::new();
    }
    let mut headers = Vec::new();
    if let Some(token) = std::env::var(TOKEN_ENV)
        .ok()
        .map(|token| token.trim().to_owned())
        .filter(|token| !token.is_empty())
    {
        headers.push(("authorization", format!("Bearer {token}")));
    } else {
        log::warn!("{TOKEN_ENV} is not set; the remote-control server will refuse this terminal");
    }
    if let Some(host) = host_label() {
        headers.push((HOST_LABEL_HEADER, host));
    }
    headers
}

fn host_label() -> Option<String> {
    let configured = std::env::var(HOST_LABEL_ENV)
        .ok()
        .filter(|label| !label.trim().is_empty());
    #[cfg(not(target_family = "wasm"))]
    let configured = configured.or_else(sysinfo::System::host_name);
    configured
        // Header values must be visible ASCII.
        .map(|label| {
            label
                .chars()
                .filter(|c| c.is_ascii_graphic() || *c == ' ')
                .collect::<String>()
        })
        .filter(|label| !label.is_empty())
}

/// The web/mobile app URL for a session, or `None` when not self-hosted.
pub fn web_join_link(session_id: &SessionId) -> Option<String> {
    if !is_enabled() {
        return None;
    }
    let ws_url = ChannelState::session_sharing_server_url()?;
    let origin = web_origin(&ws_url)?;
    Some(format!("{origin}/session/{session_id}"))
}

/// Maps the server's websocket origin to the origin its web app is served from.
fn web_origin(ws_url: &str) -> Option<String> {
    let mut url = url::Url::parse(ws_url).ok()?;
    let scheme = match url.scheme() {
        "wss" => "https",
        "ws" => "http",
        _ => return None,
    };
    url.set_scheme(scheme).ok()?;
    Some(url.origin().ascii_serialization())
}

#[cfg(test)]
#[path = "self_hosted_tests.rs"]
mod tests;
