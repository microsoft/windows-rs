// Test: IID extracted from macroized GUID variable declarations.
// When macros are used inside the GUID initializer (e.g. 7zip SDK pattern),
// the AST-based evaluator should resolve the values after macro expansion.

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

// Macros mimicking the 7zip SDK pattern.
#define k_7zip_GUID_Data1 0x23170F69
#define k_7zip_GUID_Data2 0x40C1
#define k_7zip_GUID_Data3_Common 0x278A

#define Z7_DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
    extern "C" const GUID name = { l, w1, w2, { b1, b2, b3, b4, b5, b6, b7, b8 } }

// IID defined using macros in the initializer.
Z7_DEFINE_GUID(IID_ISequentialInStream,
    k_7zip_GUID_Data1, k_7zip_GUID_Data2, k_7zip_GUID_Data3_Common,
    0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00);

struct ISequentialInStream : public IUnknown {
    virtual int __stdcall Read(void* data, unsigned int size, unsigned int* processedSize) = 0;
};

// Another interface using the same macro pattern.
Z7_DEFINE_GUID(IID_ISequentialOutStream,
    k_7zip_GUID_Data1, k_7zip_GUID_Data2, k_7zip_GUID_Data3_Common,
    0x00, 0x00, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00);

struct ISequentialOutStream : public IUnknown {
    virtual int __stdcall Write(const void* data, unsigned int size, unsigned int* processedSize) = 0;
};
