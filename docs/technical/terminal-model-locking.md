---
type: Technical Reference
title: Terminal model locking
description: Nested TerminalModel.lock() calls can deadlock the UI. Pass an already-locked model down the stack.
resource: technical/terminal-model-locking.md
tags: [docs, technical, terminal]
timestamp: 2026-08-15T00:00:00Z
okf_version: 0.1
---

# Terminal model locking

Calling `model.lock()` on `TerminalModel` from overlapping call sites can deadlock and freeze the UI.

Before adding a `model.lock()`:

1. Check that nothing in the current call stack already holds the lock.
2. Prefer passing an already-locked model reference down the stack instead of locking again.
3. Keep the lock scope short. Avoid calling other functions that might lock the same model.

Canonical wording: [AGENTS.md](../../AGENTS.md) (Terminal Model Locking). See also: [architecture/system-overview.md](../architecture/system-overview.md), [TROUBLESHOOTING.md](../TROUBLESHOOTING.md).
