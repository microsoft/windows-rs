#![allow(non_upper_case_globals)]
#![doc = include_str!("../readme.md")]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use windows_metadata as metadata;

use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;

use windows_rdl::emit::{uuid_to_u128_literal, write_ident, write_typed_value};
use windows_rdl::{Error, expand_input_files, formatter, implib, write_to_file};

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

fn write_type(namespace: &str, ty: &metadata::Type) -> TokenStream {
    windows_rdl::emit::write_type(namespace, &normalize_rdl_type(ty))
}

/// Creates a libclang-backed RDL generator.
pub fn clang() -> Clang {
    Clang::new()
}

/// Returns the loaded libclang version string.
pub fn clang_version() -> Result<String, Error> {
    Clang::version()
}

/// Parse context shared across the AST walk; pending vectors are drained after the walk.
pub(crate) struct Parser<'a> {
    pub namespace: &'a str,
    /// `Some(root)` enables per-header mode: references route through defining headers.
    pub header_root: Option<&'a str>,
    pub library: &'a str,
    /// Per-symbol DLL overrides recovered from the SDK import libraries.
    pub libraries: &'a HashMap<String, String>,
    pub ref_map: &'a HashMap<String, String>,
    /// Per-header mode: resolves token-only const casts whose type has no cursor.
    pub header_names: Option<&'a HashMap<String, String>>,
    pub tag_rename: &'a HashMap<String, String>,
    /// Enum reprs taken from integer typedefs in the C flags/enum idiom.
    pub enum_merge: &'a HashMap<String, &'static str>,
    pub tu: &'a TranslationUnit,
    pub pending_typedefs: Vec<Cursor>,
    pub pending_macros: Vec<String>,
    /// Per-header mode: incomplete pointer-only records emitted as opaque structs.
    pub pending_opaque: Vec<(String, String)>,
    /// Enum names for which `DEFINE_ENUM_FLAG_OPERATORS(X)` was seen.
    pub flag_enums: HashSet<String>,
    /// IID variables: interface name -> UUID, from `IID_XXX` GUID declarations.
    pub iid_vars: HashMap<String, String>,
    /// Object-like macro replacement tokens for resolving calling conventions.
    pub macro_defs: &'a HashMap<String, Vec<String>>,
    /// Expanded export name -> source spelling for object-like function aliases.
    /// Charset-selection aliases are excluded because they choose an `A`/`W` variant.
    pub alias_map: HashMap<String, String>,
    /// Non-empty means only listed functions are roots; dependencies still flow in later.
    pub symbols: &'a HashSet<String>,
    /// Drops functions with no resolved import library; off for fixtures without `.lib` inputs.
    pub drop_lib_less: bool,
    /// Resolution-winmd names that keep true WinRT ABI types out of the flat root.
    pub winrt_types: Option<&'a HashSet<String>>,
}

