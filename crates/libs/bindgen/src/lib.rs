mod r#async;
mod callbacks;
mod cfg;
mod classes;
mod constants;
mod delegates;
mod enums;
mod extensions;
mod functions;
mod gen;
mod handles;
mod helpers;
mod interfaces;
mod iterator;
mod methods;
mod names;
mod replacements;
mod signatures;
mod structs;

use cfg::*;
use functions::*;
pub use gen::*;
use helpers::*;
use iterator::*;
use methods::*;
use names::*;
use quote::*;
use r#async::*;
use reader::*;
use signatures::*;

pub fn gen_type(name: &str, gen: &Gen) -> String {
    let reader = TypeReader::get();
    let mut tokens = String::new();

    for def in reader.get_type_entry(TypeName::parse(name)).iter().flat_map(|entry| entry.def.iter()) {
        tokens.push_str(gen_element_type(def, gen).as_str());
    }

    assert!(!tokens.is_empty(), "`{}` not found", name);
    tokens
}

pub fn gen_namespace(gen: &Gen) -> String {
    let tree = TypeReader::get().get_namespace(gen.namespace).expect("Namespace not found");

    let namespaces = tree.namespaces.iter().map(move |(name, tree)| {
        if tree.namespace == "Windows.Win32.Interop" {
            return quote! {};
        }

        let name = gen_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        quote! {
            #[cfg(feature = #namespace)] pub mod #name;
        }
    });

    let functions = gen_sys_functions(tree, gen);
    let types = gen_non_sys_function_types(tree, gen);

    let tokens = quote! {
        #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
        #(#namespaces)*
        #functions
        #types
    };

    tokens.into_string()
}

fn gen_non_sys_function_types(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for entry in tree.types.values() {
        for def in &entry.def {
            tokens.combine(&gen_element_type(def, gen));
        }
    }

    tokens
}

fn gen_element_type(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::Field(def) => constants::gen(def, gen),
        ElementType::TypeDef(def) => {
            let def = &def.clone().with_generics();
            match def.kind() {
                TypeKind::Class => classes::gen(def, gen),
                TypeKind::Interface => interfaces::gen(def, gen),
                TypeKind::Enum => enums::gen(def, gen),
                TypeKind::Struct => structs::gen(def, gen),
                TypeKind::Delegate => {
                    if def.is_winrt() {
                        delegates::gen(def, gen)
                    } else {
                        callbacks::gen(def, gen)
                    }
                }
            }
        }
        ElementType::MethodDef(def) => {
            if !gen.sys {
                gen_function(def, gen)
            } else {
                quote! {}
            }
        }
        _ => quote! {},
    }
}
