//! Canonical C header type mappings for Win32 metadata.
//!
//! [`resolve_typedef`] handles references. [`param_metadata_type`] adds parameter-only rules.
//! Definition suppression in `typedef.rs`, `const.rs`, and `lib.rs` must match these mappings.

use super::*;

/// Resolves a typedef reference before parameter-specific mapping.
pub(crate) fn resolve_typedef(cursor: &Type, parser: &mut Parser<'_>) -> metadata::Type {
    let decl = cursor.ty();
    let name = decl.name();
    // String normalisation and the flat collapses are gated to the per-header scrape: a
    // namespaced scrape (WebView2) resolves `PCWSTR`/`PCSTR` through a reference winmd where they
    // are `const PWSTR`, not distinct types, so forcing them here would leave the reference
    // dangling.
    if parser.header_root.is_some() {
        if let Some(normalized) = normalize_string_alias(parser.namespace, &name) {
            return normalized;
        }
        return flat_canonical(parser.namespace, &name, cursor, parser)
            .unwrap_or_else(|| metadata::Type::value_named(parser.namespace, &name));
    }

    // Namespaced scrape: resolve through the reference metadata. A local typedef is emitted by
    // name; an external one is scheduled for a follow-up pass.
    if let Some(ns) = parser.ref_map.get(&name) {
        metadata::Type::value_named(ns, &name)
    } else if let Some(ty) = universal_alias(parser.namespace, &name) {
        ty
    } else if let Some(scalar) = collapse_scalar_typedef(&name, cursor) {
        scalar
    } else if decl.is_from_main_file() {
        metadata::Type::value_named(parser.namespace, &name)
    } else {
        parser.pending_typedefs.push(decl);
        metadata::Type::value_named(parser.namespace, &name)
    }
}

/// The ordered reference-site collapses for the flat per-header scrape. The list order *is* the
/// precedence; the tables are mutually exclusive by construction, so it is for reading, not
/// tie-breaking. `None` means no rule matched and the caller resolves by name.
fn flat_canonical(
    namespace: &str,
    name: &str,
    cursor: &Type,
    parser: &mut Parser<'_>,
) -> Option<metadata::Type> {
    if let Some(scalar) = semantic_scalar(name) {
        return Some(scalar);
    }
    if let Some(scalar) = fundamental_scalar(name) {
        return Some(scalar);
    }
    if let Some(scalar) = floating_typedef(cursor) {
        return Some(scalar);
    }
    if let Some(scalar) = pointer_sized_abi(name) {
        return Some(scalar);
    }
    if let Some(ty) = universal_alias(namespace, name) {
        return Some(ty);
    }
    if let Some(leaf) = numerics_alias(name) {
        return Some(metadata::Type::value_named(NUMERICS_NAMESPACE, leaf));
    }
    if let Some(base) = d2d_compat_alias(name) {
        // The compat synonym's numerics-mapped members reach the Numerics type through their
        // `D2D_*` base; the plain ones resolve to the shared struct.
        return Some(match numerics_alias(base) {
            Some(leaf) => metadata::Type::value_named(NUMERICS_NAMESPACE, leaf),
            None => metadata::Type::value_named(namespace, base),
        });
    }
    interface_alias(cursor, parser)
}

/// GUID synonyms and generic `void*` aliases collapse in *every* scrape mode, not just the flat
/// one - see [`resolve_typedef`]'s namespaced branch.
fn universal_alias(namespace: &str, name: &str) -> Option<metadata::Type> {
    if guid_alias(name) {
        return Some(metadata::Type::value_named(namespace, "GUID"));
    }
    void_pointer_alias(name)
}

/// Collapse a `typedef IFoo NAME` / `typedef IFoo *NAME` COM-interface alias to the interface
/// itself - interfaces are implied pointers in metadata, so the alias carries no distinct ABI.
fn interface_alias(cursor: &Type, parser: &mut Parser<'_>) -> Option<metadata::Type> {
    let underlying = cursor.ty().typedef_underlying_type();
    if !is_interface_alias(&underlying) {
        return None;
    }
    Some(if underlying.is_interface() {
        underlying.to_type(parser)
    } else {
        underlying.pointee_type().to_type(parser)
    })
}

/// Whether a typedef's `underlying` type aliases a COM interface - the direct `typedef IFoo NAME`
/// or the `typedef IFoo *NAME` (`LP*`/`P*`) spelling.
pub(crate) fn is_interface_alias(underlying: &Type) -> bool {
    underlying.is_interface()
        || (underlying.kind() == CXType_Pointer && underlying.pointee_type().is_interface())
}

