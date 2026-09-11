"""X11 integration harness; not an application/runtime dependency.
Run inside xvfb-run, with the built binary as argv[1].
"""
import ctypes as C
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

binary = str(Path(sys.argv[1]).resolve())
x = C.CDLL('libX11.so.6')
xt = C.CDLL('libXtst.so.6')
x.XOpenDisplay.argtypes=[C.c_char_p]; x.XOpenDisplay.restype=C.c_void_p
x.XDefaultRootWindow.argtypes=[C.c_void_p]; x.XDefaultRootWindow.restype=C.c_ulong
x.XQueryTree.argtypes=[C.c_void_p,C.c_ulong,C.POINTER(C.c_ulong),C.POINTER(C.c_ulong),C.POINTER(C.POINTER(C.c_ulong)),C.POINTER(C.c_uint)]
x.XSetInputFocus.argtypes=[C.c_void_p,C.c_ulong,C.c_int,C.c_ulong]
x.XFetchName.argtypes=[C.c_void_p,C.c_ulong,C.POINTER(C.c_void_p)]
x.XFree.argtypes=[C.c_void_p]
x.XKeysymToKeycode.argtypes=[C.c_void_p,C.c_ulong]; x.XKeysymToKeycode.restype=C.c_uint
x.XFlush.argtypes=[C.c_void_p]
x.XCloseDisplay.argtypes=[C.c_void_p]
xt.XTestFakeKeyEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
xt.XTestFakeButtonEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
xt.XTestFakeMotionEvent.argtypes=[C.c_void_p,C.c_int,C.c_int,C.c_int,C.c_ulong]
display=x.XOpenDisplay(os.environ['DISPLAY'].encode()); assert display

def windows():
    root=C.c_ulong();parent=C.c_ulong();children=C.POINTER(C.c_ulong)();n=C.c_uint()
    x.XQueryTree(display,x.XDefaultRootWindow(display),C.byref(root),C.byref(parent),C.byref(children),C.byref(n))
    result=[]
    for i in range(n.value):
        name=C.c_void_p();x.XFetchName(display,children[i],C.byref(name))
        if name.value:
            text=C.string_at(name).decode(errors='replace');x.XFree(name)
            if 'Omarchy' in text:result.append(children[i])
    if children:x.XFree(children)
    return result

def key(sym,ctrl=False,hold=.06):
    control=x.XKeysymToKeycode(display,0xffe3);code=x.XKeysymToKeycode(display,sym)
    if ctrl:xt.XTestFakeKeyEvent(display,control,1,0)
    xt.XTestFakeKeyEvent(display,code,1,0);x.XFlush(display);time.sleep(hold)
    xt.XTestFakeKeyEvent(display,code,0,0)
    if ctrl:xt.XTestFakeKeyEvent(display,control,0,0)
    x.XFlush(display);time.sleep(.18)

def click(px,py):
    xt.XTestFakeMotionEvent(display,-1,px,py,0);x.XFlush(display);time.sleep(.1)
    xt.XTestFakeButtonEvent(display,1,1,0);x.XFlush(display);time.sleep(.06)
    xt.XTestFakeButtonEvent(display,1,0,0);x.XFlush(display);time.sleep(.2)

with tempfile.TemporaryDirectory(prefix='arcade-native-') as tmp:
    state=Path(tmp)/'state'
    env=dict(os.environ,XDG_STATE_HOME=str(state),XDG_CONFIG_HOME=tmp+'/config',XDG_DATA_HOME=tmp+'/data')
    app=subprocess.Popen([binary],env=env)
    try:
        for _ in range(100):
            found=windows()
            if found:break
            assert app.poll() is None
            time.sleep(.1)
        assert len(found)==1,found
        window=found[0];time.sleep(.6);x.XSetInputFocus(display,window,1,0);x.XFlush(display);time.sleep(.3)
        duplicate=subprocess.run([binary],env=env,capture_output=True,timeout=5)
        assert duplicate.returncode!=0 and b'already running' in duplicate.stderr
        def ready(name):
            for _ in range(160):
                ptr=C.c_void_p();x.XFetchName(display,window,C.byref(ptr))
                value=C.string_at(ptr).decode(errors='replace') if ptr.value else ''
                if ptr.value:x.XFree(ptr)
                if value==name:
                    time.sleep(.3)
                    assert windows()==[window]
                    return
                assert app.poll() is None
                time.sleep(.1)
            raise AssertionError(('Window never became ready',name,value))
        def enter(name):
            key(0xff0d);ready(name+' - Omarchy Arcade')
        def home():
            key(ord('h'),True);ready('Omarchy Arcade')
        # Enter Solitaire from the shared shelf, draw, leave and reopen.
        key(0xff53);enter('Solitaire');key(0x20);home()
        save=state/'omarchy-solitaire/session.json'
        first=json.loads(save.read_text());assert first['game']['state']['moves']==1,first
        enter('Solitaire');home()
        second=json.loads(save.read_text());assert second['game']==first['game']
        # Scram and Invaders use their existing storage identities.
        key(0xff53);enter('Scram');key(0xff53,hold=.5);home()
        scram=json.loads((state/'omarchy-munch/session.json').read_text());assert scram['version']>=1
        key(0xff53);enter('Invaders');key(0x20,hold=.4);home()
        invaders=json.loads((state/'omarchy-invaders/session.json').read_text());assert invaders['version']==1
        key(0xff53);enter('Chess');home()
        assert (state/'omarchy-chess/session.json').is_file()
        # Pinball runs within the SAME native window; no SDL desktop window.
        key(0xff53);enter('Circuit Pinball');time.sleep(.7)
        assert windows()==[window],windows()
        key(0x20,hold=.6);key(ord('a'),hold=.2);key(ord('d'),hold=.2)
        key(ord('h'),True);time.sleep(.2)
        # Confirm return using the first modal action; keyboard focus is explicit.
        key(0xff0d);ready('Omarchy Arcade')
        time.sleep(.4)
        assert windows()==[window]
        key(0xff53);enter('Solitaire');home()
        # Close from the app-level shortcut.
        key(ord('q'),True);app.wait(timeout=8);assert app.returncode==0
        assert json.loads(save.read_text())['game']==first['game']
        print('PASS: singleton; one window across five games; Solitaire draw/save/reopen; legacy save paths; native keys; clean shutdown.')
    finally:
        if app.poll() is None:app.kill();app.wait()
x.XCloseDisplay(display)
