use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(metadata::trim_tick(item.name()));

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

    quote! {
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

    let return_type = if signature.return_type == metadata::Type::Void {
        quote! {}
    } else {
        let ty = write_type(namespace, &signature.return_type);
        quote! { -> #ty }
    };

    let params = item.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = write_ident(param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty, }
    });

    quote! {
        fn #name(&self, #(#params)*) #return_type;
    }
}
