// Native settings reload and keyboard retry regression checks, using isolated user data.
import {spawn,execFileSync} from 'node:child_process';import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import assert from 'node:assert/strict';
const root=path.resolve(import.meta.dirname,'..');const state=fs.mkdtempSync(os.tmpdir()+'/orbit-artwork-');
const env={...process.env,DISPLAY:'127.0.0.1:124',LIBGL_ALWAYS_SOFTWARE:'1',XDG_STATE_HOME:state,XDG_DATA_HOME:state+'/data'};
const wait=ms=>new Promise(r=>setTimeout(r,ms));const server=spawn(process.env.XVFB||'Xvfb',[':124','-screen','0','1200x1100x24','-nolisten','unix','-listen','tcp','-ac'],{env,stdio:'ignore'});let app;
const probe=(...args)=>execFileSync(process.env.XPROBE||'/tmp/invaders-xprobe',args,{env,encoding:'utf8'}).trim();
const capture=name=>execFileSync('import',['-window',probe('id'),root+'/docs/'+name+'.png'],{env});
const clickArt=(x)=>{capture('orbit-settings');const pixels=execFileSync('convert',[root+'/docs/orbit-settings.png','-crop','860x150+0+0','-depth','8','rgb:-']);let left,top;
outer:for(let y=0;y<150;y++){let run=0;for(let col=0;col<860;col++){const i=(y*860+col)*3;const r=pixels[i],g=pixels[i+1],b=pixels[i+2];if(r===g&&g===b&&r>=40&&r<=50){run++;if(run===90){left=col-89;top=y;break outer;}}else run=0;}}
assert.notEqual(top,undefined,'Settings title not located');probe('click',String(left+x-16),String(top+203));};
const saved=()=>JSON.parse(fs.readFileSync(state+'/omarchy-invaders/session.json','utf8'));
const launch=async()=>{app=spawn(process.env.INVADERS_BINARY,[],{env});await wait(1000);probe('focus');await wait(100);};
const stop=async()=>{probe('key','q','60','ctrl');for(let n=0;n<30&&app.exitCode===null;n++)await wait(100);assert.equal(app.exitCode,0);};
try{
 await wait(600);await launch();capture('orbit-opening');probe('key','comma','60','ctrl');await wait(200);
 clickArt(100);await wait(200);const atlas=state+'/data/omarchy-invaders/orbit/atlas.png';assert(fs.existsSync(atlas),'Editable copy not created');const original=fs.readFileSync(atlas);
 await wait(5200);const before=saved().game;
 // Replacing the whole atlas exercises the actual PNG decoder and settings handler.
 const png=fs.readFileSync(root+'/assets/orbit/atlas.png');fs.writeFileSync(atlas,png);clickArt(245);await wait(200);capture('orbit-settings');
 fs.writeFileSync(atlas,'broken image');clickArt(245);await wait(200);capture('orbit-invalid-artwork');
 clickArt(100);assert.equal(fs.readFileSync(atlas,'utf8'),'broken image','Create copy overwrote existing file');
 fs.writeFileSync(atlas,original);clickArt(245);await wait(5200);assert.deepEqual(saved().game,before,'Artwork changes advanced or reset the run');assert.equal(saved().custom_art,true,'Custom artwork selection not saved');
 probe('key','Escape');probe('resize','600','680');await wait(150);capture('orbit-compact-paused');await stop();
 const end=saved();end.game.over=true;end.game.lives=0;end.game.score=1234;end.high=1234;fs.writeFileSync(state+'/omarchy-invaders/session.json',JSON.stringify(end));
 await launch();capture('orbit-game-over');probe('key','Return');await wait(5200);assert.equal(saved().game.over,false,'Enter did not restart');assert.equal(saved().game.score,0);assert.equal(saved().high,1234);await stop();
 console.log('PASS native artwork reload, invalid-file preservation, pause, compact UI and Enter retry');
}finally{app?.kill();server.kill();}
