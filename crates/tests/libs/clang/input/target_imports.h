extern "C" int __stdcall Available(unsigned value);
#define PublicAvailable Available

#ifdef _M_IX86
#define QUALIFIER volatile
#else
#define QUALIFIER
#endif
extern "C" int __stdcall Unsupported(QUALIFIER int* value);
