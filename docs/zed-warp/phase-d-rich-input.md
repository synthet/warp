---
title: Phase D Rich agent input — Zed × Warp Hybrid
description: Warp Ctrl-G injects a host editor into CLI stdin. Zed already has MessageEditor for ACP. Treat TTY overlays as a per-agent enhancement after status and launcher.
---

# Phase D — Rich agent input overlay

Parent: [Level 3](level-3.md) · Prev: [Phase C](phase-c-blocks.md)

Warp’s rich input (`Ctrl-G`) is a **host editor** that injects into the CLI stdin (`CLIAgentInputState` on `CLIAgentSession`). Zed already has `MessageEditor` for ACP/native threads (mentions, images, slash commands, ACP prompt capabilities).

## Rule of thumb {#rule}

| You want | Use |
|----------|-----|
| Mentions, images, Zed permissions, structured tool cards | External Agents / Zed Agent (ACP/native) |
| Native TUI, CLI-owned auth/subscription/config | Terminal Threads |

Do not try to make Terminal Threads into ACP. A host composer that writes a prompt into the PTY is feasible but **agent-specific** (full-screen TUI vs line-mode). Treat it as a per-agent enhancement after [Phase A](phase-a-status.md) and [Phase B](phase-b-launcher.md).

## If you still build it {#if}

- Overlay or docked composer in the Agent Panel while a Terminal Thread is focused
- Submit writes the prompt to the PTY the way the CLI expects (newline vs bracketed paste vs TUI keybinds)
- Attach-code-as-context can reuse Zed’s open buffer (the feature Warp-side issue 9550 wants in the other direction)
- Keep the raw TTY visible; do not hide the agent’s own input unless the agent is in a known line-mode

## See also

- [AgentSession](agent-session.md)
- [Crate map](crate-map.md)
- [CLI agent sessions](surfaces/cli-agent-sessions.md)
