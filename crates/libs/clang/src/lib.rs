#![allow(non_upper_case_globals)]
#![doc = include_str!("../readme.md")]

use std::collections::{BTreeMap, HashMap, HashSet};
use windows_metadata as metadata;

use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;

use windows_rdl::emit::{uuid_to_u128_literal, write_ident, write_type, write_value};
use windows_rdl::{Error, expand_input_paths, formatter, implib, write_to_file};

mod cx;
use cx::*;
mod canon;
use canon::*;
mod r#enum;
use r#enum::*;
mod item;
use item::*;
mod r#struct;
use r#struct::*;
mod collector;
use collector::*;
use field::*;
mod annotation;
mod field;
use annotation::*;
mod typedef;
use typedef::*;
mod callback;
use callback::*;
mod r#fn;
use r#fn::*;
mod r#const;
use r#const::*;
mod interface;
use interface::*;
mod provision;
pub use provision::*;
mod scrape;
pub use scrape::*;
mod guid;
use guid::*;
mod scope;
use scope::*;
mod naming;
use naming::*;
mod macros;
use macros::*;

/// Creates a [`Clang`] that generates RDL from C/C++ headers using libclang.
pub fn clang() -> Clang {
    Clang::new()
}

/// Returns the version string of the loaded libclang library,
/// e.g. `"clang version 18.1.0 (...)"`.
pub fn clang_version() -> Result<String, Error> {
    Clang::version()
}

/// Shared parse context that is threaded through all `parse` methods in the
/// clang module, eliminating the need to pass a fixed set of parameters
/// (`namespace`, `library`, `ref_map`, `tag_rename`, `tu`) individually to
/// every call.
///
/// `pending_typedefs` and `pending_macros` accumulate side-effects during the
/// main AST walk and are consumed by [`Clang::process_tu`] after the walk
/// completes.
pub(crate) struct Parser<'a> {
    pub namespace: &'a str,
    /// When `Some(root)`, the parser is in **per-header** mode: cross-type
    /// references resolve to the *defining header* of the referenced declaration
    /// (`<root>.<HeaderStem>`) instead of consulting [`ref_map`](Self::ref_map),
    /// and every declaration is emitted into its own header partition. This is the
    /// faithful, source-expressed partitioning that replaces editorial namespaces.
    pub header_root: Option<&'a str>,
    pub library: &'a str,
    /// Per-symbol DLL overrides recovered from the SDK import libraries; a
    /// function whose name is present here is stamped with that DLL instead of
    /// the fallback [`library`](Self::library).
    pub libraries: &'a HashMap<String, String>,
    pub ref_map: &'a HashMap<String, String>,
    /// Per-header mode only: a global map of every declaration's emitted name to
    /// its defining-header namespace, used to resolve *token-based* const casts
    /// (`#define INVALID_ATOM ((ATOM)0)`) whose type is only a spelled name with no
    /// clang cursor to route via [`header_root`](Self::header_root). `None` in
    /// legacy mode (const casts then resolve via [`ref_map`](Self::ref_map)).
    pub header_names: Option<&'a HashMap<String, String>>,
    pub tag_rename: &'a HashMap<String, String>,
    /// Public integer typedef names (`SHCONTF`, `TASKDIALOG_FLAGS`, ...) that absorbed a
    /// sibling `enum _FOO` via the C flags/enum idiom, mapped to the enum `repr` the
    /// typedef's storage type dictates. Populated by [`naming::merge_enum_typedef_idiom`];
    /// the redundant typedef is dropped in [`Typedef::parse`] and the renamed enum adopts
    /// this repr at emission.
    pub enum_merge: &'a HashMap<String, &'static str>,
    pub tu: &'a TranslationUnit,
    pub pending_typedefs: Vec<Cursor>,
    pub pending_macros: Vec<String>,
    /// Per-header mode only: `(namespace, name)` pairs for incomplete record types
    /// that are referenced through a pointer but never defined anywhere in the
    /// translation unit (e.g. the CRT's `struct __crt_locale_data`). Drained after
    /// the pass to emit opaque empty structs so those references resolve.
    pub pending_opaque: Vec<(String, String)>,
    /// Enum names for which `DEFINE_ENUM_FLAG_OPERATORS(X)` was seen.
    pub flag_enums: HashSet<String>,
    /// IID variables: maps interface name → UUID string (e.g. `"IFoo"` → `"23170f69-40c1-278a-0000-000300010000"`).
    /// Populated from `extern "C" const GUID IID_XXX = { ... };` variable declarations.
    pub iid_vars: HashMap<String, String>,
    /// Object-like macro definitions, mapping each macro name to the spellings of
    /// its replacement-list tokens.  Used to resolve calling-convention macros
    /// (e.g. `WINAPI` → `__stdcall`) back to the underlying compiler keyword,
    /// transitively, regardless of which header defined them.
    pub macro_defs: &'a HashMap<String, Vec<String>>,
    /// Reverse map from a function's macro-expanded export symbol to the source
    /// (pre-expansion) spelling it is declared under, derived from object-like alias
    /// macros (`#define RtlGenRandom SystemFunction036`,
    /// `#define EnumProcesses K32EnumProcesses`). clang reports the expanded export name
    /// for such a declaration, losing both the documented name and — because the
    /// convention keyword no longer anchors on the name token — the calling convention;
    /// [`Fn::parse`] consults this map to recover both, emitting the function under its
    /// documented name with the raw export recorded as the P/Invoke import name.
    ///
    /// ANSI/Unicode charset-selection macros (`#define GetWindowText GetWindowTextA`) are
    /// deliberately excluded: those select a `-A`/`-W` variant rather than forwarding to a
    /// differently-named export, and the reference metadata emits only the explicit
    /// `…A`/`…W` functions, never the bare name.
    pub alias_map: HashMap<String, String>,
    /// Symbol allowlist. When non-empty, only functions whose name is listed are
    /// emitted as roots and every other root (types, consts, macros, GUIDs) is
    /// suppressed; the allowlisted functions' transitive type/const closure still
    /// arrives via [`pending_typedefs`](Self::pending_typedefs) and the reference
    /// dedup. Empty (the default) leaves emission unrestricted. Used to carve a
    /// handful of symbols out of an enormous header (e.g. `RtlGetVersion` from the
    /// WDK's `wdm.h`) without dragging in the header's whole surface.
    pub symbols: &'a HashSet<String>,
    /// When `true`, a function that resolves to an empty import library is dropped rather
    /// than emitted with `#[library("")]`, matching the reference metadata (which carries no
    /// lib-less function). Off by default so unit-test fixtures that supply no import
    /// libraries keep emitting their functions; `tool_win32` opts in.
    pub drop_lib_less: bool,
    /// Per-header mode only: the set of `Namespace.Name` type full-names defined in the
    /// resolution winmd (`Windows.winmd`), backtick-arity-stripped. Used to classify a
    /// type declared in the `ABI::Windows::*` C++/WinRT projection namespace: a name present
    /// here is a *true* WinRT projection of a `Windows.winmd` type and is mapped to a
    /// cross-winmd reference (never flattened into `Windows.Win32`), while a name *absent*
    /// here is a Win32 COM interop entity that merely lives in the ABI namespace (e.g.
    /// `IActivatableClassRegistration`) and is scraped into the flat root. `None`/empty (no
    /// resolution winmd, the default) leaves the whole `ABI` namespace skipped as before.
    pub winrt_types: Option<&'a HashSet<String>>,
}

/// Per-namespace parameters for a single emission pass over a parsed translation
/// unit. The same parsed TU can be walked once per namespace with a different spec,
/// so a large shared header set (`windows.h` + every API header) is parsed only once
/// and each namespace selects its own declarations via [`filter`](Self::filter).
struct NamespaceSpec<'a> {
    namespace: &'a str,
    library: &'a str,
    libraries: &'a HashMap<String, String>,
    filter: &'a [String],
    symbols: &'a HashSet<String>,
}

impl<'a> Parser<'a> {
    #[expect(clippy::too_many_arguments)]
    fn new(
        namespace: &'a str,
        library: &'a str,
        libraries: &'a HashMap<String, String>,
        ref_map: &'a HashMap<String, String>,
        tag_rename: &'a HashMap<String, String>,
        enum_merge: &'a HashMap<String, &'static str>,
        macro_defs: &'a HashMap<String, Vec<String>>,
        tu: &'a TranslationUnit,
        symbols: &'a HashSet<String>,
    ) -> Self {
        Self {
            namespace,
            header_root: None,
            library,
            libraries,
            ref_map,
            header_names: None,
            tag_rename,
            enum_merge,
            tu,
            pending_typedefs: vec![],
            pending_macros: vec![],
            pending_opaque: vec![],
            flag_enums: HashSet::new(),
            iid_vars: HashMap::new(),
            alias_map: build_alias_map(macro_defs),
            macro_defs,
            symbols,
            drop_lib_less: false,
            winrt_types: None,
        }
    }

    /// Inserts a parsed function into `collector`, unless [`drop_lib_less`](Self::drop_lib_less)
    /// is set and the function resolved to an empty import library — in which case it is a
    /// lib-less residue the reference metadata omits (inline intrinsic, header-only
    /// presence-check, RPC internal, or a DLL export with no SDK import library) and is dropped.
    fn insert_fn(&self, item: Fn, collector: &mut Collector) {
        if self.drop_lib_less && item.library.is_empty() {
            return;
        }
        collector.insert(Item::Fn(item));
    }

