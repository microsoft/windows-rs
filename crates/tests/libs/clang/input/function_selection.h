extern "C" int NativeA(int value);
extern "C" int NativeW(int value);
extern "C" int Shadow(unsigned value);
#define Invariant NativeW

#ifdef UNICODE
#define Switched NativeW
#define Shadow NativeW
#else
#define Switched NativeA
#define Shadow NativeA
#endif

#if defined(_M_IX86)
#define Variant NativeA
#ifdef UNICODE
#define Wide NativeW
#else
#define Wide NativeA
#endif
#else
extern "C" int Variant(int value);
extern "C" int Wide(int value);
#endif
