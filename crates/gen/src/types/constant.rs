use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub tables::Field);

impl Constant {
    // TODO: move to Field?
    pub fn gen(def: &tables::Field, gen: &Gen) -> TokenStream {
        let name = def.name();
        let name = to_ident(name);

        if let Some(constant) = def.constant() {
            let signature = def.signature();
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
            match Guid::from_attributes(def.attributes()) {
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
