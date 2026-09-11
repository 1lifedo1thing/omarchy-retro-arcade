#pragma once
class DatFile;
class TPinballComponent;
enum class MessageCode;
namespace OmarchyTable {
extern bool Enabled;
DatFile* Build();
void ComponentEvent(MessageCode code, TPinballComponent* component);
void TableEvent(MessageCode code);
void Shutdown();
}
