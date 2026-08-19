use crate::channel::ChannelState;

pub const USER_DOCS_URL: &str = "https://docs.warp.dev/";
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub const GITHUB_ISSUES_URL: &str = "https://github.com/warpdotdev/Warp/issues";
pub const SLACK_URL: &str = "http://go.warp.dev/join-preview";
const WARP_PRIVACY_POLICY_URL: &str = "https://www.warp.dev/privacy";
/// Synth Warp keeps everything on-machine, so Warp Inc.'s hosted privacy policy does
/// not describe this build. The fork's local-first page is the accurate substitute.
pub const SYNTH_PRIVACY_URL: &str =
    "https://github.com/synthet/warp/blob/master/docs/features/implemented/local-first.md";
pub const LICENSE_AGPL_URL: &str = "https://github.com/synthet/warp/blob/master/LICENSE-AGPL";
pub const LICENSE_MIT_URL: &str = "https://github.com/synthet/warp/blob/master/LICENSE-MIT";

/// Whether Warp Inc.'s community and commercial destinations may be surfaced.
/// See [`Channel::shows_warp_inc_links`](crate::channel::Channel::shows_warp_inc_links).
pub fn warp_inc_links_enabled() -> bool {
    ChannelState::channel().shows_warp_inc_links()
}

/// The privacy policy to link from menus and the Privacy settings page.
pub fn privacy_policy_url() -> &'static str {
    if warp_inc_links_enabled() {
        WARP_PRIVACY_POLICY_URL
    } else {
        SYNTH_PRIVACY_URL
    }
}

/// The bug-report form, or `None` when this build has nowhere to send feedback.
///
/// Returns `None` on the fork: the form files issues on `warpdotdev/Warp`, and
/// `synthet/warp` has issues disabled, so there is no destination to retarget to.
/// Callers must hide their entry point rather than opening a fallback URL.
pub fn feedback_form_url() -> Option<String> {
    if !warp_inc_links_enabled() {
        return None;
    }
    let mut url = url::Url::parse("https://github.com/warpdotdev/Warp/issues/new/choose")
        .expect("Should not fail to parse");
    if let Some(version) = ChannelState::app_version() {
        url.query_pairs_mut().append_pair("warp-version", version);
    }
    url.query_pairs_mut()
        .append_pair("os-version", &os_info::get().version().to_string());
    Some(url.to_string())
}

#[cfg(test)]
#[path = "links_tests.rs"]
mod tests;
