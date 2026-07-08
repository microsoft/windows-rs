// A function whose calling-convention macro is written inside a preprocessor
// conditional — the pattern used by `DefWindowProc` in `winuser.h`:
//
//     WINUSERAPI
//     #ifndef _MAC
//     LRESULT
//     WINAPI
//     #else
//     LRESULT
//     CALLBACK
//     #endif
//     DefWindowProcW(...);
//
// `clang_tokenize` is purely lexical, so the directive tokens and the inactive
// `#else` branch land between `WINAPI` and the function name. The convention must
// still be recovered as `extern "system"` (`__stdcall`), not fall back to the
// linkage-based `extern "C"`.
#define WINAPI __stdcall
#define CALLBACK __stdcall
#define WINUSERAPI __declspec(dllimport)

extern "C" {

WINUSERAPI
#ifndef _MAC
int
WINAPI
#else
int
CALLBACK
#endif
SplitConvention(int a, int b);

// A plain adjacent convention still resolves (regression guard).
WINUSERAPI int WINAPI SimpleConvention(int a);

// No convention macro: falls back to the C linkage of the enclosing block.
WINUSERAPI int PlainCdecl(int a);

}
