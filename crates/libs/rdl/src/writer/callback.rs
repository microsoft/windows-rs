use super::*;

pub fn write_callback(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .ok_or_else(|| {
            Error::new(
                &format!("callback `{}` has no `Invoke` method", item.name()),
                "",
                0,
                0,
            )
        })?;

    let signature = method.signature(&[]);
    let return_type = write_return_type(namespace, &method, &signature);
    let params = write_params(namespace, &method, signature.types);

    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["UnmanagedFunctionPointerAttribute"],
    );

    let abi = match read_unmanaged_abi(item) {
        None => None,
        Some(1) => None, // "system" is the default
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
        #(#custom_attrs)*
        extern #abi fn #name (#(#params),*) #return_type;
    })
}
