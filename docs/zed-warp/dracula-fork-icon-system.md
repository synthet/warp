# Dracula Fork Icon System — Zed + Warp
**Status:** design plan / implementation spec  
**Target platforms:** macOS 26/27-era icon system + Windows 11  
**Apps:** custom forks of Zed and Warp  
**Visual reference:** Dracula / Dracula JetBrains family

---

## 1. Goal

Create a sibling icon family for custom forks of **Zed** and **Warp** that:

- feels native on both macOS and Windows;
- is clearly derived from a shared design system;
- remains distinguishable from the upstream Zed and Warp apps;
- reads cleanly at 16–32 px as well as at 1024 px;
- uses the Dracula palette as the color foundation;
- is easy to regenerate from vector sources;
- supports future additional developer-tool forks without redesigning the system.

The two icons should look like products from the same suite, not like recolored copies of unrelated upstream icons.

---

## 2. Design direction: “Dracula Prism”

### Shared family grammar

Every app icon uses the same five-part system:

1. **Dark foundation**
   - Dracula Background `#282A36`
   - secondary raised plane `#44475A`

2. **Near-white semantic glyph**
   - Dracula Foreground `#F8F8F2`

3. **Primary neon accent**
   - chosen per application

4. **Secondary accent**
   - used only on one edge / layer / cut
   - never equal in visual weight to the primary accent

5. **Restrained depth**
   - 2–3 planes maximum
   - soft shadow / specular separation at large sizes
   - no glow at small sizes

### App identities

| App | Primary accent | Secondary accent | Metaphor |
|---|---|---|---|
| Zed fork | Purple `#BD93F9` | Cyan `#8BE9FD` | code/editor cut, angular Z-like split |
| Warp fork | Pink `#FF79C6` | Purple `#BD93F9` or Cyan `#8BE9FD` | terminal prompt / flow / folded path |

Use the upstream apps only as **semantic context**. Do not trace or slightly recolor their existing marks.

---

## 3. Dracula reference palette

Canonical Dracula colors:

```text
Background    #282A36
Current Line  #6272A4
Selection     #44475A
Foreground    #F8F8F2
Comment       #6272A4
Red           #FF5555
Orange        #FFB86C
Yellow        #F1FA8C
Green         #50FA7B
Cyan          #8BE9FD
Purple        #BD93F9
Pink          #FF79C6
```

### Palette rules for this icon family

Use only:

```text
Base dark       #282A36
Raised dark     #44475A
Muted edge      #6272A4
Glyph           #F8F8F2
Zed primary     #BD93F9
Zed secondary   #8BE9FD
Warp primary    #FF79C6
Warp secondary  #BD93F9
Optional tiny status accent:
                 #50FA7B
```

Avoid using red, orange, yellow, green simultaneously. The icon family should feel controlled, not rainbow-colored.

---

## 4. Geometry

### Master canvas

Design all masters at:

```text
1024 × 1024
```

Keep the source vector-based.

### Shared construction grid

Use a 64-unit logical grid mapped to the 1024 master.

```text
1 logical unit = 16 px at 1024 master size
```

Important anchors:

```text
outer art safe region     x/y 5–59
primary glyph region      x/y 14–50
minimum internal gap      3 units
minimum major stroke      5 units
small-detail cutoff       < 2 units
```

### Family silhouette

The family should use a broad square / rounded-square footprint rather than a narrow logo.

Do not use typography, full application names, or tiny terminal/code characters in the launcher icon.

---

## 5. Zed fork concept

### Semantic idea

A **cut / split editor plane** that indirectly forms a Z-shaped movement.

It should read first as:

> fast editor / code tool

and only secondarily as:

> Z

### Construction

Large version:

```text
Layer 1 — dark foundation
Layer 2 — purple angled upper plane
Layer 3 — cyan lower edge / cut
Layer 4 — near-white central slash or cursor-like notch
```

The primary form should have 2–3 large angles.

Avoid a literal alphabetic “Z” drawn as text.

### Small-size simplification

At 32 px and below:

- remove internal shadows;
- remove translucency;
- merge cyan edge into one solid accent strip;
- increase negative-space gap by about 10–15%.

