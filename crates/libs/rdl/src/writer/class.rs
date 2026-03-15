use super::*;

pub fn write_class(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());
    let extends = item.extends().expect("class always extends");

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    // Build the colon list: optional base class followed by interfaces.
    // The default interface (if any) must appear first among the interfaces.
    let mut colon_parts: Vec<TokenStream> = vec![];

    if extends != ("System", "Object") {
        let ty = write_type_ref(namespace, &extends);
        colon_parts.push(quote! { #ty });
    }

    let mut impls: Vec<_> = item.interface_impls().collect();
    // Sort so that the default interface appears first.
    impls.sort_by_key(|imp| {
        if imp.has_attribute("DefaultAttribute") {
            0
        } else {
            1
        }
    });

    for imp in &impls {
        let interface = write_type(namespace, &imp.interface(&[]));
        colon_parts.push(quote! { #interface });
    }

    let colon_list = if colon_parts.is_empty() {
        quote! {}
    } else {
        quote! { : #(#colon_parts),* }
    };

    quote! {
        #(#custom_attrs)*
        class #name #colon_list {}
    }
}
