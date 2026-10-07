#define _Inout_ __attribute__((annotate("_Inout_")))
#define _Outptr_ __attribute__((annotate("_Outptr_")))
typedef long HRESULT;

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed051")) IContext {
    virtual void __stdcall Touch() = 0;
};

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed052")) ITranslator {
    virtual HRESULT __stdcall Mutate(_Inout_ IContext* context) = 0;
    virtual HRESULT __stdcall Create(_Outptr_ IContext** context) = 0;
};

extern "C" void MutateObject(_Inout_ IContext* context);
