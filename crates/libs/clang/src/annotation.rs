use super::*;

/// Flags derived from SAL annotations or MIDL block-comment annotations on a parameter.
///
/// SAL annotations (`_In_`, `_Out_`, `_Inout_`, `_In_opt_`, etc.) are extracted from
/// `CXCursor_AnnotateAttr` or `CXCursor_UnexposedAttr` children of a `CXCursor_ParmDecl`
/// cursor.
///
/// MIDL comment annotations (`/* [in] */`, `/* [out] */`, `/* [retval][out] */`, etc.)
/// are extracted by tokenising the `ParmDecl` cursor extent and looking for
/// `CXToken_Comment` tokens that match the known MIDL attribute names.
///
/// When no annotation is detected, all fields remain `false` and direction defaults are
/// inferred from the parameter's type (mutable pointer/reference -> Out, everything else -> In),
/// matching the reader's `parse_param_attributes` logic.
#[derive(Debug, Default, Clone)]
pub struct ParamAnnotation {
    /// The parameter is annotated as an input (`_In_`, `_In_opt_`, `_Inout_`, `[in]`, ...).
    pub in_param: bool,
    /// The parameter is annotated as an output (`_Out_`, `_Out_opt_`, `_Inout_`, `[out]`, ...).
    pub out_param: bool,
    /// The parameter may be `NULL` / absent (`_In_opt_`, `_Out_opt_`, `[optional]`, ...).
    pub optional: bool,
    /// The parameter is a retval (`[retval]` MIDL comment annotation).
    pub retval: bool,
    /// The parameter is reserved and must be zero/null (`_Reserved_`) -> `Reserved`.
    pub reserved: bool,
    /// The parameter is a COM out-pointer (`_COM_Outptr_` and variants) -> `ComOutPtr`.
    pub com_out_ptr: bool,
    /// A speculative `_COM_Outptr_` recovered from a bare identifier *token* (the
    /// abstract-virtual COM-method case, where clang does not attach the SAL as a
    /// `ParmDecl` `AnnotateAttr` and the MIDL comment omits `[iid_is]`). Unlike
    /// `com_out_ptr`, this is promoted to a real `ComOutPtr` in [`parse_params`] only
    /// when the pointee is `void` (the caller-chosen-type idiom, e.g. `GetParent`);
    /// a concrete interface pointee (e.g. `EnumAdapters` -> `IDXGIAdapter**`) keeps
    /// its declared type, matching the canonical projection.
    pub com_out_ptr_token: bool,
    /// The parameter is a *pure* null-terminated string (`_In_z_`, `_In_opt_z_`,
    /// `_Out_z_`, `_Inout_z_`, `_Inout_opt_z_`) - the `_z_` SAL contract with no explicit
    /// element count. A raw `WCHAR*`/`char*` so annotated is promoted to the canonical
    /// `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` wrapper in [`super::cx::param_metadata_type`]. The
    /// *counted* `_*_reads_z_`/`_*_writes_z_`/`_*_updates_z_` variants do not
    /// set this - they carry a `NativeArrayInfo` and stay raw pointers.
    pub null_terminated: bool,
    /// Buffer/array size reference captured from a SAL `_*_reads_`/`_*_writes_`/
    /// `_*_updates_` macro argument, before parameter-name -> index resolution.
    pub size: Option<SalSize>,
    /// Resolved array/size attribute, emitted before the direction attributes by
    /// [`param_attrs_for_annotation`].  Populated by [`resolve_param_array_info`].
    pub array: Option<ArrayInfo>,
}

impl ParamAnnotation {
    /// Returns `true` if any annotation was detected.
    pub fn is_annotated(&self) -> bool {
        self.in_param
            || self.out_param
            || self.optional
            || self.retval
            || self.reserved
            || self.com_out_ptr
            || self.size.is_some()
            || self.array.is_some()
    }
}

/// A buffer/array size argument captured from a SAL count/byte macro, before
/// parameter-name -> index resolution.
#[derive(Debug, Clone)]
pub enum SalSizeArg {
    /// An integer-literal element count or byte count (`_Out_writes_(8)`).
    Const(i32),
    /// A bare identifier - a parameter name (later resolved to a 0-based index) or,
    /// for struct fields, a sibling field name (`_In_reads_(count)`).
    Name(String),
}

/// Raw buffer-size information captured from a SAL `_*_reads_`/`_*_writes_`/
/// `_*_updates_` macro.
#[derive(Debug, Clone)]
pub struct SalSize {
    /// `true` for `*_bytes_*` macros (-> `MemorySize`); `false` for element-count
    /// macros (-> `NativeArrayInfo`).
    pub bytes: bool,
    /// The (first) size argument expression.
    pub arg: SalSizeArg,
}

/// A resolved array/size attribute ready to emit as RDL.
#[derive(Debug, Clone)]
pub enum ArrayInfo {
    /// `NativeArrayInfo(CountParamIndex = N)` - element count given by another parameter.
    CountParamIndex(i16),
    /// `NativeArrayInfo(CountConst = N)` - fixed element count.
    CountConst(i32),
    /// `MemorySize(BytesParamIndex = N)` - byte count given by another parameter.
    BytesParamIndex(i16),
}