/// Collapse a scalar typedef to its primitive, for the namespaced scrape's fallback. Pointer-sized
/// names ([`pointer_sized_abi`]) collapse by name; every other typedef collapses only when its
/// resolved canonical type is a builtin scalar, so handle/pointer/record typedefs are untouched.
///
/// Callers must check the reference metadata first: a scalar typedef the reference preserves
/// (`HRESULT`, `BOOL`) must resolve to that type, not collapse.
fn collapse_scalar_typedef(name: &str, ty: &Type) -> Option<metadata::Type> {
    if let Some(scalar) = pointer_sized_abi(name) {
        return Some(scalar);
    }

    let canonical = ty.canonical_type();
    is_fundamental_scalar_kind(canonical.kind()).then(|| scalar_kind_to_type(canonical.kind()))
}

/// Collapse a typedef whose canonical type is floating-point to the bare primitive
/// (`float`/`double`/`long double` -> `f32`/`f64`). Structural, not name-keyed: the reference
/// metadata drops every floating typedef (unlike integer aliases, which use
/// [`fundamental_scalar`]). MSVC `long double` is 64-bit. Also backs the definition skip in
/// [`Typedef::parse`].
pub(crate) fn floating_typedef(ty: &Type) -> Option<metadata::Type> {
    match ty.canonical_type().kind() {
        CXType_Float => Some(metadata::Type::F32),
        CXType_Double | CXType_LongDouble => Some(metadata::Type::F64),
        _ => None,
    }
}

/// The fundamental C builtin scalar kinds that map to a Rust primitive via [`scalar_kind_to_type`].
pub(crate) fn is_fundamental_scalar_kind(kind: CXTypeKind) -> bool {
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

/// The fixed-width integer/char portability typedefs (`DWORD`, `WORD`, `INTn`, `intN_t`, ...),
/// each mapped to the primitive of its width and collapsed at every use site.
///
/// This is a **curated** list - scalar typedefs are preserved by default, only these names
/// collapse. The choice cannot be structural: `HFILE` (`typedef int`), `ATOM` (`typedef WORD`)
/// and `COLORREF` (`typedef DWORD`) are byte-identical to the primitives here, so only the *name*
/// separates a meaningful domain type from portability noise. Pointer-sized aliases are in
/// [`pointer_sized_abi`]. Also backs the const-cast collapse in [`parse_named_cast`].
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

/// Named Win32 types whose canonical projection is a primitive, not their header shape:
/// `BOOLEAN` -> `bool` (a semantically-boolean `BYTE`) and `LARGE_INTEGER`/`ULARGE_INTEGER` ->
/// `i64`/`u64` (64-bit overlay unions every consumer uses as one scalar). Collapsed at every
/// reference like `DWORD` -> `u32`. Name-keyed because the collapse cannot be structural
/// (`BOOLEAN` is byte-identical to `BYTE`, and RPC's lowercase `boolean` stays `u8`); the
/// definitions are suppressed in `typedef.rs` (`BOOLEAN`) and `lib.rs` (the union records).
pub(crate) fn semantic_scalar(name: &str) -> Option<metadata::Type> {
    Some(match name {
        "BOOLEAN" => metadata::Type::Bool,
        "LARGE_INTEGER" => metadata::Type::I64,
        "ULARGE_INTEGER" => metadata::Type::U64,
        _ => return None,
    })
}

/// The `GUID` synonyms (`typedef GUID X` with no distinct ABI), collapsed to `GUID` itself. This
/// lets bindgen apply the ergonomic `QueryInterface<T>()` / `Resolve<T>()` COM projection, which
/// keys off a `*const GUID` companion to a `ComOutPtr` out-param. `REFIID`/`REFCLSID`/`REFGUID`
/// (the `const *` forms) collapse structurally once their pointee resolves through here.
pub(crate) fn guid_alias(name: &str) -> bool {
    matches!(name, "IID" | "CLSID" | "FMTID" | "UUID")
}

/// The generic `void*` portability aliases, returning the raw pointer they spell. Name-keyed
/// rather than structural because a `void*` *handle* (`HANDLE`) is structurally identical but
/// semantic - it stays named (same `HFILE`-vs-`DWORD` judgement as [`fundamental_scalar`]).
pub(crate) fn void_pointer_alias(name: &str) -> Option<metadata::Type> {
    Some(match name {
        "PVOID" | "LPVOID" | "PVOID64" => metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1),
        "LPCVOID" | "PCVOID" | "LPCTVOID" => {
            metadata::Type::PtrConst(Box::new(metadata::Type::Void), 1)
        }
        _ => return None,
    })
}

