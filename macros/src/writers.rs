use crate::*;
use quote::quote;

pub(crate) fn write_module(scope: &ImportScope, module: &str) -> proc_macro::TokenStream {

    let gen = quote! {

        struct CODE {}

    };

    gen.into()
}
