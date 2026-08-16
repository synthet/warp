---
type: Documentation Index
title: Architecture Index
description: System overview, crate map, entity-handle pattern, and Synth Warp fork boundary.
resource: architecture/INDEX.md
tags: [docs, architecture, index]
timestamp: 2026-08-16T01:20:00Z
okf_version: 0.1
---

# Architecture

Warp-only design notes. Hybrid Zed work lives in the mirrored [zed-warp](../zed-warp/README.md) tree (do not add relative links from that folder back here).

- [system-overview.md](system-overview.md) — GUI vs TUI and the shared core
- [crate-map.md](crate-map.md) — Warp crates a change typically touches
- [entity-handle.md](entity-handle.md) — Entity / `ViewHandle` / `AppContext`
- [synth-fork.md](synth-fork.md) — local-first fork boundary, AppId, telemetry export, OSS sinkhole URLs, cloud-strip compile checks

See also: [ARCHITECTURE.md](../ARCHITECTURE.md), [zed-warp/README.md](../zed-warp/README.md).
