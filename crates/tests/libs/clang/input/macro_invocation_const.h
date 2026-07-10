//! flat
// Regression: an object-like macro whose replacement list *invokes* a function-like
// macro (here `ARRAYSIZE`) must not be misparsed by `parse_nested_cast` as a
// `(TYPE)value` cast — `ARRAYSIZE(WIDE_PREFIX)` is a macro call, not a cast to a type
// named `ARRAYSIZE`. It must fall through to the batch evaluator, which computes the
// real integer value. Mirrors `VOLUME_PREFIX`/`VOLUME_PREFIX_LEN` in the SDK's pathcch.h,
// which previously emitted a bogus `const VOLUME_PREFIX_LEN: ARRAYSIZE = -1`.
#define ARRAYSIZE(A) (sizeof(A) / sizeof((A)[0]))
#define WIDE_PREFIX L"\\\\?\\Volume"
#define WIDE_PREFIX_LEN (ARRAYSIZE(WIDE_PREFIX) - 1)
