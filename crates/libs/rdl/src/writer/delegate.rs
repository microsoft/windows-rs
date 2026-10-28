use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let (generics, generics_tokens) = write_generic_params(item);

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .ok_or_else(|| writer_err!("delegate `{}` has no `Invoke` method", item.name()))?;

    let signature = method.signature(&generics);
    let return_type = write_return_type(namespace, &method, &signature)?;
    let params = write_params(namespace, &method, signature.types)?;

    let guid_token = write_guid_attribute(item)?;
    let arch_attr = write_arch_attr(item.arches());
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &[
            "GuidAttribute",
            "UnmanagedFunctionPointerAttribute",
            "SupportedArchitectureAttribute",
        ],
    )?;

    Ok(quote! {
        #guid_token
        #arch_attr
        #(#custom_attrs)*
        delegate fn #name #generics_tokens (#(#params),*) #return_type;
    })
}
