# Third-party components

Omarchy Chess is GPL-3.0-or-later. Full license: LICENSE. Exact Rust dependency versions and checksums are in Cargo.lock.

- **shakmaty** and **pgn-reader**, Niklas Fiekas and contributors: GPL-3.0-or-later. Chess rules and PGN parsing. https://github.com/niklasf/shakmaty and https://github.com/niklasf/rust-pgn-reader
- **egui / eframe / egui_extras**, Emil Ernerfeldt and contributors: MIT OR Apache-2.0. Native window, interface and SVG loading. https://github.com/emilk/egui
- **rfd**, contributors: MIT. Desktop file dialogs through XDG portals. https://github.com/PolyMeilex/rfd
- **serde / serde_json**, **toml**, **tempfile**, **rustix** and their contributors: see each crate's license files in the source resolved by Cargo.lock. They provide serialization, configuration, atomic files and operating-system interfaces. **fs2** is MIT OR Apache-2.0.
- **Chess piece artwork**, Cburnett, adapted by python-chess: the twelve SVGs in `assets/pieces/` were extracted from `chess.svg` in chess 1.11.2. Used under the GPL option stated in that module. https://github.com/niklasf/python-chess/blob/v1.11.2/chess/svg.py . Python-chess is artwork provenance only; it is not an application or build dependency.
- **Stockfish**, the Stockfish developers: GPL-3.0-or-later. A separate executable, not included in the application binary or Arch package. https://github.com/official-stockfish/Stockfish

The small app icon in packaging/ is original project artwork, GPL-3.0-or-later. The app reads user-installed Omarchy theme colours but does not redistribute Omarchy branding or theme files.

Rust dependencies are linked into the binary. Distributors must retain applicable copyright/license notices and provide required corresponding source, including dependencies. `cargo vendor --locked` can collect the locked dependency sources and their license files for a source distribution; Cargo.lock alone is not a source offer. This preview does not publish a combined installer or stable binary release.
