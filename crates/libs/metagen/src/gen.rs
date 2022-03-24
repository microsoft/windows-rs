use super::*;

// TODO: should be able to write a trivial CLI for this, call it ridl. :)

#[derive(Default)]
pub struct Gen<'a> {
    // Source files to include
    pub source: Vec<&'a str>,

    // Winmd files to include.
    pub input: Vec<&'a str>,

    // Winmd files to reference.
    pub reference: Vec<&'a str>,

    // Name of resulting winmd file.
    pub output: &'a str,
}

pub fn gen(gen: &Gen) {

}
