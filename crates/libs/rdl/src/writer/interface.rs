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
