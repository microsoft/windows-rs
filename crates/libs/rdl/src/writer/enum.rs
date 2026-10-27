use super::*;

pub fn write_enum(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    write_enum_scoped(item.namespace(), item, None)
}

pub(super) fn write_enum_scoped(
    namespace: &str,
    item: &metadata::reader::TypeDef,
    parent_arches: Option<i32>,
) -> Result<TokenStream, Error> {
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

    let fields = item
        .fields()
        .filter_map(|field| {
            field.constant().map(|constant| -> Result<_, Error> {
                let name = write_ident(field.name());
                let value = write_value(namespace, &constant.value());
                let attributes =
                    write_custom_attributes(field.attributes(), namespace, field.index())?;
                Ok(quote! { #(#attributes)* #name = #value, })
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let is_flags_attr = |attr: metadata::reader::Attribute| {
        attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
    };

    let has_flags = item.attributes().any(is_flags_attr);

    let arch_attr = if parent_arches == Some(item.arches()) {
        quote! {}
    } else {
        write_arch_attr(item.arches())
    };
    let custom_attrs = write_custom_attributes_except(
        item.attributes().filter(|attr| !is_flags_attr(*attr)),
        namespace,
        item.index(),
        &["SupportedArchitectureAttribute"],
    )?;

    if has_flags {
        Ok(quote! {
            #[repr(#repr)]
            #[flags]
            #arch_attr
            #(#custom_attrs)*
            enum #name {
                #(#fields)*
            }
        })
    } else {
        Ok(quote! {
            #[repr(#repr)]
            #arch_attr
            #(#custom_attrs)*
            enum #name {
                #(#fields)*
            }
        })
    }
}
