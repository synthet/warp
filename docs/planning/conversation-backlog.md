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

### 6. Retarget leftover Warp.dev menu / help links — Done

Gate: `Channel::shows_warp_inc_links()` in [channel/mod.rs](../../crates/warp_core/src/channel/mod.rs), false only for `Oss`. Deliberately **not** `warp_cloud_enabled()` — that varies with `server_root_url`, and self-hosting a backend must not re-enable Warp Inc.'s support destinations.

Audit result: most of the surface named above was already dead code, so the live leak was narrower than this item assumed.

- **Live, now fixed:** `/feedback` slash command (opened the upstream new-issue form) is no longer registered on OSS ([static_commands/commands.rs](../../app/src/search/slash_command_menu/static_commands/commands.rs)); `feedback_form_url()` returns `Option` and is `None` on OSS so no caller can fall back to a URL. Privacy-policy link (command palette, app menu, Privacy settings page) now resolves via `privacy_policy_url()`.
- **Already dead, left alone:** `make_new_help_menu` (GitHub Issues + Slack items) and `render_footer_button` (resource-center Docs/Slack/Feedback) are both `#[allow(dead_code)]` and never called. `JoinSlack` / `ViewUserDocs` have no live dispatcher since the fork trimmed `add_overflow_menu_items_as_editable_binding`. `USER_DOCS_URL` / `SLACK_URL` / `GITHUB_ISSUES_URL` are therefore unreachable, not user-visible.
- **Out of scope, verified unreachable:** `mailto:support@warp.dev` and `warp.dev/contact-sales` in [admin_actions.rs](../../app/src/settings_view/admin_actions.rs) are reached only from `teams_page.rs`, and Teams is already hidden on OSS. `warp.dev/download` in [workspace/view.rs](../../app/src/workspace/view.rs) is inside a `#[cfg(target_family = "wasm")]` webapp-only path.
- **Kept:** `docs.warp.dev` still describes the terminal accurately, so `USER_DOCS_URL` was not retargeted.

**Verify:** `cargo test -p warp_core --lib channel` (14 pass); `cargo test -p warp --lib --features gui links` (4 new `util::links::tests` pass); `-- privacy_page slash_command static_commands` (128 pass).

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

### 14. Warp crate test-binary compile errors — Done (no longer reproduces)

Verified 16 Aug 2026: `cargo test -p warp --lib --features gui` builds the whole lib test binary (6180 tests) and both modules are present and running — `search::command_search::view::tests::test_render_view` is listed, and `terminal::input::tests::*` pass in a 128-test run. No fix was needed; whatever broke them was resolved by the commits since.

Only remaining noise is two pre-existing warnings in [request_usage_model_tests.rs](../../app/src/ai/request_usage_model_tests.rs) (unused `warp_graphql::billing` imports and an unused `app` param on a stubbed-out helper) — leftovers from the billing strip, not compile errors. Fold into item 12.

### 15. Windows overlay: release deploy vs debug overlay — Open

`C:\Program Files\Warp` currently has a **debug** `warp-oss` overlay (console window). When cargo is free, run `.\script\windows\deploy.ps1` (release, features `release_bundle,gui`). Documented mismatch: `build.ps1 -Release` is `--features gui` only. See [guides/windows-local-deploy.md](../guides/windows-local-deploy.md).

### 19. Clippy gate on `warp` and `warp_core` — Done

Found 16 Aug 2026 while running clippy for item 6; all four findings were pre-existing on HEAD (confirmed by stashing), none introduced by that work.

- `clippy::nonminimal_bool` **error** in [teams_page.rs](../../app/src/settings_view/teams_page.rs) `should_show_reload_credits_confirmation` — `2aaba072` hard-disabled the check by prepending `false &&`. Collapsed to a plain `false` with a comment; the `ai_request_usage_model` field is still used elsewhere, so nothing was orphaned.
- Unused `warp_graphql::billing` imports and an unused `app` param on the emptied `set_addon_credits_pricing_info` stub in [request_usage_model_tests.rs](../../app/src/ai/request_usage_model_tests.rs) — billing-strip leftovers. Import dropped, param renamed `_app`; the stub is still called by 6 tests so it stays.
- `clippy::let_and_return` in [agent_message_bar.rs](../../app/src/ai/blocklist/agent_view/agent_message_bar.rs) — upstream style, fixed only because `-D warnings` makes it fatal.
- `clippy::items_after_test_module` in [channel/state.rs](../../crates/warp_core/src/channel/state.rs) — moved `app_id_from_bundle` above the two test modules.

