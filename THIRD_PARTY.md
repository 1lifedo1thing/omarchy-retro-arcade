# Third-party components

Omarchy Chess is GPL-3.0-or-later. Full license: LICENSE. Exact Rust dependency versions and checksums are in Cargo.lock.

- **shakmaty** and **pgn-reader**, Niklas Fiekas and contributors: GPL-3.0-or-later. Chess rules and PGN parsing. https://github.com/niklasf/shakmaty and https://github.com/niklasf/rust-pgn-reader
- **egui / eframe / egui_extras**, Emil Ernerfeldt and contributors: MIT OR Apache-2.0. Native window, interface and SVG loading. https://github.com/emilk/egui
- **rfd**, contributors: MIT. Desktop file dialogs through XDG portals. https://github.com/PolyMeilex/rfd
- **serde / serde_json**, **toml**, **tempfile**, **rustix** and their contributors: see each crate's license files in the source resolved by Cargo.lock. They provide serialization, configuration, atomic files and operating-system interfaces. **fs2** is MIT OR Apache-2.0.
- **Chisel chess piece artwork**: original project SVGs translated from the selected generated Chisel concept, used under the project GPL-3.0-or-later licence. Geometry and facets are retained from the artwork handoff; only the isolated inlay follows the theme accent. See assets/pieces/CHISEL.md. These replace the former Cburnett/python-chess SVGs, which remain in Git history.
- **Stockfish**, the Stockfish developers: GPL-3.0-or-later. The Arch package includes a separate generic-CPU executable compiled from commit 59aae690f91d6f69aac194f447d84b4a2c3be778, with network nn-1a298aa575a0.nnue. Full source and network SHA-256 checksums are in packaging/PKGBUILD; corresponding engine sources/network are uploaded alongside the package as stockfish-and-app-source. Its license is installed with the package. https://github.com/official-stockfish/Stockfish

The small app icon in packaging/ is original project artwork, GPL-3.0-or-later. The app reads user-installed Omarchy theme colours but does not redistribute Omarchy branding or theme files.

Rust dependencies are linked into the binary. Distributors must retain applicable copyright/license notices and provide required corresponding source, including dependencies. `cargo vendor --locked` can collect the locked dependency sources and their license files for a source distribution; Cargo.lock alone is not a source offer. This preview does not publish a combined installer or stable binary release.
