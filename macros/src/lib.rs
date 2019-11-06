extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Stringable)]
pub fn gen_to_string(input: TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote!{
        impl Stringable for #name {
            fn to_string() {
                println!("Hello macro {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}