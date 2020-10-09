mod build;
mod implements;

use build::BuildMacro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// A macro for generating WinRT modules to a .rs file at build time.
///
/// This macro can be used to import WinRT APIs from OS dependencies as well
/// as NuGet packages. It is only intended for use from a crate's build.rs script.
///
/// The macro generates a single `build` function which can be used in build scripts
/// to generate the WinRT bindings. After using the `build` macro, call the
/// generated `build` function somewhere in the build.rs script's main function.
///
/// # Usage
/// To use, first specify which dependencies you are relying on. This can be both
/// `os` for depending on WinRT metadata shipped with Windows or `nuget: My.Package`
/// for NuGet packages.
///
/// ## NuGet
/// NuGet dependencies are expected in a well defined place. The `winmd` metadata files
/// should be in the cargo workspace's `target` directory in a subdirectory `nuget\My.Package`
/// where `My.Package` is the name of the NuGet package.
///
/// Any DLLs needed for the NuGet package to work should be next to work must be next to the final
/// executable.
///
/// Instead of handling this yourself, you can use the [`cargo winrt`](https://github.com/microsoft/winrt-rs/tree/master/crates/cargo)
/// helper subcommand.
///
/// ## Types
/// After specifying the dependencies, you must then specify which types you want to use. These
/// follow the same convention as Rust `use` paths. Types know which other types they depend on so
/// `build` will generate any other WinRT types needed for the specified type to work.
///
/// # Example
/// The following `build!` depends on both `os` metadata (i.e., metadata shipped on Windows 10), as well
/// as a 3rd-party NuGet dependency. It then generates all types inside of the `microsoft::ai::machine_learning`
/// namespace.
///
/// ```rust,ignore
/// build!(
///     dependencies
///         os
///         nuget: Microsoft.AI.MachineLearning
///     types
///         microsoft::ai::machine_learning::*
/// );
/// ```
#[proc_macro]
pub fn build(stream: TokenStream) -> TokenStream {
    let build = parse_macro_input!(stream as BuildMacro);
    let winmd_paths = build.winmd_paths().iter().map(|p| p.display().to_string());

    let change_if = quote! {
        #(println!("cargo:rerun-if-changed={}", #winmd_paths);)*
    };

    let tokens = match build.to_tokens_string() {
        Ok(t) => t,
        Err(t) => return t.into(),
    };

    let tokens = quote! {
        #change_if
        use ::std::io::Write;
        let mut path = ::std::path::PathBuf::from(
            ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
        );

        path.push("winrt.rs");
        let mut file = ::std::fs::File::create(&path).expect("Failed to create winrt.rs");
        file.write_all(#tokens.as_bytes()).expect("Could not write generated code to output file");

        // TODO:make this an opt in with a "feature"?
        //
        // let mut cmd = ::std::process::Command::new("rustfmt");
        // cmd.arg(&path);
        // let output = cmd.output();
        // match output {
        //     Err(_) => eprintln!("Could not execute rustfmt"),
        //     Ok(o) if !o.status.success() => {
        //         let stderr = String::from_utf8_lossy(&o.stderr);
        //         eprintln!("rustfmt did not exit properly: {:?}\n{}", o.status.code(), stderr);
        //     }
        //     _ => {}
        // };
    };
    tokens.into()
}

// Rust structs can use the winrt::implement macro to implement entire WinRT classes or
// any combination of existing COM and WinRT interfaces. If the attribute TokenStream contains
// the name of a WinRT class then all of its interfaces are implemented. Otherwise, whatever
// interfaces are contained within the attribute TokenStream are implemented as a local
// implementation.
#[proc_macro_attribute]
pub fn implements(attribute: TokenStream, input: TokenStream) -> TokenStream {
    implements::gen(attribute, input)
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
pub(crate) fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::with_capacity(namespace.len());
    for c in namespace.chars() {
        // TODO: why '"'?
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}
