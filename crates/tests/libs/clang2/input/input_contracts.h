#ifndef INPUT
#define INPUT __attribute__((annotate("windows_clang:sal:1.2:__RPC__in\n()\n()")))
#endif

struct Handle__;
typedef Handle__* Handle;
struct Value { int data; };
typedef Value ValueAlias;

#define PARAMETERS INPUT void* raw, INPUT Handle handle, INPUT const ValueAlias& reference
typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
