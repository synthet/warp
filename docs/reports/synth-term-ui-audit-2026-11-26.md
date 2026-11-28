---
type: Report
title: Synth Term UI audit (screenshot sweep), 2026-11-26
description: Local-first, cloud/commercial residue, dead settings and naming issues seen in a full GUI screenshot sweep of build v0.1.0+0a2b8580; delta to the 2026-11-19/-21 branding audits and the cloud-surface audit.
resource: reports/synth-term-ui-audit-2026-11-26.md
tags: [docs, reports, branding, local-first, settings]
timestamp: 2026-11-26T07:00:00Z
okf_version: 0.1
---

# Synth Term UI audit (screenshot sweep), 2026-11-26

Build under test: installed `%LOCALAPPDATA%\Programs\SynthTerm\warp-oss.exe`, About shows
`v0.1.0+0a2b8580` (master minus one). Windows 11, 4K, default OSS settings,
`SYNTH_WARP_SERVER_ROOT_URL` and `SYNTH_WARP_REMOTE_CONTROL_URL` unset (User and Machine scope).
81 screenshots were taken with win-gui-mcp; they live outside the repo
(`Pictures/synth-term-screenshots/2026-11-26/`) because several show personal data. They are cited
by file name below (`settings/…` = Settings pages).

This report is a **delta**. Branding strings are inventoried in
[synth-term-branding-audit-2026-11-21.md](synth-term-branding-audit-2026-11-21.md) and gated cloud
surfaces in [cloud-surface-audit.md](../architecture/cloud-surface-audit.md); they are only repeated
here when the screenshots confirm they are still visible. Rename work belongs to the Synth Term rename
epic (warp-iz9u9, zephyr); nothing here is a ticket yet.

Evidence key: **S** = seen in a screenshot; **C** = confirmed in source (`file:line`); **U** = seen
but behavior on click not verified.

## 1. Local-first verdict

