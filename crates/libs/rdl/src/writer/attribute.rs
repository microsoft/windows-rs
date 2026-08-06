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
    let fields = item
        .fields()
        .filter(|field| {
            let flags = field.flags();
            flags.contains(metadata::FieldAttributes::Public)
                && !flags.contains(metadata::FieldAttributes::Static)
                && !flags.contains(metadata::FieldAttributes::Literal)
                && !flags.contains(metadata::FieldAttributes::SpecialName)
        })
        .map(|field| {
            if field.attributes().next().is_some() {
                return Err(writer_err!(
                    "attribute property `{}` has unrepresentable custom attributes",
                    field.name()
                ));
            }
            let name = write_ident(field.name());
            let ty = write_type(namespace, &field.ty());
            Ok(quote! { #name: #ty, })
        })
        .collect::<Result<Vec<_>, Error>>()?;

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
    if item.attributes().next().is_some() {
        return Err(writer_err!(
            "attribute constructor `{}` has unrepresentable custom attributes",
            item.name()
        ));
    }
    reject_method_generics(item)?;
    let signature = item.signature(&[]);
    reject_variadic_method(item, &signature, "attribute constructor")?;
    if signature.return_type != metadata::Type::Void {
        return Err(writer_err!(
            "attribute constructor `{}` has an unrepresentable return type",
            item.name()
        ));
    }
    let params = write_params(namespace, item, signature.types)?;

    Ok(quote! {
        fn(#(#params),*);
    })
}
