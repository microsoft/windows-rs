extern crate proc_macro;

use proc_macro::*;
use quote::quote;
use syn;

#[proc_macro_derive(Stringable)]
pub fn gen_to_string(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Stringable for #name {
            fn to_string() {
                println!("Hello {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn replace_your_innards(_args: TokenStream, target: TokenStream) -> TokenStream {
    let value = target.to_string();
    let gen = quote! {
        pub fn change() {
            println!("{}", #value);
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn echo_target(_args: TokenStream, target: TokenStream) -> TokenStream {
    let value = target.to_string();
    println!("{}", value);
    target
}

#[derive(PartialEq)]
enum ImportCategory {
    None,
    Dependency,
    Module
}

fn parse_import(stream: TokenStream) -> (Vec::<String>, Vec::<String>) {
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

#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    let (dependencies, modules) = parse_import(stream);

    for value in dependencies {
        println!("winmd {}", value);
    }

    for value in modules {
        println!("namespace {}", value);
    }

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
