---
type: Runbook
title: Windows local deploy
description: Overlay Synth Warp OSS onto C:\Program Files\Warp without Inno Setup or the stable channel.
resource: guides/windows-local-deploy.md
tags: [docs, guides, windows, deploy]
timestamp: 2026-08-16T01:15:00Z
okf_version: 0.1
---

# Windows local deploy

SoT: [`script/windows/deploy.ps1`](../../script/windows/deploy.ps1). Operator notes: [`script/windows/README.md`](../../script/windows/README.md). Compile/resume: [build-and-run.md](build-and-run.md).

Repeatable file-copy overlay onto the live official install at `C:\Program Files\Warp\warp.exe`. Start Menu shortcuts and `unins000.*` stay. Do **not** build `CHANNEL=stable`: that binary auto-updates from Warp.dev and would overwrite the fork.

```powershell
.\script\windows\deploy.ps1              # build release OSS, overlay, restart
.\script\windows\deploy.ps1 -SkipBuild   # copy last warp-oss.exe only
.\script\windows\deploy.ps1 -Debug       # faster non-release overlay
```

`build.ps1 -Release` defaults to `--features gui`. `deploy.ps1` defaults to `--features release_bundle,gui`, so a full deploy **rebuilds** `warp` / `warp_assets` even when `target\release\warp-oss.exe` already exists. Use `-SkipBuild` to overlay that last exe. Run a full `deploy.ps1` when the overlay should include the `release_bundle` feature set.

`script/windows/deploy.cmd` is a double-click wrapper. UAC runs only for the copy into Program Files.

## Identity

| Piece | Official Warp | Default OSS bundle | After `deploy.ps1` |
|-------|---------------|--------------------|--------------------|
| Folder / exe | `C:\Program Files\Warp\warp.exe` | `{autopf}\WarpOss\warp-oss.exe` | Official folder / `warp.exe` |
| Channel / scheme | Stable / `warp://` | Oss / `warposs://` | Oss / `warposs://` |
| Installer AppId | `warp-terminal-stable` | `warp-terminal-oss` | Unchanged official uninstaller |
| User data | `%APPDATA%\warp\Warp` | `io.github.synthet.Warp` | OSS app id (settings do **not** migrate) |
| Auto-update | Warp.dev installer | None (`autoupdate_config: None`) | None |

OSS URI parsing rejects `warp://` ([`app/src/uri/mod.rs`](../../app/src/uri/mod.rs)). Deploy rewrites Explorer Directory shell commands that still launch `Warp://` to `warposs://`. It does not register a fake `warp-terminal-stable` AppId.

## Do not

- Uninstall official Warp as part of this loop. Inno `[UninstallDelete]` wipes `%APPDATA%\warp\Warp` and removes the shortcuts the overlay uses.
- Run [`script/windows/bundle.ps1`](../../script/windows/bundle.ps1) for the inner loop. That is the shipping Inno path (`rlto` + ISCC, default dir `WarpOss`).
- `/MIR` the whole install directory (would delete `unins000.*`). Deploy `/MIR`s only `resources\`.
- Reinstall official Warp into the same folder afterward. Pin winget if Warp came from it (`winget list Warp`, then `winget pin add`).

## Payload

Same files as [`script/windows/windows-installer.iss`](../../script/windows/windows-installer.iss) (`warp.exe`, ConPTY, OpenConsole, VC runtime, DXC, icon, `pwsh.ps1`, `resources\`). Missing DLLs look installed but fail at PTY or GPU.

If `cargo about` is missing, deploy still copies `resources\bundled` and skips license/schema generation.

Fork identity (app id `io.github.synthet.Warp`, scheme `warposs`): [architecture/synth-fork.md](../architecture/synth-fork.md). AppId parse and data dirs: [technical/app-id.md](../technical/app-id.md).