**Verify:** `cargo clippy -p warp --lib --features gui --tests -- -D warnings` and `cargo clippy -p warp_core --all-targets --tests -- -D warnings` both exit 0.

Scope caveat: only these two packages were cleared. [`./script/presubmit`](../../script/presubmit) runs `--workspace --all-targets --all-features`, which is far broader and was **not** verified.

### 20. 18 stale test failures in `ai::request_usage_model` — Done

Resolved 17 Aug 2026. `cargo test -p warp --lib --features gui -- request_usage` is now **28 passed, 0 failed** (was 18 of 42 failing).

Verdict per group: the **model is correct, the tests were stale**. The commercial strip hard-wired
`has_any_ai_remaining() -> true`, `has_base_plan_requests_remaining() -> true`, `is_unlimited() -> true`,
and `requests_remaining() -> request_limit().max(1)`, each with a "Synth Warp is commercial-free"
comment. `has_any_ai_remaining` is still called from 11 production sites, so the constant is
deliberate policy, not dead code. No model behavior was changed.

- **2 numeric tests** (`test_request_limit_info`, `..._with_limit`): expectation updated to the full
  limit (200, 999999999) since usage is no longer deducted. `requests_remaining` has no production
  caller but is kept covered.
- **9 `..._false_...` gating tests** deleted. Their hosted-billing setups (`PurchaseAddOnCreditsPolicy`,
  `max_monthly_spend_cents`, `EnterpriseCreditsAutoReloadPolicy`, …) are inert — nothing they configure
  reaches the answer any more, so flipping the assertion would have produced a test that reads as
  though premium auto-reload were still evaluated. Replaced by one guard,
  `test_has_any_ai_remaining_is_not_gated_by_hosted_quota`, asserting the fork invariant (quota spent,
  no bonus, no BYO key, **and** a server `OutOfCredits` denial ⇒ still available).
- **4 server-refinement tests** deleted (`test_server_availability_overrides_locally_derived_state`,
  `test_out_of_credits_refined_by_local_byo_key`, `..._bedrock_credentials`,
  `test_server_unavailable_overrides_local_byo_key`): they exercise `server_availability_permits_ai`
  and `has_any_ai_remaining_before_server_decision`, both now `#[allow(dead_code)]` and uncalled.
- **2 availability-state tests kept**, with only the stale `has_any_ai_remaining` assertions dropped —
  `apply_server_availability` / `reset_server_availability` are live, so their state assertions still
  earn their place. `test_reset_server_availability_restores_prefetch_fallback` renamed to
  `..._clears_the_stored_decision` to match what it now proves.
- **1 test inverted** — this one was *not* a stale number. `test_byo_api_key_disabled_for_anonymous_firebase_user`
  failed on its **first** assertion, not the gating one: `UserWorkspaces::is_byo_api_key_enabled` is
  now hard-`true`, so BYOK works without a Warp account. That is the intended BYOK ungating (item 1),
  so the test became `test_byo_api_key_enabled_for_anonymous_firebase_user`. Its doc comment in
  [user_workspaces.rs](../../app/src/workspaces/user_workspaces.rs) still claimed "Anonymous or
  logged-out users are not allowed to use BYO API keys" while the body returned `true`; corrected.

Orphaned by the deletions and removed: `std::time::SystemTime`, `AwsCredentials`, `AwsCredentialsState`.

**Verify:** `cargo test -p warp --lib --features gui -- request_usage` → 28 passed, 0 failed;
`cargo clippy -p warp --lib --features gui --tests -- -D warnings` exit 0; `cargo fmt -p warp -- --check` clean.

Follow-up not done (deliberately out of scope): **11 `test_has_any_ai_remaining_true_*` tests now pass
vacuously.** Their setups are just as inert as the deleted ones — `..._true_with_payg_enabled` would
pass with no setup at all — so they now imply PAYG matters with no counter-example left to show it
does not. They are green, so removing them was not needed to close this item; decide whether to delete
them or keep the scaffolding for a possible future re-enablement.

