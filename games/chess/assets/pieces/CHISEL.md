> Original artwork handoff, imported into Omarchy Chess on 11 September 2026. Statements below about the app being unchanged describe the handoff before integration.

# Omarchy Chess: Chisel

This pack develops the selected **Chisel** concept into twelve transparent SVG pieces. It supersedes the earlier Cathedral Cut artwork for this design task. No app or repository files have been changed.

## Contents

- `svg/`: `wk.svg`, `wq.svg`, `wb.svg`, `wn.svg`, `wr.svg`, `wp.svg`, `bk.svg`, `bq.svg`, `bb.svg`, `bn.svg`, `br.svg`, `bp.svg`.
- `png/32/`, `png/48/`, `png/64/`, `png/256/`: direct transparent renders of every SVG at each size.
- `previews/contact-sheet.png`: all twelve labelled vector pieces.
- `previews/board-light.png`, `board-dark.png`: exact starting positions in two board palettes.
- `previews/readability-native.png`: 32/48/64 px samples on light and dark squares; view at 100% for actual pixel size.
- `previews/readability-32-enlarged.png`: the exact 32 px pixels enlarged 4× without smoothing.
- `previews/reference-vs-vector.png`: the selected concept beside the resulting artwork, at a common scale and baseline.
- `previews/recolour-proof.png`: sage, coral, blue and no-accent variants, rendered from the SVGs.
- `manifest.json`: palette, dimensions, hashes and validation results.

## Colour roles

| Role | White | Black |
|---|---|---|
| Main body | `#e4e8df` | `#242c27` |
| Highlight plane | `#f3f5ed` | `#515c53` |
| Mid plane | `#c9d0c1` | `#354138` |
| Shadow plane | `#a7b29f` | `#171c1a` |
| Deep detail | `#7b8975` | `#0d1310` |
| Fine outline / eye | `#171c1a` | `#e4e8df` |
| Recolourable inlay | `#b3cb92` | `#b3cb92` |

The small palette of solid planes preserves Chisel's sculpted appearance. The dark body includes slightly lighter charcoal faces so its facets remain visible. These body and outline colours stay fixed independently of the user's theme accent.

The diagonal inlay is isolated in `<g id="accent" data-color-role="accent">`. Change **only this group's fill** before rasterising. Each supplied SVG also contains exactly one literal `#b3cb92`, so exact replacement of that token works for these files. Rebuild cached textures when the accent, output size or display scale changes. The recolour proof includes complete removal of the inlay; the body and outline geometry remain unchanged.

## Geometry and rendering

All pieces share `viewBox="0 0 128 128"` and a common baseline, with transparent padding. Preserve the whole square when scaling. Do not crop to individual bounds or enlarge pawns to king height. The slender proportions and graduated heights come from the selected study.

The silhouette, sculpted faces and inlay are actual vector paths. A local `clipPath` keeps facets inside the silhouette; it references no external file. There are no embedded images, fonts, gradients, filters, scripts or linked resources. A 1.25-unit contrasting rim improves edge separation on both square colours while retaining a fine outline at ordinary playing sizes.

The SVGs were directly rendered at 32, 48, 64 and 256 px and checked for clipping, transparency and matching side geometry. Both square backgrounds were visually inspected. **48 px and above shows the sculpted facets more clearly; at 32 px the silhouette carries most of the detail.** Separate recolour checks verify that three alternate states change only the inlay region. Opaque recolouring preserves alpha exactly; hiding the inlay allows only minor edge antialiasing differences, recorded in the manifest.

Board previews are artwork mockups, not app screenshots. Native Rust rendering and fractional desktop scaling still need checking during integration.

## Provenance and translation choices

The Chisel concept was generated specifically for this project using Image Generation and selected as option 1 in the latest design round. The production shapes and facet boundaries were newly authored from that study; they contain no raster payload. `manifest.json` records the selected image's hash.

The translation preserves the cross, open crown, split mitre, horse profile, battlements, faceted pawn head, stepped feet and diagonal sage seam. Flat colour planes replace the raster study's subtle lighting variation. A fine contrasting edge, a clearer dark-side eye and controlled facet clipping were added for screen use. See the comparison sheet for the actual differences.

No third-party source artwork, official Omarchy logo or wordmark is included. This artwork handoff does not change the repository's licensing or publish a game release.
