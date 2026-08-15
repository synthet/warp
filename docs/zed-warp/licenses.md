---
title: Licenses — Zed × Warp Hybrid
description: GPL Zed, AGPL Warp, MIT/Apache UI frameworks. Reimplement protocols. Do not vendor Warp crates into Zed or GPUI into Warp.
---

# Licenses

Parent: [Zed × Warp Hybrid](README.md)

Same language is not a license merger. This page is a design constraint, not legal advice.

## Zed {#zed}

| Component | License |
|-----------|---------|
| `gpui` | Apache-2.0 (`crates/gpui`) |
| `editor`, `terminal`, `agent_ui`, most app crates | GPL-3.0-or-later |
| Root | Dual `LICENSE-APACHE` / `LICENSE-GPL`; the app/editor path is GPL |

ACP agents as **separate processes** stay outside Zed’s GPL boundary. WASM extensions stay capability-limited. Synth Zed keeps upstream licensing (commercial-free / local-first, not a license change).

## Warp {#warp}

| Component | License |
|-----------|---------|
| `warpui` / `warpui_core` | MIT |
| Rest of the open client (`app`, `warp_terminal`, `lsp`, `warp_tui`, `local_control`, …) | AGPLv3 |
| Drive server, Oz orchestration, auth backend | **Not in the public repo** |

`warpui_extras` and most helpers are AGPL (`license.workspace = true`). Do not assume “UI-adjacent = MIT.”

The MIT UI framework is still **WarpUI**, not GPUI. You cannot drop a Warp widget into Zed by depending on MIT `warpui`. You would still be embedding a second renderer and app architecture.

## Rules of engagement {#rules}

**Allowed**

- Read Warp’s public client as a **reference** for OSC 777, BlockList semantics, Tab Config shape, vertical-tab metadata
- Reimplement parsers and models in the GPL Zed tree
- Speak the OSC 777 `warp://cli-agent` envelope so existing Warp *plugins* (often MIT/Apache) keep working
- Use Warp as an external process ([Level 1](level-1.md), [Level 2](level-2.md) IPC)
- Zed talking to Warp over HTTP/IPC does **not** AGPL Zed

**Not allowed / not useful**

- Vendor `(Warp) app/src/terminal/cli_agent_sessions` or `crates/warpui` into Zed
- Link AGPL Warp application code into GPL Zed (AGPL contagion, two UI runtimes)
- Depend on Oz / Drive / Warp cloud Remote Control as if they were local OSS
- Copy Warp Tab Config files as Zed’s native format (define a Zed-native config; [Phase B](phase-b-launcher.md))
- Transplant Zed `editor` / `project` / GPUI into Warp

## Extension publishing {#extensions}

Zed’s extension index wants Apache-2.0 / MIT / GPL/LGPL-family licenses. That is irrelevant to custom agent chrome: the extension API cannot add panels anyway. Level 3 is core. See [Zed extension limits](constraints/zed-extensions.md).

## See also

- [What not to build](what-not-to-build.md)
- [Overview](overview.md)
- [Synth forks](constraints/synth-fork.md)
