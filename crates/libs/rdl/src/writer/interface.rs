use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let methods = item
        .methods()
        .map(|method| write_method(namespace, &method, &generics));

    let generics = if generics.is_empty() {
        quote! {}
    } else {
        let generics = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#generics),*> }
    };

    let custom_attrs = write_custom_attributes(item);

    quote! {
        #(#custom_attrs)*
        interface #name #generics {
            #(#methods)*
        }
    }
}

fn write_method(
    namespace: &str,
    item: &metadata::reader::MethodDef,
    generics: &[metadata::Type],
) -> TokenStream {
    let name = write_ident(item.name());
    let signature = item.signature(generics);

    let return_type = write_return_type(namespace, &signature);
    let params = item.params().filter(|param| param.sequence() != 0);

    let params =
        std::iter::once(quote! { &self }).chain(params.zip(signature.types).map(|(param, ty)| {
            let name = write_ident(param.name());
            let ty = write_type(namespace, &ty);
            quote! { #name: #ty }
        }));

    quote! {
        fn #name(#(#params),*) #return_type;
    }
}
