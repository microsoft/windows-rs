// Linkage/return macros as they appear in winnt.h / combaseapi.h: defined in a
// header that is NOT part of the emitted namespace (not matched by the filter).
// A function spelled `STDAPI Foo(...)` therefore has its `extern "C"` linkage
// token spelled here, while the function itself is written in the API header.
#define EXTERN_C extern "C"
typedef long HRESULT;
#define STDAPI EXTERN_C HRESULT
