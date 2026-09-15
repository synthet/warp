---
type: Implemented Feature
title: Local-first Synth Warp
description: Shipped fork behavior — no Warp account required, no hosted Warp-credit AI, external CLI agents or BYOK.
resource: features/implemented/local-first.md
tags: [docs, features, fork]
timestamp: 2026-08-16T01:20:00Z
okf_version: 0.1
---

# Local-first Synth Warp

User-facing answers: [FAQ.md](../../../FAQ.md). Fork boundary: [architecture/synth-fork.md](../../architecture/synth-fork.md).

Shipped in this fork:

- Local terminal use without signing in
- No Stripe billing, plan upgrades, credit packs, or referral UI
- No Warp Drive cloud sync
- No hosted Warp-credit AI; use external CLI agents (Claude Code, Codex, Gemini CLI, and others). The built-in agent's AI settings are no longer gated on a Warp login, but its BYOK keys are relayed by `server_root_url`, so it only works against a self-hosted backend named by `SYNTH_WARP_SERVER_ROOT_URL` — see [architecture/synth-fork.md](../../architecture/synth-fork.md#byok-and-the-built-in-agent)
- Settings → Agents → **Agent** (renamed from "Warp Agent") drops the Warp-hosted half of the page: Active AI, Voice (Wispr Flow), Cloud Handoff, Warp-credit fallback, SuperGrok, AWS Bedrock, Gemini Enterprise, and cloud computer-use. What remains configures the conversation UI every harness shares, so the third-party CLI agents keep all of it. `Channel::offers_warp_hosted_ai()` is always false for `Oss`, which also strips `SuperGrok`, `GeminiEnterprise`, `AgentModeComputerUse`, `BackgroundComputerUse`, and `SharedBlockTitleGeneration` from the runtime flag set, and the matching `AISettings` getters stay off regardless of any stored value
- Settings → Agents → Knowledge omits "Manage rules" and "Warp Drive as agent context": both read hosted Drive content this build never syncs. Its local rule toggles stay
- No Warp-hosted cloud agents (Oz); those session types stay hidden (`ChannelState::oz_enabled()` is always false)
- Known remote telemetry and crash paths are disabled by the process-wide `TELEMETRY_POLICY`; channel configuration removes RudderStack credentials and Sentry configuration. Optional local log files via `SendTelemetryToFile` write under the log directory. Explicitly configured local OTLP remains available only through a literal loopback IP with an explicit port, redirects disabled, and proxies ignored. Privacy toggles default off. See [architecture/synth-fork.md](../../architecture/synth-fork.md#local-telemetry-transport-boundary) for the enforcement and verification boundary.
- No persisted Warp user in secure storage and no personal Drive. Console `Unable to read user from secure storage: NotFound` and `Unable to create cloud preferences due to unset personal drive` are expected. Expected Windows `[WARN]` catalog: [guides/oss-windows-runtime-warnings.md](../../guides/oss-windows-runtime-warnings.md).
- Settings → Privacy omits Warp account deletion ("Manage your data"); OSS has no hosted Warp account, and `server_root_url` is the TEST-NET discard sink (`http://192.0.2.0:9`)

Warp’s server is not in this repository. A deeper Zed session bridge is planned, not shipped; see [zed-warp/overview.md](../../zed-warp/overview.md).
