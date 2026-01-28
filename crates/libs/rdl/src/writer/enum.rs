use super::*;

pub fn write_enum(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = format_ident!("{}", item.name());

    let repr = item.fields().next().unwrap();
    let repr = if let Some(constant) = repr.constant() {
        constant.ty()
    } else {
        repr.ty()
    };

    let repr = write_type(namespace, &repr);

    let fields = item.fields().filter_map(|field| {
        field.constant().map(|constant| {
            let name = format_ident!("{}", field.name());
            let value = write_value(&constant.value());
            quote! { #name = #value, }
        })
    });

    quote! {
        #[repr(#repr)]
        enum #name {
            #(#fields)*
        }
    }
}
