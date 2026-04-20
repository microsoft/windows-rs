typedef int HRESULT;
typedef unsigned int ULONG;

struct __declspec(uuid("00000000-0000-0000-C000-000000000046"))
IUnknown {
    virtual HRESULT __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual ULONG __stdcall AddRef() = 0;
    virtual ULONG __stdcall Release() = 0;
};

class __declspec(uuid("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90"))
IInspectable : public IUnknown {
public:
    virtual HRESULT __stdcall GetIids(ULONG* iidCount, void** iids) = 0;
    virtual HRESULT __stdcall GetRuntimeClassName(void** className) = 0;
    virtual HRESULT __stdcall GetTrustLevel(int* trustLevel) = 0;
};
