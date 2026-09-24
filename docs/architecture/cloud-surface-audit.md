---
type: Technical Reference
title: Cloud and commercial surface audit
description: Which Warp-hosted, paid, and vendored surfaces are gated in the OSS build, by which of the four mechanisms, and what is still exposed.
resource: architecture/cloud-surface-audit.md
tags: [docs, architecture, fork, settings]
timestamp: 2026-09-15T00:00:00Z
okf_version: 0.1
---

# Cloud and commercial surface audit

Companion to [synth-fork.md](synth-fork.md), which states the fork *boundary*. This page is the
*inventory*: every Warp-hosted, paid, or vendor-brokered surface found in the client, which
mechanism gates it today, and what is still exposed.

Shipped-behavior summary for users: [features/implemented/local-first.md](../features/implemented/local-first.md).

## The four gating mechanisms

Pick the narrowest one that fits. They are not interchangeable — the first two answer different
questions and the distinction matters.

| # | Mechanism | Where | Answers |
|---|---|---|---|
| 1 | `ChannelState::warp_cloud_enabled()` | [`channel/state.rs`](../../crates/warp_core/src/channel/state.rs) `warp_cloud_enabled_for` | *May this process contact Warp-hosted backends?* **Varies with `server_root_url`** — `false` for the shipped `192.0.2.0:9` sinkhole, `true` for a user-supplied root via `SYNTH_WARP_SERVER_ROOT_URL` |
| 2 | `Channel::offers_warp_hosted_ai()` / `shows_warp_inc_links()` | [`channel/mod.rs`](../../crates/warp_core/src/channel/mod.rs) | *Is this build a Warp Inc. customer?* **Never varies** — always `false` for `Oss`. Self-hosting a backend does not buy you Warp's AI, support, or community |
| 3 | Subtractive flag lists | [`channel/state.rs`](../../crates/warp_core/src/channel/state.rs) `filter_unsupported_features_for` | Strips flags from the runtime set. Runs *last* in `enabled_features()`, so it beats `app/Cargo.toml`'s `default = [...]` without touching it |
| 4 | `SettingsSection::is_hidden_in_local_only_build()` | [`settings_view/mod.rs`](../../app/src/settings_view/mod.rs) | Removes a whole settings page from the sidebar, arrow-key nav, deeplinks, and search |

**Choosing between 1 and 2 is the judgment call.** Hosted *storage or backend reachability* →
mechanism 1. A *commercial relationship with Warp Inc.* (its AI, its credits, its support desk, the
vendors it brokers) → mechanism 2. Getting this backwards means a self-hoster either loses settings
they could use, or is shown settings that bill a company they have no account with.

Mechanism 3's lists: `CLOUD_AGENT_FEATURES` (removed whenever cloud agents are off),
`OSS_CONTROL_PLANE_FEATURES` and `WARP_HOSTED_AI_FEATURES` (removed for `Channel::Oss`).

Below these sits the settings-widget layer: a `Category` disappears automatically once no child
widget renders (`categories_with_visible_content`, `settings_page.rs`), so gating widgets is
sufficient — but gate the `build_page` push too, so no view is constructed for nothing. Gating in
`build_page` is also the more testable shape, since the assertion needs no `AppContext`
(see `privacy_page_tests.rs`, `knowledge_page_tests.rs`).

### Why the settings UI needed gating at all