/// Method-level MIDL attributes extracted from block comments in a method declaration.
///
/// `is_propget` and `is_propput` indicate COM property accessor methods.  Both are
/// collapsed to `#[special]` in the RDL output because the get/put distinction is
/// already encoded in the `get_`/`put_` method name prefix.
#[derive(Debug, Default)]
pub struct MethodAnnotation {
    /// True when `[propget]` appears in a block comment before the method name.
    pub is_propget: bool,
    /// True when `[propput]` appears in a block comment before the method name.
    pub is_propput: bool,
}

/// Extract method-level MIDL annotations by scanning `tokens` up to (but not
/// including) the first identifier token whose spelling equals `method_name`.
///
/// Only `CXToken_Comment` tokens are inspected; all others are skipped.
/// Scanning stops at the function name to avoid reading into the parameter list.
pub fn extract_method_annotation(
    tokens: &[(CXTokenKind, String)],
    method_name: &str,
) -> MethodAnnotation {
    let mut annotation = MethodAnnotation::default();
    for (kind, spelling) in tokens {
        if *kind == CXToken_Identifier && spelling == method_name {
            break;
        }
        if *kind == CXToken_Comment {
            if spelling.contains("[propget]") {
                annotation.is_propget = true;
            }
            if spelling.contains("[propput]") {
                annotation.is_propput = true;
            }
        }
    }
    annotation
}

/// Extract parameter annotations from a `CXCursor_ParmDecl` cursor.
///
/// Two annotation sources are tried in priority order:
///
/// 1. **SAL annotations** - `CXCursor_AnnotateAttr` and `CXCursor_UnexposedAttr` children of
///    the cursor, produced by `__attribute__((annotate("_In_")))` portable stubs or by
///    `__declspec(SAL_*)` on Windows/MSVC targets.
///
/// 2. **MIDL block-comment annotations** - checked separately via
///    [`scan_method_param_annotations`] on the parent method/function's token stream,
///    because the ParmDecl extent does not include the leading `/* [in] */` comment.
///    Call sites should merge the result of this function with the MIDL scan:
///    if this returns `is_annotated() == false`, use the MIDL annotation instead.
pub fn extract_param_annotation(cursor: &Cursor, tu: &TranslationUnit) -> ParamAnnotation {
    let mut annotation = ParamAnnotation::default();

    for child in cursor.children() {
        match child.kind() {
            CXCursor_AnnotateAttr => {
                // __attribute__((annotate("_In_"))) and similar portable stubs.  The
                // spelling carries the SAL argument expression verbatim, e.g.
                // `_In_reads_(count)`, so split the name from its argument list.
                let spelling = child.name();
                let (name, arg) = split_sal_annotation(&spelling);
                apply_sal_string(name, &mut annotation);
                if annotation.size.is_none() {
                    annotation.size = capture_sal_size(name, arg);
                }
            }
            CXCursor_UnexposedAttr => {
                // __declspec(SAL_*) attributes produced by the real Windows SDK
                // sal.h when compiled with --target=x86_64-pc-windows-msvc.
                let tokens = tu.tokenize(tu.to_expansion_range(child.extent()));
                for (kind, spelling) in &tokens {
                    if *kind == CXToken_Identifier {
                        apply_sal_string(spelling, &mut annotation);
                    }
                }
            }
            _ => {}
        }
    }

    annotation
}

/// Split a SAL annotation spelling into its macro name and optional argument list.
///
/// `"_In_reads_(count)"` -> `("_In_reads_", Some("count"))`, `"_In_"` -> `("_In_", None)`.
fn split_sal_annotation(s: &str) -> (&str, Option<&str>) {
    match s.find('(') {
        Some(open) if s.ends_with(')') => (&s[..open], Some(&s[open + 1..s.len() - 1])),
        _ => (s, None),
    }
}

/// Capture buffer-size information from a SAL macro name and argument.
///
/// Returns `None` for non-sizing macros, for a missing argument, and for argument
/// expressions that are not a simple integer literal or identifier (e.g. `cb * 2`).
/// The metadata asserts a size only when the source expresses it unambiguously.
fn capture_sal_size(name: &str, arg: Option<&str>) -> Option<SalSize> {
    let bytes = sal_size_kind(name)?;
    // `_*_to_`/`_*_part_` variants take two arguments; the buffer extent is the first.
    let first = arg?.split(',').next()?.trim();
    Some(SalSize {
        bytes,
        arg: parse_size_arg(first)?,
    })
}

/// Classify a SAL macro name as a buffer-size annotation.
///
/// Returns `Some(true)` for byte-sized buffers (`*_bytes_*` -> `MemorySize`),
/// `Some(false)` for element-count buffers (`*_reads_`/`*_writes_`/`*_updates_` ->
/// `NativeArrayInfo`), and `None` for non-sizing macros.
fn sal_size_kind(name: &str) -> Option<bool> {
    let is_size =
        name.contains("_reads_") || name.contains("_writes_") || name.contains("_updates_");
    is_size.then(|| name.contains("_bytes"))
}

