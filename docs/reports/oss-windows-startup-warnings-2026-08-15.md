---
type: Report
title: OSS Windows startup warnings (2026-08-15)
description: Point-in-time analysis of warp-oss.exe [WARN] lines from a logged-out Windows GUI session.
resource: reports/oss-windows-startup-warnings-2026-08-15.md
tags: [docs, reports, windows, oss, logging]
timestamp: 2026-08-16T01:20:00Z
okf_version: 0.1
---

# OSS Windows startup warnings (2026-08-15)

Living lookup: [guides/oss-windows-runtime-warnings.md](../guides/oss-windows-runtime-warnings.md).

## Source

Local `target\release\warp-oss.exe` console on Windows. Two launches: first-run onboarding, then restore. Terminal log was not copied to `docs/raw/` (machine-local paths).

## Outcome

The app launched, PowerShell 7 bootstrapped, settings opened, and the process stayed up. Onboarding ran on the first launch and was skipped on restore. No warning in that session was a launch failure.

Worth changing later: log level on empty overlay `render()` (share modal, native modal, suggested-workflow modal). Not a product bug.
