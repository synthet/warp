# Spec: Local-first capability cleanup (OSS build)

Sources: `docs/reports/synth-term-ui-audit-2026-11-26.md` (screenshot audit, 21 crops), reviewer package
`synth-term-audit-report-with-mockups.zip` (REPORT.md, 2026-11-26), and code exploration on master 0e6fd3ac.
Goal: every visible command, menu entry and setting in the default OSS build corresponds to a capability that works,
or states its prerequisite. Gate, never delete (cloud-surface-audit.md conventions). Branding strings are out of scope:
they belong to the Synth Term rename epic (T-8/T-9/T-11, specs/SYNTH-TERM).

## Acceptance criteria

- AC-1: WHEN the OSS build runs without SYNTH_WARP_SERVER_ROOT_URL THEN Settings → Agents → Agent → Custom Inference shows no "Warp credits" or "Warp's servers" text and explains that keys are only used by a self-hosted backend.
- AC-2: WHEN no backend is configured THEN the provider key fields are disabled with a one-line prerequisite, and enabled when `ChannelState::warp_cloud_enabled()` is true.
- AC-3: WHEN the OSS build is exercised (startup, idle 10 min, settings walk, workflows, session restore, SSH, indexing toggle, agent toggle) THEN no connection to *.warp.dev or any host other than user-configured ones is observed, and any exception is filed as a ticket.
- AC-4: WHEN the OSS build runs THEN Settings → Keyboard shortcuts and the Keyboard Shortcuts panel list none of: Left Panel: Warp Drive, Toggle Warp Drive, Toggle Team Workflows Modal, Share Pane, Share Selected Block, Slash command: /connect-grok, Accept Prompt Suggestion, Open AI Command Suggestions.
- AC-5: WHEN Accept Prompt Suggestion is hidden from listings THEN the code-diff accept keystroke lookup (code_diff_view.rs get_binding_by_name) still resolves.
- AC-6: WHEN the Agent Management View has no local function in OSS THEN its binding (Ctrl+Shift+M) is gated on `ChannelState::cloud_agents_enabled()`; otherwise the finding is documented and the binding works.
- AC-7: WHEN SYNTH_WARP_REMOTE_CONTROL_URL is unset THEN no "Share session" entry appears in the tab, pane, terminal, block, input or alt-screen menus, the command palette, slash commands, inline banners or the Features "Confirm before closing shared session" row.
- AC-8: WHEN SYNTH_WARP_REMOTE_CONTROL_URL is set to a valid non-warp.dev URL THEN those entries appear and sharing uses the self-hosted server, never a Warp login prompt.
- AC-9: WHEN the OSS build updates workspace settings THEN `update_session_sharing_enablement` cannot turn `CreatingSharedSessions` on without a configured remote-control URL.
- AC-10: WHEN no backend is configured THEN Agent Profiles (base model aliases), MCP servers, codebase-indexing context widgets and the tools-panel Agent conversations tab show a prerequisite message instead of inert controls, for each surface verified to serve only the built-in agent.
- AC-11: WHEN the MCP servers page lists servers detected from Claude/Codex configs THEN it states whether they are read-only, and editing them never writes the external agent's config.
- AC-12: WHEN `autoupdate_config` is None THEN Check for Updates, Install Update and Relaunch, Features "Show changelog toast after updates", Agent "Show Warp Agent changelog…" and /changelog are hidden.
- AC-13: WHEN Voice input is unavailable (`!warp_hosted_ai_enabled()`) THEN the CLI-agent toolbar configurator offers no Voice Input chip, saved layouts drop it on load, and Restore default produces only supported chips.
- AC-14: WHEN the OSS build runs THEN the Workflows modal shows no Team Workflows category, the bundled catalog and Repository workflows remain, and the Features workflow info link goes through `warp_inc_links_enabled()`.
- AC-15: WHEN cloud-surface-audit.md and local-first.md are read THEN they state that global workflows are a bundled offline catalog and that sharing requires a configured remote-control server.
- AC-16: WHEN nextest runs THEN an OSS exposure test enumerates editable binding names and visible settings sections and fails if any belongs to a denylisted unavailable capability (Drive, team, updater, sharing without server, Oz, SuperGrok, voice).
- AC-17: WHEN the settings information architecture is revised THEN a spec exists that makes CLI Agents first-class, groups the built-in agent as one optional section, and covers the reviewer's mockups and capability (supported/configured/can-execute) model.
- AC-18: WHEN About shows the version THEN its source is documented and consistent with the release branch (0.1.0 vs chore/release-v0.3.0 explained or fixed).
