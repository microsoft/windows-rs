typedef int HRESULT;
typedef unsigned int ULONG;

struct __attribute__((annotate("00000000-0000-0000-c000-000000000046")))
IUnknown {
    virtual HRESULT QueryInterface(void* riid, void** ppvObject) = 0;
    virtual ULONG AddRef() = 0;
    virtual ULONG Release() = 0;
};

struct __attribute__((annotate("af86e2e0-b12d-4c6a-9c5a-d7aa65101e90")))
IInspectable : public IUnknown {
    virtual HRESULT GetIids(ULONG* iidCount, void** iids) = 0;
    virtual HRESULT GetRuntimeClassName(void** className) = 0;
    virtual HRESULT GetTrustLevel(int* trustLevel) = 0;
};
