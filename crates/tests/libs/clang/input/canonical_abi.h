#if defined(_M_IX86)
#define TARGET_ABI __stdcall
#define EXPLICIT_ABI __stdcall
#define OBSERVED_ABI __stdcall
#else
#define TARGET_ABI
#define EXPLICIT_ABI __cdecl
#if defined(EXPLICIT_WIDE) && defined(_M_X64)
#define OBSERVED_ABI __cdecl
#else
#define OBSERVED_ABI
#endif
#endif

typedef int (__cdecl *CdeclCallback)(int);
typedef int (TARGET_ABI *PortableCallback)(int);
typedef int (EXPLICIT_ABI *ExplicitCallback)(int);
typedef int (OBSERVED_ABI *ObservedCallback)(int);

extern "C" int TARGET_ABI Portable(CdeclCallback callback);
extern "C" int TARGET_ABI Inline(int (__cdecl *callback)(short));
extern "C" int EXPLICIT_ABI Explicit(int value);
#if defined(EXPLICIT_WIDE)
extern "C" int OBSERVED_ABI Observed(int zvalue);
#else
extern "C" int OBSERVED_ABI Observed(int value);
#endif
extern "C" int TARGET_ABI LibraryVariant(int value);
extern "C" int TARGET_ABI ExportVariant(int value);

#if defined(_M_IX86)
extern "C" int TARGET_ABI ParameterVariant(const int* value);
extern "C" int TARGET_ABI NarrowOnly(int value);
#else
extern "C" int TARGET_ABI ParameterVariant(int* value);
extern "C" int WideOnly(int value);
#endif
