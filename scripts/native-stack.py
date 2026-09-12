"""Run inside an X11 display: python scripts/native-stack.py BINARY [OUTDIR]."""
import ctypes as C
import json, os, subprocess, sys, tempfile, time
from pathlib import Path
from PIL import ImageGrab
scale=float(os.environ.get('WINIT_X11_SCALE_FACTOR','1'))
out=Path(sys.argv[2] if len(sys.argv)>2 else 'games/stack/docs');out.mkdir(parents=True,exist_ok=True)
helpers=(Path(__file__).parent/'native-check.py').read_text().split('with tempfile.TemporaryDirectory')[0]
exec(helpers)
x.XResizeWindow.argtypes=[C.c_void_p,C.c_ulong,C.c_uint,C.c_uint]
with tempfile.TemporaryDirectory(prefix='stack-native-') as tmp:
 env=dict(os.environ,XDG_STATE_HOME=tmp+'/state',XDG_CONFIG_HOME=tmp+'/config',XDG_DATA_HOME=tmp+'/data')
 if os.environ.get('STACK_LIGHT'):
  theme=Path(tmp)/'config/omarchy/current/theme/colors.toml';theme.parent.mkdir(parents=True);theme.write_text('background = "#f3f0e7"\nforeground = "#262b24"\naccent = "#526f3a"\n')
 app=subprocess.Popen([binary,'--game','stack'],env=env)
 try:
  for _ in range(100):
   found=windows()
   if found:break
   assert app.poll() is None;time.sleep(.1)
  assert len(found)==1
  w=found[0];time.sleep(.8);x.XSetInputFocus(display,w,1,0);x.XFlush(display);time.sleep(.2)
  key(0xff0d)
  # Play using genuine XTest keyboard events, including hold and both rotations.
  key(0xff51,hold=.32);key(0x20);key(0xff53,hold=.20);key(ord('z'));key(0x20);key(ord('c'));key(0xff52);key(0xff54,hold=.4)
  time.sleep(.2);ImageGrab.grab(xdisplay=os.environ['DISPLAY']).crop((0,0,int(1280*scale),int(900*scale))).save(out/'stack-game.png')
  key(ord('p'));time.sleep(.25)
  save=Path(tmp)/'state/omarchy-stack/session.json';before=json.loads(save.read_text())['marathon'];assert before['locks']>=2 and before['held'] is not None
  time.sleep(2.2);after=json.loads(save.read_text())['marathon'];assert before==after,'Pause changed simulation'
  key(ord('p'));key(0xff53,hold=.2)
  x.XSetInputFocus(display,x.XDefaultRootWindow(display),1,0);x.XFlush(display);time.sleep(.5)
  before=json.loads(save.read_text())['marathon'];time.sleep(2.2);assert json.loads(save.read_text())['marathon']==before,'Focus loss changed simulation'
  x.XSetInputFocus(display,w,1,0);x.XFlush(display);time.sleep(.3)
  key(ord('h'),True);assert windows()==[w]
  saved=json.loads(save.read_text())['marathon'];assert saved==before
  # Reopen from shelf, choose resume by mouse, verify exact saved state on leaving.
  key(0xff0d);time.sleep(.3)
  ImageGrab.grab(xdisplay=os.environ['DISPLAY']).crop((0,0,int(1280*scale),int(900*scale))).save(out/'stack-menu.png')
  key(ord('r'));key(ord('p'));time.sleep(.2)
  restored=json.loads(save.read_text())['marathon']
  for field in ['board','queue','rng','held','score']:assert restored[field]==saved[field],field
  assert restored['ticks']>=saved['ticks'];key(ord('h'),True);key(0xff0d)
  saved=restored
  key(ord('s'));key(0xff53);key(0x20);key(ord('h'),True)
  both=json.loads(save.read_text());assert both['sprint']['mode']=='Sprint' and both['marathon']==saved
  key(0xff0d);key(0xff0d);key(ord('p'))
  x.XResizeWindow(display,w,int(900*scale),int(760*scale));x.XFlush(display);time.sleep(.4)
  ImageGrab.grab(xdisplay=os.environ['DISPLAY']).crop((0,0,int(900*scale),int(760*scale))).save(out/'stack-compact.png')
  key(ord('q'),True);app.wait(timeout=5)
  assert app.returncode==0
  print('PASS native keyboard, drops, turns, hold, pause, focus loss, single-window shelf, both saved modes, resize and shutdown')
 finally:
  if app.poll() is None:app.terminate();app.wait()
