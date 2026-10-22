#include <dependencies.h>

#if CAPTURE_TEST != 7
#error missing compiler definition
#endif

#if defined(_M_IX86)
struct X86Only { int value; };
#elif defined(_M_ARM64)
struct ArmOnly { long value; };
#endif

struct Root {
    Required embedded;
    const Required* again;
    Opaque* opaque;
#if defined(_M_IX86)
    X86Only* arch;
#elif defined(_M_ARM64)
    ArmOnly arch;
#else
    long arch;
#endif
};
