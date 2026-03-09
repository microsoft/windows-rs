use super::*;

pub fn write_attribute(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let methods = item
        .methods()
        .map(|method| write_method(namespace, &method));

    // Named instance-field properties (e.g. `version: u32`).
    // Skip literals (enum variants), statics, and special-name fields (value__).
    let fields = item.fields().filter_map(|f| {
        let flags = f.flags();
        if flags.contains(metadata::FieldAttributes::Public)
            && !flags.contains(metadata::FieldAttributes::Static)
            && !flags.contains(metadata::FieldAttributes::Literal)
            && !flags.contains(metadata::FieldAttributes::SpecialName)
        {
            let name = write_ident(f.name());
            let ty = write_type(namespace, &f.ty());
            Some(quote! { #name: #ty, })
        } else {
            None
        }
    });

    quote! {
        attribute #name {
            #(#methods)*
            #(#fields)*
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
