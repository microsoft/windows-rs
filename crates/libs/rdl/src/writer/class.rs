use super::*;

pub fn write_class(item: &metadata::reader::TypeDef) -> TokenStream {
    let _namespace = item.namespace();
    let name = format_ident!("{}", item.name());

    quote! {
        class #name {
        }
    }
}
