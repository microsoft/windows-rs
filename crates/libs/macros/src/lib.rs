// mod build_macro;
// mod implement;
mod implement2;
//mod implement_macro;

//use build_macro::*;
//use gen::*;l
//use implement_macro::*;
//use quote::*;
//use reader::*;
//use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn implement(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implement2::gen(attribute, input)
}

// TODO: get rid of all the remaining macros below

// #[proc_macro]
// pub fn build(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     parse_macro_input!(stream as BuildMacro);
//     gen_build().as_str().parse().unwrap()
// }

// #[doc(hidden)]
// #[proc_macro]
// pub fn build_legacy(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     parse_macro_input!(stream as BuildMacro);
//     gen_build_legacy().as_str().parse().unwrap()
// }

// #[proc_macro]
// pub fn generate(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     parse_macro_input!(stream as BuildMacro);

//     let mut tokens = String::new();
//     tokens.push_str("r#\"");
//     tokens.push_str(&gen_source_tree().into_string());
//     tokens.push_str("\"#");
//     tokens.parse().unwrap()
// }

// #[proc_macro_attribute]
// pub fn implement(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     implement::gen(attribute, input)
// }

// #[proc_macro]
// pub fn include_bindings(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     r#"::std::include!(::std::concat!(::std::env!("OUT_DIR"), "/windows.rs"));"#.parse().unwrap()
// }
