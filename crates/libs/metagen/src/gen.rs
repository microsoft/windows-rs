use super::*;

// TODO: should be able to write a trivial CLI for this, call it ridl. :)

#[derive(Default)]
pub struct Gen {
    // Source files to include
    pub sources: Vec<String>,

    // Winmd files to include.
    pub inputs: Vec<String>,

    // Winmd files to reference.
    pub references: Vec<String>,

    // Name of resulting winmd file.
    pub output: String,
}

impl Gen {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn gen(gen: &Gen) -> std::io::Result<()> {
    Ok(())
}
