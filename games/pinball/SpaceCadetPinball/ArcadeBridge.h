#pragma once
struct SDL_Renderer;
namespace ArcadeBridge {
bool Enabled();
void Init();
void Pump();
void Present(SDL_Renderer* renderer);
}
