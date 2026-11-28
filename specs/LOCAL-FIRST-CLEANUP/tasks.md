# Tasks: Local-first capability cleanup

Spec: [spec.md](spec.md). One task is one PR. Reuse `EditableBinding::with_enabled` and ChannelState predicates
(`warp_cloud_enabled`, `cloud_agents_enabled`, `warp_hosted_ai_enabled`, `show_autoupdate_menu_items`,
`self_hosted_remote_control_enabled`). Mark each deviation `// Synth Warp is local-first:`; test positively and negatively.

## Traceability matrix

| AC | Tasks | Verification |
|----|-------|--------------|
| AC-1, AC-2 | T-1 | warp_agent_page test; screenshot crop 10 re-shot |
| AC-3 | T-2 | docs/reports network-isolation report |
| AC-4, AC-5, AC-6 | T-3 | bindings_tests absence/presence; code_diff_view keystroke test |
| AC-7, AC-8, AC-9 | T-4 | state_tests, view_impl_tests, workspace/view_tests |
| AC-10, AC-11 | T-5 | page tests (knowledge_page_tests pattern) |
| AC-12 | T-6 | bindings + features_page tests |
| AC-13 | T-7 | toolbar_item tests |
| AC-14, AC-15 | T-8 | categories_tests; doc review |
| AC-16 | T-9 | cargo nextest run -p warp -E 'test(oss_exposure)' |
| AC-17 | T-10 | spec review |
| AC-18 | T-11 | About page check + release notes |

## Task table

| ID | Task | Owner role | Files / areas | Depends on | ACs | Verification |
|----|------|------------|---------------|------------|-----|--------------|
| T-1 | Honest BYOK / Custom Inference copy: OSS-specific text, key fields disabled with prerequisite unless a backend is configured | settings | app/src/settings_view/warp_agent_page.rs (~5449, 5461) | - | AC-1, AC-2 | cargo nextest run -p warp -E 'test(warp_agent_page)' |
| T-2 | Runtime network-isolation audit (no code): capture startup, idle, settings walk, workflows, restore, SSH, indexing, agent toggle with the Network log console and an OS capture; file follow-ups | qa | docs/reports/ | - | AC-3 | report committed under docs/reports |
| T-3 | Hide dead commands from shortcut lists: with_enabled on Drive bindings, team workflows, share pane/block (warp_cloud_enabled), agent management view (cloud_agents_enabled after verifying local use); filter supports_gui() at slash-command registration; listing-only denylist helper in util/bindings.rs for Accept Prompt Suggestion and Open AI Command Suggestions | keybindings | app/src/workspace/mod.rs, app/src/terminal/view/init.rs, app/src/pane_group/pane/view/mod.rs, app/src/terminal/input.rs, app/src/util/bindings.rs, app/src/settings_view/keybindings.rs, app/src/resource_center/keybindings_page.rs | - | AC-4, AC-5, AC-6 | cargo nextest run -p warp -E 'test(bindings)' |
| T-4 | Gate session sharing on a configured remote-control server: add can_create_shared_sessions() and use it at every entry point; stop update_session_sharing_enablement from enabling the flag on OSS; update local-first.md and cloud-surface-audit.md | sharing | app/src/tab.rs, app/src/terminal/view/pane_impl.rs, app/src/terminal/view.rs, app/src/terminal/view/init.rs, app/src/workspace/view.rs, app/src/settings_view/features_page.rs, app/src/app_menus.rs, app/src/workspaces/user_workspaces/mod.rs, docs/features/implemented/local-first.md | - | AC-7, AC-8, AC-9 | cargo nextest run -p warp_core -E 'test(self_hosted)'; nextest shared_session + workspace view tests |
| T-5 | Built-in agent surfaces require a backend: add AISettings::is_builtin_agent_available and gate profiles base model, MCP servers page, indexing context widgets and the conversations tab (verify each serves only the built-in agent); label detected Claude/Codex MCP configs read-only | agent-settings | app/src/settings/ai.rs, app/src/settings_view/agent_profiles_page.rs, app/src/settings_view/mcp_servers_page.rs, app/src/settings_view/code_indexing_page.rs | T-1 | AC-10, AC-11 | page tests in knowledge_page_tests style |
| T-6 | Updater and changelog surfaces off in OSS: gate update bindings, changelog toast row, agent changelog row and /changelog | settings | app/src/workspace/mod.rs (~1214-1233), app/src/settings_view/features_page.rs, app/src/settings_view/warp_agent_page.rs, app/src/search/slash_command_menu/static_commands/commands.rs | - | AC-12 | nextest bindings + features_page tests |
| T-7 | Capability-aware CLI-agent toolbar chips: drop VoiceInput from available/default lists when unavailable, sanitize saved layouts, Restore default yields supported chips only | agent-ui | app/src/ai/blocklist/agent_view/agent_input_footer/toolbar_item.rs, agent_input_footer/mod.rs | - | AC-13 | nextest toolbar_item tests |
| T-8 | Workflows: hide Team Workflows on OSS via a pure sidebar_workflow_types(cloud) helper, route the docs link through warp_inc_links_enabled, correct cloud-surface-audit.md | workflows | app/src/workflows/categories.rs, app/src/settings_view/features_page.rs (~7528), docs/architecture/cloud-surface-audit.md | - | AC-14, AC-15 | cargo nextest run -p warp -E 'test(categories)' |
| T-9 | OSS exposure regression test: enumerate editable binding names and visible settings sections under the OSS channel and fail on denylisted capabilities | qa | app/src/util/bindings_tests.rs or new app/src/oss_exposure_tests.rs | T-3, T-4, T-5, T-6, T-7 | AC-16 | cargo nextest run -p warp -E 'test(oss_exposure)' |
| T-10 | Settings information-architecture spec (no code): CLI Agents first-class, Built-in Agent optional group, Sessions & Workspaces; capability model supported/configured/can-execute; reference reviewer mockups | product | specs/SETTINGS-IA/ | - | AC-17 | spec review |
| T-11 | Version source check: explain or fix About 0.1.0 vs chore/release-v0.3.0 | release | app/Cargo.toml, release scripts, About page | - | AC-18 | About page shows the intended version |
