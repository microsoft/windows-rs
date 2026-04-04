use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let (generics, generics_tokens) = write_generic_params(item);

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .expect("delegates are expected to have this named method");

    let signature = method.signature(&generics);
    let return_type = write_return_type(namespace, &method, &signature);
    let params = write_params(namespace, &method, signature.types);

    let guid_token = match delegate_guid_output(item, &generics) {
        GuidOutput::None => quote! { #[no_guid] },
        GuidOutput::Omit => quote! {},
        GuidOutput::Explicit(d1, d2, d3, d4) => {
            let hex: TokenStream = format_guid_u128(d1, d2, d3, d4).parse().unwrap();
            quote! { #[guid(#hex)] }
        }
    };
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["GuidAttribute", "UnmanagedFunctionPointerAttribute"],
    );

    let abi = read_unmanaged_abi(item).map(|n| match n {
        1 => Some("system"),
        2 => Some("C"),
        5 => Some("fastcall"),
        _ => {
            unreachable!("unexpected CallingConvention value in UnmanagedFunctionPointerAttribute")
        }
    });

    quote! {
        #guid_token
        #(#custom_attrs)*
        delegate #abi fn #name #generics_tokens (#(#params),*) #return_type;
    }
}