At 16 px:

- use only dark body + purple body + one white notch;
- cyan may be removed if it becomes a single-pixel artifact.

---

## 6. Warp fork concept

### Semantic idea

A **flowing terminal path**: prompt arrow + folded ribbon / warp trajectory.

It should read first as:

> terminal / command execution / flow

and only secondarily resemble a W-shaped motion.

### Construction

Large version:

```text
Layer 1 — dark foundation
Layer 2 — pink forward/folded path
Layer 3 — purple or cyan rear plane
Layer 4 — near-white terminal prompt notch / chevron
```

The path should feel more curved or folded than the Zed icon so the two remain distinct.

### Small-size simplification

At 32 px and below:

- reduce the path to one main pink shape;
- preserve one white prompt chevron;
- remove secondary reflections.

At 16 px:

- dark body + pink path + white prompt only.

---

## 7. Shared style rules

### Do

- use a strong silhouette;
- keep the semantic glyph centered optically, not mathematically;
- make the Zed icon more angular;
- make the Warp icon more directional / flowing;
- use one dominant accent per app;
- retain visible dark area around the bright glyph;
- build explicit small-size variants.

### Do not

- copy the upstream Zed or Warp logo;
- use a literal `Z`, `W`, `>_`, or app name as the whole icon;
- use more than 3 depth planes;
- use heavy neon bloom;
- depend on subtle gradients for recognition;
- use hairline strokes;
- rely on color alone to distinguish the apps.

---

## 8. macOS strategy

Apple’s current app-icon workflow is centered around a **1024×1024 layered source** and Icon Composer.

### Preferred modern workflow

Source:

```text
SVG layers
   ↓
Apple Icon Composer
   ↓
Icon Composer file in Xcode project
   ↓
system-generated platform / appearance / size variants
```

Prepare separate vector layers:

```text
01-foundation.svg
02-secondary-plane.svg
03-primary-accent.svg
04-glyph.svg
```

For Icon Composer:

- do not export a baked canvas mask;
- keep layers separate;
- prefer SVG;
- convert any text to outlines;
- leave final blur, shadow, specular, translucency and Liquid Glass behavior to Icon Composer;
- preview Default, Dark and Mono appearances;
- preview specifically on macOS, not only iOS.

### Legacy / non-Xcode fallback

If the fork’s packaging expects an `.icns` file, generate a traditional iconset from the same master artwork.

Recommended raster set:

```text
16 × 16
32 × 32
64 × 64
128 × 128
256 × 256
512 × 512
1024 × 1024
```

For a conventional `iconutil` pipeline, keep the normal 1x / 2x pairs in an `.iconset` directory and compile to `.icns`.

Important: the legacy raster version should be a flattened derivative, not the design source of truth.

---

## 9. Windows strategy

Windows 11 app icons are displayed in many contexts and Windows prefers exact-size assets when available.

### Win32 / classic desktop package

Ship a multi-resolution `.ico`.

Recommended contained sizes:

```text
16 × 16
24 × 24
32 × 32
48 × 48
64 × 64
128 × 128
256 × 256
```

At minimum, Microsoft's current guidance calls for:

```text
16
24
32
48
256
```

The 256 px entry should be 32-bit RGBA.

### MSIX / packaged app

Create explicit target-size PNG assets.

Recommended target-size set:

```text
16
20
24
30
32
36
40
48
60
64
72
80
96
256
```

Also produce the appropriate package / store logo assets required by the manifest.

For target-size assets, prepare:

```text
default
dark-theme unplated
light-theme unplated
```

where the package format requires them.

### Windows visual treatment

Windows guidance favors:

- simple metaphors;
- straight-on presentation;
- very few layers;
- soft corners;
- restrained gradients;
- good contrast on both light and dark backgrounds.

For this family:

- keep outside transparency for Win32/target-size artwork when practical;
- use the Dracula dark plate only as part of the actual silhouette;
- avoid a giant opaque square merely to simulate the macOS container;
- make the Windows small-size glyph slightly larger than the macOS glyph.

---

## 10. macOS vs Windows differences