    /// Process a single cursor: insert the corresponding [`Item`] into
    /// `collector` or record the name in `pending_macros` for the second-pass
    /// evaluator.  `extern_c` is `true` when the cursor was found inside an
    /// `extern "C" { }` block (relevant only for function declarations).
    fn process_cursor(
        &mut self,
        child: Cursor,
        collector: &mut Collector,
        extern_c: bool,
    ) -> Result<(), Error> {
        // Symbol-allowlist mode: when an allowlist is configured, emit only the
        // named functions as roots and suppress every other root (types, consts,
        // macros, GUIDs). The allowlisted functions' transitive type/const closure
        // still arrives through `pending_typedefs` (drained after the walk) and is
        // deduplicated against the reference winmd, so a handful of symbols can be
        // carved out of an enormous header without emitting its whole surface. Gated
        // on a non-empty allowlist, so the default corpus scrape is unaffected.
        //
        // Closure limitation: `pending_typedefs` schedules only *typedef'd* and
        // *callback* dependencies (see `cx.rs`'s `CXType_Typedef` arm and the drain in
        // `process_tu`), not bare `struct`/`enum` *tag* references. A closure that
        // reaches a record/enum type which is neither in the reference winmd nor named
        // through a typedef would leave a dangling reference — but that fails loudly at
        // the `reader()` compile step (`validate_use_declarations`), never silently. For
        // the current allowlist (`RtlGetVersion`, whose closure is entirely Win32) this
        // does not arise; extend the drain, or add the type to the reference, before
        // carving out a symbol whose closure needs a WDK-only record/enum.
        if !self.symbols.is_empty() {
            match child.kind() {
                CXCursor_FunctionDecl if !child.is_definition() => {
                    if self.symbols.contains(&child.name())
                        && !is_midl_proxy_stub(&child, self.libraries)
                    {
                        let item = Fn::parse(child, self, extern_c)?;
                        self.insert_fn(item, collector);
                    }
                }
                CXCursor_LinkageSpec => {
                    for inner in child.children() {
                        let inner_extern_c = inner.language() == CXLanguage_C;
                        self.process_cursor(inner, collector, inner_extern_c)?;
                    }
                }
                _ => {}
            }
            return Ok(());
        }
        match child.kind() {
            CXCursor_StructDecl if child.is_definition() => {
                // Recursively lift any named or anonymous nested struct/union
                // declarations to the collector before processing the outer struct
                // so that field type references to those nested types are already
                // registered.
                self.process_nested_types(child, collector, extern_c)?;
                // A directly-inlined anonymous record (the C anonymous aggregate
                // idiom) is emitted inline by its enclosing record as
                // `Anonymous: struct { … }` (see `Struct::parse`), so it must not
                // also be hoisted to a top-level `{Outer}_{n}` sibling. Its own
                // non-anonymous nested descendants were still collected above.
                if child.is_anonymous_record() || is_named_instance_record(&child) {
                    return Ok(());
                }
                let tag_name = child.name();
                let name = if is_anonymous_name(&tag_name) {
                    self.tag_rename
                        .get(&child.location_id())
                        .cloned()
                        .unwrap_or(tag_name)
                } else {
                    self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name)
                };
                // Skip anonymous types that were not given a synthetic name (e.g.
                // an anonymous struct that is not nested inside any named type).
                if is_anonymous_name(&name) {
                    // nothing to emit
                } else if child.has_pure_virtual_methods()
                    || child.extract_uuid(self.tu).is_some()
                    || (child.has_interface_base() && !child.has_data_fields())
                {
                    if !self.ref_map.contains_key(&name) {
                        collector.insert(Item::Interface(Interface::parse(child, self)?));
                    }
                } else if !self.ref_map.contains_key(&name) {
                    collector.insert(Item::Struct(Struct::parse(child, self, false)?));
                }
            }
            // A struct/union that is only ever forward-declared, with no definition
            // anywhere in the translation unit, is a genuinely opaque type referenced
            // through a pointer (e.g. `struct NDR_ALLOC_ALL_NODES_CONTEXT;`). Emit it as
            // an empty struct so those pointer references resolve. Handle tags (`<name>__`)
            // are excluded — they back the DECLARE_HANDLE idiom and emit as `*mut void`.
            CXCursor_StructDecl | CXCursor_UnionDecl
                if !child.is_definition() && !child.has_definition() =>
            {
                let tag_name = child.name();
                if !is_anonymous_name(&tag_name) && !tag_name.ends_with("__") {
                    let name = self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
                    // Only fill a genuine gap: never clobber a real definition that another
                    // tag may alias to the same public name (e.g. propidlbase.h's two
                    // `#ifdef`'d `PROPVARIANT` typedefs, one of which is never defined).
                    if !self.ref_map.contains_key(&name) && !collector.contains_key(&name) {
                        collector.insert(Item::Struct(Struct::opaque(&name)));
                    }
                }
            }
            CXCursor_UnionDecl if child.is_definition() => {
                // Recursively lift any named or anonymous nested struct/union
                // declarations to the collector before processing the outer union.
                self.process_nested_types(child, collector, extern_c)?;
                if child.is_anonymous_record() || is_named_instance_record(&child) {
                    return Ok(());
                }
                let tag_name = child.name();
                let name = if is_anonymous_name(&tag_name) {
                    self.tag_rename
                        .get(&child.location_id())
                        .cloned()
                        .unwrap_or(tag_name)
                } else {
                    self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name)
                };
                if !is_anonymous_name(&name) && !self.ref_map.contains_key(&name) {
                    collector.insert(Item::Struct(Struct::parse(child, self, true)?));
                }
            }
            CXCursor_ClassDecl
                if child.is_definition()
                    && (child.has_pure_virtual_methods()
                        || child.extract_uuid(self.tu).is_some()
                        || (child.has_interface_base() && !child.has_data_fields())) =>
            {
                let tag_name = child.name();
                let name = self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
                if !self.ref_map.contains_key(&name) {
                    collector.insert(Item::Interface(Interface::parse(child, self)?));
                }
            }
            // A C++ class that is only forward-declared (no body `{}`) but carries a
            // `__declspec(uuid("..."))` attribute should be emitted as a GUID constant.
            // This handles the MIDL pattern for COM server activation CLSIDs, e.g.:
            //   class DECLSPEC_UUID("e6756135-...") DiaSource;
            // Only emit when no definition for the class exists anywhere in the TU,
            // to avoid adding a GUID constant for a class that is separately defined.
            CXCursor_ClassDecl if !child.is_definition() && !child.has_definition() => {
                if let Some(uuid) = child.extract_uuid(self.tu) {
                    let tag_name = child.name();
                    let name = self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
                    if !name.is_empty() && !self.ref_map.contains_key(&name) {
                        collector.insert(Item::GuidConst(GuidConst { name, uuid }));
                    }
                }
            }
            CXCursor_EnumDecl if child.is_definition() => {
                let mut e = Enum::parse(child)?;
                let tag = e.name.clone();
                // Use the public typedef alias if one exists (e.g. `_EXCEPTION_DISPOSITION`
                // → `EXCEPTION_DISPOSITION`), matching how references and structs resolve
                // the name. Without this the enum definition would keep its internal tag
                // while every reference uses the alias, leaving a dangling type.
                if !is_anonymous_name(&e.name)
                    && let Some(alias) = self.tag_rename.get(&e.name)
                {
                    e.name.clone_from(alias);
                }
                if is_anonymous_name(&e.name) || is_midl_anonymous_enum_name(&e.name) {
                    // Unnamed enums (libclang spelling like "(unnamed enum at file.h:6:1)")
                    // and MIDL-synthesised names like `__MIDL___MIDL_itf_<...>` are emitted
                    // as top-level RDL constants rather than a named enum type.
                    for (name, value) in e.variants {
                        let const_value = enum_variant_value(e.repr, value);
                        collector.insert(Item::Const(Const {
                            name,
                            value: const_value,
                        }));
                    }
                } else if !self.ref_map.contains_key(&e.name) {
                    // If DEFINE_ENUM_FLAG_OPERATORS was seen before the enum
                    // definition (unusual but possible), mark it now. The macro may key
                    // on either the internal tag or the renamed public name, so check
                    // both — the flags/enum merge renames `_FOO` to `FOO` before this.
                    if self.flag_enums.contains(&e.name) || self.flag_enums.contains(&tag) {
                        e.flags = true;
                    }
                    // The C flags/enum idiom renamed `_FOO` to the public integer
                    // typedef `FOO`; adopt that typedef's storage type over the enum's
                    // accidental `int`. See `merge_enum_typedef_idiom`.
                    if let Some(&repr) = self.enum_merge.get(&e.name) {
                        e.repr = repr;
                    }
                    collector.insert(Item::Enum(e));
                }
            }
            CXCursor_TypedefDecl if child.is_definition() => {
                let name = child.name();
                if !self.ref_map.contains_key(&name) {
                    if let Some(cb) = Callback::parse(child, self)? {
                        collector.insert(Item::Callback(cb));
                    } else if let Some(td) = Typedef::parse(child, self)? {
                        collector.insert(Item::Typedef(td));
                    }
                }
            }
            CXCursor_FunctionDecl if !child.is_definition() => {
                // Skip MIDL-generated marshaling plumbing — the `_Proxy`/`_Stub` method
                // thunks (`is_midl_proxy_stub`) and the per-type `_User*` wire-marshaling
                // helpers (`is_midl_user_marshal_stub`): RPC internals, not public API.
                if !is_midl_proxy_stub(&child, self.libraries) && !is_midl_user_marshal_stub(&child)
                {
                    let item = Fn::parse(child, self, extern_c)?;
                    self.insert_fn(item, collector);
                }
            }
            // An `extern "C"` / `extern "C++"` block. MIDL-generated headers wrap all
            // declarations this way, and these blocks may be nested. Items from
            // transitively included headers are filtered out by the ref_map check.
            CXCursor_LinkageSpec => {
                for inner in child.children() {
                    let inner_extern_c = inner.language() == CXLanguage_C;
                    self.process_cursor(inner, collector, inner_extern_c)?;
                }
            }
            CXCursor_MacroDefinition => {
                if let Some(c) = Const::parse(child, self)? {
                    collector.insert(Item::Const(c));
                } else if !child.is_macro_builtin()
                    && !child.is_macro_function_like()
                    && !child.name().is_empty()
                    && !child.name().starts_with('_')
                {
                    // Skip macros whose body contains a non-type keyword or a
                    // string literal. Such macros (e.g. `#define EXTERN_C extern
                    // "C"`) are not integer constant expressions and the evaluator
                    // would emit bogus zero constants for them. Builtin *type*
                    // keywords are allowed through: casts like `((int)0x80000000)`
                    // (e.g. `CW_USEDEFAULT`) are valid constant expressions that the
                    // batch evaluator can resolve.
                    let tokens = self.tu.tokenize(child.extent());
                    let body_has_non_type_keyword = tokens
                        .iter()
                        .skip(1) // first token is the macro name
                        .any(|(kind, spelling)| {
                            *kind == CXToken_Keyword && !is_type_keyword(spelling)
                        });
                    let body_has_string_literal = tokens.iter().skip(1).any(|(kind, spelling)| {
                        *kind == CXToken_Literal
                            && (spelling.starts_with('"') || spelling.starts_with("L\""))
                    });
                    // Skip macros with a 128-bit integer literal (MSVC `i128`/`ui128`
                    // suffix, e.g. `#define INT128_MAX 170141183460469231731687303715884105727i128`
                    // in `intsafe.h`, which clang accepts under `-fms-extensions`). The metadata
                    // `Value` type has no 128-bit variant and `clang_getEnumConstantDeclValue`
                    // returns a 64-bit `long long`, so the batch evaluator would silently
                    // truncate such a constant to a wrong value (`INT128_MAX` -> `-1`).
                    let body_has_int128_literal = tokens.iter().skip(1).any(|(kind, spelling)| {
                        *kind == CXToken_Literal && spelling.to_ascii_lowercase().ends_with("i128")
                    });
                    // A valid integer constant expression has balanced delimiters.
                    // A macro whose replacement list has unbalanced parentheses or
                    // braces (e.g. a `#define ...END... }, };` initializer tail) is,
                    // once interpolated into the batch evaluator's synthetic
                    // `enum { __rdl_eval_X = (X) };`, able to swallow the enum
                    // declarations that follow it up to a matching delimiter —
                    // silently dropping every constant emitted after it. Reject
                    // such macros so they cannot poison their neighbours. (The
                    // evaluator also recovers from any residual swallow, but this
                    // cheap pre-filter keeps that rare and fast.)
                    let body_is_balanced = tokens_balanced(tokens.iter().skip(1));
                    if !body_has_non_type_keyword
                        && !body_has_string_literal
                        && !body_has_int128_literal
                        && body_is_balanced
                    {
                        // The token parser returned None for a candidate
                        // object-like macro.  Defer to the batch evaluator.
                        self.pending_macros.push(child.name());
                    }
                }
            }
            // Detect DEFINE_ENUM_FLAG_OPERATORS(EnumName) macro invocations.
            // These mark an enum as a bitfield flags type, which causes `#[flags]`
            // to be emitted in the RDL output for that enum.
            CXCursor_MacroExpansion if child.name() == "DEFINE_ENUM_FLAG_OPERATORS" => {
                // Tokenize the invocation to extract the enum name argument.
                // Expected token sequence:
                //   [0] DEFINE_ENUM_FLAG_OPERATORS  (identifier, the macro name)
                //   [1] (                            (punctuation)
                //   [2] EnumName                    (identifier, the argument)
                //   [3] )                            (punctuation)
                let tokens = self.tu.tokenize(child.extent());
                if let [
                    _,
                    (CXToken_Punctuation, lp),
                    (CXToken_Identifier, enum_name),
                    ..,
                ] = tokens.as_slice()
                    && lp == "("
                {
                    let enum_name = enum_name.clone();
                    // The flags/enum merge (and the inline tag→typedef idiom) may rename
                    // the enum before it is inserted, while `DEFINE_ENUM_FLAG_OPERATORS`
                    // can key on the internal tag (`_SVGIO`); resolve the argument to the
                    // enum's emitted public name so the mark lands on the right type.
                    let enum_name = self
                        .tag_rename
                        .get(&enum_name)
                        .cloned()
                        .unwrap_or(enum_name);
                    // Mark the enum in the collector if already inserted.
                    collector.mark_flags(&enum_name);
                    // Also record for the case where the enum definition
                    // comes after the macro invocation.
                    self.flag_enums.insert(enum_name);
                }
            }
            // Detect `DEFINE_GUID(name, ...)` / `DEFINE_OLEGUID(name, ...)` macro
            // invocations and emit them as named GUID constants. Without `INITGUID`
            // these expand to a valueless `extern const GUID name` declaration, so the
            // faithful value lives only in the macro arguments parsed here.
            CXCursor_MacroExpansion
                if matches!(child.name().as_str(), "DEFINE_GUID" | "DEFINE_OLEGUID") =>
            {
                let ole = child.name() == "DEFINE_OLEGUID";
                let tokens = self.tu.tokenize(child.extent());
                if let Some((name, uuid)) = parse_define_guid_tokens(&tokens, ole)
                    && !name.is_empty()
                {
                    // An `IID_<Interface>` GUID associates a UUID with an interface
                    // whose C++ declaration carries no `__declspec(uuid(...))`. Reading
                    // it here (from the macro arguments) keeps the association
                    // independent of `INITGUID`/definition mode — the same UUID the
                    // initialized-variable path below would see, but always available.
                    if let Some(iface_name) = name.strip_prefix("IID_") {
                        self.iid_vars
                            .entry(iface_name.to_string())
                            .or_insert_with(|| uuid.clone());
                    }
                    if !self.ref_map.contains_key(&name) {
                        collector.insert(Item::GuidConst(GuidConst { name, uuid }));
                    }
                }
            }
            // Detect `DEFINE_PROPERTYKEY(name, ...)` / `DEFINE_DEVPROPKEY(name, ...)` macro
            // invocations. Both expand to a `{ { fmtid }, pid }` initializer for a
            // `PROPERTYKEY`/`DEVPROPKEY` (a GUID plus a `u32`), so the faithful value lives in
            // the macro arguments parsed here. The `fmtid` is emitted as a `#[guid]` attribute
            // and the `pid` as an ordinary integer constant.
            CXCursor_MacroExpansion
                if matches!(
                    child.name().as_str(),
                    "DEFINE_PROPERTYKEY" | "DEFINE_DEVPROPKEY"
                ) =>
            {
                let ty = if child.name() == "DEFINE_DEVPROPKEY" {
                    "DEVPROPKEY"
                } else {
                    "PROPERTYKEY"
                };
                let tokens = self.tu.tokenize(child.extent());
                if let Some((name, uuid, pid)) = parse_define_property_key_tokens(&tokens)
                    && !name.is_empty()
                    && !self.ref_map.contains_key(&name)
                    && !collector.contains_key(&name)
                {
                    collector.insert(Item::PropertyKeyConst(PropertyKeyConst {
                        name,
                        ty: ty.to_string(),
                        uuid,
                        pid,
                    }));
                }
            }
            // Detect `extern "C" const GUID IID_XXX = { ... };` variable declarations.
            // These associate a GUID with an interface whose C++ declaration does not
            // carry `__declspec(uuid("..."))` (e.g. the 7zip SDK pattern).
            CXCursor_VarDecl => {
                let name = child.name();
                if let Some(iface_name) = name.strip_prefix("IID_")
                    && is_guid_type(&child.ty())
                {
                    if let Some(uuid) = parse_guid_initializer_ast(&child) {
                        self.iid_vars.insert(iface_name.to_string(), uuid);
                    } else {
                        // Fallback to token-based parsing for simple cases
                        // where the AST shape doesn't match (e.g. no init-list children).
                        let tokens = self.tu.tokenize(self.tu.to_expansion_range(child.extent()));
                        if let Some(uuid) = parse_guid_initializer_tokens(&tokens) {
                            self.iid_vars.insert(iface_name.to_string(), uuid);
                        }
                    }
                } else if let Some(c) = Const::parse_var_decl(&child)
                    && !self.ref_map.contains_key(&c.name)
                    && !collector.contains_key(&c.name)
                {
                    collector.insert(Item::Const(c));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Iterate the direct children of `parent` and call [`process_cursor`] for
    /// every `CXCursor_StructDecl` or `CXCursor_UnionDecl` definition found
    /// there, whether named or anonymous.
    ///
    /// This lifts nested struct/union type declarations — i.e. structs or
    /// unions declared *inside* another struct or union body — into the
    /// top-level collector before the outer type is processed.  Without this
    /// step the outer struct's field types would reference names that have
    /// never been added to the collector, producing dangling type references.
    ///
    /// The recursion naturally handles arbitrary nesting depth: processing
    /// a nested struct will in turn call this function for *its* children,
    /// so `struct A { struct B { struct C { ... } c; } b; };` is handled
    /// correctly by emitting `C`, then `B`, then `A` into the collector.
    fn process_nested_types(
        &mut self,
        parent: Cursor,
        collector: &mut Collector,
        extern_c: bool,
    ) -> Result<(), Error> {
        for nested in parent.children() {
            if (nested.kind() == CXCursor_StructDecl || nested.kind() == CXCursor_UnionDecl)
                && nested.is_definition()
            {
                self.process_cursor(nested, collector, extern_c)?;
            } else if nested.kind() == CXCursor_EnumDecl && nested.is_definition() {
                // A nested anonymous enum (e.g. `enum { ElementType, TextType } Type;`
                // inside `_WSDXML_NODE`) leaks its enumerators into the enclosing scope.
                // The field itself takes the enum's underlying integer type (see
                // `to_type`); its enumerators are emitted as top-level constants, exactly
                // as a file-scope anonymous enum is. A named nested enum keeps its own
                // identity and is emitted when referenced, so only the nameless (or
                // MIDL-synthesised) case is lifted here.
                let e = Enum::parse(nested)?;
                if is_anonymous_name(&e.name) || is_midl_anonymous_enum_name(&e.name) {
                    for (name, value) in e.variants {
                        let const_value = enum_variant_value(e.repr, value);
                        collector.insert(Item::Const(Const {
                            name,
                            value: const_value,
                        }));
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
/// Builder that generates RDL from C/C++ headers using libclang.
pub struct Clang {
    input: Vec<String>,
    input_str: Vec<String>,
    output: String,
    namespace: String,
    args: Vec<String>,
    library: String,
    /// Per-symbol DLL overrides recovered from the SDK import libraries: maps an
    /// exported function name to the DLL that exports it. A function found here
    /// is stamped with that DLL; any other function falls back to [`library`].
    libraries: HashMap<String, String>,
    filter: Vec<String>,
    target: Option<String>,
    /// In-scope header directory segments (e.g. `["um", "shared"]`) for
    /// reachability-by-reference capture in [`write_by_header`](Self::write_by_header).
    /// When non-empty, a declaration whose defining header lives *outside* these
    /// directories is emitted only if it is transitively referenced by an in-scope
    /// declaration; unreferenced out-of-scope declarations (C-runtime / compiler-toolset
    /// noise) are swept. Empty (the default) emits the whole closure unchanged.
    scope: Vec<String>,
    /// Explicitly requested headers (by stem) that are in-scope regardless of their SDK
    /// directory. The directory-based [`scope`](Self::scope) covers the bulk `um`/`shared`
    /// closure; a header named here is treated as in-scope even when it lives outside those
    /// directories — e.g. the WinRT C-ABI interop headers under `winrt/` (`roerrorapi.h`,
    /// `roapi.h`) — so its own declarations are emitted as roots rather than dropped by the
    /// sweep. The projection headers such a header transitively pulls stay out of scope
    /// unless named here too.
    scope_headers: HashSet<String>,
    /// Defining-header stems (as in [`write_by_header`](Self::write_by_header)) whose partitions
    /// are dropped entirely, even though they are in [`scope`](Self::scope). Use this for an
    /// in-scope SDK header that contributes no genuine API surface — e.g. `intsafe.h`, a bundle of
    /// inline safe-integer-math helpers whose only scraped output is standard C type-limit macros
    /// (`INT32_MAX`, `UINT8_MAX`, …) and internal `*_ERROR` sentinels. The drop happens before the
    /// reachability sweep, so an excluded header never acts as a root; it is safe only for headers
    /// nothing in-scope references (leaf constants). Empty (the default) drops nothing.
    exclude_headers: HashSet<String>,
    /// Symbol allowlist for a targeted extraction. When non-empty, [`write`](Self::write)
    /// emits only the named functions (and their transitive type/const closure,
    /// deduplicated against the reference winmd) and suppresses every other root. Empty
    /// (the default) leaves emission unrestricted. See [`Parser::symbols`].
    symbols: HashSet<String>,
    /// When `true`, functions that resolve to an empty import library are dropped rather than
    /// emitted with `#[library("")]`. See [`Parser::drop_lib_less`]. `tool_win32` sets this so
    /// the corpus carries no lib-less function (matching the reference metadata); off by default
    /// so unit-test fixtures that supply no import libraries keep emitting their functions.
    drop_lib_less: bool,
    /// Resolution `.winmd` inputs (e.g. `Windows.winmd`) consulted *only* to classify types
    /// declared in the `ABI::Windows::*` C++/WinRT projection namespace during a per-header
    /// scrape. Unlike [`input`](Self::input) winmds — which are an *exclusion* base whose every
    /// entity is skipped from emission — these are never excluded and never emitted; their type
    /// names merely tell the scraper which `ABI` declarations are true WinRT projections (mapped
    /// to a cross-winmd reference) versus Win32 COM interop entities (scraped into flat Win32).
    /// Empty (the default) leaves the `ABI` namespace skipped entirely.
    resolution_input: Vec<String>,
}

/// Read-only inputs shared by every per-header pass of a by-header scrape.
#[derive(Clone, Copy)]
struct HeaderPass<'a> {
    /// Flat namespace root every partition emits into (`Windows.Win32`).
    root: &'a str,
    /// Defining-header stems whose partitions are written (empty writes all).
    allow: &'a HashSet<&'a str>,
    /// Resolution-winmd type-name membership classifying `ABI::Windows::*` decls;
    /// empty when no resolution winmd is supplied (the `ABI` namespace is skipped).
    winrt_types: &'a HashSet<String>,
}

impl Clang {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an input header (`.h`) or `.winmd` file or directory.
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    /// Adds multiple input headers or `.winmd` files.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input.push(input.as_ref().to_string());
        }
        self
    }

    /// Adds inline source text to compile instead of a file on disk.
    pub fn input_str(&mut self, input: &str) -> &mut Self {
        self.input_str.push(input.to_string());
        self
    }

    /// Sets the output `.rdl` file path.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    /// Sets the namespace for the generated types.
    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = namespace.to_string();
        self
    }

    /// Sets the library name recorded for imported functions.
    pub fn library(&mut self, library: &str) -> &mut Self {
        self.library = library.to_string();
        self
    }

    /// Drops functions that resolve to an empty import library instead of emitting them with
    /// `#[library("")]`. Use when scraping against a full SDK import-library set (`tool_win32`),
    /// so the corpus carries no lib-less residue — inline intrinsics, header-only presence-checks,
    /// RPC internals, and DLL exports the SDK ships no import library for, none of which can lower
    /// to a working `link!`, matching the reference metadata. Leave off (the default) when no
    /// import libraries are supplied, so every declared function is still emitted.
    pub fn drop_lib_less(&mut self, drop_lib_less: bool) -> &mut Self {
        self.drop_lib_less = drop_lib_less;
        self
    }

    /// Adds a resolution `.winmd` input (e.g. `Windows.winmd`) used only to classify types in
    /// the `ABI::Windows::*` C++/WinRT projection namespace during a per-header scrape. See
    /// [`resolution_input`](Self::resolution_input). Unlike [`input`](Self::input), the winmd is
    /// never used as an exclusion base and none of its entities are emitted; supplying one opts
    /// the scrape into reaching the Win32 COM interop interfaces the SDK declares inside the ABI
    /// namespace (e.g. `roregistrationapi.h`'s `IActivatableClassRegistration`).
    pub fn resolution_input(&mut self, input: &str) -> &mut Self {
        self.resolution_input.push(input.to_string());
        self
    }

    /// Adds a symbol → DLL mapping that overrides the fallback [`library`] for
    /// matching functions.
    ///
    /// This is the faithful function → DLL truth headers do not carry; build the
    /// map from the SDK import libraries with [`import_library`] (or feed one
    /// directly), preferring the per-DLL libraries (`kernel32.lib`) over the
    /// umbrella/apiset libraries so symbols resolve to their real DLL rather
    /// than an `api-ms-win-*` apiset name.
    ///
    /// [`library`]: Self::library
    /// [`import_library`]: Self::import_library
    pub fn libraries<I, K, V>(&mut self, libraries: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.libraries
            .extend(libraries.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Reads a COFF import library (`.lib`) and adds every symbol → DLL mapping
    /// it declares, so functions are stamped with the DLL that actually exports
    /// them. Existing mappings win, so earlier (more specific) libraries take
    /// precedence over later ones.
    pub fn import_library(&mut self, path: &str) -> Result<&mut Self, Error> {
        extend_libraries(&mut self.libraries, path)?;
        Ok(self)
    }

    /// Adds a header path suffix to the inclusion filter.
    ///
    /// When one or more filters are set, only declarations from headers whose
    /// path ends with a registered suffix are emitted into the output RDL
    /// (in addition to declarations from the main input file, which are always
    /// included).  This is useful when [`input_str`][Self::input_str] is used
    /// with `#include` directives that pull in both dependency headers (whose
    /// types should not appear in the output) and API headers (whose types
    /// should).
    ///
    /// Matching is done by path suffix after normalizing directory separators
    /// to `/`, so `.filter("api1.h")` matches any file whose path ends with
    /// `api1.h` and `.filter("vendor/foo/helpers.h")` can be used to
    /// disambiguate when multiple files share the same base name.
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filter.push(filter.to_string());
        self
    }

    /// Adds multiple header path suffixes to the inclusion filter.
    pub fn filters<I, S>(&mut self, filters: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for filter in filters {
            self.filter.push(filter.as_ref().to_string());
        }
        self
    }

    /// Add a single compiler argument to pass to libclang.
    pub fn arg<S: AsRef<str>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Adds multiple compiler arguments to pass to libclang.
    pub fn args<I>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }

    /// Sets the target triple used for all clang invocations, e.g.
    /// `"x86_64-pc-windows-msvc"`, `"i686-pc-windows-msvc"`, or
    /// `"aarch64-pc-windows-msvc"`.
    ///
    /// This is equivalent to passing `--target=<triple>` as the first argument
    /// via [`arg`][Self::arg], but is cleaner for per-arch builds.
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }

    /// Sets the in-scope header directory segments for reachability-by-reference
    /// capture in [`write_by_header`](Self::write_by_header), e.g. `["um", "shared"]`
    /// to treat only the SDK API headers as in-scope. A declaration whose defining
    /// header path contains one of these segments (as a `/<seg>/` directory) is
    /// emitted unconditionally; any other declaration (C-runtime under `ucrt`, the
    /// MSVC toolset `include`, …) is emitted only when transitively referenced by an
    /// in-scope declaration, and otherwise dropped. Leaving this empty emits the
    /// entire parse closure unchanged.
    pub fn scope<I, S>(&mut self, scope: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for seg in scope {
            self.scope.push(seg.as_ref().to_string());
        }
        self
    }

    /// Marks specific headers (by file name; the stem is taken as in
    /// [`write_by_header`](Self::write_by_header)) as in-scope regardless of their SDK
    /// directory, complementing the directory-based [`scope`](Self::scope). Use this to
    /// pull a hand-picked header out of an otherwise out-of-scope directory — e.g. the
    /// WinRT interop ABI headers under `winrt/` (`roerrorapi.h`, `roapi.h`) — without
    /// bringing the whole directory (the winmd-generated projection headers) into scope.
    pub fn scope_headers<I, S>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for header in headers {
            let stem = header_stem_to_namespace(header.as_ref());
            if !stem.is_empty() {
                self.scope_headers.insert(stem);
            }
        }
        self
    }

    /// Drops the partitions of the named headers (by file name; the stem is taken as in
    /// [`write_by_header`](Self::write_by_header)) even though they are in [`scope`](Self::scope).
    /// Use this to suppress an in-scope SDK header that carries no genuine API surface — e.g.
    /// `intsafe.h`, whose scraped output is nothing but standard C type-limit macros and internal
    /// `*_ERROR` sentinels. Safe only for headers nothing in-scope references (leaf constants).
    pub fn exclude_headers<I, S>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for header in headers {
            let stem = header_stem_to_namespace(header.as_ref());
            if !stem.is_empty() {
                self.exclude_headers.insert(stem);
            }
        }
        self
    }

    /// Restricts emission to an allowlist of function symbols. When one or more
    /// symbols are set, [`write`](Self::write) emits only those functions as roots —
    /// their transitive type/const closure still resolves (either to the reference
    /// winmd or emitted as follow-up typedefs) — and suppresses every other root
    /// (types, consts, macros, GUIDs). This carves a handful of symbols out of an
    /// enormous header (e.g. `RtlGetVersion` from the WDK's `wdm.h`) without dragging
    /// in the header's whole surface. Empty (the default) leaves emission unrestricted.
    pub fn symbols<I, S>(&mut self, symbols: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for symbol in symbols {
            self.symbols.insert(symbol.as_ref().to_string());
        }
        self
    }

    /// Returns the version string reported by the loaded libclang, e.g.
    /// `"clang version 18.1.0 (...)"`.  Loads libclang on first call.
    pub fn version() -> Result<String, Error> {
        let lib = Library::new()?;
        Ok(lib.version())
    }

    /// Generates the RDL and writes it to the configured output.
    pub fn write(&self) -> Result<(), Error> {
        let reference = self.load_reference()?;
        let spec = NamespaceSpec {
            namespace: &self.namespace,
            library: &self.library,
            libraries: &self.libraries,
            filter: &self.filter,
            symbols: &self.symbols,
        };
        let rdl = self.parse_and_emit(&reference, std::slice::from_ref(&spec))?;
        write_to_file(&self.output, formatter::format(&rdl[0]))?;
        Ok(())
    }

    /// Generates RDL for the flat root namespace (`Windows.Win32`), writing one
    /// `<header-stem>.rdl` file per defining header into `output_dir`.
    ///
    /// `partitions` is an allowlist of header stems (e.g. `["Wingdi", "Tlhelp32"]`); only
    /// those files are *written*, while cross-header references still resolve globally by
    /// bare name within the flat namespace. An empty allowlist emits every defining header
    /// in the parse — faithful but only viable for small, self-contained inputs, since the
    /// full `windows.h` closure spans the whole SDK.
    ///
    /// This is the faithful, source-expressed metadata that replaces editorial namespaces:
    /// the headers are parsed once, every entity is canonically deduplicated by clang USR
    /// (so a declaration repeated across headers is emitted exactly once), and the defining
    /// header only chooses which file the entity is written to. No reference winmd, claims,
    /// per-header namespaces, or two-pass ownership table is involved — ownership and
    /// resolution both come from the clang cursor.
    ///
    /// A `.winmd` input (via [`input`](Self::input)) is treated as an *exclusion reference*
    /// for additive scrapes: any entity it already defines is skipped rather than re-emitted.
    /// `tool_win32` passes no winmd and emits its full closure; `tool_wdk` passes the Win32
    /// winmd so only the WDK-net-new surface is written into the shared flat namespace.
    pub fn write_by_header(
        &self,
        root: &str,
        partitions: &[&str],
        output_dir: &str,
    ) -> Result<(), Error> {
        let allow: HashSet<&str> = partitions.iter().copied().collect();
        let outputs = self.parse_and_emit_by_header(root, &allow)?;
        for (stem, rdl) in outputs {
            // The flat module is the single root namespace (e.g. `Windows.Win32`); the
            // file on disk is the lowercased defining-header stem (`windef.rdl`).
            let leaf = stem.to_lowercase();
            write_to_file(&format!("{output_dir}/{leaf}.rdl"), formatter::format(&rdl))?;
        }
        Ok(())
    }

    /// Loads libclang and parses every configured header and in-memory source once
    /// into translation units, shared by both emit paths. The returned struct owns the
    /// libclang `Library` guard and `Index`, keeping the translation units valid for its
    /// whole lifetime; its field order is the drop order, so the units and index are
    /// disposed before the `Library` guard unloads libclang.
    fn parse_inputs(&self) -> Result<ParsedInputs, Error> {
        let (h_paths, _) = expand_input_paths(&self.input, "h", "winmd")?;
        let library = Library::new()?;
        let index = Index::new()?;

        // The effective args list: optional `--target=` first, then user args.
        let args: Vec<String> = self
            .target
            .as_ref()
            .map(|t| format!("--target={t}"))
            .into_iter()
            .chain(self.args.iter().cloned())
            .collect();
        let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

        let mut h_tus = vec![];
        for input in &h_paths {
            h_tus.push((input.clone(), index.parse(input, &arg_refs)?));
        }
        let mut str_tus = vec![];
        for content in &self.input_str {
            str_tus.push((
                content.clone(),
                index.parse_unsaved(
                    ".h",
                    content,
                    &arg_refs,
                    CXTranslationUnit_DetailedPreprocessingRecord,
                )?,
            ));
        }

        Ok(ParsedInputs {
            args,
            h_tus,
            str_tus,
            index,
            _library: library,
        })
    }

    /// Parses the configured headers once and emits one RDL string per emitted
    /// defining-header file, keyed by the header stem (e.g. `Windef`). Every string
    /// is the flat root namespace (`Windows.Win32`); the stem only names the file.
    fn parse_and_emit_by_header(
        &self,
        root: &str,
        allow: &HashSet<&str>,
    ) -> Result<BTreeMap<String, String>, Error> {
        // Optional reference winmd: an *additive* scrape (e.g. `tool_wdk` layering the WDK
        // surface onto Win32) supplies the base winmd here so every entity it already
        // defines is skipped rather than re-emitted, which would collide with the base at
        // read time (the reader panics on a duplicate `(namespace, name)`). The two winmds
        // share the flat root namespace, so a skipped entity's references resolve against
        // the base by bare name once both are loaded together. The base is split into type
        // and value names so the exclusion below can be category-matched (functions and
        // constants live on the base's `Apis` class, so `iter_items` — not `iter` — is
        // required to see them). No winmd input (the `tool_win32` case) leaves both empty
        // and emission is unrestricted.
        let reference = self.load_reference()?;
        let mut exclude_types: HashSet<String> = HashSet::new();
        let mut exclude_values: HashSet<String> = HashSet::new();
        for (_, name, item) in reference.iter_items() {
            match item {
                metadata::reader::Item::Type(_) => exclude_types.insert(name.to_string()),
                metadata::reader::Item::Fn(_) | metadata::reader::Item::Const(_) => {
                    exclude_values.insert(name.to_string())
                }
            };
        }

        let parsed = self.parse_inputs()?;
        let arg_refs: Vec<&str> = parsed.args.iter().map(String::as_str).collect();

        // Type-name membership of the resolution winmd (`Windows.winmd`), used to classify
        // declarations found in the `ABI::Windows::*` C++/WinRT projection namespace. Stored as
        // backtick-arity-stripped `Namespace.Name` so a generic instantiation reached in C++
        // (`ABI::Windows::Foundation::Collections::IMapView<...>`) matches the winmd's `IMapView`2`
        // by its bare qualified name. Empty (no resolution winmd) leaves the ABI namespace skipped.
        let winrt_types = self.load_winrt_types()?;

        let mut collectors: BTreeMap<String, Collector> = BTreeMap::new();
        // Per-partition scope flag (defining-header stem → in-scope), accumulated while
        // bucketing so the reachability sweep below knows which partitions are roots.
        let mut scope_in: BTreeMap<String, bool> = BTreeMap::new();

        let pass = HeaderPass {
            root,
            allow,
            winrt_types: &winrt_types,
        };

        for (input, tu) in &parsed.h_tus {
            self.process_tu_by_header(
                tu,
                &pass,
                &mut collectors,
                &mut scope_in,
                MacroEval {
                    source: MacroSource::File(input),
                    args: &arg_refs,
                },
            )?;
        }
        for (content, tu) in &parsed.str_tus {
            self.process_tu_by_header(
                tu,
                &pass,
                &mut collectors,
                &mut scope_in,
                MacroEval {
                    source: MacroSource::Str(content),
                    args: &arg_refs,
                },
            )?;
        }

        // Drop excluded in-scope partitions (headers named via `exclude_headers`, e.g. `intsafe.h`)
        // before the sweep, so an excluded header is never a reachability root. Safe because these
        // headers carry only leaf constants that no in-scope declaration references.
        if !self.exclude_headers.is_empty() {
            collectors.retain(|stem, _| !self.exclude_headers.contains(stem));
            scope_in.retain(|stem, _| !self.exclude_headers.contains(stem));
        }

        // Reachability-by-reference sweep: when a scope is declared, drop every
        // out-of-scope declaration that no in-scope declaration transitively references
        // (the C-runtime / compiler-toolset closure that `windows.h` pulls in). The
        // genuine cross-over types (`size_t`, `va_list`, `EXCEPTION_DISPOSITION`, …)
        // survive automatically because in-scope APIs reference them — no allowlist.
        if !self.scope.is_empty() {
            sweep_unreferenced(&mut collectors, &scope_in);
        }

        // Drop every entity the reference winmd already defines, matched by category: a WDK
        // type is dropped only when the base defines a *type* of that name, and a WDK
        // function/constant only when the base defines a *value* of that name. This excludes
        // the true duplicates (`RtlCaptureContext`, shared consts) whose references then
        // resolve against the base by bare name, while a rare cross-category name clash (a
        // WDK type sharing a bare name with a Win32 constant, e.g. `CALLBACK_FUNCTION`) keeps
        // the WDK type so its own pointer typedef still resolves. Empty (no reference winmd)
        // is a no-op.
        if !exclude_types.is_empty() || !exclude_values.is_empty() {
            for collector in collectors.values_mut() {
                collector.retain_items(|name, item| {
                    if item.is_type() {
                        !exclude_types.contains(name)
                    } else {
                        !exclude_values.contains(name)
                    }
                });
            }
        }

        // A WDK *type* whose bare name collides with a Win32 *value* (the WDK `enum MODE`
        // vs the Win32 `const MODE`) cannot coexist in the flat root namespace once both
        // winmds are read together: bindgen projects the enum's members through the newtype
        // constructor `MODE(n)`, but value-position `MODE` resolves to the Win32 constant, so
        // the whole-surface build fails to compile. Drop the colliding WDK type in favour of
        // the already-defined Win32 name — but only when no *retained* WDK item still
        // references it, so a referenced cross-kind clash (the WDK `CALLBACK_FUNCTION` callback
        // reached via `type PCALLBACK_FUNCTION = *mut CALLBACK_FUNCTION`, colliding with the
        // Win32 `CALLBACK_FUNCTION` constant) is preserved and does not dangle. The
        // category-matched pass above already removed the same-kind duplicates.
        if !exclude_values.is_empty() {
            let mut referenced: HashSet<String> = HashSet::new();
            for collector in collectors.values() {
                for item in collector.values() {
                    item_refs(item, &mut referenced);
                }
            }
            for collector in collectors.values_mut() {
                collector.retain_items(|name, item| {
                    !(item.is_type() && exclude_values.contains(name) && !referenced.contains(name))
                });
            }
        }

        // Drop redundant `IID_<Interface>` GUID constants whose interface is defined in
        // this scrape: the interface already carries the GUID (attached via `apply_iid_vars`
        // from the same `DEFINE_GUID`/`IID_` declaration), so the free constant is a pure
        // duplicate. The reference metadata emits no such standalone `IID_<Interface>` const,
        // and keeping it breaks the `--sys` projection, where bindgen synthesises the
        // interface's own `IID_<Interface>` constant from its GUID and the two collide
        // (`the name is defined multiple times`). An `IID_<Name>` whose `<Name>` is *not* a
        // defined interface (a reference-only or non-COM GUID) is preserved.
        let interfaces: HashSet<String> = collectors
            .values()
            .flat_map(|collector| collector.iter())
            .filter(|(_, item)| matches!(item, Item::Interface(_)))
            .map(|(name, _)| name.clone())
            .collect();
        if !interfaces.is_empty() {
            for collector in collectors.values_mut() {
                collector.retain_items(|name, item| {
                    !(matches!(item, Item::GuidConst(_))
                        && name
                            .strip_prefix("IID_")
                            .is_some_and(|iface| interfaces.contains(iface)))
                });
            }
        }

        // Drop a top-level integer constant that merely duplicates an enum member of the
        // same name and value. Legacy Win32 headers expose such values twice: once as a
        // typed enumerator (`D3DFORMAT::D3DFMT_X8R8G8B8` in d3d9types.h,
        // `OLEMISC::OLEMISC_ACTSLIKELABEL` in oleidl.h) and once as a loose object-like
        // macro in an unrelated header (mfapi.h, olectl.h) — the former often a transient
        // `#define`/`#undef` scaffold that libclang still reports as a macro definition.
        // The bare `u32` copy is a redundant, weaker-typed duplicate of the canonical
        // enumerator, so drop it. Constrained to an exact name + value match and to
        // constants that nothing else references, so a genuinely distinct constant (a name
        // shared with an unrelated enumerator of a different value) is never removed.
        let enum_members = enum_member_values(&collectors);
        if !enum_members.is_empty() {
            let mut referenced = HashSet::new();
            for collector in collectors.values() {
                for item in collector.values() {
                    item_refs(item, &mut referenced);
                }
            }
            for collector in collectors.values_mut() {
                collector.retain_items(|name, item| {
                    let Item::Const(c) = item else {
                        return true;
                    };
                    if referenced.contains(name) {
                        return true;
                    }
                    let (Some(values), Some(value)) =
                        (enum_members.get(name), const_integer_bits(&c.value))
                    else {
                        return true;
                    };
                    !values.iter().any(|&member| enum_member_eq(member, value))
                });
            }
        }

        let mut outputs = BTreeMap::new();
        for (stem, collector) in &collectors {
            // A partition fully emptied by the sweep is not written (its stale file, if
            // any, was already removed before regeneration).
            if collector.is_empty() {
                continue;
            }
            // Every file emits the single flat root namespace; the stem only names the
            // file on disk. Dedup guarantees no entity name appears in two files.
            outputs.insert(stem.clone(), emit_module(root, collector)?);
        }
        Ok(outputs)
    }

    /// Walk a parsed translation unit once, routing every top-level declaration to
    /// the collector of its defining-header partition. Unlike [`process_tu`], there
    /// is no header filter (every declaration is emitted, in its own partition) and
    /// no `ref_map` suppression — resolution is by defining header, threaded through
    /// [`Parser::header_root`].
    fn process_tu_by_header(
        &self,
        tu: &TranslationUnit,
        pass: &HeaderPass<'_>,
        collectors: &mut BTreeMap<String, Collector>,
        scope_in: &mut BTreeMap<String, bool>,
        eval: MacroEval<'_>,
    ) -> Result<(), Error> {
        let HeaderPass {
            root,
            allow,
            winrt_types,
        } = *pass;
        // Error-tolerant diagnostics. An error in an *emitted* header — one that is in
        // scope, so its declarations are scraped into the corpus — is a real defect and
        // still aborts the whole scrape, preserving the loud-failure guarantee. An error
        // in a transitive-only include (a header pulled in solely to name a few types but
        // never itself emitted, e.g. a `winrt\` C++/WinRT projection header an interop
        // header references) is tolerated: clang's best-effort AST still yields the
        // in-scope declarations, so the scrape continues past it. When no directory scope
        // is configured every header is emitted, so any error aborts as before; an error
        // with no attributable file (a driver/command-line error) also aborts, since it
        // cannot be proven to originate in a tolerable transitive-only header.
        for diag in tu.diagnostics() {
            if !diag.is_err() {
                continue;
            }
            let emitted = self.scope.is_empty()
                || diag.file_name.is_empty()
                || self
                    .scope_headers
                    .contains(&header_stem_to_namespace(&diag.file_name))
                || header_in_scope(&diag.file_name, &self.scope);
            if emitted {
                return Err(Error::new(
                    &diag.message,
                    &diag.file_name,
                    diag.line.try_into().unwrap(),
                    (diag.column.saturating_sub(1)).try_into().unwrap(),
                ));
            }
        }

        let mut tag_rename = build_tag_rename_map(tu);
        assign_nested_names(tu, &mut tag_rename);
        let enum_merge = merge_enum_typedef_idiom(tu, &mut tag_rename);
        // `macro_defs` depends only on the TU, so compute it once and share it across every
        // per-header bucket below (each `Parser` borrows it) instead of re-scanning the whole
        // TU per bucket.
        let macro_defs = collect_macro_defs(tu);

        // Flat-namespace, canonically deduplicated routing. The SDK repeats the same
        // declaration across many headers (include-guard idioms, MIDL proxy/stub pairs,
        // shared typedefs); clang's USR gives each entity one stable identity across all
        // those redeclarations, so the closure emits every entity exactly once. The whole
        // closure lives in the single flat `Windows.Win32` namespace (`root`); the
        // defining header only selects which `<stem>.rdl` file the entity is written to.
        //
        // `extern "C"` linkage blocks are flattened: a single `extern "C" { … }` opened in
        // one header can wrap declarations pulled in (via `#include`) from many others, so
        // each enclosed declaration is routed by *its own* defining header. Handle-tag
        // structs (`<name>__`, the DECLARE_HANDLE idiom) are dropped: they carry no payload
        // and the handle typedef itself emits an opaque `*mut void`.
        let mut decls = Vec::new();
        // A resolution winmd opts the scrape into reaching the Win32 COM interop entities the
        // SDK declares inside the `ABI::Windows::*` C++/WinRT projection namespace (classified by
        // `winrt_types`); with none supplied the ABI namespace is skipped entirely as before.
        let abi = (!winrt_types.is_empty()).then_some(winrt_types);
        flatten_decls(tu.cursor(), false, false, None, abi, &mut decls);

        // Deduplicate by canonical identity. For each entity keep the most informative
        // cursor — a definition outranks a forward declaration — so a record routes to the
        // header that *defines* it rather than one that merely forward-declares it. The key
        // is the USR; entities clang gives no USR (e.g. anonymous types) fall back to the
        // canonical cursor's unique source location.
        let mut chosen: BTreeMap<String, (Cursor, bool)> = BTreeMap::new();
        for (child, extern_c) in decls {
            if is_handle_tag_struct(&child) {
                continue;
            }
            if header_stem_of(&child).is_none() {
                continue;
            }
            let usr = child.usr();
            let key = if usr.is_empty() {
                child.canonical().location_id()
            } else {
                usr
            };
            match chosen.entry(key) {
                std::collections::btree_map::Entry::Vacant(e) => {
                    e.insert((child, extern_c));
                }
                std::collections::btree_map::Entry::Occupied(mut e) => {
                    let existing = &e.get().0;
                    // A definition outranks any forward declaration. When neither is a
                    // definition, prefer a forward declaration that carries a `uuid`
                    // attribute (a COM coclass CLSID, e.g. `class DECLSPEC_UUID(...) X;`)
                    // over a bare one (such as the `typedef class X X;` MIDL forward
                    // decl), so the CLSID is not lost.
                    let replace = if child.is_definition() {
                        !existing.is_definition()
                    } else if !existing.is_definition() {
                        child.extract_uuid(tu).is_some() && existing.extract_uuid(tu).is_none()
                    } else {
                        false
                    };
                    if replace {
                        e.insert((child, extern_c));
                    }
                }
            }
        }

        // Bucket each chosen entity by its defining-header file stem. The flat namespace
        // means cross-header references resolve by bare name, so the allowlist only limits
        // which files are *written*, never which references resolve.
        let mut buckets: BTreeMap<String, Vec<(Cursor, bool)>> = BTreeMap::new();
        for (_, (child, extern_c)) in chosen {
            let stem = header_stem_of(&child).expect("filtered above");
            if !allow.is_empty() && !allow.contains(stem.as_str()) {
                continue;
            }
            // Classify this partition's scope from the defining-header path. A stem maps
            // to a single header, so any cursor settles it; `or` keeps it in-scope if any
            // contributing cursor is in-scope (defensive against path quirks).
            if !self.scope.is_empty() {
                let in_scope = self.scope_headers.contains(&stem)
                    || header_path_of(&child).is_none_or(|p| header_in_scope(&p, &self.scope));
                scope_in
                    .entry(stem.clone())
                    .and_modify(|v| *v |= in_scope)
                    .or_insert(in_scope);
            }
            buckets.entry(stem).or_default().push((child, extern_c));
        }

        let empty_ref: HashMap<String, String> = HashMap::new();
        let empty_symbols: HashSet<String> = HashSet::new();
        let mut all_opaque: Vec<(String, String)> = vec![];
        // Macro-defined constants are evaluated per bucket but deduplicated globally:
        // a `#define` visible in several headers is one translation-unit-wide value, so it
        // must be emitted once. Stashed with its file stem until every entity name is known.
        let mut all_consts: Vec<(String, Vec<String>)> = vec![];

        for (stem, cursors) in buckets {
            let collector = collectors.entry(stem.clone()).or_default();
            let mut parser = Parser::new(
                root,
                &self.library,
                &self.libraries,
                &empty_ref,
                &tag_rename,
                &enum_merge,
                &macro_defs,
                tu,
                &empty_symbols,
            );
            parser.header_root = Some(root);
            parser.drop_lib_less = self.drop_lib_less;
            parser.winrt_types = abi;

            for (child, extern_c) in cursors {
                parser.process_cursor(child, collector, extern_c)?;
            }

            collector.apply_iid_vars(&parser.iid_vars);

            let pending = std::mem::take(&mut parser.pending_macros);
            if !pending.is_empty() {
                all_consts.push((stem.clone(), pending));
            }
            for (_ns, name) in std::mem::take(&mut parser.pending_opaque) {
                all_opaque.push((stem.clone(), name));
            }
        }

        // The set of every entity name now emitted across all files — the canonical owners
        // that follow-up consts and opaque placeholders must defer to. A flat-layout enum
        // contributes both its type name and every member name, since its members are
        // emitted as top-level constants: a macro sharing a member's name would collide
        // (the WDK ARM64 `#define NonPagedPool NonPagedPoolNx` shadowing the
        // `POOL_TYPE::NonPagedPool` enumerator).
        let mut global_names: HashSet<String> = collectors
            .values()
            .flat_map(|c| c.values())
            .flat_map(|item| {
                let mut names = vec![item.to_string()];
                if let Item::Enum(e) = item {
                    names.extend(e.variants.iter().map(|(name, _)| name.clone()));
                }
                names
            })
            .collect();

        // Emit macro constants, skipping any name already owned by a real entity or an
        // earlier file (BTreeMap iteration keeps file order stable, so the winner is
        // deterministic). Each per-stem evaluation re-parses the synthetic translation
        // unit and is independent of the others, so they run concurrently; the results
        // are merged back in the original `all_consts` order to keep the first-owner-wins
        // resolution byte-for-byte identical to a sequential pass.
        let evaluated = evaluate_macros_parallel(&all_consts, eval.source, eval.args)?;
        for ((stem, _pending), consts) in all_consts.into_iter().zip(evaluated) {
            let collector = collectors.entry(stem).or_default();
            for c in consts {
                if global_names.insert(c.name.clone()) {
                    collector.insert(Item::Const(c));
                }
            }
        }

        // Emit opaque empty structs for incomplete record types referenced through a
        // pointer but never defined anywhere in the translation unit. A real definition (in
        // any file) always wins over the placeholder.
        for (stem, name) in all_opaque {
            if global_names.insert(name.clone()) {
                let collector = collectors.entry(stem).or_default();
                collector.insert(Item::Struct(Struct::opaque(&name)));
            }
        }

        Ok(())
    }

    /// Loads the `.winmd` reference inputs into an index used to resolve
    /// cross-namespace type names.
    fn load_reference(&self) -> Result<metadata::reader::Index, Error> {
        let (_, winmd_paths) = expand_input_paths(&self.input, "h", "winmd")?;

        let mut winmd_files = vec![];
        for file_name in &winmd_paths {
            winmd_files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }

        Ok(metadata::reader::Index::new(winmd_files))
    }

    /// Loads the [`resolution_input`](Self::resolution_input) winmds and returns the set of every
    /// type's `Namespace.Name`, backtick-arity-stripped (`IMapView`2` → `IMapView`). This membership
    /// set classifies declarations in the `ABI::Windows::*` C++/WinRT projection namespace during a
    /// per-header scrape; empty when no resolution winmd is configured.
    fn load_winrt_types(&self) -> Result<HashSet<String>, Error> {
        let mut winmd_files = vec![];
        for file_name in &self.resolution_input {
            winmd_files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }
        let index = metadata::reader::Index::new(winmd_files);
        let mut set = HashSet::new();
        for (namespace, name, _) in index.iter() {
            let bare = name.split('`').next().unwrap_or(name);
            set.insert(format!("{namespace}.{bare}"));
        }
        Ok(set)
    }

    /// Parses the configured headers once and emits one RDL string per spec.
    ///
    /// The translation units are parsed a single time and reused across every spec; each
    /// spec walks the cached TUs with its own header filter and reference map, so the
    /// shared `windows.h` prelude is never re-parsed.
    fn parse_and_emit(
        &self,
        reference: &metadata::reader::Index,
        specs: &[NamespaceSpec<'_>],
    ) -> Result<Vec<String>, Error> {
        // Parse every input once; the resulting translation units are reused for all specs.
        let parsed = self.parse_inputs()?;
        let arg_refs: Vec<&str> = parsed.args.iter().map(String::as_str).collect();

        // Pass 1 — discover ownership. Walk the cached TUs once per spec with the
        // upstream-only reference to learn which *type* names each namespace emits,
        // building a global in-house `name → namespace` table. The expensive macro
        // re-evaluation is skipped here: it only yields `const`s, which are never the
        // target of a cross-namespace *type* reference, so it is not needed to know
        // ownership.
        //
        // A name emitted by more than one namespace is a namespace-local typedef
        // artifact (e.g. the pointer aliases `LPVOID`/`LPWORD` that every namespace
        // re-emits), not an authoritative cross-namespace owner — those must keep
        // resolving locally, so a conflicting name is dropped from the table (`None`).
        let mut owners: HashMap<String, Option<String>> = HashMap::new();
        for spec in specs {
            let ref_map = build_ref_map(reference, spec.namespace);
            let mut collector = Collector::new();
            for (_, tu) in &parsed.h_tus {
                self.process_tu(tu, &mut collector, &ref_map, spec)?;
            }
            for (_, tu) in &parsed.str_tus {
                self.process_tu(tu, &mut collector, &ref_map, spec)?;
            }
            for name in collector.keys() {
                owners
                    .entry(name.clone())
                    .and_modify(|owner| {
                        if owner.as_deref() != Some(spec.namespace) {
                            *owner = None;
                        }
                    })
                    .or_insert_with(|| Some(spec.namespace.to_string()));
            }
        }
        let in_house: HashMap<String, String> = owners
            .into_iter()
            .filter_map(|(name, owner)| owner.map(|ns| (name, ns)))
            .collect();

        // Pass 2 — emit. Resolve cross-namespace references against the in-house table
        // first (upstream as fallback for names not yet built in-house) and run the full
        // macro evaluation. Where upstream and in-house agree the output is unchanged;
        // the table is what lets resolution survive once the upstream reference is dropped.
        let mut outputs = Vec::with_capacity(specs.len());

        for spec in specs {
            let ref_map = build_resolution_map(reference, &in_house, spec.namespace);
            let mut collector = Collector::new();

            for (input, tu) in &parsed.h_tus {
                let pending = self.process_tu(tu, &mut collector, &ref_map, spec)?;
                for c in Const::evaluate_macros(input, &pending, &parsed.index, &arg_refs)? {
                    collector.insert(Item::Const(c));
                }
            }

            for (content, tu) in &parsed.str_tus {
                let pending = self.process_tu(tu, &mut collector, &ref_map, spec)?;
                for c in Const::evaluate_macros_str(content, &pending, &parsed.index, &arg_refs)? {
                    collector.insert(Item::Const(c));
                }
            }

            outputs.push(emit_module(spec.namespace, &collector)?);
        }

        Ok(outputs)
    }

    /// Process a parsed translation unit: check for fatal diagnostics, walk
    /// the cursor tree to populate `collector` with top-level items, and
    /// return the names of any object-like macros whose bodies are too complex
    /// for the token-based parser (to be evaluated in a second pass).
    fn process_tu(
        &self,
        tu: &TranslationUnit,
        collector: &mut Collector,
        ref_map: &HashMap<String, String>,
        spec: &NamespaceSpec<'_>,
    ) -> Result<Vec<String>, Error> {
        for diag in tu.diagnostics() {
            if diag.is_err() {
                return Err(Error::new(
                    &diag.message,
                    &diag.file_name,
                    diag.line.try_into().unwrap(),
                    (diag.column.saturating_sub(1)).try_into().unwrap(),
                ));
            }
        }

        // Build a map from struct/enum tag names to their preferred public typedef
        // aliases.  This handles the C idiom `typedef struct _TAG {} TAG, *PTAG;`
        // where `_TAG` is the internal tag and `TAG` is the intended public name.
        let mut tag_rename = build_tag_rename_map(tu);

        // Extend tag_rename with synthetic names for all nested struct/union
        // types, using the source location as the key for anonymous types (since
        // anonymous type cursors all return an empty spelling) and the tag name
        // as the key for named types.  Synthetic names follow the writer's scheme:
        // `{OuterName}_{index}` where index is the 0-based position of the nested
        // definition among all struct/union definitions in the parent.  All nested
        // types use synthetic names regardless of their C name to avoid collisions.
        assign_nested_names(tu, &mut tag_rename);
        let enum_merge = merge_enum_typedef_idiom(tu, &mut tag_rename);
        let macro_defs = collect_macro_defs(tu);

        let mut parser = Parser::new(
            spec.namespace,
            spec.library,
            spec.libraries,
            ref_map,
            &tag_rename,
            &enum_merge,
            &macro_defs,
            tu,
            spec.symbols,
        );

        for child in tu.cursor().children() {
            // Only process cursors from the main input file or from headers matching the
            // spec's path-suffix filters.
            if !child.is_from_main_file() {
                let passes_filter = !spec.filter.is_empty() && {
                    let file = child.file_name();
                    spec.filter.iter().any(|f| matches_filter(&file, f))
                };
                if !passes_filter {
                    // A CXCursor_LinkageSpec produced by a linkage macro such as
                    // `#define EXTERN_C extern "C"` (e.g. `STDAPI`/`WINAPI` functions)
                    // reports its *spelling* location at the macro definition, which is
                    // typically an unfiltered header (winnt.h, combaseapi.h). Fall back
                    // to its *expansion* location — where the macro is invoked — which
                    // is the API header, so accept it when that matches the main file or
                    // the spec's filter.
                    let passes_expansion = child.kind() == CXCursor_LinkageSpec && {
                        child.is_expansion_from_main_file(tu) || {
                            let file = child.expansion_file_name();
                            spec.filter.iter().any(|f| matches_filter(&file, f))
                        }
                    };
                    if !passes_expansion {
                        continue;
                    }
                }
            }

            parser.process_cursor(child, collector, false)?;
        }

        // Emit typedef/callback items for types referenced by main-file items but defined
        // only in included headers. The vec may grow during this loop as transitive
        // dependencies are discovered.
        let mut seen: HashSet<String> = HashSet::new();
        let mut i = 0;
        while i < parser.pending_typedefs.len() {
            let cursor = parser.pending_typedefs[i];
            i += 1;
            let name = cursor.name();
            // Skip if already processed, already collected, or in the reference.
            if !seen.insert(name.clone())
                || collector.contains_key(&name)
                || parser.ref_map.contains_key(&name)
            {
                continue;
            }
            if let Some(cb) = Callback::parse(cursor, &mut parser)? {
                collector.insert(Item::Callback(cb));
            } else if let Some(td) = Typedef::parse(cursor, &mut parser)? {
                collector.insert(Item::Typedef(td));
            }
        }

        // Apply IID variable declarations (e.g. `const GUID IID_IFoo = { ... }`)
        // to interfaces that don't already have a UUID from __declspec(uuid(...)).
        collector.apply_iid_vars(&parser.iid_vars);

        Ok(parser.pending_macros)
    }
}

/// Owns the libclang state and parsed translation units produced by
/// [`Clang::parse_inputs`], shared by both the namespaced and per-header emit paths.
/// The field declaration order is the drop order: the translation units and index are
/// disposed before the `Library` guard unloads libclang.
struct ParsedInputs {
    args: Vec<String>,
    h_tus: Vec<(String, TranslationUnit)>,
    str_tus: Vec<(String, TranslationUnit)>,
    index: Index,
    _library: Library,
}

/// Flatten the direct children of `parent` into a list of `(declaration, extern_c)`
/// pairs, descending through `extern "C"`/`extern "C++"` linkage blocks at any depth.
///
/// A single linkage block opened in one header can wrap declarations pulled in via
/// `#include` from many other headers, so per-header routing must look through the
/// block and bucket each enclosed declaration by its own defining header rather than
/// the block's. `extern_c` tracks whether the declaration sits inside a C linkage
/// block (matching the per-child language test [`process_cursor`] applies when it
/// recurses into a `CXCursor_LinkageSpec`).
///
/// `abi_ns` is `Some(path)` once inside the `ABI` projection namespace, carrying the
/// accumulated WinRT namespace path (`ABI` stripped, e.g. `Windows.Foundation`); it is
/// `None` everywhere else. `winrt_types`, when supplied, opts the walk into descending
/// into `ABI`: a declaration there is captured only when its `{path}.{name}` is *absent*
/// from the set (a Win32 COM interop entity such as `IActivatableClassRegistration`), and
/// a name *present* in the set (a true `Windows.winmd` projection) is left for a
/// cross-winmd reference. Class/function templates in the ABI namespace (the open generic
/// interface definitions) are always skipped — only concrete declarations are captured.
fn flatten_decls(
    parent: Cursor,
    in_linkage: bool,
    in_interop_ns: bool,
    abi_ns: Option<&str>,
    winrt_types: Option<&HashSet<String>>,
    out: &mut Vec<(Cursor, bool)>,
) {
    for child in parent.children() {
        if child.kind() == CXCursor_LinkageSpec {
            flatten_decls(child, true, in_interop_ns, abi_ns, winrt_types, out);
        } else if child.kind() == CXCursor_Namespace {
            if let Some(path) = abi_ns {
                // Already inside `ABI`: accumulate the WinRT namespace path and keep descending.
                let name = child.name();
                let child_path = if path.is_empty() {
                    name
                } else {
                    format!("{path}.{name}")
                };
                flatten_decls(
                    child,
                    in_linkage,
                    in_interop_ns,
                    Some(&child_path),
                    winrt_types,
                    out,
                );
            } else if winrt_types.is_some() && child.name() == "ABI" {
                // Enter the `ABI::Windows::*` projection namespace to reach the Win32 COM interop
                // entities the SDK declares there (classified against `winrt_types` below). The
                // path starts empty; `ABI` itself is stripped from the WinRT namespace.
                flatten_decls(child, in_linkage, in_interop_ns, Some(""), winrt_types, out);
            } else if in_interop_ns || child.name() == "Windows" {
                // The hand-authored `Windows::*` C++ interop namespace — e.g.
                // `Windows::Storage::Streams::IBufferByteAccess` (robuffer.h) and
                // `Windows::Foundation::IMemoryBufferByteAccess` (memorybuffer.h) — routed to the
                // flat root (matching how win32metadata maps them to `Windows.Win32.System.WinRT`).
                // Every other top-level namespace (`std`/`Concurrency`/`Microsoft` C++ support
                // libraries) is left alone.
                flatten_decls(child, in_linkage, true, None, winrt_types, out);
            }
        } else if let (Some(path), Some(set)) = (abi_ns, winrt_types) {
            // A declaration inside `ABI`. Skip the open generic templates (their concrete
            // instantiations are reached through references, not as declarations) and skip any
            // name that is a true `Windows.winmd` projection — only Win32 COM interop entities
            // absent from the winmd are captured into the flat root.
            if matches!(
                child.kind(),
                CXCursor_ClassTemplate
                    | CXCursor_ClassTemplatePartialSpecialization
                    | CXCursor_FunctionTemplate
            ) {
                continue;
            }
            let name = child.name();
            let full = if path.is_empty() {
                name.clone()
            } else {
                format!("{path}.{name}")
            };
            if set.contains(&full) {
                continue;
            }
            let extern_c = in_linkage && child.language() == CXLanguage_C;
            out.push((child, extern_c));
        } else {
            let extern_c = in_linkage && child.language() == CXLanguage_C;
            out.push((child, extern_c));
        }
    }
}

/// True for a `DECLARE_HANDLE` tag struct: the dummy `struct X__ { int unused; }` that
/// backs an opaque handle typedef (e.g. `HWND`). `DECLARE_HANDLE(X)` expands to
/// `struct X__ { int unused; }; typedef struct X__ *X`, so the genuine tag is empty or
/// has a single primitive-`int` field and is never emitted as a type of its own (the
/// handle typedef itself emits `*mut void`). MIDL also tags real *value* structs with the
/// same `<name>__` convention (`typedef struct X__ { … } X`), but those carry
/// record/typedef/multiple fields — keep them so the `<name>` typedef they back resolves.
///
/// The same drop applies to a MIDL file-scope handle placeholder
/// (`struct __MIDL___MIDL_itf_* { int unused; }`, [`is_midl_placeholder_tag`]): it is the
/// opaque tag behind an IDL handle typedef, which likewise emits `*mut void`.
fn is_handle_tag_struct(child: &Cursor) -> bool {
    if !matches!(child.kind(), CXCursor_StructDecl | CXCursor_UnionDecl) || !child.is_definition() {
        return false;
    }
    let name = child.name();
    if !name.ends_with("__") && !is_midl_placeholder_tag(&name) {
        return false;
    }
    is_handle_shape(child)
}

/// MIDL generates a pair of per-method marshaling thunks for every COM interface
/// method in the `*_p.c` proxy file that the generated header pulls in
/// (`IDispatch_Invoke_Proxy`, `IDispatch_Invoke_Stub`, the `_Remote*` variants, …).
/// These are RPC plumbing, not part of the public API the `.idl` declares, and they
/// pollute the scrape only because we read the post-MIDL header rather than the IDL.
///
/// They are recognised by the MIDL naming convention — a `_Proxy`/`_Stub` suffix —
/// together with the generated explicit `This` first parameter that a flat C API
/// never has, *and* the absence of an exporting DLL: a handful of these thunks
/// (`IUnknown_QueryInterface_Proxy`, `IUnknown_AddRef_Proxy`,
/// `IUnknown_Release_Proxy`) are genuine `RPCRT4.dll` exports, so a symbol present
/// in the import libraries is a real API and kept. The three conditions together
/// yield zero false positives across the SDK closure.
fn is_midl_proxy_stub(cursor: &Cursor, libraries: &HashMap<String, String>) -> bool {
    let name = cursor.name();
    if !name.ends_with("_Proxy") && !name.ends_with("_Stub") {
        return false;
    }
    if libraries.contains_key(&name) {
        return false;
    }
    cursor
        .children()
        .iter()
        .find(|c| c.kind() == CXCursor_ParmDecl)
        .is_some_and(|p| p.name() == "This")
}

/// MIDL generates a quartet of wire-marshaling helpers for every user-marshaled type
/// (`CLIPFORMAT_UserSize`/`_UserMarshal`/`_UserUnmarshal`/`_UserFree`, plus the `64`
/// variants the 64-bit stubs use). They are invoked only by the generated stub code to
/// serialize a value onto the RPC wire — never by application code — so the reference
/// metadata omits the whole category, even for the handful a runtime DLL (e.g.
/// `ole32.dll`) genuinely exports. They are recognised by the fixed suffix set together
/// with the mandatory leading `unsigned long *` marshaling-flags parameter that no flat
/// public API carries, giving zero false positives across the SDK closure.
fn is_midl_user_marshal_stub(cursor: &Cursor) -> bool {
    let name = cursor.name();
    let base = name.strip_suffix("64").unwrap_or(&name);
    if !base.ends_with("_UserSize")
        && !base.ends_with("_UserMarshal")
        && !base.ends_with("_UserUnmarshal")
        && !base.ends_with("_UserFree")
    {
        return false;
    }
    cursor
        .children()
        .iter()
        .find(|c| c.kind() == CXCursor_ParmDecl)
        .is_some_and(|p| {
            let ty = p.ty().canonical_type();
            ty.kind() == CXType_Pointer && ty.pointee_type().canonical_type().kind() == CXType_ULong
        })
}

/// Collect every enum member (variant) name across all partitions, mapping each name to
/// the set of `i64` values it is defined with. Used to detect a top-level constant that
/// redundantly duplicates an enum member of the same name and value.
fn enum_member_values(collectors: &BTreeMap<String, Collector>) -> HashMap<String, Vec<i64>> {
    let mut members: HashMap<String, Vec<i64>> = HashMap::new();
    for collector in collectors.values() {
        for item in collector.values() {
            if let Item::Enum(e) = item {
                for (name, value) in &e.variants {
                    members.entry(name.clone()).or_default().push(*value);
                }
            }
        }
    }
    members
}

/// Reinterpret an integer [`metadata::Value`] as an `i128` bit pattern for comparison with
/// an enum member value. Non-integer constants (floats, strings, type names) return `None`.
fn const_integer_bits(value: &metadata::Value) -> Option<i128> {
    Some(match value {
        metadata::Value::Bool(v) => *v as i128,
        metadata::Value::U8(v) => *v as i128,
        metadata::Value::I8(v) => *v as i128,
        metadata::Value::U16(v) => *v as i128,
        metadata::Value::I16(v) => *v as i128,
        metadata::Value::U32(v) => *v as i128,
        metadata::Value::I32(v) => *v as i128,
        metadata::Value::U64(v) => *v as i128,
        metadata::Value::I64(v) => *v as i128,
        metadata::Value::USize(v) => *v as i128,
        metadata::Value::ISize(v) => *v as i128,
        metadata::Value::EnumValue(_, inner) => return const_integer_bits(inner),
        _ => return None,
    })
}

/// Whether an enum member's `i64` value equals a constant's `i128` value. The enum member
/// carries clang's raw (possibly sign-extended) bit pattern, so a 32-bit-truncated compare
/// also matches a high-bit flag value (e.g. `0x8000_0000`) that the enum stores signed while
/// the macro constant stores unsigned. Only the enum side is truncated, so a wide constant is
/// never spuriously matched to a narrow enum member.
fn enum_member_eq(member: i64, constant: i128) -> bool {
    member as i128 == constant || member as u32 as i128 == constant
}

/// Wraps a collector's items in the nested `mod` path for `namespace`, producing the
/// RDL source for a single namespace (e.g. `#[win32] mod Windows { mod Win32 { … } }`).
fn emit_module(namespace: &str, collector: &Collector) -> Result<String, Error> {
    let parts: Vec<&str> = namespace.split('.').collect();
    let mut output = format!("#[win32] mod {} {{", parts[0]);

    for part in &parts[1..] {
        output.push_str(&format!("mod {part} {{"));
    }

    for item in collector.values() {
        output.push_str(&item.write(namespace)?.to_string());
    }

    for _ in 0..parts.len() {
        output.push('}');
    }

    Ok(output)
}

/// Convert an enum variant's `i64` value to the [`metadata::Value`] variant
/// that matches the enum's underlying integer representation.
fn enum_variant_value(repr: &str, value: i64) -> metadata::Value {
    match repr {
        "u8" => metadata::Value::U8(value as u8),
        "i8" => metadata::Value::I8(value as i8),
        "u16" => metadata::Value::U16(value as u16),
        "i16" => metadata::Value::I16(value as i16),
        "u32" => metadata::Value::U32(value as u32),
        "u64" => metadata::Value::U64(value as u64),
        "i64" => metadata::Value::I64(value),
        _ => metadata::Value::I32(value as i32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_refs_descend_into_anonymous_nested_records() {
        // A C11 anonymous nested member carries its real content in `nested` with a
        // `Void` `ty`; walking only `field.ty` would miss every type it references.
        let nested = Struct {
            name: "Anonymous".to_string(),
            fields: vec![Field {
                name: "value".to_string(),
                ty: metadata::Type::ValueName(metadata::TypeName::named("", "InnerType")),
                nested: None,
                bitfields: vec![],
            }],
            is_union: true,
            packing: None,
            alignment: None,
        };
        let field = Field {
            name: "Anonymous".to_string(),
            ty: metadata::Type::Void,
            nested: Some(Box::new(nested)),
            bitfields: vec![],
        };

        let mut refs = HashSet::new();
        collect_field_refs(std::slice::from_ref(&field), &mut refs);
        assert!(refs.contains("InnerType"));
    }

    #[test]
    fn value_refs_name_typed_constants() {
        let mut refs = HashSet::new();
        collect_value_refs(
            &metadata::Value::TypeName(metadata::TypeName::named("", "NamedType")),
            &mut refs,
        );
        assert!(refs.contains("NamedType"));

        let mut enum_refs = HashSet::new();
        collect_value_refs(
            &metadata::Value::EnumValue(
                metadata::TypeName::named("", "NamedEnum"),
                Box::new(metadata::Value::I32(3)),
            ),
            &mut enum_refs,
        );
        assert!(enum_refs.contains("NamedEnum"));
    }

    #[test]
    fn const_integer_bits_reads_integers_only() {
        assert_eq!(const_integer_bits(&metadata::Value::U32(22)), Some(22));
        assert_eq!(const_integer_bits(&metadata::Value::I32(-1)), Some(-1));
        assert_eq!(
            const_integer_bits(&metadata::Value::U32(0x8000_0000)),
            Some(0x8000_0000)
        );
        assert_eq!(
            const_integer_bits(&metadata::Value::EnumValue(
                metadata::TypeName::named("", "E"),
                Box::new(metadata::Value::U16(7)),
            )),
            Some(7)
        );
        assert_eq!(const_integer_bits(&metadata::Value::F32(1.0)), None);
        assert_eq!(
            const_integer_bits(&metadata::Value::Utf8("x".to_string())),
            None
        );
    }

    #[test]
    fn enum_member_eq_matches_value_and_high_bit_flag() {
        // Plain equal values match.
        assert!(enum_member_eq(22, 22));
        assert!(!enum_member_eq(22, 23));
        // A high-bit flag the enum stores sign-extended (`int` backing) still matches the
        // unsigned macro constant via the 32-bit-truncated compare.
        assert!(enum_member_eq(-2147483648, 0x8000_0000));
        // A wide constant is never spuriously matched to a narrow enum member sharing its
        // low 32 bits.
        assert!(!enum_member_eq(0, 0x1_0000_0000));
    }

    #[test]
    fn enum_member_values_collects_variants_across_partitions() {
        let mut a = Collector::new();
        a.insert(Item::Enum(Enum {
            name: "D3DFORMAT".to_string(),
            repr: "i32",
            variants: vec![("D3DFMT_X8R8G8B8".to_string(), 22)],
            flags: false,
            scoped: false,
        }));
        let mut b = Collector::new();
        b.insert(Item::Const(Const {
            name: "D3DFMT_X8R8G8B8".to_string(),
            value: metadata::Value::U32(22),
        }));

        let collectors: BTreeMap<String, Collector> =
            [("d3d9types".to_string(), a), ("mfapi".to_string(), b)].into();
        let members = enum_member_values(&collectors);
        assert_eq!(
            members.get("D3DFMT_X8R8G8B8").map(Vec::as_slice),
            Some([22].as_slice())
        );
    }
}
