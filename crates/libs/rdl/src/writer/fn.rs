use super::*;

pub fn write_fn(namespace: &str, item: &metadata::reader::MethodDef) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let signature = item.signature(&[]);

    let return_type = write_return_type(namespace, item, &signature)?;
    let vararg = signature
        .flags
        .contains(metadata::MethodCallAttributes::VARARG);
    let mut params = write_params(namespace, item, signature.types)?;

    if vararg {
        params.push(quote! { ... });
    }

    let Some(impl_map) = item.impl_map() else {
        return Err(writer_err!("fn `{}` has no `ImplMap` record", item.name()));
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
        return Err(writer_err!(
            "unexpected calling convention in `ImplMap` flags for fn `{}`",
            item.name()
        ));
    };

    let arch_attr = write_arch_attr(item.arches());
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["SupportedArchitectureAttribute"],
    )?;

    let library_attr = if flags.contains(metadata::PInvokeAttributes::SupportsLastError) {
        quote! { #[library(#library, last_error = true)] }
    } else {
        quote! { #[library(#library)] }
    };

    Ok(quote! {
        #arch_attr
        #(#custom_attrs)*
        #library_attr
        extern #abi fn #name(#(#params),*) #return_type;
    })
}
