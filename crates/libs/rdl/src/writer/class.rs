use super::*;

pub fn write_class(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());
    let extends = item.extends().expect("class always extends");

    let extends = if extends == ("System", "Object") {
        quote! {}
    } else {
        let ty = write_type_ref(namespace, &extends);
        quote! { : #ty }
    };

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    let mut impls: Vec<_> = item.interface_impls().collect();
    impls.sort_by_key(|imp| !imp.has_attribute("DefaultAttribute"));

    let interfaces = impls.iter().map(|imp| write_interface(namespace, imp));

    quote! {
        #(#custom_attrs)*
        class #name #extends {
            #(#interfaces)*
        }
    }
}

fn write_interface(namespace: &str, imp: &metadata::reader::InterfaceImpl) -> TokenStream {
    let interface = write_type(namespace, &imp.interface(&[]));

    quote! {
        #interface,
    }
}
