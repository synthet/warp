---
title: Phase C Terminal blocks — Zed × Warp Hybrid
description: Warp blocks are a shell-integration model, not a widget. Defer until Phase A. Prefer cheap ACP tool headers unless you need interactive-shell blocks.
---

# Phase C — Terminal blocks

Parent: [Level 3](level-3.md) · Prev: [Phase B](phase-b-launcher.md) · Next: [Phase D](phase-d-rich-input.md)

Warp blocks are a **shell-integration model** (prompt start/end, command, output, exit, duration), not a GPUI widget you can paste. Zed has no OSC 133. Interactive Terminal Threads are an Alacritty grid with scrollback. Warp model notes: [terminal blocks](surfaces/terminal-blocks.md).

Do this **only after [Phase A](phase-a-status.md) feels good.** Agent TUIs (Claude/Codex full-screen) often **fight** block UIs. Warp’s win for agents is the toolbelt + tabs, not wrapping the TUI in blocks.

## Two honest options {#options}

### Cheap (probably enough)

Keep interactive TUIs as raw PTY. Only show blocks for **Zed-owned** commands.

Already implemented: `acp_thread::Terminal` + `TerminalToolHeader` (elapsed, exit code, expand/stop, sandbox warning). Tasks still spawn into a terminal tab, not a block stream.

This is Warp-block-shaped for ACP tool calls. It is **not** Warp-like for `ls` / tests typed in a Terminal Thread.

### Real (weeks, not days)

Add shell integration (OSC 133 or a Zed prompt hook) and a block list beside the Alacritty grid in `(Zed) crates/terminal` / `crates/terminal_view`. That is a terminal-emulator project: prompt markers, command capture, virtualized block rendering, copy-as-block, click-to-re-run.

Reimplement BlockList **semantics** from Warp’s public client. Do not vendor AGPL terminal code. See [Licenses](licenses.md).

## Recommendation {#recommendation}

Ship cheap. Revisit real blocks only if users miss structured `make test` / git output **inside Terminal Threads** after status badges exist. Many of those commands already belong in Tasks or ACP tool terminals.

[Level 2](level-2.md) v1 also skips full block export from Warp.

## See also

- [Current state](current-state.md)
- [What not to build](what-not-to-build.md)
