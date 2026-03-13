use super::*;

pub fn write_callback(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .expect("callbacks are expected to have this named method");

    let signature = method.signature(&[]);
    let return_type = write_return_type(namespace, &signature);
    let params = method.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = write_ident(param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty }
    });

    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["UnmanagedFunctionPointerAttribute"],
    );

    let mut abi = None;

    if let Some(attribute) = item.find_attribute("UnmanagedFunctionPointerAttribute") {
        if let Some((_, metadata::Value::EnumValue(_, value))) = attribute.value().first() {
            match &**value {
                metadata::Value::I32(1) => {} // "system" is the default
                metadata::Value::I32(2) => abi = Some("C"),
                metadata::Value::I32(5) => abi = Some("fastcall"),
                rest => todo!("{rest:?}"),
            }
        }
    }

    quote! {
        #(#custom_attrs)*
        extern #abi fn #name (#(#params),*) #return_type;
    }
}
