---
title: What not to build — Zed × Warp Hybrid
description: Embeddings, editor transplants, extension dashboards, AGPL vendoring, third brokers, and PTY/buffer sync are out of scope.
---

# What not to build

Parent: [Zed × Warp Hybrid](README.md)

These look like integration and are not.

| Idea | Why not |
|------|---------|
| Embed WarpUI in GPUI (or vice versa) | Two UI frameworks, two renderers, [license clash](licenses.md) |
| Transplant Zed `editor` / `project` / LSP into Warp | Pulls most of Zed; worse direction; Warp already has a [native editor](surfaces/native-editor.md) |
| VS Code-style extension that draws a custom agent dashboard | [Extension API](constraints/zed-extensions.md) cannot add panels, badges, or block renderers |
| Copy Warp AGPL sources into Zed | AGPL contagion; reimplement protocols instead |
| Depend on Oz / Warp Drive / cloud Remote Control | Not in the public client; stripped in Synth Warp |
| A third `~/.devsessions` daemon as **source of truth** | Duplicates [local control](surfaces/local-control.md). Cache file optional; live state stays in Warp or Zed |
| Sync full PTYs or file buffers through a bridge | That is embedding; sync [AgentSession](agent-session.md) snapshots only |
| Force ACP and Terminal Threads onto one wire protocol | Unify at AgentSession / sidebar only |
| A separate session-dashboard app | Warp vertical tabs **or** Threads Sidebar + `ThreadItem` |
| Warp Tab Config parser as Zed’s native format | Zed-native launcher in [Phase B](phase-b-launcher.md) |
| Level 2 bridge if Zed is the only agent host | Throwaway; do [Phase A](phase-a-status.md) instead |
| Full block export in Level 2 v1 | Large and UI-entangled; add `last_command` later |

## Extension reality {#extensions}

| Via extension / ACP / tasks | Via core/fork only |
|-----------------------------|--------------------|
| Warp-like MCP integration | Custom Warp-style terminal UI |
| Agent integration via ACP | Custom session dashboard |
| Commands / tasks | Custom agent sidebar chrome |
| Custom language tooling | Terminal block renderer |
| BEL / OSC title inside a Terminal Thread | Terminal Thread status providers |

## See also

- [Overview](overview.md)
- [Licenses](licenses.md)
- [Feasibility](feasibility.md)
