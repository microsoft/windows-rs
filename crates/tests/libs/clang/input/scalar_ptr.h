//! library kernel32.dll
// C pointer-sized integer typedefs (size_t, intptr_t, ...) collapse to usize/isize
// at reference sites, exactly like the Windows SIZE_T/ULONG_PTR aliases. On a 64-bit
// parse their canonical type is an indistinguishable 64-bit integer, so they are
// recognised by name.

typedef unsigned __int64 size_t;
typedef unsigned __int64 uintptr_t;
typedef __int64 intptr_t;
typedef __int64 ptrdiff_t;

extern "C" {
    void* Alloc(size_t count);
    intptr_t Range(uintptr_t base, ptrdiff_t offset);
}
