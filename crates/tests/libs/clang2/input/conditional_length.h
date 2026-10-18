#ifndef VERSION
#define VERSION "1.2"
#endif
#ifndef ANNOTATION
#define ANNOTATION "__RPC__out_ecount_part"
#endif
#ifndef LENGTH
#define LENGTH "written ? *written : capacity"
#endif
#ifndef COUNT_TYPE
#define COUNT_TYPE unsigned int*
#endif
#ifndef FALLBACK_TYPE
#define FALLBACK_TYPE unsigned int
#endif
#define BUFFER __attribute__((annotate("windows_clang:sal:" VERSION ":" ANNOTATION "\n(capacity," LENGTH ")\n(capacity," LENGTH ")")))
#define PARAMETERS FALLBACK_TYPE capacity, BUFFER unsigned short* buffer, COUNT_TYPE written, unsigned int other

typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
