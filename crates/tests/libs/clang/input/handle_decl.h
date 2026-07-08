//! namespace Test
//! library user32.dll

// DECLARE_HANDLE idiom: `typedef struct X__ *X` has no structural payload beyond
// identity, so each handle must collapse to an opaque `*mut void` (a NativeTypedef
// handle, matching the hand-authored Foundation::HANDLE) — never a pointer to the
// one-off `X__` tag struct, which is not emitted at all.
#include "handle_decl_inc.inl"

HWND CreateThing(HINSTANCE instance);

// MIDL-generated value structs use the same `name##__` tag convention as
// DECLARE_HANDLE, but as a *value* typedef with a real field (not a `*` pointer to an
// `{ int unused; }` dummy). These are NOT handle tags and must be emitted under their
// public (de-underscored) name so references to them resolve.
typedef struct THING_INFO0__ {
    HWND owner;
} THING_INFO0;

THING_INFO0 GetThingInfo(void);
