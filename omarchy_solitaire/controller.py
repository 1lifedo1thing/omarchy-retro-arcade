"""Qt view model; all rules remain in game.py."""
from pathlib import Path
import shutil
import uuid

from PySide6.QtCore import QObject, Property, Signal, Slot, QTimer, QUrl

from .game import Game, RANKS, SYMBOLS
from .storage import atomic_json, read_json, xdg_dir
from .theme import Theme, validate_art, palette


class Controller(QObject):
    changed = Signal()
    boardChanged = Signal()
    messageChanged = Signal()
    preferencesChanged = Signal()
    victory = Signal()

    def __init__(self, state_dir=None):
        super().__init__()
        self.directory = Path(state_dir) if state_dir else xdg_dir("XDG_STATE_HOME", ".local/state") / "omarchy-solitaire"
        self.path = self.directory / "session.json"
        self.theme = Theme()
        self.game = Game()
        self.elapsed = 0
        self.session = str(uuid.uuid4())
        self.counted = False
        self.started = False
        self.stats = {"1": {"played": 0, "won": 0, "best": 0}, "3": {"played": 0, "won": 0, "best": 0}}
        self.prefs = {"pattern": 0, "finish": "matte", "reducedMotion": False, "followTheme": True, "customArt": ""}
        self.selection = (-1, -1)
        self._message = "A little time for a game. Draw a card to begin."
        self._save_error = ""
        self.active = True
        self.paused = False
        self.completing = False
        self.hint_target = -1
        self._load()
        self.theme.setFollowing(self.prefs["followTheme"])
        self.clock = QTimer(self)
        self.clock.setInterval(1000)
        self.clock.timeout.connect(self.tick)
        self.clock.start()
        self.completer = QTimer(self)
        self.completer.setInterval(100)
        self.completer.timeout.connect(self._complete_step)

    def _load(self):
        if not self.path.exists():
            return
        try:
            data = read_json(self.path)
            game = Game.deserialize(data["game"])
            elapsed = data.get("elapsed", 0)
            if type(elapsed) is not int or not 0 <= elapsed < 2**31:
                raise ValueError("Invalid time")
            self.game, self.elapsed = game, elapsed
            self.session = str(data.get("session", self.session))[:80]
            self.counted = bool(data.get("counted", False))
            self.started = bool(data.get("started", game.moves > 0))
            for mode in ("1", "3"):
                for key in ("played", "won", "best"):
                    value = data.get("stats", {}).get(mode, {}).get(key, 0)
                    if type(value) is int and 0 <= value < 2**31:
                        self.stats[mode][key] = value
            prefs = data.get("preferences", {})
            if prefs.get("finish") in ("matte", "holographic"):
                self.prefs["finish"] = prefs["finish"]
            for key in ("reducedMotion", "followTheme"):
                if type(prefs.get(key)) is bool:
                    self.prefs[key] = prefs[key]
            snapshot = data.get("themeSnapshot", {})
            if not self.prefs["followTheme"] and isinstance(snapshot, dict):
                self.theme._colors = palette({"background": snapshot.get("table"),
                                              "foreground": snapshot.get("text"), "accent": snapshot.get("accent")})
                self.theme._name = str(data.get("themeName", "Locked theme"))[:80]
                locked = data.get("lockedArt", "")
                if locked in ("locked-theme.svg", "locked-theme.png") and (self.directory / locked).is_file():
                    validate_art(self.directory / locked)
                    self.theme._art = QUrl.fromLocalFile(str(self.directory / locked)).toString()
            if type(prefs.get("pattern")) is int and prefs["pattern"] in (0, 1, 2, 3):
                self.prefs["pattern"] = prefs["pattern"]
            # Only load the managed local copy. Saves cannot name arbitrary files.
            art = prefs.get("customArt", "")
            if art in ("custom-back.svg", "custom-back.png") and (self.directory / art).exists():
                validate_art(self.directory / art)
                self.prefs["customArt"] = art
            self._message = "Welcome back. Your table is just as you left it."
        except (OSError, ValueError, KeyError, TypeError, AttributeError):
            # Preserve corrupt data for recovery before any later autosave.
            backup = self.path.with_name(f"session-unreadable-{uuid.uuid4().hex[:8]}.json")
            try:
                shutil.copy2(self.path, backup)
                self._message = "The saved game could not be read. A recovery copy was kept; a new deal is ready."
            except OSError:
                self._save_error = "The old save could not be backed up. Autosave is disabled for this session."
            self.game = Game()
            self.elapsed = 0

    @Slot(result=bool)
    def save(self):
        if self._save_error.startswith("The old save"):
            return False
        try:
            atomic_json(self.path, {"game": self.game.serialize(), "elapsed": self.elapsed,
                "session": self.session, "counted": self.counted, "started": self.started, "stats": self.stats,
                "preferences": self.prefs, "themeSnapshot": self.theme.colors, "themeName": self.theme.name,
                "lockedArt": Path(QUrl(self.theme.artwork).toLocalFile()).name if not self.theme.following else ""})
            if self._save_error:
                self._save_error = ""
                self.messageChanged.emit()
            return True
        except (OSError, ValueError):
            self._save_error = "Couldn't save this game. Check available disk space and folder permissions."
            self.messageChanged.emit()
            return False

    def say(self, message):
        self._message = message
        self.messageChanged.emit()

    def _changed(self):
        self.selection = (-1, -1)
        self.hint_target = -1
        if self.game.won:
            if not self.counted:
                record = self.stats[str(self.game.draw_count)]
                record["won"] += 1
                record["best"] = self.elapsed if not record["best"] else min(record["best"], self.elapsed)
                self.counted = True
            self.say("All home. Beautifully played.")
            self.victory.emit()
        self.changed.emit()
        self.boardChanged.emit()
        self.save()

    def _started(self):
        if not self.started:
            self.stats[str(self.game.draw_count)]["played"] += 1
            self.started = True

    @Slot()
    def draw(self):
        if self.completing or self.game.won:
            return
        if self.game.piles[0] or self.game.piles[1]:
            self._started()
            recycling = not self.game.piles[0]
            self.game.draw()
            self.say("Stock recycled. Keep looking." if recycling else "Build down in alternating colours; aces go above.")
            self._changed()

    @Slot(int, int, int, result=bool)
    def move(self, source, index, target):
        if self.completing or not self.game.legal(source, index, target):
            self.say("That move doesn't fit. Build down in alternating colours; only kings fill empty columns.")
            return False
        self._started()
        self.game.move(source, index, target)
        self.say("Nicely placed.")
        self._changed()
        return True

    @Slot(int, int)
    def choose(self, pile, index):
        if self.completing:
            return
        if pile == 0:
            self.draw()
            return
        if self.selection[0] >= 0 and self.game.legal(*self.selection, pile):
            self.move(*self.selection, pile)
            return
        if (pile, index) == self.selection:
            self.clearSelection()
        elif self.game.movable(pile, index):
            self.selection = (pile, index)
            self.hint_target = -1
            self.say(f"{self.game.piles[pile][index].name} selected. Choose a destination, or press F for a foundation.")
            self.changed.emit()
        else:
            self.clearSelection()

    @Slot(int, int)
    def toFoundation(self, pile, index):
        if self.completing:
            return
        for target in range(2, 6):
            if self.game.legal(pile, index, target):
                self.move(pile, index, target)
                return
        self.say("That card isn't ready for a foundation yet.")

    @Slot()
    def clearSelection(self):
        self.selection = (-1, -1)
        self.hint_target = -1
        self.changed.emit()

    @Slot()
    def undo(self):
        self.stopCompletion()
        if self.game.undo():
            self.say("One move back.")
            self._changed()

    @Slot(int, bool)
    def newGame(self, draw, restart):
        if draw not in (1, 3):
            return
        self.stopCompletion()
        self.game = Game(self.game.seed if restart else None, draw)
        self.elapsed = 0
        if not restart:
            self.counted = False
            self.started = False
            self.session = str(uuid.uuid4())
        self.say("Same deal, fresh start." if restart else "A fresh deck. Make yourself at home.")
        self._changed()

    @Slot()
    def hint(self):
        if self.completing or self.game.won:
            return
        result = self.game.hint()
        if result is None:
            self.say("No useful move found. Try undo, restart this deal, or deal again. Hints are not a solver.")
        elif result[0] == 0:
            self.hint_target = 0
            self.selection = (-1, -1)
            self.say("Draw from the stock." if self.game.piles[0] else "Recycle the waste to look through the stock again.")
        else:
            self.selection = result[:2]
            self.hint_target = result[2]
            destination = "a foundation" if result[2] < 6 else f"column {result[2] - 5}"
            self.say(f"Try {self.game.piles[result[0]][result[1]].name} to {destination}. This is a legal suggestion, not a guarantee.")
        self.changed.emit()

    @Slot()
    def complete(self):
        if self.game.can_complete:
            self.completing = True
            self.completer.start()
            self.changed.emit()

    @Slot()
    def stopCompletion(self):
        self.completing = False
        self.completer.stop()
        self.changed.emit()

    def _complete_step(self):
        move = self.game.completion_move()
        if not move:
            self.stopCompletion()
            return
        self.game.move(*move)
        self._changed()

    @Slot(bool)
    def setActive(self, active):
        self.active = active

    @Slot(bool)
    def setPaused(self, paused):
        self.paused = paused

    def tick(self):
        if self.active and not self.paused and self.game.moves and not self.game.won:
            self.elapsed += 1
            self.changed.emit()
            if self.elapsed % 10 == 0:
                self.save()

    @Slot(str, "QVariant")
    def preference(self, name, value):
        if name == "pattern" and type(value) is int and value in (0, 1, 2, 3):
            self.prefs[name] = value
        elif name == "finish" and value in ("matte", "holographic"):
            self.prefs[name] = value
        elif name in ("reducedMotion", "followTheme") and type(value) is bool:
            self.prefs[name] = value
            if name == "followTheme":
                if not value and self.theme.artwork:
                    try:
                        source = validate_art(QUrl(self.theme.artwork).toLocalFile())
                        self.directory.mkdir(parents=True, exist_ok=True)
                        dest = self.directory / ("locked-theme" + source.suffix.lower())
                        if source.resolve() != dest.resolve():
                            shutil.copyfile(source, dest)
                        self.theme._art = QUrl.fromLocalFile(str(dest)).toString()
                    except (OSError, ValueError):
                        self.theme._art = ""
                self.theme.setFollowing(value)
        else:
            return
        self.preferencesChanged.emit()
        self.save()

    @Slot(QUrl)
    def importBack(self, url):
        try:
            if not url.isLocalFile():
                raise ValueError("Choose an image on this computer.")
            path = validate_art(url.toLocalFile())
            self.directory.mkdir(parents=True, exist_ok=True)
            name = "custom-back" + path.suffix.lower()
            destination = self.directory / name
            if path.resolve() != destination.resolve():
                temporary = self.directory / (name + ".tmp")
                shutil.copyfile(path, temporary)
                temporary.replace(destination)
            self.prefs["customArt"] = name
            self.prefs["pattern"] = 3
            self.preferencesChanged.emit()
            self.save()
            self.say("Your custom card back is ready.")
        except (OSError, ValueError) as exc:
            self.say(str(exc))

    @Property("QVariantList", notify=boardChanged)
    def cards(self):
        output = []
        for pile, cards in enumerate(self.game.piles):
            offset = 0.0
            for index, card in enumerate(cards):
                shown = (pile >= 6 or index == len(cards) - 1 or
                         (pile == 1 and index >= len(cards) - self.game.draw_count))
                output.append({"id": card.id, "pile": pile, "row": index, "rank": RANKS[card.rank],
                    "value": card.rank, "suit": SYMBOLS[card.suit], "red": card.red, "faceUp": card.up,
                    "shown": shown, "offset": offset,
                    "fan": max(0, index - max(0, len(cards) - self.game.draw_count)) if pile == 1 else 0,
                    "name": card.name if card.up else "Face-down card", "movable": self.game.movable(pile, index)})
                offset += .27 if card.up else .14
        return output

    @Property("QVariantList", notify=boardChanged)
    def counts(self):
        return [len(p) for p in self.game.piles]

    @Property("QVariantList", notify=changed)
    def legalTargets(self):
        return [t for t in range(2, 13) if self.game.legal(*self.selection, t)] if self.selection[0] >= 0 else []

    @Slot(int, int, int, result=bool)
    def canMove(self, source, row, target):
        return self.game.legal(source, row, target)

    @Property("QVariantMap", notify=changed)
    def status(self):
        return {"moves": self.game.moves, "score": self.game.score, "draw": self.game.draw_count,
                "time": f"{self.elapsed // 60:02d}:{self.elapsed % 60:02d}", "seed": str(self.game.seed),
                "won": self.game.won, "canUndo": bool(self.game.history), "canComplete": self.game.can_complete,
                "completing": self.completing, "selectedPile": self.selection[0], "selectedRow": self.selection[1],
                "hintTarget": self.hint_target, "foundationCount": sum(len(p) for p in self.game.piles[2:6]),
                "statistics": self.stats}

    @Property(str, notify=messageChanged)
    def message(self):
        return self._save_error or self._message

    @Property("QVariantMap", notify=preferencesChanged)
    def preferences(self):
        return self.prefs

    @Property(str, notify=preferencesChanged)
    def customArt(self):
        name = self.prefs["customArt"]
        return QUrl.fromLocalFile(str(self.directory / name)).toString() if name else ""
