use crate::*;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

pub(crate) fn write_modules(reader: &winmd::Reader, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    write_namespace_set(reader.namespaces(), scope)
}

pub(crate) fn write_namespace_set(namespaces: winmd::NamespaceSet, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let mut tokens = quote! {};

    for namespace in namespaces {
        if scope.contains(namespace.full_name()) {
            let namespace = write_namespace(&namespace, scope);

            tokens = quote! {
                #tokens
                #namespace
            };
        }
    }

    tokens
}

fn write_namespace(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let module = format_ident!("{}", namespace.name().to_lowercase());
    let enums = write_enums(namespace);
    let structs = write_structs(namespace);
    let interfaces = write_interfaces(namespace);
    let classes = write_classes(namespace);
    let namespaces = write_namespace_set(namespace.namespaces(), scope);

    quote! {
        pub mod #module {
            #enums
            #structs
            #interfaces
            #classes
            #namespaces
        }
    }
}

fn write_classes(namespace: &winmd::Namespace) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for t in namespace.classes() {
        let name = format_ident!("{}", t.name());
        tokens = quote! {
            #tokens
            pub struct #name { ptr: *const std::ffi::c_void }
        };
    }

    tokens
}

fn write_interfaces(namespace: &winmd::Namespace) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for interface in namespace.interfaces() {
        let name = interface.name();
        let name_ident = format_ident!("{}", name);
        let abi_name_ident = format_ident!("abi_{}", name);
        let methods = write_abi_methods(&interface);
        tokens = quote! {
            #tokens
            #[repr(C)]
            pub struct #name_ident { ptr: *const std::ffi::c_void }
            #[repr(C)]
            struct #abi_name_ident {
                _0: usize,
                _1: usize,
                _2: usize,
                _3: usize,
                _4: usize,
                _5: usize,
                #methods
            }
        };
    }

    tokens
}

// fn write_consume_methods(interface: &winmd::TypeDef) -> proc_macro2::TokenStream {
// }
// fn write_produce_methods(interface: &winmd::TypeDef) -> proc_macro2::TokenStream {
// }

fn write_abi_methods(interface: &winmd::TypeDef) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for method in interface.methods() {
        let name = format_ident!("{}", method.name());
        let params = write_abi_params(&method.signature());
        tokens = quote! {
            #tokens
            #name: extern "system" fn(*const std::ffi::c_void, #params) -> winrt::ErrorCode,
        };
    }

    tokens
}

fn write_abi_params(signature: &winmd::MethodSig) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for (_, param) in signature.params() {
        tokens = quote! {
            #tokens
            
        };
    }

    if let Some(param) = signature.return_type() {
        tokens = quote! {
            #tokens
            
        };
    }

    tokens
}

// fn write_abi_type_sig(value: &winmd::TypeSig) -> proc_macro2::TokenStream {

// }

fn write_enums(namespace: &winmd::Namespace) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for t in namespace.enums() {
        let name = format_ident!("{}", t.name());
        let fields = write_enum_fields(&t);
        tokens = quote! {
            #tokens
            pub enum #name { #fields }
        };
    }

    tokens
}

fn write_enum_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        for c in f.constants() {
            let name = format_ident!("{}", f.name());
            let value = c.value();

            tokens = quote! {
                #tokens
                #name,
                // TODO: write out the enum value
            };
        }
    }

    tokens
}

fn write_structs(namespace: &winmd::Namespace) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for t in namespace.structs() {
        let name = format_ident!("{}", t.name());
        let fields = write_struct_fields(&t);
        tokens = quote! {
            #tokens
            #[repr(C)]
            #[derive(Default, Debug)]
            pub struct #name { #fields }
        };
    }

    tokens
}

fn write_struct_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        let name = format_ident!("{}", to_snake(f.name()));

        tokens = quote! {
            #tokens
            #name: u32,
            // TODO: write out field type
        };
    }

    tokens
}

fn to_snake(camel: &str) -> String {
    let mut result = String::new();
    for c in camel.chars() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            for c in c.to_lowercase() {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    // TODO: go through all keywords and prepend `r#` if result is keyword
    if result == "type" {
        result.insert_str(0, "r#");
    }

    result
}
