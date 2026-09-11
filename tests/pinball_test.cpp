#include "PinballModel.h"
#include <sstream>
#include <iostream>
#include <cmath>
#include <cstdlib>
void check(bool b,const char* name){if(!b){std::cerr<<"FAIL "<<name<<'\n';std::exit(1);}}
int main(){
 oma::Model m;check(m.waiting&&m.balls==3&&m.score==0,"new game");
 for(int i=0;i<120;++i)m.step(1./120,{false,false,true});check(m.charge>.8&&m.waiting,"charge");m.step(1./120,{});check(!m.waiting&&m.velocity.y<0&&m.saveTime>0,"release launches");
 m.paused=true;auto before=m.ball;double time=m.time;m.step(1./120,{true,true,true});check(m.ball.x==before.x&&m.ball.y==before.y&&m.time==time,"pause freezes game");m.paused=false;
 m.ball=oma::Model::bumpers()[0]+oma::Vec{0,-37};m.velocity={0,100};m.step(1./120,{});check(m.score>=100&&m.velocity.y<0,"bumper scores and reflects");
 m.lanes.fill(true);m.targets.fill(true);m.step(1./120,{});check(m.multiplier==2&&m.circuits==1&&!m.lanes[0],"mission resets lights and increases multiplier");
 m.ball={300,916};m.velocity={0,100};m.saveTime=1;m.step(1./120,{});check(m.waiting&&m.balls==3,"ball save");
 m.launch(.8);m.saveTime=0;m.ball={300,916};m.velocity={0,100};m.step(1./120,{});check(m.balls==2&&m.waiting&&m.multiplier==1,"drain loses ball and resets multiplier");
 m.launch(.8);m.nudge();m.nudge();m.nudge();check(m.tilted&&m.saveTime==0,"repeated nudge tilts");
 for(int k=0;k<2;++k){m.waiting=false;m.ball={300,916};m.velocity={0,100};m.saveTime=0;m.step(1./120,{});}check(m.gameOver&&m.balls==0,"game over");m.launch(1);check(m.waiting,"cannot launch after game over");
 m.newGame();m.waiting=false;m.ball={248,792};m.velocity={0,150};m.saveTime=0;
 for(int i=0;i<10;++i)m.step(1./120,{true,false,false});check(m.velocity.y<0,"flipper lifts ball");
 m.newGame();m.launch(.9);for(int i=0;i<200;++i)m.step(1./120,{i%20<10,i%25<10,false});
 std::ostringstream saved;check(m.write(saved),"save");std::istringstream in(saved.str());oma::Model restored;check(restored.read(in)&&restored.paused&&restored.score==m.score&&restored.ball.x==m.ball.x,"resume exact state paused");
 std::istringstream corrupt("OMARCHY_PINBALL 1\nNaN 0");check(!restored.read(corrupt),"reject malformed save");
 std::istringstream foreign("UNKNOWN 1\n");check(!restored.read(foreign),"reject wrong save version");
 oma::Model soak;int games=0;for(int i=0;i<120*180;++i){if(soak.gameOver){++games;soak.newGame();}if(soak.waiting)soak.launch(.8);soak.step(1./120,{i%47<23,i%61<30,false});check(std::isfinite(soak.ball.x)&&std::isfinite(soak.ball.y)&&std::abs(soak.velocity.x)<20000,"finite physics soak");}
 std::cout<<"Pinball checks passed; 180 simulated seconds, games="<<games<<" final score="<<soak.score<<'\n';
}
