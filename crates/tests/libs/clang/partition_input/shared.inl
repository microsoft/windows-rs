// Shared system-header declarations (the `windef.h` analogue): an opaque handle
// via DECLARE_HANDLE plus a pointer-typedef alias. Both partitions `#include`
// this, so each independently *sees* both names.
struct HFOO__ { int unused; };
typedef struct HFOO__ *HFOO;

typedef void *PSHARED;

// A scalar typedef (canonical `long`). In per-header mode it stays a faithful named
// `type LRESULT = i32` in its defining header and resolves by name.
typedef long LRESULT;

// Pointer-sized ABI typedefs. Both the base `ULONG_PTR` (canonical 64-bit integer on
// this target) and the chained alias `SIZE_T` collapse to `usize` at every reference and
// are never emitted as named `type` items — exactly like the fixed-width portability
// aliases (`DWORD` -> u32). They carry no meaning beyond being pointer-sized, so this
// keeps them architecture-neutral (no per-arch `u32`-vs-`usize` split).
typedef unsigned __int64 ULONG_PTR;
typedef ULONG_PTR SIZE_T;