The core is local-first: no sign-in (`settings/01-account`: "Synth Term runs locally. No account or
sign-in needed." + "Local" badge), Drive hidden from the tools panel and settings, Billing/Teams/
Referrals pages gone, About and Privacy rebranded. What remains is **UI that advertises capabilities the
default build cannot use**: the built-in agent stack, sharing, update/changelog and the bundled
vendor workflow catalog. None of these was observed sending traffic (the network log console was not
inspected; do that next), but each is a dead end or a mixed message for the user.

## 2. New findings (not in the earlier audits)

### 2.1 Keyboard-shortcut list exposes dead commands — S, C
`settings/10-keyboard-shortcuts-p01…p30` lists every `EditableBinding`, including those whose context
predicate or feature flag can never be true in OSS. Dispatch is gated (e.g. `workspace/mod.rs:861`
`Toggle Warp Drive` requires `flags::ENABLE_WARP_DRIVE`), the **listing is not**. Seen in the list:

| Row | Why dead in OSS |
|---|---|
| Check for Updates; Install Update and Relaunch | `autoupdate_config: None` (`bin/oss.rs`) |
| Left Panel: Warp Drive (Alt+4); Toggle Warp Drive | Drive hidden (warp-cpo45) — **feedback for that ticket** |
| Toggle Team Workflows Modal (Ctrl+Shift+S) | no teams |
| Toggle the Agent Management View (Ctrl+Shift+M) | pressing it does nothing (tried) |
| Share Current Session / Share Pane / Share Selected Block | no sharing server configured (see 2.3) |
| Slash command: /connect-grok, /changelog | SuperGrok stripped; changelog fetch off |
| Accept Prompt Suggestion; Open AI Command Suggestions (Ctrl+`) | Active AI is Warp-hosted |
| Show **Warp** Network Log | naming (the Privacy page already says Synth Term) |

The Keyboard Shortcuts side panel (`08-keyboard-shortcuts-panel`) opens its "Essentials" with
**Open AI Command Suggestions** — the first thing a new user reads is a dead feature.
Fix shape: filter the settings list and the cheat sheet with the same predicate the dispatcher uses
(or a per-binding `is_offered()`), mechanism 3/4 style, no deletions.

### 2.2 Bundled vendor workflow catalog — S, C
Ctrl+Shift+R (`15-workflows`) opens on "All" with commercial vendor recipes ("Add a Google BigQuery
resource to the **Meroxa** Platform", …) plus **Team Workflows** and **Repository Workflows**
categories. Source: `Cargo.toml:350` `warp-workflows = { git = "https://github.com/warpdotdev/workflows" … }`.
This is upstream content, shipped offline, not Drive. **Correction to cloud-surface-audit.md**
("Global workflows come from Warp Drive"): the global set is this bundled crate; the Features toggle
"Show Global Workflows in Command Search (ctrl-r)" (`features_page.rs:7524`, off) controls it, and its
info link goes to docs.warp.dev. Decide: keep as a useful local catalog (then hide "Team Workflows")
or drop vendor categories.

### 2.3 "Share session" offered with no sharing server — S, U, C
Shown in the tab menu (`11a-tab-context-menu-session`), terminal and input context menus
(`12a`, `12b`) and the pane menu (`17a`) while `SYNTH_WARP_REMOTE_CONTROL_URL` is unset.
Gate is `FeatureFlag::CreatingSharedSessions && ContextFlag::CreateSharedSession` (`tab.rs:489`),
not the remote-control URL. This contradicts
[local-first.md](../features/implemented/local-first.md) ("Off unless `SYNTH_WARP_REMOTE_CONTROL_URL`
is set"). Click behavior not tested. Also gate Features → "Confirm before closing shared session".

### 2.4 Built-in agent settings are inert by default — S, C
With no `SYNTH_WARP_SERVER_ROOT_URL`, the built-in agent has no inference path
([synth-fork.md § BYOK](../architecture/synth-fork.md#byok-and-the-built-in-agent)). Its master toggle
is off (`settings/02-agents-agent-p01`), yet these pages render as if usable:

| Page | What is inert | Screens |
|---|---|---|
| Agents → Agent | Custom Inference key fields (keys are relayed to the sink), Custom Routers, Other | 02-agents-agent-p01/p02 |
| Agents → Profiles | "Base model: auto (cost-efficient)", "Full terminal use: auto" are Warp-hosted model aliases | 03-agents-profiles-p01 |
| Agents → MCP servers | servers feed the built-in agent only; the CLI agents read their own configs. Page lists the user's Claude/Codex MCP configs | 04-agents-mcp-servers-p01/p02 |
| Agents → Knowledge | rules feed the built-in agent | 05-agents-knowledge-p01 |
| Code → Indexing | "Warp can automatically index…"; consumer is the built-in agent | 07a-code-indexing-and-projects-p01 |
| Tools panel → Agent conversations | empty state "Enable AI in Settings…" on a default-on tab | 02b-tools-panel-conversations |

Third party CLI agents (`06-agents-third-party-cli-p01`) is the page that works. Proposal: when the
server root is the sinkhole, show the built-in-agent pages under one collapsed "Built-in agent
(requires a self-hosted backend)" group with a single explanatory line, and default the
Agent-conversations tools tab off.

### 2.5 Other dead or misleading rows — S
- Features → "Show changelog toast after updates" (`features_page.rs:5005`, `ToggleState::Enabled`)
  and Agent → "Show Warp Agent changelog in new conversation view": no updater, no changelog fetch.
- Agent → "Let us know" link → Warp's Typeform (`warp_agent_page.rs:3963`; already listed as residue).
- Third party CLI agents → toolbar layout still offers a **Voice Input** chip (Voice is Wispr Flow,
  gated elsewhere).
- Appearance → "…conversation title for **Oz** and third-party agent sessions"
  (`appearance_page.rs:5124`); Oz is hard-disabled.
- Actions palette (`04b-palette-actions`): "Disable Agent Task Completion Notifications" while AI is off;
  "Open Settings: Warpify".
- "New worktree config" in the + menu (`10c`) highlights but shows no submenu.
- About shows 0.1.0 while `chore/release-v0.3.0` exists — check the version source before the release.

## 3. Still visible from the 2026-11-21 audit (confirmed in screenshots)

| String | Screen | 11-21 row |
|---|---|---|
| "…will consume Warp credits", "…not on Warp's servers" | settings/02-agents-agent-p02 | warp_agent_page.rs:5449,5461 (policy bug) |
| Warpify page, "Configure whether Warp attempts to…", "Warpify SSH Sessions", "Warp-owned" ControlMaster | settings/11-warpify-p01 | warpify_page.rs |
| "Start Warp at login", "Warp completions", "Receive desktop notifications from Warp" | settings/09-features-p01…p04 | features_page.rs |
| Input type radio "Warp", "Pin to the bottom (Warp mode)" | settings/08-appearance-p02 | appearance_page.rs:1661 |
| "Learn more" → docs.warp.dev (Agent, MCP, Warpify, Knowledge) | several | links policy bug |
| "Open Settings: Warpify" | 04b-palette-actions | workspace/mod.rs:1588 |

**Not in the 11-21 table, also visible:** Code → Editor and Code Review
(`settings/07b…`): editor choice value **"Warp"**, "Choose a layout to open files in **Warp**",
"Open Markdown files in **Warp's** Markdown Viewer", "…in the **Warp** text editor"; Code → Indexing
description "**Warp** can automatically index…" (`code_indexing_page.rs:77`).

**Fixed since 11-21 (verified):** About page (`settings/13-about-p01`), avatar menu "View Synth Term
logs" (`07-avatar-menu`), Privacy copy (`settings/12-privacy-p01`), Drive removed from the tools panel
and settings, notification toasts titled "Synth Term".

## 4. Suggested order

1. Filter binding lists by offer predicate (2.1) — one change removes ~15 dead rows; feeds warp-cpo45.
2. Gate sharing on the remote-control URL (2.3) and fix local-first.md either way.
3. Collapse built-in-agent pages behind "requires backend" (2.4).
4. Decide the workflow catalog (2.2) and correct cloud-surface-audit.md.
5. Hand §3 strings to the rename epic.

## 5. Limits

Windows only; title-bar system menu, onboarding, TUI, file-open banners and agent surfaces with AI on
were not exercised. No network capture. "Inert" claims rest on synth-fork.md and the gates cited, not
on a packet trace.
