use super::*;

pub fn write_fn(namespace: &str, item: &metadata::reader::MethodDef) -> TokenStream {
    let name = write_ident(item.name());
    let signature = item.signature(&[]);

    let return_type = write_return_type(namespace, &signature);
    let vararg = signature
        .flags
        .contains(metadata::MethodCallAttributes::VARARG);
    let mut params = write_params(namespace, item, signature.types);

    if vararg {
        params.push(quote! { ... });
    }

    let Some(impl_map) = item.impl_map() else {
        unreachable!("fn item must have an ImplMap to be written as an `fn` item")
    };

    let scope = impl_map.import_scope();
    let library = scope.name();
    let flags = impl_map.flags();

    let abi = if flags.contains(metadata::PInvokeAttributes::CallConvFastcall) {
        Some("fastcall")
    } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
        Some("C")
    } else if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
        None
    } else {
        unreachable!(
            "unexpected calling convention in ImplMap flags: {:?}",
            flags
        )
    };

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    let library_attr = if flags.contains(metadata::PInvokeAttributes::SupportsLastError) {
        quote! { #[library(#library, last_error = true)] }
    } else {
        quote! { #[library(#library)] }
    };

    quote! {
        #(#custom_attrs)*
        #library_attr
        extern #abi fn #name(#(#params),*) #return_type;
    }
}