/// Parse a single SAL size argument into a [`SalSizeArg`].
///
/// Accepts a decimal/hex integer literal (-> `Const`) or a bare C identifier
/// (-> `Name`).  A count held in an in/out pointer parameter is written `*param`
/// in SAL (e.g. `_Out_writes_to_(*Length, ...)`); the referenced parameter is the
/// pointer itself, so a leading dereference is stripped before matching.
/// Anything else (arithmetic, casts, member access, ...) returns `None`.
fn parse_size_arg(s: &str) -> Option<SalSizeArg> {
    let s = s.trim();
    if let Some(n) = parse_int_literal(s) {
        Some(SalSizeArg::Const(n))
    } else {
        let name = s.trim_start_matches('*').trim();
        is_c_identifier(name).then(|| SalSizeArg::Name(name.to_string()))
    }
}

/// Parse a C integer literal (decimal or `0x` hex, with optional `u`/`l` suffix).
fn parse_int_literal(s: &str) -> Option<i32> {
    let t = s.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        i32::from_str_radix(hex, 16).ok()
    } else {
        t.parse::<i32>().ok()
    }
}

/// Returns `true` when `s` is a single bare C identifier.
fn is_c_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

/// Parse the `CXCursor_ParmDecl` children of `cursor` into a `Vec<Param>`.
///
/// Shared by free functions ([`Fn::parse`]) and COM interface methods
/// ([`Interface::parse`]); the two only differ in how they obtain `cursor` and
/// the MIDL fallback annotations, both computed by the caller.
///
/// For each parameter: the name falls back to `paramN` (declaration index) when
/// anonymous; the direction annotation is taken from the SAL attribute when
/// present, otherwise from `midl_annotations` (positional MIDL comment fallback);
/// the type is mapped via [`param_metadata_type`] (SAL-driven pointer const-ness,
/// alias unwrapping); and a
/// `_COM_Outptr_`/`[iid_is]` output (`ComOutPtr`) is erased to `void**` because
/// the concrete pointee is redundant (the caller selects it via the sibling
/// `REFIID`, and bindgen's `QueryInterface<T>()`/`Resolve<T>()` keys off the
/// generic `*mut *mut c_void` slot). Array `SalSize`s are then resolved.
/// Returns `true` when `ty` is a `*mut *mut void` (mutable pointer of total depth 2
/// to a `Void` pointee), whether depth-encoded (`PtrMut(Void, 2)`) or nested.
///
/// This is the shape of a caller-chosen-type COM out-pointer (`GetParent`,
/// `QueryInterface`, ...): the concrete type is selected at runtime via a sibling
/// `REFIID`, so a `_COM_Outptr_` on such a slot is a real `ComOutPtr`. A concrete
/// interface pointee (`IDXGIAdapter**` -> `*mut IDXGIAdapter`) is not this shape and
/// keeps its declared type.
fn is_void_double_ptr(ty: &metadata::Type) -> bool {
    let mut depth = 0usize;
    let mut cur = ty;
    loop {
        match cur {
            metadata::Type::PtrMut(inner, d) => {
                depth += *d;
                cur = inner;
            }
            metadata::Type::Void => return depth == 2,
            _ => return false,
        }
    }
}

/// Parameter names conventionally used for the `REFIID` that selects a caller-chosen
/// COM out-pointer's runtime type (`riid`/`iid`/`riidltf`). A `*const GUID` parameter so
/// named is the *type selector* of the `QueryInterface` idiom, not a data GUID.
const IID_SELECTOR_PARAM_NAMES: [&str; 3] = ["riid", "iid", "riidltf"];

/// Returns `true` when `ty` names the `HRESULT` scalar typedef.
fn is_hresult(ty: &metadata::Type) -> bool {
    matches!(ty, metadata::Type::ValueName(tn) if tn.name == "HRESULT")
}

/// Returns `true` when `ty` is a single-indirection `*const GUID` - the `REFIID`/`REFCLSID`
/// shape a caller-chosen-type COM creator uses to select its out-pointer's type. (`IID` and
/// friends are already collapsed to `GUID` by [`super::canon`].)
fn is_const_guid_ptr(ty: &metadata::Type) -> bool {
    matches!(ty, metadata::Type::PtrConst(inner, 1)
        if matches!(inner.as_ref(), metadata::Type::ValueName(tn) if tn.name == "GUID"))
}

/// Returns `true` when `ty` is a single-indirection out-pointer to a *base* COM interface -
/// `*mut IUnknown` or `*mut IInspectable`. Some caller-chosen-type creators
/// (`DWriteCreateFactory`, the `GetService` service-locators, OLE DB's
/// `CreateTableWithConstraints`) spell their `[iid_is]`-selected out-parameter as the base
/// interface `IUnknown**`/`IInspectable**` in the SDK header instead of the canonical
/// `void**` - an upstream header inconsistency. At the ABI a COM interface pointer is itself
/// a pointer, so `*mut IUnknown` is the same shape as `*mut *mut void`; recovering the marker
/// here lets bindgen project the ergonomic `QueryInterface` generic rather than a manual
/// `iid` + untyped `-> Result<IUnknown>` that forces a caller `.cast()`. A *concretely* typed
/// out (`IDWriteFactory**`, `IActivateAudioInterfaceAsyncOperation**`) is **not**
/// matched - there the `riid` selects a *different* object and the out must stay typed.
fn is_base_interface_out_ptr(ty: &metadata::Type) -> bool {
    let metadata::Type::PtrMut(inner, 1) = ty else {
        return false;
    };
    matches!(inner.as_ref(),
        metadata::Type::ClassName(tn) | metadata::Type::ValueName(tn)
            if tn.name == "IUnknown" || tn.name == "IInspectable")
}

