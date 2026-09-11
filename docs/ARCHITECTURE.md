# Rust code map

- `src/game.rs`: authoritative positions, legal moves, notation, results, draw claims, takebacks and PGN. Uses shakmaty and pgn-reader; no GUI dependency in this module.
- `src/storage.rs`: versioned JSON, legacy migration, bounded reads, atomic writes, archives and lifetime advisory lock.
- `src/theme.rs`: bounded TOML reads, strict colour validation and fallback palette.
- `src/engine.rs`: external UCI process, worker thread, cancellation, startup/search/write deadlines and legal-result validation.
- `src/ui.rs`: egui board, controls, modal dialogs, history, input, persistence orchestration and engine-result acceptance.
- `src/main.rs`: CLI version, single-instance guard and eframe native window lifecycle.
- `assets/pieces/`: embedded SVG pieces; no artwork downloads at runtime.

## State transitions

Human move intent is validated against the authoritative game. Changes cancel outstanding work and advance the revision, then update the UI, save and request an engine turn if needed. New game, import, resignation, draw claim and takeback invalidate outstanding results. Only current-revision legal engine moves may change the board.

Each move/hint launches a fresh UCI process. Startup, readiness, writes and search are bounded; cancellation interrupts waiting and kills/reaps the child. Nonblocking stdin prevents a stalled reader from trapping a large position write. Errors surface with Retry; there is no network fallback or random substitute opponent.

## Persistence

Version 2 JSON stores the initial FEN, full UCI move history, game settings, result and board preferences. Loading reconstructs all positions so repetition survives restart. Version 1 Python-preview sessions are read through their PGN and backed up before migration. Corrupt bytes are archived before replacement. Atomic writes use a temporary file in the destination directory, file fsync, rename and directory fsync.

A Rust advisory file lock is held for the application lifetime. The legacy Qt lock is also checked so both implementations cannot silently overwrite the same session. Stale legacy lock recovery is deliberately manual.

## Testing boundaries

Rules tests exercise integration, unusual chess rules, PGN rejection, migration and recovery. Headless egui tests deliver pointer events and inspect state/layout. Fake UCI executables exercise failures; real Stockfish validates interoperability. Xvfb CI rendering and Arch packaging complement these tests. Real Wayland/Hyprland behaviour and assistive-technology acceptance remain separate checks.
