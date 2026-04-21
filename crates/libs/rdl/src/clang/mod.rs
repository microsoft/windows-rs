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

#[derive(Default)]
pub struct Clang {
    input: Vec<String>,
    input_str: Vec<String>,
    output: String,
    namespace: String,
    args: Vec<String>,
    library: String,
}

impl Clang {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
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
        let tag_rename = build_tag_rename_map(tu);

        // Macros that the token-based parser cannot handle (complex
        // expressions, references to other macros, arithmetic, etc.) are
        // collected here and evaluated in a second pass via the
        // synthetic-enum technique.
        let mut pending_macros: Vec<String> = vec![];

        // Typedef cursors from included/system headers that are referenced by
        // main-file items but not yet collected.  Processed after the main
        // walk; may grow during that processing pass as transitive typedef
        // dependencies are discovered.
        let mut pending_typedefs: Vec<Cursor> = vec![];

        for child in tu.cursor().children() {
            // Only process cursors from the main input file (not from
            // transitively included headers).
            if !child.is_from_main_file() {
                continue;
            }

            // Recurse into `extern "C" { }` or `extern "C++" { }` blocks.
            // MIDL-generated headers place all declarations (structs, enums,
            // typedefs, functions, …) inside such a block, so we must handle
            // every item kind here, not just function declarations.
            if child.kind() == CXCursor_LinkageSpec {
                for inner in child.children() {
                    if !inner.is_from_main_file() {
                        continue;
                    }
                    // A function inside `extern "C" { }` has C language linkage.
                    let extern_c = inner.language() == CXLanguage_C;
                    self.process_cursor(
                        inner,
                        collector,
                        ref_map,
                        &tag_rename,
                        tu,
                        &mut pending_macros,
                        &mut pending_typedefs,
                        extern_c,
                    )?;
                }
            } else {
                self.process_cursor(
                    child,
                    collector,
                    ref_map,
                    &tag_rename,
                    tu,
                    &mut pending_macros,
                    &mut pending_typedefs,
                    false,
                )?;
            }
        }

        // Emit typedef/callback items for any types that were referenced by
        // main-file items but are defined only in included/system headers.
        // The vec may grow during this loop as transitive typedef dependencies
        // are discovered (e.g. `typedef BYTE MY_BYTE` pulls in `BYTE` too).
        let mut seen: HashSet<String> = HashSet::new();
        let mut i = 0;
        while i < pending_typedefs.len() {
            let cursor = pending_typedefs[i];
            i += 1;
            let name = cursor.name();
            // Skip if already processed, already collected, or in the reference.
            if !seen.insert(name.clone())
                || collector.contains_key(&name)
                || ref_map.contains_key(&name)
            {
                continue;
            }
            if let Some(cb) = Callback::parse(
                cursor,
                &self.namespace,
                ref_map,
                &tag_rename,
                &mut pending_typedefs,
            )? {
                collector.insert(Item::Callback(cb));
            } else if let Some(td) = Typedef::parse(
                cursor,
                &self.namespace,
                ref_map,
                &tag_rename,
                &mut pending_typedefs,
            )? {
                collector.insert(Item::Typedef(td));
            }
        }

        Ok(pending_macros)
    }

    /// Process a single cursor: insert the corresponding [`Item`] into
    /// `collector` or record the name in `pending_macros` for the second-pass
    /// evaluator.  `extern_c` is `true` when the cursor was found inside an
    /// `extern "C" { }` block (relevant only for function declarations).
    #[allow(clippy::too_many_arguments)]
    fn process_cursor(
        &self,
        child: Cursor,
        collector: &mut Collector,
        ref_map: &HashMap<String, String>,
        tag_rename: &HashMap<String, String>,
        tu: &TranslationUnit,
        pending_macros: &mut Vec<String>,
        pending_typedefs: &mut Vec<Cursor>,
        extern_c: bool,
    ) -> Result<(), Error> {
        match child.kind() {
            CXCursor_StructDecl if child.is_definition() => {
                let tag_name = child.name();
                // Resolve the effective public name via the tag→typedef rename map.
                let name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
                if child.has_pure_virtual_methods() {
                    if !ref_map.contains_key(&name) {
                        collector.insert(Item::Interface(Interface::parse(
                            child,
                            &self.namespace,
                            tu,
                            ref_map,
                            tag_rename,
                            pending_typedefs,
                        )?));
                    }
                } else if !ref_map.contains_key(&name) {
                    collector.insert(Item::Struct(Struct::parse(
                        child,
                        &self.namespace,
                        ref_map,
                        tag_rename,
                        pending_typedefs,
                        false,
                    )?));
                }
            }
            CXCursor_UnionDecl if child.is_definition() => {
                let name = child.name();
                if !ref_map.contains_key(&name) {
                    collector.insert(Item::Struct(Struct::parse(
                        child,
                        &self.namespace,
                        ref_map,
                        tag_rename,
                        pending_typedefs,
                        true,
                    )?));
                }
            }
            CXCursor_ClassDecl if child.is_definition() && child.has_pure_virtual_methods() => {
                let tag_name = child.name();
                let name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
                if !ref_map.contains_key(&name) {
                    collector.insert(Item::Interface(Interface::parse(
                        child,
                        &self.namespace,
                        tu,
                        ref_map,
                        tag_rename,
                        pending_typedefs,
                    )?));
                }
            }
            CXCursor_EnumDecl if child.is_definition() => {
                let e = Enum::parse(child)?;
                if e.name.is_empty() || e.name.starts_with('(') {
                    // Unnamed enums (e.g. `enum { ONE = 1, TWO };`) are
                    // reported by libclang with a synthesised spelling like
                    // "(unnamed enum at file.h:6:1)" which always starts
                    // with '('.  Each variant is emitted as a top-level
                    // RDL constant rather than a named enum type.
                    for (name, value) in e.variants {
                        let const_value = enum_variant_value(e.repr, value);
                        collector.insert(Item::Const(Const {
                            name,
                            value: const_value,
                        }));
                    }
                } else if !ref_map.contains_key(&e.name) {
                    collector.insert(Item::Enum(e));
                }
            }
            CXCursor_TypedefDecl if child.is_definition() => {
                let name = child.name();
                if !ref_map.contains_key(&name) {
                    if let Some(cb) = Callback::parse(
                        child,
                        &self.namespace,
                        ref_map,
                        tag_rename,
                        pending_typedefs,
                    )? {
                        collector.insert(Item::Callback(cb));
                    } else if let Some(td) = Typedef::parse(
                        child,
                        &self.namespace,
                        ref_map,
                        tag_rename,
                        pending_typedefs,
                    )? {
                        collector.insert(Item::Typedef(td));
                    }
                }
            }
            CXCursor_FunctionDecl if !child.is_definition() => {
                collector.insert(Item::Fn(Fn::parse(
                    child,
                    &self.namespace,
                    &self.library,
                    extern_c,
                    ref_map,
                    tag_rename,
                    pending_typedefs,
                )?));
            }
            CXCursor_MacroDefinition => {
                if let Some(c) = Const::parse(child, &self.namespace, tu, ref_map)? {
                    collector.insert(Item::Const(c));
                } else if !child.is_macro_builtin()
                    && !child.is_macro_function_like()
                    && !child.name().is_empty()
                    && !child.name().starts_with('_')
                {
                    // The token parser returned None for a candidate
                    // object-like macro.  Defer to the batch evaluator.
                    pending_macros.push(child.name());
                }
            }
            _ => {}
        }
        Ok(())
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
