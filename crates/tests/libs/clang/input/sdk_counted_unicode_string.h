#if defined(_M_AMD64)
#define _AMD64_
#define AMD64
#elif defined(_M_ARM64)
#define _ARM64_
#define ARM64
#elif defined(_M_IX86)
#define _X86_
#define i386 1
#else
#error Unsupported target
#endif
#include <ntdef.h>
