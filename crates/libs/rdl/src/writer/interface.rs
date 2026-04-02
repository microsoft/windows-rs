use super::*;

const EVENT_REGISTRATION_TOKEN_NAMESPACE: &str = "Windows.Foundation";
const EVENT_REGISTRATION_TOKEN_NAME: &str = "EventRegistrationToken";

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let methods: Vec<_> = item.methods().collect();
    let method_tokens = write_interface_methods(namespace, &methods, &generics);

    let requires: Vec<_> = item.interface_impls().collect();

    let requires_tokens = if requires.is_empty() {
        quote! {}
    } else {
        let ifaces = requires
            .iter()
            .map(|imp| write_type(namespace, &imp.interface(&generics)));
        quote! { : #(#ifaces)+* }
    };

    let generics = if generics.is_empty() {
        quote! {}
    } else {
        let generics = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#generics),*> }
    };

    let guid_exclude: &[&str] = if interface_guid_is_derived(item) {
        &["GuidAttribute"]
    } else {
        &[]
    };
    let custom_attrs =
        write_custom_attributes_except(item.attributes(), namespace, item.index(), guid_exclude);

    quote! {
        #(#custom_attrs)*
        interface #name #generics #requires_tokens {
            #(#method_tokens)*
        }
    }
}

/// Returns true if the given metadata type is `Windows.Foundation.EventRegistrationToken`.
fn is_event_registration_token(ty: &metadata::Type) -> bool {
    match ty {
        metadata::Type::ValueName(tn) => {
            tn == (
                EVENT_REGISTRATION_TOKEN_NAMESPACE,
                EVENT_REGISTRATION_TOKEN_NAME,
            )
        }
        _ => false,
    }
}

fn write_interface_methods(
    namespace: &str,
    methods: &[metadata::reader::MethodDef],
    generics: &[metadata::Type],
) -> Vec<TokenStream> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < methods.len() {
        let method = &methods[i];
        let method_name = method.name();

        if method
            .flags()
            .contains(metadata::MethodAttributes::SpecialName)
        {
            // Try to detect a property pair: get_X followed by put_X
            if let Some(base) = method_name.strip_prefix("get_") {
                if let Some(next) = methods.get(i + 1) {
                    if next
                        .flags()
                        .contains(metadata::MethodAttributes::SpecialName)
                    {
                        if let Some(next_base) = next.name().strip_prefix("put_") {
                            if next_base == base {
                                let get_sig = method.signature(generics);
                                let put_sig = next.signature(generics);
                                // get: no params, non-void return; put: one param matching get's return, void return
                                if get_sig.types.is_empty()
                                    && get_sig.return_type != metadata::Type::Void
                                    && put_sig.types.len() == 1
                                    && put_sig.return_type == metadata::Type::Void
                                    && get_sig.return_type == put_sig.types[0]
                                {
                                    let prop_name = write_ident(base);
                                    let ty = write_type(namespace, &get_sig.return_type);
                                    tokens.push(quote! { #prop_name: #ty; });
                                    i += 2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }

            // Try to detect an event pair: add_X followed by remove_X
            if let Some(base) = method_name.strip_prefix("add_") {
                if let Some(next) = methods.get(i + 1) {
                    if next
                        .flags()
                        .contains(metadata::MethodAttributes::SpecialName)
                    {
                        if let Some(next_base) = next.name().strip_prefix("remove_") {
                            if next_base == base {
                                let add_sig = method.signature(generics);
                                let remove_sig = next.signature(generics);
                                // add: one param (handler), returns EventRegistrationToken
                                // remove: one param (EventRegistrationToken), void return
                                if add_sig.types.len() == 1
                                    && is_event_registration_token(&add_sig.return_type)
                                    && remove_sig.types.len() == 1
                                    && is_event_registration_token(&remove_sig.types[0])
                                    && remove_sig.return_type == metadata::Type::Void
                                {
                                    let event_name = write_ident(base);
                                    let ty = write_type(namespace, &add_sig.types[0]);
                                    tokens.push(quote! { event #event_name: #ty; });
                                    i += 2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fall through: emit as a regular or special method
        tokens.push(write_method(namespace, method, generics));
        i += 1;
    }

    tokens
}

fn write_method(
    namespace: &str,
    item: &metadata::reader::MethodDef,
    generics: &[metadata::Type],
) -> TokenStream {
    let name = write_ident(item.name());
    let signature = item.signature(generics);

    let return_type = write_return_type(namespace, &signature);
    let params =
        std::iter::once(quote! { &self }).chain(write_params(namespace, item, signature.types));

    let method_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    // Emit the built-in `#[special]` pseudo-attribute when SpecialName is set,
    // preserving properties and events on round-trip.
    let special_attr = if item
        .flags()
        .contains(metadata::MethodAttributes::SpecialName)
    {
        quote! { #[special] }
    } else {
        quote! {}
    };

    quote! {
        #special_attr
        #(#method_attrs)*
        fn #name(#(#params),*) #return_type;
    }
}