`Channel::Oss` ships `WarpServerConfig::disabled()`, so the built-in Warp Agent has **no reachable
inference path** — BYOK keys included, since they are serialized as request *fields* and relayed by
whatever `server_root_url` names ([synth-fork.md § BYOK](synth-fork.md#byok-and-the-built-in-agent)).
The runtime was therefore already inert, but the settings pages still advertised the capability,
several rows toggled **on** by default. Inert backend + live UI is the gap this audit closes.

## Gated: settings pages

Mechanism 4 — `is_hidden_in_local_only_build()`:
`BillingAndUsage`, `Referrals`, `Teams`, `WarpDrive`, `CloudEnvironments`, `WarpCloudAgentAPIKeys`.

Note the nav lists in `settings_view/mod.rs` still contain every upstream entry, and every page
handle is still constructed. The predicate does all the work at render time. This is deliberate:
deleting entries conflicts on every upstream sync.

## Gated: Agents → Agent (was "Warp Agent")

Section `Display` is now `"Agent"`. **`slug()` deliberately still returns `"Warp Agent"`** — it is a
stored contract for SQLite session restore and `surface.settings.open --page`. New divergences go in
`SECTIONS_WITH_RENAMED_DISPLAY_LABELS` in `mod_tests.rs`, never by moving a slug.

| Category / widget | Why it goes | Gate |
|---|---|---|
| **Active AI** — Next Command, Prompt Suggestions, Suggested Code Banners, NL Autosuggestions, Shared Block Title Generation, Commit & PR Generation | All six are Warp-hosted inference, gated upstream on the team's `warp_ai_policy` billing tier | 2, via `is_active_ai_offered()` in the six `is_*_toggleable` helpers **and** the `build_page` guard |
| **Voice** | Transcription by **Wispr Flow**, a vendor Warp brokers; also billing-tier gated | 2 |
| **Cloud Handoff** — handoff, auto-handoff on sleep, `&` trigger | Moves a conversation to a Warp-hosted cloud agent | 2 |
| **Warp credit fallback** (inside Custom Inference) | Spends a paid warp.dev credit balance | 2 |
| **SuperGrok row** | Paid xAI subscription via OAuth using a **Warp client id** (`crates/ai/src/grok_subscription/oauth.rs`) | 3 (`SuperGrok`) |
| **AWS Bedrock** | Warp relays the user's AWS CLI credentials; needs a Warp team admin to enable | 2 |
| **Gemini Enterprise** | Warp routes through the workspace's Google Cloud project; team-gated | 2 + 3 (`GeminiEnterprise`) |
| **Experimental** — cloud computer use | Warp-hosted cloud agents | 3 (`AgentModeComputerUse`, `BackgroundComputerUse`) |

**Deliberately kept**, because `Harness::{Oz, Claude, Codex, Gemini, OpenCode}`
(`crates/warp_cli/src/agent.rs`) share one conversation UI — these govern the UI the third-party CLI
agents run in, not Oz:

Input · Other · Agent Attribution · Custom Routers (local files under `WarpConfig`) · BYOK API keys
(`is_byo_api_key_enabled` was deliberately forced `true`; a user's own key is the fork's primary
inference path) · the page-title master AI toggle.

**Behavior, not just UI:** `AISettings::is_active_ai_offered()` gates the six Active-AI getters and
`is_voice_input_enabled`, so a value already in `settings.toml` cannot keep a feature live with no UI
left to turn it off. Hiding a widget without this leaves the setting stuck at its stored value.

### Flags left out of `WARP_HOSTED_AI_FEATURES`, and why

Each is a trap worth remembering:

| Flag | Why not stripped |
|---|---|
| `GitOperationsInCodeReview` | Also gates **local** code-review git UI. Widget hidden instead |
| `CustomModelRouters` | Routers are local files, not hosted inference |
| `UsageBasedPricing` | **Inverted logic** — `agent_profiles_page::build_page` only pushes the Warp-credit `UsageWidget` when this flag is *off*. Removing it would *re-enable* the commercial widget |
| `PredictAMQueries` | Already absent from `enabled_features()` |

## Gated: other pages

- **Knowledge** — "Manage rules" (opens the Warp Drive rule collection, whose page is already
  hidden, so it was a dangling entry point) and "Warp Drive as agent context". Gate 1. Local
  `RulesWidget` / `SuggestedRulesWidget` kept.
- **Profiles** — `UsageWidget` (Warp credit quota, `upgrade_link_for_team`, "Compare plans",
  `mailto:support@warp.dev`, "Restricted due to billing issue"). Gate 2. **Was already unreachable**
  via the `UsageBasedPricing` inversion above; the gate keeps it dead if that flips.
- **Privacy** — `DataManagementWidget` (warp.dev account deletion) already gated on 1;
  `telemetry_docs_url` / `privacy_policy_url` already retargeted via `util/links.rs`.
- **Account** — settings sync, referrals, IAP, and logout widgets already unregistered; anonymous
  branch already reads "This build runs locally without a Warp account".

## Not gated — known residue

Nothing below is a regression; these are surfaces the audit found and did not act on. Roughly
descending by user visibility.

| Surface | Where | Note |
|---|---|---|
| **MCP gallery** | `app/src/ai/mcp/gallery.rs` (`warp_graphql::mcp_gallery_template`), `mcp_servers/list_page.rs` | GraphQL-fed cards grouped under a literal "Shared by Warp and …" header. The local file-based `mcp.json` lane is what works offline. **Most visible remaining item** |
| **MCP cloud objects + team share** | `mcp_servers_page.rs`, `list_page.rs` | Personal/team `CloudMCPServer` objects; share actions require a `team_uid` |
| **Codebase index plan cap** | `code_indexing_page.rs` `CODEBASE_INDEX_LIMIT_REACHED` | "…maximum number of codebase indices **for your plan**." The cap arrives by GraphQL (`max_indices_allowed`); indexing compute itself is local |
| **Account page commercial actions** | `main_page.rs` `MainPageAction::{Upgrade, GenerateStripeBillingPortalLink, ToggleSettingsSync}` | Still registered with handlers calling `UserWorkspaces::upgrade_link*`, though no live widget dispatches them |
| **`TeamUpdateManager` refresh on page open** | `main_page.rs`, `code_indexing_page.rs` | Fires a workspace-metadata network call when the page is selected |
| **Settings-sync keybinding** | `main_page.rs` `init_actions_from_parent_view` | Still registered after the widget was removed |
| **Privacy → Enterprise secret-redaction tab** | `privacy_page.rs` | Fed by `get_enterprise_secret_redaction_regex_list()`; renders only for an enterprise workspace |
| **Shared-session toggles** | `features_page.rs` `ConfirmCloseSharedSessionWidget` | Local pref for a Warp-cloud feature |
| **Warp Drive workflows** | `features_page.rs` "Show Global Workflows in Command Search" | Global workflows come from Warp Drive |
| **Agent profiles team scoping** | `agent_profiles_page.rs` | Profiles resolve through `ResolvedTeamScope`; `ExecutionProfiles` is `SyncToCloud::Globally(Yes)` |
| **`warp.dev` docs links** | `features_page.rs`, `knowledge_page.rs`, `warpify_page.rs`, `warp_agent_page.rs` | Informational only; `util/links.rs` retargets just the privacy and feedback destinations |
| **Typeform feedback link** | `warp_agent_page.rs` "Let us know" → `warpdotdev.typeform.com` | On the kept Input category; goes to a form Warp Inc. reads |

Two structural facts worth knowing before extending this list:

- **`SyncToCloud` scale.** ~253 `Globally`, 13 `PerPlatform`, 116 `Never` across the tree. The
  `RespectUserSyncSetting::No` set — which syncs *regardless* of the user's sync toggle — is small
  and is exactly the sensitive one: `settings_sync_enabled` itself, the four privacy settings, and
  ten AI settings.
- **The "local only" badge is inert.** `LocalOnlyIconState::for_setting` returns `Hidden` whenever
  settings sync is off, which it always is here. Do not read its absence as a signal.

## Conventions when stripping more

Compile-level pitfalls are in [synth-fork.md § When stripping a cloud surface](synth-fork.md#when-stripping-a-cloud-surface).
Beyond those:

- **Do not delete the surface.** Keep it compiling, `#[allow(dead_code)]` only where the compiler
  insists. Deletions conflict on every upstream sync; a predicate does not.
- **Mark every deviation** with a one-line `// Synth Warp is local-first: …` or
  `// Synth Warp is commercial-free: …`. 60+ sites already do.
- **Hoist, do not re-indent.** When adding a conjunct to a long upstream condition, bind the upstream
  expression verbatim and gate the binding. Keeps the merge surface byte-identical.
- **Test positively and negatively** in a `*_tests.rs` sibling — assert what is hidden *and* what
  survives, so a future over-broad gate fails loudly.
