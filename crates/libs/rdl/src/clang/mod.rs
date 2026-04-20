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
mod param;
use param::*;
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

        // Build a map from type name to namespace for all types in the reference.
        // This is used to suppress locally-defined types that already exist in
        // the reference metadata and to emit qualified names when referencing them.
        // Only types whose short name is unique across all reference namespaces are
        // included; ambiguous names (same short name in multiple namespaces) are
        // excluded so that unqualified C/C++ references are not silently mapped to
        // a potentially wrong namespace.
        let mut name_count: HashMap<String, usize> = HashMap::new();
        for (_, name, _) in reference.iter() {
            *name_count.entry(name.to_string()).or_insert(0) += 1;
        }
        let mut ref_map: HashMap<String, String> = HashMap::new();
        for (namespace, name, _) in reference.iter() {
            if name_count.get(name).copied().unwrap_or(0) == 1 {
                ref_map.insert(name.to_string(), namespace.to_string());
            }
        }

        let _library = Library::new()?;
        let index = Index::new()?;
        let mut collector = Collector::new();
        let args: Vec<_> = self.args.iter().map(String::as_str).collect();

        for input in &h_paths {
            let tu = index.parse(input, &args)?;
            let pending = self.process_tu(&tu, &mut collector, &ref_map)?;
            for c in Const::evaluate_macros(input, &pending, &self.namespace, &index, &args)? {
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
            for c in Const::evaluate_macros_str(content, &pending, &self.namespace, &index, &args)?
            {
                collector.insert(Item::Const(c));
            }
        }

        let namespace: Vec<&str> = self.namespace.split('.').collect();
        let mut output = format!("#[win32] mod {} {{", namespace[0]);

        for namespace in &namespace[1..] {
            output.push_str(&format!("mod {} {{", namespace));
        }

        for item in collector.values() {
            output.push_str(&item.write()?.to_string());
        }

        for _ in 0..namespace.len() {
            output.push('}');
        }

        let output = formatter::format(&output);
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

        // Macros that the token-based parser cannot handle (complex
        // expressions, references to other macros, arithmetic, etc.) are
        // collected here and evaluated in a second pass via the
        // synthetic-enum technique.
        let mut pending_macros: Vec<String> = vec![];

        for child in tu.cursor().children() {
            // Only process cursors from the main input file (not from
            // transitively included headers).
            if !child.is_from_main_file() {
                continue;
            }

            match child.kind() {
                CXCursor_StructDecl if child.is_definition() => {
                    let name = child.name();
                    if Interface::is_com_interface(child) {
                        if !ref_map.contains_key(&name) {
                            collector.insert(Item::Interface(Interface::parse(
                                child,
                                &self.namespace,
                                tu,
                                ref_map,
                            )?));
                        }
                    } else if !ref_map.contains_key(&name) {
                        collector.insert(Item::Struct(Struct::parse(
                            child,
                            &self.namespace,
                            ref_map,
                        )?));
                    }
                }
                CXCursor_ClassDecl
                    if child.is_definition() && Interface::is_com_interface(child) =>
                {
                    let name = child.name();
                    if !ref_map.contains_key(&name) {
                        collector.insert(Item::Interface(Interface::parse(
                            child,
                            &self.namespace,
                            tu,
                            ref_map,
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
                                namespace: self.namespace.clone(),
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
                        if let Some(cb) = Callback::parse(child, &self.namespace, ref_map)? {
                            collector.insert(Item::Callback(cb));
                        } else if let Some(td) = Typedef::parse(child, &self.namespace, ref_map)? {
                            collector.insert(Item::Typedef(td));
                        }
                    }
                }
                CXCursor_FunctionDecl if !child.is_definition() => {
                    collector.insert(Item::Fn(Fn::parse(
                        child,
                        &self.namespace,
                        &self.library,
                        false,
                        ref_map,
                    )?));
                }
                // Recurse into `extern "C" { }` or `extern "C++" { }` blocks so
                // that function declarations inside them are collected with the
                // correct ABI annotation.
                CXCursor_LinkageSpec => {
                    for inner in child.children() {
                        if !inner.is_from_main_file() {
                            continue;
                        }
                        if inner.kind() == CXCursor_FunctionDecl && !inner.is_definition() {
                            // A function inside `extern "C" { }` has C language linkage.
                            let extern_c = inner.language() == CXLanguage_C;
                            collector.insert(Item::Fn(Fn::parse(
                                inner,
                                &self.namespace,
                                &self.library,
                                extern_c,
                                ref_map,
                            )?));
                        }
                    }
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
        }

        Ok(pending_macros)
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
