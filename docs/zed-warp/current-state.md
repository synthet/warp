---
title: Current state — Zed × Warp Hybrid
description: What Zed and Warp already provide, and the exact gaps a hybrid would close.
---

# Current state

Parent: [Zed × Warp Hybrid](README.md) · Files: [Crate map](crate-map.md)

Zed is further along than “needs Warp’s agent tabs from scratch.” Warp is further along than “just a pretty terminal.”

## Zed: three agent lanes {#zed-lanes}

Zed already hosts three lanes in one Threads Sidebar:

| Lane | Integration | UI | Status today |
|------|-------------|----|--------------|
| Zed Agent | Native (`crates/agent`) | `ConversationView` + `MessageEditor` | Real badges: Running / WaitingForConfirmation / Error / Completed |
| External Agents | ACP JSON-RPC (`agent_servers` + `acp_thread`) | Same chat UI | Same badges, permissions, elicitations |
| Terminal Threads | Interactive PTY (`AgentTerminal` + Alacritty) | Raw terminal | **Always `Completed`** plus a bell notification dot |

Closest Warp-like pieces **already exist** for ACP and native threads. The biggest product gaps are on Terminal Threads.

### What Zed already has {#zed-has}

- Parallel sessions, project grouping, worktrees: `(Zed) crates/sidebar`
- Status enum: `AgentThreadStatus` in `(Zed) crates/ui` (`ThreadItem`)
- Rich input for ACP/native: `MessageEditor`
- Command blocks for **agent-owned** terminals (label, elapsed, exit, stop): `acp_thread::Terminal` + `TerminalToolHeader` — Warp-block-shaped, but only when Zed spawned the command as a tool, not for interactive Claude/Codex TUIs
- Attention: BEL → `TerminalEvent::Bell` → `AgentNotification`; OSC titles (Codex is documented)
- Deep links exist (`synthzed://…` in Synth Zed) but **no Warp handler**
- Tasks can already launch OS URLs/commands

### What Zed does not have {#zed-gaps}

- Terminal Thread status badges (sidebar hard-codes `AgentThreadStatus::default()`)
- OSC 777 / `warp://cli-agent` parsing
- Interactive terminal **blocks** (no OSC 133 / shell integration)
- Declarative workspace = worktree + pane layout + commands + agent roster (Tab Config analogue)
- Any extension-built custom panel (WASM extensions cannot add agent chrome)

## Warp: CLI agents + vertical tabs {#warp-has}

- Official **Zed as file-link editor** on macOS/Linux — [external editor](surfaces/external-editor.md). **Windows: not implemented.**
- Tab Configs as TOML + `warposs://tab_config/<name>` (Synth) / `warp://…` (upstream) — [URI](surfaces/uri-and-tab-configs.md)
- Vertical tabs with branch, cwd, worktree, agent icon, status overlay
- CLI-agent protocol: `WARP_CLI_AGENT_PROTOCOL_VERSION` plus OSC 777 JSON — [CLI agent sessions](surfaces/cli-agent-sessions.md)
- BlockList terminal model (shell-owned commands — not widgets you can paste into Zed)
- Native editor that is now “good enough,” so grafting Zed’s editor into Warp is the worse direction
- Third-party CLI agents: Claude Code, Codex, OpenCode, Gemini CLI, Amp, and others (notifications are **not** universal)
- `local_control` / `warpctrl` for layout/input/`file.open` — **no agent export**, Cargo feature off on OSS, publication **off on Windows**

**Not OSS:** Warp Drive server, Oz orchestration, auth backend. A hybrid cannot assume those. See [Synth forks](constraints/synth-fork.md).

## Capability matrix {#matrix}

| Warp-like feature | Zed today | Warp today | Gap |
|-------------------|-----------|------------|-----|
| Agent tabs + status | `ThreadItem` for ACP/native; Terminal Threads always Completed | Vertical tabs + `CLIAgentSessionStatus` | Zed: parse OSC 777; Warp: export over local_control |
| Rich agent input | `MessageEditor` for ACP/native; Terminal Threads = raw TTY | Ctrl-G / `CLIAgentInputState` | Phase D if hosting TUIs in Zed |
| Terminal blocks | `TerminalToolHeader` for **tool** runs | Full BlockList in-app; not exported | Phase C optional; Level 2 v1 skips full blocks |
| Workspace configs | Fragmented: settings, tasks, `agent_servers` | Tab Config TOML | Phase B Zed-native launcher |
| Session dashboard | Threads Sidebar | Vertical tabs + agent management | Export or Phase A, not a third app |
| Parallel agents | Strong | Strong (CLI) | — |
| External agent protocol | ACP 2.0 | OSC 777 plugins | Unify only at [AgentSession](agent-session.md) |
| Warp ↔ Zed file open | Tasks / `open_url` | Zed editor macOS/Linux | [Windows](constraints/windows.md) Warp→Zed |
| Agent list from outside | None | `session.list` = pane IDs only | [Level 2](level-2.md) catalog |

## Signal quality {#signals}

| Signal | Zed ACP/native | Zed Terminal Threads | Warp CLI agents |
|--------|----------------|----------------------|-----------------|
| Sidebar status | Running / Waiting / Error / Completed | Effectively Completed only | InProgress / Success / Failed / Blocked |
| Notification | Status transitions | Bell → `has_notification` | OSC 777 + `AgentNotificationsModel` |
| Permission UI | In-thread approval + elicitation | CLI-owned (BEL) | `permission_request` / `question_asked` |
| Title | Thread title | OSC 0 + optional custom | OSC 0 spinner + metadata |

Zed Terminal Thread docs already describe BEL + title conventions for Claude, Amp, OpenCode, Pi, and Codex. That is a **lossy** approximation of Warp’s OSC 777 machine. [Phase A](phase-a-status.md) teaches Zed the structured protocol. [Level 2](level-2.md) lets Zed read Warp’s already-parsed machine while Warp remains the host.

## See also

- [Crate map](crate-map.md)
- [AgentSession](agent-session.md)
- [Overview](overview.md)
