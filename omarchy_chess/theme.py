"""Read colours as data, never execute theme files."""
import os
import re
import tomllib
from pathlib import Path

DEFAULT = {"background": "#171c1a", "foreground": "#e4e8df", "accent": "#b3cb92", "color1": "#ed8276"}


def theme_path() -> Path:
    return Path(os.environ.get("XDG_CONFIG_HOME", str(Path.home() / ".config"))) / "omarchy/current/theme/colors.toml"


def read_theme(path: Path | None = None) -> dict[str, str]:
    result = DEFAULT.copy()
    try:
        path = path or theme_path()
        if path.stat().st_size > 65536:
            return result
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        for key in result:
            value = data.get(key)
            if isinstance(value, str) and re.fullmatch(r"#[0-9a-fA-F]{6}", value):
                result[key] = value
    except (OSError, ValueError):
        pass
    return result