Do **not** export one identical bitmap to both platforms.

Use one source design with platform-specific composition.

| Attribute | macOS | Windows |
|---|---|---|
| Master | 1024 layered | 1024 vector master |
| Outer treatment | system/native enclosure | transparent outside silhouette preferred |
| Depth | layered / material-friendly | flatter |
| Shadow | can be richer at large size | restrained |
| Small variant | generated + manually checked | manually optimized |
| 16 px glyph | simplified | aggressively simplified |
| Primary deliverable | Icon Composer / asset catalog; ICNS fallback | ICO + PNG target assets |

---

## 11. Light, dark, mono and high-contrast behavior

### Default full-color

Use the canonical Dracula palette.

### Light environment

Do not lighten the dark foundation into gray.

Instead:

- keep `#282A36`;
- slightly strengthen the outer edge;
- ensure foreground remains visually separated.

### Dark environment

Prevent the foundation from disappearing into dark UI:

- use a subtle `#44475A` edge plane;
- preserve bright accent area around the silhouette;
- avoid relying on a black shadow.

### Monochrome

Create a single-shape version of each app:

```text
Zed: angular split/cut silhouette
Warp: directional folded-path silhouette
```

These must remain recognizable without color.

### Windows high contrast

Windows 11 does not require separate high-contrast app-icon assets, but the icon should still have a valid black/white simplification for accessibility and future use.

---

## 12. Source file layout

Recommended repository layout:

```text
assets/
  icons/
    README.md

    palette/
      dracula.json

    source/
      shared/
        grid.svg
        safe-area.svg

      zed-fork/
        01-foundation.svg
        02-secondary-plane.svg
        03-primary-accent.svg
        04-glyph.svg
        master.svg
        mono.svg

      warp-fork/
        01-foundation.svg
        02-secondary-plane.svg
        03-primary-accent.svg
        04-glyph.svg
        master.svg
        mono.svg

    macos/
      zed-fork/
        icon-composer/
        legacy.iconset/
        AppIcon.icns
      warp-fork/
        icon-composer/
        legacy.iconset/
        AppIcon.icns

    windows/
      zed-fork/
        AppIcon.ico
        target-size/
        store/
      warp-fork/
        AppIcon.ico
        target-size/
        store/

    previews/
      contact-sheet.png
      macos-dock.png
      windows-taskbar.png
      light-dark-grid.png
```

---

## 13. Palette machine-readable file

Suggested `dracula.json`:

```json
{
  "background": "#282A36",
  "selection": "#44475A",
  "comment": "#6272A4",
  "foreground": "#F8F8F2",
  "red": "#FF5555",
  "orange": "#FFB86C",
  "yellow": "#F1FA8C",
  "green": "#50FA7B",
  "cyan": "#8BE9FD",
  "purple": "#BD93F9",
  "pink": "#FF79C6"
}
```

---

## 14. Build pipeline

Keep vector masters as the source of truth.

Suggested pipeline:

```text
SVG master
  ├─ validate SVG geometry
  ├─ render 1024 PNG preview
  ├─ render macOS legacy raster set
  ├─ compile ICNS fallback
  ├─ render Windows target-size PNGs
  ├─ assemble multi-resolution ICO
  └─ render preview/contact sheets
```

Possible tooling:

- macOS: `iconutil` for legacy `.icns`
- ImageMagick or a small Python renderer for deterministic PNG resizing
- ImageMagick / Pillow / dedicated ICO tool for `.ico`
- Apple Icon Composer for the modern native Apple deliverable

Do not use ordinary bilinear downscaling as the only small-icon strategy. The 16/24/32 px assets should come from a simplified small-size source variant.

---

## 15. Small-size QA matrix

Review these sizes at **100% actual pixels**:

```text
16
20
24
32
48
64
128
256
512
1024
```

For each size test against:

```text
white
#F0F0F0
#202020
#000000
Dracula #282A36
busy desktop wallpaper
```

Pass criteria:

- app identity recognizable at 16 px;
- Zed and Warp cannot be confused at 24 px;
- no 1 px accidental gaps;
- no muddy accent blending;
- no halo around transparent edges;
- white glyph does not bloom into the accent;
- icon remains legible in grayscale.

