// #define constants that cast to C builtin type keywords (`int`, `long`), the
// form used by e.g. `CW_USEDEFAULT`. These are `CXToken_Keyword` casts (not the
// Win32 typedef identifier casts), so they exercise the keyword-cast path.
#define CW_USEDEFAULT ((int)0x80000000)
#define INT_CAST ((int)5)
#define LONG_MIN_STYLE ((long)0x80000000)
