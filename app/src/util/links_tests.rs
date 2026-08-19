use super::*;
use crate::channel::Channel;

#[test]
fn oss_build_does_not_surface_warp_inc_links() {
    assert_eq!(
        ChannelState::channel(),
        Channel::Oss,
        "unit tests run as Channel::Oss"
    );
    assert!(!warp_inc_links_enabled());
}

#[test]
fn oss_privacy_policy_points_at_the_fork_wiki() {
    assert_eq!(privacy_policy_url(), SYNTH_PRIVACY_URL);
    assert!(
        !privacy_policy_url().contains("warp.dev"),
        "OSS must not link users at Warp Inc.'s privacy policy"
    );
}

#[test]
fn oss_has_no_feedback_form() {
    // `synthet/warp` has issues disabled, so there is no destination to retarget to;
    // callers hide their entry point instead.
    assert_eq!(feedback_form_url(), None);
}

#[test]
fn license_urls_point_at_the_fork() {
    assert!(LICENSE_AGPL_URL.starts_with("https://github.com/synthet/warp/blob/master/"));
    assert!(LICENSE_MIT_URL.starts_with("https://github.com/synthet/warp/blob/master/"));
}
