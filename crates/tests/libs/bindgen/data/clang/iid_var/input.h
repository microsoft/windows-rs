// Test: IID extracted from `const GUID IID_XXX = { ... }` variable declarations
// and applied to interfaces that lack __declspec(uuid(...)).

typedef struct _GUID {
    unsigned int Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID, IID;

struct IUnknown {
    virtual int __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual unsigned int __stdcall AddRef() = 0;
    virtual unsigned int __stdcall Release() = 0;
};

// Interface with IID defined via a separate variable.
extern "C" const GUID IID_ISequentialInStream = { 0x23170F69, 0x40C1, 0x278A, { 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00 } };

struct ISequentialInStream : public IUnknown {
    virtual int __stdcall Read(void* data, unsigned int size, unsigned int* processedSize) = 0;
};

// IID variable appears AFTER the interface declaration.
struct ISequentialOutStream : public IUnknown {
    virtual int __stdcall Write(const void* data, unsigned int size, unsigned int* processedSize) = 0;
};

extern "C" const GUID IID_ISequentialOutStream = { 0x23170F69, 0x40C1, 0x278A, { 0x00, 0x00, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00 } };

// Interface with __declspec(uuid(...)) should NOT be overridden by an IID variable.
struct __declspec(uuid("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90"))
IExplicitGuid : public IUnknown {
    virtual int __stdcall DoSomething() = 0;
};

extern "C" const GUID IID_IExplicitGuid = { 0x00000000, 0x0000, 0x0000, { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 } };

// IID variable using decimal literals instead of hex.
extern "C" const GUID IID_IDecimalGuid = { 591397737, 16577, 10122, { 0, 0, 0, 3, 0, 3, 0, 0 } };

struct IDecimalGuid : public IUnknown {
    virtual int __stdcall GetValue(int* result) = 0;
};
