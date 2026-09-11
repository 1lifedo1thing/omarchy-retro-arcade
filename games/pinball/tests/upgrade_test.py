"""Exercise the real application's restore and save path with a 0.2.0 game."""
import os
from pathlib import Path
import shutil
import subprocess
import sys

root = Path(sys.argv[2]).resolve()
data = root / "omarchy-spacecadet"
fixture = Path(__file__).parent / "fixtures/native-game-v0.2.0"
if sys.argv[1] == "seed":
    data.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(fixture, data / "native-game")
    (data / "native-preferences").write_text("2 999999 1 0\n")
else:
    before = (data / "native-game").read_text()
    assert before == fixture.read_text(), "Package update changed saved game"
    assert (data / "native-preferences").read_text() == "2 999999 1 0\n"
    env = dict(os.environ, XDG_DATA_HOME=str(root), SDL_VIDEODRIVER="dummy",
               SDL_AUDIODRIVER="dummy", SPACECADET_VERIFY_RESTORE="1")
    result = subprocess.run([sys.argv[3]], env=env, capture_output=True, text=True, timeout=15, check=True)
    assert "restored=1" in result.stdout, result.stdout
    assert (data / "native-game").read_text() == before, "Restored paused game changed"
    assert (data / "native-preferences").read_text() == "2 999999 1 0\n"
    print("0.2.0 game and preferences preserved; native app restored and saved successfully")
