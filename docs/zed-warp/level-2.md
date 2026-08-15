---
title: Level 2 Bridge — Zed × Warp Hybrid
description: Warp exports AgentSession over local_control. Zed consumes via poll or MCP. No third daemon as source of truth.
---

# Level 2 — Bridge (Warp export, Zed consume)

Parent: [Zed × Warp Hybrid](README.md) · Prev: [Level 1](level-1.md) · Next: [Level 3](level-3.md)

A process boundary is the right **interop** design. A third `~/.devsessions` service as **source of truth** is not — it duplicates `(Warp) crates/local_control`.

**When to skip Level 2:** Synth Zed is the only shell for agents. Invest in [Phase A](phase-a-status.md) instead; a Warp export would be throwaway.

```text
┌─────────┐                 ┌─────────┐
│   Zed   │                 │  Warp   │
│ editor  │                 │ agents  │
└────┬────┘                 └────┬────┘
     │  MCP / tasks / poll       │
     └──────────┬────────────────┘
                │
         local_control
         agent.session.*
```

Warp already tracks everything the 5–15 session workflow needs **in-process**. Nothing outside the process can read it today. Details: [local control](surfaces/local-control.md), [CLI agent sessions](surfaces/cli-agent-sessions.md).

## Synchronize only {#sync}

- repo, worktree, branch, cwd
- active file+line (**Zed authoritative** when dual-app)
- Warp tab/pane id ↔ Zed project/worktree
- agent status / attention / last query
- optional: “focus this session” deep links (`synthzed://agent/…`, `warposs://tab_config/…`)

## Do not synchronize {#do-not-sync}

- Editor buffers, LSP state, raw PTY bytes (that is embedding)
- Full `SerializedBlock` streams in v1 (large)

See [What not to build](what-not-to-build.md).

## Warp-side work {#warp}

### 2a. Turn local_control on for OSS and Windows

- Add `warp_control_cli` to OSS default features, or enable `FeatureFlag::WarpControlCli` in Synth `enabled_features()`.
- Windows: named pipe (or AF_UNIX) for the credential broker, plus ACL equivalent of `0600`. Until this lands, Level 2 cannot run on Windows. See [Windows](constraints/windows.md).
- Keep Settings → Scripting as the user gate (off by default on public channels).

### 2b. New catalog actions

Follow `(Warp) crates/local_control/src/catalog.rs`. Names illustrative:

| Action | Purpose |
|--------|---------|
| `agent.session.list` | All CLI-agent sessions as [AgentSession](agent-session.md) JSON |
| `agent.session.inspect` | One session + pane/tab/window ids |
| `agent.session.focus` | Optional: activate that pane |
| `agent.notification.list` | Complete / Request / Error |
| `tab.create` + `cwd` / `path` | Fixes `TabCreateParams` = `{ tab_type }` only |

`input.insert` / `input.replace` already exist (they do not submit). Permission *approve* is not a local_control action today.

Prefer CLI sessions as the Synth source of truth. Do not require cloud Oz or Drive.

### 2c. Push vs poll

- **v1:** Zed (or an MCP server) polls `agent.session.list` every 1–2s.
- **v2:** SSE or named-pipe event stream mirroring `CLIAgentSessionsModelEvent`.

### 2d. Optional disk cache

`~/.synth/sessions/*.json` on status change helps when Warp is down. **Cache only.** Same snapshot schema as AgentSession so a future Zed-only host and a dual-app bridge speak one model.

## Zed-side work {#zed}

Not Warp-checkout work. Extensions **cannot** add a Warp-style sidebar; they **can** do MCP, ACP, tasks. See [Zed extension limits](constraints/zed-extensions.md).

1. MCP server wrapping `warpctrl agent.session.list` so Zed agents see sibling sessions.
2. Tasks/keybindings for `warposs://tab_config/…` and “focus Warp pane for this worktree.”
3. Optional later: `synthzed://agent/…` focus links.
4. A panel that *looks* like Warp vertical tabs is already [Level 3](level-3.md) / [Phase A](phase-a-status.md) UI.

**License:** Zed talking to Warp over HTTP/IPC does not AGPL Zed. See [Licenses](licenses.md).

## Why the common model still matters {#why}

Even if you never ship the bridge, defining [AgentSession](agent-session.md) once means Level 3’s sidebar and a dual-app export speak the same snapshot.

## See also

- [Local control](surfaces/local-control.md)
- [AgentSession](agent-session.md)
- [Feasibility](feasibility.md)
