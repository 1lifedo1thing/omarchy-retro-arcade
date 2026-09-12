"""Real XTest gameplay checks. Run under Xvfb; this is not human playtesting.
Usage: python3 scripts/native-blast.py BINARY [SCREENSHOT_DIRECTORY]
Requires Pillow only for capture/assertions, never at runtime.
"""
import json
from PIL import ImageGrab

# Keep exactly the same X11/input plumbing as the collection acceptance harness.
from pathlib import Path
exec(compile((Path(__file__).parent / 'native-check.py').read_text().split(
    'with tempfile.TemporaryDirectory')[0], 'arcade-input-helpers', 'exec'))

out = Path(sys.argv[2]) if len(sys.argv) > 2 else None
if out:
    out.mkdir(parents=True, exist_ok=True)


def launch(env):
    app = subprocess.Popen([binary, '--game', 'blast'], env=env)
    for _ in range(100):
        found = windows()
        if found:
            break
        assert app.poll() is None
        time.sleep(.1)
    assert len(found) == 1, found
    time.sleep(.8)
    x.XSetInputFocus(display, found[0], 1, 0)
    x.XFlush(display)
    time.sleep(.3)
    return app, found[0]


def capture():
    return ImageGrab.grab(xdisplay=os.environ['DISPLAY']).crop((0, 0, 1280, 900))


def assert_frozen():
    time.sleep(.25)
    before = capture().tobytes()
    time.sleep(.7)
    assert before == capture().tobytes(), 'Paused scene advanced'


def title(window):
    ptr = C.c_void_p()
    x.XFetchName(display, window, C.byref(ptr))
    value = C.string_at(ptr).decode() if ptr.value else ''
    if ptr.value:
        x.XFree(ptr)
    return value


with tempfile.TemporaryDirectory(prefix='blast-native-') as tmp:
    env = dict(os.environ, XDG_STATE_HOME=tmp+'/state', XDG_CONFIG_HOME=tmp+'/config', XDG_DATA_HOME=tmp+'/data')
    app, window = launch(env)
    try:
        key(0xff0d)  # Start solo using real setup UI.
        key(ord('d'), hold=.42)
        key(0x20, hold=.02)
        key(ord('a'), hold=.38)
        if out:
            capture().save(out/'game.png')
        key(0xff1b)
        assert_frozen()
        key(0xff1b)
        x.XSetInputFocus(display, x.XDefaultRootWindow(display), 1, 0)
        x.XFlush(display)
        assert_frozen()
        x.XSetInputFocus(display, window, 1, 0)
        x.XFlush(display)
        assert_frozen()  # Focus return must not automatically resume.
        key(0xff1b)
        key(ord('h'),True)
        for _ in range(30):
            if title(window) == 'Omarchy Arcade': break
            time.sleep(.1)
        assert title(window) == 'Omarchy Arcade', title(window)
        assert windows() == [window]
        key(ord('q'), True)
        app.wait(timeout=5)
        assert app.returncode == 0
    finally:
        if app.poll() is None:
            app.terminate(); app.wait()
    print('Solo: live play, pause, focus-loss pause, explicit resume and same-window shelf passed', flush=True)

    prefs = Path(tmp)/'state/omarchy-retro-arcade/blast.json'
    settings = json.loads(prefs.read_text())
    settings.update(humans=2, bots=0)
    prefs.write_text(json.dumps(settings))
    app, window = launch(env)
    try:
        key(0xff0d)
        # P2 places using Enter and escapes with the independent arrow controls.
        key(0xff0d, hold=.02)
        key(0xff51, hold=.6)
        # P1 deliberately loses three rounds to exercise an entire local match.
        for round_number in range(3):
            key(0x20, hold=.02)
            time.sleep(2.85)
            if round_number < 2:
                key(0xff0d)
        recorded = json.loads(prefs.read_text())
        assert recorded['matches'] == 1 and recorded['wins'] == [0, 1, 0, 0], recorded
        if out:
            capture().save(out/'local-match.png')
        key(ord('h'), True)
        for _ in range(30):
            if title(window) == 'Omarchy Arcade': break
            time.sleep(.1)
        assert title(window) == 'Omarchy Arcade', title(window)
        key(ord('q'), True)
        app.wait(timeout=5)
        assert app.returncode == 0
    finally:
        if app.poll() is None:
            app.terminate(); app.wait()
    loaded = json.loads(prefs.read_text())
    assert loaded['matches'] == 1 and loaded['wins'][1] == 1
    app, window = launch(env)
    try:
        key(ord('q'), True)
        app.wait(timeout=5)
        assert json.loads(prefs.read_text())['wins'][1] == 1
    finally:
        if app.poll() is None:
            app.terminate(); app.wait()
    print('Local: independent P2 bomb/movement, completed first-to-three, one record and restart persistence passed', flush=True)
