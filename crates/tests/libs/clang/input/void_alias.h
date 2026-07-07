//! library kernel32.dll
// Generic void-pointer aliases (`PVOID`/`LPVOID`/`LPCVOID`) are pure portability
// spellings of `void*` with no domain meaning, so they collapse to the raw pointer they
// spell — `*mut void` / `*const void` — at *every* site (parameter, return, struct field,
// and nested pointer), emitting no `type LPVOID = ...` alias. This is the pointer-world
// analog of the `DWORD` -> `u32` collapse.
//
// A `void*` *handle* (`HANDLE`) is structurally identical but semantic: it is kept named
// (`type HANDLE = *mut void`), proving the collapse is name-keyed, not structural.

typedef void *PVOID;
typedef void *LPVOID;
typedef const void *LPCVOID;
typedef void *HANDLE;

struct BUFFER {
    LPVOID data;
    LPCVOID readonly;
    HANDLE owner;
};

extern "C" {
    LPVOID Alloc(unsigned int count, LPCVOID src);
    void Store(PVOID mem, LPVOID *out);
    HANDLE OpenThing(void);
}
