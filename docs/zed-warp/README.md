---
title: Zed × Warp Hybrid
description: Mirrored architecture wiki for Synth Warp and Synth Zed. Use both apps, bridge via Warp local_control, then port session UX into Zed.
---

# Zed × Warp Hybrid

A **Zed-first hybrid** is feasible without embedding either GPU UI. Same language (Rust), incompatible stacks (GPUI vs WarpUI), incompatible app licenses (GPL vs AGPL). Copy **behavior and protocols**, not widgets.

This folder is mirrored in both checkouts. See [MIRROR.md](MIRROR.md). It is design wiki, not shipped product docs.

**Live session truth, when Warp is the agent host:** Warp already owns CLI-agent state. Extend `(Warp) crates/local_control` — do **not** invent a third `~/.devsessions` daemon as source of truth. An on-disk JSON dump is a cache only.

**When Synth Zed is the only agent host:** skip [Level 2](level-2.md). Invest in [Phase A](phase-a-status.md).

Until UIs merge, Warp is the process/session manager and Zed is the authoritative code workspace. End-state still favors **Zed as the base**.

## Start here

| Page | What it answers |
|------|-----------------|
| [Overview](overview.md) | Thesis, three levels, why Zed is the base |
| [Current state](current-state.md) | What each app already has vs the gaps |
| [Crate map](crate-map.md) | `(Warp)` and `(Zed)` files a change would touch |
| [AgentSession](agent-session.md) | Shared model (native / ACP / Terminal Thread / Warp CLI) |
| [Licenses](licenses.md) | GPL vs AGPL; reimplement vs copy |
| [Feasibility](feasibility.md) | 5–15 sessions, build order, todos |
| [What not to build](what-not-to-build.md) | Embeddings, AGPL vendoring, extension dashboards |

## Integration levels

| Level | Page | When |
|-------|------|------|
| 1 Dual-app | [Level 1](level-1.md) | Daily driver until Level 3 (~70–80%) |
| 2 Bridge | [Level 2](level-2.md) | Keep **both** apps; Warp exports, Zed consumes |
| 3 Port into Zed | [Level 3](level-3.md) | End-state: Warp session UX inside Synth Zed |

### Level 3 phases (Zed core/fork)

| Phase | Page | Priority |
|-------|------|----------|
| A Status | [Phase A](phase-a-status.md) | **First Zed implementation slice** |
| B Launcher | [Phase B](phase-b-launcher.md) | After A feels good |
| C Blocks | [Phase C](phase-c-blocks.md) | Optional, weeks not days |
| D Rich input | [Phase D](phase-d-rich-input.md) | Per-agent, after A+B |

## Warp surfaces (what already exists)

- [External editor](surfaces/external-editor.md) — Warp → Zed
- [URI scheme and tab configs](surfaces/uri-and-tab-configs.md) — `warposs://` / `warp://`
- [Local control](surfaces/local-control.md) — `warpctrl`, catalog, Windows publication gap
- [CLI agent sessions](surfaces/cli-agent-sessions.md) — OSC 777, status machine
- [Terminal blocks](surfaces/terminal-blocks.md) — command / output / exit / duration
- [Native Warp editor](surfaces/native-editor.md) — why not to transplant Zed’s editor

## Constraints

- [Windows](constraints/windows.md) — Warp→Zed missing; `local_control` publication off
- [Zed extension limits](constraints/zed-extensions.md) — MCP/ACP/tasks yes; custom chrome no
- [Synth forks](constraints/synth-fork.md) — no Drive, no hosted Warp AI; CLI agents only

## Sibling product docs (Zed checkout only)

Not linked from this mirror (paths differ per repo). In Synth Zed: `docs/src/ai/overview.md`, `parallel-agents.md`, `terminal-threads.md`, `external-agents.md`, `agent-panel.md`, `docs/src/terminal.md`, `docs/src/tasks.md`, `docs/src/extensions/developing-extensions.md`.
