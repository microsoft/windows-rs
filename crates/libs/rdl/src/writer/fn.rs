use super::*;

pub fn write_fn(namespace: &str, item: &metadata::reader::MethodDef) -> TokenStream {
    let name = write_ident(item.name());
    let signature = item.signature(&[]);

    let return_type = write_return_type(namespace, &signature);
    let params = item.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = write_ident(param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty }
    });

    let Some(impl_map) = item.impl_map() else {
        todo!()
    };

    let scope = impl_map.import_scope();
    let library = scope.name();
    let flags = impl_map.flags();

    let abi = if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
        None
    } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
        Some("C")
    } else {
        todo!()
    };

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    quote! {
        #(#custom_attrs)*
        #[library(#library)]
        extern #abi fn #name(#(#params),*) #return_type;
    }
}
