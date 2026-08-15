---
title: Phase A Terminal Thread status — Zed × Warp Hybrid
description: Parse OSC 777 warp://cli-agent events, store live status on AgentTerminal, and reuse ThreadItem badges. First Zed implementation slice.
---

# Phase A — Terminal Thread status

Parent: [Level 3](level-3.md) · Model: [AgentSession](agent-session.md)

**Highest leverage, smallest surface in Zed.** Teach Terminal Threads the status machine ACP threads already have. This is the Warp vertical-tab experience for 5–15 sessions, on the sidebar Zed already has.

If Warp remains the agent host, you can instead read the same status via [Level 2](level-2.md) without this parser — but then badges live in Warp, not in Zed’s Threads Sidebar.

ACP/native threads already map to `AgentThreadStatus`. Terminal Threads are hard-coded to `AgentThreadStatus::default()` (`Completed`) in:

- `(Zed) crates/sidebar/src/sidebar.rs`
- `(Zed) crates/sidebar/src/thread_switcher.rs`

They only get a **notification dot** from BEL. See [Current state](current-state.md).

## Work items {#work}

1. **Parse OSC 777** `warp://cli-agent` JSON in `(Zed) crates/terminal`. Alacritty already delivers OSC; this tree currently ignores that payload. Event names: [AgentSession](agent-session.md#osc) (match Warp’s `CLIAgentEventType`, including `permission_request` / `stop_failure`, not only `question_asked`).
2. **Compat env (decide, default off if dirty):** optionally set `WARP_CLI_AGENT_PROTOCOL_VERSION=1` on Terminal Thread PTYs so existing Warp plugins emit structured events. Impersonating Warp via `TERM_PROGRAM` is too dirty; a Zed-native env plus thin plugins is cleaner. A setting can enable Warp-compat env for people who already installed Warp plugins.
3. **Keep fallbacks:** BEL + OSC 0 titles (Claude `preferredNotifChannel: terminal_bell`, Amp `AMP_FORCE_BEL`, OpenCode/Pi bell plugins, Codex `tui.terminal_title`). Documented in `(Zed) docs/src/ai/terminal-threads.md`.
4. **Store live status** on `AgentTerminal`: Idle / Running / Permission / Done / Error, last query, attention.
5. **Wire the sidebar** to `ThreadItem` badges instead of `AgentThreadStatus::default()`.

Envelope (reimplement, do not copy Warp sources):

```text
ESC ] 777 ; notify ; warp://cli-agent ; <json> BEL
```

## Files {#files}

| File | Change |
|------|--------|
| `(Zed) crates/terminal/` (Alacritty adapter) | Emit parsed OSC 777 as a `TerminalEvent` |
| `(Zed) crates/agent_ui/src/agent_panel.rs` | `AgentTerminal` status fields; subscribe to events |
| `(Zed) crates/sidebar/src/sidebar.rs` | Map terminal → `AgentThreadStatus` |
| `(Zed) crates/sidebar/src/thread_switcher.rs` | Same |
| `(Zed) crates/agent_settings/` | Setting for Warp-compat env vs Zed-native |
| `(Zed) docs/src/ai/terminal-threads.md` | Document structured notifications when shipped |

Reference parser behavior: `(Warp) app/src/terminal/cli_agent_sessions/event/v1.rs`.

## Windows note {#windows}

Many Warp plugins write OSC 777 to `/dev/tty`, which is Unix-centric. On Windows, prefer parsing whatever the PTY already delivers, and Zed-native plugins that write to stdout/stderr of the session. Do not assume Warp’s Unix plugin scripts work unchanged.

## Done when {#done}

- A Claude/Codex/OpenCode Terminal Thread shows Running while working, Permission/waiting when blocked, Done/Idle when finished
- Unfocused sessions still BEL-notify as today
- ACP thread badges are unchanged
- No Warp AGPL code in the Zed tree

## See also

- [Phase B](phase-b-launcher.md)
- [Crate map](crate-map.md)
- [Licenses](licenses.md)
