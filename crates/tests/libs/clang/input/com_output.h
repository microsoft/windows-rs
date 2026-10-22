#ifndef VERSION
#define VERSION "2"
#endif
#ifndef ANNOTATION
#define ANNOTATION "_COM_Outptr_"
#endif
#define OUTPUT __attribute__((annotate("windows_clang:sal:" VERSION ":" ANNOTATION "\n()\n()")))

typedef void* Untyped;
struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IObject {
    virtual void __stdcall Run() = 0;
};
#define PARAMETERS OUTPUT void** raw, OUTPUT Untyped* alias, OUTPUT IObject** typed, OUTPUT const void** constant, OUTPUT void*** indirect, /* [iid_is][out] */ void** comment

typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
