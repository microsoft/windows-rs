//! library user32.dll
// Calling conventions as written in the source. clang folds these away in the
// canonical type on a 64-bit target but preserves the as-written attribute in
// the type spelling, so the source intent is recovered faithfully.

#define WINAPI __stdcall

// The COM/shell `STDAPI` family: a function-like convention macro that carries
// its return type as a macro argument. `STDAPI_(type)` expands to
// `EXTERN_C type STDAPICALLTYPE`, so the convention token sits inside the macro
// body and the call site leaves only the macro's `)` before the function name.
#define STDAPICALLTYPE __stdcall
#define EXTERN_C extern "C"
#define STDAPI EXTERN_C HRESULT STDAPICALLTYPE
#define STDAPI_(type) EXTERN_C type STDAPICALLTYPE

// An export macro whose replacement list carries a `__declspec(dllimport)`
// storage-class specifier ahead of the convention token, mirroring DWrite's
// `#define DWRITE_EXPORT __declspec(dllimport) WINAPI`. The `__declspec(...)` is
// stripped before the small-macro length gate so the `WINAPI` convention survives.
#define DWRITE_EXPORT __declspec(dllimport) WINAPI

typedef long HRESULT;

// STDAPI -> extern "system", returning HRESULT.
STDAPI StdapiFunc(int a);

// EXTERN_C ... DWRITE_EXPORT (__declspec(dllimport) WINAPI) -> extern "system".
EXTERN_C HRESULT DWRITE_EXPORT ExportMacroFunc(int a);

// STDAPI_(type) -> extern "system", returning the macro-argument type.
STDAPI_(int) StdapiTypedFunc(int a);

extern "C" {
    // __stdcall / WINAPI -> extern "system" (the Win32 default convention).
    int WINAPI StdcallFunc(int a);

    // __cdecl -> extern "C".
    int __cdecl CdeclFunc(int a);

    // __fastcall -> extern "fastcall".
    int __fastcall FastcallFunc(int a);

    // No explicit convention in an extern "C" block defaults to __cdecl, so it
    // remains faithful to emit extern "C".
    int PlainFunc(int a);

    // A callback parameter carries its own (__cdecl) convention; the function's
    // own (__stdcall) convention must still be recovered correctly.
    void WINAPI WithCallback(int (__cdecl *callback)(int));
}