/// Infer a `ComOutPtr` (`#[iid_is]`) marker on an *unannotated* caller-chosen-type COM
/// out-pointer.
///
/// Some QueryInterface-idiom creators (`DCompositionCreateDevice`, `D3D11CreateDevice`'s
/// interop factories, the `OleCreate*`/`OleLoad*` family, the property-system `PS*`
/// functions, ...) ship in the SDK headers with **no** `_COM_Outptr_` SAL and **no** MIDL
/// `[iid_is]` comment, so [`parse_params`] leaves the `void**` out-parameter a bare,
/// caller-opaque `*mut *mut c_void`. This recovers the marker from the surrounding
/// signature shape rather than a per-parameter annotation, so the projected surface matches
/// the annotated creators (`-> Result<T>` via `QueryInterface`) instead of a raw double
/// pointer.
///
/// The gate is narrow, so it is a *promotion* of a genuine idiom and never a
/// lossy guess: the function must return `HRESULT`, a `*const GUID` parameter must be named
/// `riid`/`iid`/`riidltf` (the `REFIID` convention), and the promoted parameter must be a
/// caller-chosen-type output with no array/buffer length (`size`/`array` unset) - either a
/// `*mut *mut void` ([`is_void_double_ptr`]) or a base-interface `*mut IUnknown`/
/// `*mut IInspectable` ([`is_base_interface_out_ptr`]). The `HRESULT`-return + `riid`-*name*
/// requirement is what excludes the buffer-style GUID-out functions whose GUID parameter is
/// *data*, not a type selector (`DsReplicaGetInfoW`'s `puuidForSourceDsaObjGuid`,
/// `RpcServerInqIf`'s `MgrTypeUuid`, `WlanQueryInterface`'s `pInterfaceGuid`,
/// `TdhCreatePayloadFilter`'s `ProviderGuid` - all returning `u32`/`RPC_STATUS`/`TDHSTATUS`,
/// not `HRESULT`). The array/length exclusion is what excludes the caller-chosen-type *array
/// enumerators* (`IEnumObjects::Next(celt, riid, size_is(celt) void **rgelt, ...)`) whose
/// `void**` is a counted array, not a single-object out-pointer - promoting those to a
/// `ComOutPtr` generic produces broken codegen (the projection would drop the array/count
/// parameter it still references). Restricting the base-interface arm to the *bare* base
/// spelling (never a concretely typed `IFoo**`) is what excludes the creators whose `riid`
/// selects a *different* object than the fixed-type out-pointer (`ActivateAudioInterfaceAsync`'s
/// `IActivateAudioInterfaceAsyncOperation**`, `RoGetAgileReference`'s `IAgileReference**`).
/// Adjacency is *not* required - the OLE family has several parameters between the `riid` and
/// the out-pointer. Only sets the marker (never clears one) and only on a not-already-`In`
/// out-pointer, so it composes with the SAL/MIDL promotion in [`parse_params`].
pub(crate) fn infer_iid_is(params: &mut [Param], return_type: &metadata::Type) {
    if !is_hresult(return_type) {
        return;
    }
    let has_iid_selector = params
        .iter()
        .any(|p| IID_SELECTOR_PARAM_NAMES.contains(&p.name.as_str()) && is_const_guid_ptr(&p.ty));
    if !has_iid_selector {
        return;
    }
    for param in params.iter_mut() {
        if !param.annotation.com_out_ptr
            && !param.annotation.in_param
            && param.annotation.array.is_none()
            && param.annotation.size.is_none()
            && (is_void_double_ptr(&param.ty) || is_base_interface_out_ptr(&param.ty))
        {
            param.annotation.com_out_ptr = true;
            // The out-pointer is an output; mark it so `param_attrs_for_annotation` renders
            // the direction like the annotated (`_COM_Outptr_`/`[iid_is]`) path rather than
            // defaulting a now-`is_annotated` mutable pointer to `#[in]`.
            param.annotation.out_param = true;
            // Normalise to the canonical `*mut *mut void` shape the SAL/MIDL promotion
            // emits (see [`parse_params`]), so an inferred `ComOutPtr` is byte-identical to
            // an annotated one in the RDL.
            param.ty = metadata::Type::PtrMut(
                Box::new(metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1)),
                1,
            );
        }
    }
}

