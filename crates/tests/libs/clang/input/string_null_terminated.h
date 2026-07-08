//! flat
//! namespace Windows.Win32
// Item 5: a *raw* null-terminated string parameter (`_In_z_ WCHAR const*`, with no named
// `LP*` alias) is promoted to the canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` wrapper from
// the `_z_` SAL bit, so bindgen's ergonomic string projection applies just as it does for
// the *named* aliases (`flat_string_alias.h`). Wide (`WCHAR` = u16) picks `P*WSTR`; narrow
// (`char`) picks `P*STR`; SAL direction picks const (`_In_`) vs mut (`_Out_`/`_Inout_`).
//
// The promotion is gated strictly on the `_z_` bit: a *counted* wide buffer
// (`_In_reads_`, non-`z`) carries a `NativeArrayInfo` and must stay a raw `*const u16`
// (the `CreateTextLayout` shape). See ledger #5.

#define _In_z_ __attribute__((annotate("_In_z_")))
#define _In_opt_z_ __attribute__((annotate("_In_opt_z_")))
#define _Out_z_ __attribute__((annotate("_Out_z_")))
#define _Inout_z_ __attribute__((annotate("_Inout_z_")))
#define _In_reads_(c) __attribute__((annotate("_In_reads_(" #c ")")))

typedef unsigned short WCHAR;

// _In_z_ WCHAR const* -> PCWSTR (wide, read-only).
void SetName(_In_z_ WCHAR const* name);

// _In_z_ char const* -> PCSTR (narrow, read-only).
void SetNameAnsi(_In_z_ char const* name);

// _Out_z_ WCHAR* -> PWSTR (wide, writable; #[out] is explicit because the promoted
// wrapper is a by-value type whose inferred direction is In).
void GetName(_Out_z_ WCHAR* buffer);

// _Inout_z_ WCHAR* -> PWSTR (wide, writable; #[r#in] #[out]).
void EditName(_Inout_z_ WCHAR* buffer);

// _In_opt_z_ WCHAR const* -> PCWSTR + #[opt].
void SetNameOpt(_In_opt_z_ WCHAR const* name);

// _In_z_ on an already-mutable `WCHAR*`: SAL In makes it read-only -> PCWSTR.
void SetNameMut(_In_z_ WCHAR* name);

// Counted (non-`z`) wide buffer: stays a raw `*const u16` with NativeArrayInfo.
void ReadChars(_In_reads_(count) WCHAR const* chars, unsigned long count);
