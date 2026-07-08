//! The single canonicalisation surface: every place a *faithful* C type is rewritten
//! to the *canonical* Win32 metadata type lives here.
//!
//! The scrape is deliberately faithful to the C headers, but a small, explicitly-ordered
//! set of rules recover the semantics the SDK authors intended (that decades of typedef /
//! `#define` conventions erased at the type level). Those rules used to be scattered across
//! `cx.rs` as a long implicit-precedence `if`/`else` chain; collecting them here makes the
//! precedence a declared contract and gives every rule one home. Each rule maps 1:1 onto a
//! row of the "editorial deviations" ledger in `docs/crates/windows-rdl.md`.
//!
//! # The two entry points
//!
//! - [`resolve_typedef`] — resolves a `CXType_Typedef` reference at *any* site (field, return,
//!   parameter). It applies the **universal** collapses (string-wrapper normalise, error-code
//!   domain, fixed-width / floating / pointer-sized scalar collapse, `GUID` synonyms, generic
//!   `void*`) and then falls back to structural resolution (reference-metadata lookup, local
//!   emission, pending-typedef scheduling).
//! - [`param_metadata_type`] — the **parameter-only** overlay, applied on top of a resolved
//!   type: collapse `LP*`/`P*` pointer aliases to raw pointers, apply SAL-driven pointer
//!   const-ness, promote raw null-terminated string pointers to the canonical wrapper, then
//!   re-qualify canonical string wrappers for namespaced scrapes.
//!
//! # Precedence (mirrors the ledger)
//!
//! Universal (in [`resolve_typedef`], flat scrape unless noted): string-wrapper normalise
//! ([`normalize_string_alias`]) → error-code domain ([`error_code_typedef`]) → fixed-width
//! scalar ([`fundamental_scalar`]) → floating ([`floating_typedef`]) → pointer-sized
//! ([`pointer_sized_abi`]) → `GUID` synonym ([`guid_alias`]) → generic `void*`
//! ([`void_pointer_alias`]). Parameter overlay (in [`param_metadata_type`]): pointer-alias
//! collapse ([`collapse_pointer_alias_param`]) → SAL const-ness ([`apply_sal_constness`]) →
//! null-terminated promotion ([`promote_null_terminated_string`]) → namespace re-qualify
//! ([`requalify_string_alias`]).
//!
//! Definition-suppression counterparts live in `typedef.rs` and `const.rs` and must always
//! stay paired with the reference-site collapse here, or a use-site would dangle.

use super::*;

