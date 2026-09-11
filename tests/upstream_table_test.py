"""Run the authored data through the real upstream executable without a DAT."""
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

with tempfile.TemporaryDirectory() as tmp:
    env = dict(os.environ, XDG_DATA_HOME=tmp, XDG_CONFIG_HOME=tmp,
               SDL_VIDEODRIVER="dummy", SDL_AUDIODRIVER="dummy",
               OMARCHY_TEST_TICKS="21600")
    result = subprocess.run([str(Path(sys.argv[1]).resolve()), "--omarchy-table", "-sw"],
                            env=env, capture_output=True, text=True, timeout=90)
    assert result.returncode == 0, result.stdout[-4000:] + result.stderr[-4000:]
    match = re.search(r"UPSTREAM_TABLE ticks=(\d+) score=(\d+) balls=(\d+)", result.stdout)
    assert match, result.stdout[-4000:]
    ticks, score, balls = map(int, match.groups())
    assert ticks == 21600
    assert score >= 100, "No upstream bumper collision registered"
    assert balls < 3, "No upstream drain registered"
    assert not list(Path(tmp).rglob("*.DAT")), "Test unexpectedly used external resources"
    print(f"Upstream engine: 180 simulated seconds, score {score}, balls {balls}; no external DAT")
