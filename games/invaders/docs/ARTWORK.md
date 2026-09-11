# Orbit artwork format

The built-in atlas is `assets/orbit/atlas.png`. Settings can create a copy at `$XDG_DATA_HOME/omarchy-invaders/orbit/atlas.png` (default `~/.local/share/omarchy-invaders/orbit/atlas.png`). Edit with a pixel editor and choose Reload artwork. No Rust rebuild is needed. The active custom/built-in selection is saved with preferences. Invalid files retain the loaded image; missing custom art on next launch falls back to built-in art and reports the error in Settings.

Use a **192×128 RGBA PNG**, maximum 1 MB, arranged as **six columns × four rows of 32×32 cells**. Keep transparency around every sprite. Positions are fixed presentation slots, counted left to right:

| Row | Cells |
| --- | --- |
| 1 | Player idle, thrust, firing; clamp open, closed, spare |
| 2 | Sentry idle, alternate, spare; carrier idle, alternate, spare |
| 3 | Bonus craft A, B; reference player shot, enemy shot; explosion A, B |
| 4 | Explosion C, D; bunker reference; ivory spark, amber spark; icon reference |

Player, enemies, bonus craft and explosions use the atlas. Projectiles are deliberately drawn as crisp solid pixels above effects. Bunkers are rendered from their surviving collision cells in flat colour bands so visible damage exactly tracks physical damage. Their reference cells are unused. The station background is a separate embedded texture. Launcher artwork is packaged separately; runtime overrides do not alter system launcher files.

Frames share a cell centre anchor. Preserve that anchor and leave the sprite inside its cell. Typical new enemy cell display size is 32 logical units, player 64, bonus craft 64, explosion 48. The new enemy collision half-extents are 10×11 logical units, independently defined in game.rs. Old saved runs keep 18×15 enemy half-extents until their next wave. Replacing art never changes physics, points, formation positions or saves.

The exact colour `#b3cb92` is the theme-recolourable inlay. Ivory highlights remain fixed. Very dark, overly bright or orange/red theme accents fall back to sage in the arena; hostile shots remain amber. Nearest-neighbour sampling keeps pixels sharp. Fractional window scaling may produce uneven physical pixel widths and still needs a real desktop playtest.

`tools/prepare-orbit.cjs` documents colour-key removal, nearest-neighbour atlas reduction and palette normalisation from the generated source atlas. It needs Node and Sharp; normal builds use committed assets and do not need either. Source provenance is in THIRD_PARTY.md. Full source artwork and the selected concept are in docs/design.
