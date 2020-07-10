extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::iter::FromIterator;

#[proc_macro_attribute]
pub fn type_code(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    if args.len() != 1 {
        panic!("The `type_code` attribute expects a single integer literal argument");
    }

    let bits = &args[0];
    let name = &input.ident;
    let mut variants = Vec::new();
    let mut decodes = Vec::new();
    let mut encodes = Vec::new();
    let mut enumerator = 0;

    for variant in input.variants.iter() {
        let name = &variant.ident;
        let table = format_ident!("{}", name);

        if let Some((_, syn::Expr::Lit(value))) = &variant.discriminant {
            if let syn::Lit::Int(value) = &value.lit {
                enumerator = value.base10_parse::<u32>().unwrap();
            }
        }

        variants.push(quote!(
            #name(#name),
        ));

        decodes.push(quote!(
            #enumerator => Self::#name(#name(Row::new(code.1, TableIndex::#table, file))),
        ));

        encodes.push(quote!(
            Self::#name(value) => ((value.0.index + 1) << #bits) | #enumerator,
        ));

        enumerator += 1;
    }

    let variants = TokenStream2::from_iter(variants);
    let decodes = TokenStream2::from_iter(decodes);
    let encodes = TokenStream2::from_iter(encodes);

    let output = quote!(
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
        pub enum #name {
            #variants
        }
        impl Decode for #name {
            fn decode(code: u32, file:u16) -> Self {
                let code = (code & ((1 << #bits) - 1), (code >> #bits) - 1);
                match code.0 {
                    #decodes
                    _ => panic!("Failed to decode type code"),
                }
            }
        }
        impl #name {
            pub fn encode(&self) -> u32 {
                match self {
                    #encodes
                }
            }
        }
    );

    output.into()
}
