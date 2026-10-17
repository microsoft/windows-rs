#ifndef VERSION
#define VERSION "2"
#endif
#ifndef ANNOTATION
#define ANNOTATION "_Reserved_"
#endif
#ifndef ARGUMENTS
#define ARGUMENTS ""
#endif
#ifndef RESERVED
#define RESERVED __attribute__((annotate("windows_clang:sal:" VERSION ":" ANNOTATION "\n(" ARGUMENTS ")\n(" ARGUMENTS ")")))
#endif
#define INPUT __attribute__((annotate("windows_clang:sal:2:_In_\n()\n()")))
#define PARAMETERS RESERVED unsigned int zero, RESERVED void* null, INPUT RESERVED void* input, unsigned int ordinary

typedef void (*Reserved)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IReserved {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
