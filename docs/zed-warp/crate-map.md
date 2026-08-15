---
title: Crate map — Zed × Warp Hybrid
description: Where a hybrid change would land in Synth Zed and Synth Warp. Paths are labeled by checkout.
---

# Crate map

Parent: [Zed × Warp Hybrid](README.md) · Related: [Current state](current-state.md)

This is a fork-internal map. Paths are **not** clickable across checkouts; see [MIRROR.md](MIRROR.md).

## Zed — product docs {#zed-product-docs}

In the Synth Zed checkout under `docs/src/`: `ai/terminal-threads.md`, `ai/external-agents.md`, `ai/parallel-agents.md`, `ai/agent-panel.md`, `extensions/developing-extensions.md`, `extensions/agent-servers.md`, `tasks.md`, `terminal.md`.

## Zed — agent UI / sidebar {#zed-agent-ui}

| Path | Role |
|------|------|
| `crates/agent_ui/` | Agent Panel, conversation UI, Terminal Thread hosting |
| `crates/agent_ui/src/agent_panel.rs` | `AgentPanel`, `AgentTerminal`, `BaseView`, persistence |
| `crates/agent_ui/src/agent_ui.rs` | `Agent`, `AgentThreadSource`, actions |
| `crates/agent_ui/src/conversation_view.rs` | Thread rendering, permissions, elicitations |
| `crates/agent_ui/src/message_editor.rs` | Mentions, images, ACP prompt capabilities |
| `crates/agent_ui/src/thread_metadata_store.rs` | `ThreadId`, `ThreadMetadata` (SQLite sidebar index) |
| `crates/agent_ui/src/terminal_thread_metadata_store.rs` | `TerminalThreadMetadata` |
| `crates/agent_ui/src/ui/agent_notification.rs` | Pop-up notification windows |
| `crates/agent_ui/src/ui/terminal_tool_header.rs` | Structured command header (ACP/native tool calls) |
| `crates/sidebar/` | Threads Sidebar (`sidebar.rs`, `thread_switcher.rs`) |
| `crates/ui/src/components/ai/thread_item.rs` | `ThreadItem`, `AgentThreadStatus` |

**Phase A:** `AgentTerminal` live status; stop using `AgentThreadStatus::default()` for terminals in `sidebar.rs` / `thread_switcher.rs`.

**Phase B:** `terminal_thread_metadata_store.rs`, New Thread menu in `agent_panel.rs`.

## Zed — ACP / agent runtime {#zed-acp}

| Path | Role |
|------|------|
| `crates/acp_thread/` | `AcpThread`, `AgentThreadEntry`, `AgentSessionInfo` |
| `crates/acp_thread/src/connection.rs` | `AgentConnection`, session list, permissions |
| `crates/acp_thread/src/terminal.rs` | Structured ACP `Terminal` + `TerminalOutput` |
| `crates/agent_servers/src/acp.rs` | Spawn/talk to external ACP agents |
| `crates/agent/` | Native Zed agent and tools |
| `crates/agent_settings/` | Notify/sound/init-command, panel layout |
| `crates/project/src/agent_server_store.rs` | Registry + custom agent install |
| Workspace dep | `agent-client-protocol = "=2.0.0"` |

Do **not** force Terminal Threads onto ACP. Unify only at [AgentSession](agent-session.md) / sidebar. See [Phase B](phase-b-launcher.md).

## Zed — terminal / workspace {#zed-terminal}

| Path | Role |
|------|------|
| `crates/terminal/` | Alacritty-based PTY |
| `crates/terminal/src/alacritty.rs` | Adapter over Alacritty term/PTY |
| `crates/terminal_view/` | `TerminalView`, terminal panel |
| `crates/task/` | `TaskTemplate`, `SpawnInTerminal` |
| `crates/workspace/src/persistence.rs` | `SerializedWorkspace`, docks, panes |
| `crates/extension_api/` | WASM Extension trait + WIT |
| `crates/zed/src/zed/open_listener.rs` | Deep-link URL handling (`synthzed://…`) |

**Phase A:** parse OSC 777 in `crates/terminal` (Alacritty already delivers OSC; this tree currently ignores `warp://cli-agent` payloads).

**Phase C:** shell integration / OSC 133 here plus a block list beside the grid.

[Level 1](level-1.md) can use tasks/`open_url` without new deep-link code.

## Zed types closest to AgentSession {#zed-types}

| Type | Location | Meaning |
|------|----------|---------|
| `acp::SessionId` | ACP schema | Protocol session id |
| `AgentSessionInfo` | `acp_thread/src/connection.rs` | List entry |
| `ThreadMetadata` | `thread_metadata_store.rs` | Sidebar index |
| `ThreadId` | UUID | Zed-local id |
| `AcpThread` | `acp_thread.rs` | Live conversation |
| `Agent` | `agent_ui.rs` | `NativeAgent` \| `Custom { id }` |
| `TerminalThreadMetadata` | `terminal_thread_metadata_store.rs` | Title, cwd, worktrees, remote |
| `AgentTerminal` | `agent_panel.rs` | Live PTY + notification windows |
| `AgentThreadStatus` | `thread_item.rs` | `Completed` / `Running` / `WaitingForConfirmation` / `Error` |
| `AgentPanelTerminalInfo` | `agent_panel.rs` | Live terminal-thread info for sidebar |

## Warp — CLI agents and UI join {#warp-agents}

| Path | Role |
|------|------|
| `app/src/terminal/cli_agent.rs` | `CLIAgent` enum |
| `app/src/terminal/cli_agent_sessions/mod.rs` | `CLIAgentSession`, `CLIAgentSessionStatus`, `CLIAgentSessionsModel` |
| `app/src/terminal/cli_agent_sessions/event/` | OSC 777 parse (`CLIAgentEventType`) |
| `crates/warp_core/src/cli_agent_protocol.rs` | `CLIAgentNotification`, sentinel `warp://cli-agent` |
| `app/src/ai/agent_management/` | `AgentNotificationsModel` |
| `app/src/workspace/view/vertical_tabs.rs` | Title + status + cwd + branch join |
| `app/src/tab_configs/tab_config.rs` | `TabConfig` / `TabConfigPaneNode` |
| `app/src/uri/mod.rs` | `warposs://` / `warp://` handlers |
| `app/src/util/file/external_editor/` | Warp → Zed/VS Code/Cursor |
| `crates/local_control/` | Protocol, catalog, discovery |
| `app/src/local_control/` | App server + `LocalControlBridge` |
| `app/src/terminal/model/` | `Block`, `BlockList`, `SerializedBlock` |
| `app/src/code/` | Native code editor UI |
| `crates/lsp/` | LSP client (five first-party servers) |
| `app/src/tui_export.rs` | App → TUI second renderer |
| `crates/warp_tui/` | Headless TUI (still AGPL `app`) |

Narrative pages: [surfaces](surfaces/external-editor.md).

## Warp types closest to AgentSession {#warp-types}

| Type | Meaning |
|------|---------|
| `CLIAgentSessionStatus` | `InProgress` / `Success` / `Failed` / `Blocked` |
| `CLIAgentSessionContext` | cwd, project, session_id, query, summary, tool preview |
| `CLIAgentEventType` | `session_start`, `prompt_submit`, `tool_complete`, `stop`, `stop_failure`, `permission_request`, `permission_replied`, `question_asked`, `idle_prompt` |
| `ConversationStatus` | Oz / Agent Mode overlay of the same idea |
| `NotificationCategory` | `Complete` / `Request` / `Error` |

## See also

- [Phase A](phase-a-status.md)
- [Local control](surfaces/local-control.md)
- [Licenses](licenses.md)
