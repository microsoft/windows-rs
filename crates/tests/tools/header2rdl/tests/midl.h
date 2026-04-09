/* Test exercising the MIDL-generated COM interface pattern against the
 * Windows SDK macro conventions.
 *
 * The definitions of MIDL_INTERFACE, STDMETHOD, STDMETHOD_, PURE and IUnknown
 * are pulled in from <windows.h>.  In the test suite this resolves to the
 * lightweight shim at tests/include/windows.h, which is added to the include
 * path via the `--include include` sidecar option in midl.h.args.
 */

#include <windows.h>

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
