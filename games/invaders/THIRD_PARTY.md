# Credits and licences

Application code: GPL-3.0-or-later; see LICENSE.

Theme loading and optional PCM playback helpers adapted from tcballard/omarchy-chess, GPL-3.0-or-later.

Rust dependencies are resolved in Cargo.lock. Notable libraries: eframe/egui (MIT OR Apache-2.0), serde/serde_json (MIT OR Apache-2.0), tempfile (MIT OR Apache-2.0), fs2 (MIT OR Apache-2.0), toml (MIT OR Apache-2.0). Transitive crates and bundled font licences retain their own terms.

No Space Invaders ROM, original sprite, music or sound recording is included. No affiliation with the original game's publisher is implied.

Orbit sprites and station background were created with OpenAI ImageGen from the user-approved concept, then colour-keyed, reduced and palette-normalised for pixel rendering. Source images and approved reference are in docs/design. No imported commercial game artwork is used. The small bitmap alphabet uses conventional 5×7 character forms. The formation is a readable low-resolution spelling of OMARCHY, not a reproduction of the official wordmark.

PNG decoding: image crate, MIT OR Apache-2.0.
