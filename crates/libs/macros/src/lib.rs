mod build_macro;
mod implement;
mod implement_macro;

use build_macro::*;
use gen::*;
use implement_macro::*;
use quote::*;
use reader::*;
use syn::parse_macro_input;

/// A macro for generating Windows API bindings to a .rs file at build time.
///
/// This macro can be used to import Windows APIs from any Windows metadata (winmd) file.
/// It is only intended for use from a crate's build.rs script.
///
/// The macro generates a single `build` function which can be used in build scripts
/// to generate the Windows bindings. After using the `build` macro, call the
/// generated `build` function somewhere in the build.rs script's main function.
///
/// # Usage
/// To use, you must then specify which types you want to use. These
/// follow the same convention as Rust `use` paths. Types know which other types they depend on so
/// `build` will generate any other Windows types needed for the specified type to work.
///
/// # Example
/// The following `build!` generates all types inside of the `Microsoft::AI::MachineLearning`
/// namespace.
///
/// ```rust,ignore
/// build!(
///     Microsoft::AI::MachineLearning::*
/// );
/// ```
#[proc_macro]
pub fn build(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(stream as BuildMacro);
    gen_build().as_str().parse().unwrap()
}

#[doc(hidden)]
#[proc_macro]
pub fn build_legacy(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(stream as BuildMacro);
    gen_build_legacy().as_str().parse().unwrap()
}

/// A macro for generating Windows API bindings ahead of time.
#[proc_macro]
pub fn generate(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(stream as BuildMacro);

    let mut tokens = String::new();
    tokens.push_str("r#\"");
    tokens.push_str(&gen_source_tree().into_string());
    tokens.push_str("\"#");
    tokens.parse().unwrap()
}

/// Rust structs can use the [`macro@implement`] attribute macro to implement entire WinRT or COM
/// classes or any combination of existing COM and WinRT interfaces.
///
/// If the attribute [`proc_macro::TokenStream`] contains the name of a WinRT class then all
/// of its interfaces are implemented. Otherwise, whatever interfaces are contained within
/// the attribute TokenStream are implemented.
#[proc_macro_attribute]
pub fn implement(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implement::gen(attribute, input)
}

/// Includes the generated bindings into the current context.
#[proc_macro]
pub fn include_bindings(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO: check that input stream is empty
    r#"::std::include!(::std::concat!(::std::env!("OUT_DIR"), "/windows.rs"));"#.parse().unwrap()
}
