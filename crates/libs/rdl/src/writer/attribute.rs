use super::*;

pub fn write_attribute(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let methods: Vec<TokenStream> = item
        .methods()
        .map(|method| write_method(namespace, &method))
        .collect::<Result<Vec<_>, _>>()?;

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

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;

    Ok(quote! {
        #(#custom_attrs)*
        attribute #name {
            #(#methods)*
            #(#fields)*
        }
    })
}

fn write_method(namespace: &str, item: &metadata::reader::MethodDef) -> Result<TokenStream, Error> {
    let signature = item.signature(&[]);
    let params = write_params(namespace, item, signature.types)?;

    Ok(quote! {
        fn(#(#params),*);
    })
}
