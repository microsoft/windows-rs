/* Self-contained test exercising the MIDL-generated COM interface pattern.
 *
 * Real MIDL headers (e.g. WebView2.h) use the same conventions: forward
 * declarations via `typedef struct X X;` followed by interface definitions
 * via the MIDL_INTERFACE() macro.  Define the necessary macros inline here so
 * the test does not require the Windows SDK.
 */

#define MIDL_INTERFACE(x) struct __declspec(uuid(x))
#define STDMETHOD(m) virtual long __stdcall m
#define STDMETHOD_(t, m) virtual t __stdcall m
#define PURE = 0

struct IUnknown {
    virtual long QueryInterface(const void* riid, void** ppv) = 0;
    virtual unsigned long AddRef(void) = 0;
    virtual unsigned long Release(void) = 0;
};

/* Forward declarations (the pattern used by MIDL-generated headers).
 * These must not poison the `seen` set and block the actual definitions. */
typedef struct ICoreWebView2 ICoreWebView2;
typedef struct ICoreWebView2Environment ICoreWebView2Environment;

/* Interface definitions via MIDL_INTERFACE. */
MIDL_INTERFACE("76eceacb-0462-4d94-ac83-423a6793775e")
ICoreWebView2 : public IUnknown {
public:
    STDMETHOD(Navigate)(const wchar_t* uri) PURE;
    STDMETHOD_(unsigned long, GetId)(void) PURE;
};

MIDL_INTERFACE("b96d755e-0319-11eb-adc1-0242ac120002")
ICoreWebView2Environment : public IUnknown {
public:
    STDMETHOD(CreateWebView)(ICoreWebView2** out) PURE;
};
