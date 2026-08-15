---
type: Technical Reference
title: Feature flags
description: FeatureFlag lives in warp_features; warp_core re-exports it. Prefer runtime is_enabled() over cfg.
resource: technical/feature-flags.md
tags: [docs, technical, feature-flags]
timestamp: 2026-08-15T16:00:00Z
okf_version: 0.1
---

# Feature flags

Warp uses compile-time `FeatureFlag` variants plus a small runtime plumbing layer.

| Piece | Path |
|-------|------|
| Enum and `DOGFOOD_FLAGS` / `PREVIEW_FLAGS` / `RELEASE_FLAGS` | [`crates/warp_features/src/lib.rs`](../../crates/warp_features/src/lib.rs) |
| Re-export used by app code | [`crates/warp_core/src/features.rs`](../../crates/warp_core/src/features.rs) (`pub use warp_features::*;`) |

Gate paths with `FeatureFlag::YourFlag.is_enabled()`. Prefer that over `#[cfg(...)]` unless the code cannot compile without the flag (missing deps, platform-only modules). Keep flags product-focused; remove them after launch has stabilized.

The app enable list is `enabled_features()` in [`app/src/features.rs`](../../app/src/features.rs). That set is `HashSet<FeatureFlag>`. To keep a flag off, omit the entry. Do not write `false` under a `#[cfg(feature = "...")]` — that does not compile. Cargo features in `app/Cargo.toml` are a separate axis from this runtime set.

How to add a flag, dogfood/preview/release lists, and UI gating: [AGENTS.md](../../AGENTS.md) (Feature Flags). Authority map: [CANONICAL_SOURCES.md](../CANONICAL_SOURCES.md). Fork strip pitfalls: [architecture/synth-fork.md](../architecture/synth-fork.md).
