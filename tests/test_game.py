import chess
import pytest
from omarchy_chess.game import Game
from omarchy_chess.storage import load, save
from omarchy_chess.theme import DEFAULT, read_theme


def play(game, *moves):
    for text in moves:
        game.play(game.board.parse_san(text))


def test_mate_and_illegal_move_rejection():
    game = Game(mode="local")
    play(game, "f3", "e5", "g4", "Qh4#")
    assert game.finished and "Checkmate" in game.status()
    with pytest.raises(ValueError):
        game.play(chess.Move.from_uci("a2a3"))


def test_castling_and_en_passant_are_recorded_correctly():
    game = Game(mode="local")
    play(game, "e4", "a6", "e5", "d5", "exd6")
    assert game.board.piece_at(chess.D5) is None
    assert game.board.piece_at(chess.D6) == chess.Piece(chess.PAWN, chess.WHITE)
    game = Game(mode="local")
    play(game, "e4", "e5", "Nf3", "Nc6", "Bc4", "Nf6", "O-O")
    assert game.board.king(chess.WHITE) == chess.G1
    assert game.board.piece_at(chess.F1).piece_type == chess.ROOK
    assert Game.from_pgn(game.pgn()).board.fen() == game.board.fen()


@pytest.mark.parametrize("promotion", [chess.QUEEN, chess.ROOK, chess.BISHOP, chess.KNIGHT])
def test_underpromotion(promotion):
    game = Game(board=chess.Board("7k/P7/8/8/8/8/8/7K w - - 0 1"))
    game.play(chess.Move(chess.A7, chess.A8, promotion=promotion))
    assert game.board.piece_at(chess.A8).piece_type == promotion


@pytest.mark.parametrize("fen,reason", [
    ("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1", "Stalemate"),
    ("7k/8/6K1/8/8/8/8/8 w - - 0 1", "Insufficient material"),
    ("7k/8/6K1/8/8/8/8/R7 w - - 150 90", "Seventyfive moves"),
])
def test_automatic_draws(fen, reason):
    game = Game(board=chess.Board(fen))
    assert game.finished and reason in game.status()


def test_threefold_is_claimed_not_automatic_and_survives_save(tmp_path):
    game = Game(mode="local")
    play(game, "Nf3", "Nf6", "Ng1", "Ng8", "Nf3", "Nf6", "Ng1", "Ng8")
    assert not game.finished
    path = tmp_path / "session.json"
    save(path, game, False, True)
    restored, *_ = load(path)
    assert restored.board.can_claim_threefold_repetition()
    restored.claim_draw()
    assert restored.result == "1/2-1/2"
    restored.takeback()
    assert not restored.finished


def test_fifty_move_claim():
    game = Game(board=chess.Board("7k/8/6K1/8/8/8/8/R7 w - - 100 90"))
    assert not game.finished
    game.claim_draw()
    assert game.finished


def test_takeback_after_engine_reply_and_while_thinking():
    game = Game()
    play(game, "e4", "e5")
    game.takeback()
    assert game.board.fen() == chess.STARTING_FEN
    play(game, "d4")
    game.takeback()
    assert game.board.fen() == chess.STARTING_FEN


def test_black_takeback_and_resignation():
    game = Game(human=chess.BLACK)
    play(game, "e4", "e5", "Nf3")
    game.takeback()
    assert len(game.board.move_stack) == 1 and game.human_turn
    game.resign()
    assert game.result == "1-0"


def test_roundtrip_settings_and_custom_position(tmp_path):
    game = Game(board=chess.Board("7k/8/6K1/8/8/8/8/R7 w - - 0 1"), human=False, difficulty="Club")
    play(game, "Ra2")
    path = tmp_path / "nested/session.json"
    save(path, game, True, False)
    restored, flipped, guides = load(path)
    assert restored.board.fen() == game.board.fen()
    assert restored.board.root().fen() == game.board.root().fen()
    assert (restored.human, restored.difficulty, flipped, guides) == (False, "Club", True, False)


@pytest.mark.parametrize("pgn", ["", '[FEN "8/8/8/8/8/8/8/8 w - - 0 1"]\n\n*',
    '[Variant "Atomic"]\n\n1. e4 *', '1. e4 e5 2. Bh6 *',
    '[Result "nonsense"]\n\n*', '1. e4 *\n\n[Event "Other"]\n\n1. d4 *'])
def test_bad_import_rejected(pgn):
    with pytest.raises(ValueError):
        Game.from_pgn(pgn)


def test_corrupt_session_never_modified_by_load(tmp_path):
    path = tmp_path / "session.json"
    path.write_text('{"version": 800}')
    before = path.read_bytes()
    with pytest.raises(ValueError):
        load(path)
    assert path.read_bytes() == before


def test_theme_validation_and_partial_fallback(tmp_path):
    path = tmp_path / "colors.toml"
    path.write_text('accent = "#123456"\nbackground = "url(evil)"')
    result = read_theme(path)
    assert result["accent"] == "#123456"
    assert result["background"] == DEFAULT["background"]
    path.write_text('not valid toml')
    assert read_theme(path) == DEFAULT
