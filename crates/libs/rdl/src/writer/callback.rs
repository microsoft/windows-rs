use super::*;

pub fn write_callback(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .expect("callbacks are expected to have this named method");

    let signature = method.signature(&[]);
    let return_type = write_return_type(namespace, &method, &signature);
    let params = write_params(namespace, &method, signature.types);

    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["UnmanagedFunctionPointerAttribute"],
    );

    let abi = read_unmanaged_abi(item).and_then(|n| match n {
        1 => None, // "system" is the default
        2 => Some("C"),
        5 => Some("fastcall"),
        _ => {
            unreachable!("unexpected CallingConvention value in UnmanagedFunctionPointerAttribute")
        }
    });

    quote! {
        #(#custom_attrs)*
        extern #abi fn #name (#(#params),*) #return_type;
    }
}
