---
title: Terminal blocks — Zed × Warp Hybrid
description: Warp command/output grouping. Level 2 v1 should not dump full blocks. Level 3 Phase C ports semantics into Zed.
---

# Terminal blocks

← [Zed × Warp](../README.md)

Command/output grouping is one of Warp’s distinctive features. [Phase C](../phase-c-blocks.md) would port the *behavior* into Zed. [Level 2](../level-2.md) should not dump full blocks in v1.

## Layers `(Warp)`

| Layer | Path | WarpUI coupling |
|-------|------|-----------------|
| Grid / ANSI | `crates/warp_terminal/` | Low |
| `Block`, `BlockList`, lifecycle | `app/src/terminal/model/` | High |
| GUI paint | `block_list_element.rs`, `terminal/view.rs` | Full WarpUI |
| TUI paint | `crates/warp_tui/src/terminal_block.rs` | Same model via `tui_export` |

`Block` already has: command, output, `pwd`, `exit_code`, `start_ts` / `completed_ts`, git metadata. `SerializedBlock` is serde JSON of the same.

## Separability

Conceptually separable. Practically still in AGPL `app` with WarpUI entity types. `warp_tui` is proof of a second renderer, not a standalone library.

**Portable:** the transcript idea. **Not portable as-is:** `app/src/terminal/model/*` and the GUI list element.

## See also

- [Phase C](../phase-c-blocks.md)
- [CLI agent sessions](cli-agent-sessions.md)
- [Licenses](../licenses.md)
