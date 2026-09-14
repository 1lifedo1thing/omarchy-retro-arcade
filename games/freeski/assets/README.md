# FreeSki asset provenance

All FreeSki visuals are original project work by Omarchy Arcade contributors,
created on 12–14 September 2026 under GPL-3.0-or-later.

| Source | Contents | Creation method |
| --- | --- | --- |
| `shelf.svg` | Snowfield, skier, trees and FreeSki nameplate | Hand-authored SVG geometry |
| `../src/render.rs` | Skier, trees, rocks, striped ramps, shadows, protection ring, boundary marks, finish flags, Slalom gates and tracks | Original native egui vector drawing |
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
