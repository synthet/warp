use futures::executor::block_on;
use warp_core::channel::ChannelState;

use super::get_current_changelog;
use crate::server::server_api::ServerApiProvider;

#[test]
fn get_current_changelog_skips_network_when_warp_cloud_is_disabled() {
    assert!(
        !ChannelState::warp_cloud_enabled(),
        "unit tests run as Channel::Oss with Warp cloud disabled"
    );

    let server_api = ServerApiProvider::new_for_test().get();
    let changelog = block_on(get_current_changelog(server_api))
        .expect("skipping Warp cloud changelog fetch should succeed");
    assert!(changelog.is_none());
}