/// Resolve a `CXType_Typedef` reference to its canonical [`metadata::Type`].
///
/// This is the universal type entry point (see the [module docs](self)). `cursor` is the
/// typedef-kinded clang type; the canonicalisation rules are applied in ledger precedence,
/// then structural resolution provides the fallback. Called from [`Type::to_type`] for every
/// field, return and parameter type; parameters then receive the [`param_metadata_type`]
/// overlay.
pub(crate) fn resolve_typedef(cursor: &Type, parser: &mut Parser<'_>) -> metadata::Type {
    let decl = cursor.ty();
    let name = decl.name();
    // A string-pointer alias (`LPCWSTR`, `LPWSTR`, `LPOLESTR`, the `P*`/`PC*`
    // spellings) normalises to its canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR`
    // at *every* site — field, return, or parameter — so bindgen's core string
    // projection applies uniformly (an `LPCWSTR` field would otherwise become a
    // raw `*const u16`, unlike an `LPCWSTR` parameter). SAL later re-selects the
    // const variant for parameters. Universal like `guid_alias`/`void_pointer_alias`
    // below, so it is resolved once here ahead of the mode split. See ledger #5.
    //
    // Gated to the flat per-header scrape: there the four canonical wrappers are
    // defined locally (`winnt.rdl`) in the single root namespace and resolve by
    // name. A *namespaced* scrape (WebView2) instead references an external
    // reference winmd whose const variants (`PCWSTR`/`PCSTR`) are not distinct
    // types (win32metadata models them as `const PWSTR`); there the existing
    // mode-split resolution — `ref_map` for the mutable base, `pending_typedefs`
    // for the local const wrapper, plus the parameter-level `apply_sal_constness`
    // — already yields the canonical spellings, so forcing it here would drop the
    // local definition and leave the reference dangling ("type not found").
    if parser.header_root.is_some()
        && let Some(normalized) = normalize_string_alias(parser.namespace, &name)
    {
        return normalized;
    }
    if parser.header_root.is_some() {
        // Flat namespace, fully faithful: a named typedef is *not*
        // collapsed to its primitive but resolves to the single root
        // namespace by its own name, where it is emitted in its
        // defining-header file (`type HRESULT = i32`, `struct PROPVARIANT`,
        // `type CRM_PROTOCOL_ID = GUID`, …). Whether the typedef is a
        // record's public name or a distinct alias, the reference is by
        // the typedef's name, which resolves within the flat namespace.
        // (`LSTATUS` is the sole remap — the Win32 error-code domain.)
        if let Some(remapped) = error_code_typedef(&name) {
            metadata::Type::value_named(parser.namespace, remapped)
        } else if let Some(scalar) = fundamental_scalar(&name) {
            // A pure fixed-width portability alias (`DWORD` -> u32) carries no
            // semantics beyond its bits; collapse it out of the canon. Every
            // other scalar typedef (`HFILE`, `ATOM`, `COLORREF`, `LRESULT`, …)
            // is preserved by name; see [`fundamental_scalar`].
            scalar
        } else if let Some(scalar) = floating_typedef(cursor) {
            // A floating-point typedef (`FLOAT`, `DOUBLE`, `DATE`, `D3DVALUE`,
            // the OLE/GL/SQL float aliases, chained `UI_ANIMATION_SECONDS` -> …)
            // collapses structurally to `f32`/`f64`. Unlike the *integer* side —
            // where domain names (`HFILE`/`COLORREF`) are byte-identical to the
            // portability spellings and so must be told apart by name (see
            // [`fundamental_scalar`]) — the reference metadata drops *every*
            // floating typedef (zero `NativeTypedef`s with an `f32`/`f64` value),
            // so there is no domain-vs-noise split to preserve and the collapse is
            // decided by canonical kind, not a curated list. See [`floating_typedef`].
            scalar
        } else if let Some(scalar) = pointer_sized_abi(&name) {
            // A pointer-sized ABI alias (`ULONG_PTR`, `SIZE_T`, `LONG_PTR`, …)
            // collapses to `usize`/`isize` just like the fixed-width aliases:
            // it carries no semantics beyond being pointer-sized, and the
            // reference metadata has no such `type` item. Collapsing here (vs.
            // emitting a named alias whose canonical width resolves per-arch)
            // also keeps it architecture-neutral — no spurious x86-vs-64-bit
            // `#[arch]` split. See [`pointer_sized_abi`].
            scalar
        } else if guid_alias(&name) {
            // `IID`/`CLSID`/`FMTID` are `typedef GUID X` synonyms — no distinct
            // ABI, only a documentation name. Collapse them to `GUID` rather than
            // emitting redundant `type IID = GUID` aliases, matching the reference
            // metadata and enabling the ergonomic `QueryInterface<T>()`/
            // `Resolve<T>()` COM projection. See [`guid_alias`]; mirrors the
            // `fundamental_scalar` (`DWORD` -> u32) collapse.
            metadata::Type::value_named(parser.namespace, "GUID")
        } else if let Some(ptr) = void_pointer_alias(&name) {
            // `PVOID`/`LPVOID`/`LPCVOID`/... are generic void-pointer portability
            // spellings with no domain meaning; collapse them to the raw
            // `*mut void` / `*const void` they spell at every site (matching the
            // reference metadata's bare `*mut c_void`), rather than emitting a
            // named alias. The pointer-world analog of the `DWORD` -> u32 collapse;
            // a `void*` *handle* (`HANDLE`) is excluded and stays named. See
            // [`void_pointer_alias`].
            ptr
        } else if let Some(base) = d2d_compat_alias(&name) {
            // `D2D1_POINT_2F`/`D2D1_RECT_F`/... are `typedef D2D_X D2D1_X`
            // compatibility synonyms: the Direct2D 1.1 headers re-export the shared
            // `D2D_*` primitives (`dcommon.h`, `d2d1_1.h`) under a `D2D1_`-prefixed
            // spelling with no distinct ABI. Collapse each to its `D2D_` base at
            // every reference — matching the reference metadata, which carries no
            // `D2D1_*` alias layer — so the shared primitive is the single referent.
            // The numerics-mapped members (`D2D_POINT_2F` -> `Vector2`,
            // `D2D_MATRIX_3X2_F` -> `Matrix3x2`, ...) then resolve through bindgen's
            // one substitution table instead of a redundant alias, and the plain
            // ones (`D2D_RECT_F`, `D2D_SIZE_F`, ...) resolve to the shared struct.
            // See [`d2d_compat_alias`].
            metadata::Type::value_named(parser.namespace, base)
        } else if is_interface_alias(&decl.typedef_underlying_type()) {
            // `LPSTORAGE`/`LPOLEOBJECT`/`LPDIRECTDRAWSURFACE`/... are `typedef IFoo *NAME`
            // (or the rarer `typedef IFoo NAME`) aliases to a COM interface. Interfaces are
            // implied pointers in Windows metadata, so the alias carries no distinct ABI —
            // collapse every reference to the interface itself, matching the reference
            // metadata, which omits these aliases and types the field/parameter as the
            // interface directly. Emitting the alias would otherwise surface an
            // interface-by-value field (no `Default`), breaking bindgen's struct derive.
            // See [`is_interface_alias`]; definition suppressed in `typedef.rs`.
            let underlying = decl.typedef_underlying_type();
            if underlying.is_interface() {
                underlying.to_type(parser)
            } else {
                underlying.pointee_type().to_type(parser)
            }
        } else {
            metadata::Type::value_named(parser.namespace, &name)
        }
    } else if let Some(ns) = parser.ref_map.get(&name) {
        // Type is known in the reference metadata — use the qualified name.
        metadata::Type::value_named(ns, &name)
    } else if guid_alias(&name) {
        // `IID`/`CLSID`/`FMTID` collapse to `GUID` in every mode (see the
        // header_root branch above and [`guid_alias`]).
        metadata::Type::value_named(parser.namespace, "GUID")
    } else if let Some(ptr) = void_pointer_alias(&name) {
        // Generic void-pointer aliases collapse to `*mut`/`*const void` in every
        // mode (see the header_root branch above and [`void_pointer_alias`]).
        ptr
    } else if let Some(scalar) = collapse_scalar_typedef(&name, cursor) {
        // A fundamental scalar typedef that the reference does not preserve
        // as a distinct type (DWORD -> u32, UINT -> u32, ULONG_PTR -> usize,
        // ...). Collapse to the primitive so it is not duplicated as a
        // per-namespace `type` alias, matching the canonical Win32 metadata
        // (which has no `DWORD`/`UINT`/`ULONG_PTR` types).
        scalar
    } else if decl.is_from_main_file() {
        // Local typedef — it will be emitted separately as a `type` item.
        metadata::Type::value_named(parser.namespace, &name)
    } else {
        // A non-scalar typedef from an included/system header that is not in
        // the reference (e.g. a handle struct): preserve the name and
        // schedule its definition for emission in a follow-up pass.
        parser.pending_typedefs.push(decl);
        metadata::Type::value_named(parser.namespace, &name)
    }
}

/// Whether a typedef's `underlying` type aliases a COM interface — either the direct
/// `typedef IFoo NAME` or the common `typedef IFoo *NAME` (`LP*`/`P*`) spelling.
///
/// COM interfaces are implied pointers in Windows metadata, so such an alias carries
/// no distinct ABI; [`resolve_typedef`] collapses every reference to the interface and
/// `typedef.rs` suppresses the redundant definition, matching the reference metadata
/// (which omits the alias and types the field/parameter as the interface directly).
pub(crate) fn is_interface_alias(underlying: &Type) -> bool {
    underlying.is_interface()
        || (underlying.kind() == CXType_Pointer && underlying.pointee_type().is_interface())
}

