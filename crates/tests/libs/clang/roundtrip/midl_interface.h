#define MIDL_INTERFACE(x) struct __declspec(uuid(x))

MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
IBase {
    virtual int __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual unsigned int __stdcall AddRef() = 0;
    virtual unsigned int __stdcall Release() = 0;
};

MIDL_INTERFACE("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")
IDerived : public IBase {
public:
    virtual int __stdcall GetIids(unsigned int* iidCount, void** iids) = 0;
    virtual int __stdcall GetRuntimeClassName(void** className) = 0;
    virtual int __stdcall GetTrustLevel(int* trustLevel) = 0;
};

// IThird is unique to midl_interface.h and exercises a third consecutive
// MIDL_INTERFACE expansion.  This keeps the golden file distinct from
// interface.rdl (which only has IBase and IDerived) while ensuring the
// expansion-range UUID extraction fix works for more than two interfaces.
MIDL_INTERFACE("CAFEBABE-CAFE-BABE-CAFE-BABECAFEBABE")
IThird : public IDerived {
public:
    virtual int __stdcall Execute(int command) = 0;
};
