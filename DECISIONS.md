# Integration decisions

- One repository, native window, desktop identity, Arch package and release version. Games are ordinary source subdirectories, not submodules or downloaded plugins.
- Four Rust/egui games are library dependencies of the Arcade executable. Preserve approved rendering, gameplay and existing storage paths. Acquire each game's original session lock before opening it, flush on leaving, and drop it on returning to the shelf.
- Circuit retains its C++ upstream engine. A private bundled worker renders through SDL software into the Arcade window. This avoids X11 window embedding and works with the same frontend on Wayland. The worker has no visible window or desktop entry. Its framed local pipes carry pixels and input, never network traffic.
- Existing per-game save directories and artwork choices remain authoritative. No bulk move or conversion risks existing saves. Circuit retains high scores/settings; like its source version it does not restore unfinished games.
- Shared navigation is Ctrl+H / Back to Arcade. Existing game shortcuts remain available. Leaving Circuit explicitly confirms ending the current table.
- Source repositories and open PRs remain intact until the consolidation is accepted. Imported Git history and source hashes make every migration traceable.

## Stack and optional community leaderboards

- Stack is a Rust library game inside the same eframe window, desktop entry and package. Both modes work offline; its state uses a new `omarchy-stack` directory without changing existing games' save locations.
- Gameplay uses deterministic 60 Hz ticks and documented Stack-specific symmetric rotation kicks. The service links this same engine with UI dependencies disabled. Rules are versioned independently from the app release.
- Shared HTTP transport is in `shared/leaderboard`; the separately deployable SQLite service is in `services/leaderboard`. No public URL is bundled. Explicit end-of-run sharing, pseudonymous credentials, bounded replay verification and private retry storage are required before results become public.
- The initial service is deliberately single-process and intended for a small community. Deployment behind the supplied HTTPS proxy, backups and operational checks must be verified before activating public sharing. Hosting is a separate approval gate, estimated at $11/month before tax in HOSTING.md.
