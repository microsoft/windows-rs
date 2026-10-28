#ifndef VERSION
#define VERSION "2"
#endif
#ifndef ANNOTATION
#define ANNOTATION "_In_reads_"
#endif
#ifndef COUNT
#define COUNT count
#endif
#ifndef BOUND
#define BOUND COUNT
#endif
#ifndef COUNT_TYPE
#define COUNT_TYPE unsigned int
#endif
#define TEXT_INNER(value) #value
#define TEXT(value) TEXT_INNER(value)
#define BUFFER __attribute__((annotate("windows_clang:sal:" VERSION ":" ANNOTATION "\n(BOUND)\n(" TEXT(BOUND) ")")))
#define PARAMETERS BUFFER unsigned short* buffer, COUNT_TYPE COUNT, unsigned int other

typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
