---
title: Feasibility / roadmap — Zed × Warp Hybrid
description: Verdict for 5–15 simultaneous Claude/Codex sessions. Dual-app vs Zed-only sequences. Shared todos.
---

# Feasibility and roadmap

Parent: [Zed × Warp Hybrid](README.md)

For **5–15 simultaneous Claude/Codex sessions**, “Warp’s agent/session UX inside Zed” is a feasible open-source project. Terminal Threads already created the architectural slot; Warp already runs the dashboard in vertical tabs.

If choosing an end-state: **Zed as the base, not Warp.** The editor is the harder component to recreate. Warp’s session/agent UX is more separable and therefore more realistic to port.

## Verdict table {#table}

| Need | Feasible now | How |
|------|--------------|-----|
| Many simultaneous Claude/Codex/OpenCode | Yes | Warp vertical tabs **or** Zed Terminal Threads + ACP |
| Warp-quality status scan **in Warp** | Yes | `CLIAgentSessionsModel` (in-process only) |
| Warp-quality status scan **in Zed** | Not yet | [Phase A](phase-a-status.md) if Zed hosts; [Level 2](level-2.md) if Warp hosts |
| Click path in Warp, edit in Zed | macOS/Linux yes; Windows no | [Level 1](level-1.md) + [Windows](constraints/windows.md) |
| One-key launch of a Warp layout from Zed | Yes | Task + `warposs://` / `warp://` |
| Shared session truth while using both apps | Yes, extra work | [Level 2](level-2.md) — extend `local_control` |
| Warp-like blocks + rich TUI input inside Zed | Yes, core fork | [Phase C](phase-c-blocks.md) / [Phase D](phase-d-rich-input.md) |
| One merged binary of both apps | No, not useful | Different frameworks + [licenses](licenses.md) |

## Two sequences {#sequence}

Pick the daily driver, then stop when it feels good.

### Dual-app (Warp sessions, Zed editor)

1. [Level 1](level-1.md): Windows Zed editor + `warposs://` tasks.
2. [Level 2](level-2.md) Warp side: enable `warp_control_cli`, Windows named-pipe broker, `agent.session.list`.
3. Zed consumer: MCP/tasks that poll that catalog (separate from core Phase A).
4. Only if you want the dashboard *inside* Zed: [Phase A](phase-a-status.md) onward.

### Zed-only agent host

1. Keep Zed editor; run Claude/Codex as Terminal Threads (lossy BEL/title today).
2. **First implementation slice:** [Phase A](phase-a-status.md) only.
3. **Skip [Level 2](level-2.md)** — a Warp export would be throwaway.
4. [Phase B](phase-b-launcher.md) once status is scannable.
5. Defer [Phase C](phase-c-blocks.md) and [Phase D](phase-d-rich-input.md).

## Implementation todos {#todos}

### Synth Warp checkout

- [ ] Add Zed / Zed Preview as a Windows external editor (`app/src/util/file/external_editor/windows.rs`)
- [ ] Register `warposs` URL scheme on Windows
- [ ] Enable `warp_control_cli` for OSS; implement Windows local_control publication (named pipe / ACL)
- [ ] Add `agent.session.list` / `inspect` and `agent.notification.list`; add `cwd` to `tab.create`

### Synth Zed checkout

- [ ] Parse OSC 777 `warp://cli-agent` in `crates/terminal`; map events onto `AgentTerminal`; wire `ThreadItem` badges instead of `AgentThreadStatus::default()` for Terminal Threads
- [ ] Decide Warp-compat env (`WARP_CLI_AGENT_PROTOCOL_VERSION`) vs Zed-native plugins; keep BEL + OSC title fallbacks
- [ ] Add declarative session/worktree/agent launcher (Tab Config analogue) on `TerminalThreadMetadata` + New Thread menu
- [ ] Zed task/keybinding to open `warposs://tab_config` or `warposs://action/new_tab` (and `warp://` for upstream Warp)
- [ ] MCP/poll consumer of Warp `agent.session.*` **only if** dual-app remains the daily workflow
- [ ] Defer interactive terminal blocks (OSC 133) and Ctrl-G-style rich TUI input until status/launcher prove out

## See also

- [Overview](overview.md)
- [What not to build](what-not-to-build.md)
- [Windows](constraints/windows.md)
