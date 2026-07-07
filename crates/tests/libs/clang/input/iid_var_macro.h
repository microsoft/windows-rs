// Interface GUID association from a DEFINE_GUID(IID_<Interface>) macro.
//
// A COM interface declared without an inline `__declspec(uuid(...))` (the classic
// `DECLARE_INTERFACE_` idiom) carries no UUID on the type itself; its IID is supplied
// separately by a `DEFINE_GUID(IID_<Interface>, ...)`. Without INITGUID that macro
// expands to a valueless `extern "C" const GUID IID_<Interface>` declaration, so the
// faithful value lives only in the macro arguments. The scraper reads it there and
// associates it with the interface, independent of INITGUID/definition mode — so the
// interface below is emitted with `#[guid(...)]`, not `#[no_guid]`.
typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;

#define DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
    extern "C" const GUID name

struct IUnknown {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};

// No inline uuid on the interface; its IID arrives via the macro below.
struct ISample : public IUnknown {
    virtual int GetValue() = 0;
};

DEFINE_GUID(IID_ISample, 0x12345678, 0x1234, 0x5678, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44);