/// Collapse a fundamental scalar typedef to its primitive [`metadata::Type`].
///
/// Returns `Some` when `name`/`ty` is a C scalar typedef that the canonical Win32
/// metadata does not preserve as a distinct named type, so it should be emitted as
/// the underlying primitive rather than a per-namespace `type` alias.
///
/// Pointer-sized integer typedefs from `basetsd.h` (`ULONG_PTR`, `SIZE_T`, ...) and
/// the C runtime (`size_t`, `intptr_t`, `ptrdiff_t`, ...) are `usize`/`isize` in the
/// Windows ABI, but on a 64-bit parse their canonical type is an indistinguishable
/// 64-bit integer; they are recognised by name (a small, ABI-defined set) so they are
/// not mistaken for `u64`/`i64`. Every other typedef is
/// collapsed only when its fully-resolved canonical type is a builtin scalar, so
/// handle/pointer/record typedefs are left untouched.
///
/// Callers must check the reference metadata first: a scalar typedef the reference
/// *does* preserve (e.g. `HRESULT`, `BOOL`) must resolve to that type, not collapse.
fn collapse_scalar_typedef(name: &str, ty: &Type) -> Option<metadata::Type> {
    if let Some(scalar) = pointer_sized_abi(name) {
        return Some(scalar);
    }

    let canonical = ty.canonical_type();
    is_fundamental_scalar_kind(canonical.kind()).then(|| scalar_kind_to_type(canonical.kind()))
}

/// Collapse a typedef whose fully-resolved canonical type is a floating-point primitive
/// to the bare Rust primitive (`float`/`double`/`long double` -> `f32`/`f64`).
///
/// Where the *integer* portability aliases need a curated name list (see
/// [`fundamental_scalar`]) — because domain typedefs like `HFILE`/`COLORREF` are
/// byte-for-byte identical to `DWORD`/`WORD` and only their *name* marks them as worth
/// keeping — the floating side has no such distinction: the reference metadata drops
/// **every** floating typedef (there are zero `NativeTypedef`s with an `f32`/`f64` value —
/// `FLOAT`, `DOUBLE`, `DATE`, `REFTIME`, the OLE/GL/SQL float aliases, …). So the collapse
/// is structural, keyed on clang's canonical kind, which also transparently handles chained
/// aliases (`UI_ANIMATION_SECONDS` -> `DOUBLE` -> `double`). MSVC `long double` is 64-bit,
/// matching the direct-`double` mapping in [`Type::to_type`].
///
/// The same predicate backs the definition-emission skip in [`Typedef::parse`], so a
/// floating typedef is never emitted as a redundant `type FLOAT = f32` item.
pub(crate) fn floating_typedef(ty: &Type) -> Option<metadata::Type> {
    match ty.canonical_type().kind() {
        CXType_Float => Some(metadata::Type::F32),
        CXType_Double | CXType_LongDouble => Some(metadata::Type::F64),
        _ => None,
    }
}

/// The fundamental C builtin scalar kinds (`unsigned long`, `int`, `wchar_t`, `float`, …)
/// that map to a Rust primitive via [`scalar_kind_to_type`].
fn is_fundamental_scalar_kind(kind: CXTypeKind) -> bool {
    matches!(
        kind,
        CXType_Bool
            | CXType_Char_U
            | CXType_UChar
            | CXType_UShort
            | CXType_UInt
            | CXType_ULong
            | CXType_ULongLong
            | CXType_Char_S
            | CXType_SChar
            | CXType_Short
            | CXType_Int
            | CXType_Long
            | CXType_LongLong
            | CXType_Float
            | CXType_Double
            | CXType_WChar
            | CXType_Char16
            | CXType_Char32
    )
}

/// The pure fixed-width integer/char *portability* typedefs — the C and Win32 spellings
/// that name "an integer of N bits" and nothing more (`DWORD`, `WORD`, `LONG`, `UINT`,
/// the fixed-width `INTn`/`UINTn`, the C99 `intN_t`/`uintN_t`, `WCHAR`, `CHAR`, …). Each
/// maps to the Rust primitive of its width; `windows-bindgen` treats `DWORD` and `u32`
/// identically, so the alias is pure noise and collapses at every use site (emitting no
/// `type` item).
///
/// This is a **curated collapse-list**, and scalar typedefs are *preserved by default* —
/// only the names listed here collapse. The determination cannot be structural: `HFILE`
/// (`typedef int`), `ATOM` (`typedef WORD`), and `COLORREF` (`typedef DWORD`) are
/// byte-for-byte identical to `DWORD`/`WORD` at the type level, so only the *name*
/// distinguishes a meaningful domain type — kept as `type HFILE = i32`, `type ATOM = u16`,
/// `type COLORREF = u32` — from pure portability noise. Pointer-sized aliases
/// ([`pointer_sized_abi`]) and the error-code domain ([`error_code_typedef`]) are handled
/// separately; every remaining scalar typedef (`HRESULT`, `BOOL`, `BOOLEAN`, `NTSTATUS`,
/// the domain handles/ids, …) is preserved by name.
///
/// The same list backs the const-cast collapse in [`parse_named_cast`], so a typedef and
/// any constant typed by it never disagree (e.g. `const HFILE_ERROR: HFILE`,
/// `const INVALID_ATOM: ATOM` — both resolve because `HFILE`/`ATOM` stay named).
pub(crate) fn fundamental_scalar(name: &str) -> Option<metadata::Type> {
    Some(match name {
        "BYTE" | "UCHAR" | "UINT8" | "uint8_t" => metadata::Type::U8,
        "WORD" | "USHORT" | "WCHAR" | "UINT16" | "uint16_t" => metadata::Type::U16,
        "DWORD" | "UINT" | "ULONG" | "DWORD32" | "UINT32" | "ULONG32" | "uint32_t" => {
            metadata::Type::U32
        }
        "QWORD" | "ULONGLONG" | "DWORD64" | "UINT64" | "ULONG64" | "uint64_t" => {
            metadata::Type::U64
        }
        "CHAR" | "INT8" | "int8_t" => metadata::Type::I8,
        "SHORT" | "INT16" | "int16_t" => metadata::Type::I16,
        "INT" | "LONG" | "INT32" | "LONG32" | "int32_t" => metadata::Type::I32,
        "LONGLONG" | "INT64" | "LONG64" | "int64_t" => metadata::Type::I64,
        _ => return None,
    })
}

