extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::iter::FromIterator;

#[proc_macro]
pub fn table(stream: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(stream as syn::Ident);
    let table = format_ident!("TABLE_{}", name.to_string().to_uppercase());

    let output = quote!(
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
        pub struct #name {
            pub(crate) row: RowData,
        }
        impl Row for #name {
            fn new(row:u32, file:u16) -> Self {
                Self { row: RowData::new(row, #table as u16, file ) }
            }
            fn table() -> u16 {
                #table as u16
            }
        }
    );

    output.into()
}

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

        if let Some((_, syn::Expr::Lit(value))) = &variant.discriminant {
            if let syn::Lit::Int(value) = &value.lit {
                enumerator = value.base10_parse::<u32>().unwrap();
            }
        }

        variants.push(quote!(
            #name(#name),
        ));

        decodes.push(quote!(
            #enumerator => Self::#name(#name::new(code.1, file)),
        ));

        encodes.push(quote!(
            Self::#name(value) => ((value.row.row + 1) << #bits) | #enumerator,
        ));

        enumerator += 1;
    }

    let variants = TokenStream2::from_iter(variants);
    let decodes = TokenStream2::from_iter(decodes);
    let encodes = TokenStream2::from_iter(encodes);

    let output = quote!(
        pub enum #name {
            #variants
        }
        impl Code for #name {
            fn decode(code: u32, file: u16) -> Self {
                let code = (code & ((1 << #bits) - 1), (code >> #bits) - 1);
                match code.0 {
                    #decodes
                    _ => panic!(),
                }
            }
            fn encode(&self) -> u32 {
                match self {
                    #encodes
                }
            }
        }
    );

    output.into()
}
