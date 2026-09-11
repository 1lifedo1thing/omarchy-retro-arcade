# Code map

- `game.py`: authoritative board, move validation, results, takebacks, PGN and notation. No Qt dependency.
- `storage.py`: versioned JSON session, bounded loading, XDG location, atomic text/byte writes.
- `theme.py`: small TOML reader with strict six-digit colour validation and fallback.
- `engine.py`: QThread job around python-chess's UCI client. A separate process per bounded search; cancellation closes its transport. Requests carry a game revision.
- `board.py`: scalable QPainter board using SVG pieces. Coordinate mapping handles both orientations. Emits move intent; never changes game state itself.
- `app.py`: native menus, dialogs, history, persistence orchestration and engine-result acceptance. Only current-job/current-revision legal moves may change the live board.

## State transitions

A human move is validated, outstanding hint work is cancelled, the revision advances, the game changes, UI and disk are updated, and an engine turn is requested if needed. New game, import, resignation, draw claim and takeback invalidate outstanding work. Engine results must match the active job and revision, and remain legal in the current board.

Engine exceptions surface in the window with Retry. No network fallback or random substitute move is used. Shutdown cancels workers and lets bounded startup/search work finish before Qt objects are destroyed.

## Persistence

PGN stores move history, including a custom starting FEN. JSON wraps mode, side, difficulty and board preferences. Loading reconstructs the full move stack so repetition detection survives restart. A malformed session cannot replace the live game; starting a new game archives the old bytes first. The lock is held for the application's lifetime, not merely during a write.

## Testing boundaries

Unit tests validate the application's integration with rules, imports and persistence rather than reimplementing chess. Qt tests exercise actual events and dialogs. Real engine tests cover Black's opening, hints and cancellation; a silent executable verifies startup timeout and responsive UI. CI tests an Ubuntu Python installation and an Arch package build separately.
