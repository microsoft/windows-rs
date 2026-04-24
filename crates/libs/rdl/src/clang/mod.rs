#![allow(non_upper_case_globals)]

use super::*;
mod cx;
use cx::*;
use windows_metadata as metadata;
mod r#enum;
use r#enum::*;
mod item;
use item::*;
mod r#struct;
use r#struct::*;
mod collector;
use collector::*;
use field::*;
mod field;
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

/// Shared parse context that is threaded through all `parse` methods in the
/// clang module, eliminating the need to pass a fixed set of parameters
/// (`namespace`, `library`, `ref_map`, `tag_rename`, `tu`) individually to
/// every call.
///
/// `pending_typedefs` and `pending_macros` accumulate side-effects during the
/// main AST walk and are consumed by [`Clang::process_tu`] after the walk
/// completes.
pub struct Parser<'a> {
    pub namespace: &'a str,
    pub library: &'a str,
    pub ref_map: &'a HashMap<String, String>,
    pub tag_rename: HashMap<String, String>,
    pub tu: &'a TranslationUnit,
    pub pending_typedefs: Vec<Cursor>,
    pub pending_macros: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(
        namespace: &'a str,
        library: &'a str,
        ref_map: &'a HashMap<String, String>,
        tag_rename: HashMap<String, String>,
        tu: &'a TranslationUnit,
    ) -> Self {
        Self {
            namespace,
            library,
            ref_map,
            tag_rename,
            tu,
            pending_typedefs: vec![],
            pending_macros: vec![],
        }
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
        match child.kind() {
            CXCursor_StructDecl if child.is_definition() => {
                // Recursively lift any named or anonymous nested struct/union
                // declarations to the collector before processing the outer struct
                // so that field type references to those nested types are already
                // registered.
                self.process_nested_types(child, collector, extern_c)?;
                let tag_name = child.name();
                // Resolve the effective public name via the tag→typedef rename map.
                // For anonymous types the spelling is empty; use location_id instead.
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
                } else if child.has_pure_virtual_methods() || child.extract_uuid(self.tu).is_some()
                {
                    if !self.ref_map.contains_key(&name) {
                        collector.insert(Item::Interface(Interface::parse(child, self)?));
                    }
                } else if !self.ref_map.contains_key(&name) {
                    collector.insert(Item::Struct(Struct::parse(child, self, false)?));
                }
            }
            CXCursor_UnionDecl if child.is_definition() => {
                // Recursively lift any named or anonymous nested struct/union
                // declarations to the collector before processing the outer union.
                self.process_nested_types(child, collector, extern_c)?;
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
                        || child.extract_uuid(self.tu).is_some()) =>
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
                let e = Enum::parse(child)?;
                if is_anonymous_name(&e.name) || is_midl_anonymous_enum_name(&e.name) {
                    // Unnamed enums (e.g. `enum { ONE = 1, TWO };`) are
                    // reported by libclang with a synthesised spelling like
                    // "(unnamed enum at file.h:6:1)" which always starts
                    // with '('.  MIDL also synthesises names of the form
                    // `__MIDL___MIDL_itf_<...>` for originally anonymous
                    // enumerations from IDL.  In both cases each variant is
                    // emitted as a top-level RDL constant rather than a
                    // named enum type.
                    for (name, value) in e.variants {
                        let const_value = enum_variant_value(e.repr, value);
                        collector.insert(Item::Const(Const {
                            name,
                            value: const_value,
                        }));
                    }
                } else if !self.ref_map.contains_key(&e.name) {
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
                collector.insert(Item::Fn(Fn::parse(child, self, extern_c)?));
            }
            // An `extern "C" { }` or `extern "C++" { }` block — encountered
            // either at the top level (e.g. MIDL-generated headers wrap all
            // declarations in such a block) or nested inside another one
            // (e.g. `#define EXTERN_C extern "C"` expanded inside an outer
            // `extern "C" { }` block).  Recurse so that every declaration
            // inside is processed with the correct linkage.
            // NOTE: Items from transitively included headers that slip through
            // will be filtered out by the ref_map check in the individual arms.
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
                    // Skip macros whose body contains keyword tokens.  Such
                    // macros are language constructs (e.g.
                    // `#define EXTERN_C extern "C"`) and cannot be integer
                    // constant expressions; adding them to pending_macros
                    // causes the evaluator to emit bogus zero constants.
                    // The first token is always the macro name itself; skip
                    // it to examine only the replacement-list body tokens.
                    let tokens = self.tu.tokenize(child.extent());
                    let body_has_keyword = tokens
                        .iter()
                        .skip(1) // first token is the macro name
                        .any(|(kind, _)| *kind == CXToken_Keyword);
                    if !body_has_keyword {
                        // The token parser returned None for a candidate
                        // object-like macro.  Defer to the batch evaluator.
                        self.pending_macros.push(child.name());
                    }
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
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Clang {
    input: Vec<String>,
    input_str: Vec<String>,
    output: String,
    namespace: String,
    args: Vec<String>,
    library: String,
    filter: Vec<String>,
}

impl Clang {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

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

    pub fn input_str(&mut self, input: &str) -> &mut Self {
        self.input_str.push(input.to_string());
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = namespace.to_string();
        self
    }

    pub fn library(&mut self, library: &str) -> &mut Self {
        self.library = library.to_string();
        self
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

    /// Returns the version string reported by the loaded libclang, e.g.
    /// `"clang version 18.1.0 (...)"`.  Loads libclang on first call.
    pub fn version() -> Result<String, Error> {
        let lib = Library::new()?;
        Ok(lib.version())
    }

    pub fn write(&self) -> Result<(), Error> {
        let (h_paths, winmd_paths) = expand_input_paths(&self.input, "h", "winmd")?;

        let mut winmd_files = vec![];

        for file_name in &winmd_paths {
            winmd_files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }

        let reference = metadata::reader::TypeIndex::new(winmd_files);

        let mut ref_map: HashMap<String, String> = HashMap::new();
        for (namespace, name, _) in reference.iter() {
            ref_map.insert(name.to_string(), namespace.to_string());
        }

        let _library = Library::new()?;
        let index = Index::new()?;
        let mut collector = Collector::new();
        let args: Vec<_> = self.args.iter().map(String::as_str).collect();

        for input in &h_paths {
            let tu = index.parse(input, &args)?;
            let pending = self.process_tu(&tu, &mut collector, &ref_map)?;
            for c in Const::evaluate_macros(input, &pending, &index, &args)? {
                collector.insert(Item::Const(c));
            }
        }

        for content in &self.input_str {
            let tu = index.parse_unsaved(
                ".h",
                content,
                &args,
                CXTranslationUnit_DetailedPreprocessingRecord,
            )?;
            let pending = self.process_tu(&tu, &mut collector, &ref_map)?;
            for c in Const::evaluate_macros_str(content, &pending, &index, &args)? {
                collector.insert(Item::Const(c));
            }
        }

        let namespace: Vec<&str> = self.namespace.split('.').collect();
        let mut output = format!("#[win32] mod {} {{", namespace[0]);

        for namespace in &namespace[1..] {
            output.push_str(&format!("mod {} {{", namespace));
        }

        for item in collector.values() {
            output.push_str(&item.write(&self.namespace)?.to_string());
        }

        for _ in 0..namespace.len() {
            output.push('}');
        }

        write_to_file(&self.output, formatter::format(&output))?;

        Ok(())
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
    ) -> Result<Vec<String>, Error> {
        for diag in tu.diagnostics() {
            if diag.is_err() {
                return Err(Error::new(
                    &diag.message,
                    &diag.file_name,
                    diag.line.try_into().unwrap(),
                    (diag.column - 1).try_into().unwrap(),
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

        let mut parser = Parser::new(&self.namespace, &self.library, ref_map, tag_rename, tu);

        for child in tu.cursor().children() {
            // Only process cursors from the main input file or from headers
            // that match the caller-supplied path-suffix filters.
            if !child.is_from_main_file() {
                // Check whether this cursor's source file matches any filter.
                let passes_filter = !self.filter.is_empty() && {
                    let file = child.file_name();
                    self.filter.iter().any(|f| matches_filter(&file, f))
                };
                if !passes_filter {
                    // A CXCursor_LinkageSpec produced by a macro such as
                    // `#define EXTERN_C extern "C"` (defined in an included
                    // header) has its spelling location inside the macro body in
                    // the included header, so is_from_main_file() returns false.
                    // Accept it anyway when the *expansion* location (where the
                    // macro was invoked) is in the main file.
                    if child.kind() != CXCursor_LinkageSpec
                        || !child.is_expansion_from_main_file(tu)
                    {
                        continue;
                    }
                }
            }

            parser.process_cursor(child, collector, false)?;
        }

        // Emit typedef/callback items for any types that were referenced by
        // main-file items but are defined only in included/system headers.
        // The vec may grow during this loop as transitive typedef dependencies
        // are discovered (e.g. `typedef BYTE MY_BYTE` pulls in `BYTE` too).
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

        Ok(parser.pending_macros)
    }
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
        if !tag_name.is_empty() && typedef_name != tag_name {
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
    for child in tu.cursor().children() {
        if child.kind() == CXCursor_LinkageSpec {
            for inner in child.children() {
                visit_for_nested_names(inner, tag_rename);
            }
        } else {
            visit_for_nested_names(child, tag_rename);
        }
    }
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
fn matches_filter(file: &str, filter: &str) -> bool {
    if filter.is_empty() {
        return false;
    }
    let file = file.replace('\\', "/");
    let filter = filter.replace('\\', "/");
    file.ends_with(filter.as_str())
        && (file.len() == filter.len() || file.as_bytes()[file.len() - filter.len() - 1] == b'/')
}
