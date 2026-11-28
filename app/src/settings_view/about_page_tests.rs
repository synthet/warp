use super::*;
use warpui::AssetProvider as _;

use crate::ASSETS;
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

#[test]
fn about_page_names_synth_term_and_keeps_upstream_copyright() {
    assert_eq!(PRODUCT_NAME, "Synth Term");
    assert!(COPYRIGHT_NOTICE.contains("Denver Technologies, Inc."));
    assert!(COPYRIGHT_NOTICE.contains("Synth Term contributors"));
}

#[test]
fn about_page_logos_are_bundled_and_not_the_warp_mark() {
    // A path from Warp's two-panel logo.
    const WARP_MARK_PATH: &str = "M170.286 27.5005";

    for path in [LOGO_LIGHT_ON_DARK, LOGO_DARK_ON_LIGHT] {
        let bytes = ASSETS
            .get(path)
            .unwrap_or_else(|err| panic!("{path} should be bundled: {err:?}"));
        let svg = std::str::from_utf8(&bytes).expect("logo should be UTF-8 SVG");
        assert!(
            svg.contains("Synth Term"),
            "{path} should be the Synth Term mark"
        );
        assert!(
            !svg.contains(WARP_MARK_PATH),
            "{path} still draws the Warp mark"
        );
    }
}

#[test]
fn about_headline_matches_the_version_row() {
    assert_eq!(headline_text("v0.3.0"), "Synth Term 0.3.0");
    assert_eq!(headline_text("v0.1.0+081315e7"), "Synth Term 0.1.0");
    assert_eq!(
        headline_text(about_version_text(None)),
        format!("Synth Term {}", env!("CARGO_PKG_VERSION"))
    );
    assert!(UPSTREAM_NOTICE.contains("Not affiliated"));
}
