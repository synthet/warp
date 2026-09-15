use warp_core::channel::ChannelState;

use super::super::settings_page::FilteredPageType;
use super::*;

fn knowledge_widget_ids() -> Vec<&'static str> {
    let page = KnowledgePageView::build_page();
    let FilteredPageType::Uncategorized { widgets, .. } = page.get_filtered() else {
        panic!("expected uncategorized Knowledge page");
    };
    widgets.iter().map(|widget| widget.widget_id()).collect()
}

#[test]
fn warp_drive_widgets_are_omitted_when_warp_cloud_is_disabled() {
    assert!(
        !ChannelState::warp_cloud_enabled(),
        "unit tests run as Channel::Oss with Warp cloud disabled"
    );

    let ids = knowledge_widget_ids();

    assert!(
        !ids.contains(&ManageRulesWidget::static_widget_id()),
        "Manage rules opens the Warp Drive rule collection: {ids:?}"
    );
    assert!(
        !ids.contains(&WarpDriveContextWidget::static_widget_id()),
        "Warp Drive as agent context has no Drive to read: {ids:?}"
    );
}

#[test]
fn local_rule_widgets_survive() {
    // The fork keeps the on-machine halves of this page: only the Warp Drive
    // surfaces go.
    if !FeatureFlag::AIRules.is_enabled() {
        return;
    }

    let ids = knowledge_widget_ids();

    assert!(ids.contains(&RulesWidget::static_widget_id()), "{ids:?}");
}
