struct __declspec(uuid("00000000-0000-0000-C000-000000000046"))
IBase {
    virtual int __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual unsigned int __stdcall AddRef() = 0;
    virtual unsigned int __stdcall Release() = 0;
};

class __declspec(uuid("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90"))
IDerived : public IBase {
public:
    virtual int __stdcall GetIids(unsigned int* iidCount, void** iids) = 0;
    virtual int __stdcall GetRuntimeClassName(void** className) = 0;
    virtual int __stdcall GetTrustLevel(int* trustLevel) = 0;
};
