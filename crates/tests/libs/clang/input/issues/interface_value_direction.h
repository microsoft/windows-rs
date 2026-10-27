#include "sal.h"

typedef long HRESULT;

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed051")) IContext {
    virtual void __stdcall Touch() = 0;
};

typedef IContext* LPCONTEXT;

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed052")) ITranslator {
    virtual HRESULT __stdcall Mutate(INOUT IContext* context) = 0;
    virtual HRESULT __stdcall MutateOptional(INOUT_OPT IContext* context) = 0;
    virtual HRESULT __stdcall Fill(OUT IContext* context) = 0;
    virtual HRESULT __stdcall MutateAlias(INOUT LPCONTEXT context) = 0;
    virtual HRESULT __stdcall MutateReference(INOUT IContext& context) = 0;
    virtual HRESULT __stdcall Read(IN IContext* context) = 0;
    virtual HRESULT __stdcall Create(OUTPTR IContext** context) = 0;
    virtual HRESULT __stdcall Replace(INOUT IContext** context) = 0;
};

typedef HRESULT(__stdcall* MUTATE_CALLBACK)(INOUT IContext* context);

extern "C" HRESULT __stdcall MutateContext(INOUT IContext* context, MUTATE_CALLBACK callback);
