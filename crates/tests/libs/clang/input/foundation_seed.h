//! namespace Windows.Win32.System.Memory
//! library test.dll
//! reference foundation_seed

// A leaf namespace built with NO upstream Windows.Win32.winmd — only the small
// hand-authored Foundation seed as reference. The semantic scalars (BOOL, HRESULT,
// LPARAM/WPARAM/LRESULT, NTSTATUS, BOOLEAN) and the opaque HANDLE must resolve to the
// named Foundation types; the universal C fundamentals (DWORD, UINT) must collapse to
// u32 and a raw `void *` must stay `*mut void` (the seed names HANDLE specifically,
// not every void pointer). Neither kind leaks a per-namespace `type` alias here.
#include "foundation_seed_inc.inl"

HRESULT DoThing(BOOL enabled, DWORD count, UINT index, LPARAM lp, WPARAM wp, LRESULT lr, NTSTATUS status, BOOLEAN flag);
HANDLE OpenThing(void *reserved);
