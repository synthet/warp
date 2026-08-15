---
title: CLI agent sessions — Zed × Warp Hybrid
description: Warp in-process model for Claude/Codex/Gemini/OpenCode. Live source of truth for AgentSession when Warp hosts agents.
---

# CLI agent sessions

← [Zed × Warp](../README.md)

Warp’s in-process model for Claude Code, Codex, Gemini CLI, OpenCode, and similar harnesses. Live source of truth for [AgentSession](../agent-session.md) when Warp is the host.

## Core types `(Warp)`

| Path | Types |
|------|-------|
| `app/src/terminal/cli_agent.rs` | `CLIAgent` |
| `app/src/terminal/cli_agent_sessions/mod.rs` | `CLIAgentSession`, `CLIAgentSessionStatus`, `CLIAgentSessionsModel` |
| `cli_agent_sessions/event/` | OSC 777 parse (`CLIAgentEventType`) |
| `crates/warp_core/src/cli_agent_protocol.rs` | `CLIAgentNotification` |
| `app/src/ai/agent/conversation.rs` | `ConversationStatus` |
| `app/src/ai/agent_conversations_model.rs` | Unified list rows |
| `app/src/ai/agent_management/` | `AgentNotificationsModel` |

Plugins: `cli_agent_sessions/plugin_manager/{claude,codex,gemini,opencode}.rs`.

## Status machine

`InProgress` / `Success` / `Failed` / `Blocked { message }`. Maps to `ConversationStatus` via `to_conversation_status()`. Event strings: [AgentSession OSC table](../agent-session.md#osc).

`supports_rich_status()` is true only after a structured OSC 777 notification. Codex OSC 9 fallback never sets `received_rich_notification`.

Sentinel: `warp://cli-agent`. Env: `WARP_CLI_AGENT_PROTOCOL_VERSION`, `WARP_CLIENT_VERSION`.

## Notifications

- `Blocked` / permission → `NotificationCategory::Request`
- `Success` → `Complete`
- `Failed` → `Error` (gated by `HOANotifications`)

No external mailbox. Level 2 should add `agent.notification.list`.

## Vertical tabs

`app/src/workspace/view/vertical_tabs.rs` already joins title, `ConversationStatus`, cwd, branch, unread activity. Export the data; do not reverse-engineer the pixels.

## What is not available outside the process

- No `all_sessions()` iterator — map keyed by terminal `EntityId`
- `session.list` over local_control is layout handles only
- Internal events stay in-process

In Synth Warp, treat CLI agents as primary. Cloud-oriented `AgentConversationsModel` fields will not populate without Warp servers. See [Synth forks](../constraints/synth-fork.md).

## See also

- [Level 2](../level-2.md)
- [Phase A](../phase-a-status.md) (Zed-side parser of the same envelope)
- [Local control](local-control.md)