/// The Direct2D 1.1 `typedef D2D_X D2D1_X` source-compat synonyms, returning the `D2D_*` base.
/// Collapsing to the base lets the numerics-mapped members reach [`numerics_alias`] under their
/// canonical name. Curated: a `D2D1_` type without a shared base (`D2D1_TAG`, the enums) is
/// preserved.
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

/// Win32 aggregates that are bit-for-bit identical to a `Windows.Foundation.Numerics` value type,
/// returned as the Numerics leaf name. Every reference collapses to the shared Numerics type, so
/// the `windows` crate resolves them through `Windows.winmd` with no per-consumer substitution.
/// The struct definitions are suppressed in `lib.rs`'s `CXCursor_StructDecl` arm, and
/// `windows-sys` drops every API mentioning them via its `!Windows` package filter.
///
/// Name-keyed because layout alone is ambiguous (`D2D_POINT_2F` and `D2D_SIZE_F` are both
/// `{ f32; f32 }` but only the former is a `Vector2`). The `D2D1_*` spellings reach here via
/// [`d2d_compat_alias`].
pub(crate) fn numerics_alias(name: &str) -> Option<&'static str> {
    Some(match name {
        "D2D_MATRIX_3X2_F" => "Matrix3x2",
        "D3DMATRIX" | "D2D_MATRIX_4X4_F" => "Matrix4x4",
        "D2D_POINT_2F" | "D2D_VECTOR_2F" => "Vector2",
        "D2D_VECTOR_3F" => "Vector3",
        "D2D_VECTOR_4F" => "Vector4",
        _ => return None,
    })
}

/// The metadata namespace the [`numerics_alias`] leaf names live in.
pub(crate) const NUMERICS_NAMESPACE: &str = "Windows.Foundation.Numerics";

/// Recognises a pointer-sized integer typedef by its ABI-defined name and returns `usize`/`isize`,
/// covering both the `basetsd.h` aliases (`ULONG_PTR`, `SIZE_T`, ...) and the C-runtime ones
/// (`size_t`, `intptr_t`, ...). Name-keyed because on a 64-bit parse the canonical type is an
/// indistinguishable 64-bit integer. Collapsing (rather than emitting a named alias) is what keeps
/// them arch-neutral: a named alias would freeze the width per-arch and split under a spurious
/// `#[arch]` gate at merge time.
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

/// The name-keyed policy for the handful of parameter aliases whose treatment cannot be decided
/// structurally: a `void*` handle (`HANDLE`) and a `void*` data pointer (`PVOID`) are the same C
/// type, and `BSTR` and `LPCWSTR` are both `wchar_t*`. Everything not listed is decided
/// structurally by [`collapse_pointer_alias_param`].
#[derive(Clone, Copy)]
enum AliasPolicy {
    /// A canonical string-pointer wrapper, kept named so the `windows` projection maps it to a
    /// string type. `canonical` is this spelling's variant; `mut_name`/`const_name` are the
    /// variants SAL selects between (`PWSTR`<->`PCWSTR`, `PSTR`<->`PCSTR`).
    String {
        canonical: &'static str,
        mut_name: &'static str,
        const_name: &'static str,
    },
    /// Kept named verbatim. `BSTR` is a length-prefixed, `SysAllocString`-owned COM string, not a
    /// bare `OLECHAR*`.
    KeepNamed,
}

/// Look up the name-keyed [`AliasPolicy`], or `None` to fall through to the structural rules. The
/// one place the string wrappers and `BSTR` are enumerated. (Generic `void*` aliases are handled
/// earlier by [`void_pointer_alias`].)
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

/// The canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` spelling a string-pointer alias normalises to,
/// or `None` for a non-string alias (and `BSTR`, being [`AliasPolicy::KeepNamed`]). Callers use
/// `canonical != name` to tell a redundant alias (`LPCWSTR`) from the four canonical wrappers.
pub(crate) fn string_alias_canonical(name: &str) -> Option<&'static str> {
    match alias_policy(name) {
        Some(AliasPolicy::String { canonical, .. }) => Some(canonical),
        _ => None,
    }
}