pub(crate) fn parse_params(
    cursor: &Cursor,
    midl_annotations: &[ParamAnnotation],
    parser: &mut Parser<'_>,
) -> Vec<Param> {
    let mut params = vec![];
    let mut param_idx = 0usize;
    for child in cursor.children() {
        if child.kind() != CXCursor_ParmDecl {
            continue;
        }
        let mut name = child.name();
        if name.is_empty() || is_midl_synthetic_param_name(&name) {
            name = format!("param{param_idx}");
        }
        let sal_annotation = extract_param_annotation(&child, parser.tu);
        let mut annotation = if sal_annotation.is_annotated() {
            sal_annotation
        } else {
            midl_annotations.get(param_idx).cloned().unwrap_or_default()
        };
        let mut ty = param_metadata_type(&child.ty(), &annotation, parser);
        // Promote a speculative `_COM_Outptr_` token to a real `ComOutPtr` only when the
        // pointee is `void**` (the caller-chosen-type idiom): a concrete interface
        // pointee (e.g. `EnumAdapters` -> `IDXGIAdapter**`) has no sibling `REFIID` to
        // select the type and must keep it, matching the canonical
        // projection (`EnumAdapters -> Result<IDXGIAdapter>` vs `GetParent<T>`).
        if annotation.com_out_ptr_token && is_void_double_ptr(&ty) {
            annotation.com_out_ptr = true;
        }
        if annotation.com_out_ptr {
            ty = metadata::Type::PtrMut(
                Box::new(metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1)),
                1,
            );
        }
        // An inline fixed-size array parameter (`T name[N]`) decays to a pointer in
        // `param_metadata_type`; record its length as a `CountConst` so bindgen can
        // reconstruct the `&[T; N]` safe wrapper, matching the reference. Only when the
        // array's role is unambiguous - an explicit `[in]`/`[out]` direction or a `const`
        // (input) pointer: the reference leaves an unannotated raw output buffer (e.g. ICU
        // `ucnv_getStarters`) a bare pointer with no count. A SAL count, if any, takes
        // precedence and is resolved by `resolve_param_array_info` below.
        if annotation.size.is_none()
            && (annotation.in_param
                || annotation.out_param
                || matches!(ty, metadata::Type::PtrConst(..)))
            && let Some(n) = inline_array_param_count(&child.ty())
        {
            annotation.array = Some(ArrayInfo::CountConst(n));
        }
        param_idx += 1;
        params.push(Param {
            name,
            ty,
            annotation,
        });
    }
    resolve_param_array_info(&mut params);
    params
}

/// Resolve each parameter's captured [`SalSize`] into an emittable [`ArrayInfo`].
///
/// Parameter-name size references are resolved to 0-based parameter indices in
/// declaration order; integer-literal counts become `CountConst`.  References that
/// name no parameter - and constant byte counts, which have no bindgen sink - are
/// dropped, so the metadata asserts only what resolves unambiguously.
pub fn resolve_param_array_info(params: &mut [Param]) {
    let index_of: HashMap<&str, i16> = params
        .iter()
        .enumerate()
        .map(|(i, p)| (p.name.as_str(), i as i16))
        .collect();

    let resolved: Vec<Option<ArrayInfo>> = params
        .iter()
        .map(|p| {
            p.annotation.size.as_ref().and_then(|size| match &size.arg {
                SalSizeArg::Const(n) if !size.bytes => Some(ArrayInfo::CountConst(*n)),
                SalSizeArg::Const(_) => None,
                SalSizeArg::Name(name) => index_of.get(name.as_str()).map(|&idx| {
                    if size.bytes {
                        ArrayInfo::BytesParamIndex(idx)
                    } else {
                        ArrayInfo::CountParamIndex(idx)
                    }
                }),
            })
        })
        .collect();

    // A SAL-resolved count overrides any inline-array `CountConst` pre-set in `parse_params`;
    // when SAL resolves nothing, the inline count (if any) is preserved.
    for (p, info) in params.iter_mut().zip(resolved) {
        if info.is_some() {
            p.annotation.array = info;
        }
    }
}

