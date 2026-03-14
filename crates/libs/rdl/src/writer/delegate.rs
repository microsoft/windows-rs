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
    let return_type = write_return_type(namespace, &signature);
    let params = method.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = write_ident(param.name());
        let out_attr = if param.flags().contains(metadata::ParamAttributes::Out)
            && !matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..))
        {
            quote! { #[out] }
        } else {
            quote! {}
        };
        let ty = write_type(namespace, &ty);
        quote! { #out_attr #name: #ty }
    });

    let generics = if generics.is_empty() {
        quote! {}
    } else {
        let generics = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#generics),*> }
    };

    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        // GuidAttribute is derived from the delegate shape; skip it so round-trips stay clean
        &["GuidAttribute", "UnmanagedFunctionPointerAttribute"],
    );

    let mut abi = None;

    if let Some(attribute) = item.find_attribute("UnmanagedFunctionPointerAttribute") {
        if let Some((_, metadata::Value::EnumValue(_, value))) = attribute.value().first() {
            match &**value {
                metadata::Value::I32(1) => abi = Some("system"),
                metadata::Value::I32(2) => abi = Some("C"),
                metadata::Value::I32(5) => abi = Some("fastcall"),
                rest => {
                    eprintln!(
                        "windows-rdl: unsupported calling convention `{rest:?}` on delegate `{}`, skipping ABI annotation",
                        item.name()
                    );
                }
            }
        }
    }

    quote! {
        #(#custom_attrs)*
        delegate #abi fn #name #generics (#(#params),*) #return_type;
    }
}
