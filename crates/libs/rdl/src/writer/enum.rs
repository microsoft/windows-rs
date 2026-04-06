use super::*;

pub fn write_enum(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let repr_field = item
        .fields()
        .next()
        .ok_or_else(|| writer_err!("enum `{}` has no fields", item.name()))?;
    let repr = if let Some(constant) = repr_field.constant() {
        constant.ty()
    } else {
        repr_field.ty()
    };

    let repr = write_type(namespace, &repr);

    let fields = item.fields().filter_map(|field| {
        field.constant().map(|constant| {
            let name = write_ident(field.name());
            let value = write_value(namespace, &constant.value());
            quote! { #name = #value, }
        })
    });

    let is_flags_attr = |attr: metadata::reader::Attribute| {
        attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
    };

    let has_flags = item.attributes().any(is_flags_attr);

    let custom_attrs = write_custom_attributes(
        item.attributes().filter(|attr| !is_flags_attr(*attr)),
        namespace,
        item.index(),
    )?;

    if has_flags {
        Ok(quote! {
            #[repr(#repr)]
            #[flags]
            #(#custom_attrs)*
            enum #name {
                #(#fields)*
            }
        })
    } else {
        Ok(quote! {
            #[repr(#repr)]
            #(#custom_attrs)*
            enum #name {
                #(#fields)*
            }
        })
    }
}
