use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub tables::Field);

impl Constant {
    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.0.name());
        quote! { #name }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.0.name();

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/90
        if name == "NaN" || name == "POSITIVE_INFINITY" || name == "NEGATIVE_INFINITY" {
            return quote! {};
        }

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/88
        if self.0.constant().is_none() {
            return quote! {};
        }

        let name = to_ident(name);
        let value = self.0.constant().expect("Field").value().gen();

        quote! {
            pub const #name: #value;
        }
    }
}
