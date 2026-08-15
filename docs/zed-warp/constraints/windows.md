---
title: Windows constraints — Zed × Warp Hybrid
description: Warp→Zed editor missing on Windows. local_control publication off. warposs scheme registration.
---

# Windows constraints

← [Zed × Warp](../README.md)

Two Level 1–2 paths that work on macOS/Linux are **off** on Windows.

## Warp → Zed (external editor)

`(Warp) app/src/util/file/external_editor/windows.rs` probes the registry for VS Code, Cursor, and Windsurf. `Editor::Zed` / `ZedPreview` are omitted from `SUPPORTED_EDITORS` on Windows.

**Patch (Level 1, Warp checkout):** detect `zed.exe` on `PATH` and `%LOCALAPPDATA%\Programs\Zed\zed.exe`; launch `zed path:line:col`; add both editor variants to the Windows list.

Zed → Warp via `warposs://` still works in principle if the protocol is registered.

## `warpctrl` / local_control publication

`local_control_publication_supported()` is `cfg!(not(target_os = "windows"))`. The credential broker is a Unix socket. Specs call out that Windows publication stays disabled until equivalent ACL and broker protections exist (`(Warp) specs/warp-control-cli/README.md`). Until a named-pipe broker lands, [Level 2](../level-2.md) cannot run on Windows even if you enable the Cargo feature.

Workaround for Level 1: OS `start warposs://…`.

## OSC 777 plugins

Many Warp plugins write OSC 777 to `/dev/tty` (Unix-centric). Zed [Phase A](../phase-a-status.md) on Windows should parse PTY output and prefer Zed-native plugins.

## Compiling Warp and Zed together

On a 32 GB Windows host, a Warp GUI debug compile and a Zed `cargo` at the same time typically exhaust the page file (`os error 1455`) and contend for the global crates.io package-cache lock. Run one tree at a time. Warp resume compile: `(Warp) script/windows/build.ps1`. Overlay the Warp OSS GUI onto `C:\Program Files\Warp`: `(Warp) script/windows/deploy.ps1` (do not build Warp `CHANNEL=stable`).

## URL scheme

OSS scheme is `warposs`. Packaging must register that protocol. See [URI and tab configs](../surfaces/uri-and-tab-configs.md).

## See also

- [Level 1](../level-1.md)
- [External editor](../surfaces/external-editor.md)
- [Local control](../surfaces/local-control.md)
