"""Native desktop application. Run with python -m omarchy_chess."""

from __future__ import annotations

import argparse
import sys
from datetime import datetime, timezone
from pathlib import Path

import chess
from PySide6.QtCore import QLockFile, Qt, QTimer
from PySide6.QtGui import QAction, QColor, QKeySequence
from PySide6.QtWidgets import (
    QApplication,
    QCheckBox,
    QComboBox,
    QDialog,
    QDialogButtonBox,
    QFileDialog,
    QFormLayout,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QListWidget,
    QMainWindow,
    QMessageBox,
    QPushButton,
    QVBoxLayout,
    QWidget,
)

from . import __version__
from .board import BoardWidget
from .engine import EngineJob, find_engine
from .game import DIFFICULTIES, MAX_PGN_BYTES, Game
from .storage import atomic_write, load, save, state_directory
from .theme import read_theme


class Window(QMainWindow):
    def __init__(self, state_dir: Path | None = None, restore: bool = True):
        super().__init__()
        self.setWindowTitle("Omarchy Chess")
        self.resize(1060, 780)
        self.setMinimumSize(740, 560)
        self.state_dir = state_dir or state_directory()
        self.session_path = self.state_dir / "session.json"
        self.game = Game()
        self.revision = 0
        self.jobs = set()
        self.active_job = None
        self.engine_error = ""
        self.save_error = ""
        self.preview = False
        self.closing = False
        self.board = BoardWidget()
        self.colors = None
        self._build_ui()
        self._menus()
        warning = ""
        if restore and self.session_path.exists():
            try:
                self.game, self.board.flipped, self.board.guides = load(self.session_path)
            except (OSError, ValueError, TypeError, KeyError) as error:
                warning = f"Your saved game could not be loaded: {error}\nThe original file has been left untouched."
                # Disable persistence until the user explicitly starts/imports a game.
                self.save_error = (
                    "Previous save needs attention. Start a new game to archive it and continue."
                )
        self.guides.setChecked(self.board.guides)
        self.apply_theme()
        self.theme_timer = QTimer(self)
        self.theme_timer.timeout.connect(self.apply_theme)
        self.theme_timer.start(2000)
        self.refresh()
        if warning:
            QTimer.singleShot(0, lambda: self.error("Saved game", warning))
        QTimer.singleShot(0, self.maybe_engine)

    def _build_ui(self):
        root = QWidget()
        layout = QVBoxLayout(root)
        layout.setContentsMargins(26, 18, 26, 18)
        layout.setSpacing(14)
        header = QHBoxLayout()
        title = QLabel("OMARCHY / CHESS")
        title.setObjectName("eyebrow")
        header.addWidget(title)
        header.addStretch()
        self.new_button = QPushButton("New game")
        self.new_button.clicked.connect(self.new_game)
        header.addWidget(self.new_button)
        layout.addLayout(header)
        body = QHBoxLayout()
        body.setSpacing(24)
        board_column = QVBoxLayout()
        self.top_player = QLabel()
        self.top_player.setObjectName("player")
        board_column.addWidget(self.top_player)
        board_column.addWidget(self.board, 1)
        self.bottom_player = QLabel()
        self.bottom_player.setObjectName("player")
        board_column.addWidget(self.bottom_player)
        body.addLayout(board_column, 1)
        panel = QWidget()
        panel.setObjectName("panel")
        panel.setFixedWidth(250)
        side = QVBoxLayout(panel)
        side.setContentsMargins(18, 18, 18, 18)
        side.setSpacing(12)
        self.status = QLabel()
        self.status.setObjectName("status")
        self.status.setWordWrap(True)
        side.addWidget(self.status)
        self.opponent = QLabel()
        self.opponent.setWordWrap(True)
        side.addWidget(self.opponent)
        line = QLabel("MOVE HISTORY")
        line.setObjectName("eyebrow")
        side.addWidget(line)
        self.history = QListWidget()
        self.history.setAccessibleName("Move history; select a move to view its position")
        self.history.currentRowChanged.connect(self.show_history)
        side.addWidget(self.history, 1)
        self.live_button = QPushButton("Return to live board")
        self.live_button.clicked.connect(self.return_live)
        side.addWidget(self.live_button)
        self.move_input = QLineEdit()
        self.move_input.setPlaceholderText("Enter move: e4 or e2e4")
        self.move_input.setAccessibleName("Enter a move in algebraic or coordinate notation")
        self.move_input.returnPressed.connect(self.text_move)
        side.addWidget(self.move_input)
        self.hint_button = QPushButton("Hint")
        self.hint_button.clicked.connect(self.hint)
        self.undo_button = QPushButton("Take back")
        self.undo_button.clicked.connect(self.takeback)
        row = QHBoxLayout()
        row.addWidget(self.hint_button)
        row.addWidget(self.undo_button)
        side.addLayout(row)
        self.retry_button = QPushButton("Retry engine")
        self.retry_button.clicked.connect(self.retry_engine)
        side.addWidget(self.retry_button)
        self.claim_button = QPushButton("Claim draw")
        self.claim_button.clicked.connect(self.claim_draw)
        side.addWidget(self.claim_button)
        flip = QPushButton("Flip board")
        flip.clicked.connect(self.flip)
        side.addWidget(flip)
        self.guides = QCheckBox("Show legal moves")
        self.guides.setChecked(True)
        self.guides.toggled.connect(self.toggle_guides)
        side.addWidget(self.guides)
        body.addWidget(panel)
        layout.addLayout(body, 1)
        self.message = QLabel("A quiet board. A good game.")
        self.message.setWordWrap(True)
        self.message.setTextFormat(Qt.TextFormat.PlainText)
        self.message.setAccessibleName("Game messages")
        layout.addWidget(self.message)
        self.setCentralWidget(root)
        self.board.move_requested.connect(self.board_move)
        self.board.square_focused.connect(self.statusBar().showMessage)

    def _menus(self):
        game_menu = self.menuBar().addMenu("&Game")
        view_menu = self.menuBar().addMenu("&View")
        help_menu = self.menuBar().addMenu("&Help")

        def action(menu, name, shortcut, callback):
            item = QAction(name, self)
            if shortcut:
                item.setShortcut(QKeySequence(shortcut))
            item.triggered.connect(callback)
            menu.addAction(item)
            return item

        action(game_menu, "&New game…", "Ctrl+N", self.new_game)
        action(game_menu, "&Import PGN…", "Ctrl+O", self.import_pgn)
        action(game_menu, "&Export PGN…", "Ctrl+S", self.export_pgn)
        action(game_menu, "Take &back", "Ctrl+Z", self.takeback)
        action(game_menu, "&Hint", "Ctrl+H", self.hint)
        action(game_menu, "&Resign…", "", self.resign)
        action(game_menu, "&Quit", "Ctrl+Q", self.close)
        action(view_menu, "&Flip board", "Ctrl+F", self.flip)
        action(view_menu, "&Live board", "Ctrl+L", self.return_live)
        action(help_menu, "&Keyboard controls", "F1", self.help)
        action(help_menu, "&About", "", self.about)

    def apply_theme(self):
        colors = read_theme()
        if colors == self.colors:
            return
        self.colors = colors
        bg, fg, accent = colors["background"], colors["foreground"], colors["accent"]
        panel = QColor(bg).lighter(125).name()
        self.setStyleSheet(f"""
            QMainWindow, QWidget {{ background: {bg}; color: {fg}; font-size: 14px; }}
            QWidget#panel {{ background: {panel}; border-radius: 10px; }}
            QWidget#panel QLabel, QWidget#panel QCheckBox {{ background: transparent; }}
            QLabel#eyebrow {{ font-size: 11px; letter-spacing: 2px; color: {accent}; }}
            QLabel#status {{ font-size: 26px; font-weight: 600; }}
            QLabel#player {{ font-size: 14px; font-weight: 600; padding-left: 22px; }}
            QPushButton {{ padding: 9px 12px; border: 1px solid {QColor(fg).darker(230).name()}; border-radius: 5px; }}
            QPushButton:hover {{ border-color: {accent}; }}
            QPushButton:focus, QLineEdit:focus, QListWidget:focus {{ border: 2px solid {accent}; }}
            QPushButton:disabled {{ color: {QColor(fg).darker(190).name()}; }}
            QLineEdit, QComboBox {{ padding: 8px; border: 1px solid {QColor(fg).darker(230).name()}; border-radius: 4px; }}
            QListWidget {{ border: 0; background: transparent; }}
            QListWidget::item {{ padding: 5px; }}
            QListWidget::item:selected {{ background: {accent}; color: {bg}; }}
            QMenu::item:selected {{ background: {accent}; color: {bg}; }}
            QCheckBox {{ spacing: 8px; }}
        """)
        self.board.accent = QColor(accent)
        self.board.ink = QColor(fg)
        # Blend accent into stable light/dark squares. Piece contrast stays predictable.
        a = QColor(accent)
        self.board.dark = QColor(
            int(a.red() * 0.35 + 45), int(a.green() * 0.35 + 45), int(a.blue() * 0.35 + 45)
        )
        self.board.light = QColor(
            int(a.red() * 0.12 + 195), int(a.green() * 0.12 + 195), int(a.blue() * 0.12 + 195)
        )
        self.board.update()

    def error(self, title, message):
        QMessageBox.warning(self, title, str(message))

    def persist(self):
        if self.save_error:
            self.message.setText(self.save_error)
            return
        try:
            save(self.session_path, self.game, self.board.flipped, self.board.guides)
        except OSError as error:
            self.save_error = f"Could not save this game: {error}. Export PGN to keep a copy."
            self.message.setText(self.save_error)

    def archive(self):
        stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
        if self.save_error and self.session_path.exists():
            atomic_write(
                self.state_dir / "archive" / f"{stamp}-recovery.json",
                self.session_path.read_bytes(),
            )
        if self.game.board.move_stack:
            atomic_write(self.state_dir / "archive" / f"{stamp}.pgn", self.game.pgn())
        self.save_error = ""

    def refresh(self):
        self.preview = False
        self.board.set_position(self.game.board, self.game.human_turn)
        self.status.setText(self.game.status())
        self.opponent.setText(
            f"Stockfish · {self.game.difficulty}\nYou play {'white' if self.game.human else 'black'}"
            if self.game.mode == "computer"
            else "Two players · Shared board"
        )
        self.history.blockSignals(True)
        self.history.clear()
        self.history.addItem("Starting position")
        for prefix, san in self.game.notation():
            self.history.addItem(f"{prefix:<6} {san}")
        self.history.setCurrentRow(self.history.count() - 1)
        self.history.scrollToBottom()
        self.history.blockSignals(False)
        self.live_button.hide()
        self.undo_button.setEnabled(bool(self.game.board.move_stack))
        self.claim_button.setVisible(self.game.human_turn and self.game.board.can_claim_draw())
        self.retry_button.setVisible(bool(self.engine_error))
        self.hint_button.setEnabled(
            self.game.human_turn and self.active_job is None and find_engine() is not None
        )
        self.move_input.setEnabled(self.game.human_turn)
        self.move_input.clear()
        captured = {True: [], False: []}
        replay = self.game.board.root()
        for move in self.game.board.move_stack:
            piece = replay.piece_at(move.to_square)
            if replay.is_en_passant(move):
                piece = chess.Piece(chess.PAWN, not replay.turn)
            if piece:
                captured[replay.turn].append(piece.unicode_symbol())
            replay.push(move)

        def label(color):
            player = (
                "Stockfish"
                if self.game.mode == "computer" and color != self.game.human
                else ("White" if color else "Black")
            )
            return f"{player}   {' '.join(captured[color])}"

        self.top_player.setText(label(chess.WHITE if self.board.flipped else chess.BLACK))
        self.bottom_player.setText(label(chess.BLACK if self.board.flipped else chess.WHITE))
        self.message.setText(
            self.save_error
            or self.engine_error
            or (
                "Game finished. Start another, or take back a move to explore."
                if self.game.finished
                else "Your move."
                if self.game.human_turn
                else "Stockfish is thinking…"
            )
        )

    def cancel_engine(self):
        self.revision += 1
        if self.active_job:
            self.active_job.cancel()
            self.active_job = None

    def start_engine(self, hint=False):
        if self.active_job or self.game.finished or self.closing:
            return
        job = EngineJob(self.revision, self.game.board, self.game.difficulty, hint)
        self.jobs.add(job)
        self.active_job = job
        job.answer.connect(lambda rev, move, error: self.engine_answer(job, rev, move, error))
        job.finished.connect(lambda: self.job_finished(job))
        self.message.setText("Finding a hint…" if hint else "Stockfish is thinking…")
        self.hint_button.setEnabled(False)
        job.start()

    def job_finished(self, job):
        self.jobs.discard(job)
        job.deleteLater()

    def maybe_engine(self):
        if not self.game.finished and not self.game.human_turn and not self.engine_error:
            self.start_engine()

    def engine_answer(self, job, revision, move, error):
        if self.closing or revision != self.revision or job is not self.active_job:
            return
        self.active_job = None
        if error:
            self.engine_error = error
            self.refresh()
            return
        if move not in self.game.board.legal_moves:
            return
        if job.hint:
            self.return_live()
            self.board.hint = move
            self.board.update()
            self.message.setText(f"Try {self.game.board.san(move)}. You choose whether to play it.")
            self.hint_button.setEnabled(True)
        else:
            self.game.play(move)
            self.revision += 1
            self.refresh()
            self.persist()

    def retry_engine(self):
        self.engine_error = ""
        self.refresh()
        self.maybe_engine()

    def accept_move(self, move):
        if self.preview or not self.game.human_turn:
            return
        if move not in self.game.board.legal_moves:
            self.message.setText("That move is not legal. Choose another square.")
            return
        self.cancel_engine()
        self.engine_error = ""
        self.game.play(move)
        self.refresh()
        self.persist()
        self.maybe_engine()

    def board_move(self, source, target):
        if self.preview or not self.game.human_turn:
            return
        candidates = [
            m for m in self.game.board.legal_moves if m.from_square == source and m.to_square == target
        ]
        if not candidates:
            self.message.setText("That move is not legal. Choose another square.")
            return
        move = candidates[0]
        if len(candidates) > 1:
            dialog = QDialog(self)
            dialog.setWindowTitle("Promote pawn")
            form = QFormLayout(dialog)
            choice = QComboBox()
            for kind in (chess.QUEEN, chess.ROOK, chess.BISHOP, chess.KNIGHT):
                choice.addItem(chess.piece_name(kind).capitalize(), kind)
            form.addRow("Promote to", choice)
            buttons = QDialogButtonBox(
                QDialogButtonBox.StandardButton.Ok | QDialogButtonBox.StandardButton.Cancel
            )
            buttons.accepted.connect(dialog.accept)
            buttons.rejected.connect(dialog.reject)
            form.addRow(buttons)
            revision = self.revision
            if dialog.exec() != QDialog.DialogCode.Accepted or revision != self.revision:
                return
            move = chess.Move(source, target, promotion=choice.currentData())
        self.accept_move(move)

    def text_move(self):
        try:
            text = self.move_input.text().strip()
            try:
                move = self.game.board.parse_san(text)
            except ValueError:
                move = self.game.board.parse_uci(text)
            self.accept_move(move)
        except ValueError:
            self.message.setText("Move not recognised. Try e4, Nf3, O-O or e7e8q for promotion.")

    def takeback(self):
        if not self.game.board.move_stack:
            return
        self.cancel_engine()
        self.engine_error = ""
        self.game.takeback()
        self.refresh()
        self.persist()
        self.maybe_engine()

    def hint(self):
        if self.game.human_turn and not self.preview:
            self.start_engine(hint=True)

    def claim_draw(self):
        if not self.game.human_turn:
            return
        self.cancel_engine()
        try:
            self.game.claim_draw()
        except ValueError as error:
            self.message.setText(str(error))
            return
        self.refresh()
        self.persist()

    def resign(self):
        if self.game.finished:
            return
        if QMessageBox.question(self, "Resign game", "Resign this game?") != QMessageBox.StandardButton.Yes:
            return
        self.cancel_engine()
        self.game.resign()
        self.refresh()
        self.persist()

    def flip(self):
        self.board.flipped = not self.board.flipped
        self.refresh()
        self.persist()

    def toggle_guides(self, checked):
        self.board.guides = checked
        self.board.update()
        self.persist()

    def show_history(self, row):
        if row < 0:
            return
        if row == len(self.game.board.move_stack):
            self.return_live()
            return
        board = self.game.board.root()
        for move in self.game.board.move_stack[:row]:
            board.push(move)
        self.preview = True
        self.board.set_position(board, False)
        self.move_input.setEnabled(False)
        self.hint_button.setEnabled(False)
        self.live_button.show()
        self.message.setText("Viewing an earlier position. Return to the live board to play.")

    def return_live(self):
        self.refresh()

    def new_game(self):
        dialog = QDialog(self)
        dialog.setWindowTitle("New game")
        form = QFormLayout(dialog)
        mode, color, level = QComboBox(), QComboBox(), QComboBox()
        mode.addItem("Computer · Stockfish", "computer")
        mode.addItem("Friend · Shared board", "local")
        mode.setCurrentIndex(0 if self.game.mode == "computer" else 1)
        color.addItem("White", True)
        color.addItem("Black", False)
        color.setCurrentIndex(0 if self.game.human else 1)
        level.addItems(list(DIFFICULTIES))
        level.setCurrentText(self.game.difficulty)
        form.addRow("Play against", mode)
        form.addRow("Your side", color)
        form.addRow("Difficulty", level)
        note = QLabel(
            "Untimed play. Your current game will be archived.\nGentle reduces engine strength; it is not a rated beginner bot."
        )
        note.setWordWrap(True)
        form.addRow(note)

        def update():
            color.setEnabled(mode.currentData() == "computer")
            level.setEnabled(mode.currentData() == "computer")

        mode.currentIndexChanged.connect(update)
        update()
        buttons = QDialogButtonBox(
            QDialogButtonBox.StandardButton.Ok | QDialogButtonBox.StandardButton.Cancel
        )
        buttons.accepted.connect(dialog.accept)
        buttons.rejected.connect(dialog.reject)
        form.addRow(buttons)
        if dialog.exec() != QDialog.DialogCode.Accepted:
            return
        try:
            self.archive()
        except OSError as error:
            self.error("Could not archive current game", error)
            return
        self.cancel_engine()
        self.game = Game(mode=mode.currentData(), human=color.currentData(), difficulty=level.currentText())
        self.board.flipped = self.game.mode == "computer" and not self.game.human
        self.engine_error = ""
        self.refresh()
        self.persist()
        self.maybe_engine()

    def import_pgn(self):
        filename, _ = QFileDialog.getOpenFileName(
            self, "Import one PGN game", "", "Chess games (*.pgn);;All files (*)"
        )
        if not filename:
            return
        try:
            path = Path(filename)
            if path.stat().st_size > MAX_PGN_BYTES:
                raise ValueError("PGN is too large (maximum 1 MB).")
            imported = Game.from_pgn(path.read_text(encoding="utf-8-sig"))
            self.archive()
        except (OSError, ValueError) as error:
            self.error("Could not import PGN", error)
            return
        self.cancel_engine()
        self.game = imported
        self.engine_error = ""
        self.refresh()
        self.persist()
        self.message.setText("Imported main line for local play and review. Original file is unchanged.")

    def export_pgn(self):
        filename, _ = QFileDialog.getSaveFileName(
            self, "Export PGN", "omarchy-chess.pgn", "Chess games (*.pgn)"
        )
        if filename:
            try:
                atomic_write(Path(filename), self.game.pgn())
                self.message.setText("Game exported.")
            except OSError as error:
                self.error("Could not export PGN", error)

    def help(self):
        QMessageBox.information(
            self,
            "Keyboard controls",
            "Board: arrows to navigate, Enter or Space to select, Escape to clear.\nTab: move between controls.\nMove field: e4, Nf3, O-O, e2e4 or e7e8n.\n\nCtrl+N: new game\nCtrl+O: import PGN\nCtrl+S: export PGN\nCtrl+Z: take back\nCtrl+H: hint\nCtrl+F: flip board\nCtrl+L: live board\nCtrl+Q: quit",
        )

    def about(self):
        QMessageBox.about(
            self,
            "About Omarchy Chess",
            f"Omarchy Chess {__version__}\n\nAn independent community app for Omarchy.\nPowered by Stockfish, python-chess and Qt.\nGPL-3.0-or-later.\n\nPiece artwork: python-chess, adapted from Wikimedia chess pieces by Cburnett (GPL).\nNo accounts, analytics or network services.",
        )

    def closeEvent(self, event):
        self.closing = True
        self.theme_timer.stop()
        self.cancel_engine()
        for job in list(self.jobs):
            job.cancel()
        # Startup and search have bounded timeouts. Keep QThreads alive until finished.
        if any(job.isRunning() for job in self.jobs):
            self.hide()
            event.ignore()
            QTimer.singleShot(100, self.close)
            return
        self.persist()
        event.accept()


def main():
    parser = argparse.ArgumentParser(description="Native Omarchy chess, powered by Stockfish")
    parser.add_argument("--version", action="version", version=__version__)
    parser.add_argument("--screenshot", type=Path, help=argparse.SUPPRESS)
    args = parser.parse_args()
    app = QApplication(sys.argv[:1])
    app.setApplicationName("Omarchy Chess")
    app.setDesktopFileName("omarchy-chess")
    state = state_directory()
    state.mkdir(parents=True, exist_ok=True)
    lock = QLockFile(str(state / "session.lock"))
    lock.setStaleLockTime(0)
    if not lock.tryLock(100):
        QMessageBox.information(
            None, "Omarchy Chess", "Another Omarchy Chess window is already using this saved game."
        )
        return 1
    window = Window(state)
    window.show()
    if args.screenshot:
        QTimer.singleShot(500, lambda: (window.grab().save(str(args.screenshot)), window.close()))
    result = app.exec()
    lock.unlock()
    return result
