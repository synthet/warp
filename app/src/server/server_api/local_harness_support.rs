//! Synth Warp is local-first: a [`HarnessSupportClient`] for builds with no Warp server.
//!
//! Third-party harnesses (Claude Code, Codex, Gemini, ... over PTY or ACP) run entirely on this
//! machine, but their runners ask warp-server for a conversation id before the first turn and
//! upload transcripts and snapshots as they go. Against the shipped blackhole root those calls
//! fail, and `create_external_conversation` failing aborts the run. This client issues a local
//! conversation token instead, accepts conversation data without storing it remotely, and treats
//! Oz task reporting as a no-op. Runs that need an Oz task (prompt resolution, handoff
//! snapshots, transcript resume) fail with an explicit error rather than reaching the network.
use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use warp_cli::agent::{Harness, HarnessTransport};

use super::ServerApi;
use super::harness_support::{
    CommitSnapshotRequest, CommitSnapshotResponse, HarnessSupportClient, ReportArtifactResponse,
    ResolvePromptRequest, ResolvedHarnessPrompt, SnapshotUploadRequest, UploadTarget,
};
use crate::ai::agent::api::ServerConversationToken;
use crate::ai::artifacts::Artifact;
use crate::channel::ChannelState;

const LOCAL_CONVERSATION_TOKEN_PREFIX: &str = "local-";

/// Whether `token` was issued by [`LocalHarnessSupportClient`] rather than warp-server, so
/// there is no remote conversation to upload transcripts or block snapshots to.
pub(crate) fn is_local_conversation_token(token: &ServerConversationToken) -> bool {
    token.as_str().starts_with(LOCAL_CONVERSATION_TOKEN_PREFIX)
}

/// Returns the harness-support client for this process: warp-server when Warp-hosted backends
/// are reachable, otherwise the local stand-in.
pub(crate) fn harness_support_client_for(
    server_api: Arc<ServerApi>,
) -> Arc<dyn HarnessSupportClient> {
    if ChannelState::warp_cloud_enabled() {
        server_api
    } else {
        Arc::new(LocalHarnessSupportClient::new(server_api.owned_http_client()))
    }
}

fn requires_oz_task(operation: &str) -> anyhow::Error {
    anyhow!("{operation} needs a Warp-hosted agent task, which this local-only build does not have")
}

pub(crate) struct LocalHarnessSupportClient {
    http_client: Arc<http_client::Client>,
}

impl LocalHarnessSupportClient {
    pub(crate) fn new(http_client: Arc<http_client::Client>) -> Self {
        Self { http_client }
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl HarnessSupportClient for LocalHarnessSupportClient {
    async fn create_external_conversation(
        &self,
        _format: &str,
        _harness: Harness,
    ) -> Result<ServerConversationToken> {
        Ok(ServerConversationToken::new(format!(
            "{LOCAL_CONVERSATION_TOKEN_PREFIX}{}",
            uuid::Uuid::new_v4()
        )))
    }

    async fn upload_conversation_data(
        &self,
        _conversation_id: &ServerConversationToken,
        _conversation_data: &[u8],
    ) -> Result<()> {
        // The native conversation is already persisted locally; there is no remote copy.
        Ok(())
    }

    async fn get_transcript_upload_target(
        &self,
        _conversation_id: &ServerConversationToken,
    ) -> Result<UploadTarget> {
        Err(requires_oz_task("Transcript upload"))
    }

    async fn get_block_snapshot_upload_target(
        &self,
        _conversation_id: &ServerConversationToken,
    ) -> Result<UploadTarget> {
        Err(requires_oz_task("Block snapshot upload"))
    }

    async fn resolve_prompt(
        &self,
        _request: ResolvePromptRequest,
    ) -> Result<ResolvedHarnessPrompt> {
        Err(requires_oz_task("Prompt resolution"))
    }

    async fn report_artifact(&self, _artifact: &Artifact) -> Result<ReportArtifactResponse> {
        Err(requires_oz_task("Artifact reporting"))
    }

    async fn notify_user(&self, _message: &str) -> Result<()> {
        Ok(())
    }

    async fn finish_task(&self, _success: bool, _summary: &str) -> Result<()> {
        Ok(())
    }

    async fn report_transport(
        &self,
        _transport: HarnessTransport,
        _fallback_reason: Option<&'static str>,
    ) -> Result<()> {
        Ok(())
    }

    async fn report_clean_shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn report_error_shutdown(
        &self,
        _error_category: String,
        _error_message: String,
        _exit_code: Option<u8>,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_snapshot_upload_targets(
        &self,
        _request: &SnapshotUploadRequest,
    ) -> Result<Vec<UploadTarget>> {
        Err(requires_oz_task("Workspace snapshot upload"))
    }

    async fn commit_snapshot(
        &self,
        _request: &CommitSnapshotRequest,
    ) -> Result<CommitSnapshotResponse> {
        Err(requires_oz_task("Workspace snapshot commit"))
    }

    async fn fetch_transcript(&self) -> Result<bytes::Bytes> {
        Err(requires_oz_task("Transcript resume"))
    }

    fn external_http_client(&self) -> &http_client::Client {
        &self.http_client
    }
}

/// Test-only: points the process at a self-hosted server root so code under test takes the
/// warp-server path, restoring the shipped channel state on drop. Run such tests serially.
#[cfg(test)]
pub(crate) struct SelfHostedServerRootForTest;

#[cfg(test)]
impl SelfHostedServerRootForTest {
    pub(crate) fn new() -> Self {
        ChannelState::override_server_root_url("http://127.0.0.1:9")
            .expect("valid self-hosted server root");
        assert!(ChannelState::warp_cloud_enabled());
        Self
    }
}

#[cfg(test)]
impl Drop for SelfHostedServerRootForTest {
    fn drop(&mut self) {
        ChannelState::set(ChannelState::init());
    }
}

#[cfg(test)]
#[path = "local_harness_support_tests.rs"]
mod tests;
