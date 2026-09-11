"""Read-only Omarchy palette adapter and data-only custom card artwork."""
from pathlib import Path
import re
import tomllib
import xml.etree.ElementTree as ET

from PySide6.QtCore import QObject, Property, Signal, Slot, QTimer, QUrl
from PySide6.QtGui import QColor, QImageReader

from .storage import xdg_dir

DEFAULT = {"background": "#182622", "foreground": "#e6eadb", "accent": "#b7ce91"}
HEX = re.compile(r"^#[0-9a-fA-F]{6}$")


def luminance(color):
    c = QColor(color)
    values = [v / 12.92 if v <= .04045 else ((v + .055) / 1.055)**2.4
              for v in (c.redF(), c.greenF(), c.blueF())]
    return .2126 * values[0] + .7152 * values[1] + .0722 * values[2]


def contrast(a, b):
    x, y = sorted((luminance(a), luminance(b)))
    return (y + .05) / (x + .05)


def readable(foreground, background):
    if contrast(foreground, background) >= 4.5:
        return foreground
    return "#141814" if contrast("#141814", background) > contrast("#ffffff", background) else "#ffffff"


def blend(a, b, amount):
    x, y = QColor(a), QColor(b)
    return QColor.fromRgbF(*[u * (1 - amount) + v * amount for u, v in
                            zip((x.redF(), x.greenF(), x.blueF()), (y.redF(), y.greenF(), y.blueF()))]).name()


def palette(raw):
    colors = {key: raw.get(key, value) if isinstance(raw.get(key, value), str) and
              HEX.fullmatch(raw.get(key, value)) else value for key, value in DEFAULT.items()}
    bg, fg, accent = colors["background"], colors["foreground"], colors["accent"]
    fg = readable(fg, bg)
    accent = readable(accent, bg)
    surface = blend(bg, fg, .06)
    return {"table": bg, "surface": surface, "text": fg,
            "muted": readable(blend(bg, fg, .62), bg), "accent": accent,
            "onAccent": readable(bg, accent), "line": blend(bg, fg, .22),
            "back": blend(bg, accent, .16), "backPattern": blend(bg, accent, .37),
            "face": "#faf7ef", "ink": "#202826", "red": "#ab263d"}


def validate_art(path):
    path = Path(path)
    if not path.is_file() or path.stat().st_size > 2_000_000:
        raise ValueError("Choose an SVG or PNG smaller than 2 MB.")
    if path.suffix.lower() == ".svg":
        raw = path.read_bytes()
        if b"<!DOCTYPE" in raw.upper() or b"<!ENTITY" in raw.upper():
            raise ValueError("SVG entities are not supported.")
        try:
            root = ET.fromstring(raw)
        except ET.ParseError as exc:
            raise ValueError("That SVG could not be read.") from exc
        allowed = {"svg", "g", "path", "rect", "circle", "ellipse", "line", "polyline", "polygon",
                   "defs", "linearGradient", "radialGradient", "stop", "clipPath", "title", "desc"}
        if root.tag.split("}")[-1] != "svg":
            raise ValueError("Choose a valid SVG image.")
        for node in root.iter():
            if node.tag.split("}")[-1] not in allowed:
                raise ValueError("Use a static SVG made from paths and shapes; outline text first.")
            for key, value in node.attrib.items():
                name = key.split("}")[-1].lower()
                if name.startswith("on") or name in ("href", "src", "style"):
                    raise ValueError("SVG scripts, linked images and styles are not supported.")
                if "url(" in value.lower() and not re.fullmatch(r"url\(#[\w-]+\)", value):
                    raise ValueError("SVG references must stay inside the artwork.")
    elif path.suffix.lower() == ".png":
        reader = QImageReader(str(path))
        size = reader.size()
        if not reader.canRead() or size.width() < 1 or size.height() < 1 or max(size.width(), size.height()) > 4096:
            raise ValueError("Choose a valid PNG no larger than 4096 pixels per side.")
    else:
        raise ValueError("Choose an SVG or PNG card back.")
    return path


class Theme(QObject):
    changed = Signal()

    def __init__(self):
        super().__init__()
        self._colors = palette({})
        self._name = "House green"
        self._follow = True
        self._signature = None
        self._art = ""
        self._error = ""
        self.timer = QTimer(self)
        self.timer.setInterval(1500)
        self.timer.timeout.connect(self.refresh)
        self.timer.start()
        self.refresh()

    def candidates(self):
        # Current Omarchy, fixed default state path, and older config path.
        return list(dict.fromkeys([
            xdg_dir("XDG_STATE_HOME", ".local/state") / "omarchy/current/theme",
            Path.home() / ".local/state/omarchy/current/theme",
            xdg_dir("XDG_CONFIG_HOME", ".config") / "omarchy/current/theme",
        ]))

    @Slot()
    def refresh(self):
        if not self._follow:
            return
        directory = next((p for p in self.candidates() if (p / "colors.toml").is_file()), None)
        if directory is None:
            if self._signature is not None:
                self._signature = None
                self._colors, self._name, self._art = palette({}), "House green", ""
                self.changed.emit()
            return
        paths = [directory / "colors.toml", directory.parent / "theme.name",
                 directory / "solitaire/back.svg", directory / "solitaire/back.png"]
        try:
            signature = tuple((str(p), p.stat().st_ino, p.stat().st_mtime_ns, p.stat().st_size)
                              if p.exists() else (str(p),) for p in paths)
            if signature == self._signature:
                return
            with paths[0].open("rb") as stream:
                raw = stream.read(65537)
            if len(raw) > 65536:
                raise ValueError("Palette is too large")
            colors = palette(tomllib.loads(raw.decode("utf-8")))
            name = paths[1].read_text()[:80].strip().replace("-", " ").title() if paths[1].is_file() else "Omarchy theme"
            art = next((p for p in paths[2:] if p.is_file()), None)
            self._error = ""
            self._art = ""
            if art:
                try:
                    validate_art(art)
                    self._art = QUrl.fromLocalFile(str(art)).toString() + f"?v={art.stat().st_mtime_ns}"
                except ValueError as exc:
                    self._error = str(exc)
            self._colors, self._name, self._signature = colors, name, signature
            self.changed.emit()
        except (OSError, ValueError, UnicodeError):
            # Keep last valid palette during a theme directory replacement or invalid write.
            pass

    @Property("QVariantMap", notify=changed)
    def colors(self):
        return self._colors

    @Property(str, notify=changed)
    def name(self):
        return self._name

    @Property(str, notify=changed)
    def artwork(self):
        return self._art

    @Property(str, notify=changed)
    def artworkError(self):
        return self._error

    @Property(bool, notify=changed)
    def following(self):
        return self._follow

    @Slot(bool)
    def setFollowing(self, value):
        self._follow = value
        if value:
            self._signature = None
            self.refresh()
        self.changed.emit()

