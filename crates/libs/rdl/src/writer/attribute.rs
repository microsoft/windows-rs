use super::*;

pub fn write_attribute(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let methods = item
        .methods()
        .map(|method| write_method(namespace, &method));

    quote! {
        attribute #name {
            #(#methods)*
        }
    }
}

fn write_method(namespace: &str, item: &metadata::reader::MethodDef) -> TokenStream {
    let signature = item.signature(&[]);
    let params = item.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = write_ident(param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty }
    });

    quote! {
        fn(#(#params),*);
    }
}
