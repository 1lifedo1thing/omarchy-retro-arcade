#include "PinballModel.h"
#include <cmath>
#include <algorithm>
#include <istream>
#include <ostream>
#include <iomanip>
namespace oma {
static double dot(Vec a,Vec b){return a.x*b.x+a.y*b.y;}
static double length(Vec a){return std::sqrt(dot(a,a));}
const std::vector<Segment>& Model::walls(){
 static const std::vector<Segment> data={
 {{22,865},{22,180}},{{22,180},{48,102}},{{48,102},{116,50}},{{116,50},{475,50}},
 {{475,50},{552,112}},{{552,112},{580,190}},{{580,190},{580,865}},
 {{532,225},{532,875}},
 {{22,700},{90,762}},{{90,762},{175,832}},
 {{510,700},{452,762}},{{452,762},{425,832}},
 {{85,360},{85,580}},{{490,360},{490,580}},
 {{85,580},{172,704}},{{490,580},{428,704}},
 {{142,645},{182,712}},{{182,712},{142,699}},{{142,699},{142,645}},
 {{458,645},{418,712}},{{418,712},{458,699}},{{458,699},{458,645}}};return data;
}
std::array<Vec,3> Model::bumpers(){return {{{220,285},{370,285},{295,395}}};}
std::array<Vec,3> Model::lanePositions(){return {{{168,164},{295,140},{420,164}}};}
std::array<Vec,3> Model::targetPositions(){return {{{111,474},{295,535},{463,474}}};}
Vec Model::leftTip()const{return Vec{205,785}+Vec{std::cos(leftAngle),std::sin(leftAngle)}*90;}
Vec Model::rightTip()const{return Vec{395,785}+Vec{std::cos(rightAngle),std::sin(rightAngle)}*90;}
void Model::newGame(){*this=Model{};}
void Model::addScore(int points){score=std::min(999999999,score+points*multiplier);}
void Model::launch(double power){if(!waiting||gameOver||paused)return;power=std::max(.2,std::min(1.,power));waiting=false;velocity={-10,-1100-power*360};saveTime=9;charge=0;events.push_back(Event::Launch);message="Light three lanes and three targets.";}
void Model::collideSegment(Vec a,Vec b,double restitution,Vec surface,double radius){
 Vec ab=b-a; double ll=dot(ab,ab);if(ll==0)return;
 double t=std::max(0.,std::min(1.,dot(ball-a,ab)/ll));Vec nearest=a+ab*t,n=ball-nearest;double d=length(n);
 if(d>=radius)return;
 if(d<.0001){n={-ab.y,ab.x};d=length(n);}n=n*(1./d);
 ball=nearest+n*(radius+.01);double approach=dot(velocity-surface,n);
 if(approach<0)velocity=velocity-n*((1+restitution)*approach);
}
void Model::collideCircle(Vec center,double radius,double restitution,double boost){
 Vec n=ball-center;double d=length(n);if(d>=radius+9)return;if(d<.001){n={0,1};d=1;}n=n*(1./d);
 ball=center+n*(radius+9.01);double approach=dot(velocity,n);
 if(approach<0)velocity=velocity-n*((1+restitution)*approach);
 velocity=velocity+n*boost;
}
void Model::drain(){
 if(saveTime>0&&!tilted){waiting=true;ball={555,824};velocity={};events.push_back(Event::Save);message="BALL SAVED. Launch again.";return;}
 --balls;events.push_back(Event::Drain);waiting=true;ball={555,824};velocity={};tilted=false;nudgeHeat=0;
 multiplier=1;lanes.fill(false);targets.fill(false);
 if(balls<=0){balls=0;gameOver=true;message="GAME OVER. Start a new game.";}else message="Next ball. Hold SPACE to charge.";
}
void Model::nudge(){
 if(waiting||gameOver||paused||tilted)return;
 nudgeHeat+=1.1;if(nudgeHeat>3.){tilted=true;saveTime=0;events.push_back(Event::Tilt);message="TILT. Flippers disabled until the next ball.";}
 else {velocity.y-=180;velocity.x+=ball.x<300?75:-75;message="Nudge carefully. Repeated nudges cause tilt.";}
}
void Model::step(double dt,Input input){
 events.clear();if(paused||gameOver)return;dt=std::max(0.,std::min(dt,1./30));time+=dt;
 nudgeHeat=std::max(0.,nudgeHeat-dt*.35);
 for(auto& g:bumperGlow)g=std::max(0.,g-dt);for(auto& g:targetGlow)g=std::max(0.,g-dt);
 if(waiting){if(input.plunger)charge=std::min(1.,charge+dt*.9);if(wasPlunger&&!input.plunger)launch(charge);wasPlunger=input.plunger;}
 else saveTime=std::max(0.,saveTime-dt);
 for(int sub=0;sub<4;++sub){
  double h=dt/4;double la=leftAngle,ra=rightAngle;
  auto approach=[h](double v,double target){double delta=std::max(-h*15.,std::min(h*15.,target-v));return v+delta;};
  leftAngle=approach(leftAngle,input.left&&!tilted?-.55:.35);rightAngle=approach(rightAngle,input.right&&!tilted?3.691592653589793:2.791592653589793);
  if(waiting)continue;
  velocity.y+=880*h;velocity=velocity*(1-h*.018);double speed=length(velocity);if(speed>1800)velocity=velocity*(1800/speed);
  ball=ball+velocity*h;
  for(const auto& w:walls())collideSegment(w.a,w.b,.78);
  for(int i=0;i<3;++i){auto p=bumpers()[i];if(length(ball-p)<38){collideCircle(p,29,.95,120);if(bumperGlow[i]<=0&&!tilted){addScore(100);events.push_back(Event::Bumper);bumperGlow[i]=.12;}}}
  for(int i=0;i<3;++i){if(length(ball-lanePositions()[i])<22&&!lanes[i]&&!tilted){lanes[i]=true;addScore(250);events.push_back(Event::Target);}}
  for(int i=0;i<3;++i){auto p=targetPositions()[i];if(length(ball-p)<25){collideCircle(p,16,.8,35);if(!targets[i]&&!tilted){targets[i]=true;targetGlow[i]=.3;addScore(400);events.push_back(Event::Target);}}}
  auto flipper=[&](Vec pivot,Vec tip,double angular){Vec edge=tip-pivot;double t=std::max(0.,std::min(1.,dot(ball-pivot,edge)/dot(edge,edge)));Vec arm=edge*t;collideSegment(pivot,tip,.9,{-angular*arm.y,angular*arm.x},17);};
  if(h>0){flipper({205,785},leftTip(),(leftAngle-la)/h);flipper({395,785},rightTip(),(rightAngle-ra)/h);}
  if(std::all_of(lanes.begin(),lanes.end(),[](bool x){return x;})&&std::all_of(targets.begin(),targets.end(),[](bool x){return x;})){
   addScore(2500);++circuits;multiplier=std::min(5,multiplier+1);lanes.fill(false);targets.fill(false);events.push_back(Event::Mission);message="CIRCUIT COMPLETE. Multiplier increased.";
  }
  if(ball.y>915){drain();break;}
 }
}
bool Model::write(std::ostream& o)const{
 o<<"OMARCHY_PINBALL 1\n"<<std::setprecision(17)<<ball.x<<' '<<ball.y<<' '<<velocity.x<<' '<<velocity.y<<'\n'
 <<waiting<<' '<<gameOver<<' '<<tilted<<' '<<balls<<' '<<score<<' '<<multiplier<<' '<<circuits<<'\n'
 <<leftAngle<<' '<<rightAngle<<' '<<time<<' '<<saveTime<<' '<<nudgeHeat<<'\n';
 for(bool x:lanes)o<<x<<' ';for(bool x:targets)o<<x<<' ';o<<'\n';return bool(o);
}
bool Model::read(std::istream& in){
 Model m;std::string magic;int version;if(!(in>>magic>>version)||magic!="OMARCHY_PINBALL"||version!=1)return false;
 if(!(in>>m.ball.x>>m.ball.y>>m.velocity.x>>m.velocity.y>>m.waiting>>m.gameOver>>m.tilted>>m.balls>>m.score>>m.multiplier>>m.circuits>>m.leftAngle>>m.rightAngle>>m.time>>m.saveTime>>m.nudgeHeat))return false;
 for(auto& x:m.lanes)if(!(in>>x))return false;for(auto& x:m.targets)if(!(in>>x))return false;
 for(double x:{m.ball.x,m.ball.y,m.velocity.x,m.velocity.y,m.leftAngle,m.rightAngle,m.time,m.saveTime,m.nudgeHeat})if(!std::isfinite(x))return false;
 if(m.ball.x<0||m.ball.x>600||m.ball.y<0||m.ball.y>920||length(m.velocity)>1900||m.balls<0||m.balls>3||m.score<0||m.score>999999999||m.multiplier<1||m.multiplier>5||m.circuits<0||m.circuits>1000000||m.time<0||m.saveTime<0||m.saveTime>9||m.nudgeHeat<0||m.nudgeHeat>10||m.leftAngle<-.56||m.leftAngle>.36||m.rightAngle<2.78||m.rightAngle>3.70)return false;
 if(m.gameOver!=(m.balls==0))return false;
 m.paused=!m.gameOver;m.message=m.gameOver?"GAME OVER. Start a new game.":"Saved game. Press P to resume.";*this=m;return true;
}
}
