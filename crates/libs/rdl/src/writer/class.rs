use super::*;

pub fn write_class(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = format_ident!("{}", item.name());
    let extends = item.extends().expect("class always extends");

    let extends = if extends == ("System", "Object") {
        quote! {}
    } else {
        let ty = write_type_ref(namespace, &extends);
        quote! { : #ty }
    };

    let activatable = if is_activatable(item) {
        quote! { #[activatable(1)] }
    } else {
        quote! {}
    };

    let interfaces = item
        .interface_impls()
        .map(|imp| write_interface(namespace, &imp));
    let factories = item
        .attributes()
        .filter_map(|attribute| write_factory(namespace, &attribute));

    quote! {
        #activatable
        class #name #extends {
            #(#interfaces)*
            #(#factories)*
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
        #interface
    }
}

fn write_factory(namespace: &str, attribute: &metadata::reader::Attribute) -> Option<TokenStream> {
    let name = attribute.name();

    let name = if name == "ActivatableAttribute" {
        format_ident!("activatable")
    } else if name == "StaticAttribute" {
        format_ident!("statics")
    } else {
        // TODO: need to handle any other attributes?
        return None;
    };

    for (_, value) in attribute.value() {
        if let metadata::Value::Utf8(tn) = value {
            let index = tn
                .rfind('.')
                .expect("expected full name separated with `.`");

            let ty = write_type(
                namespace,
                &metadata::Type::named(&tn[0..index], &tn[index + 1..]),
            );

            return Some(quote! {
                #[#name(1)]
                #ty,
            });
        }
    }

    None
}

fn is_activatable(item: &metadata::reader::TypeDef) -> bool {
    item.attributes()
        .filter(|attribute| attribute.name() == "ActivatableAttribute")
        .any(|attribute| {
            !attribute
                .value()
                .iter()
                .any(|(_, value)| matches!(value, metadata::Value::Utf8(_)))
        })
}
