mod build_limits;
mod implement;
mod implement_tree;
mod symbols;

use build_limits::*;
use gen::*;
use implement_tree::*;
use symbols::*;
use syn::parse_macro_input;

struct RawString(String);

impl ToTokens for RawString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.push_str("r#\"");
        tokens.push_str(&self.0);
        tokens.push_str("\"#");
    }
}

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
    let build = parse_macro_input!(stream as BuildLimits);

    let tokens = RawString(build.into_tokens_string());
    let workspace_windows_dir = gen::workspace_windows_dir();

    let mut destination = workspace_windows_dir.clone();
    destination.pop();
    destination.push("target");
    let destination = RawString(
        destination
            .to_str()
            .expect("Invalid workspace target dir")
            .to_string(),
    );

    let workspace_windows_dir = RawString(
        workspace_windows_dir
            .to_str()
            .expect("Invalid workspace windows dir")
            .to_string(),
    );

    let tokens = quote! {
        {
            // The following must be injected into the token stream because the `OUT_DIR` and `PROFILE`
            // environment variables are only set when the build script run and not when it is being compiled.

            use ::std::io::Write;
            let mut path = ::std::path::PathBuf::from(
                ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
            );

            path.push("windows.rs");
            let mut file = ::std::fs::File::create(&path).expect("Failed to create windows.rs");
            file.write_all(#tokens.as_bytes()).expect("Could not write generated code to output file");

            let mut cmd = ::std::process::Command::new("rustfmt");
            cmd.arg(&path);
            let _ = cmd.output();

            fn copy(source: &::std::path::Path, destination: &mut ::std::path::PathBuf) {
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

            fn copy_to_profile(source: &::std::path::Path, destination: &::std::path::Path, profile: &str) {
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

            if ::std::path::PathBuf::from(#workspace_windows_dir).exists() {
                println!("cargo:rerun-if-changed={}", #workspace_windows_dir);
                let mut source = ::std::path::PathBuf::from(#workspace_windows_dir);

                // The `target_arch` cfg is not set for build scripts so we need to sniff it out from the environment variable.
                source.push(match ::std::env::var("CARGO_CFG_TARGET_ARCH").expect("No `CARGO_CFG_TARGET_ARCH` env variable set").as_str() {
                    "x86_64" => "x64",
                    "x86" => "x86",
                    "arm" => "arm",
                    "aarch64" => "arm64",
                    unexpected => panic!("Unexpected `{}` architecture set by `CARGO_CFG_TARGET_ARCH`", unexpected),
                });

                let destination = ::std::path::PathBuf::from(#destination);
                let profile = ::std::env::var("PROFILE").expect("No `PROFILE` env variable set");
                copy_to_profile(&source, &destination, &profile);
            }
        }
    };

    tokens.as_str().parse().unwrap()
}

#[proc_macro]
pub fn generate(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let build = parse_macro_input!(stream as BuildLimits);

    let mut tokens = String::new();
    tokens.push_str("r#\"");
    tokens.push_str(&build.into_tokens_string());
    tokens.push_str("\"#");
    tokens.parse().unwrap()
}

/// Rust structs can use the `implement` macro to implement entire WinRT classes or
/// any combination of existing COM and WinRT interfaces.
///
/// If the attribute `TokenStream` contains the name of a WinRT class then all of its
/// interfaces are implemented. Otherwise, whatever interfaces are contained within
/// the attribute TokenStream are implemented.
#[proc_macro_attribute]
pub fn implement(
    attribute: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    implement::gen(attribute, input)
}
