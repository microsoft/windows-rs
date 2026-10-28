#include "interfaces.h"
struct __declspec(uuid("fedcba98-1234-5678-90ab-cdef01234567")) IOverloaded : IBase {
    virtual int __stdcall Read(int value) = 0;
    virtual int __stdcall Read(float value) = 0;
    virtual int __stdcall Read(int value) const = 0;
    virtual int __stdcall Tail() = 0;
};
