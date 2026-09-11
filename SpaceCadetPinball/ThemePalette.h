#pragma once
#include <cstdint>
#include <istream>
#include <string>

namespace cadet {
struct Palette {
    uint32_t background = 0x171b25;
    uint32_t foreground = 0xe4e9f2;
    uint32_t accent = 0x89b4fa;
    uint32_t warm = 0xf2bd79;
};
// Parse only literal colour assignments. Never execute theme configuration.
Palette readPalette(std::istream& input, Palette fallback = Palette{});
std::string themePath();
uint32_t tint(uint32_t rgb, const Palette& palette);
}
