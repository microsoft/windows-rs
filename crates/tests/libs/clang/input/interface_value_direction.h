//! namespace Test
//! library test.dll
#define _In_ __attribute__((annotate("_In_")))
#define _Inout_ __attribute__((annotate("_Inout_")))
#define _Inout_opt_ __attribute__((annotate("_Inout_opt_")))
#define _Out_ __attribute__((annotate("_Out_")))
#define _Outptr_ __attribute__((annotate("_Outptr_")))
typedef long HRESULT;

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed051")) IContext {
    virtual void __stdcall Touch() = 0;
};

typedef IContext* LPCONTEXT;
typedef IContext& ContextRef;

struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual void __stdcall Unused() = 0;
};

typedef IUnknown* LPUNKNOWN;

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed052")) ITranslator {
    virtual HRESULT __stdcall Mutate(_Inout_ IContext* context) = 0;
    virtual HRESULT __stdcall MutateOptional(_Inout_opt_ IContext* context) = 0;
    virtual HRESULT __stdcall Fill(_Out_ IContext* context) = 0;
    virtual HRESULT __stdcall MutateAlias(_Inout_ LPCONTEXT context) = 0;
    virtual HRESULT __stdcall MutateUnknown(_Inout_ LPUNKNOWN object) = 0;
    virtual HRESULT __stdcall FillReferenceAlias(_Out_ ContextRef context) = 0;
    virtual HRESULT __stdcall MutateReference(_Inout_ IContext& context) = 0;
    virtual HRESULT __stdcall Read(_In_ IContext* context) = 0;
    virtual HRESULT __stdcall Create(_Outptr_ IContext** context) = 0;
    virtual HRESULT __stdcall Replace(_Inout_ IContext** context) = 0;
};

typedef HRESULT(__stdcall* MUTATE_CALLBACK)(_Inout_ IContext* context);

extern "C" HRESULT __stdcall MutateContext(_Inout_ IContext* context, MUTATE_CALLBACK callback);
