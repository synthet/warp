use warp_core::channel::ChannelState;

use super::super::settings_page::FilteredPageType;
use super::*;

#[test]
fn data_management_url_is_none_when_warp_cloud_is_disabled() {
    assert!(
        !ChannelState::warp_cloud_enabled(),
        "unit tests run as Channel::Oss with Warp cloud disabled"
    );
    assert_eq!(data_management_url(None), None);
    assert_eq!(data_management_url(Some("token")), None);
}

#[test]
fn data_management_widget_is_omitted_when_warp_cloud_is_disabled() {
    assert!(
        !ChannelState::warp_cloud_enabled(),
        "unit tests run as Channel::Oss with Warp cloud disabled"
    );

    let page = PrivacyPageView::build_page();
    let FilteredPageType::Uncategorized { widgets, .. } = page.get_filtered() else {
        panic!("expected uncategorized Privacy page");
    };

    assert!(
        widgets
            .iter()
            .all(|widget| !widget.search_terms().contains("delete account")),
        "hosted Warp account deletion must not appear when Warp cloud is disabled"
    );
}
