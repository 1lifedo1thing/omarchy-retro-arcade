#include "ThemePalette.h"
#include <algorithm>
#include <string>

namespace cadet {
static std::string trim(const std::string& s) {
    auto first = s.find_first_not_of(" \t\r\n");
    if (first == std::string::npos) return {};
    return s.substr(first, s.find_last_not_of(" \t\r\n") - first + 1);
}
Palette readPalette(std::istream& input, Palette result) {
    std::string line;
    unsigned lines = 0;
    while (lines++ < 512 && std::getline(input, line)) {
        auto eq = line.find('=');
        if (eq == std::string::npos) continue;
        auto key = trim(line.substr(0, eq));
        auto value = trim(line.substr(eq + 1));
        if (value.size() < 9 || (value[0] != '"' && value[0] != '\'') ||
            value[1] != '#' || value[8] != value[0]) continue;
        auto tail = trim(value.substr(9));
        if (!tail.empty() && tail[0] != '#') continue;
        uint32_t colour = 0;
        bool valid = true;
        for (unsigned i = 2; i < 8; ++i) {
            char c = value[i];
            int n = c >= '0' && c <= '9' ? c - '0' :
                c >= 'a' && c <= 'f' ? c - 'a' + 10 :
                c >= 'A' && c <= 'F' ? c - 'A' + 10 : -1;
            if (n < 0) { valid = false; break; }
            colour = (colour << 4) | unsigned(n);
        }
        if (!valid) continue;
        if (key == "background") result.background = colour;
        else if (key == "foreground") result.foreground = colour;
        else if (key == "accent") result.accent = colour;
        else if (key == "yellow") result.warm = colour;
    }
    return result;
}
static uint32_t mix(uint32_t a, uint32_t b, unsigned t) {
    uint32_t result = 0;
    for (unsigned shift : {0u, 8u, 16u}) {
        unsigned av = (a >> shift) & 255, bv = (b >> shift) & 255;
        result |= ((av * (255-t) + bv * t) / 255) << shift;
    }
    return result;
}
uint32_t tint(uint32_t rgb, const Palette& p) {
    unsigned r=(rgb>>16)&255, g=(rgb>>8)&255, b=rgb&255;
    unsigned brightness = std::max(r, std::max(g,b));
    unsigned saturation = brightness - std::min(r,std::min(g,b));
    // Preserve bright ball/rail highlights and distinguish warm target lights.
    uint32_t mid = saturation < 24 ? mix(p.accent,p.foreground,128) :
        (r > b + 24 && r >= g ? p.warm : p.accent);
    return brightness < 176 ? mix(p.background,mid,brightness*255/176) :
        mix(mid,p.foreground,(brightness-176)*255/79);
}
}
