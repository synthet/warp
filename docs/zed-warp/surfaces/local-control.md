---
title: Local control — Zed × Warp Hybrid
description: warpctrl IPC. Extend this catalog for Level 2. Do not add a competing ~/.devsessions daemon.
---

# Local control (`warpctrl`)

← [Zed × Warp](../README.md)

The existing IPC surface that [Level 2](../level-2.md) should extend. Operator spec: `(Warp) specs/warp-control-cli/README.md`.

## Layers `(Warp)`

| Layer | Path |
|-------|------|
| Protocol, catalog, discovery | `crates/local_control/` |
| App server + bridge | `app/src/local_control/` |
| CLI parsing | `crates/warp_cli/src/local_control/` |

Transport: owner-only discovery record → Unix-socket credential broker → `POST /v1/control` on `127.0.0.1` with a short-lived action-scoped bearer.

Gates: `FeatureFlag::WarpControlCli` **and** Settings → Scripting. Scripting defaults **off** on public channels (Stable, Preview, OSS) and **on** for dogfood.

## Feature flags (Synth Warp)

- Cargo feature `warp_control_cli` is defined in `app/Cargo.toml` but **not** in `default`.
- `FeatureFlag::WarpControlCli` is in `DOGFOOD_FLAGS` and when the Cargo feature is on (`app/src/features.rs`).
- OSS `warp-oss` therefore does **not** ship local control unless you add the feature or flag.

## Windows

`local_control_publication_supported()` is `cfg!(not(target_os = "windows"))` in `app/src/local_control/mod.rs`. The broker is a Unix socket. Level 2 on Windows requires a named-pipe (or AF_UNIX) broker plus ACLs. See [Windows](../constraints/windows.md).

## Catalog that already helps a hybrid

From `crates/local_control/src/catalog.rs`: `instance.*`, `window.*`, `tab.*`, `pane.*`, `session.*`, `input.insert` / `input.replace`, `file.open`, surface toggles (vertical tabs, agent management, …).

Useful today (when enabled, non-Windows):

```bash
warpctrl file open /abs/file.rs --line 42 --column 8 --new-tab
warpctrl tab create --type terminal
warpctrl input insert "some text"
warpctrl instance list
```

### Limits

- `TabCreateParams` is `{ tab_type }` only — **no cwd**
- `file.open` always opens Warp’s in-app editor
- `session.inspect` returns pane ids / `is_active` — **not** agent, cwd, or status (`app/src/local_control/handlers/metadata.rs`)
- No `agent.*` actions, no SSE, no block export, no notification mailbox

## What Level 2 should add

See [Level 2](../level-2.md): `agent.session.list` / `inspect`, `agent.notification.list`, `tab.create` cwd, later an event stream. Reuse discovery + credential broker.

## See also

- [AgentSession](../agent-session.md)
- [CLI agent sessions](cli-agent-sessions.md)
