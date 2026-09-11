"""Real XTest input in the Arcade window. Run in Xvfb; fixtures are local saves.
python scripts/native-snake.py BINARY [OUTDIR]
No claim of human playtesting or sound-device verification.
"""
import ctypes as C
import json, os, subprocess, sys, tempfile, time
from pathlib import Path
from PIL import ImageGrab
out=Path(sys.argv[2] if len(sys.argv)>2 else 'games/snake/docs');out.mkdir(parents=True,exist_ok=True)
exec((Path(__file__).parent/'native-check.py').read_text().split('with tempfile.TemporaryDirectory')[0])
def wait_for(predicate, timeout=8):
 end=time.monotonic()+timeout
 while time.monotonic()<end:
  if predicate():return
  time.sleep(.03)
 raise AssertionError('Timed out waiting for native state')
def focus(w):x.XSetInputFocus(display,w,1,0);x.XFlush(display);time.sleep(.15)
def snap(name,w,h):ImageGrab.grab(xdisplay=os.environ['DISPLAY']).crop((0,0,w,h)).save(out/name)
def launch(env,compact=False):
 app=subprocess.Popen([binary,'--game','snake']+(['--compact'] if compact else []),env=env)
 wait_for(lambda:bool(windows()))
 w=windows()[0];time.sleep(.5);focus(w);return app,w

def quit_app(app):key(ord('q'),True);app.wait(timeout=5);assert app.returncode==0
with tempfile.TemporaryDirectory(prefix='snake-native-') as tmp:
 env=dict(os.environ,XDG_STATE_HOME=tmp+'/state',XDG_CONFIG_HOME=tmp+'/config',XDG_DATA_HOME=tmp+'/data')
 save=Path(tmp)/'state/omarchy-snake/session.json'
 app,w=launch(env)
 try:
  wait_for(save.exists);initial=json.loads(save.read_text())['sim'];assert initial['tick']==0 and initial['speed']=='Normal'
  time.sleep(.3);assert json.loads(save.read_text())['sim']==initial,'Moved before explicit start'
  snap('snake-menu.png',1120,860)
  key(ord('1'));key(0xff0d);key(ord('w'),hold=.015);key(0xff1b,hold=.015)
  paused=json.loads(save.read_text())['sim'];assert paused['tick']>0 and paused['direction']=='Up';assert paused['buffered']==[]
  time.sleep(.35);assert json.loads(save.read_text())['sim']==paused
  key(0xff0d);time.sleep(.3);key(0xff1b);assert json.loads(save.read_text())['sim']==paused,'Countdown cancellation advanced simulation'
  key(0xff0d);time.sleep(3.1)
  focus(x.XDefaultRootWindow(display));before=json.loads(save.read_text())['sim'];assert before['tick']>paused['tick']
  focus(w);time.sleep(.5);assert json.loads(save.read_text())['sim']==before,'Focus return auto-resumed'
  key(ord('h'),True);assert windows()==[w];saved=json.loads(save.read_text())['sim'];assert saved==before
  key(0xff51);key(0xff0d);time.sleep(.3);assert windows()==[w]
  key(ord('h'),True);assert json.loads(save.read_text())['sim']==saved,'Shelf restore changed phase or RNG'
  key(0xff0d);key(0xff0d);time.sleep(3.2);key(0xff1b);assert json.loads(save.read_text())['sim']['tick']>saved['tick']
  quit_app(app)
 finally:
  if app.poll() is None:app.terminate();app.wait()
 # Valid local snapshot positioned to eat immediately. Ordinary restore path, no test code in app.
 fixture={'rules':'snake-v1','body':[253,252,251,250,249,248,224,200],'direction':'Right','buffered':[],'food':254,'rng':7,'score':40,'speed':'Slow','tick':0,'phase':0,'outcome':'Playing'}
 # A fake sound process tests ownership/cleanup separately from audible device acceptance.
 bindir=Path(tmp)/'bin';bindir.mkdir();pidfile=Path(tmp)/'sound-pid';sound=bindir/'paplay'
 sound.write_text('#!/usr/bin/env python3\nimport os,time\nopen(os.environ["SNAKE_SOUND_PID"],"w").write(str(os.getpid()))\ntime.sleep(30)\n');sound.chmod(0o755)
 env.update(PATH=str(bindir)+':'+env['PATH'],SNAKE_SOUND_PID=str(pidfile))
 for variant,compact,scale in [('dark',False,1),('compact',True,1),('light',False,1),('200',False,2)]:
  save.parent.mkdir(parents=True,exist_ok=True);save.write_text(json.dumps({'version':1,'sim':fixture}))
  (save.parent/'records.json').write_text(json.dumps({'version':1,'best':[40,0,0],'preferences':{'speed':'Slow','audio':True,'reduced_motion':False}}))
  config=Path(tmp)/'state/omarchy/current/theme/colors.toml';config.parent.mkdir(parents=True,exist_ok=True)
  config.write_text('background = "#f5f3ed"\nforeground = "#24352b"\naccent = "#526b39"\n' if variant=='light' else 'background = "#171c1a"\nforeground = "#e4e8df"\naccent = "#b3cb92"\n')
  env['WINIT_X11_SCALE_FACTOR']=str(scale)
  if pidfile.exists():pidfile.unlink()
  app,w=launch(env,compact)
  try:
   key(0xff0d);wait_for(pidfile.exists,5);time.sleep(.06)
   width,height=(900,760) if compact else (1120,860)
   snap('snake-'+variant+'.png',width*scale,height*scale)
   sound_pid=int(pidfile.read_text())
   if variant=='compact':
    key(ord('h'),True);assert windows()==[w]
   else:key(0xff1b,hold=.015)
   ate=json.loads(save.read_text())['sim'];assert ate['score']==50 and len(ate['body'])==9
   def audio_stopped():
    try:os.kill(sound_pid,0);return False
    except ProcessLookupError:return True
   wait_for(audio_stopped)
   if variant=='dark':
    # Resume, collide, then restart via real keyboard and verify score reset.
    key(0xff0d);wait_for(lambda:not save.exists(),7)
    snap('snake-result.png',1120,860)
    key(0xff0d);key(0xff1b,hold=.015);assert json.loads(save.read_text())['sim']['score']==0
   if variant!='compact':key(ord('h'),True)
   assert windows()==[w];wait_for(audio_stopped)
   quit_app(app)
  finally:
   if app.poll() is None:app.terminate();app.wait()
 print('PASS native start, WASD turn, eat, collision, restart, pause/countdown, focus loss, no auto-resume, single-window shelf, exact restored run, sound child cleanup, compact/light/dark/200%')
x.XCloseDisplay(display)
