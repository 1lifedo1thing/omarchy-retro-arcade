# Integration decisions

- One repository, native window, desktop identity, Arch package and release version. Games are ordinary source subdirectories, not submodules or downloaded plugins.
- Four Rust/egui games are library dependencies of the Arcade executable. Preserve approved rendering, gameplay and existing storage paths. Acquire each game's original session lock before opening it, flush on leaving, and drop it on returning to the shelf.
- Circuit retains its C++ upstream engine. A private bundled worker renders through SDL software into the Arcade window. This avoids X11 window embedding and works with the same frontend on Wayland. The worker has no visible window or desktop entry. Its framed local pipes carry pixels and input, never network traffic.
- Existing per-game save directories and artwork choices remain authoritative. No bulk move or conversion risks existing saves. Circuit retains high scores/settings; like its source version it does not restore unfinished games.
- Shared navigation is Ctrl+H / Back to Arcade. Existing game shortcuts remain available. Leaving Circuit explicitly confirms ending the current table.
- Source repositories and open PRs remain intact until the consolidation is accepted. Imported Git history and source hashes make every migration traceable.
- Snake is a Rust/egui library under `games/snake`, appended to the existing shelf and rendered in the same window. Shelf indexing derives from `Game::ALL.len()` to accommodate concurrent additions without replacing artwork or reordering the approved five entries.
- Snake's `snake-v1` engine and replay adapter build without desktop features. The concurrent Stack-only leaderboard is an explicit dependency; Snake performs no network activity and does not ship a second service. See `games/snake/docs/LEADERBOARD-HANDOFF.md`.
- Snake uses existing theme loading and atomic writes, an independent schema-versioned save directory, and separate records/session files. No other game's save or settings schema changes.
