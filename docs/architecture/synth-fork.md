---
type: Technical Reference
title: Synth Warp fork boundary
description: Local-first fork — no billing, Drive, or hosted Warp AI. AGPL app, MIT warpui.
resource: architecture/synth-fork.md
tags: [docs, architecture, fork]
timestamp: 2026-08-16T01:20:00Z
okf_version: 0.1
---

# Synth Warp fork boundary

Product claims live in [README.md](../../README.md) and [FAQ.md](../../FAQ.md). This page is the wiki pointer.

**This fork is:** a commercial-free, local-first Warp client. Terminal and local tooling stay; billing, upgrades, credits, referrals, Warp Drive cloud sync, and hosted Warp-credit AI flows are removed or disabled. Local terminal use does not require a Warp account.

**This fork is not:** Warp’s server, Drive backend, or hosted agent inference. It does not unlock paid Warp cloud services against Warp’s servers.

Licenses: AGPL v3 for the app; MIT for `warpui` / `warpui_core`.

Bundle IDs are `io.github.synthet.Warp` (OSS GUI) and `io.github.synthet.Warp*` for other binaries — not `dev.synth.*` / `synth.dev`. Parse rules: [technical/app-id.md](../technical/app-id.md).

OSS `ChannelState` ships without remote telemetry destinations, crash reporting, or automatic updates.
Incoming channel configuration is sanitized: RudderStack credentials and Sentry configuration are
removed, while an optional local telemetry filename is retained. The process-wide
`TELEMETRY_POLICY` forbids remote export; `ChannelState::telemetry_remote_export_enabled()` delegates
to that policy. Privacy setting defaults are off. In-process recording and optional local log files
can still exist. Overlaying OSS onto official Warp on Windows:
[guides/windows-local-deploy.md](../guides/windows-local-deploy.md).

### Local telemetry transport boundary

Local OTLP is separate from remote telemetry export. When configured for the cloud-agent bridge, it
accepts only `http` or `https` URLs with a literal loopback IP and explicit port. Hostnames (including
`localhost`), URL userinfo, and non-loopback addresses are rejected. Its dedicated HTTP client ignores
environment/system proxies, does not follow redirects, and fails closed if those transport properties
cannot be configured.

RudderStack send/flush paths, Sentry initialization, and the Sentry minidump worker all check the
process policy. These are defense-in-depth guards around the known remote paths; do not describe the
implementation as a complete proof that all future telemetry egress is impossible. That stronger claim
still requires a Sentry no-transport regression test, application-level socket/black-box verification,
and removing or compile-time-gating remote exporters from the local-only service composition.

OSS server URLs use `WarpServerConfig::disabled()` (`http://192.0.2.0:9`, TEST-NET-1 + discard port) so leftover URL interpolation cannot hit Warp production. Logged-out Windows GUI `[WARN]` lines from that sinkhole, missing secure-storage user, and unset personal Drive are expected: [guides/oss-windows-runtime-warnings.md](../guides/oss-windows-runtime-warnings.md).

Shipped behavior summary: [features/implemented/local-first.md](../features/implemented/local-first.md). Hybrid constraints that depend on this boundary: [zed-warp/constraints/synth-fork.md](../zed-warp/constraints/synth-fork.md).

## BYOK and the built-in agent

The built-in Warp Agent has no client-side inference path. BYOK provider keys and custom endpoints are
serialized as *request fields* (`crates/ai/src/api_keys.rs`) and relayed by whatever backend
`ChannelState::server_root_url()` names — the client never calls Anthropic/OpenAI directly. Only the
CLI-agent harness (`app/src/ai/agent_sdk/`) talks to providers locally.

So the fork ungates the UI but does not fabricate a backend:

- `AISettings::is_any_ai_enabled()` no longer requires a Warp login — the credential that matters is
  the user's own provider key, not a warp.dev account.
- `SYNTH_WARP_SERVER_ROOT_URL` (read in `app/src/bin/oss.rs`) points the client at a self-hosted
  MAA-compatible backend. Warp production hosts are rejected; the override is ignored with a message.
- `warp_cloud_enabled_for(Channel::Oss, …)` is `false` for the shipped blackhole root and for
  `*.warp.dev`, and `true` only for a user-supplied root. With no override, the agent still fails at
  send time — by design, since this fork ships no inference server.
- `ChannelState::oz_enabled()` is permanently `false`: Warp-hosted cloud agents (Oz) are a paid
  warp.dev service with no self-hosted equivalent, so `OzConfig::disabled()` is never overridden and
  Oz session types stay hidden.

## When stripping a cloud surface

Keep the client compiling after billing/Drive/hosted-AI removals:

- `app/src/features.rs` `enabled_features()` is a `HashSet<FeatureFlag>`. Omit a variant to leave it off. Never insert `false` (or any non-flag) as a placeholder. See [technical/feature-flags.md](../technical/feature-flags.md).
- If a view no longer implements `TypedActionView`, construct it with `ctx.add_view`, not `ctx.add_typed_action_view`.
- `Appearance::as_ref(ctx)` needs `use warpui::SingletonEntity` in that module (`crate::appearance::Appearance` is a re-export of `warp_core`'s type).
- Onboarding login checks: `AuthStateProvider::as_ref(ctx).get().is_logged_in()` — do not assume a local `is_logged_in` binding exists.
- Unused imports: remove them. Unused bindings/params: prefix `_` (keep lock calls such as `let _model = self.model.lock()`).
- Keep unused cloud types, methods, fields, and variants compiling with `#[allow(dead_code)]` on the unused item (or its `impl` when many associated items are unused together). Do not delete the surface, and do not add crate-level `#![allow(dead_code)]`.
