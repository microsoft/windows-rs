#include "interfaces.h"
struct __declspec(uuid("fedcba98-1234-5678-90ab-cdef01234567")) ICollision : IBase {
    virtual int __stdcall Read(int value) = 0;
    virtual int __stdcall Read(float value) = 0;
    virtual int __stdcall Read2() = 0;
};
