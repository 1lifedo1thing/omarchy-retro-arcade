#include <SDL.h>
#include "imgui.h"
#include "imgui_impl_sdl.h"
#include "imgui_impl_sdlrenderer.h"
#include "PinballModel.h"
#include "ThemePalette.h"
#include "BrandLogo.h"
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <array>
#include <cstring>

namespace {
constexpr double pi=3.141592653589793;
struct Voice{double hz,remaining,phase=0,volume=.2;};
struct Audio {
 SDL_AudioDeviceID device=0;std::vector<Voice> voices;bool muted=false,music=false;double songTime=0;int note=0;
 Audio(){SDL_AudioSpec wanted{},got{};wanted.freq=48000;wanted.format=AUDIO_F32SYS;wanted.channels=1;wanted.samples=1024;
  device=SDL_OpenAudioDevice(nullptr,0,&wanted,&got,0);if(device)SDL_PauseAudioDevice(device,0);}
 ~Audio(){if(device)SDL_CloseAudioDevice(device);}
 void tone(double hz,double seconds=.12,double volume=.16){if(!muted&&voices.size()<24)voices.push_back({hz,seconds,0,volume});}
 void event(oma::Event e){switch(e){case oma::Event::Bumper:tone(230,.09);tone(460,.05,.08);break;case oma::Event::Target:tone(660,.18);break;case oma::Event::Launch:tone(120,.24);break;case oma::Event::Mission:tone(440,.5);tone(554.37,.5,.12);tone(659.25,.5,.12);break;case oma::Event::Drain:tone(110,.45);break;case oma::Event::Save:tone(880,.25);break;case oma::Event::Tilt:tone(65,.5);break;default:tone(180,.035,.07);}}
 void pump(){if(!device)return;if(muted){voices.clear();SDL_ClearQueuedAudio(device);return;}
  if(SDL_GetQueuedAudioSize(device)>4096)return;
  float samples[2048];const double notes[]={220,0,329.63,0,293.66,0,261.63,0,220,0,392,0,329.63,0,293.66,0};
  for(auto& out:samples){songTime+=1./48000;if(music&&songTime>=.32){songTime=0;double n=notes[note++%16];if(n)tone(n,.20,.035);}
   double sum=0;for(auto& v:voices){v.phase+=2*pi*v.hz/48000;v.remaining-=1./48000;double envelope=std::min(1.,std::max(0.,v.remaining)*30);sum+=std::sin(v.phase)*envelope*v.volume;}
   out=static_cast<float>(std::max(-.8,std::min(.8,sum)));
   voices.erase(std::remove_if(voices.begin(),voices.end(),[](const Voice& v){return v.remaining<=0;}),voices.end());}
  SDL_QueueAudio(device,samples,sizeof(samples));
 }
};
struct Preferences {int appearance=0,best=0;bool muted=false,music=false;};
std::string readSmall(const std::string& path){std::ifstream file(path,std::ios::binary);char bytes[65536];file.read(bytes,sizeof bytes);return std::string(bytes,static_cast<size_t>(file.gcount()));}
bool atomicWrite(const std::string& path,const std::string& text){std::string tmp=path+".tmp";std::ofstream file(tmp,std::ios::trunc);if(!(file<<text))return false;file.close();if(!file)return false;return std::rename(tmp.c_str(),path.c_str())==0;}
ImU32 col(uint32_t rgb,int alpha=255){return IM_COL32((rgb>>16)&255,(rgb>>8)&255,rgb&255,alpha);}
ImVec4 vec(uint32_t rgb,float a=1){return {float((rgb>>16)&255)/255,float((rgb>>8)&255)/255,float(rgb&255)/255,a};}
struct Theme {
 cadet::Palette p;std::string path,previous;Uint32 last=0;int selected=-1;bool found=false;
 Theme(){path=cadet::themePath();}
 void update(int mode){Uint32 now=SDL_GetTicks();if(selected==mode&&now-last<1500)return;last=now;
  path=cadet::themePath();std::string text=mode==0?readSmall(path):"";if(mode==selected&&text==previous)return;previous=text;selected=mode;p=cadet::Palette{};found=!text.empty();
  if(mode==0){std::istringstream in(text);p=cadet::readPalette(in,p);}if(mode==2){p.background=0x211b14;p.foreground=0xffecd2;p.accent=0xf4b860;p.warm=0xf18455;}
  ImGui::StyleColorsDark();auto& s=ImGui::GetStyle();s.WindowRounding=12;s.FrameRounding=6;s.GrabRounding=6;s.WindowPadding={18,18};s.FramePadding={12,9};s.ItemSpacing={10,12};
  s.Colors[ImGuiCol_Text]=vec(p.foreground);s.Colors[ImGuiCol_TextDisabled]=vec(p.foreground,.60f);s.Colors[ImGuiCol_WindowBg]=vec(p.background);s.Colors[ImGuiCol_PopupBg]=vec(p.background);
  for(int id:{ImGuiCol_Button,ImGuiCol_FrameBg,ImGuiCol_Header})s.Colors[id]=vec(p.accent,.22f);
  for(int id:{ImGuiCol_ButtonHovered,ImGuiCol_FrameBgHovered,ImGuiCol_HeaderHovered})s.Colors[id]=vec(p.accent,.42f);
  for(int id:{ImGuiCol_ButtonActive,ImGuiCol_FrameBgActive,ImGuiCol_HeaderActive,ImGuiCol_CheckMark,ImGuiCol_SliderGrab})s.Colors[id]=vec(p.accent,.7f);
  s.Colors[ImGuiCol_PlotHistogram]=vec(p.accent);s.Colors[ImGuiCol_Border]=vec(p.accent,.3f);s.Colors[ImGuiCol_NavHighlight]=vec(p.accent);
 }
};
void drawTable(oma::Model& m,const cadet::Palette& p,SDL_Texture* logo,float x,float y,float scale,ImFont* font){
 auto d=ImGui::GetBackgroundDrawList();auto pos=[=](oma::Vec v){return ImVec2{x+float(v.x)*scale,y+float(v.y)*scale};};
 auto line=[&](oma::Vec a,oma::Vec b,ImU32 c,float width){d->AddLine(pos(a),pos(b),c,width*scale);};
 auto circle=[&](oma::Vec a,float radius,ImU32 c,bool filled=true){if(filled)d->AddCircleFilled(pos(a),radius*scale,c,32);else d->AddCircle(pos(a),radius*scale,c,32,2*scale);};
 auto text=[&](oma::Vec a,const char* t,float size,ImU32 c){d->AddText(font,size*scale,pos(a),c,t);};
 d->AddRectFilled(pos({0,0}),pos({600,930}),col(p.background),22*scale);
 d->AddRect(pos({1,1}),pos({599,929}),col(p.accent,100),22*scale,0,2*scale);
 for(int i=0;i<46;++i){double sx=45+(i*137)%480,sy=85+(i*83)%740;circle({sx,sy},1,col(p.foreground,25));}
 // Original geometry and decoration; no original Space Cadet art or data.
 for(const auto& w:oma::Model::walls()){line(w.a,w.b,col(p.accent,25),13);line(w.a,w.b,col(p.foreground,195),4);}
 d->AddRectFilled(pos({248,199}),pos({342,293}),IM_COL32(23,27,37,255),5*scale);
 d->AddImage((ImTextureID)logo,pos({254,205}),pos({336,287}));
 text({214,78},"S P A C E C A D E T",14,col(p.foreground,150));
 for(int i=0;i<3;++i){auto a=oma::Model::lanePositions()[i];circle(a,23,col(p.accent,m.lanes[i]?220:30));circle(a,23,col(p.accent,160),false);char c[2]={char("OMA"[i]),0};text(a+oma::Vec{-6,-10},c,18,col(m.lanes[i]?p.background:p.foreground));}
 for(int i=0;i<3;++i){auto a=oma::Model::bumpers()[i];circle(a+oma::Vec{0,7},34,IM_COL32(0,0,0,95));circle(a,36,col(p.accent,m.bumperGlow[i]>0?100:25));circle(a,29,col(p.accent));circle(a,23,col(p.background));circle(a,18,col(p.foreground,180),false);circle(a,6,col(p.warm));}
 for(int i=0;i<3;++i){auto a=oma::Model::targetPositions()[i];circle(a,21,col(p.warm,m.targets[i]?220:35));circle(a,16,col(p.warm),false);text(a+oma::Vec{-5,-8},"+",17,col(p.foreground));}
 text({230,592},"CIRCUIT / 01",16,col(p.foreground,160));
 text({213,622},"BUILD. LAUNCH. REPEAT.",12,col(p.foreground,110));
 line({142,645},{182,712},col(p.warm),5);line({458,645},{418,712},col(p.warm),5);
 line({205,785},m.leftTip(),col(p.accent,35),23);line({395,785},m.rightTip(),col(p.accent,35),23);
 line({205,785},m.leftTip(),col(m.tilted?p.warm:p.foreground),15);line({395,785},m.rightTip(),col(m.tilted?p.warm:p.foreground),15);
 circle({205,785},9,col(p.accent));circle({395,785},9,col(p.accent));
 for(int i=0;i<8;++i)line({542,850.+i*5},{568,850.+i*5},col(p.accent,65),2);
 d->AddRectFilled(pos({544,897-50*m.charge}),pos({566,901}),col(p.warm),3*scale);
 text({225,873},"OMARCHY PINBALL",13,col(p.foreground,100));
 if(!m.gameOver){circle(m.ball+oma::Vec{3,4},10,IM_COL32(0,0,0,125));circle(m.ball,9,IM_COL32(217,228,240,255));circle(m.ball+oma::Vec{-3,-3},3,IM_COL32(255,255,255,255));}
 if(m.paused||m.gameOver){d->AddRectFilled(pos({60,400}),pos({520,520}),col(p.background,242),12*scale);text({m.gameOver?165.:207.,426},m.gameOver?"GAME OVER":"PAUSED",32,col(p.foreground));text({137,479},m.gameOver?"Start a new game in the side panel.":"Press P or select Resume to continue.",15,col(p.foreground,180));}
}
}

