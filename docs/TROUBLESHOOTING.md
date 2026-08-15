---
type: Documentation Hub
title: Troubleshooting
description: Pointers for build failures, resume compiles, and fork-behavior questions.
resource: TROUBLESHOOTING.md
tags: [docs, troubleshooting, hub]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# Troubleshooting

Thin hub. Failure log: [AGENTS.md](../AGENTS.md) (RCA table).

- Windows OOM (`os error 1455` / page file), rustc `STATUS_STACK_BUFFER_OVERRUN`, resume builds, package-cache lock (including another repo’s cargo), PowerShell `NativeCommandError` from cargo stderr, hung waiter after a successful link: [guides/build-and-run.md](guides/build-and-run.md)
- Overlaying this fork onto `C:\Program Files\Warp` (do not use `CHANNEL=stable`; do not uninstall official Warp): [guides/windows-local-deploy.md](guides/windows-local-deploy.md)
- Bundle ID / AppId parse (`io.github.synthet.Warp*`): [technical/app-id.md](technical/app-id.md)
- What this fork includes and excludes: [FAQ.md](../FAQ.md)
- Deadlocks from terminal model locks: [technical/terminal-model-locking.md](technical/terminal-model-locking.md)
- Compile errors after stripping cloud/billing UI (`false` in a `FeatureFlag` array, `add_typed_action_view` without `TypedActionView`, `Appearance::as_ref` without `SingletonEntity`): [architecture/synth-fork.md](architecture/synth-fork.md), [technical/feature-flags.md](technical/feature-flags.md)
- Do not re-run framework `bootstrap.py --force` against this checkout: [ai-workflow/README.md](ai-workflow/README.md)
