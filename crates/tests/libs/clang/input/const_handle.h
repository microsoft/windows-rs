// Nested handle/pointer-cast #define constants (parse_nested_cast).
//
// Win32 handle constants cast an integer literal through one or more inner
// scalar casts up to a pointer/handle type. The faithful value is the
// *innermost* scalar reinterpretation: `(LONG)0x80000002` is the signed i32
// -2147483646, which the pointer type's `as _` later sign-extends. Emitting the
// outer unsigned (`ULONG_PTR`) reading instead would zero-extend and produce the
// wrong pointer. A single-cast handle constant (`(HANDLE)0`) is already covered
// by parse_named_cast; these exercise the nested (multi-cast) forms.
typedef long LONG;
typedef __int64 LONG_PTR;
typedef unsigned __int64 ULONG_PTR;
typedef void *HANDLE;

struct HKEY__ {
    int unused;
};
typedef struct HKEY__ *HKEY;

// Double cast with a negated pointer-sized inner literal (INVALID_HANDLE_VALUE).
#define INVALID_HANDLE_VALUE ((HANDLE)(LONG_PTR)-1)

// Triple cast: inner (LONG) reinterprets 0x80000002 as the signed i32
// -2147483646 (HKEY_LOCAL_MACHINE).
#define HKEY_LOCAL_MACHINE ((HKEY)(ULONG_PTR)((LONG)0x80000002))

// Triple cast hitting i32::MIN (HKEY_CLASSES_ROOT).
#define HKEY_CLASSES_ROOT ((HKEY)(ULONG_PTR)((LONG)0x80000000))
