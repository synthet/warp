---
type: Reference
title: App Icons
description: Catalog of every app icon asset in the repo, what consumes each one, and how to change them.
resource: reference/app-icons.md
tags: [icons, branding, channels, packaging, windows, macos, linux]
timestamp: 2026-08-18T00:00:00Z
okf_version: 0.1
---

# App icons

Every app-icon asset in this checkout, grouped by the system that consumes it.

There are **three independent icon systems** — changing one does not change the others:

| System | Assets | Platforms | Picked by |
|--------|--------|-----------|-----------|
| [Channel raster icons](#1-channel-raster-icons) | `icon.ico`, `<size>.png` | Windows, Linux | Build channel (compile time) |
| [macOS adaptive bundles](#2-macos-adaptive-icon-bundles) | `AppIcon.icon/` | macOS 26+ | Build channel (compile time) |
| [Dock icon variants](#3-macos-dock-icon-variants) | `DockTilePlugin/Resources/*.png` | macOS only | User setting (runtime) |

> **Glyph note.** `local`, `oss`, and `warp-oss` carry the Synth fork glyph; the rest are inherited
> Warp brand assets, and some still have the design guideline grid baked into the raster (visible as
> circles and diagonals on `dev` and `preview`) — see [Known issues](#known-issues).

## 1. Channel raster icons

`app/channels/<channel>/icon/no-padding/`

<table>
<tr>
<td align="center"><img src="../../app/channels/stable/icon/no-padding/128x128.png" width="96"><br><b>stable</b></td>
<td align="center"><img src="../../app/channels/preview/icon/no-padding/128x128.png" width="96"><br><b>preview</b></td>
<td align="center"><img src="../../app/channels/dev/icon/no-padding/128x128.png" width="96"><br><b>dev</b></td>
<td align="center"><img src="../../app/channels/local/icon/no-padding/128x128.png" width="96"><br><b>local</b></td>
<td align="center"><img src="../../app/channels/oss/icon/no-padding/128x128.png" width="96"><br><b>oss</b> / <b>warp-oss</b></td>
</tr>
</table>

| Channel | Bin name | Bundle ID | PNG sizes present | `icon.ico` |
|---------|----------|-----------|-------------------|------------|
| `stable` | `stable` | `dev.warp.Warp` | 16, 32, 48, 64, 128, 256, 512 | 6 entries (adds 128) |
| `preview` | `preview` | `dev.warp.WarpPreview` | 16, 32, 48, 64, 128, 256, 512 | 5 entries |
| `dev` | `dev` | `dev.warp.WarpDev` | 16, 32, 48, 64, 128, 256, 512 | 5 entries |
| `local` | `warp` | `dev.warp.WarpLocal` | 16, 32, 48, 64, 128, 256, 512 | 5 entries |
| `oss` | — | `dev.warp.WarpOss` | 16, 32, 48, 64, 128, 256, 512 | 5 entries |
| `warp-oss` | `warp-oss` | — | 16, 32, 48, 64, 128, 256, 512 | 5 entries |

Every `.ico` packs 16/32/48/64/256 as 32-bit BMP except the 256 entry, which is embedded PNG;
`stable` additionally carries 128.

### Which channel your build uses

[`script/run`](../../script/run) probes for `warp-channel-config` on `PATH`:

| Condition | Bin name | Channel |
|-----------|----------|---------|
| `warp-channel-config` found | `warp` | `local` |
| not found (typical for this fork) | `warp-oss` | `oss` |

### Consumers

**Windows — executable, window, and taskbar icon.**
[`app/build.rs:527`](../../app/build.rs) (`embed_resource_file`) resolves
`channels/<CARGO_BIN_NAME>/icon/no-padding/icon.ico`, copies it next to a generated `resource.rc`,
and declares it as `IDI_ICON`. At runtime
[`crates/warpui/src/windowing/winit/window.rs:1369`](../../crates/warpui/src/windowing/winit/window.rs)
reads it back with `Icon::from_resource(IDI_ICON, None)` and assigns it to both the window and the
taskbar. One file drives all three surfaces.

Because the lookup keys on `CARGO_BIN_NAME`, an OSS build reads `channels/warp-oss/`, **not**
`channels/oss/`. If the directory is missing the build emits a `cargo:warning` and ships with no
icon rather than failing.

**Windows — installers.** [`windows-installer.iss:63,104`](../../script/windows/windows-installer.iss)
and [`tui-installer.iss:59,82`](../../script/windows/tui-installer.iss) use
`{#ReleaseChannel}/icon/no-padding/icon.ico` as both `SetupIconFile` and an installed payload file.
[`deploy.ps1:218-220`](../../script/windows/deploy.ps1) tries `oss` first, then falls back to
`stable`.

**Linux — hicolor theme.** [`script/linux/bundle_install:38-44`](../../script/linux/bundle_install)
installs `16x16 32x32 64x64 128x128 256x256 512x512` into
`/usr/share/icons/hicolor/<size>/apps/<bundle-id>.png`. Missing sizes are skipped silently, so
`local` and `oss` currently install a single 512 icon. Note the loop **skips 48x48** even though the
`.ico` uses it.

**Linux — desktop entry.** `app/channels/<channel>/<bundle-id>.desktop` carries
`Icon=<bundle-id>`, which resolves through the hicolor theme above.

**Cargo packaging.** [`app/Cargo.toml:1054`](../../app/Cargo.toml) includes only the `oss` 512 PNG
and `.ico` in the published package.

## 2. macOS adaptive icon bundles

`app/channels/<channel>/icon/AppIcon.icon/` — Apple's layered `.icon` format (macOS 26 / Xcode 26),
which supplies the tinted, translucent, and dark variants the system composites at runtime.

[`script/compile_icon`](../../script/compile_icon) runs `xcrun actool --app-icon AppIcon` over the
bundle, writes `Assets.car` into the app bundle's `Resources/`, and sets `CFBundleIconFile` /
`CFBundleIconName` to `AppIcon`. The `oss` channel has no bundle — the script warns and skips
instead of erroring.

| Channel | Layers (`icon.json`) | Source assets |
|---------|----------------------|---------------|
| `stable` | `Glyph (1).svg` (multiply) + `4-2.png` (overlay), specular, individual lighting, blue auto-gradient fill | `Glyph (1).svg`, [`4-2.png`](../../app/channels/stable/icon/AppIcon.icon/Assets/4-2.png) |
| `preview` | Single `Preview.png` layer, blue auto-gradient fill | [`Preview.png`](../../app/channels/preview/icon/AppIcon.icon/Assets/Preview.png) |
| `dev` | `Glyph.svg` + `Guidelines.svg` + `PLACE HERE.png`, `system-dark` fill | [`Glyph.svg`](../../app/channels/dev/icon/AppIcon.icon/Assets/Glyph.svg), [`Guidelines.svg`](../../app/channels/dev/icon/AppIcon.icon/Assets/Guidelines.svg), `PLACE HERE.png` |
| `local` | Single glass layer, gray linear-gradient fill with a dark-appearance specialization | `warp-glyph 3.svg` |
| `oss` | *(no bundle)* | — |

All four bundles declare `squares: shared` and `circles: [watchOS]`, with a neutral 0.5 shadow and
0.5 translucency.

## 3. macOS dock icon variants

`app/DockTilePlugin/Resources/` — 20 PNGs, all 1024x1024. These back the user-facing icon picker in
Settings; they are **mac-only** and have no effect on Windows or Linux.

<table>
<tr>
<td align="center"><img src="../../app/DockTilePlugin/Resources/warp_2.png" width="72"><br><code>warp_2</code><br>Default</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/aurora.png" width="72"><br><code>aurora</code><br>Aurora</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/classic_1.png" width="72"><br><code>classic_1</code><br>Classic 1</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/classic_2.png" width="72"><br><code>classic_2</code><br>Classic 2</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/classic_3.png" width="72"><br><code>classic_3</code><br>Classic 3</td>
</tr>
<tr>
<td align="center"><img src="../../app/DockTilePlugin/Resources/comets.png" width="72"><br><code>comets</code><br>Comets</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/cow.png" width="72"><br><code>cow</code><br>Cow</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/glass_sky.png" width="72"><br><code>glass_sky</code><br>Glass Sky</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/glitch.png" width="72"><br><code>glitch</code><br>Glitch</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/glow.png" width="72"><br><code>glow</code><br>Glow</td>
</tr>
<tr>
<td align="center"><img src="../../app/DockTilePlugin/Resources/holographic.png" width="72"><br><code>holographic</code><br>Holographic</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/mono.png" width="72"><br><code>mono</code><br>Mono</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/neon.png" width="72"><br><code>neon</code><br>Neon</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/original.png" width="72"><br><code>original</code><br>Original</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/starburst.png" width="72"><br><code>starburst</code><br>Starburst</td>
</tr>
<tr>
<td align="center"><img src="../../app/DockTilePlugin/Resources/sticker.png" width="72"><br><code>sticker</code><br>Sticker</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/blue.png" width="72"><br><code>blue</code><br>Warp 1</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/dev.png" width="72"><br><code>dev</code><br>Default (dev)</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/preview.png" width="72"><br><code>preview</code><br>Default (preview)</td>
<td align="center"><img src="../../app/DockTilePlugin/Resources/local.png" width="72"><br><code>local</code><br>Default (local)</td>
</tr>
</table>

### Wiring

The `AppIcon` enum in [`app/src/settings/app_icon.rs`](../../app/src/settings/app_icon.rs) defines
the picker entries; `AppIconSettings::get_base_icon_file_name` maps each variant to a bare filename.
`AppIcon::Default` is channel-dependent — `dev`, `preview`, `local`, or `warp_2` for everything
else.

[`app/src/appearance.rs:227-255`](../../app/src/appearance.rs) loads `<filename>.png` from
`WarpDockTilePlugin.docktileplugin` via `pathForResource:ofType:@"png"` and calls
`setApplicationIconImage:`; passing `nil` restores the bundled icon. The Objective-C side does the
same lookup at [`WarpDockTilePlugin.m:131`](../../app/DockTilePlugin/WarpDockTilePlugin.m).

The setting itself (`appearance.icon.app_icon`, storage key `AppIcon`) is declared
`SupportedPlatforms::MAC` and `SyncToCloud::Never`. Its sibling `show_dock_icon`
(`appearance.icon.show_dock_icon`) toggles Dock and Cmd-Tab presence.

**Adding a variant** requires three coordinated edits: a new `AppIcon` enum variant with its
`Display` and `get_base_icon_file_name` arms, the PNG in `DockTilePlugin/Resources/`, and matching
logic in `WarpDockTilePlugin.m`.

## Changing an icon

### Windows (this fork's usual target)

1. Author a 1024x1024 master, export `16x16 32x32 48x48 64x64 128x128 256x256 512x512`.
2. Regenerate the `.ico` — per [`script/windows/README.md:102-112`](../../script/windows/README.md):
   ```shell
   convert 16x16.png 32x32.png 48x48.png 64x64.png 256x256.png icon.ico
   ```
   Sizes above 256 are not supported in `.ico`. On ImageMagick 7 the `convert` alias is gone, so
   that line is `magick 16x16.png … icon.ico` (the `README.md` recipe still says `convert`).
   ImageMagick writes the 256 entry as an uncompressed BMP, which triples the file; the shipped
   `.ico` files instead embed 256 as PNG, so splice that entry in afterwards. Do **not** let Pillow
   write the whole `.ico` — it PNG-compresses every entry, and Inno Setup's `SetupIconFile` wants
   BMP for the small ones.
3. Write to **all three** of `app/channels/{local,oss,warp-oss}/icon/no-padding/` — the build reads
   `warp-oss`, the Cargo package and `deploy.ps1` read `oss`.
4. Force a rebuild of the resource: `cargo clean -p warp` (or touch `app/build.rs`), then
   `./script/run`.

### Linux

Drop the full size ladder into `app/channels/<channel>/icon/no-padding/`. `bundle_install` picks up
whatever exists, and every channel now ships the full ladder.

### macOS

Edit `AppIcon.icon/Assets/` plus `icon.json`, then `script/compile_icon <channel> <path>.app`.
Requires Xcode 26+; earlier versions produce no `Assets.car` and the script warns.

## Known issues

- **`local`, `oss`, and `warp-oss` are duplicate files, not links** (byte-identical across all three
  directories). Editing one and not the others silently splits Windows branding from packaged
  branding.
- **Design guidelines are baked into shipped rasters.** The `dev` and `preview` icons show
  construction circles and diagonals in the final PNG/ICO. The `dev` `icon.json` still references a
  literal `PLACE HERE.png` placeholder layer.
- **`bundle_install` skips 48x48** while the `.ico` includes it — the exported 48 PNG is
  Windows-only.

## See also

- [`script/windows/README.md`](../../script/windows/README.md) — installer and icon generation
- [`docs/guides/build-and-run.md`](../guides/build-and-run.md) — channel selection when building
- [`docs/guides/windows-local-deploy.md`](../guides/windows-local-deploy.md) — `deploy.ps1` flow
