---
title: URI scheme and tab configs — Zed × Warp Hybrid
description: warposs:// and warp:// actions, TabConfig TOML, Zed tasks. Zed → Warp half of Level 1.
---

# URI scheme and tab configs

← [Zed × Warp](../README.md)

How Zed (or any OS URL handler) can open Warp tabs, files, and layouts. Zed → Warp half of [Level 1](../level-1.md).

## Channel schemes `(Warp)`

`ChannelState::url_scheme()` in `crates/warp_core/src/channel/state.rs`:

| Channel | Scheme |
|---------|--------|
| Stable | `warp` |
| Preview | `warppreview` |
| Dev | `warpdev` |
| Local | `warplocal` |
| **Oss (Synth Warp)** | **`warposs`** |

Entry: `uri::handle_incoming_uri` in `app/src/uri/mod.rs`. Synth Zed tasks must use `warposs://` against this fork.

## `…://action/…`

| Path | Behavior |
|------|----------|
| `/new_tab?path=…` | Open path. `~` expanded. **Requires** `path` or it no-ops. |
| `/new_window?path=…` | Same, new window |
| `/open_file_editor?path=ABS&line=&column=` | Open file **in Warp editor** (absolute path required) |

`classify_open_file_action` routes a path to notebook, in-app editor, or “execute in session.” A directory → new session at that cwd. No dedicated “always terminal at this folder” action.

## `…://tab_config/<name>`

Gated by `FeatureFlag::TabConfigs` (Cargo feature `tab_configs`, **in default**).

- Match on filename **stem**, case-insensitive
- On-disk: `~/.warp/tab_configs/` plus defaults
- `?new_window=true` opens a new window
- Open API: `Workspace::open_tab_config`

## TabConfig schema

`app/src/tab_configs/tab_config.rs`: `name`, optional `title`, `color`, `params`; flat `[[panes]]` with `type` (`terminal` | `agent` | `cloud`), `directory`, `commands`, `shell`.

This is the right mapping key for “Zed project ↔ Warp layout”: **directory / worktree**.

Related: `…://launch/<launch_config>`. `…://session/<uuid>` focuses an existing pane — not practical from Zed.

Zed should **not** ingest this TOML as its native launcher format. See [Phase B](../phase-b-launcher.md).

## See also

- [Level 1](../level-1.md)
- [Windows](../constraints/windows.md)
- [Local control](local-control.md)
