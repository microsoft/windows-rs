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
                    // definition (unusual but possible), mark it now.
                    if self.flag_enums.contains(&e.name) {
                        e.flags = true;
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
                    if !body_has_non_type_keyword && !body_has_string_literal && body_is_balanced {
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

    /// Parses the configured headers once and emits one RDL string per emitted
    /// defining-header file, keyed by the header stem (e.g. `Windef`). Every string
    /// is the flat root namespace (`Windows.Win32`); the stem only names the file.
    fn parse_and_emit_by_header(
        &self,
        root: &str,
        allow: &HashSet<&str>,
    ) -> Result<BTreeMap<String, String>, Error> {
        let (h_paths, _) = expand_input_paths(&self.input, "h", "winmd")?;

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

        let _library = Library::new()?;
        let index = Index::new()?;

        let target_arg: Option<String> = self.target.as_ref().map(|t| format!("--target={t}"));
        let args: Vec<&str> = target_arg
            .iter()
            .map(String::as_str)
            .chain(self.args.iter().map(String::as_str))
            .collect();

        let mut h_tus = vec![];
        for input in &h_paths {
            h_tus.push((input.as_str(), index.parse(input, &args)?));
        }
        let mut str_tus = vec![];
        for content in &self.input_str {
            str_tus.push((
                content.as_str(),
                index.parse_unsaved(
                    ".h",
                    content,
                    &args,
                    CXTranslationUnit_DetailedPreprocessingRecord,
                )?,
            ));
        }

        let mut collectors: BTreeMap<String, Collector> = BTreeMap::new();
        // Per-partition scope flag (defining-header stem → in-scope), accumulated while
        // bucketing so the reachability sweep below knows which partitions are roots.
        let mut scope_in: BTreeMap<String, bool> = BTreeMap::new();

        for (input, tu) in &h_tus {
            self.process_tu_by_header(
                tu,
                root,
                allow,
                &mut collectors,
                &mut scope_in,
                MacroEval {
                    source: MacroSource::File(input),
                    args: &args,
                },
            )?;
        }
        for (content, tu) in &str_tus {
            self.process_tu_by_header(
                tu,
                root,
                allow,
                &mut collectors,
                &mut scope_in,
                MacroEval {
                    source: MacroSource::Str(content),
                    args: &args,
                },
            )?;
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
        root: &str,
        allow: &HashSet<&str>,
        collectors: &mut BTreeMap<String, Collector>,
        scope_in: &mut BTreeMap<String, bool>,
        eval: MacroEval<'_>,
    ) -> Result<(), Error> {
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

        let mut tag_rename = build_tag_rename_map(tu);
        assign_nested_names(tu, &mut tag_rename);
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
        flatten_decls(tu.cursor(), false, false, &mut decls);

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
                &macro_defs,
                tu,
                &empty_symbols,
            );
            parser.header_root = Some(root);
            parser.drop_lib_less = self.drop_lib_less;

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
        let (h_paths, _) = expand_input_paths(&self.input, "h", "winmd")?;

        let _library = Library::new()?;
        let index = Index::new()?;

        // Build the effective args list: optional --target= first, then user args.
        let target_arg: Option<String> = self.target.as_ref().map(|t| format!("--target={t}"));
        let args: Vec<&str> = target_arg
            .iter()
            .map(String::as_str)
            .chain(self.args.iter().map(String::as_str))
            .collect();

        // Parse every input once; the resulting translation units are reused for all specs.
        let mut h_tus = vec![];
        for input in &h_paths {
            h_tus.push((input.as_str(), index.parse(input, &args)?));
        }
        let mut str_tus = vec![];
        for content in &self.input_str {
            str_tus.push((
                content.as_str(),
                index.parse_unsaved(
                    ".h",
                    content,
                    &args,
                    CXTranslationUnit_DetailedPreprocessingRecord,
                )?,
            ));
        }

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
            for (_, tu) in &h_tus {
                self.process_tu(tu, &mut collector, &ref_map, spec)?;
            }
            for (_, tu) in &str_tus {
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

            for (input, tu) in &h_tus {
                let pending = self.process_tu(tu, &mut collector, &ref_map, spec)?;
                for c in Const::evaluate_macros(input, &pending, &index, &args)? {
                    collector.insert(Item::Const(c));
                }
            }

            for (content, tu) in &str_tus {
                let pending = self.process_tu(tu, &mut collector, &ref_map, spec)?;
                for c in Const::evaluate_macros_str(content, &pending, &index, &args)? {
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
        let macro_defs = collect_macro_defs(tu);

        let mut parser = Parser::new(
            spec.namespace,
            spec.library,
            spec.libraries,
            ref_map,
            &tag_rename,
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

/// Reads an import library and extends `map` with its symbol → DLL entries without
/// overwriting existing mappings.
fn extend_libraries(map: &mut HashMap<String, String>, path: &str) -> Result<(), Error> {
    let bytes = std::fs::read(path).map_err(|_| Error::new("invalid input", path, 0, 0))?;
    for import in implib::read(&bytes)? {
        map.entry(import.symbol).or_insert(import.dll);
    }
    Ok(())
}

/// Builds the name → namespace resolution map from the reference metadata, excluding
/// the namespace under construction so its own types are re-emitted from source rather
/// than resolving to the (possibly stale or upstream) reference copy. Types from other
/// namespaces stay in the map so cross-namespace dependencies (`Foundation::HRESULT`,
/// `Gdi::HMONITOR`, …) resolve as qualified references.
fn build_ref_map(reference: &metadata::reader::Index, exclude: &str) -> HashMap<String, String> {
    let mut ref_map = HashMap::new();
    for (namespace, name, _) in reference.iter() {
        if namespace == exclude {
            continue;
        }
        ref_map.insert(name.to_string(), namespace.to_string());
    }
    ref_map
}

/// Overlays the in-house `name → namespace` table (built from what each partition
/// emits) onto the upstream reference map so cross-namespace references resolve
/// against types we build ourselves, falling back to the upstream `reference` only
/// for names not yet built in-house. In-house entries win, and the namespace under
/// construction is excluded from both so its own types re-emit from source. This is
/// the resolution half of becoming self-sustaining: as in-house coverage grows the
/// upstream fallback shrinks toward zero, the prerequisite for dropping `--reference`.
fn build_resolution_map(
    reference: &metadata::reader::Index,
    in_house: &HashMap<String, String>,
    exclude: &str,
) -> HashMap<String, String> {
    let mut map = build_ref_map(reference, exclude);
    for (name, namespace) in in_house {
        if namespace == exclude {
            continue;
        }
        map.insert(name.clone(), namespace.clone());
    }
    map
}

/// Maps a clang declaration cursor to the namespace of its **defining header**,
/// e.g. a typedef declared in `wingdi.h` with `root = "Windows.Win32"` becomes
/// `Windows.Win32.Wingdi`. This is the per-header partition key: intrinsic to the
/// source (clang's cursor location), total over all declarations, and stable across
/// SDK versions — unlike the editorial namespaces of `win32metadata`.
///
/// Returns the defining-header partition leaf (file stem) for `cursor`, e.g.
/// `Windef` for a declaration written in `windef.h`. Used to route each emitted
/// declaration to its per-header output file in the flat `Windows.Win32` namespace.
///
/// `CXCursor_LinkageSpec` cursors produced by linkage macros (`STDAPI`/`WINAPI`)
/// report their spelling location at the macro definition site (an unfiltered
/// header such as `winnt.h`); the *expansion* location — where the macro was
/// invoked — is the real API header, so it is preferred when present.
///
/// Returns `None` for cursors with no associated file (builtins, the predefined
/// translation-unit buffer).
fn header_stem_of(cursor: &Cursor) -> Option<String> {
    let file = header_path_of(cursor)?;
    let stem = header_stem_to_namespace(&file);
    if stem.is_empty() {
        // The synthetic top-level translation-unit buffer is parsed as `.h` (an empty
        // stem); anything clang attributes to it (predefined/builtin artifacts) is not
        // a real header declaration and must not create an empty partition.
        return None;
    }
    Some(stem)
}

/// Returns the full defining-header path for `cursor` (the spelling location, falling
/// back to the macro expansion location), or `None` for cursors with no associated file.
/// Unlike [`header_stem_of`] this keeps the directory, so the scope predicate can tell an
/// SDK `um`/`shared` header from a C-runtime (`ucrt`) or MSVC-toolset (`include`) one.
fn header_path_of(cursor: &Cursor) -> Option<String> {
    let file = cursor.file_name();
    let file = if file.is_empty() {
        cursor.expansion_file_name()
    } else {
        file
    };
    if file.is_empty() { None } else { Some(file) }
}

/// Whether a defining-header `path` is in scope: one of its directory components (after
/// collapsing `.`/`..`) equals a `scope` segment, e.g. `um`/`shared` matches
/// `…/Include/10.0.26100.0/um/winuser.h` (and the nested `…/um/gl/gl.h`) but not a
/// C-runtime header under `…/ucrt/`. Matching is case-insensitive, separator-agnostic,
/// and component-based so a sibling directory reached via `..` cannot match by substring.
fn header_in_scope(path: &str, scope: &[String]) -> bool {
    let norm = path.replace('\\', "/").to_lowercase();
    let mut components: Vec<&str> = vec![];
    for part in norm.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            other => components.push(other),
        }
    }
    // The final component is the file name itself; only the directory components define
    // scope.
    let dirs = components.split_last().map_or(&[][..], |(_, dirs)| dirs);
    let want: HashSet<String> = scope.iter().map(|s| s.to_lowercase()).collect();
    dirs.iter().any(|dir| want.contains(*dir))
}

/// Collects the bare names of every nominal type (`ClassName`/`ValueName`, including
/// generic arguments) referenced by `ty`, descending through pointer/array/reference
/// wrappers. Primitive and built-in types contribute no name.
fn collect_type_refs(ty: &metadata::Type, out: &mut HashSet<String>) {
    match ty {
        metadata::Type::ClassName(name) | metadata::Type::ValueName(name) => {
            out.insert(name.name.clone());
            for generic in &name.generics {
                collect_type_refs(generic, out);
            }
        }
        metadata::Type::Array(inner)
        | metadata::Type::RefMut(inner)
        | metadata::Type::RefConst(inner)
        | metadata::Type::PtrMut(inner, _)
        | metadata::Type::PtrConst(inner, _)
        | metadata::Type::ArrayFixed(inner, _) => collect_type_refs(inner, out),
        _ => {}
    }
}

/// Collects the bare names of every nominal type a constant's value references: a
/// `TypeName` value names a type directly, and an `EnumValue` names its enum type (and,
/// recursively, its inner value). Scalar/string values contribute no name.
fn collect_value_refs(value: &metadata::Value, out: &mut HashSet<String>) {
    match value {
        metadata::Value::TypeName(tn) => {
            out.insert(tn.name.clone());
        }
        metadata::Value::EnumValue(tn, inner) => {
            out.insert(tn.name.clone());
            collect_value_refs(inner, out);
        }
        _ => {}
    }
}

/// Collects the type references of a record's fields, descending into anonymous nested
/// records (`Field::nested`). A nested anonymous member carries its real content in
/// `nested` with a `Void` `ty`, so walking only `field.ty` would miss every type the
/// nested record references.
fn collect_field_refs(fields: &[Field], out: &mut HashSet<String>) {
    for field in fields {
        collect_type_refs(&field.ty, out);
        if let Some(nested) = &field.nested {
            collect_field_refs(&nested.fields, out);
        }
    }
}

/// Collects the bare names of every type `item` references through its signature: function
/// and callback parameters/returns, interface base and method signatures, struct/union
/// field types (including anonymous nested records), typedef targets, and typed-constant
/// values. Enums and GUID constants reference only primitives (or the always-in-scope
/// `GUID`), so they contribute no edges.
fn item_refs(item: &Item, out: &mut HashSet<String>) {
    match item {
        Item::Fn(item) => {
            for param in &item.params {
                collect_type_refs(&param.ty, out);
            }
            collect_type_refs(&item.return_type, out);
        }
        Item::Callback(item) => {
            for param in &item.params {
                collect_type_refs(&param.ty, out);
            }
            collect_type_refs(&item.return_type, out);
        }
        Item::Interface(item) => {
            if let Some(base) = &item.base {
                collect_type_refs(base, out);
            }
            for method in &item.methods {
                for param in &method.params {
                    collect_type_refs(&param.ty, out);
                }
                collect_type_refs(&method.return_type, out);
            }
        }
        Item::Struct(item) => collect_field_refs(&item.fields, out),
        Item::Typedef(item) => collect_type_refs(&item.ty, out),
        Item::Const(item) => collect_value_refs(&item.value, out),
        Item::PropertyKeyConst(item) => {
            out.insert(item.ty.clone());
        }
        Item::Enum(_) | Item::GuidConst(_) => {}
    }
}

/// Reachability-by-reference sweep. Roots are every declaration in an in-scope partition;
/// the type-reference graph is then walked to mark every transitively referenced name.
/// Each out-of-scope partition keeps only its marked declarations, so the C-runtime and
/// compiler-toolset closure that no in-scope API references falls away while genuine
/// cross-over types (referenced from in-scope code) survive without any allowlist.
fn sweep_unreferenced(
    collectors: &mut BTreeMap<String, Collector>,
    scope_in: &BTreeMap<String, bool>,
) {
    // A partition with no recorded scope (e.g. the hand-authored seed, or a stem the
    // bucketing never classified) is treated as in-scope so the sweep can only ever
    // *remove* known out-of-scope noise, never in-scope surface.
    let in_scope = |stem: &str| scope_in.get(stem).copied().unwrap_or(true);

    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    let mut known: HashSet<String> = HashSet::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = vec![];
    for (stem, collector) in collectors.iter() {
        let roots = in_scope(stem);
        for (name, item) in collector.iter() {
            known.insert(name.clone());
            let mut refs = HashSet::new();
            item_refs(item, &mut refs);
            edges.insert(name.clone(), refs);
            if roots && seen.insert(name.clone()) {
                stack.push(name.clone());
            }
        }
    }

    while let Some(name) = stack.pop() {
        if let Some(refs) = edges.get(&name) {
            for r in refs {
                if known.contains(r) && seen.insert(r.clone()) {
                    stack.push(r.clone());
                }
            }
        }
    }

    for (stem, collector) in collectors.iter_mut() {
        if in_scope(stem) {
            continue;
        }
        collector.retain(|name| seen.contains(name));
    }
}

/// Reduces a header path to its partition leaf name: the file stem (basename minus
/// the final extension) with its first character upper-cased, e.g.
/// `C:\sdk\um\wingdi.h` → `Wingdi`, `shared.inl` → `Shared`.
fn header_stem_to_namespace(file: &str) -> String {
    let base = file.rsplit(['/', '\\']).next().unwrap_or(file);
    let stem = base.rsplit_once('.').map_or(base, |(s, _)| s);
    // A header whose own name is dotted (the WinRT interop headers, e.g.
    // `Windows.Devices.Display.Core.Interop.h`) must collapse to a single flat
    // partition segment. The flat Win32 surface ignores header namespacing, so the
    // leftover dots must not survive to spawn a nested `Windows::Devices::…` module
    // tree under `Win32`.
    let stem: String = stem.chars().filter(|c| *c != '.').collect();
    let mut chars = stem.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
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
fn flatten_decls(
    parent: Cursor,
    in_linkage: bool,
    in_interop_ns: bool,
    out: &mut Vec<(Cursor, bool)>,
) {
    for child in parent.children() {
        if child.kind() == CXCursor_LinkageSpec {
            flatten_decls(child, true, in_interop_ns, out);
        } else if child.kind() == CXCursor_Namespace {
            // Capture the flat WinRT interop interfaces that the SDK declares inside the
            // hand-authored `Windows::*` C++ namespaces — e.g.
            // `Windows::Storage::Streams::IBufferByteAccess` (robuffer.h) and
            // `Windows::Foundation::IMemoryBufferByteAccess` (memorybuffer.h) — routing them
            // to their defining-header partition under the flat root (matching how
            // win32metadata maps them to `Windows.Win32.System.WinRT`). The `ABI` projection
            // namespace is skipped: `ABI::Windows::*` is the C++/WinRT projection of
            // `Windows.winmd` and is out of scope. Every other top-level namespace
            // (`std`/`Concurrency`/`Microsoft` C++ support libraries) is likewise left alone.
            if in_interop_ns || child.name() == "Windows" {
                flatten_decls(child, in_linkage, true, out);
            }
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

/// Source of a translation unit for the macro second-pass evaluator: either a header
/// file path or an in-memory header string.
#[derive(Clone, Copy)]
enum MacroSource<'a> {
    File(&'a str),
    Str(&'a str),
}

/// Evaluate every stem's pending macro names concurrently, returning one result vector
/// per input entry in the original order.
///
/// Evaluate the macro second pass for every partition, but parse the header closure as
/// few times as possible.
///
/// A macro's *value* does not depend on which partition referenced it: every synthetic
/// evaluation TU `#include`s the **whole** closure, so a given `#define` resolves to one
/// translation-unit-wide value regardless of the file it is attributed to. The
/// per-partition split only decides *which file owns* the emitted const (first-owner-wins).
/// So instead of re-parsing the full closure once per macro-bearing partition (the dominant
/// cost of the scrape — hundreds of redundant full-closure parses), the deduplicated
/// **union** of all pending macro names is evaluated in a handful of synthetic TUs — one
/// per worker thread — and the results are routed back to the first partition that
/// requested each name, leaving the deterministic first-owner-wins merge byte-for-byte
/// identical to a per-partition pass.
///
/// A libclang `CXIndex` must not be shared across threads, but independent indexes parse
/// concurrently safely, so every worker owns its own [`Index`].
fn evaluate_macros_parallel(
    all_consts: &[(String, Vec<String>)],
    source: MacroSource<'_>,
    args: &[&str],
) -> Result<Vec<Vec<Const>>, Error> {
    let n = all_consts.len();
    if n == 0 {
        return Ok(vec![]);
    }

    // The deduplicated union of every partition's pending macro names, in first-seen order.
    // Evaluating each name once (rather than once per partition that references it) is what
    // collapses the redundant full-closure re-parses.
    let mut seen = HashSet::new();
    let mut union: Vec<String> = vec![];
    for (_, names) in all_consts {
        for name in names {
            if seen.insert(name.as_str()) {
                union.push(name.clone());
            }
        }
    }

    // Evaluate the union, split into one contiguous chunk per worker. Each chunk is a single
    // synthetic TU (the closure plus that chunk's evaluation enums), so the closure is parsed
    // `workers` times total instead of once per partition.
    let evaluated_union: Vec<Const> = if union.is_empty() {
        vec![]
    } else {
        let workers = std::thread::available_parallelism()
            .map_or(1, |p| p.get())
            .min(union.len());
        let chunk_size = union.len().div_ceil(workers);

        std::thread::scope(|scope| -> Result<Vec<Const>, Error> {
            let handles: Vec<_> = union
                .chunks(chunk_size)
                .map(|chunk| {
                    scope.spawn(move || -> Result<Vec<Const>, Error> {
                        // clang-sys loads `libclang` into thread-local storage, so each
                        // worker must load it before use; the guard unloads it at thread end.
                        let _library = Library::new()?;
                        // One index per worker: created and dropped on this thread, never shared.
                        let index = Index::new()?;
                        match source {
                            MacroSource::File(input) => {
                                Const::evaluate_macros(input, chunk, &index, args)
                            }
                            MacroSource::Str(content) => {
                                Const::evaluate_macros_str(content, chunk, &index, args)
                            }
                        }
                    })
                })
                .collect();

            let mut all = vec![];
            for handle in handles {
                all.extend(
                    handle
                        .join()
                        .map_err(|_| Error::new("macro evaluation worker panicked", "", 0, 0))??,
                );
            }
            Ok(all)
        })?
    };

    // Route each evaluated const back to the *first* partition (in `all_consts` order) whose
    // pending list requested it: `remove` hands the value to that partition and leaves later
    // partitions with `None`, exactly reproducing the first-owner-wins selection a
    // per-partition pass would make. Names that failed to evaluate are absent from the map
    // and dropped everywhere, as before.
    let mut map: HashMap<String, Const> = evaluated_union
        .into_iter()
        .map(|c| (c.name.clone(), c))
        .collect();
    let mut out: Vec<Vec<Const>> = Vec::with_capacity(n);
    for (_, names) in all_consts {
        let mut consts = vec![];
        for name in names {
            if let Some(c) = map.remove(name) {
                consts.push(c);
            }
        }
        out.push(consts);
    }
    Ok(out)
}

/// The context the per-header walk needs to run the macro second-pass evaluator: the
/// translation-unit source and argument list. Each evaluation creates its own libclang
/// index so the per-stem passes can run concurrently (see [`evaluate_macros_parallel`]).
#[derive(Clone, Copy)]
struct MacroEval<'a> {
    source: MacroSource<'a>,
    args: &'a [&'a str],
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

/// Build a map from C struct/enum tag names to their public typedef aliases.
///
/// Scans all top-level `CXCursor_TypedefDecl` cursors in the translation unit
/// (including those inside `extern "C"` / `extern "C++"` linkage-spec blocks)
/// and records the first typedef that directly aliases each tagged struct or
/// enum as `tag_name → typedef_name`.
///
/// This handles the common C idiom:
/// ```c
/// typedef struct _TEST { int value; } TEST, *PTEST;
/// ```
/// Here `_TEST` is the internal struct tag and `TEST` is the intended public
/// name.  The map entry `"_TEST" → "TEST"` is used by the code generator to
/// replace every occurrence of `_TEST` with `TEST` in the emitted RDL.
fn build_tag_rename_map(tu: &TranslationUnit) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for child in tu.cursor().children() {
        collect_typedef_renames(child, &mut map);
    }
    map
}

/// Inspect a single cursor for tag→typedef rename candidates and recurse
/// into `CXCursor_LinkageSpec` blocks.
fn collect_typedef_renames(cursor: Cursor, map: &mut HashMap<String, String>) {
    if cursor.kind() == CXCursor_LinkageSpec {
        for inner in cursor.children() {
            collect_typedef_renames(inner, map);
        }
        return;
    }
    if cursor.kind() != CXCursor_TypedefDecl {
        return;
    }
    let underlying = cursor.typedef_underlying_type();
    // Unwrap a single elaborated wrapper if present.
    let inner = if underlying.kind() == CXType_Elaborated {
        underlying.underlying_type()
    } else {
        underlying
    };
    if inner.kind() == CXType_Record || inner.kind() == CXType_Enum {
        let tag_name = inner.ty().name();
        let typedef_name = cursor.name();
        // Collapse the tag→typedef idiom when this typedef defines the record/enum inline
        // (`typedef struct _T {...} T;`) or aliases a private tag named with the `_`/`tag`
        // prefix idiom (`struct tagVARIANT {...}; typedef struct tagVARIANT VARIANT;`). A
        // typedef aliasing an already-public tag from elsewhere (e.g. dwrite's `typedef
        // interface ID2D1SimplifiedGeometrySink IDWriteGeometrySink;`) is a distinct alias,
        // not a rename: hijacking it would orphan every reference to that interface.
        let defines_inline = cursor.children().iter().any(|c| {
            matches!(
                c.kind(),
                CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_EnumDecl
            ) && c.is_definition()
        });
        if !tag_name.is_empty()
            && typedef_name != tag_name
            && (defines_inline || tag_name.starts_with('_') || tag_name.starts_with("tag"))
        {
            // First typedef wins (for `typedef struct _T {} T, *PT;`, `T` is
            // registered because it appears before the pointer typedef `PT`).
            map.entry(tag_name).or_insert(typedef_name);
        }
    }
}

/// Walk the translation unit and insert `key → synthetic_name` entries into
/// `tag_rename` for every nested struct/union type — whether named or anonymous.
///
/// For named types the tag name is used as the key (since `to_type()` resolves
/// `CXType_Record` by the declaration's spelling).  For anonymous types the
/// source location (`"file:line:col"`) is used as the key because their spelling
/// is always empty.
///
/// All nested types receive a synthetic name regardless of their C name to
/// avoid collisions (two different structs could each have an inner struct
/// called `Inner`).  Names follow the same scheme as the windows-rdl writer:
/// `{OuterName}_{index}` where `index` is the 0-based position of the nested
/// definition among **all** struct/union definitions in the parent body.
///
/// Recursion handles arbitrary nesting depth.
fn assign_nested_names(tu: &TranslationUnit, tag_rename: &mut HashMap<String, String>) {
    fn walk(cursor: Cursor, tag_rename: &mut HashMap<String, String>) {
        for child in cursor.children() {
            if child.kind() == CXCursor_LinkageSpec {
                walk(child, tag_rename);
            } else {
                visit_for_nested_names(child, tag_rename);
            }
        }
    }
    walk(tu.cursor(), tag_rename);
}

/// Visit a single top-level cursor; if it is a named struct/union definition,
/// assign synthetic names to all its nested type children.
fn visit_for_nested_names(cursor: Cursor, tag_rename: &mut HashMap<String, String>) {
    let kind = cursor.kind();
    if (kind == CXCursor_StructDecl || kind == CXCursor_UnionDecl) && cursor.is_definition() {
        let tag_name = cursor.name();
        // Skip anonymous top-level types – they have no outer name to derive from.
        if is_anonymous_name(&tag_name) {
            return;
        }
        let outer_name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
        assign_nested_child_names(&outer_name, cursor, tag_rename);
    }
}

/// For each struct/union definition that is a direct child of `parent`,
/// assign it a synthetic flat name `{outer_name}_{index}` and recurse to
/// handle deeper nesting.
///
/// `index` counts **all** nested struct/union definitions in order, matching
/// the writer's convention so that a type round-tripped through
/// clang → RDL → winmd → RDL produces names consistent with what the
/// writer would have generated.
fn assign_nested_child_names(
    outer_name: &str,
    parent: Cursor,
    tag_rename: &mut HashMap<String, String>,
) {
    let mut index = 0usize;
    for child in parent.children() {
        let kind = child.kind();
        if (kind == CXCursor_StructDecl || kind == CXCursor_UnionDecl) && child.is_definition() {
            let synthetic = format!("{outer_name}_{index}");
            let child_name = child.name();
            if is_anonymous_name(&child_name) {
                // Anonymous type: key by source location (unique per declaration site).
                tag_rename.insert(child.location_id(), synthetic.clone());
            } else {
                // Named type: key by the tag name so that to_type() can look it up,
                // overriding any pre-existing typedef alias with the synthetic name.
                tag_rename.insert(child_name, synthetic.clone());
            }
            // Recurse so that nested-nested types are also handled.
            assign_nested_child_names(&synthetic, child, tag_rename);
            index += 1;
        }
    }
}

/// Returns `true` if `file` ends with `filter` and the match falls on a
/// clean path-segment boundary.
///
/// Both paths are normalized to forward slashes before comparison, so this
/// works on both Windows and POSIX.  `filter("api1.h")` matches
/// `/path/to/api1.h` but not `/path/to/myapi1.h`.
/// Collect every object-like macro definition in the translation unit, mapping
/// each macro name to the spellings of its replacement-list tokens.
///
/// Used to resolve calling-convention macros (`WINAPI`, `CALLBACK`,
/// `STDMETHODCALLTYPE`, ...) back to the underlying `__stdcall` / `__cdecl` /
/// `__fastcall` keyword, transitively and regardless of which (possibly system)
/// header defined them. Only short replacement lists are retained, since
/// convention macros expand to a single token; this keeps the map small.
/// Returns `true` for C/C++ builtin arithmetic-type and signedness keywords that
/// may legitimately appear inside an integer-constant cast (e.g. `(int)`,
/// `(unsigned long)`, `(__int64)`). Such casts — used by `#define`s like
/// `CW_USEDEFAULT ((int)0x80000000)` — are valid constant expressions and must
/// not be filtered out along with genuinely non-constant keyword macros
/// (`extern "C"`, `static`, ...).
fn is_type_keyword(spelling: &str) -> bool {
    matches!(
        spelling,
        "int"
            | "long"
            | "short"
            | "char"
            | "unsigned"
            | "signed"
            | "bool"
            | "wchar_t"
            | "__int8"
            | "__int16"
            | "__int32"
            | "__int64"
    )
}

/// Returns `true` if the delimiters `()`, `[]`, `{}` in the token stream are
/// balanced and correctly nested. Used to reject object-like macros whose
/// replacement list has unbalanced delimiters before they reach the batch macro
/// evaluator, where an unbalanced `{` or `(` would swallow the enum declarations
/// of every following candidate in the same synthetic translation unit.
///
/// Delimiter *characters* are counted (skipping string/character literals and
/// comments, whose contents may contain unmatched `(`/`{`), so a delimiter glued
/// to a line-continuation by the tokenizer (e.g. `"\\\r\n}"`) is still counted
/// correctly and a valid multi-line scalar macro is never mis-rejected.
fn tokens_balanced<'a>(tokens: impl Iterator<Item = &'a (CXTokenKind, String)>) -> bool {
    let mut stack: Vec<char> = vec![];
    for (kind, spelling) in tokens {
        if *kind == CXToken_Literal || *kind == CXToken_Comment {
            continue;
        }
        for ch in spelling.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if stack.pop() != Some(ch) => return false,
                _ => {}
            }
        }
    }
    stack.is_empty()
}

fn collect_macro_defs(tu: &TranslationUnit) -> HashMap<String, Vec<String>> {
    let mut defs = HashMap::new();

    for child in tu.cursor().children() {
        if child.kind() != CXCursor_MacroDefinition || child.is_macro_builtin() {
            continue;
        }

        let name = child.name();
        if name.is_empty() {
            continue;
        }

        let tokens = tu.tokenize(child.extent());
        // The first token is the macro name; the rest is the replacement list.
        let mut body: Vec<String> = tokens.into_iter().skip(1).map(|(_, s)| s).collect();

        // A function-like macro (`STDAPI_(type) ...`) carries its parameter list
        // between the name and the replacement list; strip `( ... )` so the body
        // holds only replacement tokens (e.g. `EXTERN_C type STDAPICALLTYPE`).
        if child.is_macro_function_like() && body.first().map(String::as_str) == Some("(") {
            let mut depth = 0usize;
            let mut end = None;
            for (idx, token) in body.iter().enumerate() {
                match token.as_str() {
                    "(" => depth += 1,
                    ")" => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(idx);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(end) = end {
                body.drain(0..=end);
            }
        }

        // A storage-class specifier such as `__declspec(dllimport)` may prefix an
        // export macro's replacement list (e.g. DWrite's
        // `#define DWRITE_EXPORT __declspec(dllimport) WINAPI`). It is irrelevant to
        // the calling convention but would otherwise push the body past the
        // small-macro length gate below, dropping the macro — and with it the
        // `WINAPI` token — from the table, so `DWriteCreateFactory` would fall back
        // to its `EXTERN_C` linkage and be mis-typed `extern "C"` instead of the
        // `extern "system"` (`__stdcall`) it really is. That mismatch corrupts the
        // stack on the x86 `__stdcall`/`__cdecl` ABI split.
        strip_declspec(&mut body);

        if !body.is_empty() && body.len() <= 4 {
            defs.insert(name, body);
        }
    }

    defs
}

/// Build the reverse alias map consumed by [`Parser::alias_map`] and [`Fn::parse`].
///
/// Scans the object-like macro definitions for the `#define <Alias> <Export>` forwarders
/// the SDK uses to expose an export under a documented name (`RtlGenRandom` →
/// `SystemFunction036`, `EnumProcesses` → `K32EnumProcesses`, `GetMappedFileNameW` →
/// `K32GetMappedFileNameW`) and inverts them to `export -> alias` so a scraped function —
/// whose clang name is the expanded export — can recover the source spelling.
///
/// ANSI/Unicode charset-selection macros (`#define GetWindowText GetWindowTextA`, the
/// `#ifdef UNICODE` idiom) are excluded: their replacement is the same name with an `A`/`W`
/// suffix, selecting a character-set variant rather than forwarding to a distinct export.
/// The reference metadata emits only the explicit `…A`/`…W` functions, so renaming the `A`
/// variant back to the bare name would both diverge from the reference and delete the
/// variant. On the rare chance two aliases share one export, the lexicographically smallest
/// is chosen so the result is deterministic across the unordered macro map.
fn build_alias_map(macro_defs: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (alias, body) in macro_defs {
        let [export] = body.as_slice() else {
            continue;
        };
        if export == alias || !is_c_identifier(export) {
            continue;
        }
        if *export == format!("{alias}A") || *export == format!("{alias}W") {
            continue;
        }
        map.entry(export.clone())
            .and_modify(|current| {
                if alias < current {
                    current.clone_from(alias);
                }
            })
            .or_insert_with(|| alias.clone());
    }
    map
}

/// True when `s` is a well-formed C identifier (leading letter/underscore, then
/// letters/digits/underscores), used to reject macro replacement lists that are not a bare
/// symbol name (`#define TRUE 1`, operator spellings, ...).
fn is_c_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
/// Strip a leading `__declspec(...)` / `_declspec(...)` storage-class specifier (either MSVC
/// spelling) from a macro body, leaving only the tokens that matter for calling-convention
/// detection — so an export macro like `#define ORAPI _declspec(dllimport) __stdcall` keeps its
/// `__stdcall` and stays within the small-macro length gate.
fn strip_declspec(body: &mut Vec<String>) {
    let mut i = 0;
    while i < body.len() {
        if matches!(body[i].as_str(), "__declspec" | "_declspec")
            && body.get(i + 1).map(String::as_str) == Some("(")
        {
            let mut depth = 0usize;
            let mut end = None;
            for (idx, token) in body.iter().enumerate().skip(i + 1) {
                match token.as_str() {
                    "(" => depth += 1,
                    ")" => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(idx);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(end) = end {
                body.drain(i..=end);
                continue;
            }
        }
        i += 1;
    }
}

fn matches_filter(file: &str, filter: &str) -> bool {
    if filter.is_empty() {
        return false;
    }
    let file = file.replace('\\', "/");
    let filter = filter.replace('\\', "/");
    file.ends_with(filter.as_str())
        && (file.len() == filter.len() || file.as_bytes()[file.len() - filter.len() - 1] == b'/')
}

/// Returns `true` if the clang `Type` refers to a GUID struct.
///
/// Handles `const GUID`, `const IID`, `const struct _GUID`, elaborated types,
/// and typedef aliases for `GUID`/`IID`.
fn is_guid_type(ty: &Type) -> bool {
    // Peel off any top-level const qualifier by looking at the canonical type.
    let name = match ty.kind() {
        CXType_Elaborated => ty.underlying_type().ty().name(),
        CXType_Record => ty.ty().name(),
        CXType_Typedef => ty.ty().name(),
        _ => return false,
    };
    matches!(name.as_str(), "GUID" | "_GUID" | "IID")
}

/// Parse a GUID struct initializer from the AST using `clang_Cursor_Evaluate`.
///
/// This handles cases where macro constants or expressions are used in the
/// GUID initializer (e.g. 7zip's `Z7_DEFINE_GUID` pattern) — the compiler
/// evaluates the expressions after macro expansion so the values are always
/// available regardless of how the initializer was spelled in source.
///
/// The VarDecl cursor for a GUID variable has the shape:
/// - `CXCursor_InitListExpr` (top-level, containing 4 children):
///   - `CXCursor_IntegerLiteral` × 3 (data1, data2, data3)
///   - `CXCursor_InitListExpr` (data4, containing 8 children):
///     - `CXCursor_IntegerLiteral` × 8
fn parse_guid_initializer_ast(cursor: &Cursor) -> Option<String> {
    // Find the top-level InitListExpr child of the VarDecl.
    let init_list = cursor
        .children()
        .into_iter()
        .find(|c| c.kind() == CXCursor_InitListExpr)?;

    let children = init_list.children();
    if children.len() != 4 {
        return None;
    }

    // Evaluate data1, data2, data3.
    let data1 = children[0].evaluate_unsigned()?;
    let data2 = children[1].evaluate_unsigned()?;
    let data3 = children[2].evaluate_unsigned()?;

    // Range-check: data1 ≤ u32, data2/data3 ≤ u16.
    if data1 > u32::MAX as u64 || data2 > u16::MAX as u64 || data3 > u16::MAX as u64 {
        return None;
    }

    // The 4th child should be an InitListExpr for data4[8].
    let data4_cursor = &children[3];
    if data4_cursor.kind() != CXCursor_InitListExpr {
        return None;
    }

    let data4_children = data4_cursor.children();
    if data4_children.len() != 8 {
        return None;
    }

    let mut data4 = [0u8; 8];
    for (i, child) in data4_children.iter().enumerate() {
        let v = child.evaluate_unsigned()?;
        if v > u8::MAX as u64 {
            return None;
        }
        data4[i] = v as u8;
    }

    Some(format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data1,
        data2,
        data3,
        data4[0],
        data4[1],
        data4[2],
        data4[3],
        data4[4],
        data4[5],
        data4[6],
        data4[7],
    ))
}

/// Parse a GUID struct initializer from a token stream.
///
/// Expects the token stream for a variable declaration like:
/// ```c
/// const GUID IID_IFoo = { 0x23170F69, 0x40C1, 0x278A, { 0, 0, 0, 3, 0, 1, 0, 0 } };
/// ```
///
/// Scans past the `=` token, then collects exactly 11 integer literals from
/// the balanced `{ ... { ... } }` initializer: `data1` (u32), `data2` (u16),
/// `data3` (u16), and `data4[0..8]` (8 × u8).
///
/// Returns the UUID in standard `"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"` format,
/// or `None` if the token sequence does not match the expected shape.
fn parse_guid_initializer_tokens(tokens: &[(CXTokenKind, String)]) -> Option<String> {
    // Find the `=` that starts the initializer.
    let eq_pos = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "=")?;

    // Collect all integer literals after the `=`.
    let mut values = Vec::with_capacity(11);
    for (kind, spelling) in &tokens[eq_pos + 1..] {
        if *kind == CXToken_Literal {
            let v = parse_c_int_literal(spelling)?;
            values.push(v);
        }
    }

    format_guid_from_values(&values)
}

/// Parse a `DEFINE_GUID(name, l, w1, w2, b1, …, b8)` (or the
/// `DEFINE_OLEGUID(name, l, w1, w2)` shorthand) macro-expansion token stream into
/// the GUID constant's name and its standard hyphenated UUID string.
///
/// The first identifier after the opening `(` is the constant's name; the
/// remaining integer literals are the GUID field values. The OLE shorthand omits
/// the trailing eight bytes, which are the fixed `{ 0xC0, 0, 0, 0, 0, 0, 0, 0x46 }`
/// sequence shared by every OLE-defined GUID.
fn parse_define_guid_tokens(
    tokens: &[(CXTokenKind, String)],
    ole: bool,
) -> Option<(String, String)> {
    let lparen = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "(")?;

    let name = tokens[lparen + 1..]
        .iter()
        .find(|(k, _)| *k == CXToken_Identifier)
        .map(|(_, s)| s.clone())?;

    let mut values: Vec<u64> = tokens[lparen + 1..]
        .iter()
        .filter(|(k, _)| *k == CXToken_Literal)
        .map(|(_, s)| parse_c_int_literal(s))
        .collect::<Option<_>>()?;

    if ole {
        if values.len() != 3 {
            return None;
        }
        values.extend_from_slice(&[0xC0, 0, 0, 0, 0, 0, 0, 0x46]);
    }

    let uuid = format_guid_from_values(&values)?;
    Some((name, uuid))
}

/// Parse a `DEFINE_PROPERTYKEY(name, l, w1, w2, b1..b8, pid)` /
/// `DEFINE_DEVPROPKEY(...)` macro invocation into `(name, fmtid_uuid, pid)`. Both macros
/// take the same 13 arguments: the constant name, the eleven GUID components, and a trailing
/// `pid`. Returns `None` when the token shape does not match (e.g. a non-literal argument).
fn parse_define_property_key_tokens(
    tokens: &[(CXTokenKind, String)],
) -> Option<(String, String, u32)> {
    let lparen = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "(")?;

    let name = tokens[lparen + 1..]
        .iter()
        .find(|(k, _)| *k == CXToken_Identifier)
        .map(|(_, s)| s.clone())?;

    let values: Vec<u64> = tokens[lparen + 1..]
        .iter()
        .filter(|(k, _)| *k == CXToken_Literal)
        .map(|(_, s)| parse_c_int_literal(s))
        .collect::<Option<_>>()?;

    // Eleven GUID components plus the trailing `pid`.
    if values.len() != 12 {
        return None;
    }

    let uuid = format_guid_from_values(&values[..11])?;
    let pid = u32::try_from(values[11]).ok()?;
    Some((name, uuid, pid))
}

/// Format the eleven GUID field values (`data1`, `data2`, `data3`, `data4[0..8]`)
/// as a standard hyphenated UUID string, range-checking each field.
fn format_guid_from_values(values: &[u64]) -> Option<String> {
    // Must have exactly 11 values: data1, data2, data3, data4[0..8].
    if values.len() != 11 {
        return None;
    }

    let data1 = values[0];
    let data2 = values[1];
    let data3 = values[2];

    // Range-check: data1 ≤ u32, data2/data3 ≤ u16, data4 bytes ≤ u8.
    if data1 > u32::MAX as u64 || data2 > u16::MAX as u64 || data3 > u16::MAX as u64 {
        return None;
    }
    for &b in &values[3..11] {
        if b > u8::MAX as u64 {
            return None;
        }
    }

    // Format as standard UUID: "data1-data2-data3-d4[0]d4[1]-d4[2]..d4[7]"
    Some(format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data1,
        data2,
        data3,
        values[3],
        values[4],
        values[5],
        values[6],
        values[7],
        values[8],
        values[9],
        values[10],
    ))
}

/// Parse a C integer literal into a `u64`, stripping any type suffix
/// (`U`, `L`, `UL`, `LL`, `ULL`, etc.) and handling hex (`0x`) and decimal.
fn parse_c_int_literal(lit: &str) -> Option<u64> {
    // Strip trailing suffixes (L, U, LL, ULL, etc.).
    let digits = lit.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else {
        digits.parse::<u64>().ok()
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
            }],
            is_union: true,
            packing: None,
            alignment: None,
        };
        let field = Field {
            name: "Anonymous".to_string(),
            ty: metadata::Type::Void,
            nested: Some(Box::new(nested)),
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
}
