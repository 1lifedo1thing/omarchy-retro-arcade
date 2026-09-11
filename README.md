# Omarchy Chess

Open a board. Choose a side. Play.

A native Linux chess app for offline games against Stockfish or a friend at the same computer. It follows your active Omarchy colours and keeps your unfinished game ready for next time.

![Native Omarchy Chess preview](docs/preview.png)

**Development preview, not a stable release.** Independent community project; not an official Omarchy component.

## What works

- Computer play as White or Black, with four strength settings; local two-player mode.
- Click, drag, arrow-key board navigation and typed algebraic/coordinate moves.
- Legal moves, castling, en passant, all four promotions, checkmate, stalemate, automatic draws, draw claims and resignation.
- Last-move/check highlights, captured pieces, move history and read-only position review.
- Optional move guides, hints, takebacks and board flipping.
- Atomic autosave, automatic resume, single-instance session protection and archived games.
- PGN main-line import/export, including standard-chess custom starting positions.
- Live Omarchy theme updates; a built-in fallback palette on other Linux desktops.

No account, telemetry, online service or runtime downloads. Dependencies must be installed first.

## Run from source

Requires Python 3.11+ and a Linux desktop with Qt's system libraries. Stockfish is a separate executable.

```bash
git clone --branch feat/native-chess-preview https://github.com/tcballard/omarchy-chess.git
cd omarchy-chess
python3 -m venv .venv
.venv/bin/python -m pip install .
.venv/bin/omarchy-chess
```

Install Stockfish using your distribution's supported package route, or obtain a Linux executable from [Stockfish's official downloads](https://stockfishchess.org/download/). The app checks PATH, `/usr/bin/stockfish` and `/usr/games/stockfish`. For an explicit executable:

```bash
OMARCHY_CHESS_ENGINE=/absolute/path/to/stockfish .venv/bin/omarchy-chess
```

This variable is one executable path, never a shell command. Without Stockfish, choose **New game → Friend**; local play remains available. Missing or failed engines expose **Retry engine** after a computer move is requested.

## Arch / Omarchy packaging

CI builds native `.pkg.tar.zst` development artifacts. Download the `arch-preview` artifact from a passing [Actions run](https://github.com/tcballard/omarchy-chess/actions), extract it, inspect the packages, then install the application and rules dependency together:

```bash
sudo pacman -U ./python-chess-*.pkg.tar.zst ./omarchy-chess-*.pkg.tar.zst
```

Stockfish is a separate optional package/executable and is needed for computer games and hints. No AUR account or marketplace submission is needed to build these packages. These are development artifacts, not a signed release channel.

To build packages locally, first install the build dependencies (`base-devel`, `git`, `python-build`, `python-installer`, `python-setuptools`, `python-wheel`, `python-pytest`, `pyside6`, `qt6-svg`, `qt6-wayland`). Build and install `packaging/python-chess` if `python-chess` is not already available. Then, from a clean committed checkout, run:

```bash
./packaging/build-arch.sh
```

The script builds but never installs or invokes sudo. It snapshots the exact commit and checksums that source archive. The resulting app package adds a desktop launcher and icon.

## Controls

| Action | Control |
|---|---|
| Select / move | Click twice or drag a piece |
| Navigate board | Arrow keys; Enter/Space to select; Escape to clear |
| Type a move | Move field: `e4`, `Nf3`, `O-O`, `e2e4`, `e7e8n` |
| New game | Ctrl+N |
| Import / export PGN | Ctrl+O / Ctrl+S |
| Take back / hint | Ctrl+Z / Ctrl+H |
| Flip / return live | Ctrl+F / Ctrl+L |
| Help / quit | F1 / Ctrl+Q |

Takeback against the computer normally returns to your previous turn. Local mode undoes one move. Draw claims are offered when python-chess finds a valid threefold-repetition or fifty-move claim, including claims available through an intended legal move. Fivefold repetition and the seventy-five-move rule end games automatically.

## Files and behaviour

- Session: `${XDG_STATE_HOME:-~/.local/state}/omarchy-chess/session.json`.
- Previous games: the same directory's `archive/`, before a new game or import replaces them.
- Theme: `${XDG_CONFIG_HOME:-~/.config}/omarchy/current/theme/colors.toml`, checked every two seconds.
- Corrupt saves remain untouched until a new game/import archives a recovery copy. Failed writes are reported; export PGN to keep a copy.
- Imports accept **one standard-chess game up to 1 MB** and use its main line. Comments and side variations are not retained in the new export. Original files are untouched. Imported unfinished games continue in local mode.

## Development and review

```bash
.venv/bin/python -m pip install pytest ruff build
QT_QPA_PLATFORM=offscreen .venv/bin/python -m pytest -q
.venv/bin/ruff check omarchy_chess tests
.venv/bin/python -m build
```

Real-engine tests skip when Stockfish is absent; CI explicitly installs it for the Python test jobs. Headless Qt tests are not a substitute for real Wayland/Hyprland testing.

Read [DECISIONS.md](DECISIONS.md) for judgement calls, [docs/VERIFICATION.md](docs/VERIFICATION.md) for evidence and desktop acceptance checks, and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the code map.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) and [THIRD_PARTY.md](THIRD_PARTY.md). Powered by Stockfish, python-chess and Qt for Python.
