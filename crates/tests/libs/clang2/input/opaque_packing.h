#if defined(PACK_TWO)
#pragma pack(push, 2)
#else
#pragma pack(push, 1)
#endif
struct PackingProbe { char value; };
#pragma pack(pop)
