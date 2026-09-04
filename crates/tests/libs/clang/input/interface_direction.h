// A local interface can carry invalid MIDL direction comments that normal stub generation rejects.
// Preserve the native pointer shape while keeping the emitted metadata internally consistent.

typedef int HRESULT;

#define _Out_ __attribute__((annotate("_Out_")))

struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IThing {
    virtual HRESULT Use(/* [out] */ IThing* sink) = 0;
    virtual HRESULT Reuse(/* [in] [out] */ IThing* value) = 0;
    virtual HRESULT Observe(_Out_ IThing* value) = 0;
    virtual HRESULT Return(/* [retval] [out] */ IThing* value) = 0;
    virtual HRESULT Create(/* [out] */ IThing** result) = 0;
};
