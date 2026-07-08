//! namespace Test
//! library test.dll
//! filter extern_c_fn_api.inl

// A `STDAPI`/`WINAPI` function (i.e. `extern "C" HRESULT Foo(...)`) declared in a
// filtered API header, with the linkage macro defined in a separate unfiltered
// header. Mirrors `shellscalingapi.h`'s `STDAPI GetDpiForMonitor(...)`: the
// wrapping `CXCursor_LinkageSpec` is spelled at the macro site (excluded), so the
// function is only kept if the filter also matches its expansion location.
#include "extern_c_fn_defs.inl"
#include "extern_c_fn_api.inl"