/// Scan the token stream of a method or function for MIDL block-comment parameter
/// annotations, returning one [`ParamAnnotation`] per parameter in declaration order.
///
/// MIDL-generated headers place a block comment such as `/* [in] */`, `/* [out] */`, or
/// `/* [retval][out] */` immediately before each parameter's type in the source text.
/// Because this comment falls outside the `ParmDecl` cursor's extent, it cannot be
/// detected by tokenising individual `ParmDecl` cursors.  Instead, this function
/// tokenises the entire method or function extent - which is already done in
/// `interface.rs` for propget/propput detection - and maps each comment to its
/// corresponding parameter by position in the token stream.
///
/// In addition to the block comments, a `_COM_Outptr_` SAL macro that survives as a
/// bare identifier token (the abstract-virtual COM-method case, where clang does not
/// expose it as a `ParmDecl` `AnnotateAttr`) is applied through [`apply_sal_string`],
/// marking the parameter as a COM out-pointer so it emits `#[iid_is]`.
///
/// The algorithm:
/// 1. Skip tokens until the function/method name identifier is found.
/// 2. After the opening `(` of the parameter list, accumulate MIDL comment annotations.
/// 3. Each `,` (at depth 1) commits the accumulated annotation for the current parameter
///    and resets for the next.
/// 4. The closing `)` (back to depth 0) commits the last parameter's annotation.
///
/// Parameters without a preceding MIDL comment receive a default (all-false) annotation.
pub fn scan_method_param_annotations(
    tokens: &[(CXTokenKind, String)],
    method_name: &str,
) -> Vec<ParamAnnotation> {
    let mut result = Vec::new();
    let mut current = ParamAnnotation::default();
    let mut past_name = false;
    let mut paren_depth: i32 = 0;
    let mut in_params = false;

    for (kind, spelling) in tokens {
        if !past_name {
            if *kind == CXToken_Identifier && spelling == method_name {
                past_name = true;
            }
            continue;
        }

        match (*kind, spelling.as_str()) {
            (CXToken_Punctuation, "(") => {
                paren_depth += 1;
                if paren_depth == 1 {
                    in_params = true;
                    current = ParamAnnotation::default();
                }
            }
            (CXToken_Punctuation, ")") => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
                if paren_depth == 0 && in_params {
                    // Commit the last parameter, but skip pushing a spurious default for
                    // zero-/single-parameter functions with no MIDL comment.
                    if !result.is_empty() || current.is_annotated() {
                        result.push(current.clone());
                    }
                    break;
                }
            }
            (CXToken_Punctuation, ",") if in_params && paren_depth == 1 => {
                result.push(current.clone());
                current = ParamAnnotation::default();
            }
            (CXToken_Comment, s) if in_params && paren_depth == 1 => {
                apply_midl_param_comment(s, &mut current);
            }
            // A `_COM_Outptr_` SAL macro survives as a bare identifier token in the
            // MIDL-generated vtable declaration even when clang does not expose it as a
            // `ParmDecl` `AnnotateAttr` (the abstract-virtual COM-method case): the
            // block comment then carries only `[out]`/`[retval]`, never `[iid_is]`, so
            // the out-pointer would otherwise miss its `ComOutPtr` marker. Record the
            // token speculatively - `parse_params` promotes it to a real `ComOutPtr`
            // (`#[iid_is]`) only for a `void**` pointee (the caller-chosen-type idiom,
            // e.g. DXGI `GetParent`/`GetBuffer`), driving bindgen's generic
            // `GetParent<T>() -> Result<T>` projection; a concrete interface pointee
            // (e.g. `EnumAdapters` -> `IDXGIAdapter**`) keeps its declared type.
            //
            // The ComOutPtr marker, the (definitional) output direction, and the
            // optionality are recovered here: a `_COM_Outptr_` is always an
            // out-parameter, so set `out_param` to stop the direction logic emitting a
            // spurious `#[in]` on the mutable pointer (the SAL-only free-function case,
            // e.g. `CoCreateInstance`, has no MIDL `[out]` comment). The `_opt_` variants
            // (`_COM_Outptr_opt_`, `_COM_Outptr_opt_result_maybenull_`) additionally mark
            // the slot optional; `optional` is orthogonal to the deferred
            // `void**`-vs-concrete `ComOutPtr` decision (`com_out_ptr_token`, promoted in
            // `parse_params`), so capture it here - otherwise a `_COM_Outptr_opt_`
            // out-pointer recovered from the token stream (e.g. `D3D11CreateDevice`
            // `ppDevice`/`ppImmediateContext`) would silently drop its `[Optional]`.
            // Retval, if any, still comes from the MIDL block comment.
            (CXToken_Identifier, s)
                if in_params && paren_depth == 1 && s.starts_with("_COM_Outptr_") =>
            {
                current.out_param = true;
                current.com_out_ptr_token = true;
                if s.starts_with("_COM_Outptr_opt") {
                    current.optional = true;
                }
            }
            // A *pure* null-terminated string SAL macro (`_In_z_` and friends) likewise
            // survives as a bare identifier token in an abstract-virtual COM method
            // declaration where clang attaches no `ParmDecl` `AnnotateAttr` (e.g.
            // `IDWriteFactory::CreateTextFormat`). Recover its direction + null-terminated
            // bit through [`apply_sal_string`] so a raw `WCHAR*`/`char*` COM parameter is
            // promoted to `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` just like the free-function case.
            // The counted `_*_reads_z_`/`_*_writes_z_` variants are excluded by
            // `apply_sal_string` (they set no `null_terminated`), so a counted buffer stays
            // a raw pointer.
            (CXToken_Identifier, s)
                if in_params
                    && paren_depth == 1
                    && matches!(
                        s,
                        "_In_z_" | "_In_opt_z_" | "_Out_z_" | "_Inout_z_" | "_Inout_opt_z_"
                    ) =>
            {
                apply_sal_string(s, &mut current);
            }
            _ => {}
        }
    }

    result
}

