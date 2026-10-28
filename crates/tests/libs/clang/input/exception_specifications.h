#include "interfaces.h"

typedef int (__stdcall *NoThrowCallback)(int value) noexcept;
typedef int (__stdcall *OrdinaryCallback)(int value);

struct CallbackUsers {
    NoThrowCallback safe;
    OrdinaryCallback ordinary;
    int (__stdcall *inferred_safe)(int value) noexcept;
    int (__stdcall *inferred_ordinary)(int value);
};

struct __declspec(uuid("98765432-1234-5678-90ab-cdef01234567")) INoThrow : IBase {
    virtual int __stdcall Safe(int value) noexcept = 0;
    virtual __declspec(nothrow) int __stdcall Legacy(int value) = 0;
    virtual int __stdcall Ordinary(int value) = 0;
};

extern "C" int __stdcall NativeSum(int value) noexcept;
extern "C" int __stdcall ReadPayload(int value) throw();
