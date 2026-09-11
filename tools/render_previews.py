"""Capture actual native windows for documentation. Does not touch user saves."""
import os
os.environ.setdefault("QT_QPA_PLATFORM", "offscreen")
os.environ.setdefault("QT_QUICK_BACKEND", "software")

from pathlib import Path
import sys
from tempfile import TemporaryDirectory

from PySide6.QtCore import Qt, QUrl
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine
from PySide6.QtQuickControls2 import QQuickStyle
from PySide6.QtTest import QTest
import shiboken6

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from omarchy_solitaire.controller import Controller
from omarchy_solitaire.game import Game

app = QGuiApplication([])
QQuickStyle.setStyle("Basic")
output = ROOT / "docs/screenshots"
output.mkdir(parents=True, exist_ok=True)

with TemporaryDirectory() as directory:
    game = Controller(directory)
    game.clock.stop()
    game.theme.timer.stop()
    game.game = Game(2026)
    game.theme.candidates = lambda: []
    # A real, reproducible state reached by legal actions.
    for _ in range(10):
        hint = game.game.hint()
        if hint and hint[0] != 0:
            game.move(*hint)
        else:
            game.draw()
    engine = QQmlApplicationEngine()
    engine.rootContext().setContextProperty("game", game)
    engine.rootContext().setContextProperty("theme", game.theme)
    engine.load(QUrl.fromLocalFile(str(ROOT / "omarchy_solitaire/qml/Main.qml")))
    if not engine.rootObjects():
        raise SystemExit("QML did not load")
    window = engine.rootObjects()[0]

    def capture(name, width=1120, height=800):
        window.setWidth(width)
        window.setHeight(height)
        QTest.qWait(200)
        if not window.grabWindow().save(str(output / name)):
            raise RuntimeError("Screenshot failed")

    capture("table.png")
    capture("compact.png", 800, 600)
    QTest.keyClick(window, Qt.Key_Comma, Qt.ControlModifier)
    capture("deck-compact.png", 800, 600)
    capture("deck.png")
    QTest.keyClick(window, Qt.Key_Escape)
    QTest.keyClick(window, Qt.Key_F1)
    capture("help-compact.png", 800, 600)
    QTest.keyClick(window, Qt.Key_Escape)
    light = Path(directory) / "theme"
    light.mkdir()
    (light / "colors.toml").write_text('background = "#eeeae1"\nforeground = "#303d37"\naccent = "#40634b"\n')
    (light.parent / "theme.name").write_text("paper")
    game.theme.candidates = lambda: [light]
    game.theme.refresh()
    capture("light.png")
    shiboken6.delete(engine)
    shiboken6.delete(game)

print(f"Native screenshots saved in {output}")
