---
type: Feature Spec
title: Conversation-derived Synth Warp backlog
description: Itemized open work extracted from Cursor conversations (9–15 Aug 2026). GitHub issues are disabled on synthet/warp; this page is the working queue from that snapshot.
resource: planning/conversation-backlog.md
tags: [docs, planning, backlog, fork]
timestamp: 2026-08-16T04:28:00Z
okf_version: 0.1
---

# Conversation-derived Synth Warp backlog

Snapshot of 15 Aug 2026 from 26 parent Cursor chats in this checkout, plus a Claude Code transcript of the repeating setup-wizard bug. GitHub issues are **disabled** on `synthet/warp`; do not invent issues. Hybrid build order stays in [zed-warp/feasibility.md](../zed-warp/feasibility.md) — this page does not replace it.

Status key: **Open**, **In working tree**, **Verify**, **Done**.

## P0 — Finish and ship what is already on disk

### 1. Commit the uncommitted local-first polish — In working tree

Working tree continues `12fb942` (`feat(synth): finish local-first OSS client without Warp cloud`). Bundle as one commit after format, clippy, and narrow tests. Do not include untracked `cargo-check-synth.log` or the setup-wizard transcript (local paths).

Already in the diff:

- BYOK ungating: [app/src/settings/ai.rs](../../app/src/settings/ai.rs) no longer requires a Warp login for `is_any_ai_enabled()`; Oz stays gated
- Privacy `ERR_UNSAFE_PORT`: hide Data Management when `!warp_cloud_enabled()`
- About license 404: URLs in [app/src/util/links.rs](../../app/src/util/links.rs) use `blob/master/LICENSE-AGPL` and `LICENSE-MIT` (default branch is `master`, not `main`)
- About version: fallback is `v` + `CARGO_PKG_VERSION`, not `v#.##.###`
- Telemetry / crash / tracing tests and docs in [crates/warp_core](../../crates/warp_core/src/telemetry.rs), [app/src/tracing/](../../app/src/tracing/), [architecture/synth-fork.md](../architecture/synth-fork.md)

**Verify:** `./script/format`; clippy on touched crates; `cargo nextest run -p warp_core -p warp` for `about_page_tests`, `privacy_page_tests`, `native_tests`, `cloud_agent_auth_tests`, `state_tests`.

### 2. Verify the repeating setup wizard — Verify

The fix is already in HEAD: [app/src/root_view.rs](../../app/src/root_view.rs) (`should_mark_onboarding_completed_now`) and [app/src/root_view_tests.rs](../../app/src/root_view_tests.rs). Completing the wizard never wrote `HasCompletedOnboarding` because account-first onboarding skipped the login slide.

Still needed (manual):

- Rebuild/reinstall the binary actually launched (`warp-oss` or `C:\Program Files\Warp\warp.exe`)
- Optional unblock without rebuild: set `HKCU\Software\Warp.dev\synthet.Warp\HasCompletedOnboarding` = `true`
- Fresh-profile check: delete that value, complete the wizard (do not skip), confirm the registry value, quit, relaunch with no wizard
- Confirm Skip still stays skipped

### 3. Close or ignore PR #1 — Open

