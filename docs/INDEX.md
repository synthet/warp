---
type: Documentation Index
title: Documentation Index
description: Index of all documentation pages in this bundle.
resource: INDEX.md
tags: [docs, index]
timestamp: 2026-08-16T04:28:00Z
okf_version: 0.1
---

# Index

## Governance & conventions
- [README.md](README.md) — documentation hub
- [CANONICAL_SOURCES.md](CANONICAL_SOURCES.md) — authority map
- [WIKI_SCHEMA.md](WIKI_SCHEMA.md) — wiki structure & maintenance
- [OKF_ADOPTION.md](OKF_ADOPTION.md) — OKF profile & lint

## Root hubs
- [ARCHITECTURE.md](ARCHITECTURE.md) — system design entry
- [DEVELOPMENT.md](DEVELOPMENT.md) — build and engineering entry
- [TESTING.md](TESTING.md) — test kinds entry
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — build, resume compile, expected OSS runtime warnings, fork questions

## Architecture
- [architecture/INDEX.md](architecture/INDEX.md) — architecture hub
- [architecture/system-overview.md](architecture/system-overview.md) — GUI vs TUI and shared core
- [architecture/crate-map.md](architecture/crate-map.md) — Warp crate map
- [architecture/entity-handle.md](architecture/entity-handle.md) — Entity / `ViewHandle` / `AppContext`
- [architecture/synth-fork.md](architecture/synth-fork.md) — local-first fork boundary, AppId, telemetry export, OSS sinkhole URLs

## Guides
- [guides/INDEX.md](guides/INDEX.md) — guides hub
- [guides/build-and-run.md](guides/build-and-run.md) — bootstrap, Windows toolchain, `warp-oss` resume
- [guides/windows-local-deploy.md](guides/windows-local-deploy.md) — overlay OSS onto `C:\Program Files\Warp`
- [guides/oss-windows-runtime-warnings.md](guides/oss-windows-runtime-warnings.md) — expected logged-out `warp-oss.exe` `[WARN]` lines

## Features
- [features/INDEX.md](features/INDEX.md) — planned vs implemented
- [features/planned/INDEX.md](features/planned/INDEX.md) — hybrid pointers
- [features/implemented/INDEX.md](features/implemented/INDEX.md) — shipped behavior
- [features/implemented/local-first.md](features/implemented/local-first.md) — local-first shipped behavior

## Planning
- [planning/INDEX.md](planning/INDEX.md) — zed-warp roadmap plus conversation-derived fork queue
- [planning/conversation-backlog.md](planning/conversation-backlog.md) — open work from Aug 2026 Cursor chats

## Technical
- [technical/INDEX.md](technical/INDEX.md) — technical hub
- [technical/app-id.md](technical/app-id.md) — `io.github.synthet.Warp*` bundle IDs
- [technical/feature-flags.md](technical/feature-flags.md) — `FeatureFlag` enum
- [technical/testing.md](technical/testing.md) — nextest, GUI integration, TUI
- [technical/terminal-model-locking.md](technical/terminal-model-locking.md) — `TerminalModel.lock()` rule

## Reference, reports, archive
- [reference/INDEX.md](reference/INDEX.md) — generated artifacts
- [agent-asset-inventory.md](agent-asset-inventory.md) — generated command/skill/subagent index
- [reports/INDEX.md](reports/INDEX.md) — point-in-time audits
- [reports/oss-windows-startup-warnings-2026-08-15.md](reports/oss-windows-startup-warnings-2026-08-15.md) — 2026-08-15 `warp-oss.exe` WARN snapshot
- [archive/INDEX.md](archive/INDEX.md) — deprecated pages (OKF lint excluded)

## Zed × Warp hybrid
- [zed-warp/README.md](zed-warp/README.md) — mirrored hybrid wiki (do not add relative links from that folder into the rest of `docs/`)

## Agent workflow
- [ai-workflow/README.md](ai-workflow/README.md) — asset map & SDLC loop
- [EXTERNAL_CLI_REVIEWS.md](EXTERNAL_CLI_REVIEWS.md) — external CLI review setup
- [agent-observability.md](agent-observability.md) — JSONL trace artifacts for agent workflow observability
- [security.md](security.md) — security model & checklist

## Project
- [project/INDEX.md](project/INDEX.md) — project governance index
- [project/00-backlog-workflow.md](project/00-backlog-workflow.md) — backlog/board contract

## Activity
- [log.md](log.md) — append-only wiki log
