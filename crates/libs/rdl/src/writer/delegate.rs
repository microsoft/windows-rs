use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = format_ident!("{}", item.name());

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .expect("delegates are expected to have this named method");

    let signature = method.signature(&[]);

    let return_type = if signature.return_type == metadata::Type::Void {
        quote! {}
    } else {
        let ty = write_type(namespace, &signature.return_type);
        quote! { -> #ty }
    };

    let params = method.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = format_ident!("{}", param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty, }
    });

    quote! {
        delegate fn #name(#(#params)*) #return_type;
    }
}
