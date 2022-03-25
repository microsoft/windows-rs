use super::*;

pub struct Scope<'a> {
    files: &'a [File],
    types: BTreeMap<Name<'a>, ScopeKey>
}

pub(crate) struct ScopeKey {
    pub row: u32,
    pub table: u32,
    pub file: u32,
}

pub(crate) struct Row<'a> {
    pub scope: &'a Scope<'a>,
    pub key: ScopeKey,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a[File]) -> Self {
        Self { files, types: BTreeMap::new() }
    }
}

