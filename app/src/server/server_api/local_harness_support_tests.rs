use std::sync::Arc;

use futures::executor::block_on;
use warp_cli::agent::{Harness, HarnessTransport};

use super::{LocalHarnessSupportClient, harness_support_client_for, is_local_conversation_token};
use crate::ai::agent::api::ServerConversationToken;
use crate::channel::ChannelState;
use crate::server::server_api::ServerApi;
use crate::server::server_api::harness_support::HarnessSupportClient;

fn client() -> LocalHarnessSupportClient {
    LocalHarnessSupportClient::new(Arc::new(http_client::Client::new()))
}

#[test]
fn issues_unique_local_conversation_tokens() {
    let client = client();
    let first = block_on(client.create_external_conversation("maa", Harness::Codex)).unwrap();
    let second = block_on(client.create_external_conversation("maa", Harness::Claude)).unwrap();
    assert!(is_local_conversation_token(&first));
    assert!(is_local_conversation_token(&second));
    assert_ne!(first, second);
}

#[test]
fn server_tokens_are_not_local() {
    assert!(!is_local_conversation_token(&ServerConversationToken::new(
        "0f6c1e8a-4c9b-4d3a-9a7e-1a2b3c4d5e6f".to_string()
    )));
}

#[test]
fn accepts_conversation_data_and_status_reports_without_a_server() {
    let client = client();
    let token = block_on(client.create_external_conversation("maa", Harness::Gemini)).unwrap();
    block_on(client.upload_conversation_data(&token, b"snapshot")).unwrap();
    block_on(client.notify_user("progress")).unwrap();
    block_on(client.report_transport(HarnessTransport::Acp, None)).unwrap();
    block_on(client.finish_task(true, "done")).unwrap();
    block_on(client.report_clean_shutdown()).unwrap();
    block_on(client.report_error_shutdown("category".into(), "message".into(), Some(1))).unwrap();
}

#[test]
fn task_bound_operations_fail_instead_of_reaching_the_network() {
    let client = client();
    let token = block_on(client.create_external_conversation("maa", Harness::Codex)).unwrap();
    assert!(block_on(client.get_transcript_upload_target(&token)).is_err());
    assert!(block_on(client.get_block_snapshot_upload_target(&token)).is_err());
    assert!(block_on(client.fetch_transcript()).is_err());
}

#[test]
fn shipped_build_selects_the_local_client() {
    assert!(!ChannelState::warp_cloud_enabled());
    let client = harness_support_client_for(Arc::new(ServerApi::new_for_test()));
    let token = block_on(client.create_external_conversation("maa", Harness::Codex)).unwrap();
    assert!(is_local_conversation_token(&token));
}
