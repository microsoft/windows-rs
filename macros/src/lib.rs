// TODO: add panic strings that include some link for help (either to an issue or wiki?)
    // note: for more information, see https://github.com/microsoft/winrt-rs/issues/123

extern crate proc_macro;

mod writers;

use writers::*;
use proc_macro::*;
use quote::quote;
use syn;

#[derive(PartialEq)]
enum ImportCategory {
    None,
    Dependency,
    Module
}

struct Import {
    files: Vec::<String>,
    modules: std::collections::BTreeSet::<String>,
}

fn files<P: AsRef<std::path::Path>>(dependency: P) -> std::collections::BTreeSet::<String> {
    let path = dependency.as_ref();
    let mut result = std::collections::BTreeSet::<String>::new();

    if path.is_dir() {
        for path in std::fs::read_dir(path).unwrap() {
            if let Ok(path) = path {
                let path = path.path();
                if path.is_file() {
                    result.insert(path.to_str().unwrap().to_string());
                }
            }
        }
    }
    else if path.is_file() {
        result.insert(path.to_str().unwrap().to_string());
    } else {
        let path = path.to_str().unwrap();
        if  path == "os" {
            let mut path = std::path::PathBuf::new();
            path.push(std::env::var("windir").unwrap());
            path.push(SYSTEM32);
            path.push("winmetadata");
            result.append(&mut files(path));
        }
        else {
            panic!("Dependency {} is not a file or directory", path);
        }
    }

    result
}

// impl Import {
//     fn new(stream: TokenStream) -> Import {
//         let mut category = ImportCategory::None;
//         let mut dependencies = Vec::<String>::new();
//         let mut modules = Vec::<String>::new();
    
//         for token in stream {
//             match token {
//                 TokenTree::Ident(value) => {
//                     match value.to_string().as_ref() {
//                         "dependencies" => category = ImportCategory::Dependency,
//                         "modules" => category = ImportCategory::Module,
//                         value => panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value),
//                     }
//                 },
//                 TokenTree::Literal(value) => {
//                     match category {
//                         ImportCategory::None => {
//                             panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value.to_string());
//                         },
//                         ImportCategory::Dependency => 
//                             dependencies.push(value.to_string()),
//                         ImportCategory::Module => 
//                             modules.push(value.to_string()),
//                     }
//                 },
//                 _ => panic!("winrt::import macro encountered an unrecognized token: {}", token.to_string())
//             }
//         }
//     }
// }

fn read_import_stream(stream: TokenStream) -> (Vec::<String>, Vec::<String>) {
    let mut category = ImportCategory::None;
    let mut dependencies = Vec::<String>::new();
    let mut modules = Vec::<String>::new();

    for token in stream {
        match token {
            TokenTree::Ident(value) => {
                match value.to_string().as_ref() {
                    "dependencies" => category = ImportCategory::Dependency,
                    "modules" => category = ImportCategory::Module,
                    value => panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value),
                }
            },
            TokenTree::Literal(value) => {
                match category {
                    ImportCategory::None => {
                        panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value.to_string());
                    },
                    ImportCategory::Dependency => 
                        dependencies.push(value.to_string()),
                    ImportCategory::Module => 
                        modules.push(value.to_string()),
                }
            },
            _ => panic!("winrt::import macro encountered an unrecognized token: {}", token.to_string())
        }
    }

    (dependencies, modules)
}

fn produce_output_stream(stream: TokenStream) -> proc_macro2::TokenStream {
    let (dependencies, modules) = read_import_stream(stream);

    for value in dependencies {
        println!("winmd {}", value);
    }

    for value in modules {
        println!("namespace {}", value);
    }

    let reader = winmd::Reader::from_os().unwrap();

    let gen = quote! {

        struct CODE {}

    };

    gen
}

#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {

    let output = produce_output_stream(stream);

    println!("{}", output.to_string());

    let gen = quote! {

pub mod windows {
    pub mod ui {
        #[repr(C)]
        #[derive(Default)]
        pub struct Color {
            pub a: u8,
            pub r: u8,
            pub g: u8,
            pub b: u8,
        }

        pub struct Colors;

        pub struct IColorsStatics {
            ptr: *const std::ffi::c_void,
        }

        #[repr(C)]
        struct abi_IColorsStatics {
            abi_0: usize,
            abi_1: usize,
            abi_2: usize,
            abi_3: usize,
            abi_4: usize,
            abi_5: usize,
            alice_blue: extern "system" fn(*const std::ffi::c_void, &mut Color) -> winrt::ErrorCode,
        }

        impl IColorsStatics {
            pub fn alice_blue(&self) -> winrt::Result<Color> {
                unsafe {
                    let mut color = Default::default();
                    ((*(*(self.ptr as *const *const abi_IColorsStatics))).alice_blue)(
                        self.ptr, &mut color,
                    )
                    .ok_or(color)
                }
            }
        }

        impl Drop for IColorsStatics {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    winrt::abi::IUnknown::release(self.ptr);
                }
            }
        }

        impl Colors {
            pub fn alice_blue() -> winrt::Result<Color> {
                winrt::factory::<Colors, IColorsStatics>()?.alice_blue()
            }
        }

        impl winrt::TypeInterface for IColorsStatics {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    0xCFF52E04,
                    0xCCA6,
                    0x4614,
                    &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99],
                );
                &GUID
            }

            fn take_ownership(ptr: *const std::ffi::c_void) -> Self {
                Self { ptr }
            }
        }

        impl winrt::TypeName for Colors {
            fn type_name() -> &'static str {
                "Windows.UI.Colors"
            }
        }
    }
}
    };

    gen.into()
}

// winrt::import!(
//     winmd
//         os
//         kittens
//     modules
//         kittens
//         windows.storage
//         windows.ui.composition
// )

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
