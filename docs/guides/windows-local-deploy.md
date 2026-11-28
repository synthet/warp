---
type: Runbook
title: Windows local deploy
description: Install Synth Term OSS per user into its own folder, side by side with official Warp.
resource: guides/windows-local-deploy.md
tags: [docs, guides, windows, deploy]
timestamp: 2026-11-19T23:00:00Z
okf_version: 0.1
---

# Windows local deploy

SoT: [`script/windows/deploy.ps1`](../../script/windows/deploy.ps1). Operator notes: [`script/windows/README.md`](../../script/windows/README.md). Compile/resume: [build-and-run.md](build-and-run.md).

Repeatable per-user install into `%LOCALAPPDATA%\Programs\SynthTerm\warp-oss.exe`, side by side with
any official Warp install. Deploy never writes to `C:\Program Files\Warp`, never stops official
`warp.exe`, and never edits Warp's registry entries. Do **not** build `CHANNEL=stable`: that binary
auto-updates from Warp.dev.

```powershell
.\script\windows\deploy.ps1              # build release OSS, install, launch
.\script\windows\deploy.ps1 -SkipBuild   # install the last warp-oss.exe only
.\script\windows\deploy.ps1 -Debug       # faster non-release build
.\script\windows\deploy.ps1 -WhatIf      # print the plan, change nothing
```

`deploy.ps1` defaults to `--features release_bundle,gui`, so a full deploy **rebuilds** `warp` /
`warp_assets` even when `target\release\warp-oss.exe` already exists. Use `-SkipBuild` to install that
last exe. `script/windows/deploy.cmd` is a double-click wrapper; no UAC prompt is involved.

Deploy refuses a non-empty `-InstallDir` that has no `warp-oss.exe` unless `-Force` is passed, so it
cannot be pointed at the official install by accident.

## Deploying from a terminal inside Synth Term

Before the copy, deploy force-stops every process started from the Synth Term folder and waits up to
30s for `warp-oss.exe` to unlock. When the shell running deploy is itself hosted by that install, the
deploy ends its own session mid-run. Official Warp tabs are not affected.

Check the ancestry first:

```powershell
$p = Get-CimInstance Win32_Process -Filter "ProcessId=$PID"
while ($p) {
    "{0} (pid {1}) {2}" -f $p.Name, $p.ProcessId, $p.ExecutablePath
    $p = Get-CimInstance Win32_Process -Filter "ProcessId=$($p.ParentProcessId)" -ErrorAction SilentlyContinue
}
```

If `%LOCALAPPDATA%\Programs\SynthTerm\warp-oss.exe` is in that chain, split the run in two:

1. Build from inside Synth Term, with deploy's feature set:
   `.\script\windows\build.ps1 -Release -Bin warp-oss -Features "release_bundle,gui"`
2. Install from a shell the install does not host (official Warp, Windows Terminal, conhost, or
   `script\windows\deploy.cmd` double-clicked): `.\script\windows\deploy.ps1 -SkipBuild`

## Verify what is installed

The exe carries the fork version and its build commit. Without `GIT_RELEASE_TAG` (deploy never sets it), `app/build.rs` builds both the About page version and the exe version resource from the repo-root `VERSION` file plus the short commit hash. `app/Cargo.toml` keeps upstream's `0.1.0` and is only the fallback when `VERSION` is missing:

```powershell
(Get-Item "$env:LOCALAPPDATA\Programs\SynthTerm\warp-oss.exe").VersionInfo.FileVersion   # v<VERSION>+<short sha>, e.g. v0.2.0+0e6fd3ac
```

```bash
sha256sum "$LOCALAPPDATA/Programs/SynthTerm/warp-oss.exe" target/release/warp-oss.exe   # equal => install is that build
git log --oneline <short sha>..HEAD                                                    # what the install is missing
```

`icon.ico` in the install dir is only the shortcut icon. The window and taskbar use the icon resource
compiled into `warp-oss.exe` ([`script/compile_icon`](../../script/compile_icon)), so an icon change
needs a rebuild.

## Identity

| Piece | Official Warp | Synth Term (`deploy.ps1`) | Synth Term (Inno `oss` bundle) |
|-------|---------------|---------------------------|--------------------------------|
| Folder / exe | `C:\Program Files\Warp\warp.exe` | `%LOCALAPPDATA%\Programs\SynthTerm\warp-oss.exe` | `{autopf}\WarpOss\warp-oss.exe` |
| Channel / scheme | Stable / `warp://` | Oss / `warposs://` | Oss / `warposs://` |
| Start menu / AUMID | Warp / `dev.warp.Warp` | Synth Term / `io.github.synthet.Warp` | WarpOss / `dev.warp.WarpOss` |
| Single-instance mutex | `Local\WarpStable_SingleInstance` | `Local\WarpOss_SingleInstance` | `Local\WarpOss_SingleInstance` |
| User data | `%APPDATA%\warp\Warp` | `io.github.synthet.Warp` | `io.github.synthet.Warp` |
| Auto-update | Warp.dev installer | None | None |

OSS URI parsing rejects `warp://` ([`app/src/uri/mod.rs`](../../app/src/uri/mod.rs)); the app registers
`warposs://` itself at startup. Deploy and the Inno bundle share the mutex, so only one of them runs
at a time.

## Payload

Same files as [`script/windows/windows-installer.iss`](../../script/windows/windows-installer.iss)
(`warp-oss.exe`, ConPTY, OpenConsole, VC runtime, DXC, icon, `pwsh.ps1`, `resources\`), plus
`LICENSE-AGPL`, `LICENSE-MIT` and the fork notice when present. Missing DLLs look installed but fail
at PTY or GPU.

Fork identity (app id `io.github.synthet.Warp`, scheme `warposs`): [architecture/synth-fork.md](../architecture/synth-fork.md). AppId parse and data dirs: [technical/app-id.md](../technical/app-id.md).
