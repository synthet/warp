# Dracula Prism icon sources

This directory contains the deterministic vector source for the Dracula Prism Zed and Warp
fork icon family. The artwork is original and does not reuse either upstream app glyph.

## Source layout

- `palette/dracula.json` is the machine-readable color source of truth.
- `source/shared/` contains the 64-unit construction grid and safe-area overlay.
- `source/zed-fork/` and `source/warp-fork/` contain individual Icon Composer layers,
  self-contained macOS and Windows masters, hand-simplified small variants, and monochrome
  silhouettes.
- `previews/contact-sheet.svg` is a fully vector design-review sheet.

All large masters use a `1024 1024` view box. The small variants use their native 32 px and
16 px view boxes so their geometry is explicit rather than obtained by blind downscaling.

## Render checks

ImageMagick can render the standalone composites for review:

```powershell
magick source/zed-fork/macos.svg zed-macos.png
magick source/zed-fork/windows.svg zed-windows.png
magick source/warp-fork/macos.svg warp-macos.png
magick source/warp-fork/windows.svg warp-windows.png
magick previews/contact-sheet.svg contact-sheet.png
```

The four numbered layer SVGs are the inputs intended for Apple Icon Composer. The Windows
masters deliberately use flatter fills and larger semantic marks. Keep the 16 px and 32 px
variants as independent sources when producing ICO or MSIX target-size assets.
