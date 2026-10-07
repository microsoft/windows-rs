#include <unknwnbase.h>

struct ComStats {
    unsigned long references;
    unsigned long adds;
    unsigned long releases;
    unsigned long queries;
    unsigned long created;
    unsigned long destroyed;
};

extern "C" IClassFactory* ComFactory(ComStats* factory, ComStats* instance);

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IProperties : IUnknown {
    virtual HRESULT __stdcall put_Window(/* [in] */ HWND value) = 0;
    virtual HRESULT __stdcall get_Window(/* [out] */ HWND* value) = 0;
    virtual HRESULT __stdcall SetText(/* [in] */ LPCWSTR value) = 0;
    virtual HRESULT __stdcall MatchText(/* [in] */ LPCWSTR value, /* [out] */ BOOL* equal) = 0;
};

extern "C" IProperties* ComProperties();
