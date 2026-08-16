use super::*;
use crate::util::links;

#[test]
fn about_version_prefers_release_tag() {
    assert_eq!(about_version_text(Some("v0.2026.08.15")), "v0.2026.08.15");
}

#[test]
fn about_version_fallback_is_not_placeholder() {
    let version = about_version_text(None);
    assert_ne!(version, "v#.##.###");
    assert!(
        version.starts_with('v'),
        "display version should keep the leading v: {version}"
    );
    assert!(
        version.contains(env!("CARGO_PKG_VERSION")),
        "untagged builds should include the crate version: {version}"
    );
}

#[test]
fn license_urls_point_at_synth_warp_repo() {
    assert_eq!(
        links::LICENSE_AGPL_URL,
        "https://github.com/synthet/warp/blob/master/LICENSE-AGPL"
    );
    assert_eq!(
        links::LICENSE_MIT_URL,
        "https://github.com/synthet/warp/blob/master/LICENSE-MIT"
    );
}
