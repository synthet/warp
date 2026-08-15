---
type: Technical Reference
title: Entity-handle pattern
description: App owns entities; views hold ViewHandle references; AppContext is temporary access during render and events.
resource: architecture/entity-handle.md
tags: [docs, architecture, warpui]
timestamp: 2026-08-15T00:00:00Z
okf_version: 0.1
---

# Entity-handle pattern

WarpUI uses an Entity-Component-Handle pattern (GUI and TUI):

- A global `App` owns all views/models (entities).
- Views hold `ViewHandle<T>` references to other views, not direct ownership.
- `AppContext` (and `ViewContext` / `ModelContext`) provides **temporary** access to handles during render and events.

Functions that take one of those context types name the parameter `ctx` and put it **last**, except when the function also takes a closure — then the closure is last.

## Mouse state

Create `MouseStateHandle` once during construction and clone it wherever hover/click is tracked. An inline `MouseStateHandle::default()` while rendering means no mouse interactions work. The TUI hover/click helpers (`TuiHoverable`, `tui_collapsible`) use the same ownership rule.

See [system-overview.md](system-overview.md) and [AGENTS.md](../../AGENTS.md).