### 21. Full `-p warp --lib` suite triage — Partial (23 failures left, all classified)

Found 17 Aug 2026 while closing item 20. Everything before this had only ever been run **filtered**,
which hid the true state. `cargo test -p warp --lib --features gui` (whole suite, ~6146 tests):

| Run | Result |
|-----|--------|
| Baseline (HEAD + working tree) | 44 failed = item 20's 18 + 26 others |
| After item 20 | 29 failed |
| After this pass | **6123 passed, 23 failed** |

Note: item 14 declared `terminal::input::tests::*` green on 16 Aug from a 128-test filtered run — the
full suite shows 4 of them failing. Filtered runs are not evidence about this suite.

#### Fixed this pass (4)

- **`themes::bundled_themes` — a real shipped bug, not a test problem.**
  [pink-city.yaml](../../app/assets/bundled/themes/warp-defaults/pink-city.yaml), added by `12fb942`,
  was the only bundled theme using `details: !custom`. The workspace pins `serde_yaml = "0.8"`, which
  does not read YAML tags for externally-tagged enums — it takes the map's first key as the variant,
  hence `unknown variant main_text_opacity`. The theme would fail to load in the app, so this was never
  only a test failure. Its ten opacities are byte-identical to both `DARKER_DETAILS` and
  `LIGHTER_DETAILS` ([color.rs](../../crates/warp_core/src/ui/theme/color.rs)), so `details: lighter`
  parses, matches every other light theme, and changes no rendered value.
- **`uri::test_settings_section_for_simple_subpage`** — [uri/mod.rs](../../app/src/uri/mod.rs) maps
  `"billing_and_usage" => None` on purpose; the test still expected `Some(SettingsSection::Account)`.
- **`tracing::native::traces_endpoint_rejects_remote_https`** — test bug from the 15 Aug loopback
  hardening: `traces_endpoint("https://example.com")` has no port, so the missing-port check fires
  before the loopback check and the error never contains `"loopback"`. Gave the URL an explicit port.
  Its sibling `..._rejects_remote_http` asserts only `is_err()`, so it passes for the same wrong
  reason — left green, but it is not testing what its name says.
- **`prompt_alert::test_server_out_of_credits_maps_to_request_limit_reached`** — item 20's class, but
  it mattered more than a stale number. `state_from_server_availability` refines `OutOfCredits` /
  `Unknown` / `None` through `has_any_ai_remaining`, which the strip hard-wired to `true`, so the state
  is `NoAlert`. Keeping `RequestLimitReached` would have asserted a paywall: `does_alert_block_ai_requests`
  treats it as blocking, so a dead hosted quota would block local AI. Renamed to `..._maps_to_no_alert`.
  Consequence: `out_of_credits_presentation` and the `RequestLimitReached` arm are now unreachable via
  the server path. Not deleted — decide separately.

#### A. Child-process spawn is blocked by a job object (8) — environment, not fork drift

All 7 `ai::agent_sdk::driver::environment::tests::*` real-git fixtures fail with
`git should be runnable: Os { code: 5, kind: PermissionDenied }`, and
`terminal::local_tty::windows::child::tests::test_event_is_emitted_when_child_exits` fails identically
on `Command::new("cmd.exe").spawn()`.

Root cause: [blocking.rs](../../crates/command/src/blocking.rs) `Command::new` unconditionally ORs
`CREATE_BREAKAWAY_FROM_JOB` on Windows. `CreateProcess` rejects that flag with `ERROR_ACCESS_DENIED`
(os error 5) when the parent is inside a job object lacking `JOB_OBJECT_LIMIT_BREAKAWAY_OK`.

Verified, not assumed: the agent shell reports `IsProcessInJob == True`; the identical git fixture
(bare origin with `uploadpack.allowFilter` + `allowAnySHA1InWant`, a commit hidden under
`refs/pinned/base`, `file://C:\…` clone, then the exact `checkout_command_for` string through `sh -c`)
**succeeds** when run directly; `sh.exe` is on `PATH`. Disabling the tool sandbox does not help.

