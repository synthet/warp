---
type: Technical Reference
title: Warp crate map
description: Warp-only map of crates a change typically touches. Distinct from the hybrid zed-warp crate map.
resource: architecture/crate-map.md
tags: [docs, architecture, crates]
timestamp: 2026-08-15T00:00:00Z
okf_version: 0.1
---

# Warp crate map

Parent: [architecture index](INDEX.md). This is **Warp-only**. For Zed+Warp hybrid landing spots, use [zed-warp/crate-map.md](../zed-warp/crate-map.md) (that folder is mirrored and must not link back here).

| Path | Role |
|------|------|
| [`app/`](../../app/) | GUI desktop app plus feature surfaces the TUI reuses (terminal, AI, settings, workspace, `local_control` handlers) |
| [`crates/warp_core`](../../crates/warp_core/) | Shared utilities and platform abstractions; re-exports feature flags |
| [`crates/warp_features`](../../crates/warp_features/) | `FeatureFlag` enum and rollout lists |
| [`crates/warpui`](../../crates/warpui/), [`crates/warpui_core`](../../crates/warpui_core/) | Shared UI core; GUI elements plus TUI cell-grid |
| [`crates/warp_tui`](../../crates/warp_tui/) | Headless TUI front-end |
| [`crates/editor`](../../crates/editor/) | Text editing |
| [`crates/ipc`](../../crates/ipc/) | Inter-process communication |
| [`crates/graphql`](../../crates/graphql/) | GraphQL client |
| [`crates/warp_graphql_schema`](../../crates/warp_graphql_schema/) | API schema source of truth |
| [`crates/persistence`](../../crates/persistence/) | Diesel + SQLite schema and migrations |
| [`crates/settings`](../../crates/settings/) | Settings schema |
| [`crates/local_control`](../../crates/local_control/) | Local control protocol, catalog, discovery |
| [`crates/integration`](../../crates/integration/) | GUI-only integration tests |

Contracts for schema/API/flags: [CANONICAL_SOURCES.md](../CANONICAL_SOURCES.md). Overview: [system-overview.md](system-overview.md).