/// Parse a MIDL block-comment string and apply the recognised annotations.
///
/// MIDL generates comments of the form `/* [in] */`, `/* [retval][out] */`, etc.
/// before each parameter.  This function checks for the well-known attribute names
/// using simple substring matching; unknown names are silently ignored.
pub fn apply_midl_param_comment(comment: &str, annotation: &mut ParamAnnotation) {
    if comment.contains("[in]") {
        annotation.in_param = true;
    }
    if comment.contains("[out]") {
        annotation.out_param = true;
    }
    if comment.contains("[retval]") {
        annotation.retval = true;
    }
    if comment.contains("[optional]") {
        annotation.optional = true;
    }
    // `[iid_is]` marks an interface out-pointer whose IID is supplied by a sibling
    // `REFIID` parameter - the QueryInterface / Resolve / CoCreateInstance idiom.
    // MIDL-generated vtable declarations express this only through the block comment
    // (there is no `_COM_Outptr_` SAL on the abstract method), so map an `[iid_is]`
    // output to the same `ComOutPtr` marker `_COM_Outptr_` yields. This is what drives
    // bindgen's ergonomic `QueryInterface<T>()` / `Resolve<T>() -> Result<T>`
    // projection; without it such methods degrade to a raw `*mut *mut c_void` out-param.
    if comment.contains("[iid_is]") && annotation.out_param {
        annotation.com_out_ptr = true;
    }
}

/// Map a SAL macro name to the corresponding [`ParamAnnotation`] flags.
///
/// Only the most common Win32 SAL macros are handled.  Unknown names are silently
/// ignored so that non-SAL `__attribute__((annotate(...)))` annotations on parameters
/// do not cause spurious errors.
fn apply_sal_string(sal: &str, annotation: &mut ParamAnnotation) {
    match sal {
        // --- Input-only ---
        "_In_" | "_In_z_" | "_In_nz_" | "_In_reads_" | "_In_reads_bytes_" | "_In_reads_z_"
        | "_In_reads_or_z_" | "_In_bytecount_" | "_In_count_" => {
            annotation.in_param = true;
        }

        // --- Input-only, optional ---
        "_In_opt_"
        | "_In_opt_z_"
        | "_In_reads_opt_"
        | "_In_reads_bytes_opt_"
        | "_In_reads_or_z_opt_" => {
            annotation.in_param = true;
            annotation.optional = true;
        }

        // --- Output-only ---
        "_Out_"
        | "_Out_z_"
        | "_Out_writes_"
        | "_Out_writes_bytes_"
        | "_Out_writes_z_"
        | "_Out_writes_all_"
        | "_Out_writes_bytes_all_"
        | "_Out_writes_to_"
        | "_Out_writes_bytes_to_"
        | "_Out_writes_to_ptr_"
        | "_Out_writes_to_ptr_z_"
        | "_Outptr_"
        | "_COM_Outptr_"
        | "_COM_Outptr_result_maybenull_"
        | "_Outptr_result_z_"
        | "_Outptr_result_buffer_"
        | "_Outptr_result_bytebuffer_" => {
            annotation.out_param = true;
        }

        // --- Output-only, optional ---
        "_Out_opt_"
        | "_Out_writes_opt_"
        | "_Out_writes_bytes_opt_"
        | "_Out_writes_all_opt_"
        | "_Out_writes_bytes_all_opt_"
        | "_Out_writes_to_opt_"
        | "_Out_writes_bytes_to_opt_"
        | "_Out_writes_opt_z_"
        | "_Out_writes_to_ptr_opt_"
        | "_Out_writes_to_ptr_opt_z_"
        | "_Outptr_opt_"
        | "_COM_Outptr_opt_"
        | "_COM_Outptr_opt_result_maybenull_"
        | "_Outptr_opt_result_z_"
        | "_Outptr_result_maybenull_"
        | "_Outptr_opt_result_maybenull_" => {
            annotation.out_param = true;
            annotation.optional = true;
        }

        // --- In + Out ---
        "_Inout_"
        | "_Inout_z_"
        | "_Inout_updates_"
        | "_Inout_updates_bytes_"
        | "_Inout_updates_z_"
        | "_Inout_updates_all_"
        | "_Inout_updates_bytes_all_"
        | "_Inout_updates_to_"
        | "_Inout_updates_bytes_to_" => {
            annotation.in_param = true;
            annotation.out_param = true;
        }

        // --- In + Out, optional ---
        "_Inout_opt_"
        | "_Inout_opt_z_"
        | "_Inout_updates_opt_"
        | "_Inout_updates_opt_z_"
        | "_Inout_updates_bytes_opt_"
        | "_Inout_updates_all_opt_"
        | "_Inout_updates_bytes_all_opt_"
        | "_Inout_updates_to_opt_"
        | "_Inout_updates_bytes_to_opt_" => {
            annotation.in_param = true;
            annotation.out_param = true;
            annotation.optional = true;
        }

        _ => {}
    }

    // Orthogonal markers, independent of direction.  `_Reserved_` asserts no
    // direction (the type-inferred default applies); every `_COM_Outptr_`
    // variant additionally marks the parameter as a COM out-pointer.
    if sal == "_Reserved_" {
        annotation.reserved = true;
    }

    // A *pure* null-terminated string annotation (no explicit element count) promotes a
    // raw `WCHAR*`/`char*` parameter to the canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR`
    // wrapper. The counted `_*_reads_z_`/`_*_writes_z_`/`_*_updates_z_` and the
    // `_Outptr_*_result_z_` (a `T**`) variants are intentionally excluded: they carry a
    // `NativeArrayInfo`/out-pointer shape and stay raw pointers (e.g. `CreateTextLayout`).
    if matches!(
        sal,
        "_In_z_" | "_In_opt_z_" | "_Out_z_" | "_Inout_z_" | "_Inout_opt_z_"
    ) {
        annotation.null_terminated = true;
    }
    if sal.starts_with("_COM_Outptr_") {
        annotation.com_out_ptr = true;
    }
}

