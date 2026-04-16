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

#[derive(Default)]
pub struct Clang {
    input: Vec<String>,
    output: String,
    namespace: String,
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

        for input in &h_paths {
            let tu = index.parse(input, &[])?;

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

            for child in tu.cursor().children() {
                if !child.is_definition() {
                    continue;
                }

                match child.kind() {
                    CXCursor_StructDecl => {
                        collector.insert(Item::Struct(Struct::parse(child, &self.namespace)?))
                    }
                    CXCursor_UnionDecl => {}
                    CXCursor_EnumDecl => collector.insert(Item::Enum(Enum::parse(child)?)),
                    _ => {}
                }
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
