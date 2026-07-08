//! flat
// A function-*type* typedef (`typedef R (NAME)(args)`, no `*`), then a struct field
// declared as a pointer to it (`NAME *field;`). This is the SSP `SECPKG_FUNCTION_TABLE`
// idiom. Scraped in the flat per-header mode that `tool_win32` uses, the field must
// reference the callback delegate by name (implied pointer, like a function-*pointer*
// typedef field), not collapse to opaque `*mut u8`. A plain `unsigned char *` stays
// `*mut u8`.
typedef int (__stdcall MyCallbackFn)(int a, int b);

struct Table {
    MyCallbackFn *Op;
    unsigned char *Bytes;
};
