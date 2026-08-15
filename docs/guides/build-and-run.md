---
type: Runbook
title: Build and run
description: Bootstrap, run GUI and TUI, optional local warp-server, and Windows resume builds.
resource: guides/build-and-run.md
tags: [docs, guides, build, windows]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# Build and run

Canonical command list: [AGENTS.md](../../AGENTS.md). Do not treat this page as a second command catalog.

| Goal | Command |
|------|---------|
| Platform setup | `./script/bootstrap` |
| GUI | `./script/run` / `cargo run` |
| TUI | `./script/run-tui` |
| Presubmit | `./script/presubmit` |

Optional local warp-server: `WITH_LOCAL_SERVER=1 ./script/run`. Override `SERVER_ROOT_URL` and `WS_SERVER_URL` when the server is not on port 8080.

Warp's launch script is `script/` (singular). Framework helpers live in `scripts/` (plural). Do not replace `script/` with a framework bootstrap.

## Windows toolchain

SoT for install: [`script/windows/bootstrap.ps1`](../../script/windows/bootstrap.ps1). The resume script reuses the same PATH pieces without reinstalling.

Required to link `x86_64-pc-windows-msvc`:

- Rust from [`rust-toolchain.toml`](../../rust-toolchain.toml) (currently **1.92.0**), not whatever `stable` rustup default is
- VS 2022 Build Tools: MSVC x64 + Windows SDK (`VsDevCmd.bat -arch=amd64`)
- LLVM `libclang.dll` (`LIBCLANG_PATH`)
- `protoc`, CMake
- Git for Windows `usr\bin` (`patch.exe`); put MSVC `link.exe` ahead of Git’s `link.exe`

## Windows resume compile

SoT: [`script/windows/build.ps1`](../../script/windows/build.ps1) (wrappers: `.\script\build`, `.\script\build.cmd`).

Default: `cargo build -p warp --bin warp-oss --features gui`. Artifact: `target\debug\warp-oss.exe`. Auto job count is `floor(freeRAM_GB / 6)` capped at **2**. A 32 GB machine still OOMs if cargo uses default crate parallelism or another `rustc` tree is running.

| File | Role |
|------|------|
| `target\synth-build-resume.json` | Last attempt, jobs, exit, resume command |
| `target\synth-build.log` | Command line and `cargo exit N` (not a full rustc transcript) |

Re-run the same command to resume. Incremental artifacts in `target\` are reused unless `-Clean` / `--clean`.

Rules:

- Do not start a second `cargo` against this tree while one is already compiling.
- Another `cargo` on the machine (this workspace’s rust-analyzer, **or a different checkout such as Zed**) can hold the global crates.io package-cache lock (`Blocking waiting for file lock on package cache`). Wait or stop that process; do not treat the wait as a compile error. Compiling Warp and Zed at once on 32 GB commonly exhausts the page file (`os error 1455`).
- Invoke cargo as `& cargo @args` (what the script does). Do not pipe `2>&1` into `ForEach-Object` (PowerShell turns cargo stderr — including the lock-wait line — into a terminating `NativeCommandError`). Do not `Start-Process -NoNewWindow -Wait` (can hang after a successful link while the exe is already on disk). Piping `cmd /c cargo … 2>&1` also buffers rustc output and can report `exit -1`.
- Monitor progress with `Get-Process cargo,rustc` and whether `target\debug\warp-oss.exe` appears. Tailing the resume log is not enough. Linking the GUI debug binary can take many minutes after the `warp` lib reports `Finished`.

OOM / rustc crash: the script retries with fewer jobs. Real compile errors (`exit 101`) are not retryable.

See also: [windows-local-deploy.md](windows-local-deploy.md) (overlay onto `C:\Program Files\Warp`), [DEVELOPMENT.md](../DEVELOPMENT.md), [TROUBLESHOOTING.md](../TROUBLESHOOTING.md).