/// Recognises the `GUID` synonym typedefs (`IID`, `CLSID`, `FMTID`, `UUID`) — each a
/// `typedef GUID X` with no distinct ABI or semantics beyond a documentation-only name.
/// They collapse to the `GUID` struct itself (which the `windows` projection maps to the
/// `System.Guid` intrinsic), exactly as [`fundamental_scalar`] collapses `DWORD` -> `u32`:
/// the redundant `type IID = GUID` alias is dropped and every reference resolves straight
/// to `GUID`. This matches the reference metadata (which carries no `IID`/`CLSID`/`FMTID`/
/// `UUID` types) and lets bindgen apply the ergonomic `QueryInterface<T>()` / `Resolve<T>()`
/// COM projection, which keys off a `*const GUID` companion to the `ComOutPtr` out-param.
///
/// `REFIID`/`REFCLSID`/`REFGUID` are the `const *` forms; they collapse structurally to
/// `*const GUID` once their `IID`/`CLSID` pointee resolves through here.
pub(crate) fn guid_alias(name: &str) -> bool {
    matches!(name, "IID" | "CLSID" | "FMTID" | "UUID")
}

/// Recognises the generic `void*` portability aliases (`PVOID`, `LPVOID`, `LPCVOID`, …)
/// and returns the raw pointer they spell — `*mut void` for the writable spellings,
/// `*const void` for the `const void*` ones. These carry no domain meaning beyond
/// "a void pointer": they are the pointer-world analog of [`fundamental_scalar`]'s
/// `DWORD` -> `u32` (a pure spelling of `unsigned long`), so they collapse at **every**
/// use site — parameter, return, field, or nested pointer — and no `type LPVOID = …`
/// alias item is emitted, matching the reference metadata (which has a bare `*mut c_void`
/// everywhere and no such alias types).
///
/// The collapse is name-keyed rather than structural because a `void*` *handle*
/// (`HANDLE`) is structurally identical but **semantic** — it stays named. That is the
/// same `HFILE`-vs-`DWORD` judgement [`fundamental_scalar`] makes: only the generic-void
/// spellings are listed here; handles are excluded and kept named by the structural
/// handle rule.
pub(crate) fn void_pointer_alias(name: &str) -> Option<metadata::Type> {
    Some(match name {
        "PVOID" | "LPVOID" | "PVOID64" => metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1),
        "LPCVOID" | "PCVOID" | "LPCTVOID" => {
            metadata::Type::PtrConst(Box::new(metadata::Type::Void), 1)
        }
        _ => return None,
    })
}

/// Recognises the Direct2D 1.1 `typedef D2D_X D2D1_X` compatibility synonyms and
/// returns the shared `D2D_*` base spelling.
///
/// `dcommon.h` and `d2d1_1.h` re-export the primitive geometry/colour aggregates
/// (`D2D_POINT_2F`, `D2D_RECT_F`, `D2D_MATRIX_3X2_F`, `D2D_COLOR_F`, …) under a
/// `D2D1_`-prefixed spelling for source compatibility, with no distinct ABI. These
/// collapse to their base at every reference — matching the reference metadata,
/// which carries no `D2D1_*` alias layer — so the shared primitive is the single
/// referent (its `type D2D1_* = D2D_*` alias item is suppressed in [`Typedef::parse`]).
/// This also lets the numerics-mapped members reach bindgen's one substitution table
/// under their canonical `D2D_*` name (`D2D_POINT_2F` -> `Vector2`, `D2D_MATRIX_3X2_F`
/// -> `Matrix3x2`, …) rather than through a redundant alias, while the plain members
/// (`D2D_RECT_F`, `D2D_SIZE_F`, `D2D_POINT_2L`, …) resolve to the shared struct.
///
/// Curated by name like [`guid_alias`]/[`void_pointer_alias`]: only the fixed set of
/// `D2D1_`-prefixed spellings that are genuine `D2D_*` renames collapse — a `D2D1_`
/// type without a shared base (`D2D1_TAG`, `D2D1_PIXEL_FORMAT`, the enums) is preserved.
pub(crate) fn d2d_compat_alias(name: &str) -> Option<&'static str> {
    Some(match name {
        "D2D1_COLOR_F" => "D2D_COLOR_F",
        "D2D1_MATRIX_3X2_F" => "D2D_MATRIX_3X2_F",
        "D2D1_MATRIX_4X3_F" => "D2D_MATRIX_4X3_F",
        "D2D1_MATRIX_4X4_F" => "D2D_MATRIX_4X4_F",
        "D2D1_MATRIX_5X4_F" => "D2D_MATRIX_5X4_F",
        "D2D1_POINT_2F" => "D2D_POINT_2F",
        "D2D1_POINT_2L" => "D2D_POINT_2L",
        "D2D1_POINT_2U" => "D2D_POINT_2U",
        "D2D1_RECT_F" => "D2D_RECT_F",
        "D2D1_RECT_L" => "D2D_RECT_L",
        "D2D1_RECT_U" => "D2D_RECT_U",
        "D2D1_SIZE_F" => "D2D_SIZE_F",
        "D2D1_SIZE_U" => "D2D_SIZE_U",
        "D2D1_VECTOR_2F" => "D2D_VECTOR_2F",
        "D2D1_VECTOR_3F" => "D2D_VECTOR_3F",
        "D2D1_VECTOR_4F" => "D2D_VECTOR_4F",
        _ => return None,
    })
}

