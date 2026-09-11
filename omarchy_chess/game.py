"""Game state, independent of Qt and the engine."""

from __future__ import annotations

import io
from dataclasses import dataclass, field
from datetime import date

import chess
import chess.pgn

MAX_PGN_BYTES = 1_000_000
DIFFICULTIES = {"Gentle": (0, 0.15), "Casual": (5, 0.3), "Club": (10, 0.6), "Strong": (20, 1.0)}


@dataclass
class Game:
    board: chess.Board = field(default_factory=chess.Board)
    mode: str = "computer"
    human: chess.Color = chess.WHITE
    difficulty: str = "Casual"
    result: str = "*"
    ending: str = ""
    headers: dict[str, str] = field(default_factory=dict)

    @property
    def finished(self) -> bool:
        return self.result != "*" or self.board.is_game_over()

    @property
    def human_turn(self) -> bool:
        return not self.finished and (self.mode == "local" or self.board.turn == self.human)

    def play(self, move: chess.Move) -> None:
        if self.finished or move not in self.board.legal_moves:
            raise ValueError("That move is not legal in this position.")
        self.board.push(move)

    def takeback(self) -> None:
        if not self.board.move_stack:
            return
        self.result, self.ending = "*", ""
        self.board.pop()
        if self.mode == "computer" and self.board.turn != self.human and self.board.move_stack:
            self.board.pop()

    def claim_draw(self) -> None:
        if self.finished or not self.board.can_claim_draw():
            raise ValueError("No draw can be claimed here.")
        self.result, self.ending = "1/2-1/2", "Draw claimed"

    def resign(self) -> None:
        if self.finished:
            return
        loser = self.human if self.mode == "computer" else self.board.turn
        self.result = "0-1" if loser == chess.WHITE else "1-0"
        self.ending = f"{'White' if loser else 'Black'} resigned"

    def status(self) -> str:
        if self.result != "*":
            return f"{self.ending or 'Game finished'} · {self.result}"
        outcome = self.board.outcome()
        if outcome:
            reason = outcome.termination.name.replace("_", " ").capitalize()
            return f"{reason} · {outcome.result()}"
        side = "White" if self.board.turn else "Black"
        return f"{side} to move" + (" · Check" if self.board.is_check() else "")

    def notation(self) -> list[tuple[str, str]]:
        board = self.board.root()
        rows = []
        for move in self.board.move_stack:
            prefix = f"{board.fullmove_number}." if board.turn else f"{board.fullmove_number}…"
            rows.append((prefix, board.san(move)))
            board.push(move)
        return rows

    def pgn(self) -> str:
        game = chess.pgn.Game.from_board(self.board)
        game.headers.update(self.headers)
        game.headers.setdefault("Date", date.today().strftime("%Y.%m.%d"))
        if game.headers.get("Date") == "????.??.??":
            game.headers["Date"] = date.today().strftime("%Y.%m.%d")
        game.headers["Event"] = self.headers.get("Event", "Omarchy Chess")
        game.headers["White"] = self.headers.get(
            "White", "Stockfish" if self.mode == "computer" and not self.human else "White"
        )
        game.headers["Black"] = self.headers.get(
            "Black", "Stockfish" if self.mode == "computer" and self.human else "Black"
        )
        game.headers["Result"] = self.result if self.result != "*" else self.board.result()
        # FEN and SetUp come from the actual board, never stale imported headers.
        if self.board.root().fen() != chess.STARTING_FEN:
            game.setup(self.board.root())
        else:
            for key in ("FEN", "SetUp"):
                game.headers.pop(key, None)
        return str(game) + "\n"

    @classmethod
    def from_pgn(cls, text: str) -> Game:
        if len(text.encode("utf-8")) > MAX_PGN_BYTES:
            raise ValueError("PGN is too large (maximum 1 MB).")
        stream = io.StringIO(text)
        parsed = chess.pgn.read_game(stream)
        if parsed is None or parsed.errors:
            raise ValueError("PGN is empty or contains invalid moves.")
        if parsed.headers.get("Variant", "Standard") not in ("Standard", "Chess", "Normal"):
            raise ValueError("Only standard chess is supported.")
        board = parsed.board()
        if not board.is_valid():
            raise ValueError("PGN contains an invalid starting position.")
        for move in parsed.mainline_moves():
            if board.is_game_over() or move not in board.legal_moves:
                raise ValueError("PGN contains a move after game over or an illegal move.")
            board.push(move)
        if chess.pgn.read_game(stream) is not None:
            raise ValueError("Import one game at a time. This file contains multiple games.")
        result = parsed.headers.get("Result", "*")
        if result not in ("*", "1-0", "0-1", "1/2-1/2"):
            raise ValueError("Invalid PGN result.")
        outcome = board.outcome()
        if outcome and result not in ("*", outcome.result()):
            raise ValueError("PGN result contradicts the final position.")
        return cls(
            board=board,
            mode="local",
            result=result,
            ending="Imported result" if result != "*" else "",
            headers=dict(parsed.headers),
        )
