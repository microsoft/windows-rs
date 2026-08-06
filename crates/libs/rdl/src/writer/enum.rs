use super::*;

pub fn write_enum(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let repr_field = item
        .fields()
        .next()
        .ok_or_else(|| writer_err!("enum `{}` has no fields", item.name()))?;
    if repr_field.attributes().next().is_some() {
        return Err(writer_err!(
            "enum `{}` has unrepresentable attributes on its backing field",
            item.name()
        ));
    }
    let repr = if let Some(constant) = repr_field.constant() {
        constant.ty()
    } else {
        repr_field.ty()
    };

    let repr = write_type(namespace, &repr);

    let fields = item
        .fields()
        .filter_map(|field| {
            field.constant().map(|constant| {
                write_custom_attributes(field.attributes(), namespace, field.index()).map(
                    |custom_attrs| {
                        let name = write_ident(field.name());
                        let value = write_value(namespace, &constant.value());
                        quote! {
                            #(#custom_attrs)*
                            #name = #value,
                        }
                    },
                )
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let is_flags_attr = |attr: metadata::reader::Attribute| {
        attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
    };

    let has_flags = item.attributes().any(is_flags_attr);

    let arch_attr = write_arch_attr(item.arches());
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
