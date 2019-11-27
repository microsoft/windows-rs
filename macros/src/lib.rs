extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Stringable)]
pub fn gen_to_string(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Stringable for #name {
            fn to_string() {
                println!("Hello {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn replace_your_innards(_args: TokenStream, target: TokenStream) -> TokenStream {
    let value = target.to_string();
    let gen = quote! {
        pub fn change() {
            println!("{}", #value);
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn echo_target(_args: TokenStream, target: TokenStream) -> TokenStream {
    let value = target.to_string();
    println!("{}", value);
    target
}