/// Recognises a pointer-sized integer typedef by its ABI-defined name and returns
/// `usize`/`isize`. Covers both the Windows aliases from `basetsd.h` (`ULONG_PTR`,
/// `UINT_PTR`, `LONG_PTR`, `SIZE_T`, ...) and the C-runtime equivalents (`size_t`,
/// `uintptr_t`, `intptr_t`, `ptrdiff_t`). On a single 64-bit parse their canonical
/// type is an indistinguishable 64-bit integer, so the name is the only signal that
/// they are pointer-sized — recognising it keeps them architecture-neutral (matching
/// the canonical Win32 metadata) instead of being frozen to `u64`/`i64`.
///
/// In flat/per-header mode these names **collapse** exactly like [`fundamental_scalar`]:
/// every reference resolves to `usize`/`isize` and no `type ULONG_PTR = …` item is
/// emitted (matching the reference metadata, which has no such types). Collapsing is
/// what makes them architecture-neutral in a multi-arch scrape — emitting a named alias
/// instead would freeze the width per-arch (`u32` on x86, `usize` on 64-bit) and
/// arch-merge would then split it under a spurious `#[arch]` gate.
pub(crate) fn pointer_sized_abi(name: &str) -> Option<metadata::Type> {
    match name {
        "UINT_PTR" | "ULONG_PTR" | "DWORD_PTR" | "SIZE_T" | "size_t" | "rsize_t" | "uintptr_t" => {
            Some(metadata::Type::USize)
        }
        "INT_PTR" | "LONG_PTR" | "SSIZE_T" | "intptr_t" | "ptrdiff_t" => {
            Some(metadata::Type::ISize)
        }
        _ => None,
    }
}

/// Remaps `LSTATUS` to the Win32 error-code domain type `WIN32_ERROR`.
///
/// `typedef LONG LSTATUS` is, at the C level, a signed 32-bit integer, but it is
/// *definitionally* a Win32 error code — the registry (`Reg*`) and `shlwapi` APIs
/// return `ERROR_*` values through it, exactly the unsigned domain the `WIN32_ERROR`
/// type (and the `ERROR_*` constants, now `u32`) models. Faithful `i32` would clash
/// with every consumer that treats the result as an unsigned error code. So the name
/// is remapped to `WIN32_ERROR` (a plain `type WIN32_ERROR = u32`) at both its
/// definition ([`Typedef::parse`]) and every reference (`to_type`), matching the
/// reference metadata's `WIN32_ERROR` without adopting win32metadata's giant synthetic
/// error *enum*. This is domain typing keyed on one canonical SDK typedef, not
/// per-symbol curation.
pub(crate) fn error_code_typedef(name: &str) -> Option<&'static str> {
    (name == "LSTATUS").then_some("WIN32_ERROR")
}

/// The single source of truth for the handful of parameter aliases whose treatment
/// **cannot be decided structurally** and so must be keyed on the alias's spelling.
///
/// Structure alone is ambiguous for these: a `void*` *handle* (`HANDLE`) and a `void*`
/// *data pointer* (`PVOID`) are the same C type; `BSTR` and `LPCWSTR` are both
/// `wchar_t*`. Everything **not** listed in [`alias_policy`] is decided purely
/// structurally by [`collapse_pointer_alias_param`] (interface / handle-tag /
/// function-pointer / raw-pointer collapse) and needs no name entry — that keeps the
/// name-based editorialising to this one small, auditable table rather than the scattered
/// per-type mappings that make external metadata unmaintainable.
#[derive(Clone, Copy)]
enum AliasPolicy {
    /// A canonical string-pointer wrapper, kept *named* so the `windows` projection can
    /// map it to an ergonomic string type (rather than a raw `*mut u16` / `*const i8`).
    /// `canonical` is the variant this exact spelling denotes; `mut_name`/`const_name` are
    /// the writable/read-only variants SAL selects between (`PWSTR`↔`PCWSTR`,
    /// `PSTR`↔`PCSTR`). See ledger #4/#5.
    String {
        canonical: &'static str,
        mut_name: &'static str,
        const_name: &'static str,
    },
    /// Kept named verbatim, never collapsed. `BSTR` is a length-prefixed,
    /// `SysAllocString`-owned COM string, not a bare `OLECHAR*`. See ledger #5.
    KeepNamed,
}

/// Look up the name-keyed [`AliasPolicy`] for a parameter alias, or `None` to fall through
/// to the structural rules. This is the one place the string wrappers and `BSTR` are
/// enumerated. (Generic `void*` aliases are handled earlier via [`void_pointer_alias`],
/// which collapses them at every site, not just parameters.)
fn alias_policy(name: &str) -> Option<AliasPolicy> {
    const WIDE: (&str, &str) = ("PWSTR", "PCWSTR");
    const NARROW: (&str, &str) = ("PSTR", "PCSTR");
    Some(match name {
        "LPWSTR" | "PWSTR" => AliasPolicy::String {
            canonical: WIDE.0,
            mut_name: WIDE.0,
            const_name: WIDE.1,
        },
        "LPCWSTR" | "PCWSTR" => AliasPolicy::String {
            canonical: WIDE.1,
            mut_name: WIDE.0,
            const_name: WIDE.1,
        },
        "LPSTR" | "PSTR" => AliasPolicy::String {
            canonical: NARROW.0,
            mut_name: NARROW.0,
            const_name: NARROW.1,
        },
        "LPCSTR" | "PCSTR" => AliasPolicy::String {
            canonical: NARROW.1,
            mut_name: NARROW.0,
            const_name: NARROW.1,
        },
        // The OLE string family (`OLECHAR` = `wchar_t`): same wide wrappers as `LP*WSTR`.
        "LPOLESTR" | "POLESTR" => AliasPolicy::String {
            canonical: WIDE.0,
            mut_name: WIDE.0,
            const_name: WIDE.1,
        },
        "LPCOLESTR" | "PCOLESTR" => AliasPolicy::String {
            canonical: WIDE.1,
            mut_name: WIDE.0,
            const_name: WIDE.1,
        },
        "BSTR" => AliasPolicy::KeepNamed,
        _ => return None,
    })
}