/// Normalise a string-pointer alias *reference* to its canonical value type. Applied at every
/// reference site from [`Type::to_type`], so fields speak the four canonical spellings bindgen
/// recognises (an `LPCWSTR` field would otherwise degrade to `*const u16`). SAL const-ness is
/// re-applied for parameters afterwards ([`apply_sal_constness`]).
fn normalize_string_alias(namespace: &str, name: &str) -> Option<metadata::Type> {
    string_alias_canonical(name).map(|canonical| metadata::Type::value_named(namespace, canonical))
}

/// Decay a C array parameter to a pointer (C11 6.7.6.3p7). [`Type::to_type`] maps `T[]`/`T[N]` to
/// `ArrayFixed`, correct for a struct field but wrong for a parameter (a by-value array is
/// FFI-unsafe and an unsized `[T; 0]` drops the argument). The reference carries a pointer plus a
/// `NativeArrayInfo(CountConst = N)`, from which bindgen reconstructs a length-checked `&[T; N]`
/// in the safe wrapper only (see [`inline_array_param_count`]). Pointee const-ness follows the
/// element's C const-ness; SAL may override it in [`apply_sal_constness`].
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
    // Prefer the resolved `ArrayFixed` element (preserves inline element aliases); for a named
    // array typedef the base is the alias, so resolve the element from the canonical array.
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

/// Resolve a parameter's metadata type. Fields, returns and constants keep their named aliases and
/// array shapes; only parameters are collapsed and decayed.
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

/// The `CountConst` for an inline fixed-size array parameter (`T name[N]`), recorded alongside the
/// decayed pointer so bindgen reconstructs a `&[T; N]` in the safe wrapper. `None` for typedef
/// arrays (length lives on the typedef), unsized arrays, and non-arrays. See [`decay_array_param`].
pub(crate) fn inline_array_param_count(cursor_ty: &Type) -> Option<i32> {
    if cursor_ty.kind() != CXType_ConstantArray {
        return None;
    }
    let size = cursor_ty.array_size();
    (size > 0).then_some(size as i32)
}

/// Collapse a mixed-constness pointer chain to a uniform chain governed by its outermost level.
///
/// The winmd `Type` model stores a pointer run as a single const bit plus a depth, so it cannot
/// represent a chain whose levels differ in const-ness: serialising `PtrMut(PtrConst(T))` corrupts
/// it on the winmd round-trip (the inner modifier is misread and the run degrades to
/// `*const *const T`). The outermost level carries the real read/write direction (set by
/// [`apply_sal_constness`]), so it governs the whole chain. Uniform chains are already collapsed by
/// [`Type::to_type`], so only a genuinely mixed chain nests here.
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

/// Makes every pointer run representable in RDL by applying its innermost qualifier to the run.
///
/// Non-parameter C types retain pointee constness when per-level qualifiers cannot all be stored.
/// Parameter types are already uniform here because SAL direction is applied before emission.
pub(crate) fn normalize_rdl_type(ty: &metadata::Type) -> metadata::Type {
    fn pointer_run(inner: &metadata::Type, depth: usize, is_const: bool) -> metadata::Type {
        match inner {
            metadata::Type::PtrMut(deeper, pointers) => {
                pointer_run(deeper, depth + pointers, false)
            }
            metadata::Type::PtrConst(deeper, pointers) => {
                pointer_run(deeper, depth + pointers, true)
            }
            leaf => {
                let leaf = normalize_rdl_type(leaf);
                if is_const {
                    metadata::Type::PtrConst(Box::new(leaf), depth)
                } else {
                    metadata::Type::PtrMut(Box::new(leaf), depth)
                }
            }
        }
    }

    match ty {
        metadata::Type::Array(inner) => metadata::Type::Array(Box::new(normalize_rdl_type(inner))),
        metadata::Type::ArrayFixed(inner, len) => {
            metadata::Type::ArrayFixed(Box::new(normalize_rdl_type(inner)), *len)
        }
        metadata::Type::RefMut(inner) => {
            metadata::Type::RefMut(Box::new(normalize_rdl_type(inner)))
        }
        metadata::Type::RefConst(inner) => {
            metadata::Type::RefConst(Box::new(normalize_rdl_type(inner)))
        }
        metadata::Type::PtrMut(inner, pointers) => pointer_run(inner, *pointers, false),
        metadata::Type::PtrConst(inner, pointers) => pointer_run(inner, *pointers, true),
        metadata::Type::ClassName(type_name) => {
            let mut type_name = type_name.clone();
            type_name.generics = type_name.generics.iter().map(normalize_rdl_type).collect();
            metadata::Type::ClassName(type_name)
        }
        metadata::Type::ValueName(type_name) => {
            let mut type_name = type_name.clone();
            type_name.generics = type_name.generics.iter().map(normalize_rdl_type).collect();
            metadata::Type::ValueName(type_name)
        }
        other => other.clone(),
    }
}

