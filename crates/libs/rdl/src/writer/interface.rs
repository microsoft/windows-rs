use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let name = format_ident!("{}", item.name());

    quote! {
        trait #name {
        }
    }
}
