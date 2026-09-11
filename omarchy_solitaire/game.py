"""Pure Klondike rules. No Qt, filesystem, clocks or UI dependencies.

Pile IDs: stock=0, waste=1, foundations=2..5, tableau=6..12.
Lists run bottom to top; rank 1 is ace. Every mutation is transactional.
"""
from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
import random
import secrets

SUITS = ("clubs", "diamonds", "hearts", "spades")
SYMBOLS = ("♣", "♦", "♥", "♠")
RANKS = ("", "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K")


@dataclass
class Card:
    id: int
    up: bool = False

    @property
    def rank(self):
        return self.id % 13 + 1

    @property
    def suit(self):
        return self.id // 13

    @property
    def red(self):
        return self.suit in (1, 2)

    @property
    def name(self):
        return f"{RANKS[self.rank]} of {SUITS[self.suit]}"


class Game:

    def __init__(self, seed=None, draw=1):
        if draw not in (1, 3):
            raise ValueError("Draw must be one or three")
        self.seed = secrets.randbits(32) if seed is None else seed
        self.draw_count = draw
        self.piles: list[list[Card]] = [[] for _ in range(13)]
        deck = [Card(i) for i in range(52)]
        random.Random(self.seed).shuffle(deck)
        for column in range(7):
            self.piles[6 + column] = [deck.pop() for _ in range(column + 1)]
            self.piles[6 + column][-1].up = True
        self.piles[0] = deck
        self.moves = 0
        self.score = 0
        self.passes = 0
        self.history: list[dict] = []

    @property
    def won(self):
        return sum(len(p) for p in self.piles[2:6]) == 52

    @property
    def can_complete(self):
        return (not self.won and not self.piles[0] and not self.piles[1]
                and all(c.up for p in self.piles[6:] for c in p))

    def snapshot(self):
        return {"seed": self.seed, "draw": self.draw_count, "moves": self.moves,
                "score": self.score, "passes": self.passes,
                "piles": [[[c.id, c.up] for c in p] for p in self.piles]}

    def _restore(self, state):
        self.seed = state["seed"]
        self.draw_count = state["draw"]
        self.moves, self.score, self.passes = state["moves"], state["score"], state["passes"]
        self.piles = [[Card(*c) for c in p] for p in state["piles"]]

    def _remember(self):
        self.history.append(self.snapshot())

    def undo(self):
        if not self.history:
            return False
        self._restore(self.history.pop())
        return True

    def draw(self):
        if self.won or (not self.piles[0] and not self.piles[1]):
            return False
        self._remember()
        if self.piles[0]:
            for _ in range(min(self.draw_count, len(self.piles[0]))):
                card = self.piles[0].pop()
                card.up = True
                self.piles[1].append(card)
        else:
            self.piles[0] = list(reversed(self.piles[1]))
            self.piles[1] = []
            for card in self.piles[0]:
                card.up = False
            self.passes += 1
            self.score = max(0, self.score - 20)
        self.moves += 1
        return True

    def movable(self, source, index):
        if not 1 <= source < 13 or not 0 <= index < len(self.piles[source]):
            return False
        tail = self.piles[source][index:]
        if source < 6:
            return index == len(self.piles[source]) - 1 and tail[0].up
        return all(c.up for c in tail) and all(
            a.rank == b.rank + 1 and a.red != b.red for a, b in zip(tail, tail[1:]))

    def legal(self, source, index, target):
        if self.won or source == target or not 2 <= target < 13 or not self.movable(source, index):
            return False
        tail = self.piles[source][index:]
        pile = self.piles[target]
        card = tail[0]
        if target < 6:
            return len(tail) == 1 and (
                (not pile and card.rank == 1) or
                (bool(pile) and pile[-1].suit == card.suit and pile[-1].rank + 1 == card.rank))
        return ((not pile and card.rank == 13) or
                (bool(pile) and pile[-1].up and pile[-1].rank == card.rank + 1 and pile[-1].red != card.red))

    def move(self, source, index, target):
        if not self.legal(source, index, target):
            return False
        self._remember()
        self.piles[target].extend(self.piles[source][index:])
        del self.piles[source][index:]
        if target < 6:
            self.score += 10
        elif source < 6 and source != 1:
            self.score = max(0, self.score - 15)
        elif source == 1:
            self.score += 5
        if source >= 6 and self.piles[source] and not self.piles[source][-1].up:
            self.piles[source][-1].up = True
            self.score += 5
        self.moves += 1
        return True

    def legal_moves(self):
        return [(s, i, t) for s in range(1, 13) for i in range(len(self.piles[s]))
                if self.movable(s, i) for t in range(2, 13) if self.legal(s, i, t)]

    def hint(self):
        """One legal suggestion, ranked for revealing cards. Not a solver."""
        candidates = []
        for s, i, t in self.legal_moves():
            # Avoid shuffling whole kings between empty columns and backing off foundations.
            if s < 6 and s != 1:
                continue
            if t >= 6 and not self.piles[t] and s >= 6 and i == 0:
                continue
            reveal = s >= 6 and i > 0 and not self.piles[s][i - 1].up
            weight = (100 if reveal else 0) + (40 if t < 6 else 0) + (20 if s == 1 else 0)
            candidates.append((weight, (s, i, t)))
        if candidates:
            return max(candidates, key=lambda x: x[0])[1]
        if self.piles[0] or self.piles[1]:
            return (0, 0, 1)
        return None

    def completion_move(self):
        if not self.can_complete:
            return None
        return next(((s, len(self.piles[s]) - 1, t) for s in range(6, 13)
                     if self.piles[s] for t in range(2, 6)
                     if self.legal(s, len(self.piles[s]) - 1, t)), None)

    def serialize(self):
        return {"version": 1, "state": self.snapshot(), "history": deepcopy(self.history)}

    @classmethod
    def deserialize(cls, data):
        if not isinstance(data, dict) or data.get("version") != 1:
            raise ValueError("Unsupported save version")
        history = data.get("history", [])
        if not isinstance(history, list):
            raise ValueError("Invalid undo history")
        for state in [data.get("state")] + history:
            cls.validate(state)
        game = cls(0)
        game._restore(data["state"])
        game.history = deepcopy(history)
        return game

    @staticmethod
    def validate(state):
        if not isinstance(state, dict):
            raise ValueError("Invalid state")
        for key in ("seed", "moves", "score", "passes"):
            if type(state.get(key)) is not int or not 0 <= state[key] <= 2**63 - 1:
                raise ValueError(f"Invalid {key}")
        if type(state.get("draw")) is not int or state["draw"] not in (1, 3):
            raise ValueError("Invalid draw count")
        piles = state.get("piles")
        if not isinstance(piles, list) or len(piles) != 13 or any(not isinstance(p, list) for p in piles):
            raise ValueError("Invalid piles")
        cards = [c for p in piles for c in p]
        if len(cards) != 52 or any(not isinstance(c, list) or len(c) != 2 or
                type(c[0]) is not int or type(c[1]) is not bool for c in cards):
            raise ValueError("Invalid cards")
        if sorted(c[0] for c in cards) != list(range(52)):
            raise ValueError("Cards must be unique")
        if any(c[1] for c in piles[0]) or any(not c[1] for p in piles[1:6] for c in p):
            raise ValueError("Invalid face orientation")
        for p in piles[2:6]:
            if p and any(c[0] // 13 != p[0][0] // 13 or c[0] % 13 != i for i, c in enumerate(p)):
                raise ValueError("Invalid foundation")
        for p in piles[6:]:
            if p and not p[-1][1]:
                raise ValueError("Tableau top must be visible")
            seen_up = False
            last = None
            for value, up in p:
                if seen_up and not up:
                    raise ValueError("Hidden card above visible card")
                card = Card(value, up)
                if seen_up and last and (last.rank != card.rank + 1 or last.red == card.red):
                    raise ValueError("Invalid tableau sequence")
                seen_up = up
                last = card
