use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub tables::Field);

impl Constant {
    pub fn dependencies(&self) -> Vec<ElementType> {
        self.0.signature().kind.definition()
    }

    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.0.name());
        quote! { #name }
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.0.name();

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/90
        if name == "NaN" || name == "POSITIVE_INFINITY" || name == "NEGATIVE_INFINITY" {
            return quote! {};
        }

        let name = to_ident(name);

        if let Some(constant) = self.0.constant() {
            let signature = self.0.signature();
            if signature.kind == constant.value_type() {
                let value = constant.value().gen();

                quote! {
                    pub const #name: #value;
                }
            } else {
                let kind = signature.gen_win32(gen);
                let value = constant.value().gen_value();

                quote! {
                    pub const #name: #kind = #kind(#value as _);
                }
            }
        } else {
            match Guid::from_attributes(self.0.attributes()) {
                Some(guid) => {
                    let guid = guid.gen();

                    quote! {
                        pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid);
                    }
                }
                None => {
                    // TODO: add support for https://github.com/microsoft/win32metadata/issues/339
                    quote! {}
                }
            }
        }
    }
}
