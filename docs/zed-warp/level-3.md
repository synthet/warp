---
title: Level 3 Port Warp behavior into Zed — Zed × Warp Hybrid
description: Core/fork work in Synth Zed. Order by leverage: status, launcher, blocks, rich input. Reimplement; do not copy AGPL crates.
---

# Level 3 — Port Warp behavior into Zed

Parent: [Zed × Warp Hybrid](README.md) · Prev: [Level 2](level-2.md)

This cannot be a Zed extension. It is core/fork work in `agent_ui`, `sidebar`, and `terminal`. Order by leverage, not visual flash. Reimplement behavior; do not copy AGPL crates. See [Licenses](licenses.md) and [What not to build](what-not-to-build.md).

Do this only after the chosen [Level 1](level-1.md) / [Level 2](level-2.md) / Zed-only path feels good — or go straight to [Phase A](phase-a-status.md) if Zed hosts every agent.

```text
ZED
│
├── Editor                 ← keep Zed
│
├── Agent Sessions
│   ├── Claude Code
│   ├── Codex
│   ├── Gemini
│   └── OpenCode
│
├── Warp-like Agent Tabs   ← Phase A
│   ├── ● running
│   ├── ● waiting
│   ├── ⚠ permission
│   └── ✓ done
│
├── Rich Agent Input       ← Phase D (ACP already has this)
│
├── Terminal Blocks        ← Phase C (optional)
│
├── Workspace Configs      ← Phase B
│
└── Session Dashboard      ← Threads Sidebar, not a new app
```

That is **Zed’s editor + Warp’s agent/terminal UX**, which is a stronger product than the opposite combination.

## Phases {#phases}

| Phase | Page | Why this order |
|-------|------|----------------|
| **A** | [Terminal Thread status](phase-a-status.md) | Smallest change that makes 5–15 sessions scannable in Zed |
| **B** | [Session launcher](phase-b-launcher.md) | Tab Config analogue on metadata that already exists |
| **C** | [Terminal blocks](phase-c-blocks.md) | Real terminal-emulator work; agent TUIs often fight blocks |
| **D** | [Rich input overlay](phase-d-rich-input.md) | ACP already has `MessageEditor`; TTY overlay is agent-specific |

`warp_tui` proves Warp’s **model** can have a second renderer (`(Warp) app/src/tui_export.rs`), but that TUI still depends on AGPL `app`. Use it as a reading guide, not a crate to vendor.

## Why not Warp as the base {#not-warp-base}

Warp’s native editor, project tree, LSP, find/replace, and Git/review have grown. The gap is no longer terminal vs editor:

```text
Warp   terminal ██████████  agents ██████████  editor ██████░░░░
Zed    terminal ███████░░░  agents ████████░░  editor ██████████
```

Transplanting Zed’s editor means pulling `project` / LSP / workspace / GPUI, not a reusable text-editor crate. Warp’s session/agent UX is more separable: a protocol (OSC 777), a status machine, a launcher file, a block list. Those port. Widgets do not. See [native Warp editor](surfaces/native-editor.md).

## See also

- [Current state](current-state.md)
- [Crate map](crate-map.md)
- [Feasibility](feasibility.md)
