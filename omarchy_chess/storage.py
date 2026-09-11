"""Atomic session persistence. Malformed sessions never replace live state."""
import json
import os
import tempfile
from pathlib import Path

from .game import DIFFICULTIES, Game, MAX_PGN_BYTES


def state_directory() -> Path:
    return Path(os.environ.get("XDG_STATE_HOME", str(Path.home() / ".local/state"))) / "omarchy-chess"


def atomic_write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, name = tempfile.mkstemp(prefix=".chess-", dir=path.parent)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as file:
            file.write(text)
            file.flush()
            os.fsync(file.fileno())
        os.replace(name, path)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def save(path: Path, game: Game, flipped: bool, guides: bool) -> None:
    atomic_write(path, json.dumps({"version": 1, "pgn": game.pgn(), "mode": game.mode,
        "human": game.human, "difficulty": game.difficulty, "ending": game.ending,
        "flipped": flipped, "guides": guides}, indent=2))


def load(path: Path) -> tuple[Game, bool, bool]:
    if path.stat().st_size > MAX_PGN_BYTES * 2:
        raise ValueError("Saved session is too large.")
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("version") != 1:
        raise ValueError("Unknown saved-session format.")
    if data.get("mode") not in ("computer", "local") or data.get("difficulty") not in DIFFICULTIES:
        raise ValueError("Invalid saved-game settings.")
    if any(type(data.get(key)) is not bool for key in ("human", "flipped", "guides")):
        raise ValueError("Invalid saved-game preferences.")
    if not isinstance(data.get("pgn"), str) or not isinstance(data.get("ending", ""), str):
        raise ValueError("Invalid saved-game data.")
    game = Game.from_pgn(data["pgn"])
    game.mode, game.human, game.difficulty = data["mode"], data["human"], data["difficulty"]
    game.ending = data.get("ending", "")
    return game, data["flipped"], data["guides"]
