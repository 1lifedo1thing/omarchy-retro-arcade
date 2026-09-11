# Midnight Operators: After Hours

Twelve transparent SVG pieces for Omarchy Chess, artwork version 1.0.

## Files and rendering

`svg/` contains `wk`, `wq`, `wb`, `wn`, `wr`, `wp`, `bk`, `bq`, `bb`, `bn`, `br`, `bp`.
Every asset has a square `viewBox="0 0 128 128"`, with a shared base centre at x=64 and baseline at y=120. The tallest piece begins near y=9. Natural role heights are retained: the pawn is deliberately much shorter. Render the complete viewBox into the piece rectangle without another large inset. Preserve aspect ratio.

The artwork consists entirely of closed polygon and cubic Bezier paths, grouped under a local transform. Cable openings are transparent. No raster payloads, external fonts, linked resources, filters or gradients are present. White and Black share exactly the same geometry.

## Colour roles

| Role | White | Black |
|---|---|---|
| Body | `#e4e8df` | `#171c1a` |
| Exterior outline | `#171c1a` | `#e4e8df` |
| Highlight | `#f3f4e9` | `#374239` |
| Middle facet | `#c9cdc0` | `#252f28` |
| Shadow facet | `#a6afa0` | `#111712` |
| Cable | `#303a33` | `#252f29` |
| Cable rib | `#535e53` | `#465248` |
| Terminal screen | `#171c1a` | `#0b110d` |
| Recolourable accent | `#b3cb92` | `#b3cb92` |

Set the `fill` on `<g id="accent" data-color-role="accent">` to the active Omarchy accent. Its children inherit that colour. This changes the face pixels, mitre inset, horse eye and base indicators together. Each SVG contains exactly one literal `#b3cb92`, so replacing that token is also sufficient for these versioned assets. Do not recolour the body, screen or outline with the theme accent. Include the accent value in the app's texture cache key.

For accent-free rendering, set the accent group's opacity to zero. All accent shapes sit over opaque shapes. The opaque geometry and rendered alpha remain identical with the accent hidden. Keep the contrasting outline when changing board palettes.

## Verification and limits

- Direct Inkscape renders at 32, 48, 64 and 256 pixels; exact dimensions, transparent margins and no clipping verified.
- Matching White/Black path data and identical alpha at all four sizes verified.
- Coral, blue and accent-hidden renders checked for all twelve pieces at 64 pixels; alpha unchanged.
- Labelled contact sheet, light/dark starting-position mockups, native-size proof, enlarged 32-pixel proof and a concept-to-vector comparison included.
- Visual review: the defining heads remain recognisable at 32 pixels, but terminal eyes, cable ribs and some narrow gaps soften or merge. 48–64 pixels are preferable for this expressive set. At 32 pixels the rook's crooked torso and queen's spring read as broad angular masses.
- The previews are artwork mockups, not screenshots of the Rust application. Actual app rendering, fractional display scaling, drag behaviour and live theme switching have not been tested here. No game code or existing game artwork was replaced.

## Translation choices and provenance

Original concept: the approved image generated in this conversation, headed “MIDNIGHT OPERATORS — After hours / experimental concept” (`exec-0b919594-1cac-4a2b-85b4-3821fee8b180.png`). Its SHA-256 is recorded in `manifest.json`.

The SVG paths were newly authored against that image's upper-row coordinates, including hand-shaped Bezier cable curves. Inkscape's vector Boolean union produced the final exterior outline. No raster auto-tracing or embedded raster images were used. Lower-row pieces reuse the same geometry with a separate palette, correcting the small shape differences in the generated study.

Preserved: tilted terminal king and cross, asymmetric crown, concertina queen, split mitre, curled horse neck, zigzag rook and small round-headed pawn. Deliberate differences: common bases, flatter and simpler shading, a stronger continuous contrasting exterior rim (about 1.31 viewBox units) for small-square readability, and simplified cable ribs. The included comparison makes these differences reviewable; this is a vector interpretation, not a pixel-identical copy of the raster study.

The creative brief was guided by the [Omarchy Doctrine](https://omarchy.org/doctrine/): whimsy, computing heritage, craft and ownership. No official Omarchy logo or third-party chess artwork is used. This pack does not claim official Omarchy endorsement or add a software licence on the user's behalf.
