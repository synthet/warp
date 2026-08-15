---
title: Zed extension limits — Zed × Warp Hybrid
description: MCP, ACP, and tasks work. Custom terminal UI, session dashboard, and agent sidebar chrome require Zed core or a fork.
---

# Zed extension limits

← [Zed × Warp](../README.md)

You cannot implement the full Warp-like UI as a normal Zed extension. That is why [Level 2](../level-2.md) is MCP/tasks and [Level 3](../level-3.md) is a Zed fork.

## What extensions can do today

| Capability | Fit for this hybrid |
|------------|---------------------|
| Languages, grammars, snippets | Unrelated |
| Debuggers | Unrelated |
| Themes | Unrelated |
| MCP servers | **Yes** — wrap `agent.session.list` so Zed agents see Warp sessions |
| ACP external agents | **Yes** — complementary, not a Warp UI port |
| Tasks / commands | **Yes** — `warposs://` / `warp://` open tab |

## What they cannot do

Custom terminal renderer, session dashboard, agent sidebar/badges, VS Code-style webview panels. See `(Zed) docs/src/extensions/developing-extensions.md`.

A status-file or MCP resource is the maximum without forking. A panel that *looks* like Warp’s vertical tabs is [Phase A](../phase-a-status.md).

## See also

- [Level 2](../level-2.md)
- [What not to build](../what-not-to-build.md)
- [Licenses](../licenses.md)
