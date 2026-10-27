extern "C" int __stdcall Available(unsigned value);
#define PublicAvailable Available

#ifdef _M_IX86
#define QUALIFIER const
#else
#define QUALIFIER
#endif
struct UnsupportedValue { QUALIFIER int value; };
extern "C" int __stdcall Unsupported(UnsupportedValue value);
