mod classes;
mod com_methods;
mod constants;
mod delegates;
mod enums;
mod extensions;
mod functions;
mod gen;
mod handles;
mod interfaces;
mod method_names;
mod replacements;
mod structs;
mod winrt_methods;
pub use gen::*;
use metadata::reader2::*;
use method_names::*;
use std::collections::*;
use tokens::*;

pub fn define(gen: &Gen, name: &str) -> String {
    let mut tokens = String::new();
    let type_name = TypeName::parse(name);

    for def in gen.reader.get(type_name) {
        if let Some(def) = gen.define(def) {
            tokens.push_str(def.as_str());
        }
    }

    if tokens.is_empty() {
        if let Some(apis) = gen.reader.get(TypeName::new(type_name.namespace, "Apis")).next() {
            for method in gen.reader.type_def_methods(apis) {
                if gen.reader.method_def_name(method) == type_name.name {
                    tokens.push_str(functions::gen(gen, method).as_str());
                }
            }
            if tokens.is_empty() {
                for field in gen.reader.type_def_fields(apis) {
                    if gen.reader.field_name(field) == type_name.name {
                        tokens.push_str(constants::gen(gen, field).as_str());
                    }
                }
            }
        }
    }

    assert!(!tokens.is_empty(), "`{}` not found", name);
    tokens
}

pub fn namespace(gen: &Gen, tree: &Tree) -> String {
    let namespaces = tree.nested.iter().map(move |(name, tree)| {
        // TODO: make use of EXCLUDE_NAMESPACES somehow
        if tree.namespace == "Windows.Win32.Interop" {
            return quote! {};
        }

        let name = to_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        if gen.cfg {
            quote! {
                #[cfg(feature = #namespace)] pub mod #name;
            }
        } else {
            quote! {
                pub mod #name;
            }
        }
    });

    let mut functions = vec![];

    if gen.sys {
        if let Some(apis) = gen.reader.get(TypeName::new(tree.namespace, "Apis")).next() {
            let mut methods = BTreeMap::new();
            for method in gen.reader.type_def_methods(apis) {
                methods.insert(gen.reader.method_def_name(method), method);
            }
            for method in methods.values() {
                functions.push(functions::gen(gen, *method));
            }

            if !functions.is_empty() {
                functions = vec![quote! {
                    #[link(name = "windows")]
                    extern "system" {
                        #(#functions)*
                    }
                }];
            }
        }
    }

    // TODO: replace with Vec once parity is achieved - BTreeMap just used to make diffing simpler.
    let mut types= BTreeMap::<&str, TokenStream>::new();
    
    for def in gen.reader.namespace_types(tree.namespace) {
        if let Some(tokens) = gen.define(def) {
            types.insert(gen.reader.type_def_name(def), tokens);
        } else {
            if !gen.sys {
                for method in gen.reader.type_def_methods(def) {
                    types.insert(gen.reader.method_def_name(method), functions::gen(gen, method));
                }
            }
            for field in gen.reader.type_def_fields(def) {
                types.insert(gen.reader.field_name(field), constants::gen(gen, field));
            }
        }
    }

    let types = types.values();

    let tokens = quote! {
        #(#namespaces)*
        #(#functions)*
        #(#types)*
    };

    tokens.into_string()
}

pub fn namespace_impl(_gen: &Gen) -> String {
    String::new()
}
