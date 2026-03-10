use super::*;

pub fn write_enum(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let repr = item.fields().next().unwrap();
    let repr = if let Some(constant) = repr.constant() {
        constant.ty()
    } else {
        repr.ty()
    };

    let repr = write_type(namespace, &repr);

    let fields = item.fields().filter_map(|field| {
        field.constant().map(|constant| {
            let name = write_ident(field.name());
            let value = write_value(namespace, &constant.value());
            quote! { #name = #value, }
        })
    });

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    quote! {
        #[repr(#repr)]
        #(#custom_attrs)*
        enum #name {
            #(#fields)*
        }
    }
}
