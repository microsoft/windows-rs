//! flat

#define IN
#define OUT
#define OPTIONAL

typedef unsigned long DWORD;

void ReadLegacy(IN DWORD* value);
void ReadOptionalLegacy(IN OPTIONAL DWORD* value);
void OptionalLegacy(DWORD* value OPTIONAL);
void UpdateLegacy(IN OUT DWORD* value);
void WriteLegacy(OUT DWORD* value);
