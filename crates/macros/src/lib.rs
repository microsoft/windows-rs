mod build_limits;
mod implement;
mod implement_tree;
mod windows;

use build_limits::*;
use implement_tree::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// A macro for generating WinRT modules to a .rs file at build time.
///
/// This macro can be used to import WinRT APIs from any Windows metadata (winmd) file.
/// It is only intended for use from a crate's build.rs script.
///
/// The macro generates a single `build` function which can be used in build scripts
/// to generate the WinRT bindings. After using the `build` macro, call the
/// generated `build` function somewhere in the build.rs script's main function.
///
/// # Usage
/// To use, you must then specify which types you want to use. These
/// follow the same convention as Rust `use` paths. Types know which other types they depend on so
/// `build` will generate any other WinRT types needed for the specified type to work.
///
/// # Example
/// The following `build!` generates all types inside of the `microsoft::ai::machine_learning`
/// namespace.
///
/// ```rust,ignore
/// build!(
///     microsoft::ai::machine_learning::*
/// );
/// ```
#[proc_macro]
pub fn build(stream: TokenStream) -> TokenStream {
    let build = parse_macro_input!(stream as BuildLimits);

    let tokens = match build.to_tokens_string() {
        Ok(t) => t,
        Err(t) => return t.into(),
    };

    let workspace_dir = ::winrt_gen::workspace_dir();

    let mut source = ::winrt_gen::build_windows_dir_at_workspace(&workspace_dir);
    source.push(ARCHITECTURE);
    let source = source.to_str().expect("Invalid build windows dir");

    let destination = ::winrt_gen::build_target_dir_at_at_workspace(&workspace_dir);
    let destination = destination.to_str().expect("Invalid build target dir");

    let tokens = quote! {
        {
            println!("cargo:rerun-if-changed=.windows");

            // The following must be injected into the token stream because the `OUT_DIR` and `PROFILE`
            // environment variables are only set when the build script run and not when it is being compiled.

            use ::std::io::Write;
            let mut path = ::std::path::PathBuf::from(
                ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
            );

            path.push("winrt.rs");
            let mut file = ::std::fs::File::create(&path).expect("Failed to create winrt.rs");
            file.write_all(#tokens.as_bytes()).expect("Could not write generated code to output file");

            let mut cmd = ::std::process::Command::new("rustfmt");
            cmd.arg(&path);
            let _ = cmd.output();

            fn copy(source: &::std::path::PathBuf, destination: &mut ::std::path::PathBuf) {
                if let ::std::result::Result::Ok(files) = ::std::fs::read_dir(source) {
                    for file in files.filter_map(|file| file.ok())  {
                        if let ::std::result::Result::Ok(file_type) = file.file_type() {
                            if file_type.is_file() {
                                let path = file.path();
                                if let ::std::option::Option::Some(filename) = path.file_name() {
                                    destination.push(filename);
                                    let _ = ::std::fs::copy(path, &destination);
                                    destination.pop();
                                }
                            }
                        }
                    }
                }
            }

            fn copy_to_profile(source: &::std::path::PathBuf, destination: &::std::path::PathBuf, profile: &str) {
                if let ::std::result::Result::Ok(files) = ::std::fs::read_dir(destination) {
                    for file in files.filter_map(|file| file.ok())  {
                        if let ::std::result::Result::Ok(file_type) = file.file_type() {
                            if file_type.is_dir() {
                                let mut path = file.path();
                                if let ::std::option::Option::Some(filename) = path.file_name() {
                                    if filename == profile {
                                        copy(source, &mut path);
                                    } else {
                                        copy_to_profile(source, &path, profile);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let profile = ::std::env::var("PROFILE").expect("No `PROFILE` env variable set");

            let source = ::std::path::PathBuf::from(#source);
            let destination = ::std::path::PathBuf::from(#destination);

            copy_to_profile(&source, &destination, &profile);

        }
    };
    tokens.into()
}

/// Rust structs can use the `implement` macro to implement entire WinRT classes or
/// any combination of existing COM and WinRT interfaces.
///
/// If the attribute `TokenStream` contains the name of a WinRT class then all of its
/// interfaces are implemented. Otherwise, whatever interfaces are contained within
/// the attribute TokenStream are implemented.
#[proc_macro_attribute]
pub fn implement(attribute: TokenStream, input: TokenStream) -> TokenStream {
    implement::gen(attribute, input)
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
pub(crate) fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::with_capacity(namespace.len());
    for c in namespace.chars() {
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}

#[cfg(target_arch = "x86_64")]
const ARCHITECTURE: &str = "x64";
#[cfg(target_arch = "x86")]
const ARCHITECTURE: &str = "x86";
#[cfg(target_arch = "arm")]
const ARCHITECTURE: &str = "arm";
#[cfg(target_arch = "aarch64")]
const ARCHITECTURE: &str = "arm64";
