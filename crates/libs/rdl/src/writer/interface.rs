use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = format_ident!("{}", item.name());
    let generics: Vec<_> = item.generic_params().map(|param|metadata::Type::Generic(param.sequence())).collect();
    let methods = item.methods().map(|method| write_method(namespace, &method, &generics));

    quote! {
        trait #name {
            #(#methods)*
        }
    }
}

fn write_method(namespace: &str, item: &metadata::reader::MethodDef, generics: &[metadata::Type]) -> TokenStream {
    let name = format_ident!("{}", item.name());
    let signature = item.signature(generics);

    let return_type = if signature.return_type == metadata::Type::Void {
        quote! {}
    } else {
        let ty = write_type(namespace, &signature.return_type);
        quote! { -> #ty }
    };

    let params = item.params().filter(|param|param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = format_ident!("{}", param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty, }
    });

    quote! {
        fn #name(&self, #(#params)*) #return_type;
    }
}
