---
title: Level 1 Dual-app workflow — Zed × Warp Hybrid
description: Use Warp for agent/terminal sessions and Zed as the editor. File-link support plus warposs:// / warp:// tasks.
---

# Level 1 — Dual-app workflow

Parent: [Zed × Warp Hybrid](README.md) · Next: [Level 2](level-2.md)

This is the 70–80% path and should stay the daily driver until [Level 3](level-3.md) lands. Core loop needs **no broker**. On Windows, Warp→Zed still needs a small patch.

```text
Warp
├── terminal tabs/panes
├── Claude Code / Codex / OpenCode
├── agent status in vertical tabs
├── rich CLI input
├── session layouts (tab configs)
└── click file/path
        ↓
       Zed
       ├── editor, LSP, debugger, git
       └── project search / review
```

## Warp → Zed {#warp-to-zed}

Settings `code.editor.open_file_editor` → Zed (or Zed Preview). File clicks go through `open_file_path_with_editor`. Details: [external editor](surfaces/external-editor.md).

| OS | How |
|----|-----|
| macOS | `open -b dev.zed.Zed zed://file/PATH:LINE:COL` |
| Linux | Zed binary `PATH:LINE:COL` |
| Windows | **Not implemented.** VS Code / Cursor / Windsurf only. See [Windows](constraints/windows.md). |

Caveat (Warp tracker): some relative paths with `:line` suffixes have historically failed for Zed. Workaround: `zed /path/to/file.txt` from the shell.

There is no hard-coded “Open in Zed” menu item. Zed appears in the chooser when it is in `SUPPORTED_EDITORS` and detected as installed.

### Windows work (Warp checkout)

1. Add `Editor::Zed` / `ZedPreview` to the Windows `SUPPORTED_EDITORS` list.
2. Detect `zed.exe` on PATH and/or `%LOCALAPPDATA%\Programs\Zed\zed.exe` (and Preview).
3. Launch `zed path:line:col`.

## Zed → Warp {#zed-to-warp}

No Warp handler inside Zed is required. Use OS URL open. **Synth Warp OSS scheme is `warposs`, not `warp`.** Upstream Warp uses `warp` / `warppreview`. See [URI and tab configs](surfaces/uri-and-tab-configs.md).

| Goal | Synth Warp | Upstream Warp |
|------|------------|---------------|
| Terminal at project root | `warposs://action/new_tab?path=<abs>` | `warp://action/new_tab?path=<abs>` |
| New window at cwd | `warposs://action/new_window?path=…` | `warp://action/new_window?path=…` |
| Saved multi-pane layout | `warposs://tab_config/<stem>` | `warp://tab_config/<stem>` |
| File in Warp editor | `warposs://action/open_file_editor?path=<abs>&line=&column=` | same host, `warp://` |

`new_tab` / `new_window` **require** `path` or they no-op. A directory opens a session at that cwd; a code file may open the in-app editor. Tab configs: filename **stem**, `~/.warp/tab_configs/`, feature `tab_configs` is in Warp default features.

Pair with one Tab Config per worktree/agent, for example:

| Tab Config | Directory | Pane |
|------------|-----------|------|
| `main` | repo root | Codex |
| `feature-A` | worktree A | Claude Code |
| `tests` | repo root | test runner |

### Suggested Zed task (Windows, Synth Warp)

```json
{
  "label": "Open project in Synth Warp",
  "command": "cmd",
  "args": ["/c", "start", "warposs://action/new_tab?path=${ZED_WORKTREE_ROOT}"],
  "hide": "always"
}
```

macOS: `open "warposs://action/new_tab?path=…"`. Linux: `xdg-open`. For upstream Warp, swap in `warp://`. Bind in the keymap if you use it daily. Scheme registration on Windows is required for `start warposs://…` to hit this fork.

## Do not use `warpctrl` for Level 1 on Windows

Two independent blockers — [local control](surfaces/local-control.md):

1. Cargo feature `warp_control_cli` is **not** in Warp `default`. OSS builds do not enable `FeatureFlag::WarpControlCli`.
2. Publication is hard-off on Windows (`local_control_publication_supported()`).

On macOS/Linux dogfood, `warpctrl file open` / `tab create` work, but `file.open` always opens Warp’s editor and `tab.create` has no cwd.

## What Level 1 does not give you {#limits}

- Zed seeing Warp agent badges
- Warp seeing Zed’s open buffer / selection
- A shared `AgentSession` list

That is [Level 2](level-2.md). If you drop Warp and host agents only in Synth Zed, skip Level 2 and invest in [Phase A](phase-a-status.md).

## See also

- [Feasibility](feasibility.md)
- [What not to build](what-not-to-build.md)
- [Windows](constraints/windows.md)
