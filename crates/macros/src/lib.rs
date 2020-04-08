use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemStruct};
use winmd::*;

#[derive(PartialEq)]
enum ImportCategory {
    None,
    Dependency,
    Namespace,
}

fn to_dependencies<P: AsRef<std::path::Path>>(dependency: P) -> std::collections::BTreeSet<String> {
    let path = dependency.as_ref();
    let mut result = std::collections::BTreeSet::new();

    if path.is_dir() {
        for path in std::fs::read_dir(path).unwrap() {
            if let Ok(path) = path {
                let path = path.path();
                if path.is_file() {
                    result.insert(path.to_str().unwrap().to_string());
                }
            }
        }
    } else if path.is_file() {
        result.insert(path.to_str().unwrap().to_string());
    } else {
        let path = path.to_str().unwrap();
        if path == "os" {
            let mut path = std::path::PathBuf::new();
            path.push(std::env::var("windir").unwrap());
            path.push(SYSTEM32);
            path.push("winmetadata");
            result.append(&mut to_dependencies(path));
        } else {
            panic!("Dependency {} is not a file or directory", path);
        }
    }

    result
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::new();
    for c in namespace.chars() {
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}

fn parse_import_stream(
    stream: TokenStream,
) -> (
    std::collections::BTreeSet<String>,
    std::collections::BTreeSet<String>,
) {
    let mut category = ImportCategory::None;
    let mut dependencies = std::collections::BTreeSet::<String>::new();
    let mut modules = std::collections::BTreeSet::<String>::new();
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
                    value.to_string()
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
                token.to_string()
            ),
        }
    }

    (dependencies, modules)
}

#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    let (_dependencies, namespaces) = parse_import_stream(stream);

    let winmd_files = load_winmd::from_os();
    let reader = &TypeReader::new(winmd_files);

    let mut limits = TypeLimits::default();

    for namespace in namespaces {
        limits.insert(reader, &namespace);
    }

    let stage = TypeStage::from_limits(reader, &limits);
    let tree = stage.into_tree();
    let stream = tree.to_stream();
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

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
