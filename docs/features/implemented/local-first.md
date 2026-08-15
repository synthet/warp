---
type: Implemented Feature
title: Local-first Synth Warp
description: Shipped fork behavior — no Warp account required, no hosted Warp-credit AI, external CLI agents or BYOK.
resource: features/implemented/local-first.md
tags: [docs, features, fork]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# Local-first Synth Warp

User-facing answers: [FAQ.md](../../../FAQ.md). Fork boundary: [architecture/synth-fork.md](../../architecture/synth-fork.md).

Shipped in this fork:

- Local terminal use without signing in
- No Stripe billing, plan upgrades, credit packs, or referral UI
- No Warp Drive cloud sync
- No hosted Warp-credit AI; use external CLI agents (Claude Code, Codex, Gemini CLI, and others) or BYOK where the client supports them
- Telemetry and crash reports are not exported remotely (`telemetry_remote_export_enabled` is always false; OSS ships no telemetry/Sentry config). Optional local log files via `SendTelemetryToFile` write under the log directory. Privacy toggles default off.

Warp’s server is not in this repository. A deeper Zed session bridge is planned, not shipped; see [zed-warp/overview.md](../../zed-warp/overview.md).