int main(int argc,char** argv){
 int smoke=0,initialWidth=980,initialHeight=960;std::string screenshot;for(int i=1;i<argc;++i){if(std::strcmp(argv[i],"--version")==0){std::puts("omarchy-spacecadet 0.2.0-dev / original table");return 0;}
  if(std::strcmp(argv[i],"--smoke")==0&&i+1<argc)smoke=std::max(1,std::atoi(argv[++i]));else if(std::strcmp(argv[i],"--screenshot")==0&&i+1<argc)screenshot=argv[++i];
  else if(std::strcmp(argv[i],"--size")==0&&i+1<argc){int w=0,h=0;if(std::sscanf(argv[++i],"%dx%d",&w,&h)==2){initialWidth=std::max(780,std::min(3840,w));initialHeight=std::max(660,std::min(2160,h));}}
  else if(std::strcmp(argv[i],"--help")==0){std::puts("Omarchy Space Cadet\nA/D or left/right Shift: flippers. SPACE: charge/release plunger. P: pause. N: nudge. F11: fullscreen.\nNo external game files required. Use the launcher --classic option for the upstream table.");return 0;}}
 SDL_SetHint("SDL_VIDEO_X11_WMCLASS","omarchy-spacecadet");SDL_SetHint("SDL_APP_ID","omarchy-spacecadet");SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY,"1");
 if(SDL_Init(SDL_INIT_VIDEO|SDL_INIT_AUDIO|SDL_INIT_GAMECONTROLLER|SDL_INIT_TIMER)!=0){std::fprintf(stderr,"SDL initialization failed: %s\n",SDL_GetError());return 1;}
 auto window=SDL_CreateWindow("Omarchy Space Cadet",SDL_WINDOWPOS_CENTERED,SDL_WINDOWPOS_CENTERED,initialWidth,initialHeight,SDL_WINDOW_RESIZABLE|SDL_WINDOW_ALLOW_HIGHDPI);
 if(!window){SDL_Quit();return 1;}SDL_SetWindowMinimumSize(window,780,660);
 auto renderer=SDL_CreateRenderer(window,-1,SDL_RENDERER_ACCELERATED|SDL_RENDERER_PRESENTVSYNC);if(!renderer)renderer=SDL_CreateRenderer(window,-1,SDL_RENDERER_SOFTWARE);if(!renderer){SDL_DestroyWindow(window);SDL_Quit();return 1;}
 SDL_SetRenderDrawBlendMode(renderer,SDL_BLENDMODE_BLEND);
 auto brand=SDL_CreateRGBSurfaceWithFormatFrom((void*)BrandLogo,128,128,32,128*4,SDL_PIXELFORMAT_RGBA32);SDL_SetWindowIcon(window,brand);auto logo=SDL_CreateTextureFromSurface(renderer,brand);SDL_FreeSurface(brand);
 IMGUI_CHECKVERSION();ImGui::CreateContext();ImGui::GetIO().ConfigFlags|=ImGuiConfigFlags_NavEnableKeyboard;ImGui::GetIO().IniFilename=nullptr;
 ImFontConfig fc;fc.SizePixels=18;auto font=ImGui::GetIO().Fonts->AddFontDefault(&fc);
 ImGui_ImplSDL2_InitForSDLRenderer(window,renderer);ImGui_ImplSDLRenderer_Init(renderer);
 char* pref=SDL_GetPrefPath("","omarchy-spacecadet");std::string folder=pref?pref:"";SDL_free(pref);
 Preferences prefs;{std::ifstream in(folder+"native-preferences");int mode,best,mute,music;if(in>>mode>>best>>mute>>music&&mode>=0&&mode<=2&&best>=0&&best<=999999999&&(mute==0||mute==1)&&(music==0||music==1))prefs={mode,best,bool(mute),bool(music)};}
 oma::Model model;bool restored=false;if(!smoke){std::ifstream saved(folder+"native-game");if(saved)restored=model.read(saved);}
 Theme theme;theme.update(prefs.appearance);Audio audio;audio.muted=prefs.muted;audio.music=prefs.music;
 SDL_GameController* controller=nullptr;for(int i=0;i<SDL_NumJoysticks();++i)if(SDL_IsGameController(i)){controller=SDL_GameControllerOpen(i);if(controller)break;}
 bool running=true,confirm=false,dialog=false,full=false,saveError=false;int frames=0;double accumulator=0;Uint64 last=SDL_GetPerformanceCounter();Uint32 saveAt=SDL_GetTicks();oma::Input prior;
 auto save=[&](){if(smoke||folder.empty())return;std::ostringstream game;model.write(game);std::ostringstream config;config<<prefs.appearance<<' '<<prefs.best<<' '<<prefs.muted<<' '<<prefs.music<<'\n';saveError=!(atomicWrite(folder+"native-game",game.str())&&atomicWrite(folder+"native-preferences",config.str()));};
 while(running){SDL_Event e;while(SDL_PollEvent(&e)){ImGui_ImplSDL2_ProcessEvent(&e);if(e.type==SDL_QUIT)running=false;
   if(e.type==SDL_WINDOWEVENT&&e.window.event==SDL_WINDOWEVENT_FOCUS_LOST&&!smoke)model.paused=true;
   if(e.type==SDL_KEYDOWN&&!e.key.repeat){auto k=e.key.keysym.sym;if(k==SDLK_p&&!dialog)model.paused=!model.paused;
    if(k==SDLK_n&&!dialog){model.nudge();audio.event(model.tilted?oma::Event::Tilt:oma::Event::Flipper);}if(k==SDLK_F2){confirm=true;dialog=true;model.paused=true;}if(k==SDLK_ESCAPE){model.paused=true;}
    if(k==SDLK_F11){full=!full;SDL_SetWindowFullscreen(window,full?SDL_WINDOW_FULLSCREEN_DESKTOP:0);}}
   if(e.type==SDL_CONTROLLERDEVICEADDED&&!controller)controller=SDL_GameControllerOpen(e.cdevice.which);
   if(e.type==SDL_CONTROLLERDEVICEREMOVED&&controller&&SDL_JoystickInstanceID(SDL_GameControllerGetJoystick(controller))==e.cdevice.which){SDL_GameControllerClose(controller);controller=nullptr;model.paused=true;}
   if(e.type==SDL_CONTROLLERBUTTONDOWN&&e.cbutton.button==SDL_CONTROLLER_BUTTON_START&&!dialog)model.paused=!model.paused;
  }
  if(!running)break;
  const Uint8* keys=SDL_GetKeyboardState(nullptr);oma::Input input;input.left=keys[SDL_SCANCODE_A]||keys[SDL_SCANCODE_LSHIFT]||keys[SDL_SCANCODE_LEFT];input.right=keys[SDL_SCANCODE_D]||keys[SDL_SCANCODE_RSHIFT]||keys[SDL_SCANCODE_RIGHT];input.plunger=keys[SDL_SCANCODE_SPACE];
  if(controller){input.left|=SDL_GameControllerGetButton(controller,SDL_CONTROLLER_BUTTON_LEFTSHOULDER);input.right|=SDL_GameControllerGetButton(controller,SDL_CONTROLLER_BUTTON_RIGHTSHOULDER);input.plunger|=SDL_GameControllerGetButton(controller,SDL_CONTROLLER_BUTTON_A);}
  if(dialog)input={};if(smoke){model.paused=false;input.plunger=model.waiting&&frames%180<120;input.left=frames%37<18;input.right=frames%43<22;}
  if((input.left&&!prior.left)||(input.right&&!prior.right))audio.event(oma::Event::Flipper);prior=input;
  Uint64 now=SDL_GetPerformanceCounter();double elapsed=smoke?1./60:std::min(.10,double(now-last)/SDL_GetPerformanceFrequency());last=now;accumulator+=elapsed;
  while(accumulator>=1./120){model.step(1./120,input);for(auto event:model.events)audio.event(event);accumulator-=1./120;}
  prefs.best=std::max(prefs.best,model.score);audio.muted=prefs.muted;audio.music=prefs.music;audio.pump();theme.update(prefs.appearance);
  ImGui_ImplSDLRenderer_NewFrame();ImGui_ImplSDL2_NewFrame();ImGui::NewFrame();
  int width,height;SDL_GetWindowSize(window,&width,&height);float boardScale=std::min((height-124.f)/930.f,(width-330.f)/600.f);float boardX=22,boardY=100,sideX=boardX+600*boardScale+22;
  auto draw=ImGui::GetBackgroundDrawList();draw->AddRectFilled({20,20},{80,80},IM_COL32(23,27,37,255),5);draw->AddImage((ImTextureID)logo,{24,24},{76,76});draw->AddText(font,31,{92,24},col(theme.p.foreground),"Space Cadet");draw->AddText(font,15,{94,63},col(theme.p.foreground,150),"OMARCHY  /  ORIGINAL TABLE");
  drawTable(model,theme.p,logo,boardX,boardY,boardScale,font);
  ImGui::SetNextWindowPos({sideX,100});ImGui::SetNextWindowSize({width-sideX-22.f,height-122.f});
  ImGui::Begin("Game",nullptr,ImGuiWindowFlags_NoTitleBar|ImGuiWindowFlags_NoMove|ImGuiWindowFlags_NoResize|ImGuiWindowFlags_NoSavedSettings);
  ImGui::TextDisabled("SCORE");ImGui::SetWindowFontScale(2.1f);ImGui::Text("%d",model.score);ImGui::SetWindowFontScale(1);ImGui::TextDisabled("BEST  %d",prefs.best);ImGui::Separator();
  ImGui::Text("BALLS  %d / 3",model.balls);ImGui::SameLine();ImGui::TextColored(vec(theme.p.accent),"   %dx",model.multiplier);
  ImGui::TextDisabled("CIRCUITS  %d",model.circuits);ImGui::Spacing();ImGui::TextUnformatted("Complete the circuit");ImGui::TextWrapped("Light all three upper lanes and hit all three targets to raise the multiplier.");
  int lit=0;for(bool b:model.lanes)lit+=b;for(bool b:model.targets)lit+=b;ImGui::ProgressBar(lit/6.f,{-1,7},"");ImGui::TextDisabled("%d / 6 lit",lit);ImGui::Spacing();
  ImGui::TextWrapped("%s",model.message.c_str());if(model.saveTime>0&&!model.waiting)ImGui::TextColored(vec(theme.p.warm),"BALL SAVE  %.0fs",std::ceil(model.saveTime));
  if(model.waiting&&!model.gameOver){ImGui::ProgressBar(float(model.charge),{-1,13},"PLUNGER");}
  ImGui::Spacing();if(ImGui::Button(model.paused?"Resume  [P]":"Pause  [P]",{-1,0})&&!model.gameOver)model.paused=!model.paused;
  if(ImGui::Button("New game  [F2]",{-1,0})){if(model.gameOver||(model.score==0&&model.waiting)){model.newGame();restored=false;}else{confirm=true;dialog=true;model.paused=true;}}
  ImGui::Separator();const char* choices[]={"Follow Omarchy","Midnight","Amber"};ImGui::SetNextItemWidth(-1);ImGui::Combo("##appearance",&prefs.appearance,choices,3);
  if(prefs.appearance==0&&!theme.found)ImGui::TextDisabled("Using Midnight fallback");
  ImGui::Checkbox("Mute",&prefs.muted);ImGui::SameLine();ImGui::Checkbox("Music",&prefs.music);if(!audio.device)ImGui::TextDisabled("Audio device unavailable");
  ImGui::Separator();ImGui::TextDisabled("A / D or SHIFT  flippers\nSPACE  charge / release\nN  nudge   P  pause\nF11  fullscreen");
  ImGui::TextDisabled("Controller: shoulders + A");if(saveError)ImGui::TextWrapped("Could not save. Check available disk space and folder access.");
  if(ImGui::CollapsingHeader("About")){ImGui::TextWrapped("An independent community pinball game. Original Omarchy table and synthesized audio. Classic Space Cadet remains available with --classic and your own game data.");ImGui::TextWrapped("Official Omarchy logo: omarchy.org/brand. Omarchy trademark rights remain with its owner.");}
  ImGui::End();
  if(confirm){ImGui::OpenPopup("Start a new game?");confirm=false;}if(ImGui::BeginPopupModal("Start a new game?",nullptr,ImGuiWindowFlags_AlwaysAutoResize)){ImGui::TextUnformatted("This replaces the current game.");if(ImGui::Button("Start new game")){model.newGame();restored=false;dialog=false;ImGui::CloseCurrentPopup();}ImGui::SameLine();if(ImGui::Button("Keep playing")){model.paused=false;dialog=false;ImGui::CloseCurrentPopup();}ImGui::EndPopup();}
  ImGui::Render();SDL_SetRenderDrawColor(renderer,theme.p.background>>16,(theme.p.background>>8)&255,theme.p.background&255,255);SDL_RenderClear(renderer);ImGui_ImplSDLRenderer_RenderDrawData(ImGui::GetDrawData());
  ++frames;if(smoke&&frames>=smoke){if(!screenshot.empty()){int w,h;SDL_GetRendererOutputSize(renderer,&w,&h);auto surface=SDL_CreateRGBSurfaceWithFormat(0,w,h,32,SDL_PIXELFORMAT_ARGB8888);if(surface){SDL_RenderReadPixels(renderer,nullptr,SDL_PIXELFORMAT_ARGB8888,surface->pixels,surface->pitch);SDL_SaveBMP(surface,screenshot.c_str());SDL_FreeSurface(surface);}}running=false;}
  SDL_RenderPresent(renderer);if(!smoke&&SDL_GetTicks()-saveAt>5000){save();saveAt=SDL_GetTicks();}if(!smoke)SDL_Delay(1);
 }
 save();std::printf("frames=%d score=%d balls=%d circuits=%d restored=%d\n",frames,model.score,model.balls,model.circuits,restored);
 if(controller)SDL_GameControllerClose(controller);SDL_DestroyTexture(logo);ImGui_ImplSDLRenderer_Shutdown();ImGui_ImplSDL2_Shutdown();ImGui::DestroyContext();SDL_DestroyRenderer(renderer);SDL_DestroyWindow(window);
 if(audio.device){SDL_CloseAudioDevice(audio.device);audio.device=0;}
 SDL_Quit();
 return 0;
}
