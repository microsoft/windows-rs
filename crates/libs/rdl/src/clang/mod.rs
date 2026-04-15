use super::*;
mod cx;
use cx::*;

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
        let _library = Library::new()?;
        let index = Index::new()?;

        for input in &self.input {
            let _tu = index.parse(input, &[])?;
        }

        Ok(())
    }
}
