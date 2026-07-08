// SAL annotations on COM interface methods, exercising the interface.rs SAL path
// (buffer-size capture and the ComOutPtr marker) in addition to the free-function
// path covered by sal_params.h.

#define _In_ __attribute__((annotate("_In_")))
#define _In_reads_(c) __attribute__((annotate("_In_reads_(" #c ")")))
#define _COM_Outptr_ __attribute__((annotate("_COM_Outptr_")))

typedef unsigned long ULONG;
typedef int HRESULT;

struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IThing {
    // _COM_Outptr_ on a pointer-to-pointer -> #[...::ComOutPtr]; Out default applies.
    virtual HRESULT QueryInterface(_In_ const void* riid, _COM_Outptr_ void** ppvObject) = 0;

    // _In_reads_(count) -> NativeArrayInfo(CountParamIndex = 1); In default applies.
    virtual HRESULT ReadInto(_In_reads_(count) const ULONG* values, ULONG count) = 0;
};
