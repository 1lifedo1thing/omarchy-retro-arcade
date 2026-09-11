import os
os.environ.setdefault("QT_QPA_PLATFORM", "offscreen")
os.environ.setdefault("QT_QUICK_BACKEND", "software")

from pathlib import Path
from tempfile import TemporaryDirectory
import unittest

from PySide6.QtCore import QPoint, Qt, QUrl, qInstallMessageHandler
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine
from PySide6.QtQuick import QQuickItem
from PySide6.QtQuickControls2 import QQuickStyle
from PySide6.QtTest import QTest
import shiboken6

from omarchy_solitaire.controller import Controller
from omarchy_solitaire.game import Game
from omarchy_solitaire.storage import atomic_json, read_json
from omarchy_solitaire.theme import Theme, palette, contrast, validate_art
from test_game import arranged

APP = QGuiApplication.instance() or QGuiApplication([])
QQuickStyle.setStyle("Basic")
ROOT = Path(__file__).resolve().parents[1]


class IntegrationTests(unittest.TestCase):
    def setUp(self):
        self.temp = TemporaryDirectory()
        self.game = Controller(self.temp.name)
        self.game.clock.stop()
        self.game.theme.timer.stop()

    def tearDown(self):
        self.game.completer.stop()
        shiboken6.delete(self.game)
        self.temp.cleanup()

    def test_save_resume_undo_preferences_and_stats(self):
        self.game.draw()
        self.game.elapsed = 82
        self.game.preference("reducedMotion", True)
        self.game.save()
        other = Controller(self.temp.name)
        try:
            self.assertEqual(other.game.snapshot(), self.game.game.snapshot())
            self.assertEqual(other.elapsed, 82)
            self.assertTrue(other.prefs["reducedMotion"])
            other.undo(); other.draw()
            self.assertEqual(other.stats["1"]["played"], 1)
            other.newGame(1, True); other.draw()
            self.assertEqual(other.stats["1"]["played"], 1)
        finally:
            shiboken6.delete(other)

    def test_corrupt_save_preserved_and_atomic_write(self):
        Path(self.temp.name, "session.json").write_text('{"bad": true}')
        other = Controller(self.temp.name)
        try:
            self.assertEqual(len(list(Path(self.temp.name).glob("session-unreadable-*.json"))), 1)
            self.assertTrue(other.save())
            self.assertEqual(read_json(other.path)["game"]["version"], 1)
        finally:
            shiboken6.delete(other)

    def test_theme_replace_missing_invalid_and_light_contrast(self):
        directory = Path(self.temp.name, "theme")
        directory.mkdir()
        theme = self.game.theme
        theme.candidates = lambda: [directory]
        (directory / "colors.toml").write_text('background = "#ffffff"\nforeground = "#ffffff"\naccent = "#cccccc"')
        theme.refresh()
        self.assertGreaterEqual(contrast(theme.colors["text"], theme.colors["table"]), 4.5)
        before = theme.colors
        (directory / "colors.toml").write_text('invalid = [')
        theme.refresh()
        self.assertEqual(theme.colors, before)
        (directory / "colors.toml").unlink()
        directory.rmdir()
        directory.mkdir()
        (directory / "colors.toml").write_text('background = "#112233"\nforeground = "#eeeeee"\naccent = "#99aaff"')
        theme.refresh()
        self.assertEqual(theme.colors["table"], "#112233")
        theme.setFollowing(False)
        (directory / "colors.toml").write_text('background = "#223344"')
        theme.refresh()
        self.assertEqual(theme.colors["table"], "#112233")
        theme.setFollowing(True)
        self.assertEqual(theme.colors["table"], "#223344")

    def test_artwork_import_and_reject_active_content(self):
        path = Path(self.temp.name, "art.svg")
        path.write_text('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 700"><rect width="500" height="700" fill="#123456"/></svg>')
        self.game.importBack(QUrl.fromLocalFile(str(path)))
        self.assertEqual(self.game.prefs["pattern"], 3)
        self.assertTrue(Path(self.temp.name, "custom-back.svg").exists())
        for body in ('<script>alert(1)</script>', '<image href="https://example.com/a.png"/>', '<rect style="fill:red"/>'):
            path.write_text('<svg xmlns="http://www.w3.org/2000/svg">' + body + '</svg>')
            with self.assertRaises(ValueError):
                validate_art(path)

    def test_locked_palette_survives_restart(self):
        self.game.theme._colors = palette({"background": "#102030", "foreground": "#ffffff", "accent": "#99eeff"})
        self.game.theme._name = "Locked test"
        self.game.preference("followTheme", False)
        other = Controller(self.temp.name)
        try:
            self.assertFalse(other.theme.following)
            self.assertEqual(other.theme.colors["table"], "#102030")
            self.assertEqual(other.theme.name, "Locked test")
        finally:
            shiboken6.delete(other)

    def test_contrast_on_light_dark_and_mid_grey(self):
        for background in ("#ffffff", "#000000", "#777777", "#808080", "#aabbcc"):
            colors = palette({"background": background, "foreground": background, "accent": background})
            for foreground in ("text", "muted", "accent"):
                for surface in ("table", "surface"):
                    self.assertGreaterEqual(contrast(colors[foreground], colors[surface]), 4.5)

    def test_timer_pause_and_single_win_count(self):
        self.game.draw()
        self.game.setActive(False); self.game.tick()
        self.assertEqual(self.game.elapsed, 0)
        self.game.setActive(True); self.game.setPaused(True); self.game.tick()
        self.assertEqual(self.game.elapsed, 0)
        self.game.setPaused(False); self.game.tick()
        self.assertEqual(self.game.elapsed, 1)
        self.game.game = arranged({6: [12]}, foundations={2: list(range(12)), 3: list(range(13, 26)),
                                      4: list(range(26, 39)), 5: list(range(39, 52))})
        self.game.move(6, 0, 2)
        self.assertEqual(self.game.stats["1"]["won"], 1)
        self.game.undo(); self.game.move(6, 0, 2)
        self.assertEqual(self.game.stats["1"]["won"], 1)

    def test_native_ui_click_keyboard_drag_and_render(self):
        messages = []
        previous = qInstallMessageHandler(lambda kind, ctx, msg: messages.append(msg))
        engine = QQmlApplicationEngine()
        try:
            self.game.game = arranged({6: [0], 7: [12, 24], 8: [6]})
            engine.rootContext().setContextProperty("game", self.game)
            engine.rootContext().setContextProperty("theme", self.game.theme)
            engine.load(QUrl.fromLocalFile(str(ROOT / "omarchy_solitaire/qml/Main.qml")))
            self.assertTrue(engine.rootObjects(), messages)
            window = engine.rootObjects()[0]
            window.setWidth(1120); window.setHeight(800)
            QTest.qWait(150)
            board = window.findChild(QQuickItem, "board")
            self.assertIsNotNone(board)
            cw = board.property("cardWidth")
            gap = board.property("gap")
            left = board.property("insetX")
            top = board.property("topY")
            table_y = board.property("tableY")
            origin = board.mapToScene(QPoint(0, 0))
            def point(col, y, dx=.5):
                return QPoint(round(origin.x() + left + col * (cw + gap) + cw * dx), round(origin.y() + y))
            # Click ace, click first foundation.
            QTest.mouseClick(window, Qt.LeftButton, Qt.NoModifier, point(0, table_y + 50))
            self.assertEqual(self.game.selection, (6, 0))
            QTest.mouseClick(window, Qt.LeftButton, Qt.NoModifier, point(3, top + 50))
            self.assertEqual(len(self.game.game.piles[2]), 1)
            # A foundation double-click succeeds through actual pointer events.
            self.game.undo()
            QTest.mouseDClick(window, Qt.LeftButton, Qt.NoModifier, point(0, table_y + 50))
            self.assertEqual(len(self.game.game.piles[2]), 1)
            # Keyboard space draws; Ctrl+Z restores.
            board.forceActiveFocus()
            QTest.keyClick(window, Qt.Key_Space)
            self.assertEqual(len(self.game.game.piles[1]), 1)
            QTest.keyClick(window, Qt.Key_Z, Qt.ControlModifier)
            self.assertEqual(len(self.game.game.piles[1]), 0)
            # Drag a king plus queen to the now-empty first column.
            start = point(1, table_y + 12)
            end = point(0, table_y + 70)
            QTest.mousePress(window, Qt.LeftButton, Qt.NoModifier, start)
            QTest.mouseMove(window, QPoint((start.x()+end.x())//2, (start.y()+end.y())//2), 40)
            QTest.mouseMove(window, end, 40)
            QTest.mouseRelease(window, Qt.LeftButton, Qt.NoModifier, end)
            self.assertEqual([c.id for c in self.game.game.piles[6]], [12, 24])
            # Tab navigation includes the board; dialogs suppress gameplay shortcuts.
            self.assertTrue(board.property("activeFocusOnTab"))
            QTest.keyClick(window, Qt.Key_Comma, Qt.ControlModifier)
            snapshot = self.game.game.snapshot()
            QTest.keyClick(window, Qt.Key_Z, Qt.ControlModifier)
            self.assertEqual(self.game.game.snapshot(), snapshot)
            QTest.keyClick(window, Qt.Key_Escape)
            QTest.qWait(100)
            self.assertFalse(window.grabWindow().isNull())
            window.setWidth(800); window.setHeight(600)
            QTest.qWait(100)
            self.assertFalse(window.grabWindow().isNull())
            self.assertFalse(messages, messages)
        finally:
            shiboken6.delete(engine)
            qInstallMessageHandler(previous)


if __name__ == "__main__":
    unittest.main()
