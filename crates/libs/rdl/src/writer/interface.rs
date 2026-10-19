use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let (generics, generics_tokens) = write_generic_params(item);

    let is_winrt = item
        .flags()
        .contains(metadata::TypeAttributes::WindowsRuntime);
    let members = write_members(namespace, item, &generics, is_winrt)?;

    let requires: Vec<_> = item.interface_impls().collect();

    let requires_tokens = if requires.is_empty() {
        quote! {}
    } else {
        let ifaces = requires
            .iter()
            .map(|imp| write_type(namespace, &imp.interface(&generics)));
        quote! { : #(#ifaces)+* }
    };

    let guid_token = match interface_guid_output(item, &generics)? {
        GuidOutput::None => quote! { #[no_guid] },
        GuidOutput::Omit => quote! {},
        GuidOutput::Explicit(d1, d2, d3, d4) => {
            let lit = syn::LitInt::new(&format_guid_u128(d1, d2, d3, d4), Span::call_site());
            quote! { #[guid(#lit)] }
        }
    };
    let arch_attr = write_arch_attr(item.arches());
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["GuidAttribute", "SupportedArchitectureAttribute"],
    )?;

    Ok(quote! {
        #guid_token
        #arch_attr
        #(#custom_attrs)*
        interface #name #generics_tokens #requires_tokens {
            #(#members)*
        }
    })
}

fn write_members(
    namespace: &str,
    item: &metadata::reader::TypeDef,
    generics: &[metadata::Type],
    is_winrt: bool,
) -> Result<Vec<TokenStream>, Error> {
    let methods: Vec<_> = item.methods().collect();
    let count = methods.len();
    let mut consumed = vec![false; count];
    let mut tokens = Vec::with_capacity(count);
    let event_kw = proc_macro2::Ident::new("event", Span::call_site());

    for i in 0..count {
        if consumed[i] {
            continue;
        }

        let method = &methods[i];

        // Win32 COM properties stay as explicit `#[special] fn` ABI methods.
        if is_winrt
            && method
                .flags()
                .contains(metadata::MethodAttributes::SpecialName)
        {
            let name = method.name();
            let sig = method.signature(generics);

            // The shorthand regenerates accessor signatures, so it only applies to methods with
            // the accessor shape.
            if let Some(prop_name) = name.strip_prefix("get_")
                && is_getter(&sig)
            {
                let put_name = format!("put_{prop_name}");
                let j = find_unconsumed(&methods, &consumed, i, &put_name);

                let ty = write_type(namespace, &sig.return_type);
                let prop_ident = write_ident(prop_name);
                let attrs = write_accessor_attributes(namespace, method)?;

                if let Some(j) = j {
                    // Combine only adjacent get_/put_ pairs with matching types and attributes.
                    let put_sig = methods[j].signature(generics);
                    if is_setter(&put_sig)
                        && put_sig.types[0] == sig.return_type
                        && (i + 1..j).all(|k| consumed[k])
                        && write_accessor_attributes(namespace, &methods[j])?.to_string()
                            == attrs.to_string()
                    {
                        consumed[i] = true;
                        consumed[j] = true;
                        tokens.push(quote! { #attrs #prop_ident: #ty; });
                        continue;
                    }
                }
                consumed[i] = true;
                tokens.push(quote! { #[get] #attrs #prop_ident: #ty; });
                continue;
            } else if let Some(prop_name) = name.strip_prefix("put_")
                && is_setter(&sig)
            {
                let ty = write_type(namespace, &sig.types[0]);
                let prop_ident = write_ident(prop_name);
                let attrs = write_accessor_attributes(namespace, method)?;
                consumed[i] = true;
                tokens.push(quote! { #[set] #attrs #prop_ident: #ty; });
                continue;
            } else if let Some(event_name) = name.strip_prefix("add_")
                && let [handler_ty] = &sig.types[..]
            {
                let remove_name = format!("remove_{event_name}");
                let j = find_unconsumed(&methods, &consumed, i, &remove_name);

                if let Some(j) = j {
                    let attrs = write_accessor_attributes(namespace, method)?;
                    // Event shorthand requires adjacent add_/remove_ methods with matching
                    // attributes.
                    if (i + 1..j).all(|k| consumed[k])
                        && is_setter(&methods[j].signature(generics))
                        && write_accessor_attributes(namespace, &methods[j])?.to_string()
                            == attrs.to_string()
                    {
                        let handler_ty = write_type(namespace, handler_ty);
                        let event_ident = write_ident(event_name);
                        consumed[i] = true;
                        consumed[j] = true;
                        tokens.push(quote! { #attrs #event_kw #event_ident: #handler_ty; });
                        continue;
                    }
                }
            }
        }

        consumed[i] = true;
        tokens.push(write_method(namespace, method, generics)?);
    }

    Ok(tokens)
}

fn is_getter(sig: &metadata::Signature) -> bool {
    sig.types.is_empty() && sig.return_type != metadata::Type::Void
}

// Also the shape of an event's remove accessor.
fn is_setter(sig: &metadata::Signature) -> bool {
    sig.types.len() == 1 && sig.return_type == metadata::Type::Void
}

fn write_accessor_attributes(
    namespace: &str,
    item: &metadata::reader::MethodDef,
) -> Result<TokenStream, Error> {
    let attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;
    Ok(quote! { #(#attrs)* })
}

fn find_unconsumed(
    methods: &[metadata::reader::MethodDef],
    consumed: &[bool],
    from: usize,
    name: &str,
) -> Option<usize> {
    let len = methods.len();
    ((from + 1)..len)
        .chain(0..from)
        .find(|&j| !consumed[j] && methods[j].name() == name)
}

fn write_method(
    namespace: &str,
    item: &metadata::reader::MethodDef,
    generics: &[metadata::Type],
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let signature = item.signature(generics);

    let return_type = write_return_type(namespace, item, &signature)?;
    let params = std::iter::once(Ok(quote! { &self }))
        .chain(
            write_params(namespace, item, signature.types)?
                .into_iter()
                .map(Ok),
        )
        .collect::<Result<Vec<_>, Error>>()?;

    let method_attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;

    // Preserve property/event methods with the built-in `#[special]` pseudo.
    let special_attr = if item
        .flags()
        .contains(metadata::MethodAttributes::SpecialName)
    {
        quote! { #[special] }
    } else {
        quote! {}
    };

    Ok(quote! {
        #special_attr
        #(#method_attrs)*
        fn #name(#(#params),*) #return_type;
    })
}
