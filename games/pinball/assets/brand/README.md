# Official Omarchy artwork

Source: https://omarchy.org/brand/ (retrieved 11 September 2026).

- `omarchy-logo.svg`: exact bytes from https://omarchy.org/brand/omarchy-logo.svg
- `omarchy-wordmark.svg`: exact bytes from https://omarchy.org/brand/omarchy-wordmark.svg
- `omarchy-logo.png`: proportional 256 x 256 raster of the official SVG, transparent background.
- `native/BrandLogo.h`: embeds a proportional 128 x 128 RGBA rendering for SDL. Generated with Qt QSvgRenderer. No cropping, distortion, recolouring, or reconstruction.

Logo SVG SHA-256: `4d00403ea7796c1fcd9a1574f807203f8d17f94d7c7fe90045963cf150346f70`.

## Placement contract and verification

The complete square mark is preserved. It is displayed at 52 x 52 in the app header, 82 x 82 logical table units, while the launcher uses the distinct Arcade pinball composition. The 256px transparent raster and the actual native render were visually inspected. Geometry, original green colour, aspect ratio and transparent negative space are unchanged. The SVG source remains unmodified.

The logo and wordmark are Omarchy brand assets, not relicensed under this repository's MIT code licence. The brand page states that Omarchy is a pending trademark with all rights reserved. This is an independent community application, not a claim of official endorsement.

omarchy-wordmark.svg SHA-256: `be58c4b721a666f23fb9e756a39cbd62868c6717e36d4beed8fe8035bae54222`.

omarchy-logo.png SHA-256: `1fd954cc57c2880b054482c4d357cd1ea7601156c100c0ad869dcc5e7a1c9b2e`.

## Arcade icon

arcade-pinball.svg uses a 256px canvas, a dark rounded tile, green rails/flippers and a white ball. The exact official SVG path is proportionally placed in a 32px square at (112,213), without recolouring or clipping. native/ArcadeIcon.h is its 128px RGBA rendering for the window icon. This establishes the proposed family treatment for other games; those repositories are not changed here. Original brand source remains unchanged.
