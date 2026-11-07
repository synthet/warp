---
type: Report
title: Windows prompt-chip processes (2026-10-26)
description: Why the Kubernetes prompt chip runs kubectl when it is not shown, and why those PowerShell processes stay after Warp exits.
resource: reports/windows-prompt-chip-processes-2026-10-26.md
tags: [docs, reports, windows, prompt, kubectl]
timestamp: 2026-10-26T22:14:00Z
okf_version: 0.1
---

# Windows prompt-chip processes (2026-10-26)

Observed on Windows host ZEPHYR while Warp from this tree was installed at `C:\Program Files\Warp\warp.exe`. Task Manager showed many `pwsh.exe`, `uv.exe`, `python.exe`, and console-host processes. The console hosts are `conhost.exe`, not `cmd.exe`.

## Kubernetes chip

The default prompt includes a **Kubernetes Context** chip ([`prompt.rs`](../../app/src/context_chips/prompt.rs)). Its command is `kubectl config current-context` ([`builtins.rs`](../../app/src/context_chips/builtins.rs)), launched as `pwsh.exe -NoProfile -c ...`. That reads the current context name from the local kubeconfig. It does not contact the cluster.

The chip is drawn only when the input starts with a Kubernetes tool (`kubectl`, `helm`, `kubectx`, `k9s`, and the rest of the list in `ContextChipKind::should_render`). Drawing and execution are separate. `run_chips` does not consult `should_render`, so a prompt that still lists the chip starts `kubectl` on each context refresh even when the label is hidden. The chip is on-demand, not on the 30-second git timer. It runs only when `kubectl` is on `PATH`; if the executable is missing, the chip is disabled and nothing is spawned.

The default agent toolbar and CLI-agent toolbar do not include this chip.

## Disable it

There is no separate kubectl setting. Right-click the prompt, choose **Edit prompt**, and remove **Kubernetes Context**. That stores a custom chip list. The default list puts the chip back.

Removing the chip stops later launches. It does not kill PowerShell processes that are already stuck.

## Why they survive exit

Prompt chips and other `command::async::Command` spawns set `CREATE_NO_WINDOW | CREATE_BREAKAWAY_FROM_JOB` ([`async.rs`](../../crates/command/src/async.rs)). Warp's own process sits in a job with kill-on-close ([`windows.rs`](../../crates/command/src/windows.rs)). Breakaway puts the child outside that job, so closing Warp does not terminate it.

The chip generator has no timeout (`ChipRuntimePolicy::for_shell_generator` passes `None`). A matching fingerprint on the next refresh does not abort the in-flight command. On Windows, `terminate_process_group` is empty ([`local_command_executor.rs`](../../app/src/terminal/model/session/command_executor/local_command_executor.rs)). `kill_on_drop` runs only if the task future is dropped, and background-runtime shutdown uses `shutdown_background()`, which does not wait for that drop.

The same breakaway is intentional for real terminal shells ([`local_tty/windows/mod.rs`](../../crates/warp_terminal/src/local_tty/windows/mod.rs)), so a tab and anything it started (agent CLIs, `uv`, `python`, MCP servers) also stay up after the window closes.

## Snapshot

About 61 `pwsh.exe` processes were still running after their Warp parent had exited. Commands were the git branch list, `git diff --shortstat`, `git symbolic-ref`, and `kubectl config current-context`. One sample, pid 56168, had been in `D:\Projects\hetzner-mcp` for about 1h45m on a single thread with 0.36s of CPU. PowerShell was loaded and `git` was not running.
