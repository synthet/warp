---
title: External editor — Zed × Warp Hybrid
description: How Synth Warp opens a file in Zed or another editor. Warp → Zed half of Level 1.
---

# External editor (Warp → Zed)

← [Zed × Warp](../README.md)

When `code.editor.open_file_editor` is Zed, file clicks resolve to `FileTarget::ExternalEditor(Zed)` and call `open_file_path_with_editor`. This is the Warp → Zed half of [Level 1](../level-1.md).

## Key types `(Warp)`

| Path | Role |
|------|------|
| `app/src/util/file/external_editor/mod.rs` | `Editor`, `SUPPORTED_EDITORS`, `open_file_path_with_editor` |
| `…/settings.rs` | `EditorChoice`, `EditorSettings` |
| `…/mac.rs` | `zed://file` + bundle ID `dev.zed.Zed` |
| `…/linux.rs` | Zed binary `path:line:col` |
| `…/windows.rs` | Registry probe — **no Zed** |
| `app/src/util/openable_file_type.rs` | `FileTarget`, `resolve_file_target` |
| `app/src/workspace/view.rs` | `Workspace::open_file_with_target` |

## Settings

- `code.editor.open_file_editor` — default `SystemDefault`
- `code.editor.open_code_panels_file_editor` — default `Warp`

`$EDITOR` strings `"zed"` / `"zed-preview"` map to `Editor::Zed` / `Editor::ZedPreview` via `TryFrom`.

## Platform support

`SUPPORTED_EDITORS` includes Zed only on macOS and Linux/FreeBSD. Windows list: VS Code, Cursor, Windsurf, JetBrains, etc. Gap: [Windows](../constraints/windows.md).

## Click path

Terminal file links, notebook/markdown links, and workspace `FileTarget::ExternalEditor` all funnel to `open_file_path_with_editor`.

Opening a file *from* Zed *into* Warp uses URIs or `warpctrl file.open`, not this module. Those paths force Warp’s in-app editor. See [URI and tab configs](uri-and-tab-configs.md).

## See also

- [Level 1](../level-1.md)
- [Native Warp editor](native-editor.md)