/// The canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` spelling a string-pointer *alias*
/// normalises to, or `None` for anything that is not a collapsible string wrapper
/// (`BSTR`, being [`AliasPolicy::KeepNamed`], and every non-string alias return `None`).
///
/// Const-ness follows the alias's own spelling — `LPCWSTR`/`PCWSTR`/`LPCOLESTR`
/// denote the read-only `PCWSTR`; `LPWSTR`/`PWSTR`/`LPOLESTR` the writable `PWSTR`.
/// A name that already *is* its canonical spelling (`PCWSTR`) maps to itself; callers
/// use `canonical != name` to tell a redundant alias (`LPCWSTR`) apart from the four
/// canonical wrappers that must be preserved. See ledger #5.
pub(crate) fn string_alias_canonical(name: &str) -> Option<&'static str> {
    match alias_policy(name) {
        Some(AliasPolicy::String { canonical, .. }) => Some(canonical),
        _ => None,
    }
}

/// Normalise a string-pointer alias *reference* to its canonical `PWSTR`/`PCWSTR`/
/// `PSTR`/`PCSTR` value type, or `None` to leave the type unchanged.
///
/// Applied at **every** reference site — parameter, struct field, or return — from
/// [`Type::to_type`], so the whole corpus speaks the four canonical spellings that
/// bindgen's core string projection recognises (`LPCWSTR` fields would otherwise
/// degrade to a raw `*const u16`, unlike `LPCWSTR` parameters). SAL const-ness is
/// re-applied for parameters afterwards ([`apply_sal_constness`]); the redundant
/// alias *definitions* are suppressed in `typedef.rs` via [`string_alias_canonical`].
fn normalize_string_alias(namespace: &str, name: &str) -> Option<metadata::Type> {
    string_alias_canonical(name).map(|canonical| metadata::Type::value_named(namespace, canonical))
}

/// Decay a C array parameter to a pointer (C11 §6.7.6.3p7). [`Type::to_type`] maps both
/// `T[]` and `T[N]` to `ArrayFixed` (or, for a *named* array typedef such as `UVersionInfo`
/// = `[u8; 4]`, to the alias) — faithful for a *struct field*, but wrong for a parameter (a
/// by-value array is not a real ABI: it is FFI-unsafe and an unsized `[T; 0]` is a zero-byte
/// value that drops the argument and corrupts the call). Every array parameter — whether
/// spelled inline or reached through a typedef — decays to a pointer to its element,
/// matching the reference (win32metadata), whose ABI carries no by-value array parameter:
/// even a plain fixed-size buffer (`FLOAT ColorRGBA[4]`) is a pointer plus a
/// `NativeArrayInfo(CountConst = N)` attribute, and bindgen reconstructs the length-checked
/// `&[T; N]` from the count in the *safe* wrapper only (see [`inline_array_param_count`], and
/// ledger #13). A typedef array (`UVersionInfo`) decays to a bare element pointer with no
/// count, exactly as the reference does.
///
/// The pointee const-ness follows the array element's C const-ness; SAL direction may
/// override it in [`apply_sal_constness`].
fn decay_array_param(
    cursor_ty: &Type,
    base: metadata::Type,
    parser: &mut Parser<'_>,
) -> metadata::Type {
    let canonical = cursor_ty.canonical_type();
    if !matches!(
        canonical.kind(),
        CXType_ConstantArray | CXType_IncompleteArray
    ) {
        return base;
    }
    // Prefer the already-resolved `ArrayFixed` element (preserves inline element aliases);
    // for a *named* array typedef the base is the alias, so resolve the element from the
    // canonical array instead (e.g. `UVersionInfo` -> `u8`), matching the reference.
    let element = match base {
        metadata::Type::ArrayFixed(element, _size) => *element,
        _ => canonical.array_element_type().to_type(parser),
    };
    let is_const = canonical.array_element_type().is_const();
    if is_const {
        match element {
            metadata::Type::PtrConst(t, n) => metadata::Type::PtrConst(t, n + 1),
            other => metadata::Type::PtrConst(Box::new(other), 1),
        }
    } else {
        match element {
            metadata::Type::PtrMut(t, n) => metadata::Type::PtrMut(t, n + 1),
            other => metadata::Type::PtrMut(Box::new(other), 1),
        }
    }
}

/// Resolve a parameter's metadata type: decay array parameters to pointers
/// ([`decay_array_param`]), collapse `LP*`/`P*` pointer aliases to raw pointers
/// ([`collapse_pointer_alias_param`], ledger #5) then apply SAL-driven pointer const-ness
/// ([`apply_sal_constness`], ledger #4). Fields, returns and constants keep their named
/// aliases and array shapes; only parameters are collapsed and decayed.
pub(crate) fn param_metadata_type(
    cursor_ty: &Type,
    annotation: &ParamAnnotation,
    parser: &mut Parser<'_>,
) -> metadata::Type {
    let base = cursor_ty.to_type(parser);
    let base = decay_array_param(cursor_ty, base, parser);
    let base = collapse_pointer_alias_param(cursor_ty, base, parser);
    let ty = apply_sal_constness(base, annotation);
    let ty = normalize_pointer_const_chain(ty);
    let ty = promote_null_terminated_string(ty, annotation, parser);
    requalify_string_alias(ty, parser)
}

/// The `CountConst` for an *inline* fixed-size array parameter (`T name[N]`), which the
/// reference metadata records as `NativeArrayInfo(CountConst = N)` alongside the decayed
/// pointer so bindgen reconstructs a length-checked `&[T; N]` in the safe wrapper. Returns
/// `None` for typedef arrays (e.g. `UVersionInfo`, whose length lives on the typedef, not
/// the parameter), unsized arrays, and non-arrays — all of which the reference decays to a
/// bare pointer with no count. See [`decay_array_param`].
pub(crate) fn inline_array_param_count(cursor_ty: &Type) -> Option<i32> {
    if cursor_ty.kind() != CXType_ConstantArray {
        return None;
    }
    let size = cursor_ty.array_size();
    (size > 0).then_some(size as i32)
}

