#include "interfaces.h"

#if defined(_M_IX86)
#define TARGET_ABI __stdcall
#else
#define TARGET_ABI
#endif

typedef void (__attribute__((noreturn)) __cdecl *ExplicitStop)(int);
typedef void (__attribute__((noreturn)) *InferredStop)(short);
typedef void (*InferredInner)(unsigned);
typedef void (__attribute__((noreturn)) __cdecl *ExplicitOuter)(
    void (*)(unsigned));
typedef void (TARGET_ABI *NormalizedInner)(long long);
typedef void (__attribute__((noreturn)) *NamedOuter)(NormalizedInner);
typedef void (__cdecl *InterfaceCallback)(BaseAlias*);

#if defined(_M_IX86)
typedef void (__attribute__((noreturn)) __stdcall *VariantStop)(const int*);
#else
typedef void (__attribute__((noreturn)) *VariantStop)(int*);
#endif

struct CallbackUses {
    void (__attribute__((noreturn)) *explicit_alias)(int);
    void (__attribute__((noreturn)) __cdecl *inferred_alias)(short);
    void (__attribute__((noreturn)) *nested)(
        void (__cdecl *)(unsigned));
    void (__attribute__((noreturn)) *named_nested)(NormalizedInner);
    void (*interface_callback)(BaseAlias*);
#if defined(_M_IX86)
    void (__attribute__((noreturn)) __stdcall *variant)(const int*);
#else
    void (__attribute__((noreturn)) __cdecl *variant)(int*);
#endif
};
