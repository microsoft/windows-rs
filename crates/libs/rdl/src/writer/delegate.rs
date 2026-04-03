use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .expect("delegates are expected to have this named method");

    let signature = method.signature(&generics);
    let return_type = write_return_type(namespace, &method, &signature);
    let params = write_params(namespace, &method, signature.types);

    let generics = if generics.is_empty() {
        quote! {}
    } else {
        let generics = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#generics),*> }
    };

    let guid_token = match delegate_guid_output(item) {
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

    let mut abi = None;

    if let Some(attribute) = item.find_attribute("UnmanagedFunctionPointerAttribute") {
        if let Some((_, metadata::Value::EnumValue(_, value))) = attribute.value().first() {
            match &**value {
                metadata::Value::I32(1) => abi = Some("system"),
                metadata::Value::I32(2) => abi = Some("C"),
                metadata::Value::I32(5) => abi = Some("fastcall"),
                rest => unreachable!("unexpected CallingConvention value in UnmanagedFunctionPointerAttribute: {rest:?}"),
            }
        }
    }

    quote! {
        #guid_token
        #(#custom_attrs)*
        delegate #abi fn #name #generics (#(#params),*) #return_type;
    }
}
