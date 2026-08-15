---
type: Technical Reference
title: System overview
description: Two front-ends share warp_core / warpui. GUI is app/; TUI is crates/warp_tui.
resource: architecture/system-overview.md
tags: [docs, architecture]
timestamp: 2026-08-15T00:00:00Z
okf_version: 0.1
---

# System overview

Synth Warp has **two front-ends** that share `warp_core` / `warpui` (Entity/model core, actions, appearance, feature flags). They differ in UI framework, rendering, input, and how you verify them.

| Front-end | Lives in | Role |
|-----------|----------|------|
| GUI desktop | [`app/`](../../app/) on WarpUI (`warpui`, `warpui_core`) | Pixel/GPU layout (`Element`/`View`), WGSL, mouse, app bundles |
| Headless TUI | [`crates/warp_tui`](../../crates/warp_tui/) | Cell-grid `TuiElement` in `warpui_core` (feature `tui`); no GPU or `.app` bundle |

Shared pieces: [`crates/warp_core`](../../crates/warp_core/), [`crates/warpui`](../../crates/warpui/), [`crates/warpui_core`](../../crates/warpui_core/). Feature surfaces under `app/` (terminal, AI, settings, workspace) are reused by the TUI where that crate depends on them.

Engineering commands and style: [AGENTS.md](../../AGENTS.md). Crate list: [crate-map.md](crate-map.md). Handle pattern: [entity-handle.md](entity-handle.md). Fork boundary: [synth-fork.md](synth-fork.md).

Hybrid Zed work is a separate topic wiki: [zed-warp/README.md](../zed-warp/README.md).
