//! namespace Test
//! library kernel32.dll

// Inline MIDL-style comments name a virtual enum for richer projections (e.g. C#) to
// synthesize, while Rust keeps the loose constants and integer parameter. The comments
// are ordinary C comments, so headers stay source-compatible with every compiler.

#define _In_ __attribute__((annotate("_In_")))

#define FILE_SHARE_READ   /* [enum(FILE_SHARE_MODE)] */ 0x00000001
#define FILE_SHARE_WRITE  /* [enum(FILE_SHARE_MODE)] */ 0x00000002
#define FILE_SHARE_DELETE /* [enum(FILE_SHARE_MODE)] */ 0x00000004

#define PAGE_READONLY  /* [enum(PAGE_PROTECTION)] */ 0x02
#define PAGE_READWRITE /* [enum(PAGE_PROTECTION)] */ 0x04

// Flags enum with SAL present: exercises the SAL-vs-MIDL overlay. `_In_` wins the
// direction choice, yet the enum still rides the comment onto the parameter.
void OpenThing(_In_ unsigned int /* [enum(FILE_SHARE_MODE, flags)] */ dwShareMode);

// Scalar (non-flags) enum, no SAL: resolves via the MIDL branch and emits no Flags.
void SetProtection(unsigned int /* [enum(PAGE_PROTECTION)] */ flProtect);

// Combined MIDL comment: an existing `[in]` direction sits beside the new `[enum]`.
void OpenThing2(unsigned int /* [in] [enum(FILE_SHARE_MODE)] */ dwShareMode);

// Direction and enum in separate adjacent comments around the parameter type.
void OpenThing3(/* [in] */ unsigned int /* [enum(FILE_SHARE_MODE)] */ dwShareMode);
