# Synth fork icon sources (mirror)

Vector masters for the shared Synth Warp + Synth Zed app-icon family. This is a
**mirror** — the canonical copy lives in the sibling Zed checkout at
`../zed/assets/branding/jetbrains-darcula-concepts/`, alongside the generator
that renders it. Edit there, then re-copy here, the same way
[`docs/zed-warp/`](../../../docs/zed-warp/MIRROR.md) is kept in sync.

## What ships from these

`script/apply_branding_icons.py` in the Zed checkout writes this repo's channel
rasters directly:

```sh
cd ../zed
uv run --with pillow python script/apply_branding_icons.py           # Zed + Warp
uv run --with pillow python script/apply_branding_icons.py --check   # validate only
```

Targets `app/channels/{oss,warp-oss,local}/icon/no-padding/` — the fork's own
channels. `stable`, `preview` and `dev` keep inherited Warp brand assets. See
[`docs/reference/app-icons.md`](../../../docs/reference/app-icons.md) for the
full icon surface.

## The plate

Flat `#101014`, inset `64/1024`, corner radius `184`, transparent surround — the
construction Cursor and Photoshop use. No gradient, no accent rim.

The transparent surround is load-bearing. The previous rasters were opaque to
the canvas edge, so Windows drew a hard black frame around the tile.
`check_transparent_surround()` probes every corner and edge midpoint of every
emitted PNG and ICO frame to keep that from returning.

## Files

| File | Role |
| --- | --- |
| `warp-macos.svg` | Warp master, silver gradient glyph — source for the PNG sizes |
| `warp-windows.svg` | Warp master, flat `#A9B7C6` glyph — source for `icon.ico` |
| `warp-small-32.svg` | Warp at 16/24/32 px, inter-panel gap widened |
| `zed-*.svg` | Zed equivalents, mirrored so the family reads as one set |
| `comparison-sheet.svg` / `.png` | Generated contact sheet |
