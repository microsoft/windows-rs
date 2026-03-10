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

    let interfaces = item
        .interface_impls()
        .map(|imp| write_interface(namespace, &imp));

    quote! {
        #(#custom_attrs)*
        class #name #extends {
            #(#interfaces)*
        }
    }
}

fn write_interface(namespace: &str, imp: &metadata::reader::InterfaceImpl) -> TokenStream {
    let interface = write_type(namespace, &imp.interface(&[]));

    let default = if imp.has_attribute("DefaultAttribute") {
        quote! { #[default] }
    } else {
        quote! {}
    };

    quote! {
        #default
        #interface,
    }
}
