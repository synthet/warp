---
title: AgentSession — Zed × Warp Hybrid
description: Shared session model across Zed native, ACP, Terminal Threads, and Warp CLI agents.
---

# AgentSession

Parent: [Zed × Warp Hybrid](README.md)

Make this a **union over Zed’s three lanes plus optional Warp-hosted CLI sessions**, not a new product object that ignores ACP. The UI chrome (badges, dashboard) is mostly already in Zed `ThreadItem` and Warp vertical tabs. See [Current state](current-state.md) and the [crate map](crate-map.md).

Correlate Warp ↔ Zed by **worktree / cwd path**, not window title.

## Model {#model}

```text
AgentSession {
  id:            ThreadId | TerminalId | acp::SessionId | warp pane_id
  kind:          Native | Acp | TerminalThread | WarpCli
  repo / worktree / cwd / branch
  agent:         claude | codex | opencode | gemini | zed | ...
  pty_or_pid
  terminal:      Warp pane_id / tab_id / window_id   // WarpCli only
  status:        Idle | Running | WaitingInput | Permission | Done | Error
  last_query
  last_activity
  attention:     bool
  rich_status:   bool   // Warp: OSC 777 received; Codex OSC 9 does not qualify
  source:        zed | warp
}
```

`kind` stays distinct on purpose. Claude-via-ACP and Claude-via-TUI must **not** be forced onto one wire protocol. Unify them only here and in the sidebar.

## Status mapping {#status}

Map status from **whatever signal the lane actually has**:

| Lane | Source of truth | Maps to |
|------|-----------------|---------|
| ACP / native | `ThreadStatus` + pending permission + `had_error` | `AgentThreadStatus` (already) |
| Terminal Threads today | BEL (attention) + OSC 0 title (lossy) | Notification dot only; status forced to Completed |
| Warp CLI agents | OSC 777 JSON + `CLIAgentSessionStatus` | Fine-grained tab badges |
| [Level 2](level-2.md) export | Serialized `CLIAgentSession` (+ pane ids) | Cross-app `AgentSession` array |

Warp CLI status → conversation UI → hybrid badge:

| `CLIAgentSessionStatus` | `ConversationStatus` | Badge |
|-------------------------|----------------------|-------|
| `InProgress` | `InProgress` | running |
| `Blocked { message }` | `Blocked { blocked_action }` | permission / waiting |
| `Success` | `Success` | done |
| `Failed { .. }` | `Error` | failed |

`CLIAgentSession::supports_rich_status()` is true only after a structured OSC 777 notification. Codex OSC 9 fallback does not qualify.

## OSC 777 event → status {#osc}

Envelope (reimplement parsers; do not copy Warp AGPL sources):

```text
ESC ] 777 ; notify ; warp://cli-agent ; <json> BEL
```

JSON fields: `v`, `agent`, `event`, `session_id`, `cwd`, `project`, plus event-specific `query` / `response` / `tool_name` / `summary` / `error_type`. Sentinel title: `warp://cli-agent`. Env: `WARP_CLI_AGENT_PROTOCOL_VERSION`.

Actual `(Warp) CLIAgentEventType` strings:

| `event` | `status` | `attention` |
|---------|----------|-------------|
| `session_start` | Idle (or keep prior) | false |
| `prompt_submit` | Running | false |
| `tool_complete` | Running | false |
| `permission_request` | Permission | **true** |
| `permission_replied` | Running (or keep) | false |
| `question_asked` | Permission / waiting | **true** |
| `idle_prompt` | Idle / Done | optional |
| `stop` | Done | true if unfocused |
| `stop_failure` | Error | true |

[Phase A](phase-a-status.md) teaches Terminal Threads this mapping (OSC 777 first, BEL/title fallback). [Level 2](level-2.md) exports Warp’s already-parsed machine.

## Suggested Level 2 JSON {#json}

```json
{
  "session_id": "cli-abc",
  "kind": "WarpCli",
  "pane_id": "...",
  "tab_id": "...",
  "window_id": "...",
  "agent": "claude",
  "status": "blocked",
  "cwd": "D:/Projects/foo",
  "project": "foo",
  "branch": "feature-a",
  "last_query": "fix the flaky test",
  "attention_required": true,
  "blocked_message": "Approve Edit?",
  "rich_status": true,
  "source": "warp"
}
```

v1 should **not** include full terminal block output. Optionally add `last_command` later.

## Existing types to fold onto {#existing-types}

**Zed — do not invent a parallel sidebar:**

- `ThreadMetadata` / `TerminalThreadMetadata` for durable identity
- `AgentTerminal` for live Terminal Thread status
- `AcpThread` / `AgentSessionInfo` for ACP
- `AgentThreadStatus` + `ThreadItem` for rendering

**Warp — do not invent a parallel model:**

- `CLIAgentSession` / `CLIAgentSessionsModel`
- Vertical-tab summary join
- `AgentNotificationsModel` for the mailbox

A dedicated “session dashboard” app is unnecessary if those two UIs show the same fields.

## Optional disk cache {#cache}

`~/.synth/sessions/*.json` written on status change helps when Warp is down. It is **not** the broker. Live state stays in Warp or in Zed’s `AgentTerminal`.

## See also

- [Phase A](phase-a-status.md)
- [Level 2](level-2.md)
- [CLI agent sessions](surfaces/cli-agent-sessions.md)
- [Crate map](crate-map.md)
