#include "pch.h"
#include "OmarchyTable.h"
#include "GroupData.h"
#include "gdrv.h"
#include "zdrv.h"
#include "pb.h"
#include "TPinballTable.h"
#include "TPlunger.h"
#include "TDrain.h"
#include "TBumper.h"
#include "TTextBox.h"
#include "options.h"
#include "TTripwire.h"
#include "TRamp.h"
#include <map>
#include <algorithm>

// Authored table data. All distances below are engine world units.
// No original DAT, bitmap, sound or table parameters are embedded here.
namespace OmarchyTable {
bool Enabled=false;
namespace {
DatFile* data;
std::vector<int16_t> objects;
unsigned hits=0;
unsigned targetMask=0, orbitCount=0, rampCount=0, circuits=0;
std::map<std::string,float> flashes, debounce;
std::string notice="HOLD SPACE TO LAUNCH";
float noticeUntil=0;
bool over=false;
void announce(const char* text){notice=text;noticeUntil=pb::time_now+3;}
std::vector<int16_t> tone;
Mix_Chunk* effect=nullptr;
void sound(){
 if(!options::Options.Sounds)return;
 int rate,channels;Uint16 format;if(!Mix_QuerySpec(&rate,&format,&channels)||format!=AUDIO_S16SYS)return;
 if(!effect){tone.resize(rate*channels/12);for(size_t i=0;i<tone.size();i++){double t=double(i/channels)/rate;tone[i]=(int16_t)(2500*sin(t*2*3.141592653589793*660)*exp(-t*40));}effect=Mix_QuickLoad_RAW((Uint8*)tone.data(),(Uint32)(tone.size()*2));}
 if(effect){Mix_VolumeChunk(effect,options::Options.SoundVolume);Mix_PlayChannel(-1,effect,0);}
}
GroupData* group(const char* name,int type=200){
 auto g=new GroupData((int)data->Groups.size());data->Groups.push_back(g);
 auto add=[&](FieldTypes t,const void* src,int n){auto e=new EntryData();e->EntryType=t;e->FieldSize=n;e->Buffer=new char[n];memcpy(e->Buffer,src,n);g->AddEntry(e);};
 int16_t v=type;add(FieldTypes::ShortValue,&v,2);
 if(name)add(FieldTypes::GroupName,name,(int)strlen(name)+1);
 return g;
}
template<class T> void values(GroupData* g,FieldTypes type,std::initializer_list<T> a){
 auto e=new EntryData();e->EntryType=type;e->FieldSize=(int)(a.size()*sizeof(T));e->Buffer=new char[e->FieldSize];memcpy(e->Buffer,a.begin(),e->FieldSize);g->AddEntry(e);
}
void floats(GroupData* g,std::initializer_list<float> a){values(g,FieldTypes::FloatArray,a);}
void shorts(GroupData* g,std::initializer_list<int16_t> a){values(g,FieldTypes::ShortArray,a);}
void object(GroupData* g,int type){objects.push_back(type);objects.push_back(g->GroupId);}
gdrv_bitmap8* bitmap(GroupData* g,int w,int h,int x,int y,bool depth=true){
 auto b=new gdrv_bitmap8(w,h,true);b->XPosition=x;b->YPosition=y;
 memset(b->IndexedBmpPtr,0,b->IndexedStride*h);
 auto e=new EntryData(FieldTypes::Bitmap8bit,(char*)b);g->AddEntry(e);
 if(depth){auto z=new zmap_header_type(w,h,w);std::fill(z->ZPtr1,z->ZPtr1+w*h,5000);g->AddEntry(new EntryData(FieldTypes::Bitmap16bit,(char*)z));}
 return b;
}
void pixel(gdrv_bitmap8* b,int x,int y,int c){if(x>=0&&y>=0&&x<b->Width&&y<b->Height)b->IndexedBmpPtr[(b->Height-1-y)*b->IndexedStride+x]=(char)c;}
void line(gdrv_bitmap8* b,float x,float y,float xx,float yy,int c,int width=2){
 int n=std::max(1,(int)(std::hypot(xx-x,yy-y)*2));
 for(int i=0;i<=n;++i)for(int dy=-width;dy<=width;++dy)for(int dx=-width;dx<=width;++dx)pixel(b,(int)(x+(xx-x)*i/n)+dx,(int)(y+(yy-y)*i/n)+dy,c);
}
void circle(gdrv_bitmap8* b,int cx,int cy,int r,int c){for(int y=-r;y<=r;y++)for(int x=-r;x<=r;x++)if(x*x+y*y<=r*r)pixel(b,cx+x,cy+y,c);}
float wx(float x){return (x-540)/25;} float wy(float y){return (y-500)/25;}

}
DatFile* Build(){
 hits=targetMask=orbitCount=rampCount=circuits=0;over=false;flashes.clear();debounce.clear();data=new DatFile();data->AppName="Omarchy Arcade";data->Description="Circuit table / authored data / upstream physics";objects.clear();
 auto background=group("background");auto bg=bitmap(background,600,416,0,0,false);
 memset(bg->IndexedBmpPtr,10,bg->IndexedStride*bg->Height);
 auto palette=reinterpret_cast<ColorRgba*>(new char[1024]{});palette[10]=ColorRgba(23,27,37,255);palette[11]=ColorRgba(158,206,106,255);palette[12]=ColorRgba(230,237,243,255);palette[13]=ColorRgba(88,111,139,255);palette[14]=ColorRgba(244,184,96,255);
 auto pe=new EntryData(FieldTypes::Palette,(char*)palette);pe->FieldSize=1024;background->AddEntry(pe);
 // Perspective matrix with a flat playfield and exactly 10 pixels per world unit.
 auto camera=group("camera_info");
 floats(camera,{1,0,0,0, 0,1,0,0, 0,0,-1,100, 1000,0,40});
 auto table=group("table");auto board=bitmap(table,390,416,0,0);
 memset(board->IndexedBmpPtr,10,board->IndexedStride*board->Height);
 floats(table,{600,5,-22,-21,20,-21,20,23,-22,23,-22,-21});
 floats(table,{700,200,208});floats(table,{701,.08f});floats(table,{305,25,.32f,1.5707963f});
 auto ball=group("ball");floats(ball,{500,.45f});floats(ball,{501,0,0,.45f});auto ballbmp=bitmap(ball,11,11,0,0,false);circle(ballbmp,5,5,5,12);circle(ballbmp,3,3,1,14);
 auto digits=(int)data->Groups.size();
 const int masks[]={63,6,91,79,102,109,125,7,127,111};
 for(int d=0;d<10;d++){auto g=group(nullptr);auto b=bitmap(g,12,20,0,0,false);int m=masks[d];
  if(m&1)line(b,3,2,8,2,12,1);if(m&2)line(b,9,3,9,8,12,1);if(m&4)line(b,9,11,9,16,12,1);if(m&8)line(b,3,18,8,18,12,1);if(m&16)line(b,2,11,2,16,12,1);if(m&32)line(b,2,3,2,8,12,1);if(m&64)line(b,3,10,8,10,12,1);
 }
 auto score=group("score1");shorts(score,{(int16_t)digits,415,70,165,20});
 auto count=group("ballcount1");shorts(count,{(int16_t)digits,415,110,30,20});
 auto player=group("player_number1");shorts(player,{(int16_t)digits,550,110,25,20});
 auto info=group("info_text_box");shorts(info,{1500,405,155,185,100,0,0,0,0});object(info,1033);
 auto mission=group("mission_text_box");shorts(mission,{1500,405,270,185,130,0,0,0,0});object(mission,1033);
 auto mat=group(nullptr,300);floats(mat,{301,.95f,302,.8f});
 auto kick=group(nullptr,400);floats(kick,{401,1,402,28});
 // The collision coordinates below are measured against assets/circuit/table.png.
 // The custom view uses the same 25 pixels/world-unit mapping.
 auto rail=[&](const char* name,float x,float y,float xx,float yy,int mask=0,int kicker=-1){
  auto g=group(name);floats(g,{600,2,wx(xx),wy(yy),wx(x),wy(y)});
  if(kicker>=0)shorts(g,{300,(int16_t)mat->GroupId,400,(int16_t)kicker,602,(int16_t)mask});
  else shorts(g,{300,(int16_t)mat->GroupId,602,(int16_t)mask});object(g,1000);return g;
 };
 auto path=[&](const char* name,std::initializer_list<vector2> pts,int mask=0){
  auto prev=pts.begin();for(auto it=prev+1;it!=pts.end();++it){rail(name,prev->X,prev->Y,it->X,it->Y,mask);prev=it;}
 };
 // Clockwise outer boundary; roof deflects a launched ball into the bumper field.
 path("outer",{{95,975},{115,770},{155,530},{205,310},{245,145},{290,78},{370,48},{550,44},{700,65},{785,110},{845,190},{885,305},{920,500},{970,975}});
 path("shooter_inner",{{892,870},{858,480},{850,395}});
 path("shooter_back",{{850,395},{858,480},{892,870}});
 // Outlanes and returns leave deliberate drains, including the central gap.
 path("left_inlane",{{220,530},{205,715},{220,752},{340,850}});
 path("left_inlane_back",{{340,850},{220,752},{205,715},{220,530}});
 path("right_inlane",{{720,850},{822,770},{825,680}});
 path("right_inlane_back",{{825,680},{822,770},{720,850}});
 path("left_apron",{{290,900},{400,990},{485,1020}});
 path("right_apron",{{595,1020},{740,930},{765,895}});
 auto slingKick=group(nullptr,400);floats(slingKick,{401,1,402,20});
 rail("sling_left",360,778,295,635,0,slingKick->GroupId);
 rail("sling_right",755,635,690,778,0,slingKick->GroupId);
 // Stand-up targets use the upstream wall/kicker response.
 for(int i=0;i<4;i++){
  std::string n="target"+std::to_string(i);
  rail(n.c_str(),493+i*31,142,515+i*31,142,0,kick->GroupId);
 }
 for(int i=0;i<4;i++){
  std::string n="module"+std::to_string(i);
  rail(n.c_str(),736-i*8,327+i*25,750-i*8,307+i*25,0,kick->GroupId);
 }
 auto drain=group("drain");floats(drain,{600,2,wx(60),wy(1030),wx(1000),wy(1030)});floats(drain,{407,.8f});shorts(drain,{602,0,602,1});object(drain,1007);
 auto plunger=group("plunger");floats(plunger,{600,2,wx(892),wy(878),wx(949),wy(878)});floats(plunger,{601,wx(918),wy(850)});object(plunger,1001);
 bitmap(plunger,45,12,0,0);
 for(int side=0;side<2;side++){
  float origin=side?720:340,tip=side?598:462;
  auto g=group(side?"flipper_r":"flipper_l");shorts(g,{100,9,300,(int16_t)mat->GroupId});
  floats(g,{800,wx(origin),wy(888),.7f});floats(g,{801,wx(tip),wy(943),.42f});floats(g,{802,wx(tip),wy(820),.42f});
  floats(g,{803,1});floats(g,{804,.055f});floats(g,{805,.095f});object(g,side?1004:1003);
  for(int f=0;f<9;f++){auto state=f?group(nullptr,201):g;bitmap(state,1,1,0,0);}
 }
 const float xs[]={470,617,563,202},ys[]={226,202,287,431},rs[]={41,40,40,30};
 for(int i=0;i<4;i++){std::string name="bumper"+std::to_string(i);auto g=group(name.c_str());shorts(g,{100,2,300,(int16_t)mat->GroupId,400,(int16_t)kick->GroupId});floats(g,{600,1,wx(xs[i]),wy(ys[i]),rs[i]/25});floats(g,{407,.12f});object(g,1005);
  for(int f=0;f<2;f++){auto state=f?group(nullptr,201):g;bitmap(state,1,1,0,0);}
 }
 auto sensor=[&](const char* name,float x,float y,float xx,float yy,int mask=0){
  auto g=group(name);floats(g,{600,2,wx(x),wy(y),wx(xx),wy(yy)});shorts(g,{602,(int16_t)mask});object(g,1024);
 };
 sensor("orbit",770,110,840,150);sensor("orbit_back",840,150,770,110);
 sensor("return",220,735,265,765);sensor("return_back",265,765,220,735);
 // Elevated ramp: a triangulated upstream TRamp surface follows the artwork.
 // Ground and ramp rails use separate collision masks; TRamp switches them at its portals.
 std::vector<vector2> centres={{372,405},{370,330},{345,282},{292,247},{269,210},{269,163},{291,115},{327,77},{375,61},{404,79},{416,107}};
 std::vector<vector2> left,right;
 for(size_t i=0;i<centres.size();++i){auto a=centres[i?i-1:i],b=centres[i+1<centres.size()?i+1:i];float dx=b.X-a.X,dy=b.Y-a.Y,len=std::hypot(dx,dy);left.push_back({centres[i].X-dy/len*22,centres[i].Y+dx/len*22});right.push_back({centres[i].X+dy/len*22,centres[i].Y-dx/len*22});}
 auto ramp=group("ramp");shorts(ramp,{602,1});floats(ramp,{701,.04f});floats(ramp,{1305,1});
 std::vector<float> planes={1300,float((centres.size()-1)*2)};
 auto triangle=[&](vector2 a,vector2 b,vector2 c){
  // Rising deck z = (405 - image_y) * .002, continuous at the entry.
  planes.insert(planes.end(),{0,-.05f,-.19f,wx(a.X),wy(a.Y),wx(b.X),wy(b.Y),wx(c.X),wy(c.Y),.12f,1.5707963f,0,0});
 };
 for(size_t i=0;i+1<left.size();++i){triangle(left[i],right[i],right[i+1]);triangle(left[i],right[i+1],left[i+1]);
  rail("ramp_rail_l",left[i+1].X,left[i+1].Y,left[i].X,left[i].Y,1);
  rail("ramp_guard_l",left[i].X,left[i].Y,left[i+1].X,left[i+1].Y,0);
  rail("ramp_rail_r",right[i].X,right[i].Y,right[i+1].X,right[i+1].Y,1);
  rail("ramp_guard_r",right[i+1].X,right[i+1].Y,right[i].X,right[i].Y,0);
 }
 auto pe2=new EntryData();pe2->EntryType=FieldTypes::FloatArray;pe2->FieldSize=planes.size()*sizeof(float);pe2->Buffer=new char[pe2->FieldSize];memcpy(pe2->Buffer,planes.data(),pe2->FieldSize);ramp->AddEntry(pe2);
 floats(ramp,{1301,0,1,0,wx(left.front().X),wy(left.front().Y),wx(right.front().X),wy(right.front().Y),0});
 floats(ramp,{1302,0,1,0,wx(right.back().X),wy(right.back().Y),wx(left.back().X),wy(left.back().Y),0});
 floats(ramp,{1303,1,0,wx(left[5].X),wy(left[5].Y),wx(right[5].X),wy(right[5].Y)});object(ramp,1021);
 sensor("ramp_score",335,47,335,96,1);
 sensor("ramp_score_back",335,96,335,47,1);
 sensor("ramp_exit",left.back().X,left.back().Y,right.back().X,right.back().Y,1);
 auto tableObjects=group("table_objects");
 auto e=new EntryData();e->EntryType=FieldTypes::ShortArray;e->FieldSize=(objects.size()+1)*2;e->Buffer=new char[e->FieldSize];((int16_t*)e->Buffer)[0]=1025;memcpy(e->Buffer+2,objects.data(),objects.size()*2);tableObjects->AddEntry(e);
 // Finalize only authored groups; do not import the embedded original bitmap font.
 for(auto g:data->Groups){
  g->FinalizeGroup();
  auto b=g->GetBitmap(0);auto z=g->GetZMap(0);
  if(b&&z)for(int y=0;y<b->Height;y++)for(int x=0;x<b->Width;x++)if(!b->IndexedBmpPtr[(b->Height-1-y)*b->IndexedStride+x])z->ZPtr1[y*z->Stride+x]=65535;
 }
 return data;
}
unsigned Progress(){return hits%12;}
unsigned Targets(){return targetMask;}
unsigned Orbits(){return orbitCount;}
unsigned Ramps(){return rampCount;}
unsigned Circuits(){return circuits;}
bool GameOver(){return over;}
const char* Status(){return pb::time_now<noticeUntil||over?notice.c_str():"LIGHT THE CIRCUIT";}
float Flash(const char* name){auto i=flashes.find(name);return i==flashes.end()?0:std::max(0.f,1-(pb::time_now-i->second)/.25f);}
void ComponentEvent(MessageCode code,TPinballComponent* c){
 auto t=c->PinballTable;if(!t)return;
 if(code==MessageCode::ControlCollision&&!t->TiltLockFlag&&c->GroupName){
  std::string name=c->GroupName;
  if(name=="drain"){announce("BALL DRAINED");return;}
  if(debounce.count(name)&&pb::time_now-debounce[name]<.15f)return;
  debounce[name]=pb::time_now;flashes[name]=pb::time_now;
  if(dynamic_cast<TBumper*>(c)){sound();t->AddScore(100);if(++hits%12==0){++circuits;t->AddScore(2500);announce("CIRCUIT +2500");}}
  else if(name.find("target")==0||name.find("module")==0){unsigned bit=unsigned(name.back()-'0')+(name[0]=='m'?4:0);targetMask|=1u<<bit;t->AddScore(250);sound();announce("MODULE +250");if(targetMask==255){t->AddScore(5000);targetMask=0;announce("SYSTEM ONLINE +5000");}}
  else if(name=="orbit"||name=="orbit_back"){if(!debounce.count("orbit_award")||pb::time_now-debounce["orbit_award"]>2){debounce["orbit_award"]=pb::time_now;++orbitCount;t->AddScore(1000);announce("ORBIT +1000");sound();}}
  else if(name=="ramp_score"||name=="ramp_score_back"){if(!debounce.count("ramp_award")||pb::time_now-debounce["ramp_award"]>3){debounce["ramp_award"]=pb::time_now;++rampCount;t->AddScore(1500);announce("RAMP +1500");sound();}}
  else if(name.find("sling_")==0){t->AddScore(25);sound();}
 }
 if(dynamic_cast<TDrain*>(c)&&code==MessageCode::ControlTimerExpired){
  t->ChangeBallCount(t->BallCount-1);
  if(t->BallCount>0){t->Message(MessageCode::ClearTiltLock,0);t->Plunger->Message(MessageCode::PlungerFeedBall,0);announce("HOLD SPACE TO LAUNCH");}
  else {over=true;notice="GAME OVER  F2 NEW";t->Message(MessageCode::GameOver,0);}
 }
}
void Shutdown(){if(effect){Mix_HaltChannel(-1);Mix_FreeChunk(effect);effect=nullptr;}tone.clear();}
void TableEvent(MessageCode code){
 if(code==MessageCode::StartGamePlayer1)announce("HOLD SPACE TO LAUNCH");
 if(code==MessageCode::NewGame){pb::MainTable->Plunger->PullbackDelay=.10f;hits=targetMask=orbitCount=rampCount=circuits=0;over=false;debounce.clear();flashes.clear();announce("HOLD SPACE TO LAUNCH");}
}
}
