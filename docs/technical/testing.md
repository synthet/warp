---
type: Technical Reference
title: Testing
description: nextest for crate tests, crates/integration for GUI, warp_tui render-to-lines for TUI.
resource: technical/testing.md
tags: [docs, technical, testing]
timestamp: 2026-08-15T00:00:00Z
okf_version: 0.1
---

# Testing

| You say | Where | How |
|---------|-------|-----|
| unit | `*_tests.rs` / `mod_test.rs` beside the module | `cargo nextest run -p <crate>` |
| nextest | workspace | `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2` |
| GUI integration | [`crates/integration`](../../crates/integration/) | `gui-integration-test` skill |
| TUI | [`crates/warp_tui`](../../crates/warp_tui/) | render-to-lines; `tui-testing` skill |

Include test files at the end of the corresponding module:

```rust
#[cfg(test)]
#[path = "filename_tests.rs"] // or "mod_test.rs"
mod tests;
```

Commands and presubmit: [AGENTS.md](../../AGENTS.md). Hub: [TESTING.md](../TESTING.md).
