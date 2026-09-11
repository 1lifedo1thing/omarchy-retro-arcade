#pragma once
#include <cstdint>
namespace OmarchyTheme {
void Init(const char* preferences);
void Update();
void Menu();
void Transform(const uint32_t* source, uint32_t* target, int pixels);
}
