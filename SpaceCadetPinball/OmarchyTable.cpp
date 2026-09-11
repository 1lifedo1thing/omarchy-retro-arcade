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
#include "../native/BrandLogo.h"

// Authored table data. All distances below are engine world units.
// No original DAT, bitmap, sound or table parameters are embedded here.
namespace OmarchyTable {
bool Enabled=false;
namespace {
DatFile* data;
std::vector<int16_t> objects;
unsigned hits=0;
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
float px(float x){return 200+10*x;}float py(float y){return 208+10*y;}
}
DatFile* Build(){
 data=new DatFile();data->AppName="Omarchy Arcade";data->Description="Circuit table / authored data / upstream physics";objects.clear();
 auto background=group("background");auto bg=bitmap(background,600,416,0,0,false);
 memset(bg->IndexedBmpPtr,10,bg->IndexedStride*bg->Height);
 auto palette=reinterpret_cast<ColorRgba*>(new char[1024]{});palette[10]=ColorRgba(23,27,37,255);palette[11]=ColorRgba(158,206,106,255);palette[12]=ColorRgba(230,237,243,255);palette[13]=ColorRgba(88,111,139,255);palette[14]=ColorRgba(244,184,96,255);
 auto pe=new EntryData(FieldTypes::Palette,(char*)palette);pe->FieldSize=1024;background->AddEntry(pe);
 // Perspective matrix with a flat playfield and exactly 10 pixels per world unit.
 auto camera=group("camera_info");
 floats(camera,{1,0,0,0, 0,1,0,0, 0,0,-1,100, 1000,0,40});
 auto table=group("table");auto board=bitmap(table,390,416,0,0);
 memset(board->IndexedBmpPtr,10,board->IndexedStride*board->Height);
 floats(table,{600,5,-18,-19,18,-19,18,20,-18,20,-18,-19});
 floats(table,{700,200,208});floats(table,{701,.08f});floats(table,{305,25,.5f,1.5707963f});
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
 auto wall=[&](const char* name,float x,float y,float xx,float yy,int kind=1000){
  auto g=group(name);floats(g,{600,2,xx,yy,x,y});shorts(g,{300,(int16_t)mat->GroupId});object(g,kind);
  line(board,px(x),py(y),px(xx),py(yy),13,2);return g;
 };
 // Directed walls: normals point into the playfield.
 wall("left_rail",-17,18,-17,-17);wall("roof_left",-17,-17,-11,-18);wall("roof",-11,-18,11,-18);
 wall("roof_right",11,-18,17,-12);wall("right_rail",17,-12,17,18);
 wall("left_return",-17,9,-5,14);wall("right_return",5,14,12,9);
 wall("shooter_divider",12,16,12,-10);wall("shooter_divider_back",12,-10,12,16);
 auto drain=wall("drain",17,19,-17,19,1007);floats(drain,{407,.8f});
 auto plunger=group("plunger");floats(plunger,{600,2,12,17,17,17});floats(plunger,{601,14.5f,16});object(plunger,1001);
 auto pl=bitmap(plunger,45,12,323,378);line(pl,0,4,44,4,14,2);
 for(int side=0;side<2;side++){
  const float sign=side? -1.f:1.f,origin=-5*sign;
  auto g=group(side?"flipper_r":"flipper_l");shorts(g,{100,9,300,(int16_t)mat->GroupId});
  floats(g,{800,origin,14,.4f});floats(g,{801,-1*sign,16,.25f});floats(g,{802,-1*sign,12,.25f});
  floats(g,{803,1});floats(g,{804,.06f});floats(g,{805,.1f});object(g,side?1004:1003);
  for(int f=0;f<9;f++){auto state=f?group(nullptr,201):g;auto b=bitmap(state,110,90,(int)px(origin)-55,(int)py(14)-45);
   float angle=std::atan2(2.f,4.f)-f*(2*std::atan2(2.f,4.f))/8;
   line(b,55,45,55+sign*std::cos(angle)*std::sqrt(20.f)*10,45+std::sin(angle)*std::sqrt(20.f)*10,11,3);
  }
 }
 const float xs[]={-7,0,7},ys[]={-5,-10,-5};
 for(int i=0;i<3;i++){std::string name="bumper"+std::to_string(i);auto g=group(name.c_str());shorts(g,{100,2,300,(int16_t)mat->GroupId,400,(int16_t)kick->GroupId});floats(g,{600,1,xs[i],ys[i],1.7f});floats(g,{407,.12f});object(g,1005);
  for(int f=0;f<2;f++){auto state=f?group(nullptr,201):g;auto b=bitmap(state,39,39,(int)px(xs[i])-19,(int)py(ys[i])-19);circle(b,19,19,18,13);circle(b,19,19,14,f?14:11);circle(b,19,19,8,10);}
 }
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
void ComponentEvent(MessageCode code,TPinballComponent* c){
 auto t=c->PinballTable;if(!t)return;
 if(dynamic_cast<TBumper*>(c)&&code==MessageCode::ControlCollision&&!t->TiltLockFlag){
  sound();t->AddScore(100);if(++hits%10==0){t->AddScore(1000);pb::MissTextBox->Display("CIRCUIT COMPLETE +1000",3);}
 }
 if(dynamic_cast<TDrain*>(c)&&code==MessageCode::ControlTimerExpired){
  t->ChangeBallCount(t->BallCount-1);
  if(t->BallCount>0){t->Message(MessageCode::ClearTiltLock,0);t->Plunger->Message(MessageCode::PlungerFeedBall,0);}
  else t->Message(MessageCode::GameOver,0);
 }
}
void Shutdown(){if(effect){Mix_HaltChannel(-1);Mix_FreeChunk(effect);effect=nullptr;}tone.clear();}
void TableEvent(MessageCode code){
 if(code==MessageCode::StartGamePlayer1)pb::InfoTextBox->Display("Hold SPACE, release to launch.",-1);
 if(code==MessageCode::NewGame){hits=0;pb::MissTextBox->Display("OMARCHY CIRCUIT\nHit ten bumpers for a circuit bonus.\nA/D flippers  SPACE launch\nP pause  F2 new game",-1);}
}
}