So these 8 cannot pass under any agent-run `cargo test` here, and will also fail for anyone whose
shell is job-constrained. Decide: skip when spawning is unavailable, or accept them as
"local terminal only". Do not chase them as fork bugs.

#### B. Order-dependent shared globals (2–3) — pass in isolation

`experiments::tests::test_create_experiment_layer_mappings` (wants 2, sees 5) and
`server::telemetry::tests::test_persist_events_doesnt_include_ugc_events` (wants 1, sees 9, with
"More telemetry events in queue than the limit to persist" in the log) both **pass when run alone**.
`EXPERIMENT_LAYER_MAPPINGS` ([experiments/mod.rs](../../app/src/experiments/mod.rs)) is a
`lazy_static` `DashMap`, insert-only and never reset. `test_detect_secrets_no_regexes_configured` is
the same class and fires intermittently depending on the parallel schedule.

Fix the shared state, do not chase them as regressions.

#### C. Deterministic and still untriaged (13)

These reproduce in isolation, so they are genuine behavior mismatches, not pollution:

- `terminal::input::tests::test_history_up{,_buffer_restoration,_multiline,_multiline_vim}` (4) —
  the suggestion list holds 7422/7421 items where the fixture supplies 4/2. **Confirmed
  deterministic**: identical counts across separate full runs, and `test_history_up` fails on its own.
  So it is not the developer's live history bleeding in; some suggestion source is returning a large
  fixed corpus instead of the fixture's history.
- `terminal::view::tests::` `test_insert` (`"One"` vs `"None"`), `test_reinput_blocks` (`"pwd"` vs
  `"sudo pwd"`), `test_insert_into_input`, `test_viewport_iter_most_recent_at_bottom`,
  `test_scroll_position_doesnt_change_when_block_finished` (5). The first two both look like leading
  characters being dropped — check for one shared cause before treating them as five bugs.
- `workspace::view::tests::` `test_terminal_model_isnt_leaked`,
  `test_worktree_sidecar_hides_linked_worktrees_from_repo_list`, and
  `test_open_cloud_agent_setup_guide_action_opens_management_view_and_is_idempotent` (3). The last one
  belongs to the in-flight `show_setup_guide_from_link` cloud-agent gating (item 4), not to this item.
- `workspaces::user_workspaces::user_workspaces_tests::test_member_team_settings_win_over_workspace_settings` (1).

Verify: `cargo test -p warp --lib --features gui` (~75 s of test time after a warm build).

## P4 — Docs, wiki, housekeeping

### 16. Copy `docs/zed-warp/` into the Synth Zed checkout — Open

Keep the byte-identical mirror. Sibling path named in docs: `D:\Projects\zed\docs\zed-warp`.

### 17. Persist this backlog + optional session memory — Partial

This page is the persist. Remaining:

- Optionally `/log-session` into `.agent-memory` (`memory.md` is still empty)
- Delete or gitignore local artifacts: `cargo-check-synth.log`, the setup-wizard transcript (local paths)

### 18. Privacy / telemetry copy still points at Warp docs — Done

[privacy_page.rs](../../app/src/settings_view/privacy_page.rs) `TELEMETRY_DOCS_URL` became `WARP_TELEMETRY_DOCS_URL` plus a `telemetry_docs_url()` that returns `links::SYNTH_PRIVACY_URL` on OSS — the GitHub-hosted [features/implemented/local-first.md](../features/implemented/local-first.md). Same substitution backs `privacy_policy_url()` (item 6), so both Privacy-page links and the app-menu entry agree.

Not changed: the link labels ("Read Warp's privacy policy", "Read more about Warp's use of data") still say Warp, which remains true of this fork.

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

1. ~~Commit working-tree polish (item 1)~~ — landed as `d4369d6`
2. Rebuild + verify wizard (item 2)
3. Close PR #1 (item 3)
4. Warp Agent page decision + stop Drive sync of AI toggle (item 4)
5. LocalBackend Phase 1 + LocalProfile (items 8–10)
6. ~~Link/menu retarget (items 6, 18)~~ — done
7. Finish the full-suite triage (item 21) — 23 left, all classified; group C is the real work
8. Overlay warning hygiene + unused-code verify (items 12–13)
8. Hybrid Level 1 Windows editor + `warposs://` when the daily driver needs it (P5)
