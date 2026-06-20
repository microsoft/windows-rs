// COM interface with pure virtual methods and __declspec(uuid).
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};

struct __declspec(uuid("10000000-0000-0000-c000-000000000046")) IFoo : public IUnknown {
    virtual int GetValue() = 0;
    virtual void SetValue(int value) = 0;
};
