#include <intsafe.h>
#include <windows.h>

#if defined(_M_IX86)
static_assert(sizeof(CONTEXT) == 716);
static_assert(alignof(CONTEXT) == 4);
static_assert(__builtin_offsetof(CONTEXT, Eip) == 184);
#elif defined(_M_X64)
static_assert(sizeof(CONTEXT) == 1232);
static_assert(alignof(CONTEXT) == 16);
static_assert(__builtin_offsetof(CONTEXT, Rip) == 248);
#elif defined(_M_ARM64)
static_assert(sizeof(CONTEXT) == 912);
static_assert(alignof(CONTEXT) == 16);
static_assert(__builtin_offsetof(CONTEXT, Pc) == 264);
#endif
