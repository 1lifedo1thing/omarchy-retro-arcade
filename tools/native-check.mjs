// Uses the separately compiled Rust xprobe helper and Xvfb. No game runtime dependency.
import {spawn,execFileSync} from 'node:child_process';
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import assert from 'node:assert/strict';
const binary=process.env.INVADERS_BINARY;assert(binary,'Set INVADERS_BINARY');
const root=path.resolve(import.meta.dirname,'..');
const state=fs.mkdtempSync(path.join(os.tmpdir(),'invaders-window-'));
const env={...process.env,DISPLAY:'127.0.0.1:119',LIBGL_ALWAYS_SOFTWARE:'1',XDG_STATE_HOME:state};
const wait=ms=>new Promise(r=>setTimeout(r,ms));
const probe=(...args)=>execFileSync(process.env.XPROBE||'/tmp/invaders-xprobe',args,{env,encoding:'utf8'}).trim();
const server=spawn(process.env.XVFB||'Xvfb',[':119','-screen','0','1200x1100x24','-nolisten','unix','-listen','tcp','-ac'],{env,stdio:'ignore'});
let app;const read=()=>JSON.parse(fs.readFileSync(path.join(state,'omarchy-invaders/session.json'),'utf8'));
const launch=async()=>{app=spawn(binary,[],{env,stdio:'inherit'});await wait(1600);probe('focus');await wait(200);};
const stop=async()=>{probe('key','q','60','ctrl');for(let i=0;i<40&&app.exitCode===null;i++)await wait(200);assert.equal(app.exitCode,0);};
const capture=name=>execFileSync('import',['-window',probe('id'),path.join(root,'docs',name+'.png')],{env});
try {
 await wait(600);await launch();probe('key','Right','500');probe('key','space','2600');capture('preview');probe('key','p');await wait(5300);
 const paused=read();assert(paused.game.ship>500,'Arrow movement failed');assert(paused.game.score>0,'Firing/scoring failed');await wait(5300);assert.deepEqual(read().game,paused.game,'Pause failed');console.log('PASS movement, firing, scoring and pause');
 probe('key','p');await wait(250);probe('blur');await wait(5300);const blurred=read().game;await wait(5300);assert.deepEqual(read().game,blurred,'Focus pause failed');console.log('PASS focus-loss pause');
 probe('key','m','60','ctrl');probe('resize','600','680');capture('compact');await stop();const saved=read();assert(saved.sound);await launch();await wait(5300);assert.deepEqual(read().game,saved.game,'Restored game advanced');
 probe('key','comma','60','ctrl');probe('key','Escape');await wait(5300);assert.deepEqual(read().game,saved.game,'Closing settings resumed game');console.log('PASS restore, settings dismissal and sound persistence');
 const theme=path.join(state,'omarchy/current/theme');fs.mkdirSync(theme,{recursive:true});fs.writeFileSync(path.join(theme,'colors.toml'),'background="#fafafa"\nforeground="#181818"\naccent="#244c38"\n');await wait(2200);probe('key','p');capture('light');await stop();console.log('PASS native quit; screenshots captured');
}finally{app?.kill();server.kill();}
