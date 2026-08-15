---
type: Technical Reference
title: App ID and bundle identifiers
description: Synth Warp AppId parse (dotted application name) and io.github.synthet.Warp* bundle IDs.
resource: technical/app-id.md
tags: [docs, technical, app-id, windows]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# App ID and bundle identifiers

SoT: [`crates/warp_core/src/app_id.rs`](../../crates/warp_core/src/app_id.rs). Paths derived from AppId: [`crates/warp_core/src/paths.rs`](../../crates/warp_core/src/paths.rs). Product naming: [architecture/synth-fork.md](../architecture/synth-fork.md).

Do not use `dev.synth.*` or `synth.dev` — that domain is an unrelated live site. The qualifier comes from `synthet.github.io` → `io.github.synthet.Warp`.

## Parse

`AppId` is three dotted parts. `parse` uses `splitn(3, '.')`: qualifier, organization, then the **rest** as `application_name` (may contain further dots).

| String | qualifier | organization | application_name |
|--------|-----------|--------------|------------------|
| `io.github.synthet.Warp` | `io` | `github` | `synthet.Warp` |
| `io.github.synthet.Warp-Local` | `io` | `github` | `synthet.Warp-Local` |
| `io.github.synthet.Warp-Tui` | `io` | `github` | `synthet.Warp-Tui` |

Empty application name after two dots is an error.

## Bundles

| Channel / binary | Identifier |
|------------------|------------|
| OSS GUI (`warp-oss`) | `io.github.synthet.Warp` |
| Local (`warp`) | `io.github.synthet.Warp-Local` |
| Other GUI channels | `io.github.synthet.Warp-*` (Stable, Preview, Dev, …) |
| TUI OSS | `io.github.synthet.Warp-Tui` |

Constructors: `app/src/bin/oss.rs`, `app/src/bin/local.rs`, `crates/warp_tui/src/bin/oss.rs`, `crates/warp_core/src/channel/state.rs` default, `app/Cargo.toml` `package.metadata.bundle.bin.*`. macOS `CFBundleIdentifier` must match.

On macOS, `is_warp_bundle` treats `application_name` starting with `synthet.Warp` as this family ([`app/src/util/file/external_editor/mac.rs`](../../app/src/util/file/external_editor/mac.rs)).

Data/config directories follow the parsed AppId (Windows example: `github\synthet.Warp` under the OS app-data root), not a three-segment `io.github.synthet` only, and not official Warp’s `%APPDATA%\warp\Warp`. Overlaying the OSS GUI onto `C:\Program Files\Warp`: [guides/windows-local-deploy.md](../guides/windows-local-deploy.md). Tests: `crates/warp_core/src/app_id_tests.rs`, `paths_tests.rs`.
