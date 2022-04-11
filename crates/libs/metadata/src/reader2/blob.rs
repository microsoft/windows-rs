use super::*;

pub struct Blob<'a> {
    scope: &'a Scope<'a>,
    file: usize,
    slice: &'a [u8],
}

impl<'a> Blob<'a> {
    pub fn new(scope: &'a Scope, file: usize, slice: &'a [u8]) -> Self {
        Self { scope, file, slice }
    }
}
