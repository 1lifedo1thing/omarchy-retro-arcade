"""Bounded, cancellable UCI work outside the UI thread."""

import os
import shutil
import threading
from pathlib import Path

import chess
import chess.engine
from PySide6.QtCore import QThread, Signal

from .game import DIFFICULTIES


def find_engine() -> str | None:
    configured = os.environ.get("OMARCHY_CHESS_ENGINE")
    if configured:
        # One executable, not a shell command or a list of arguments.
        return shutil.which(configured)
    return shutil.which("stockfish") or next(
        (str(p) for p in (Path("/usr/games/stockfish"), Path("/usr/bin/stockfish")) if os.access(p, os.X_OK)),
        None,
    )


class EngineJob(QThread):
    answer = Signal(int, object, str)

    def __init__(self, revision: int, board: chess.Board, difficulty: str, hint: bool = False):
        super().__init__()
        self.revision, self.board = revision, board.copy(stack=True)
        self.difficulty, self.hint = difficulty, hint
        self.cancelled = threading.Event()
        self.lock = threading.Lock()
        self.engine = None

    def cancel(self):
        self.cancelled.set()
        with self.lock:
            if self.engine is not None:
                self.engine.close()

    def run(self):
        engine = None
        try:
            path = find_engine()
            if not path:
                raise RuntimeError(
                    "Stockfish was not found. Install the stockfish package, then choose Retry engine. Local two-player games still work."
                )
            if self.cancelled.is_set():
                return
            engine = chess.engine.SimpleEngine.popen_uci(path, timeout=3.0)
            with self.lock:
                self.engine = engine
                if self.cancelled.is_set():
                    return
            skill, seconds = DIFFICULTIES["Strong" if self.hint else self.difficulty]
            options = {"Threads": 1, "Hash": 32, "Skill Level": skill}
            engine.configure({key: value for key, value in options.items() if key in engine.options})
            answer = engine.play(self.board, chess.engine.Limit(time=seconds))
            if not self.cancelled.is_set():
                if answer.move is None or answer.move not in self.board.legal_moves:
                    raise RuntimeError("The engine returned no legal move. Choose Retry engine.")
                self.answer.emit(self.revision, answer.move, "")
        except Exception as error:
            if not self.cancelled.is_set():
                self.answer.emit(
                    self.revision, None, str(error) or "The engine stopped unexpectedly. Choose Retry engine."
                )
        finally:
            if engine is not None:
                engine.close()
            with self.lock:
                self.engine = None
