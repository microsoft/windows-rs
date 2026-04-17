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

#[derive(Default)]
pub struct Clang {
    input: Vec<String>,
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

        let _reference = metadata::reader::TypeIndex::new(winmd_files);

        let _library = Library::new()?;
        let index = Index::new()?;
        let mut collector = Collector::new();
        let args: Vec<_> = self.args.iter().map(String::as_str).collect();

        for input in &h_paths {
            let tu = index.parse(input, &args)?;

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
                        collector.insert(Item::Struct(Struct::parse(child, &self.namespace)?));
                    }
                    CXCursor_EnumDecl if child.is_definition() => {
                        collector.insert(Item::Enum(Enum::parse(child)?));
                    }
                    CXCursor_TypedefDecl if child.is_definition() => {
                        if let Some(cb) = Callback::parse(child, &self.namespace)? {
                            collector.insert(Item::Callback(cb));
                        } else if let Some(td) = Typedef::parse(child, &self.namespace)? {
                            collector.insert(Item::Typedef(td));
                        }
                    }
                    CXCursor_FunctionDecl if !child.is_definition() => {
                        collector.insert(Item::Fn(Fn::parse(
                            child,
                            &self.namespace,
                            &self.library,
                        )?));
                    }
                    CXCursor_MacroDefinition => {
                        if let Some(c) = Const::parse(child, &self.namespace, &tu)? {
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

            // Second pass: evaluate complex constant expressions using the
            // synthetic-enum approach.  One bad macro does not abort the rest.
            for c in Const::evaluate_macros(input, &pending_macros, &self.namespace, &index, &args)?
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
}
