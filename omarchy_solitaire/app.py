"""Native Qt entry point. Run with `python -m omarchy_solitaire`."""
import argparse
import os
from pathlib import Path
import sys

from PySide6.QtCore import QLockFile, QTimer, QUrl
from PySide6.QtGui import QGuiApplication, QIcon
from PySide6.QtQml import QQmlApplicationEngine
from PySide6.QtQuickControls2 import QQuickStyle

from .controller import Controller
from .storage import xdg_dir


def main():
    parser = argparse.ArgumentParser(description="Omarchy Solitaire")
    parser.add_argument("--version", action="version", version="Omarchy Solitaire 0.1.0")
    parser.add_argument("--state-dir", type=Path, help="Use an isolated directory for saves and preferences")
    parser.add_argument("--screenshot", type=Path, help="Capture the native window, then exit")
    parser.add_argument("--width", type=int, default=1120)
    parser.add_argument("--height", type=int, default=800)
    args = parser.parse_args()
    app = QGuiApplication(sys.argv[:1])
    app.setApplicationName("Omarchy Solitaire")
    app.setApplicationVersion("0.1.0")
    app.setOrganizationName("omarchy-solitaire")
    app.setDesktopFileName("io.github.tcballard.omarchy-solitaire")
    root = Path(__file__).parent
    app.setWindowIcon(QIcon(str(root / "assets/solitaire.svg")))
    QQuickStyle.setStyle("Basic")
    state_dir = args.state_dir or xdg_dir("XDG_STATE_HOME", ".local/state") / "omarchy-solitaire"
    try:
        state_dir.mkdir(parents=True, exist_ok=True)
    except OSError as exc:
        print(f"Cannot create the save directory: {exc}", file=sys.stderr)
        return 1
    lock = QLockFile(str(state_dir / "session.lock"))
    lock.setStaleLockTime(0)
    if not lock.tryLock(100):
        print("Omarchy Solitaire is already using this save directory.", file=sys.stderr)
        return 1
    game = Controller(state_dir)
    if os.environ.get("SOLITAIRE_REDUCED_MOTION") == "1":
        game.preference("reducedMotion", True)
    engine = QQmlApplicationEngine()
    engine.rootContext().setContextProperty("game", game)
    engine.rootContext().setContextProperty("theme", game.theme)
    engine.load(QUrl.fromLocalFile(str(root / "qml/Main.qml")))
    if not engine.rootObjects():
        return 1
    window = engine.rootObjects()[0]
    window.setWidth(max(800, args.width))
    window.setHeight(max(600, args.height))
    app.aboutToQuit.connect(game.save)
    if args.screenshot:
        def capture():
            args.screenshot.parent.mkdir(parents=True, exist_ok=True)
            ok = window.grabWindow().save(str(args.screenshot))
            app.exit(0 if ok else 1)
        QTimer.singleShot(1200, capture)
    result = app.exec()
    # Destroy QML objects while their Python context properties still exist.
    engine.setParent(None)
    del engine
    lock.unlock()
    return result

