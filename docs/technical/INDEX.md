---
type: Documentation Index
title: Technical Reference Index
description: Stable implementation references — flags, AppId, tests, terminal-model locking.
resource: technical/INDEX.md
tags: [docs, technical, index]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# Technical reference

- [feature-flags.md](feature-flags.md) — `FeatureFlag` enum, runtime checks, and `enabled_features()`
- [app-id.md](app-id.md) — `AppId` parse and `io.github.synthet.Warp*` bundle IDs
- [testing.md](testing.md) — nextest, GUI integration, TUI render-to-lines
- [terminal-model-locking.md](terminal-model-locking.md) — `TerminalModel.lock()` deadlock rule

Contracts (API, schema, settings) stay in [CANONICAL_SOURCES.md](../CANONICAL_SOURCES.md). Security checklist: [security.md](../security.md).
