---
type: Verification Report
title: Upstream sync verification — 2026-10-18
description: Fork preservation decisions, verification results, and accepted test failures.
resource: reports/upstream-sync-2026-10-18.md
tags: [docs, reports, upstream, verification]
timestamp: 2026-10-18T00:00:00Z
okf_version: 0.1
---

# Upstream sync — 2026-10-18

Merge commit [`df2e8c2f`](https://github.com/synthet/warp/commit/df2e8c2f5146b5b60c9cc6023e982610e7f94ac5)
incorporates 189 upstream commits from `warpdotdev/warp:master` at
`1e6ef379bc9f51257c4243e8c4396a046cede8af` into fork master
`bdb68c81c9f312efdecbb5ac1009fbfe0f4b4607`. Both histories remain parents of the merge.
On 2026-10-18 the maintainer explicitly authorized advancing master with the failures below
documented. This accepts the verification limits; it does not establish that all tests pass.

Upstream moved to `ae60375ef1d3895908756878fe4a07cbe703ce1a` after the reviewed snapshot.
That later change is outside this approved merge.

## Fork behavior retained

The resolution follows the [fork boundary](../architecture/synth-fork.md): account-free terminal
use, BYOK permission, disabled hosted quota/purchase gates, cloud/Drive restrictions, local telemetry
policy, disabled automatic updates, branding, Windows hooks, themes, and canonical agent assets.
All 73 paths changed by both histories were audited for lost fork guards and stale symbols.

Upstream API changes retain the fork's cloud guard in the new LLM-refresh helper, pricing-singleton
guards, empty credit catalogs, and BYOK guidance. The buy-credits banner and upstream feature-flag
cleanup workflow remain removed. Public macOS runners use upstream's new split output keys.
The OSS feature filter also excludes the newly introduced ChatGPT subscription and cloud execution,
shell respawn, deferred repository, and preview URL flags.

The separate remote-control development branch remains separate. Six verification fixes present
when the sync began and later committed as `9f51e487` were carried into this merge.

## Verification

The following Windows checks passed using a custom Cargo target directory and one build job:

- `cargo check --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets --tests --locked -- -D warnings`
- `cargo clippy -p warp --all-targets --tests --locked -- -D warnings`
- `script/format` and `script/format --check`
- Assistant-tree synchronization, macOS workflow output validation, and focused installer lint.
- Four source-extracted Rust policy tests using production definitions and repository assertions.
  The two extended OSS assertions failed before the filter changes and passed afterward.

`cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2 --locked` ran
**12,744 tests: 12,429 passed, 315 failed, 125 skipped**. The excluded legacy package is absent
from this workspace. There were one slow and five leaky passing tests.

Of the failures, 275 were GUI integration tests. Windows CI excludes `package(integration)` from its
unit-test run and runs the GUI integration steps on other platforms. Rerunning the 40 failed unit
tests with the CI profile and an explicit PowerShell path yielded **3 passed and 37 failed** (exit
100). Failures include Windows symlink privilege errors, an assertion expecting English OS error
text on a Russian installation, MCP/completion timeouts, remote file-operation expectations, and
cloud-dispatch/handoff expectations. Of those 37 test functions, 24 are byte-for-byte unchanged
from fork master and 13 are new upstream tests; this comparison does not prove baseline runtime
results. The disabled-cloud production guards were retained.

The tracing export test passed separately with `RUST_LOG=info`; the inherited `RUST_LOG=warn`
had suppressed the INFO spans that its assertion expects. No source change was needed.

Other limits remain: unchanged agent-rule frontmatter and PowerShell build/deploy lint failures;
an existing inline Rust test module; and two factory-files checks requiring symlink privilege or
Unix executable invocation. Linux/macOS cloud verification was unavailable because `oz-dev` was
not installed. No assertion, lint policy, or fork boundary was weakened to obtain a pass.
