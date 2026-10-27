#ifndef VERSION
#define VERSION "2"
#endif
#ifndef ANNOTATION
#define ANNOTATION "_In_z_"
#endif
#ifndef ARGUMENTS
#define ARGUMENTS ""
#endif
#ifndef LENGTH_TYPE
#define LENGTH_TYPE unsigned int
#endif
#ifndef LENGTH_CONTRACT
#define LENGTH_CONTRACT
#endif
#define BUFFER __attribute__((annotate("windows_clang:sal:" VERSION ":" ANNOTATION "\n(" ARGUMENTS ")\n(" ARGUMENTS ")")))
#define PARAMETERS BUFFER unsigned short* buffer, unsigned int capacity, LENGTH_CONTRACT LENGTH_TYPE written, unsigned int other

typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
