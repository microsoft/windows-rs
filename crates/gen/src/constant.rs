use crate::*;
use squote::TokenStream;

#[derive(Debug)]
pub struct Constant {
    pub field: winmd::Field,
}

impl Constant {
    pub fn new(field: &winmd::Field) -> Self {
        Self { field: *field }
    }

    pub fn gen(&self) -> TokenStream {
        self.field.gen(winmd::Gen::Absolute)
    }
}
