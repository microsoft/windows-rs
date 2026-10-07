use super::*;

pub fn write_class(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());
    let extends = item
        .extends()
        .ok_or_else(|| writer_err!("class `{}` has no base type", item.name()))?;

    let extends = if extends == ("System", "Object") {
        quote! {}
    } else {
        let ty = write_type_ref(namespace, &extends);
        quote! { : #ty }
    };

    if !item
        .flags()
        .contains(metadata::TypeAttributes::WindowsRuntime)
    {
        if !extends.is_empty()
            || item.interface_impls().next().is_some()
            || item.fields().next().is_some()
            || item.methods().next().is_some()
        {
            return Err(writer_err!(
                "native class `{}` has members or a base class",
                item.name()
            ));
        }
        let custom_attrs = write_custom_attributes_except(
            item.attributes(),
            namespace,
            item.index(),
            &["GuidAttribute"],
        )?;
        let guid = if let Some(attribute) = item.find_attribute("GuidAttribute") {
            let (d1, d2, d3, d4) = extract_guid_from_attribute(attribute)?;
            let value = syn::LitInt::new(&format_guid_u128(d1, d2, d3, d4), Span::call_site());
            quote! { #[guid(#value)] }
        } else {
            quote! {}
        };
        return Ok(quote! { #guid #(#custom_attrs)* class #name; });
    }

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;
    let mut impls: Vec<_> = item.interface_impls().collect();
    impls.sort_by_key(|imp| !imp.has_attribute("DefaultAttribute"));

    let interfaces = impls.iter().map(|imp| write_interface(namespace, imp));

    Ok(quote! {
        #(#custom_attrs)*
        class #name #extends {
            #(#interfaces)*
        }
    })
}

fn write_interface(namespace: &str, imp: &metadata::reader::InterfaceImpl) -> TokenStream {
    let interface = write_type(namespace, &imp.interface(&[]));

    quote! {
        #interface,
    }
}
