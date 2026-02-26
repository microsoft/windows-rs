use super::*;

pub fn write_attribute(item: &metadata::reader::TypeDef) -> TokenStream {
    let _name = write_ident(item.name());

    quote! {
        // class #name : Attribute {}
    }
}
