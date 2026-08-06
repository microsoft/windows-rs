use super::*;

pub fn write_callback(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .ok_or_else(|| writer_err!("callback `{}` has no `Invoke` method", item.name()))?;

    let invoke = quote! { invoke };
    let invoke_attrs = write_custom_attributes_wrapped(
        method.attributes(),
        namespace,
        item.index(),
        &[],
        Some(&invoke),
    )?;
    reject_method_generics(&method)?;
    let signature = method.signature(&[]);
    reject_variadic_method(&method, &signature, "callback")?;
    let return_type = write_return_type(namespace, &method, &signature)?;
    let params = write_params(namespace, &method, signature.types)?;

    let arch_attr = write_arch_attr(item.arches());
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &[
            "UnmanagedFunctionPointerAttribute",
            "SupportedArchitectureAttribute",
        ],
    )?;

    let abi = match read_unmanaged_abi(item) {
        None => None,
        Some(1) => None, // "system" is the default
        Some(2) => Some("C"),
        Some(5) => Some("fastcall"),
        Some(n) => {
            return Err(writer_err!(
                "unexpected CallingConvention value {n} in `UnmanagedFunctionPointerAttribute`"
            ));
        }
    };

    Ok(quote! {
        #arch_attr
        #(#invoke_attrs)*
        #(#custom_attrs)*
        extern #abi fn #name (#(#params),*) #return_type;
    })
}