[synthet/warp#1](https://github.com/synthet/warp/pull/1) is an empty “Update 08/15/2026” template against `warpdotdev:master`. Not Synth work. Close it or leave it; do not merge.

## P1 — User-facing leftover cloud chrome

### 4. Warp Agent settings page: hide vs keep-for-BYOK — Open

The header switch fired `ToggleGlobalAI` but sub-toggles stayed grey because AI required a Warp login. That login gate is already removed in the working tree. Remaining product choice:

- **Keep the page** for BYOK/API keys (current [FAQ.md](../../FAQ.md) path), but hide hosted-only rows (Teams, Cloud Environments, Oz keys, credit banners, Next Command if they still no-op)
- **Or hide Warp Agent** like Teams, and send users to Third-party CLI agents only

Also: `sync_to_cloud` on `is_any_ai_enabled` still tries Drive and logs `Unable to create cloud preferences due to unset personal drive`. Stop cloud-syncing that setting on OSS.

### 5. Built-in agent still has no local inference — Open

Documented in [FAQ.md](../../FAQ.md) and [architecture/synth-fork.md](../architecture/synth-fork.md): ungating the UI does **not** make Warp Agent work. Keys are relayed through `server_root_url`. Out of the box, send fails. Options (pick one later; do not do all):

- Leave as-is (CLI agents only; BYOK needs `SYNTH_WARP_SERVER_ROOT_URL`)
- LocalBackend phase 1c: `AIClient` methods that still matter → OpenAI / Anthropic / Ollama locally (planned, not started)

### 6. Retarget leftover Warp.dev menu / help links — Open

[app/src/util/links.rs](../../app/src/util/links.rs) still has `docs.warp.dev`, `github.com/warpdotdev/Warp/issues`, Slack preview, and warp.dev privacy. The feedback form still files issues on **upstream**. Sign in/up CTAs were hidden; docs/feedback URLs were not retargeted.

- Point Issues/Feedback at `synthet/warp` **or** hide them (issues are disabled)
- Hide Slack / warp.dev download / contact-sales / `mailto:support@warp.dev` on OSS
- Keep or replace the privacy-policy URL (no Synth privacy page exists yet)

### 7. TUI local-only auth tests were left in progress — Verify

GUI CTAs are done. Confirm TUI login-phase tests still pass (`cargo nextest run -p warp_tui` focused names).

## P2 — In-process LocalBackend

Cursor plan name: `LocalBackend in-process`. From the secure-storage `NotFound` / local sign-in / GraphQL-replacement chats.

**Do not:** localhost GraphQL server, fake Firebase `User`, or set `is_logged_in()`.

### 8. Phase 1 — wire LocalBackend behind existing `*Client` traits — Open

- Add `LocalBackend` (`app/src/local_backend/` or a crate)
- Point OSS [ServerApiProvider](../../app/src/server/server_api.rs) getters at it
- Implement `FactoryClient` on SQLite (first complete store)
- `AuthClient` OSS: fail-closed Firebase/token methods; local onboarding/settings writes
- Tests: Factory roundtrip; OSS provider returns LocalBackend; `is_logged_in()` stays false

### 9. Local identity (DPAPI `LocalProfile`), not Warp `User` — Open

- `LocalIdentity { id, display_name, avatar }`
- Key `LocalProfile` under `%LOCALAPPDATA%\github\synthet.Warp\data\io.github.synthet.Warp-LocalProfile`
- APIs: `has_local_identity()`, `local_user_id()`, `local_display_name()`
- Mint from Windows username if missing
- Demote `Unable to read user from secure storage: NotFound` from INFO to debug

### 10. Quiet leftover GraphQL to `192.0.2.0:9` — Open

Inventory remaining `send_graphql_request` call sites (referral, workspace/billing, team, object/Drive, TUI onboarding, AI, …). Phase 1 stubs Disable traits with `anyhow!("disabled in Synth Warp")` so OSS never hits the sinkhole for wired traits. Full leftover `rg send_graphql_request` is a follow-up.

### 11. Later LocalBackend phases (not Phase 1) — Open

- **1b:** real `ObjectClient` / `BlockClient` on SQLite (local Drive). Copy patterns from [fake_object_client.rs](../../app/src/server/cloud_objects/fake_object_client.rs)
- **1c:** local `AIClient` for BYOK (see item 5)
- **2:** domain types so `warp_graphql` is only a hosted adapter
- **3:** optional `synthd` on a named pipe (Windows) / localhost TCP with a per-launch token. Never `0.0.0.0`

## P3 — Log / compile / runtime hygiene

### 12. Finish unused-code warning quieting — Verify

Mechanical unused-import fixes and `#[allow(dead_code)]` on kept stubs landed; the post-check is still open.

- `cargo check -p warp --bin warp-oss --features gui` when `target\` is free
- Confirm the 15 Aug unused-import / dead_code list is gone

### 13. Cheap overlay `log::warn` hygiene — Open

From [guides/oss-windows-runtime-warnings.md](../guides/oss-windows-runtime-warnings.md). Not product bugs. Demote expected empty-state warnings:

- Share modal without a model
- Native modal with no alert
- `SuggestedAgentModeWorkflowModal` not initialized
- Optional: first-frame `HandleFocusChange`, `=C:` env skip → `debug`

Do **not** “fix” WSL GUID, SQLite WAL 283, NVIDIA Vulkan ranking, Git/K8s chips before cwd, or cloud prefs without Drive.

### 14. Warp crate test-binary compile errors — Verify

The cloud-cutoff session: [app/src/search/command_search/view_tests.rs](../../app/src/search/command_search/view_tests.rs) and [app/src/terminal/input_tests.rs](../../app/src/terminal/input_tests.rs) failed to compile, so some new tests never ran. Confirm whether still broken; if yes, fix enough that `cargo test -p warp --lib` / nextest for those modules can execute.

### 15. Windows overlay: release deploy vs debug overlay — Open

`C:\Program Files\Warp` currently has a **debug** `warp-oss` overlay (console window). When cargo is free, run `.\script\windows\deploy.ps1` (release, features `release_bundle,gui`). Documented mismatch: `build.ps1 -Release` is `--features gui` only. See [guides/windows-local-deploy.md](../guides/windows-local-deploy.md).

## P4 — Docs, wiki, housekeeping

### 16. Copy `docs/zed-warp/` into the Synth Zed checkout — Open

Keep the byte-identical mirror. Sibling path named in docs: `D:\Projects\zed\docs\zed-warp`.

### 17. Persist this backlog + optional session memory — Partial

This page is the persist. Remaining:

- Optionally `/log-session` into `.agent-memory` (`memory.md` is still empty)
- Delete or gitignore local artifacts: `cargo-check-synth.log`, the setup-wizard transcript (local paths)

### 18. Privacy / telemetry copy still points at Warp docs — Open

[privacy_page.rs](../../app/src/settings_view/privacy_page.rs) `TELEMETRY_DOCS_URL` is warp.dev. Replace with the fork wiki ([features/implemented/local-first.md](../features/implemented/local-first.md)) or hide the link.

## P5 — Zed × Warp hybrid

Canonical list: [zed-warp/feasibility.md](../zed-warp/feasibility.md). The wiki exists; **no implementation started**. Do not duplicate that roadmap here.

### Synth Warp checkout

- Add Zed / Zed Preview as a Windows external editor (`app/src/util/file/external_editor/windows.rs`) — Windows file-click is VS Code/Cursor/Windsurf only today
- Register `warposs` URL scheme on Windows
- Enable `warp_control_cli` for OSS; Windows named-pipe `local_control` with ACL
- Add `agent.session.list` / `inspect`, `agent.notification.list`; add `cwd` to `tab.create`

### Synth Zed checkout (not this repo)

- Parse OSC 777 `warp://cli-agent`; map onto Terminal Thread badges (Phase A)
- Declarative session launcher (Phase B)
- Defer blocks (Phase C) and rich TUI input (Phase D)

Do **not** embed WarpUI in GPUI, copy AGPL into Zed, or stand up a third session daemon ([zed-warp/what-not-to-build.md](../zed-warp/what-not-to-build.md)).

## Already done (do not re-open)

- Commercial-free fork: billing/credits/referrals/Drive UI stripped; no forced login (`2aaba07` + later)
- App ID: `io.github.synthet.Warp*` — not `synth.dev`
- Telemetry export: OSS `telemetry_config`/`crash_reporting_config` None; Rudderstack tests assert 0 HTTP
- Cloud phone-home: `warp_cloud_enabled()`; skip FreeAvailableModels + changelog; HTTP block of `*.warp.dev`
- Local-only account UI: Sign in/up/out CTAs no-op; Cloud/Teams hidden
- Framework adopt: synthet-code-framework; rust stack; wiki under `docs/`
- Windows build resume: `script/windows/build.ps1`, Defender notes, don’t pipe `2>&1`
- Secret alert: GitHub secret-scanning #1 closed as **used in tests**
- Commit/push: `12fb942` on `origin/master`
- Runtime-warning wiki: [guides/oss-windows-runtime-warnings.md](../guides/oss-windows-runtime-warnings.md)

## Suggested execution order

1. Commit working-tree polish (item 1)
2. Rebuild + verify wizard (item 2)
3. Close PR #1 (item 3)
4. Warp Agent page decision + stop Drive sync of AI toggle (item 4)
5. LocalBackend Phase 1 + LocalProfile (items 8–10)
6. Link/menu retarget (item 6)
7. Overlay warning hygiene + unused-code verify (items 12–13)
8. Hybrid Level 1 Windows editor + `warposs://` when the daily driver needs it (P5)
