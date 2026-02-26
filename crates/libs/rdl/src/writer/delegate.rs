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
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty }
    });

    let generics = if generics.is_empty() {
        quote! {}
    } else {
        let generics = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#generics),*> }
    };

    quote! {
        delegate fn #name #generics (#(#params),*) #return_type;
    }
}
