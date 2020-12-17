use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Constant {
    field: winmd::Field
}

impl Constant {
    pub fn from_field(field: &winmd::Field) -> Self {
        Self { field: *field }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {

        }
    }
}
