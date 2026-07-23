// A macro defined with a 128-bit integer literal (the MSVC `i128`/`ui128` suffix, which clang
// accepts under `-fms-extensions`) is dropped. The metadata `Value` type has no 128-bit variant,
// and `clang_getEnumConstantDeclValue` returns a 64-bit `long long`, so such a constant would be
// silently truncated to a wrong value (`INT128_MAX` -> `-1`). The scraper rejects any macro whose
// body contains an integer literal ending in the `i128`/`ui128` suffix before it reaches the
// batch evaluator, uniformly across architectures (`__int128` does not even exist on 32-bit x86).
//
// The three `DROP_*` macros below are the real SDK `intsafe.h` definitions of
// `INT128_MAX`/`INT128_MIN`/`UINT128_MAX`, which clang 21 accepts (clang 18 rejected them).
// Scalar 64-bit macros are unaffected.
#define KEPT_HEX 0x100
#define KEPT_NEG -7
#define DROP_INT128_MAX 170141183460469231731687303715884105727i128
#define DROP_INT128_MIN (-170141183460469231731687303715884105727i128 - 1)
#define DROP_UINT128_MAX 0xffffffffffffffffffffffffffffffffui128
