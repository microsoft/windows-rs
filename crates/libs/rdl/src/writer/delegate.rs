use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let (generics, generics_tokens) = write_generic_params(item);

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .ok_or_else(|| {
            Error::new(
                &format!("delegate `{}` has no `Invoke` method", item.name()),
                "",
                0,
                0,
            )
        })?;

    let signature = method.signature(&generics);
    let return_type = write_return_type(namespace, &method, &signature);
    let params = write_params(namespace, &method, signature.types);

    let guid_token = match delegate_guid_output(item, &generics)? {
        GuidOutput::None => quote! { #[no_guid] },
        GuidOutput::Omit => quote! {},
        GuidOutput::Explicit(d1, d2, d3, d4) => {
            let lit = syn::LitInt::new(&format_guid_u128(d1, d2, d3, d4), Span::call_site());
            quote! { #[guid(#lit)] }
        }
    };
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["GuidAttribute", "UnmanagedFunctionPointerAttribute"],
    );

    let abi = match read_unmanaged_abi(item) {
        None => None,
        Some(1) => Some("system"),
        Some(2) => Some("C"),
        Some(5) => Some("fastcall"),
        Some(n) => {
            return Err(Error::new(
                &format!(
                    "unexpected CallingConvention value {n} in `UnmanagedFunctionPointerAttribute`"
                ),
                "",
                0,
                0,
            ))
        }
    };

    Ok(quote! {
        #guid_token
        #(#custom_attrs)*
        delegate #abi fn #name #generics_tokens (#(#params),*) #return_type;
    })
}
