// Pointer-sized typedef-cast #define constants exercising parse_named_cast's
// `pointer_sized_abi` branch. A cast to a pointer-sized ABI alias
// (`ULONG_PTR`/`DWORD_PTR` -> usize, `LONG_PTR` -> isize) collapses to the native
// integer primitive — the same collapse applied at every reference — so sentinel
// constants like `((ULONG_PTR)-1)` are emitted as `usize`/`isize` rather than a
// dangling reference to a `ULONG_PTR` type the flat metadata does not preserve.
typedef unsigned __int64 ULONG_PTR;
typedef unsigned __int64 DWORD_PTR;
typedef __int64 LONG_PTR;
#define PTR_RESET ((ULONG_PTR)-1)
#define PTR_DEFAULT ((DWORD_PTR)-1)
#define PTR_NEG ((LONG_PTR)-1)
#define PTR_VALUE ((ULONG_PTR)42)
