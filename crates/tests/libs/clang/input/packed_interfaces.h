#include "interfaces.h"

#ifndef PACK
#define PACK 1
#endif

#pragma pack(push, PACK)
struct __declspec(uuid("e43a137d-594f-4f29-9566-acf00ca33a14")) __declspec(novtable)
IPackedRoot {
    virtual HRESULT __stdcall QueryInterface(const GUID& riid, void** object) = 0;
    virtual ULONG __stdcall AddRef() = 0;
    virtual ULONG __stdcall Release() = 0;
};

struct __declspec(uuid("27c47b3a-51c2-4708-881a-7544dd69b1d5")) __declspec(novtable)
IPacked : IBase {
    virtual ULONG __stdcall Read() = 0;
};
#pragma pack(pop)

struct __declspec(uuid("ac6fab2e-cd0c-4053-a74d-b49517507982")) __declspec(novtable)
IPackedChild : IPacked {
    virtual ULONG __stdcall Count() = 0;
};
