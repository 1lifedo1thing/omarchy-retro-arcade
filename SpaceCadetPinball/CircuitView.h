#pragma once
struct SDL_Renderer;
namespace CircuitView {
bool Init(SDL_Renderer* renderer);
void Draw();
void Shutdown();
}
