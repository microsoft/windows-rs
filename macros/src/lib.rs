extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Stringable)]
pub fn to_string_derive(input: TokenStream) -> TokenStream{
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    TokenStream::new()
}