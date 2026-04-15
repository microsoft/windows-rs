use super::*;
mod cx;
use cx::*;
use windows_metadata as metadata;

#[derive(Default)]
pub struct Clang {
    input: Vec<String>,
    output: String,
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
        }

        Ok(())
    }
}
