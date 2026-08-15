---
title: Phase B Session launcher — Zed × Warp Hybrid
description: A Zed-native analogue of Warp Tab Configs — worktree, agent command, cwd, env — spawned from the New Thread menu.
---

# Phase B — Session launcher / workspace configs

Parent: [Level 3](level-3.md) · Prev: [Phase A](phase-a-status.md)

Warp Tab Configs are TOML: pane splits, startup commands, shell, parameterized inputs, optional `agent` pane type, opened via `warposs://tab_config/<name>`. Schema: [URI and tab configs](surfaces/uri-and-tab-configs.md).

Zed should **not** ingest Warp’s file format. Define a Zed-native config that covers the 5–15 session workflow:

- worktree path
- agent command (`claude` / `codex` / `opencode` / …)
- cwd, env, init command
- optional: how many Terminal Threads to spawn

## Where it lives {#where}

- Persistence can extend `TerminalThreadMetadata` in `(Zed) crates/agent_ui/src/terminal_thread_metadata_store.rs` rather than a new dashboard app
- UI: Agent Panel **New Thread** menu + a command to spawn a named layout
- User/project file: e.g. `.zed/agent-sessions.toml` or a setting object — exact schema TBD at implementation time
- Deep link (optional later): `synthzed://agent-session/<name>` mirroring Warp’s URI

ACP agents stay on the ACP path (`agent_servers` / Registry). Do **not** force Claude-via-ACP and Claude-via-TUI into one protocol. Unify them only at the [AgentSession](agent-session.md) / sidebar layer.

Today’s closest knobs are fragmented:

| Existing | Covers |
|----------|--------|
| `agent.terminal_init_command` | One global command for every new Terminal Thread |
| `agent_servers` settings / Registry | ACP agent roster |
| Tasks | Shell commands, reveal/hide |
| Workspace DB + agent panel serialize | Last active thread/terminal, docks |
| Linked worktrees | Isolation, not a launcher roster |

Phase B is the missing **named roster**: “open this project’s four agent sessions.”

## Relation to Level 1 {#level-1}

Until Phase B ships, [Level 1](level-1.md) uses Warp Tab Configs as the launcher and Zed as the editor. After Phase B, Synth Zed can launch the same roster internally.

## See also

- [Phase C](phase-c-blocks.md)
- [Crate map](crate-map.md)
