use super::*;

pub struct Scope<'a> {
    files: &'a [File],
    //types: BTreeMap<(&'a str, &'a str)
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a[File]) -> Self {
        Self { files }
    }
}
