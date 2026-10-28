#include "interfaces.h"
struct __declspec(uuid("12345678-1234-5678-1234-567812345679")) IVolatile : IBase {
    virtual int __stdcall Inspect() volatile = 0;
};
struct __declspec(uuid("12345678-1234-5678-1234-567812345670")) IRefQualified : IBase {
    virtual int __stdcall Inspect() & = 0;
};
struct UsingType : IBase {
    using IBase::QueryInterface;
    using IBase::IBase;
};
