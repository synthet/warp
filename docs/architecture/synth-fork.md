---
type: Technical Reference
title: Synth Warp fork boundary
description: Local-first fork — no billing, Drive, or hosted Warp AI. AGPL app, MIT warpui.
resource: architecture/synth-fork.md
tags: [docs, architecture, fork]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# Synth Warp fork boundary

Product claims live in [README.md](../../README.md) and [FAQ.md](../../FAQ.md). This page is the wiki pointer.

**This fork is:** a commercial-free, local-first Warp client. Terminal and local tooling stay; billing, upgrades, credits, referrals, Warp Drive cloud sync, and hosted Warp-credit AI flows are removed or disabled. Local terminal use does not require a Warp account.

**This fork is not:** Warp’s server, Drive backend, or hosted agent inference. It does not unlock paid Warp cloud services against Warp’s servers.

Licenses: AGPL v3 for the app; MIT for `warpui` / `warpui_core`.

Bundle IDs are `io.github.synthet.Warp` (OSS GUI) and `io.github.synthet.Warp*` for other binaries — not `dev.synth.*` / `synth.dev`. Parse rules: [technical/app-id.md](../technical/app-id.md).

OSS `ChannelState` ships `telemetry_config: None`, `crash_reporting_config: None`, and `autoupdate_config: None`. `ChannelState::telemetry_remote_export_enabled()` is always `false` (no Rudderstack/Sentry/OTLP export). Privacy setting defaults are off. In-process recording and optional local log files can still exist. Overlaying OSS onto official Warp on Windows: [guides/windows-local-deploy.md](../guides/windows-local-deploy.md).

Shipped behavior summary: [features/implemented/local-first.md](../features/implemented/local-first.md). Hybrid constraints that depend on this boundary: [zed-warp/constraints/synth-fork.md](../zed-warp/constraints/synth-fork.md).

## When stripping a cloud surface

Keep the client compiling after billing/Drive/hosted-AI removals:

- `app/src/features.rs` `enabled_features()` is a `HashSet<FeatureFlag>`. Omit a variant to leave it off. Never insert `false` (or any non-flag) as a placeholder. See [technical/feature-flags.md](../technical/feature-flags.md).
- If a view no longer implements `TypedActionView`, construct it with `ctx.add_view`, not `ctx.add_typed_action_view`.
- `Appearance::as_ref(ctx)` needs `use warpui::SingletonEntity` in that module (`crate::appearance::Appearance` is a re-export of `warp_core`'s type).
- Onboarding login checks: `AuthStateProvider::as_ref(ctx).get().is_logged_in()` — do not assume a local `is_logged_in` binding exists.
