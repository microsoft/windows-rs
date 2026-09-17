//! library kernel32.dll
// No-return functions, exercising DoesNotReturn detection in clang/fn.rs:
//   - __declspec(noreturn) folds into the function type spelling.
//   - _Analysis_noreturn_ (SAL) appears as an annotate attribute child.
// bindgen later lowers the DoesNotReturn attribute to a `-> !` return type.

#define _Analysis_noreturn_ __attribute__((annotate("_Analysis_noreturn_")))

typedef unsigned int UINT;

extern "C" {
    // __declspec(noreturn) -> DoesNotReturn.
    __declspec(noreturn) void ExitNow(UINT code);

    // _Analysis_noreturn_ -> DoesNotReturn.
    _Analysis_noreturn_ void FailFast(UINT code);

    // Ordinary function -> no DoesNotReturn attribute.
    UINT GetCode(void);
}