---

## 16. Native-environment QA

### macOS

Check:

- Dock
- Finder Applications view
- Spotlight
- Command-Tab switcher
- Settings / system app listings
- light appearance
- dark appearance
- small and large Dock sizes

### Windows 11

Check:

- Taskbar
- Start pinned area
- Start “All apps”
- Search
- Alt-Tab
- title bar
- File Explorer
- Settings app list
- Task Manager
- light theme
- dark theme
- 100%, 125%, 150%, 200% display scaling

---

## 17. Variant policy

Recommended initial release:

```text
Zed fork:
  Dracula Purple

Warp fork:
  Dracula Pink
```

Optional later variants:

```text
Nightly      cyan edge
Dev/Debug    green micro-accent
Preview      orange micro-accent
```

Do not make release channels entirely different icons. Preserve the base silhouette and modify only a small accent region so users can still recognize the application.

---

## 18. Fork differentiation / trademark hygiene

Because these are forks:

- use an original silhouette;
- avoid merely recoloring upstream marks;
- avoid upstream wordmarks inside the icon;
- use a fork-specific app name in About / installer / metadata;
- document that Dracula colors are a palette reference, not an implication of official Dracula endorsement unless permission/branding rules allow it.

The icon should communicate functional lineage without creating unnecessary confusion with the original product.

---

## 19. Recommended implementation order

### Phase 1 — geometry

1. Draw both mono silhouettes.
2. Test them at 16/24/32 px.
3. Fix recognition before adding color.

### Phase 2 — shared visual system

1. Add the Dracula foundation.
2. Add one primary accent per app.
3. Add one secondary edge.
4. Add the near-white semantic notch.

### Phase 3 — platform adaptations

1. macOS layered version.
2. Windows flat/unplated version.
3. 16/24/32 small-icon variants.

### Phase 4 — packaging

1. Icon Composer / Xcode asset.
2. ICNS fallback.
3. Windows ICO.
4. Windows MSIX PNG assets.

### Phase 5 — QA

1. Dock + Taskbar side-by-side with other major apps.
2. Light/dark backgrounds.
3. high DPI scaling.
4. grayscale.
5. release/nightly differentiation if needed.

---

## 20. Acceptance criteria

The system is ready when:

- both icons clearly belong to the same family;
- neither looks like a simple Dracula recolor of the upstream app;
- Zed remains more angular and Warp more flowing;
- each is distinguishable at 16 px;
- macOS uses a native layered/material-aware composition;
- Windows receives explicit target-size assets;
- vector masters are source-controlled;
- raster outputs are reproducible;
- `icns`, `ico`, PNG target assets and previews can all be regenerated from the repository.

---

## 21. Primary references

Apple Human Interface Guidelines — App icons  
https://developer.apple.com/design/human-interface-guidelines/app-icons

Apple — Creating your app icon using Icon Composer  
https://developer.apple.com/documentation/xcode/creating-your-app-icon-using-icon-composer

Apple Design Resources  
https://developer.apple.com/design/resources/

Microsoft — App icons  
https://learn.microsoft.com/en-us/windows/apps/design/iconography/app-icons

Microsoft — Design guidelines for Windows app icons  
https://learn.microsoft.com/en-us/windows/apps/design/iconography/app-icon-design

Microsoft — Construct your Windows app's icon  
https://learn.microsoft.com/en-us/windows/apps/design/iconography/app-icon-construction

Dracula — canonical palette / contribution guide  
https://draculatheme.com/contribute

Warp — current icon customization examples  
https://docs.warp.dev/terminal/appearance/app-icons

---

## 22. Mockup brief

Generate one presentation sheet containing:

- Zed fork macOS icon
- Warp fork macOS icon
- Zed fork Windows icon
- Warp fork Windows icon
- 32 px simplification previews
- palette swatches
- a Dock / Taskbar context row

Visual tone:

> premium developer-tool iconography, Dracula dark surfaces, restrained neon accents, geometric vector construction, no text inside icons, no excessive glow, high small-size readability.
