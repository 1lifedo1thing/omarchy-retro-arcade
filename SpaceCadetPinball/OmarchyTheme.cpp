#include "pch.h"
#include "OmarchyTheme.h"
#include "ThemePalette.h"
#include "winmain.h"
#include <fstream>
#include <sstream>
#include <cstdlib>

namespace OmarchyTheme {
static cadet::Palette palette;
static uint32_t lookup[32768];
static int mode = 0;
static std::string settings, themeFile, previous;
static Uint32 checked = 0;
static bool ready = false, usingFile = false;
static ImVec4 colour(uint32_t c, float alpha=1.f) {
    return {((c>>16)&255)/255.f, ((c>>8)&255)/255.f, (c&255)/255.f, alpha};
}
static void apply() {
    ImGui::StyleColorsDark();
    auto& s=ImGui::GetStyle();
    s.WindowRounding=6; s.FrameRounding=4; s.PopupRounding=6;
    s.WindowPadding={14,12}; s.FramePadding={8,5}; s.ItemSpacing={10,8};
    s.Colors[ImGuiCol_Text]=colour(palette.foreground);
    s.Colors[ImGuiCol_WindowBg]=colour(palette.background);
    s.Colors[ImGuiCol_PopupBg]=colour(palette.background);
    s.Colors[ImGuiCol_MenuBarBg]=colour(palette.background);
    s.Colors[ImGuiCol_TitleBg]=colour(palette.background);
    s.Colors[ImGuiCol_TitleBgActive]=colour(palette.accent,.35f);
    for (int id : {ImGuiCol_Button,ImGuiCol_Header,ImGuiCol_FrameBg,ImGuiCol_Tab})
        s.Colors[id]=colour(palette.accent,.18f);
    for (int id : {ImGuiCol_ButtonHovered,ImGuiCol_HeaderHovered,ImGuiCol_FrameBgHovered,ImGuiCol_TabHovered})
        s.Colors[id]=colour(palette.accent,.40f);
    for (int id : {ImGuiCol_ButtonActive,ImGuiCol_HeaderActive,ImGuiCol_FrameBgActive,ImGuiCol_TabActive})
        s.Colors[id]=colour(palette.accent,.55f);
    s.Colors[ImGuiCol_CheckMark]=colour(palette.accent);
    s.Colors[ImGuiCol_SliderGrab]=colour(palette.accent);
    s.Colors[ImGuiCol_NavHighlight]=colour(palette.accent);
    SDL_SetRenderDrawColor(winmain::Renderer, (palette.background>>16)&255,
                          (palette.background>>8)&255, palette.background&255, 255);
    for (unsigned i=0;i<32768;++i) {
        unsigned r=(i>>10)&31,g=(i>>5)&31,b=i&31;
        lookup[i]=cadet::tint(((r*255/31)<<16)|((g*255/31)<<8)|(b*255/31),palette);
    }
    ready=true;
}
static void reload() {
    palette=cadet::Palette{}; usingFile=false;
    if (mode==0) {
        std::ifstream file(themeFile);
        if(file) { char bytes[65536]; file.read(bytes,sizeof(bytes)); std::istringstream bounded(std::string(bytes,static_cast<size_t>(file.gcount()))); palette=cadet::readPalette(bounded,palette); usingFile=true; }
    } else if(mode==2) {
        palette.background=0x211b14; palette.accent=0xf4b860;
        palette.foreground=0xffecd2; palette.warm=0xf18455;
    }
    apply();
}
void Init(const char* preferences) {
    settings=std::string(preferences ? preferences : "")+"appearance.txt";
    std::ifstream saved(settings); int value=0;
    if(saved>>value && value>=0 && value<=3) mode=value;
    const char* custom=std::getenv("SPACECADET_THEME_FILE");
    const char* config=std::getenv("XDG_CONFIG_HOME");
    const char* home=std::getenv("HOME");
    themeFile=custom && *custom ? custom :
        std::string(config && *config ? config : (home ? std::string(home)+"/.config" : ""))+"/omarchy/current/theme/colors.toml";
    reload();
}
void Update() {
    Uint32 now=SDL_GetTicks();
    if(mode!=0 || now-checked<2000) return;
    checked=now;
    std::ifstream file(themeFile, std::ios::binary);
    char data[65536]; file.read(data,sizeof(data));
    std::string current(data,static_cast<size_t>(file.gcount()));
    if(current!=previous) { previous=current; reload(); }
}
void Menu() {
    if(!ImGui::BeginMenu("Appearance")) return;
    const char* names[]={"Follow Omarchy", "Midnight", "Amber", "Original table colours"};
    for(int i=0;i<4;++i) if(ImGui::MenuItem(names[i],nullptr,mode==i)) {
        mode=i; reload(); std::ofstream file(settings); file<<mode<<'\n';
    }
    if(mode==0 && !usingFile) { ImGui::Separator(); ImGui::TextDisabled("Omarchy theme unavailable; using Midnight."); }
    ImGui::EndMenu();
}
void Transform(const uint32_t* source,uint32_t* target,int pixels) {
    if(!ready || mode==3) { std::memcpy(target,source,pixels*sizeof(uint32_t)); return; }
    for(int i=0;i<pixels;++i) {
        uint32_t c=source[i];
        unsigned index=(((c>>19)&31)<<10)|(((c>>11)&31)<<5)|((c>>3)&31);
        target[i]=(c&0xff000000)|lookup[index];
    }
}
}