/// Collapse a mixed-constness pointer chain (`*mut *const T`, `*const *mut T`) to a
/// uniform chain governed by its *outermost* level.
///
/// The winmd `Type` model stores an entire pointer run as a single const bit plus a depth
/// (`PtrMut(leaf, n)` / `PtrConst(leaf, n)`), so it structurally cannot represent a chain
/// whose levels differ in const-ness: serialising a `PtrMut(PtrConst(T))` silently corrupts
/// it on the winmd round-trip (the inner `IsConst` modifier, no longer at the front of the
/// signature, is misread and the run degrades to `*const *const T`). A pointer-to-const-pointer
/// output parameter (`_Out_`/`[retval] const wchar_t **`) would then project to a `*const *const`
/// the callee cannot write through. The outermost level carries the parameter's real read/write
/// direction (already set by [`apply_sal_constness`]), so it governs the whole chain: an output
/// `const wchar_t **` becomes `*mut *mut u16` (matching the canonical projection) and an input
/// `const wchar_t * const *` stays `*const *const u16`. Uniform chains are already collapsed to a
/// single node by [`Type::to_type`]'s pointer const-flattening, so only a genuinely mixed chain
/// nests here. See ledger #14.
fn normalize_pointer_const_chain(ty: metadata::Type) -> metadata::Type {
    fn flatten(inner: metadata::Type, depth: usize) -> (metadata::Type, usize) {
        match inner {
            metadata::Type::PtrMut(deeper, n) | metadata::Type::PtrConst(deeper, n) => {
                flatten(*deeper, depth + n)
            }
            leaf => (leaf, depth),
        }
    }
    match ty {
        metadata::Type::PtrMut(inner, n)
            if matches!(
                *inner,
                metadata::Type::PtrMut(..) | metadata::Type::PtrConst(..)
            ) =>
        {
            let (leaf, depth) = flatten(*inner, n);
            metadata::Type::PtrMut(Box::new(leaf), depth)
        }
        metadata::Type::PtrConst(inner, n)
            if matches!(
                *inner,
                metadata::Type::PtrMut(..) | metadata::Type::PtrConst(..)
            ) =>
        {
            let (leaf, depth) = flatten(*inner, n);
            metadata::Type::PtrConst(Box::new(leaf), depth)
        }
        other => other,
    }
}

/// Collapse an `LP*`/`P*` *pointer* typedef parameter (`LPDWORD`, `PHKEY`,
/// `LPSECURITY_ATTRIBUTES`, …) to the raw pointer it spells (`*mut DWORD`, `*mut HKEY`,
/// `*const SECURITY_ATTRIBUTES`), so the pointer level — and hence its SAL-driven
/// const-ness — is expressed structurally in the ABI rather than hidden inside an opaque
/// alias that bindgen cannot const-qualify. The alias is inlined one level via
/// `to_type`, which preserves the *named* pointee (`DWORD`, `HKEY`, a record/enum) and
/// its C const-ness; only the redundant `LP*`/`P*` wrapper is dropped. See ledger #5.
///
/// Kept named (returned unchanged, or normalised): the string wrappers
/// ([`normalize_string_alias`]); non-pointer aliases (scalars like `DWORD`, records,
/// enums); and handles — a `void*` handle (`HANDLE`) or a `DECLARE_HANDLE` tag
/// (`HWND` = `struct HWND__ *`) — which are opaque values, never pointers-to-data.
fn collapse_pointer_alias_param(
    cursor_ty: &Type,
    base: metadata::Type,
    parser: &mut Parser<'_>,
) -> metadata::Type {
    let metadata::Type::ValueName(ref type_name) = base else {
        return base;
    };
    // Consult the single name-keyed policy table first; anything not listed falls through
    // to the structural rules below. (Generic `void*` aliases never reach here: `to_type`
    // has already collapsed them to raw `*mut`/`*const void` via [`void_pointer_alias`],
    // so `base` is a pointer, not a `ValueName`.)
    match alias_policy(&type_name.name) {
        // A canonical string wrapper: kept named as its `canonical` variant; SAL later
        // selects the const/non-const form.
        Some(AliasPolicy::String { canonical, .. }) => {
            return metadata::Type::value_named(&type_name.namespace, canonical);
        }
        // Kept named verbatim (`BSTR`).
        Some(AliasPolicy::KeepNamed) => return base,
        None => {}
    }
    // Resolve one typedef level; a pointer alias's underlying type is the pointer.
    let mut underlying = cursor_ty.ty().typedef_underlying_type();
    if underlying.kind() == CXType_Elaborated {
        underlying = underlying.underlying_type();
    }
    if underlying.kind() != CXType_Pointer {
        // A scalar / record / enum alias (`DWORD`, `GUID`, …) — keep it named.
        return base;
    }
    let pointee = underlying.pointee_type();
    // Canonicalise the pointee so an elaborated/typedef-sugared handle tag
    // (`struct HWND__` / `HKEY`) is classified by its underlying shape.
    let pointee_canon = pointee.canonical_type();
    match pointee_canon.kind() {
        // A `void*` handle (`HANDLE`) is an opaque value kept named; the generic-void data
        // pointers (`PVOID`, …) were already collapsed by `to_type`.
        CXType_Void => base,
        // A `DECLARE_HANDLE` tag (`struct X__ *`) is an opaque handle value.
        CXType_Record if pointee_canon.ty().name().ends_with("__") => base,
        // A MIDL file-scope handle placeholder (`struct __MIDL___MIDL_itf_* *`) is an
        // opaque handle value — keep the typedef named rather than inlining the tag.
        CXType_Record
            if is_midl_placeholder_tag(&pointee_canon.ty().name())
                && is_handle_shape(&pointee_canon.ty()) =>
        {
            base
        }
        // A function-pointer alias (`FARPROC`, …) is kept named — it is emitted as a
        // distinct callback type, not flattened to an opaque `*mut u8`.
        CXType_FunctionProto | CXType_FunctionNoProto => base,
        // Inline the alias to its raw pointer, reusing the `CXType_Pointer` conversion
        // (named pointee, const flattening, interface stripping).
        _ => underlying.to_type(parser),
    }
}

