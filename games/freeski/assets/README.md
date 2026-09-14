# FreeSki asset provenance

All FreeSki visuals are original project work by Omarchy Arcade contributors,
created on 12–14 September 2026 under GPL-3.0-or-later.

| Source | Contents | Creation method |
| --- | --- | --- |
| `shelf.svg` | Snowfield, layered skier, snowy trees, faceted rocks and FreeSki nameplate | Hand-authored SVG geometry |
| `../src/render.rs` | World projection, engraved boundary marks, finish stripe, tracks and wind marks | Original native egui vector drawing |
| `../src/artwork.rs` | Layered skier and movement poses, snowy evergreens, faceted rocks, raised ramps, cloth gates, finish flags, shadows, powder and landing puffs | Original hand-authored egui vector geometry and bounded cosmetic animation |
| `../src/geometry.rs` | Shared polygon triangulation and ellipse drawing | Native Rust geometry helpers, extracted unchanged from the yeti renderer |
| `../src/yeti.rs` | Shaggy white yeti, claws, snarling face and running poses | Original hand-authored egui vector geometry; movement-driven animation |
| `../src/world.rs` | Six-section practice slope and three ramp/rock pairs | Original authored world coordinates |
| `../src/course.rs` | Five Slalom courses and medal targets | Original authored coordinates, calibrated against production reference runs |
| `../src/audio.rs` | Carve, jump, crash, gate, miss, warning, catch and finish cues | Original mono PCM synthesis from oscillators and deterministic noise; no samples |
| `../../../shared/presentation/assets/README.md` | Reused Arcade cabinet material | Existing approved shared asset and provenance |

The game is inspired by the downhill skiing genre. No SkiFree images, character
designs, course data or sounds have been imported. Existing games' artwork remains unchanged.

For future assets, add path, creator/source, creation method, licence,
modifications and attribution needs. For generated assets also record the tool
and creation brief. Include notices with the installed package.
