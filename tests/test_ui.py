import os
os.environ.setdefault("QT_QPA_PLATFORM", "offscreen")
import time
import chess
import pytest
from PySide6.QtCore import QPointF, Qt
from PySide6.QtTest import QTest
from PySide6.QtWidgets import QApplication, QDialog, QComboBox
from PySide6.QtCore import QTimer
from omarchy_chess.app import Window
from omarchy_chess.engine import EngineJob, find_engine
from omarchy_chess.game import Game


@pytest.fixture(scope="session")
def app():
    return QApplication.instance() or QApplication([])


@pytest.fixture
def window(app, tmp_path):
    widget = Window(tmp_path, restore=False)
    widget.game = Game(mode="local")
    widget.refresh()
    widget.show()
    app.processEvents()
    yield widget
    widget.close()
    until = time.monotonic() + 6
    while widget.jobs and time.monotonic() < until:
        app.processEvents()
        time.sleep(.01)
    assert not widget.jobs


def wait_for(app, predicate, seconds=8):
    deadline = time.monotonic() + seconds
    while not predicate() and time.monotonic() < deadline:
        app.processEvents()
        time.sleep(.01)
    assert predicate()


def test_board_click_move_and_history_read_only(window, app):
    board = window.board
    QTest.mouseClick(board, Qt.MouseButton.LeftButton, pos=board.cell(chess.E2).center().toPoint())
    QTest.mouseClick(board, Qt.MouseButton.LeftButton, pos=board.cell(chess.E4).center().toPoint())
    assert window.game.board.peek().uci() == "e2e4"
    window.history.setCurrentRow(0)
    assert window.preview and not board.interactive
    before = window.game.board.fen()
    window.accept_move(chess.Move.from_uci("e7e5"))
    assert window.game.board.fen() == before
    window.return_live()
    assert not window.preview and board.interactive


def test_keyboard_move_and_flip_geometry(window):
    board = window.board
    board.setFocus()
    board.cursor = chess.E2
    QTest.keyClick(board, Qt.Key.Key_Return)
    QTest.keyClick(board, Qt.Key.Key_Up)
    QTest.keyClick(board, Qt.Key.Key_Up)
    QTest.keyClick(board, Qt.Key.Key_Return)
    assert window.game.board.peek().uci() == "e2e4"
    for flipped in (False, True):
        board.flipped = flipped
        for square in chess.SQUARES:
            assert board.square_at(board.cell(square).center()) == square
    assert board.square_at(QPointF(-1, -1)) is None


def test_promotion_dialog_supports_knight(window, app):
    window.game = Game(mode="local", board=chess.Board("7k/P7/8/8/8/8/8/7K w - - 0 1"))
    window.refresh()
    def choose():
        dialog = app.activeModalWidget()
        assert isinstance(dialog, QDialog)
        dialog.findChild(QComboBox).setCurrentIndex(3)
        dialog.accept()
    QTimer.singleShot(50, choose)
    window.board_move(chess.A7, chess.A8)
    assert window.game.board.piece_at(chess.A8).piece_type == chess.KNIGHT


def test_stale_engine_response_cannot_mutate_new_game(window):
    job = EngineJob(window.revision, window.game.board, "Gentle")
    window.active_job = job
    previous_revision = window.revision
    window.cancel_engine()
    before = window.game.board.fen()
    window.engine_answer(job, previous_revision, chess.Move.from_uci("e2e4"), "")
    assert window.game.board.fen() == before


def test_missing_engine_is_recoverable(window, app, monkeypatch):
    monkeypatch.setenv("OMARCHY_CHESS_ENGINE", "/nonexistent/chess-engine")
    window.game = Game(human=False)
    window.refresh()
    window.maybe_engine()
    wait_for(app, lambda: bool(window.engine_error))
    assert "not found" in window.engine_error
    assert not window.game.board.move_stack
    assert window.retry_button.isVisible()


@pytest.mark.skipif(find_engine() is None, reason="Stockfish not installed")
def test_real_engine_black_start_hint_and_shutdown(window, app):
    window.game = Game(human=False, difficulty="Gentle")
    window.refresh()
    window.maybe_engine()
    wait_for(app, lambda: len(window.game.board.move_stack) == 1 or bool(window.engine_error))
    assert not window.engine_error
    assert window.game.human_turn
    window.hint()
    wait_for(app, lambda: window.board.hint is not None or bool(window.engine_error))
    assert not window.engine_error
    assert window.board.hint in window.game.board.legal_moves
    assert len(window.game.board.move_stack) == 1


def test_drag_and_outside_drop(window):
    board = window.board
    start = board.cell(chess.D2).center().toPoint()
    end = board.cell(chess.D4).center().toPoint()
    QTest.mousePress(board, Qt.MouseButton.LeftButton, pos=start)
    QTest.mouseMove(board, end)
    QTest.mouseRelease(board, Qt.MouseButton.LeftButton, pos=end)
    assert window.game.board.peek().uci() == "d2d4"


def test_resume_preserves_unfinished_game(app, tmp_path):
    first = Window(tmp_path, restore=False)
    first.game = Game(mode="local")
    first.accept_move(chess.Move.from_uci("e2e4"))
    first.close()
    second = Window(tmp_path)
    assert second.game.board.peek().uci() == "e2e4"
    assert second.game.mode == "local"
    second.close()