/// Per-namespace inputs for one emission pass over cached translation units.
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

    /// Applies the lib-less drop policy before inserting a function.
    fn insert_fn(&self, item: Fn, collector: &mut Collector) {
        if self.drop_lib_less && item.library.is_empty() {
            return;
        }
        collector.insert(Item::Fn(item));
    }

    /// Processes one cursor, inserting items or queuing macros for the second pass.
    fn process_cursor(
        &mut self,
        child: Cursor,
        collector: &mut Collector,
        extern_c: bool,
    ) -> Result<(), Error> {
        // Allowlist mode emits only named functions as roots. Bare tag dependencies are
        // not scheduled here; a missing one fails later as an unresolved reference.
        if !self.symbols.is_empty() {
            match child.kind() {
                CXCursor_FunctionDecl
                    if !child.is_definition()
                        && self.symbols.contains(&child.name())
                        && !is_midl_proxy_stub(&child, self.libraries) =>
                {
                    let item = Fn::parse(child, self, extern_c)?;
                    self.insert_fn(item, collector);
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
                let tag_name = child.name();
                let name = if is_anonymous_name(&tag_name) {
                    self.tag_rename
                        .get(&child.location_id())
                        .cloned()
                        .unwrap_or(tag_name)
                } else {
                    self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name)
                };
                // Numerics aliases collapse to shared value types; skip before lifting overlays.
                if numerics_alias(&name).is_some() {
                    return Ok(());
                }
                // Lift nested records first so field type references resolve.
                self.process_nested_types(child, collector, extern_c)?;
                // Inline anonymous records are emitted by their enclosing record.
                if child.is_anonymous_record() || is_named_instance_record(&child) {
                    return Ok(());
                }
                // No synthetic name means nothing can reference this anonymous type.
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
            // Pointer-only incomplete records need opaque structs; handle tags stay `*mut void`.
            CXCursor_StructDecl | CXCursor_UnionDecl
                if !child.is_definition() && !child.has_definition() =>
            {
                let tag_name = child.name();
                if !is_anonymous_name(&tag_name) && !tag_name.ends_with("__") {
                    let name = self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
                    // Do not clobber a real definition aliased by another tag.
                    if !self.ref_map.contains_key(&name) && !collector.contains_key(&name) {
                        if self.header_root.is_some() {
                            self.pending_opaque.push((String::new(), name));
                        } else {
                            collector.insert(Item::Struct(Struct::opaque(&name)));
                        }
                    }
                }
            }
            CXCursor_UnionDecl if child.is_definition() => {
                let tag_name = child.name();
                let name = if is_anonymous_name(&tag_name) {
                    self.tag_rename
                        .get(&child.location_id())
                        .cloned()
                        .unwrap_or(tag_name)
                } else {
                    self.tag_rename.get(&tag_name).cloned().unwrap_or(tag_name)
                };
                // Scalar overlay unions collapse to scalars; skip before lifting overlays.
                if semantic_scalar(&name).is_some() {
                    return Ok(());
                }
                // Lift nested records first so field type references resolve.
                self.process_nested_types(child, collector, extern_c)?;
                if child.is_anonymous_record() || is_named_instance_record(&child) {
                    return Ok(());
                }
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
            // Forward-declared `uuid` classes are COM server CLSIDs, not interface types.
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
                // Emit the public typedef alias, matching how references resolve the enum.
                if !is_anonymous_name(&e.name)
                    && let Some(alias) = self.tag_rename.get(&e.name)
                {
                    e.name.clone_from(alias);
                }
                if is_anonymous_name(&e.name) || is_midl_anonymous_enum_name(&e.name) {
                    // Nameless and MIDL-synthesized enums emit as loose constants.
                    for (name, value) in e.variants {
                        let const_value = enum_variant_value(e.repr, value);
                        collector.insert(Item::Const(Const {
                            name,
                            ty: None,
                            value: const_value,
                        }));
                    }
                } else if !self.ref_map.contains_key(&e.name) {
                    // The flag macro may have used the internal tag before the rename.
                    if self.flag_enums.contains(&e.name) || self.flag_enums.contains(&tag) {
                        e.flags = true;
                    }
                    // The flags/enum idiom gets its storage type from the integer typedef.
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
            // Skip MIDL marshaling thunks: RPC internals, not public API.
            CXCursor_FunctionDecl
                if !child.is_definition()
                    && !is_midl_proxy_stub(&child, self.libraries)
                    && !is_midl_user_marshal_stub(&child) =>
            {
                let item = Fn::parse(child, self, extern_c)?;
                self.insert_fn(item, collector);
            }
            // Linkage blocks may nest; recurse with the per-child language.
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
                    // Non-type keywords and string literals are not integer constants.
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
                    // Metadata has no 128-bit integer value, and clang would truncate it.
                    let body_has_int128_literal = tokens.iter().skip(1).any(|(kind, spelling)| {
                        *kind == CXToken_Literal && spelling.to_ascii_lowercase().ends_with("i128")
                    });
                    // Unbalanced replacement lists can swallow later synthetic enum entries.
                    let body_is_balanced = tokens_balanced(tokens.iter().skip(1));
                    if !body_has_non_type_keyword
                        && !body_has_string_literal
                        && !body_has_int128_literal
                        && body_is_balanced
                    {
                        // Defer object-like macro constants to the batch evaluator.
                        self.pending_macros.push(child.name());
                    }
                }
            }
            // `DEFINE_ENUM_FLAG_OPERATORS` marks an enum as `#[flags]`.
            CXCursor_MacroExpansion if child.name() == "DEFINE_ENUM_FLAG_OPERATORS" => {
                // Tokenize the invocation to extract the enum name argument.
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
                    // The macro may key on the internal tag; resolve to the emitted name.
                    let enum_name = self
                        .tag_rename
                        .get(&enum_name)
                        .cloned()
                        .unwrap_or(enum_name);
                    // Mark now if the enum was already inserted.
                    collector.mark_flags(&enum_name);
                    // Also record for enum definitions seen later.
                    self.flag_enums.insert(enum_name);
                }
            }
            // GUID macro values live in the arguments unless `INITGUID` is defined.
            CXCursor_MacroExpansion
                if matches!(child.name().as_str(), "DEFINE_GUID" | "DEFINE_OLEGUID") =>
            {
                let ole = child.name() == "DEFINE_OLEGUID";
                let tokens = self.tu.tokenize(child.extent());
                if let Some((name, uuid)) = parse_define_guid_tokens(&tokens, ole)
                    && !name.is_empty()
                {
                    // `IID_<Interface>` fills UUIDs missing from the C++ declaration.
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
            // Property key macro arguments carry the GUID plus PID value.
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
            // `IID_XXX` variables can provide UUIDs missing from interface declarations.
            CXCursor_VarDecl => {
                let name = child.name();
                if let Some(iface_name) = name.strip_prefix("IID_")
                    && is_guid_type(&child.ty())
                {
                    if let Some(uuid) = parse_guid_initializer_ast(&child) {
                        self.iid_vars.insert(iface_name.to_string(), uuid);
                    } else {
                        // Fallback when clang exposes no init-list children.
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

    /// Lifts nested records before their parent so field type references resolve.
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
                // Nested anonymous enum members leak into the enclosing C scope.
                let e = Enum::parse(nested)?;
                if is_anonymous_name(&e.name) || is_midl_anonymous_enum_name(&e.name) {
                    for (name, value) in e.variants {
                        let const_value = enum_variant_value(e.repr, value);
                        collector.insert(Item::Const(Const {
                            name,
                            ty: None,
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
    input: Vec<PathBuf>,
    input_text: Vec<String>,
    reference: Vec<PathBuf>,
    output: PathBuf,
    namespace: String,
    args: Vec<String>,
    library: String,
    /// Per-symbol DLL overrides recovered from SDK import libraries.
    libraries: HashMap<String, String>,
    filter: Vec<String>,
    target: Option<String>,
    /// Header directory segments treated as roots for the reachability sweep.
    scope: Vec<String>,
    /// Header stems treated as roots even outside the scoped SDK directories.
    scope_headers: HashSet<String>,
    /// Root header stems dropped before the reachability sweep.
    exclude_headers: HashSet<String>,
    /// Targeted function-symbol allowlist. Empty leaves emission unrestricted.
    symbols: HashSet<String>,
    /// Drops functions with no resolved import library; off for fixtures without `.lib` inputs.
    drop_lib_less: bool,
    /// Winmds used only to classify `ABI::Windows::*` projection declarations.
    resolution_input: Vec<PathBuf>,
    reference_default: bool,
    resolution_default: bool,
    reference_bytes: Vec<std::sync::Arc<[u8]>>,
    resolution_bytes: Vec<std::sync::Arc<[u8]>>,
}

/// Read-only inputs shared by every per-header pass.
#[derive(Clone, Copy)]
struct HeaderPass<'a> {
    /// Flat namespace root every partition emits into (`Windows.Win32`).
    root: &'a str,
    /// Resolution-winmd type-name membership for `ABI::Windows::*` declarations.
    winrt_types: &'a HashSet<String>,
}

fn remove_shadowed_opaque(collectors: &mut BTreeMap<String, Collector>) {
    let definitions: HashSet<String> = collectors
        .values()
        .flat_map(|collector| collector.iter())
        .filter_map(|(name, item)| match item {
            Item::Struct(item) if item.opaque => None,
            _ if item.is_type() => Some(name.clone()),
            _ => None,
        })
        .collect();

    for collector in collectors.values_mut() {
        collector.retain_items(|name, item| {
            !matches!(item, Item::Struct(item) if item.opaque && definitions.contains(name))
        });
    }
}

fn dedup_identical_items(
    collectors: &mut BTreeMap<String, Collector>,
    namespace: &str,
) -> Result<(), Error> {
    let mut signatures = HashMap::<String, HashSet<String>>::new();
    let mut duplicates = HashSet::<(String, String)>::new();

    // Package remapping processes sorted RDL files with last-wins routing. Keep the same owner
    // when removing duplicates so canonicalization does not move an API between header features.
    for (stem, collector) in collectors.iter().rev() {
        for (name, item) in collector.iter() {
            let signature = item.write(namespace)?.to_string();
            if !signatures
                .entry(name.clone())
                .or_default()
                .insert(signature)
            {
                duplicates.insert((stem.clone(), name.clone()));
            }
        }
    }

    for (stem, collector) in collectors {
        collector.retain_items(|name, _| !duplicates.contains(&(stem.clone(), name.to_string())));
    }

    Ok(())
}

fn remove_typedefs_shadowed_by_concrete_types(collectors: &mut BTreeMap<String, Collector>) {
    let concrete: HashSet<String> = collectors
        .values()
        .flat_map(|collector| collector.iter())
        .filter(|(_, item)| item.is_type() && !matches!(item, Item::Typedef(_)))
        .map(|(name, _)| name.clone())
        .collect();

    for collector in collectors.values_mut() {
        collector.retain_items(
            |name, item| !matches!(item, Item::Typedef(_) if concrete.contains(name)),
        );
    }
}

impl Clang {
    /// Creates a builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an input header (`.h`) file or directory.
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.input.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds input headers.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.input(input);
        }
        self
    }

    /// Adds inline source text to compile instead of a file on disk.
    pub fn input_text(&mut self, input: &str) -> &mut Self {
        self.input_text.push(input.to_string());
        self
    }

    /// Adds inline source texts to compile instead of files on disk.
    pub fn input_texts<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input_text(input.as_ref());
        }
        self
    }

    /// Adds a reference winmd file or directory.
    pub fn reference(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.reference.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds multiple reference winmd files or directories.
    pub fn references<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.reference(input);
        }
        self
    }

    /// Adds a reference winmd from memory.
    pub fn reference_bytes(&mut self, input: &[u8]) -> &mut Self {
        self.reference_bytes.push(input.into());
        self
    }

    /// Adds reference winmds from memory.
    pub fn reference_byte_sets<I, B>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        for input in inputs {
            self.reference_bytes(input.as_ref());
        }
        self
    }

    /// Adds the default Windows metadata as references.
    pub fn reference_default(&mut self) -> &mut Self {
        self.reference_default = true;
        self
    }

    /// Sets the output `.rdl` file path.
    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
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

    /// Drops functions with no resolved import library; leave off without `.lib` inputs.
    pub fn drop_lib_less(&mut self) -> &mut Self {
        self.drop_lib_less = true;
        self
    }

    /// Adds a winmd used only to classify `ABI::Windows::*` projection declarations.
    pub fn resolution_input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.resolution_input.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds winmds used only to classify `ABI::Windows::*` projection declarations.
    pub fn resolution_inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.resolution_input(input);
        }
        self
    }

    /// Adds a resolution-only winmd from memory.
    pub fn resolution_bytes(&mut self, input: &[u8]) -> &mut Self {
        self.resolution_bytes.push(input.into());
        self
    }

    /// Adds resolution-only winmds from memory.
    pub fn resolution_byte_sets<I, B>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        for input in inputs {
            self.resolution_bytes(input.as_ref());
        }
        self
    }

    /// Adds the default Windows Runtime metadata as a resolution-only input.
    pub fn resolution_default(&mut self) -> &mut Self {
        self.resolution_default = true;
        self
    }

    /// Adds symbol -> DLL overrides for functions.
    ///
    /// Prefer per-DLL `.lib` files over umbrella/apiset libraries for real DLL names.
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

    /// Returns the DLL currently mapped to a function symbol.
    pub fn resolved_library(&self, symbol: &str) -> Option<&str> {
        self.libraries.get(symbol).map(String::as_str)
    }

    /// Reads a COFF import library and adds its symbol -> DLL mappings.
    pub fn import_library(&mut self, path: impl AsRef<Path>) -> Result<&mut Self, Error> {
        extend_libraries(&mut self.libraries, path.as_ref())?;
        Ok(self)
    }

    /// Adds a normalized header path suffix to the inclusion filter.
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

    /// Adds a compiler argument to pass to libclang.
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

    /// Sets the target triple used for all clang invocations.
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }

    /// Adds a header directory segment that acts as a root for the reachability sweep.
    pub fn scope(&mut self, scope: &str) -> &mut Self {
        self.scope.push(scope.to_string());
        self
    }

    /// Adds multiple header directory segments as roots for the reachability sweep.
    pub fn scopes<I, S>(&mut self, scopes: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for scope in scopes {
            self.scope(scope.as_ref());
        }
        self
    }

    /// Marks a header as a sweep root regardless of SDK directory.
    pub fn scope_header(&mut self, header: &str) -> &mut Self {
        let stem = header_stem_to_namespace(header);
        if !stem.is_empty() {
            self.scope_headers.insert(stem);
        }
        self
    }

    /// Marks multiple headers as sweep roots regardless of SDK directory.
    pub fn scope_headers<I, S>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for header in headers {
            self.scope_header(header.as_ref());
        }
        self
    }

    /// Drops a named header partition before the reachability sweep.
    pub fn exclude_header(&mut self, header: &str) -> &mut Self {
        let stem = header_stem_to_namespace(header);
        if !stem.is_empty() {
            self.exclude_headers.insert(stem);
        }
        self
    }

    /// Drops multiple named header partitions before the reachability sweep.
    pub fn exclude_headers<I, S>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for header in headers {
            self.exclude_header(header.as_ref());
        }
        self
    }

    /// Restricts root emission to a named function symbol.
    pub fn symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbols.insert(symbol.to_string());
        self
    }

    /// Restricts root emission to the named function symbols.
    pub fn symbols<I, S>(&mut self, symbols: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for symbol in symbols {
            self.symbol(symbol.as_ref());
        }
        self
    }

    /// Returns the version string reported by the loaded libclang.
    pub fn version() -> Result<String, Error> {
        let lib = Library::new()?;
        Ok(lib.version())
    }

    /// Generates the RDL and writes it to the configured output.
    pub fn write(&self) -> Result<(), Error> {
        self.validate_output()?;
        let reference = self.load_reference()?;
        let spec = NamespaceSpec {
            namespace: &self.namespace,
            library: &self.library,
            libraries: &self.libraries,
            filter: &self.filter,
            symbols: &self.symbols,
        };
        let rdl = self.parse_and_emit(&reference, std::slice::from_ref(&spec))?;
        write_to_file(&self.output, formatter::format(&rdl[0])?)?;
        Ok(())
    }

    /// Writes one flat-root RDL file per defining header.
    pub fn write_by_header(&self) -> Result<(), Error> {
        self.validate_output()?;
        let outputs = self.parse_and_emit_by_header(&self.namespace)?;
        for (stem, rdl) in outputs {
            // File names are lowercased defining-header stems.
            let leaf = stem.to_lowercase();
            write_to_file(
                self.output.join(format!("{leaf}.rdl")),
                formatter::format(&rdl)?,
            )?;
        }
        Ok(())
    }

    fn validate_output(&self) -> Result<(), Error> {
        if self.output.as_os_str().is_empty() {
            Err(Error::new("output is required", "", 0, 0))
        } else {
            Ok(())
        }
    }

    /// Parses inputs once and returns the libclang state that keeps the TUs valid.
    fn parse_inputs(&self) -> Result<ParsedInputs, Error> {
        let h_paths = expand_input_files(&self.input, "h")?;
        let library = Library::new()?;
        let index = Index::new()?;

        // Put `--target=` before user args.
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
            let source = input.to_str().ok_or_else(|| {
                Error::new(
                    "input path is not valid UTF-8",
                    &input.to_string_lossy(),
                    0,
                    0,
                )
            })?;
            h_tus.push((source.replace('\\', "/"), index.parse(source, &arg_refs)?));
        }
        let mut str_tus = vec![];
        for content in &self.input_text {
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

    /// Emits one flat-root RDL string per defining-header stem.
    fn parse_and_emit_by_header(&self, root: &str) -> Result<BTreeMap<String, String>, Error> {
        // Additive scrapes skip entities already defined by input winmds. Split type and
        // value names because functions/constants live on `Apis`, not in `iter()`.
        let reference = self.load_reference()?;
        let mut exclude_types: HashSet<String> = HashSet::new();
        let mut exclude_values: HashSet<String> = HashSet::new();
        // Reference enums the scrape may carry in full: a reference (`um`) header can truncate
        // an enum (for example `winternl.h` cuts `FILE_INFORMATION_CLASS` to one member) while
        // the scraped (`km`) headers define it completely. Record each reference enum's member
        // set so an enum the scrape extends can be un-excluded below and emitted in full; the
        // winmd merge then unions the truncated reference copy with this complete one.
        let mut reference_enums: HashMap<String, HashSet<String>> = HashMap::new();
        for (_, name, item) in reference.iter_items() {
            match item {
                metadata::reader::Item::Type(def) => {
                    if def.category() == metadata::reader::TypeCategory::Enum {
                        reference_enums.insert(
                            name.to_string(),
                            def.fields()
                                .filter(|field| field.constant().is_some())
                                .map(|field| field.name().to_string())
                                .collect(),
                        );
                    }
                    exclude_types.insert(name.to_string())
                }
                metadata::reader::Item::Fn(_) | metadata::reader::Item::Const(_) => {
                    exclude_values.insert(name.to_string())
                }
            };
        }

        let parsed = self.parse_inputs()?;
        let arg_refs: Vec<&str> = parsed.args.iter().map(String::as_str).collect();

        // Backtick-stripped resolution names classify `ABI::Windows::*` declarations.
        let winrt_types = self.load_winrt_types()?;

        let mut collectors: BTreeMap<String, Collector> = BTreeMap::new();
        // Per-partition root flag for the reachability sweep.
        let mut scope_in: BTreeMap<String, bool> = BTreeMap::new();

        let pass = HeaderPass {
            root,
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

        remove_shadowed_opaque(&mut collectors);

        // Drop excluded root partitions before the sweep.
        if !self.exclude_headers.is_empty() {
            collectors.retain(|stem, _| !self.exclude_headers.contains(stem));
            scope_in.retain(|stem, _| !self.exclude_headers.contains(stem));
        }

        // Keep out-of-scope declarations only when referenced from an in-scope root.
        if !self.scope.is_empty() {
            sweep_unreferenced(&mut collectors, &scope_in);
        }

        // Un-exclude a reference enum the scrape carries with members the reference lacks: emit
        // the complete enum so the winmd merge can union it with the truncated reference copy
        // into a single enum. An enum the scrape does not extend stays excluded (the reference
        // copy already covers it).
        if !reference_enums.is_empty() {
            let mut keep: HashSet<String> = HashSet::new();
            for collector in collectors.values() {
                for item in collector.values() {
                    let Item::Enum(e) = item else {
                        continue;
                    };
                    let Some(members) = reference_enums.get(&e.name) else {
                        continue;
                    };
                    if e.variants
                        .iter()
                        .any(|(member, _)| !members.contains(member))
                    {
                        keep.insert(e.name.clone());
                    }
                }
            }
            for name in &keep {
                exclude_types.remove(name);
            }
        }

        // Exclude same-category names already in the reference winmd; cross-category clashes
        // may still be real dependencies and are handled below.
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

        // Drop unreferenced WDK types that collide with Win32 values in the flat root. A
        // referenced cross-kind clash stays so its typedefs do not dangle.
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

        // Drop `IID_<Interface>` constants when the interface in this scrape already carries
        // the GUID; bindgen synthesizes the same constant from the interface GUID.
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

        // Drop unreferenced loose constants that duplicate enum members by name and value.
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

        remove_typedefs_shadowed_by_concrete_types(&mut collectors);
        dedup_identical_items(&mut collectors, root)?;

        // Choose duplicate typedef owners only after every partition and item filter has run.
        dedup_typedefs(&mut collectors);

        let mut outputs = BTreeMap::new();
        for (stem, collector) in &collectors {
            // Empty partitions are not written.
            if collector.is_empty() {
                continue;
            }
            // Every file emits the same flat root; the stem only names the file.
            outputs.insert(stem.clone(), emit_module(root, collector)?);
        }
        Ok(outputs)
    }

    /// Routes top-level declarations to collectors keyed by defining-header stem.
    fn process_tu_by_header(
        &self,
        tu: &TranslationUnit,
        pass: &HeaderPass<'_>,
        collectors: &mut BTreeMap<String, Collector>,
        scope_in: &mut BTreeMap<String, bool>,
        eval: MacroEval<'_>,
    ) -> Result<(), Error> {
        let HeaderPass { root, winrt_types } = *pass;
        // Abort on diagnostics in emitted headers; tolerate transitive-only include errors
        // so interop headers can survive broken C++/WinRT projection includes.
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
        // Share TU-wide macro definitions across per-header parsers.
        let macro_defs = collect_macro_defs(tu);

        // Flatten linkage blocks and deduplicate by clang identity across repeated SDK
        // declarations; the defining header only selects the output file.
        let mut decls = Vec::new();
        // A resolution winmd lets the ABI namespace walker separate WinRT types from COM interop.
        let abi = (!winrt_types.is_empty()).then_some(winrt_types);
        flatten_decls(tu.cursor(), false, false, None, abi, &mut decls);

        // Prefer definitions over forward declarations so records route to defining headers.
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
                    // Among forward declarations, keep the `uuid` one so CLSIDs survive.
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

        let mut buckets: BTreeMap<String, Vec<(Cursor, bool)>> = BTreeMap::new();
        for (_, (child, extern_c)) in chosen {
            let stem = header_stem_of(&child).expect("filtered above");
            // Keep a partition in-scope if any contributing cursor is in-scope.
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
        // Macro constants are per-bucket values but are deduplicated globally.
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

        // Flat enums contribute member names too, since those emit as top-level constants.
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

        // Evaluate buckets in parallel but merge in stable order so first owner wins.
        let evaluated = evaluate_macros_parallel(&all_consts, eval.source, eval.args)?;
        for ((stem, _pending), consts) in all_consts.into_iter().zip(evaluated) {
            let collector = collectors.entry(stem).or_default();
            for c in consts {
                if global_names.insert(c.name.clone()) {
                    collector.insert(Item::Const(c));
                }
            }
        }

        // Emit opaque placeholders only when no real definition won globally.
        for (stem, name) in all_opaque {
            if global_names.insert(name.clone()) {
                let collector = collectors.entry(stem).or_default();
                collector.insert(Item::Struct(Struct::opaque(&name)));
            }
        }

        Ok(())
    }

    /// Loads `.winmd` reference inputs for cross-namespace resolution.
    fn load_reference(&self) -> Result<metadata::reader::Index, Error> {
        let winmd_paths = expand_input_files(&self.reference, "winmd")?;

        let mut winmd_files = vec![];
        for file_name in &winmd_paths {
            let source = file_name.to_string_lossy();
            winmd_files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid reference", &source, 0, 0))?,
            );
        }
        if self.reference_default {
            winmd_files.extend(
                [windows_default::WINRT, windows_default::WIN32]
                    .into_iter()
                    .map(|bytes| metadata::reader::File::new(bytes.to_vec()).unwrap()),
            );
        }
        for bytes in &self.reference_bytes {
            winmd_files.push(
                metadata::reader::File::new(bytes.to_vec())
                    .ok_or_else(|| Error::new("invalid reference", "<memory>", 0, 0))?,
            );
        }

        Ok(metadata::reader::Index::new(winmd_files))
    }

    /// Loads resolution-winmd type names, stripping generic arity for C++ ABI matching.
    fn load_winrt_types(&self) -> Result<HashSet<String>, Error> {
        let mut winmd_files = vec![];
        for file_name in &self.resolution_input {
            let source = file_name.to_string_lossy();
            winmd_files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid resolution input", &source, 0, 0))?,
            );
        }
        if self.resolution_default {
            winmd_files.push(metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap());
        }
        for bytes in &self.resolution_bytes {
            winmd_files.push(
                metadata::reader::File::new(bytes.to_vec())
                    .ok_or_else(|| Error::new("invalid resolution input", "<memory>", 0, 0))?,
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

    /// Emits one RDL string per namespace spec, reusing cached translation units.
    fn parse_and_emit(
        &self,
        reference: &metadata::reader::Index,
        specs: &[NamespaceSpec<'_>],
    ) -> Result<Vec<String>, Error> {
        // Reuse translation units across all specs.
        let parsed = self.parse_inputs()?;
        let arg_refs: Vec<&str> = parsed.args.iter().map(String::as_str).collect();

        // Pass 1: learn unique type-name owners across specs. Shared typedef artifacts stay
        // local by being dropped from the owner table.
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

        // Pass 2: emit with in-house owners preferred over the upstream reference.
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

    /// Processes one translation unit and returns macros needing batch evaluation.
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

        // Map internal tags to their public typedef aliases.
        let mut tag_rename = build_tag_rename_map(tu);

        // Give nested records synthetic names keyed by tag or source location.
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
            // Process main-file cursors plus headers matched by this spec.
            if !child.is_from_main_file() {
                let passes_filter = !spec.filter.is_empty() && {
                    let file = child.file_name();
                    spec.filter.iter().any(|f| matches_filter(&file, f))
                };
                if !passes_filter {
                    // Linkage macros often spell in helper headers; filter by expansion too.
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

        // Drain referenced typedef dependencies; parsing them can enqueue more.
        let mut seen: HashSet<String> = HashSet::new();
        let mut i = 0;
        while i < parser.pending_typedefs.len() {
            let cursor = parser.pending_typedefs[i];
            i += 1;
            let name = cursor.name();
            // Skip anything already resolved.
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

        // Apply `IID_IFoo` variables to interfaces that lack `uuid` attributes.
        collector.apply_iid_vars(&parser.iid_vars);

        Ok(parser.pending_macros)
    }
}

/// Owns libclang state; field order ensures TUs drop before the library unloads.
struct ParsedInputs {
    args: Vec<String>,
    h_tus: Vec<(String, TranslationUnit)>,
    str_tus: Vec<(String, TranslationUnit)>,
    index: Index,
    _library: Library,
}

/// Keep one stable definition when separate headers repeat an equivalent typedef.
///
/// The SDK may declare the same public alias through different but compatible spellings, such as
/// `PUNICODE_STRING` through `UNICODE_STRING` and its `LSA_UNICODE_STRING` base. Winmd cannot
/// represent both rows under one flat name, so a direct `PFOO -> FOO*` alias wins, then the first
/// surviving defining-header partition.
fn dedup_typedefs(collectors: &mut BTreeMap<String, Collector>) {
    let mut owners: HashMap<String, (String, bool)> = HashMap::new();
    for (stem, collector) in collectors.iter() {
        for (name, item) in collector.iter() {
            let Item::Typedef(ty) = item else {
                continue;
            };
            let direct = ty.is_direct_pointer_alias();
            match owners.entry(name.clone()) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert((stem.clone(), direct));
                }
                std::collections::hash_map::Entry::Occupied(mut entry)
                    if direct && !entry.get().1 =>
                {
                    entry.insert((stem.clone(), true));
                }
                _ => {}
            }
        }
    }
    for (stem, collector) in collectors.iter_mut() {
        collector.retain_items(|name, item| {
            !matches!(item, Item::Typedef(_))
                || owners.get(name).is_some_and(|(owner, _)| owner == stem)
        });
    }
}

/// Flattens linkage blocks and, when configured, descends into `ABI::Windows::*`.
///
/// Resolution-winmd membership separates true WinRT ABI projections from Win32 COM interop
/// declarations that live in the same C++ namespace.
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
                // Accumulate the ABI namespace path below `ABI`.
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
                // Strip `ABI` itself from the namespace path.
                flatten_decls(child, in_linkage, in_interop_ns, Some(""), winrt_types, out);
            } else if in_interop_ns || child.name() == "Windows" {
                // Hand-authored `Windows::*` C++ interop declarations route to the flat root.
                flatten_decls(child, in_linkage, true, None, winrt_types, out);
            }
        } else if let (Some(path), Some(set)) = (abi_ns, winrt_types) {
            // Capture ABI declarations absent from the resolution winmd; skip open templates.
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

/// True for dummy handle tags (`X__` or MIDL placeholders) that emit as `*mut void`.
/// Real MIDL value structs using the same suffix have payload shape and are kept.
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

/// MIDL per-method proxy/stub thunks are RPC plumbing unless a real import library exports them.
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

/// MIDL `_User*` wire-marshaling helpers are generated RPC internals, not public API.
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

/// Collects enum member values across partitions for duplicate loose-constant pruning.
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

/// Converts integer metadata values to `i128` for enum-member duplicate checks.
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

/// Matches high-bit flags that clang sign-extends on the enum side.
fn enum_member_eq(member: i64, constant: i128) -> bool {
    member as i128 == constant || member as u32 as i128 == constant
}

/// Emits a collector under the nested `mod` path for `namespace`.
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

/// Converts an enum value to the metadata value matching its repr.
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
        // Anonymous nested members carry their refs in `nested`, not `ty`.
        let nested = Struct {
            name: "Anonymous".to_string(),
            fields: vec![Field {
                name: "value".to_string(),
                ty: metadata::Type::ValueName(metadata::TypeName::named("", "InnerType")),
                nested: None,
                bitfields: vec![],
            }],
            is_union: true,
            opaque: false,
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
        // High-bit signed enum flags can match unsigned macro constants.
        assert!(enum_member_eq(-2147483648, 0x8000_0000));
        // Wide constants do not match by low 32 bits alone.
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
            ty: None,
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
