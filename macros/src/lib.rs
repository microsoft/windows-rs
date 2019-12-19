// TODO: add panic strings that include some link for help (either to an issue or wiki?)
// note: for more information, see https://github.com/microsoft/winrt-rs/issues/123

extern crate proc_macro;

mod writers;

use proc_macro::*;
use std::iter::FromIterator;
use writers::*;

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

// This is to support automatic importing of "windows.ui" when "windows.ui.xaml" is requested
fn to_namespaces(namespace: &str) -> std::collections::BTreeSet<String> {
    let mut result = std::collections::BTreeSet::new();

    let mut namespace = namespace;
    result.insert(namespace.to_string());

    while let Some(index) = namespace.rfind('.') {
        namespace = namespace.get(0..index).unwrap();
        result.insert(namespace.to_string());
    }

    result
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::new();
    for c in namespace.chars() {
        if c == '"' || c == '_' {
            // do nothing
        } else {
            result.push(c);
        }
        // TODO: maybe panic if uppercase char is observed
    }
    result
}

// fn module_to_namespace(reader: &winmd::Reader, module: &str) -> &str {

// }

fn parse_import_stream(stream: TokenStream) -> (winmd::Reader, std::collections::BTreeSet<String>) {
    let mut category = ImportCategory::None;
    let mut dependencies = std::collections::BTreeSet::<String>::new();
    let mut modules = std::collections::BTreeSet::<String>::new();

    for token in stream {
        match token {
            TokenTree::Ident(value) => match value.to_string().as_ref() {
                "dependencies" => category = ImportCategory::Dependency,
                "modules" => category = ImportCategory::Namespace,
                value => panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value),
            },
            TokenTree::Literal(value) => match category {
                ImportCategory::None => panic!("winrt::import macro expects either `dependencies` or `modules` but found `{}`", value.to_string()),

                ImportCategory::Dependency => dependencies.append(&mut to_dependencies(value.to_string().trim_matches('"'))),
                ImportCategory::Namespace => {
                    modules.insert(namespace_literal_to_rough_namespace(&value.to_string()));
                }
            },
            _ => panic!("winrt::import macro encountered an unrecognized token: {}", token.to_string()),
        }
    }

    let reader = winmd::Reader::from_files(&dependencies).unwrap();
    let mut namespaces = std::collections::BTreeSet::<String>::new();

    for module in &modules {
        match reader.find_namespace_lowercase(module) {
            Some(namespace) => namespaces.append(&mut to_namespaces(namespace.full_name())),
            None => panic!("winrt::import macro could not find module `{}` in dependencies", module),
        };
    }

    (reader, namespaces)
}

fn produce_output_stream(stream: TokenStream) -> TokenStream {
    let (reader, namespaces) = parse_import_stream(stream);
    write_modules(&reader, &namespaces).into()
    //let mut result = Vec::<TokenStream>::new();

    // for name in &namespaces {
    //     println!("ns {}", name);
    // }

    //let output = write_modules(&reader, &namespaces);
    //println!("{}", output.to_string());

    // for name in &namespaces {
    //     if let Some(namespace) = reader.find_namespace(name) {
    //         println!("modules {}", name);
    //         result.push(write_namespace(&namespace, &namespaces).into());
    //     }
    // }

    //TokenStream::from_iter(result)
}

#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    let output = produce_output_stream(stream);

    //println!("{}", output.to_string());

    // let gen = quote! {

    // pub mod windows {
    //     pub mod ui {
    //         #[repr(C)]
    //         #[derive(Default, Debug)]
    //         pub struct Color {
    //             pub a: u8,
    //             pub r: u8,
    //             pub g: u8,
    //             pub b: u8,
    //         }

    //         pub struct Colors;

    //         pub struct IColorsStatics {
    //             ptr: *const std::ffi::c_void,
    //         }

    //         #[repr(C)]
    //         struct abi_IColorsStatics {
    //             abi_0: usize,
    //             abi_1: usize,
    //             abi_2: usize,
    //             abi_3: usize,
    //             abi_4: usize,
    //             abi_5: usize,
    //             alice_blue: extern "system" fn(*const std::ffi::c_void, &mut Color) -> winrt::ErrorCode,
    //         }

    //         impl IColorsStatics {
    //             pub fn alice_blue(&self) -> winrt::Result<Color> {
    //                 unsafe {
    //                     let mut color = Default::default();
    //                     ((*(*(self.ptr as *const *const abi_IColorsStatics))).alice_blue)(
    //                         self.ptr, &mut color,
    //                     )
    //                     .ok_or(color)
    //                 }
    //             }
    //         }

    //         impl Drop for IColorsStatics {
    //             fn drop(&mut self) {
    //                 if !self.ptr.is_null() {
    //                     winrt::abi::IUnknown::release(self.ptr);
    //                 }
    //             }
    //         }

    //         impl Colors {
    //             pub fn alice_blue() -> winrt::Result<Color> {
    //                 winrt::factory::<Colors, IColorsStatics>()?.alice_blue()
    //             }
    //         }

    //         impl winrt::TypeInterface for IColorsStatics {
    //             fn type_guid() -> &'static winrt::Guid {
    //                 static GUID: winrt::Guid = winrt::Guid::from_values(
    //                     0xCFF52E04,
    //                     0xCCA6,
    //                     0x4614,
    //                     &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99],
    //                 );
    //                 &GUID
    //             }

    //             fn take_ownership(ptr: *const std::ffi::c_void) -> Self {
    //                 Self { ptr }
    //             }
    //         }

    //         impl winrt::TypeName for Colors {
    //             fn type_name() -> &'static str {
    //                 "Windows.UI.Colors"
    //             }
    //         }
    //     }
    // }
    //     };

    //gen.into()
    output
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
