"""Local-only native sharing check. Run under Xvfb with APP SERVICE OUTDIR arguments."""
import os, subprocess, sys, time, socket, json, sqlite3, tempfile
from pathlib import Path
from PIL import ImageGrab
root=Path(__file__).resolve().parent.parent
service_binary=str(Path(sys.argv[2]).resolve());out=Path(sys.argv[3]);out.mkdir(parents=True,exist_ok=True)
exec((root/'scripts/native-check.py').read_text().split('with tempfile.TemporaryDirectory')[0])
with tempfile.TemporaryDirectory(prefix='stack-sharing-') as tmp:
 with socket.socket() as sock:sock.bind(('127.0.0.1',0));port=sock.getsockname()[1]
 env=dict(os.environ,XDG_STATE_HOME=tmp+'/state',XDG_CONFIG_HOME=tmp+'/config',XDG_DATA_HOME=tmp+'/data',ARCADE_LEADERBOARD_URL=f'http://127.0.0.1:{port}',ARCADE_DATABASE=tmp+'/db.sqlite3',ARCADE_BIND=f'127.0.0.1:{port}')
 service=subprocess.Popen([service_binary],env=env,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
 app=subprocess.Popen([binary,'--game','stack'],env=env)
 try:
  for _ in range(100):
   found=windows()
   if found:break
   assert app.poll() is None;time.sleep(.1)
  w=found[0];time.sleep(.8);x.XSetInputFocus(display,w,1,0);x.XFlush(display);time.sleep(.3)
  def shot(name):ImageGrab.grab(xdisplay=env['DISPLAY']).crop((0,0,1120,860)).save(out/(name+'.png'))
  click(28,552);key(0xff0d);time.sleep(.6)
  for _ in range(15):key(0x20,hold=.03)
  shot('stack-community-result');click(449,427);time.sleep(.3);shot('stack-share-disclosure')
  click(493,470)
  for letter in 'native-player':key(ord(letter),hold=.02)
  click(399,492);click(446,512)
  identity_file=Path(env['XDG_STATE_HOME'])/'omarchy-retro-arcade/leaderboard.json'
  for _ in range(100):
   if identity_file.exists():
    saved=json.loads(identity_file.read_text())
    with sqlite3.connect(env['ARCADE_DATABASE']) as db:count=db.execute('SELECT COUNT(*) FROM scores').fetchone()[0]
    if count==1 and not saved['pending']:break
   time.sleep(.1)
  assert count==1 and not saved['pending'],'Native score was not accepted'
  assert saved['consent'] and saved['alias']=='native-player'
  shot('stack-shared')
  key(ord('h'),True)
  for _ in range(5):key(0xff53)
  key(0xff0d);click(94,574);time.sleep(.5);shot('stack-community-board')
  key(ord('q'),True);app.wait(timeout=5)
  print('PASS native optional ticket, consent, alias, replay submission, identity persistence, retry cleanup and own leaderboard row')
 finally:
  if app.poll() is None:app.terminate();app.wait()
  service.terminate();service.wait()
