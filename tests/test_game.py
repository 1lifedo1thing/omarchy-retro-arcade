import random
import unittest
from copy import deepcopy

from omarchy_solitaire.game import Card, Game


def arranged(tableaux=None, waste=None, foundations=None, draw=1):
    """Legal fixture builder, unused cards remain face down in stock."""
    game = Game(1, draw)
    game.piles = [[] for _ in range(13)]
    for i, cards in (tableaux or {}).items():
        game.piles[i] = [Card(*c) if isinstance(c, tuple) else Card(c, True) for c in cards]
    game.piles[1] = [Card(c, True) for c in waste or []]
    for i, cards in (foundations or {}).items():
        game.piles[i] = [Card(c, True) for c in cards]
    used = {c.id for p in game.piles for c in p}
    game.piles[0] = [Card(c) for c in range(52) if c not in used]
    Game.validate(game.snapshot())
    return game


class GameTests(unittest.TestCase):
    def test_deal_unique_reproducible_and_hidden(self):
        for draw in (1, 3):
            game = Game(314159, draw)
            self.assertEqual(game.snapshot(), Game(314159, draw).snapshot())
            self.assertEqual([len(p) for p in game.piles], [24, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7])
            self.assertEqual(sum(c.up for p in game.piles for c in p), 7)
            Game.validate(game.snapshot())

    def test_draw_three_partial_packet_recycle_preserves_order_and_undo(self):
        game = Game(3, 3)
        original = game.snapshot()
        order = [c.id for c in game.piles[0]]
        game.draw()
        self.assertEqual([c.id for c in game.piles[1]], order[-3:][::-1])
        for _ in range(7):
            game.draw()
        game.draw()
        self.assertEqual([c.id for c in game.piles[0]], order)
        self.assertTrue(all(not c.up for c in game.piles[0]))
        self.assertEqual(game.passes, 1)
        for _ in range(9):
            self.assertTrue(game.undo())
        self.assertEqual(game.snapshot(), original)
        partial = arranged(waste=list(range(50)), draw=3)
        partial.draw()
        self.assertEqual(len(partial.piles[0]), 0)
        self.assertEqual(len(partial.piles[1]), 52)

    def test_tableau_sequence_reveal_and_atomic_invalid_move(self):
        # Hidden spade ace, black 8, red 7 move together onto red 9.
        game = arranged({6: [(39, False), 7, 19], 7: [21]})
        before = game.snapshot()
        self.assertFalse(game.move(6, 0, 7))
        self.assertFalse(game.move(6, 1, 0))
        self.assertFalse(game.move(6, 1, 6))
        self.assertEqual(before, game.snapshot())
        self.assertTrue(game.move(6, 1, 7))
        self.assertTrue(game.piles[6][0].up)
        self.assertEqual(game.score, 5)
        self.assertEqual([c.id for c in game.piles[7]], [21, 7, 19])
        self.assertTrue(game.undo())
        self.assertEqual(before, game.snapshot())

    def test_foundations_same_suit_ascending_and_back_to_table(self):
        game = arranged({6: [28]}, waste=[0, 1], foundations={2: []})
        self.assertFalse(game.move(1, 0, 2))  # buried waste ace cannot move
        game = arranged({6: [28]}, waste=[1], foundations={2: [0]})
        self.assertTrue(game.move(1, 0, 2))
        self.assertEqual(game.score, 10)
        self.assertTrue(game.move(2, 1, 6))
        self.assertEqual(game.score, 0)
        game = arranged(waste=[13], foundations={2: [0]})
        self.assertFalse(game.move(1, 0, 2))
        self.assertTrue(game.move(1, 0, 3))

    def test_only_kings_in_empty_columns(self):
        game = arranged({6: [12, 24], 7: [6]})
        self.assertFalse(game.move(7, 0, 8))
        self.assertTrue(game.move(6, 0, 8))

    def test_finish_only_fully_revealed_and_no_stock_or_waste(self):
        game = arranged({6: [12], 7: [25], 8: [38], 9: [51]},
                        foundations={2: list(range(12)), 3: list(range(13, 25)),
                                     4: list(range(26, 38)), 5: list(range(39, 51))})
        self.assertTrue(game.can_complete)
        for _ in range(4):
            self.assertTrue(game.move(*game.completion_move()))
        self.assertTrue(game.won)
        self.assertFalse(game.can_complete)
        self.assertIsNone(game.hint())
        self.assertTrue(game.undo())
        self.assertFalse(game.won)
        self.assertFalse(Game(1).can_complete)

    def test_persistence_rejects_duplicate_missing_invalid_and_bad_history(self):
        valid = Game(2).serialize()
        corruptions = []
        x = deepcopy(valid); x["state"]["piles"][0][0] = x["state"]["piles"][0][1]; corruptions.append(x)
        x = deepcopy(valid); x["state"]["piles"][0].pop(); corruptions.append(x)
        x = deepcopy(valid); x["state"]["draw"] = True; corruptions.append(x)
        x = deepcopy(valid); x["state"]["piles"][0][0][1] = True; corruptions.append(x)
        x = deepcopy(valid); x["history"] = [None]; corruptions.append(x)
        for corruption in corruptions:
            with self.assertRaises(ValueError):
                Game.deserialize(corruption)

    def test_full_undo_history_beyond_500_moves(self):
        game = Game(7)
        initial = game.snapshot()
        for _ in range(550):
            game.draw()
        restored = Game.deserialize(game.serialize())
        for _ in range(550):
            self.assertTrue(restored.undo())
        self.assertEqual(restored.snapshot(), initial)

    def test_random_play_invariants_roundtrip_and_undo(self):
        rng = random.Random(42)
        for seed in range(20):
            game = Game(seed, 1 if seed % 2 else 3)
            for _ in range(120):
                before = game.snapshot()
                moves = game.legal_moves()
                if moves and rng.random() < .75:
                    game.move(*rng.choice(moves))
                elif game.piles[0] or game.piles[1]:
                    game.draw()
                else:
                    break
                Game.validate(game.snapshot())
                restored = Game.deserialize(game.serialize())
                self.assertEqual(restored.snapshot(), game.snapshot())
                self.assertTrue(restored.undo())
                self.assertEqual(restored.snapshot(), before)


if __name__ == "__main__":
    unittest.main()
