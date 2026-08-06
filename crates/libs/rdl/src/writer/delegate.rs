use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let (generics, generics_tokens) = write_generic_params(item)?;

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .ok_or_else(|| writer_err!("delegate `{}` has no `Invoke` method", item.name()))?;

    let invoke = quote! { invoke };
    let invoke_attrs = write_custom_attributes_wrapped(
        method.attributes(),
        namespace,
        item.index(),
        &[],
        Some(&invoke),
    )?;
    reject_method_generics(&method)?;
    let signature = method.signature(&generics);
    reject_variadic_method(&method, &signature, "delegate")?;
    let return_type = write_return_type(namespace, &method, &signature)?;
    let params = write_params(namespace, &method, signature.types)?;

    let guid_token = match delegate_guid_output(item, &generics)? {
        GuidOutput::None => quote! { #[no_guid] },
        GuidOutput::Omit => quote! {},
        GuidOutput::Explicit(d1, d2, d3, d4) => {
            let lit = syn::LitInt::new(&format_guid_u128(d1, d2, d3, d4), Span::call_site());
            quote! { #[guid(#lit)] }
        }
    };
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
        #(#invoke_attrs)*
        #(#custom_attrs)*
        delegate fn #name #generics_tokens (#(#params),*) #return_type;
    })
}
