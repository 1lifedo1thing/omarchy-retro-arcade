"""Capture running Blast after real keyboard input at compact, 2x and light settings.
Usage under Xvfb: python3 scripts/blast-renders.py BINARY OUTPUT_DIRECTORY
"""
from pathlib import Path
from PIL import ImageGrab
exec(compile((Path(__file__).parent/'native-check.py').read_text().split(
    'with tempfile.TemporaryDirectory')[0], 'arcade-input-helpers', 'exec'))
output = Path(sys.argv[2]); output.mkdir(parents=True, exist_ok=True)
for name, width, height, scale, light in [
    ('compact', 900, 760, 1, False), ('200-percent', 1120, 860, 2, False),
    ('light', 1120, 860, 1, True),
]:
    with tempfile.TemporaryDirectory(prefix='blast-render-') as tmp:
        env = dict(os.environ, XDG_STATE_HOME=tmp+'/state', XDG_CONFIG_HOME=tmp+'/config', XDG_DATA_HOME=tmp+'/data', WINIT_X11_SCALE_FACTOR=str(scale))
        if light:
            theme = Path(tmp)/'state/omarchy/current/theme/colors.toml'
            theme.parent.mkdir(parents=True)
            theme.write_text('background = "#f2efe5"\nforeground = "#283137"\naccent = "#a75431"\n')
        cmd = [binary, '--game', 'blast']
        if name == 'compact': cmd.append('--compact')
        app = subprocess.Popen(cmd, env=env)
        try:
            for _ in range(100):
                found = windows()
                if found: break
                assert app.poll() is None
                time.sleep(.1)
            assert len(found) == 1
            time.sleep(.8)
            x.XSetInputFocus(display, found[0], 1, 0); x.XFlush(display); time.sleep(.3)
            click(width*scale//2, int((height/2+102)*scale))
            key(ord('d'), hold=.42); key(0x20, hold=.02); key(ord('a'), hold=.38)
            ImageGrab.grab(xdisplay=os.environ['DISPLAY']).crop((0, 0, width*scale, height*scale)).save(output/(name+'.png'))
            key(ord('q'), True); app.wait(timeout=5); assert app.returncode == 0
        finally:
            if app.poll() is None: app.terminate(); app.wait()
        print(name, 'captured from running game', flush=True)
