use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemStruct};
use winmd::*;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    let (_dependencies, namespaces) = parse_import_stream(stream);

    let reader = &TypeReader::from_os();

    let mut limits = TypeLimits::default();

    for namespace in namespaces {
        limits.insert(reader, &namespace);
    }

    let stage = TypeStage::from_limits(reader, &limits);
    let tree = stage.into_tree();
    let stream = tree.to_tokens();
    //println!("{}", stream.to_string());
    stream.into()
}

#[proc_macro_attribute]
pub fn class(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn value_type(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    // TODO: if the struct is non-blittable then we need to generate a custom RuntimeType that ensures
    // that a POD is returned across the ABI.

    let output = quote! {
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        #input
        impl winrt::RuntimeType for #name {
            type Abi = Self;

            fn abi(&self) -> Self::Abi {
                *self
            }

            fn set_abi(&mut self) -> *mut Self::Abi {
                self as *mut Self::Abi
            }
        }

    };

    output.into()
}

#[derive(PartialEq)]
enum ImportCategory {
    None,
    Dependency,
    Namespace,
}

/// Parse `import!` macro and return a set of paths to dependencies and
/// a set to all the namespaces referenced
fn parse_import_stream(stream: TokenStream) -> (BTreeSet<PathBuf>, BTreeSet<String>) {
    let mut category = ImportCategory::None;
    let mut dependencies = BTreeSet::<PathBuf>::new();
    let mut modules = BTreeSet::<String>::new();
    let mut stream = stream.into_iter().peekable();

    while let Some(token) = stream.next() {
        match token {
            TokenTree::Ident(value) => {
                match value.to_string().as_ref() {
                    "dependencies" => category = ImportCategory::Dependency,
                    "modules" => category = ImportCategory::Namespace,
                    value => panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value),
                }
                if let Some(TokenTree::Punct(p)) = stream.peek() {
                    if p.as_char() == ':' {
                        let _ = stream.next();
                    }
                }
            }
            TokenTree::Literal(value) => match category {
                ImportCategory::None => panic!(
                    "winrt::import macro expects either `dependencies` or `modules` but found `{}`",
                    value
                ),
                ImportCategory::Dependency => {
                    dependencies.append(&mut to_dependencies(value.to_string().trim_matches('"')));
                }
                ImportCategory::Namespace => {
                    modules.insert(namespace_literal_to_rough_namespace(&value.to_string()));
                }
            },
            _ => panic!(
                "winrt::import macro encountered an unrecognized token: {}",
                token
            ),
        }
    }

    (dependencies, modules)
}

/// Returns the paths to resolved dependencies
fn to_dependencies<P: AsRef<Path>>(dependency: P) -> BTreeSet<PathBuf> {
    let path = dependency.as_ref();
    let mut result = BTreeSet::new();

    if path.is_dir() {
        let paths = std::fs::read_dir(path).unwrap_or_else(|e| {
            panic!(
                "Could not read dependecy directory at path {:?}: {}",
                path, e
            )
        });
        for path in paths {
            if let Ok(path) = path {
                let path = path.path();
                if path.is_file() {
                    result.insert(path);
                }
            }
        }
    } else if path.is_file() {
        result.insert(path.to_path_buf());
    } else if path.to_str().map(|p| p == "os").unwrap_or(false) {
        let mut path = PathBuf::new();
        let wind_dir_env = std::env::var("windir")
            .unwrap_or_else(|_| panic!("No `windir` environment variable found"));
        path.push(wind_dir_env);
        path.push(SYSTEM32);
        path.push("winmetadata");
        result.append(&mut to_dependencies(path));
    } else {
        panic!("Dependency {:?} is not a file or directory", path);
    }

    result
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::with_capacity(namespace.len());
    for c in namespace.chars() {
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