/// Collapse an `LP*`/`P*` pointer typedef parameter (`LPDWORD`, `PHKEY`, ...) to the raw pointer it
/// spells, so the pointer level - and its SAL-driven const-ness - is expressed structurally rather
/// than hidden in an opaque alias bindgen cannot const-qualify. The named pointee and its C
/// const-ness are preserved. Kept named: string wrappers, non-pointer aliases, and handles (a
/// `void*` handle or a `DECLARE_HANDLE` tag), which are opaque values, not pointers-to-data.
fn collapse_pointer_alias_param(
    cursor_ty: &Type,
    base: metadata::Type,
    parser: &mut Parser<'_>,
) -> metadata::Type {
    let metadata::Type::ValueName(ref type_name) = base else {
        return base;
    };
    // Generic `void*` aliases never reach here: `to_type` already collapsed them to a raw pointer.
    match alias_policy(&type_name.name) {
        Some(AliasPolicy::String { canonical, .. }) => {
            return metadata::Type::value_named(&type_name.namespace, canonical);
        }
        Some(AliasPolicy::KeepNamed) => return base,
        None => {}
    }
    // Resolve one typedef level; a pointer alias's underlying type is the pointer.
    let mut underlying = cursor_ty.ty().typedef_underlying_type();
    if underlying.kind() == CXType_Elaborated {
        underlying = underlying.underlying_type();
    }
    if underlying.kind() != CXType_Pointer {
        return base;
    }
    let pointee = underlying.pointee_type();
    // Canonicalise so an elaborated/sugared handle tag is classified by its underlying shape.
    let pointee_canon = pointee.canonical_type();
    match pointee_canon.kind() {
        // A `void*` handle (`HANDLE`) is opaque; generic-void data pointers were already collapsed.
        CXType_Void => base,
        // A `DECLARE_HANDLE` tag (`struct X__ *`) is an opaque handle value.
        CXType_Record if pointee_canon.ty().name().ends_with("__") => base,
        // A MIDL file-scope handle placeholder (`struct __MIDL___MIDL_itf_* *`) is an opaque handle.
        CXType_Record
            if is_midl_placeholder_tag(&pointee_canon.ty().name())
                && is_handle_shape(&pointee_canon.ty()) =>
        {
            base
        }
        // A function-pointer alias (`FARPROC`) is emitted as a distinct callback type, kept named.
        CXType_FunctionProto | CXType_FunctionNoProto => base,
        // Inline the alias to its raw pointer.
        _ => underlying.to_type(parser),
    }
}

/// Override a collapsed pointer parameter's const-ness from its SAL direction: `_In_`/`_Reserved_`
/// -> `*const`, `_Out_`/`_Inout_` -> `*mut`. SAL is the author's read/write intent, so it wins over
/// the C typedef's mutability (`_In_ LPWSTR` is a read-only buffer). Named aliases and non-pointers
/// are unchanged, except a canonical string wrapper flips between its const/non-const variant.
fn apply_sal_constness(ty: metadata::Type, annotation: &ParamAnnotation) -> metadata::Type {
    if !annotation.is_annotated() {
        return ty;
    }
    // A bare `_*_opt_`/array annotation with no direction leaves the C const-ness intact.
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
        // A canonical string wrapper flips between its const/non-const named variant.
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

/// Promote a raw null-terminated string parameter (`_In_z_ WCHAR const*` with no named alias) to
/// its canonical wrapper, so bindgen's string projection applies exactly as to the named aliases.
/// Gated on the `_z_` SAL bit ([`ParamAnnotation::null_terminated`]) - without it a `WCHAR const*`
/// is an opaque buffer; the `size`/`array` guard excludes counted `_*_reads_z_` shapes. The
/// variant follows the const-ness [`apply_sal_constness`] resolved and the pointee width. Flat
/// scrape only, like [`normalize_string_alias`].
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

/// Re-qualify a canonical string alias to the namespace the reference metadata defines it in. A
/// namespaced scrape (WebView2) resolves a local `LPCWSTR` to its own namespace and
/// [`apply_sal_constness`] flips it to `PCWSTR` keeping that namespace, but the wrapper is defined
/// in the reference winmd, not locally. No-op in the flat scrape (absent from `ref_map`).
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
pub(crate) fn scalar_kind_to_type(kind: CXTypeKind) -> metadata::Type {
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
        rest => panic!("{rest:?}"),
    }
}
