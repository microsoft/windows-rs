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
#ifndef PORTABLE_CALLBACK_ABI
#define PORTABLE_CALLBACK_ABI TARGET_ABI
#endif
typedef int (PORTABLE_CALLBACK_ABI *PortableCallback)(int);
typedef int (EXPLICIT_ABI *ExplicitCallback)(int);
typedef int (OBSERVED_ABI *ObservedCallback)(int);
typedef int (TARGET_ABI *AInferredCallback)(unsigned int);
typedef int (__cdecl *ZExplicitCallback)(unsigned int);
typedef int (TARGET_ABI *AInferredNested)(int (TARGET_ABI *)(unsigned int));
typedef int (TARGET_ABI *ZExplicitNested)(int (__cdecl *)(unsigned int));
struct CallbackUses {
    int (__cdecl *explicit_callback)(unsigned int);
    int (TARGET_ABI *inferred_callback)(unsigned int);
    int (TARGET_ABI *explicit_nested)(int (__cdecl *)(unsigned int));
};

#ifndef PORTABLE_ABI
#define PORTABLE_ABI TARGET_ABI
#endif

extern "C" int PORTABLE_ABI Portable(CdeclCallback callback);
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
typedef int (TARGET_ABI *ParameterCallback)(const int*);
extern "C" int TARGET_ABI ParameterVariant(const int* value);
extern "C" int TARGET_ABI NarrowOnly(int value);
#else
typedef int (TARGET_ABI *ParameterCallback)(int*);
extern "C" int TARGET_ABI ParameterVariant(int* value);
extern "C" int WideOnly(int value);
#endif
