use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let methods = item
        .methods()
        .map(|method| write_method(namespace, &method, &generics));

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

    // When the interface has no GuidAttribute at all (e.g. some Win32 COM interfaces that
    // intentionally lack a GUID), emit a null GUID so that the reader does not invent one
    // on round-trip.  This is distinct from the derived-GUID case where the attribute is
    // simply omitted because it can be re-computed from the interface shape.
    let has_no_guid = item.find_attribute("GuidAttribute").is_none();

    // Exclude the GuidAttribute when its value can be derived from the interface shape so
    // it does not need to be round-tripped.  When there is no attribute at all the exclude
    // list is harmless since there is nothing to exclude.
    let guid_exclude: &[&str] = if has_no_guid || interface_guid_is_derived(item) {
        &["GuidAttribute"]
    } else {
        &[]
    };
    let custom_attrs =
        write_custom_attributes_except(item.attributes(), namespace, item.index(), guid_exclude);

    let null_guid = if has_no_guid {
        quote! { #[Windows::Foundation::Metadata::Guid(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)] }
    } else {
        quote! {}
    };

    quote! {
        #null_guid
        #(#custom_attrs)*
        interface #name #generics #requires_tokens {
            #(#methods)*
        }
    }
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