/// Override a collapsed pointer parameter's const-ness from its SAL direction:
/// `_In_`/`_In_opt_`/`_Reserved_` (read-only) → `*const`; `_Out_`/`_Inout_` (writable)
/// → `*mut`. SAL is the author's read/write contract, so it wins over the C typedef's
/// own mutability (`_In_ LPWSTR` is a *read-only* buffer). Only a raw pointer's outermost
/// level is flipped; named aliases (string wrappers, handles) and non-pointers are
/// returned unchanged. See ledger #4.
fn apply_sal_constness(ty: metadata::Type, annotation: &ParamAnnotation) -> metadata::Type {
    if !annotation.is_annotated() {
        return ty;
    }
    // In / Reserved (read-only) → const; Out / Inout (writable) → mut; a bare
    // `_*_opt_`/array annotation with no direction leaves the C const-ness intact.
    let make_const = if annotation.out_param {
        false
    } else if annotation.in_param || annotation.reserved {
        true
    } else {
        return ty;
    };
    match ty {
        metadata::Type::PtrMut(inner, n) | metadata::Type::PtrConst(inner, n) => {
            if make_const {
                metadata::Type::PtrConst(inner, n)
            } else {
                metadata::Type::PtrMut(inner, n)
            }
        }
        // A canonical string wrapper flips between its const/non-const named variant
        // (`PWSTR` ↔ `PCWSTR`, `PSTR` ↔ `PCSTR`), per the same name-keyed policy.
        metadata::Type::ValueName(ref type_name) => {
            if let Some(AliasPolicy::String {
                mut_name,
                const_name,
                ..
            }) = alias_policy(&type_name.name)
            {
                let variant = if make_const { const_name } else { mut_name };
                metadata::Type::value_named(&type_name.namespace, variant)
            } else {
                ty
            }
        }
        other => other,
    }
}

/// Promote a raw null-terminated string *parameter* to its canonical `PWSTR`/`PCWSTR`/
/// `PSTR`/`PCSTR` wrapper, so bindgen's ergonomic string projection applies to a bare
/// `_In_z_ WCHAR const*` (no named `LP*` alias, e.g. `IDWriteFactory::CreateTextFormat`)
/// exactly as it does to the *named* aliases ([`normalize_string_alias`], ledger #9).
///
/// The `_z_` SAL bit is the null-terminated-string contract; without it a `WCHAR const*`
/// is an opaque `*const u16` buffer, so the promotion is gated on
/// [`ParamAnnotation::null_terminated`] — never on pointee width alone. A *counted* buffer
/// (`CreateTextLayout(string: *const u16, …)`, carrying a `NativeArrayInfo`) never sets the
/// bit and stays a raw pointer; the `size`/`array` guard is belt-and-suspenders for the
/// `_*_reads_z_` shapes.
///
/// The variant follows the pointer const-ness [`apply_sal_constness`] already resolved
/// from SAL direction — `*const _` → read-only (`PCWSTR`/`PCSTR`), `*mut _` → writable
/// (`PWSTR`/`PSTR`); wide (`u16`) vs narrow (`i8`/`u8`) picks the character width. Only a
/// single-level raw char pointer qualifies. Flat scrape only, like
/// [`normalize_string_alias`]: the namespaced (WebView2) scrape references a reference
/// winmd whose const wrappers are not distinct types, so a promoted `PCWSTR` would dangle.
fn promote_null_terminated_string(
    ty: metadata::Type,
    annotation: &ParamAnnotation,
    parser: &Parser<'_>,
) -> metadata::Type {
    if !annotation.null_terminated
        || parser.header_root.is_none()
        || annotation.size.is_some()
        || annotation.array.is_some()
    {
        return ty;
    }
    let (pointee, is_const) = match &ty {
        metadata::Type::PtrConst(inner, 1) => (inner.as_ref(), true),
        metadata::Type::PtrMut(inner, 1) => (inner.as_ref(), false),
        _ => return ty,
    };
    let canonical = match (pointee, is_const) {
        (metadata::Type::U16, true) => "PCWSTR",
        (metadata::Type::U16, false) => "PWSTR",
        (metadata::Type::I8 | metadata::Type::U8, true) => "PCSTR",
        (metadata::Type::I8 | metadata::Type::U8, false) => "PSTR",
        _ => return ty,
    };
    metadata::Type::value_named(parser.namespace, canonical)
}

/// Re-qualify a canonical string alias (`PWSTR`/`PCWSTR`/`PSTR`/`PCSTR`) to the
/// namespace the reference metadata defines it in.
///
/// A *namespaced* scrape (e.g. WebView2) resolves a local `LPCWSTR` typedef to its
/// own namespace, and [`apply_sal_constness`] flips it to the canonical `PCWSTR`
/// keeping that (local) namespace — but the canonical wrapper is not defined locally.
/// When the reference winmd defines it (the in-house `Windows.Win32.{PWSTR,PCWSTR,
/// PSTR,PCSTR}`), point at that definition so it resolves instead of dangling as an
/// undefined local type. In the flat scrape the canonical is absent from `ref_map`
/// (it lives in the single root namespace and already resolves by name), so this is a
/// no-op there.
fn requalify_string_alias(ty: metadata::Type, parser: &Parser<'_>) -> metadata::Type {
    if let metadata::Type::ValueName(ref type_name) = ty
        && string_alias_canonical(&type_name.name).is_some()
        && let Some(ns) = parser.ref_map.get(&type_name.name)
    {
        return metadata::Type::value_named(ns, &type_name.name);
    }
    ty
}

/// Map a builtin scalar [`CXTypeKind`] to its [`metadata::Type`] (LLP64 widths).
fn scalar_kind_to_type(kind: CXTypeKind) -> metadata::Type {
    match kind {
        CXType_Bool => metadata::Type::Bool,
        CXType_Char_U | CXType_UChar => metadata::Type::U8,
        CXType_UShort | CXType_WChar | CXType_Char16 => metadata::Type::U16,
        CXType_UInt | CXType_ULong | CXType_Char32 => metadata::Type::U32,
        CXType_ULongLong => metadata::Type::U64,
        CXType_Char_S | CXType_SChar => metadata::Type::I8,
        CXType_Short => metadata::Type::I16,
        CXType_Int | CXType_Long => metadata::Type::I32,
        CXType_LongLong => metadata::Type::I64,
        CXType_Float => metadata::Type::F32,
        CXType_Double => metadata::Type::F64,
        _ => metadata::Type::I32,
    }
}
