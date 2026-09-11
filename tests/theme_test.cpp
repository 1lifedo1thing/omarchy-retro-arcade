#include "ThemePalette.h"
#include <sstream>
#include <iostream>
#include <cstdlib>
static void check(bool ok,const char* name) { if(!ok) { std::cerr<<name<<'\n'; std::exit(1); } }
int main() {
    std::istringstream input("background = \"#123456\"\naccent = '#AbCdEf' # note\nyellow = \"#fedcba\"\nforeground = \"#ffffff\"\n");
    auto p=cadet::readPalette(input);
    check(p.background==0x123456 && p.accent==0xabcdef && p.warm==0xfedcba,"valid assignments");
    check(cadet::tint(0,p)==p.background,"black maps to background");
    check(cadet::tint(0xffffff,p)==p.foreground,"white maps to foreground");
    check(cadet::tint(0xb00000,p)!=cadet::tint(0x0000b0,p),"warm and cold lights remain distinct");
    std::istringstream bad("accent = \"#zzzzzz\"\nbackground = \"#000000\" garbage\nforeground = '#12345'\n");
    auto q=cadet::readPalette(bad,p);
    check(q.accent==p.accent && q.background==p.background && q.foreground==p.foreground,"invalid colours preserve defaults");
    std::istringstream empty("");
    check(cadet::readPalette(empty).accent==cadet::Palette{}.accent,"missing theme fallback");
    std::cout<<"Theme palette checks passed\n";
}
