import os
from pathlib import Path
import subprocess
import sys
import tempfile

binary = sys.argv[1]
with tempfile.TemporaryDirectory() as tmp:
    root = Path(tmp)
    env = dict(os.environ, HOME=str(root), XDG_CONFIG_HOME=str(root / "config"), XDG_STATE_HOME=str(root / "state"))
    env.pop("SPACECADET_THEME_FILE", None)
    suffix = Path("omarchy/current/theme/colors.toml")
    def selected():
        return subprocess.check_output([binary, "--path"], env=env, text=True)
    legacy = root / "config" / suffix
    legacy.parent.mkdir(parents=True)
    legacy.write_text('accent = "#123456"')
    assert selected() == str(legacy)
    current = root / ".local/state" / suffix
    current.parent.mkdir(parents=True)
    current.write_text('accent = "#654321"')
    assert selected() == str(current)
    xdg = root / "state" / suffix
    xdg.parent.mkdir(parents=True)
    xdg.write_text('accent = "#abcdef"')
    assert selected() == str(xdg)
    # Replacing a theme directory must not leave a stale file handle.
    moved = xdg.parent.with_name("old-theme")
    xdg.parent.rename(moved)
    xdg.parent.mkdir()
    xdg.write_text('accent = "#fedcba"')
    assert selected() == str(xdg)
    env["SPACECADET_THEME_FILE"] = str(root / "explicit.toml")
    assert selected() == env["SPACECADET_THEME_FILE"]
print("Theme path checks passed")
