use super::*;

pub fn gen(def: &TypeDef, _gen: &Gen) -> TokenStream {
    if let Some(_default_interface) = def.default_interface() {
        quote! {}
    } else {
        quote! {}
    }
}