/// Compute RDL attribute tokens for a parameter given its annotation and type.
///
/// Returns a `Vec<TokenStream>` containing zero or more of `#[r#in]`, `#[out]`,
/// `#[opt]`, and `#[retval]` that should be placed before the parameter name.
///
/// The emission rules mirror those of [`writer::write_params`] so that the
/// clang -> RDL -> winmd -> RDL roundtrip is stable: attributes emitted here are
/// the same attributes that the writer would re-emit after encoding into winmd.
///
/// When `annotation` has no flags set (no SAL or MIDL annotation was detected),
/// no attributes are emitted (the reader will infer direction from the type).
pub fn param_attrs_for_annotation(
    annotation: &ParamAnnotation,
    ty: &metadata::Type,
) -> Vec<TokenStream> {
    if !annotation.is_annotated() {
        return vec![];
    }

    let in_param = annotation.in_param;
    let out_param = annotation.out_param;
    let optional = annotation.optional;
    let retval = annotation.retval;

    let is_mutable = matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..));

    // effective_in: treat as In when explicitly in or when not explicitly out
    let effective_in = in_param || !out_param;

    let mut attrs = vec![];

    // Custom Foundation.Metadata attributes are emitted before the direction
    // attributes, matching the writer's ordering so the clang -> RDL -> winmd -> RDL
    // roundtrip is stable.
    if let Some(array) = &annotation.array {
        attrs.push(array_info_attr(array));
    }

    if annotation.reserved {
        attrs.push(quote! { #[reserved] });
    }

    // A COM out-pointer originates from an `[iid_is]` MIDL annotation (or the
    // `_COM_Outptr_` SAL that yields the same shape). Emit it under the IDL/SAL
    // source spelling `#[iid_is]`; the RDL reader maps it to the metadata
    // `ComOutPtrAttribute`, keeping the committed metadata a literal mirror of the
    // header while the metadata-vocabulary mapping lives in one place downstream.
    if annotation.com_out_ptr {
        attrs.push(quote! { #[iid_is] });
    }

    // Emit #[r#in] only when it is needed to override the default direction:
    //   - always when the param is both In and Out
    //   - when In but the type is mutable (default would otherwise be Out)
    if effective_in && (out_param || is_mutable) {
        attrs.push(quote! { #[r#in] });
    }

    // Emit #[out] only when it is needed to override the default direction:
    //   - always when the param is both In and Out
    //   - when Out but the type is not mutable (default would otherwise be In)
    if out_param && (effective_in || !is_mutable) {
        attrs.push(quote! { #[out] });
    }

    if optional {
        attrs.push(quote! { #[opt] });
    }

    if retval {
        attrs.push(quote! { #[retval] });
    }

    attrs
}

/// Detect whether a function declaration is marked no-return.
///
/// Recognises `__declspec(noreturn)` / `[[noreturn]]` - which fold into the function
/// type spelling (`void (int) __attribute__((noreturn))`) - and the
/// `_Analysis_noreturn_` SAL annotation, which appears as an `AnnotateAttr` child.
/// Both lower to the `DoesNotReturn` attribute, which bindgen renders as `-> !`.
pub fn detect_does_not_return(cursor: &Cursor) -> bool {
    if cursor.ty().spelling().contains("noreturn") {
        return true;
    }
    cursor
        .children()
        .iter()
        .any(|c| c.kind() == CXCursor_AnnotateAttr && c.name() == "_Analysis_noreturn_")
}

/// Build the `#[noreturn]` pseudo-attribute (the source `[[noreturn]]` / `_Analysis_noreturn_`
/// spelling); the RDL reader maps it to the metadata `DoesNotReturnAttribute`.
pub fn does_not_return_attr() -> TokenStream {
    quote! { #[noreturn] }
}

/// Build the `#[len_param(N)]` / `#[len_const(N)]` / `#[size_param(N)]` pseudo-attribute for a
/// resolved [`ArrayInfo`]; the RDL reader maps these to the metadata `NativeArrayInfoAttribute`
/// (`CountParamIndex`/`CountConst`) / `MemorySizeAttribute` (`BytesParamIndex`). The index/const
/// argument is carried through as a single positional value.
fn array_info_attr(info: &ArrayInfo) -> TokenStream {
    let (name, lit) = match info {
        ArrayInfo::CountParamIndex(i) => ("len_param", Literal::i16_unsuffixed(*i)),
        ArrayInfo::CountConst(n) => ("len_const", Literal::i32_unsuffixed(*n)),
        ArrayInfo::BytesParamIndex(i) => ("size_param", Literal::i16_unsuffixed(*i)),
    };

    let name = write_ident(name);
    quote! { #[#name(#lit)] }
}
