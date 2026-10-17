#include "interfaces.h"

typedef void (__attribute__((noreturn)) *StoppingCallback)(int code);
typedef void (*OrdinaryCallback)(int code);

struct StoppingCallbacks {
    StoppingCallback named;
    void (__attribute__((noreturn)) *inferred)(int code);
    OrdinaryCallback ordinary;
};

extern "C" __declspec(noreturn) int NativeSum(int code);
extern "C" __attribute__((noreturn)) void ReadPayload(int code) noexcept;
extern "C" [[noreturn]] void BufferFunction(int code);
extern "C" [[noreturn]] __declspec(noreturn) void NormalizeFunction(int code);

struct __declspec(uuid("98765432-1234-5678-90ab-cdef01234567")) IStopping : IBase {
    virtual __declspec(noreturn) void __stdcall Stop(int code) = 0;
    [[noreturn]] virtual void __stdcall StandardStop(int code) = 0;
    virtual void __stdcall Ordinary(int code) = 0;
};
