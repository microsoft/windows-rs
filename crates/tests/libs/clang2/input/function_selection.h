extern "C" int NativeA(int value);
extern "C" int NativeW(int value);
#define Neutral NativeA
#define Compatibility NativeA
#define Required NativeW

#if defined(_M_IX86)
#define Variant NativeA
#else
extern "C" int Variant(int value);
#endif
