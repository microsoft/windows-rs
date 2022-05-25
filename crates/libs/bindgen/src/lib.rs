mod classes;
mod com_methods;
mod constants;
mod delegates;
mod enums;
mod extensions;
mod functions;
mod gen;
mod handles;
mod implements;
mod interfaces;
mod iterators;
mod method_names;
mod replacements;
mod structs;
mod winrt_methods;
pub use gen::*;
use metadata::reader::*;
use method_names::*;
use std::collections::*;
use std::fmt::Write;
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
            // TODO: replace with Vec once parity is achieved - BTreeMap just used to make diffing simpler.
            let mut methods = BTreeMap::new();
            for method in gen.reader.type_def_methods(apis) {
                combine(&mut methods, gen.reader.method_def_name(method), functions::gen(gen, method));
            }
            // TODO: instead of Vec just check whether class has `Apis` methods and then pass an iterator?
            if !methods.is_empty() {
                let methods = methods.values();
                functions = vec![quote! {
                    #[link(name = "windows")]
                    extern "system" {
                        #(#methods)*
                    }
                }];
            }
        }
    }

    // TODO: replace with Vec once parity is achieved - BTreeMap just used to make diffing simpler.
    let mut types = BTreeMap::<&str, TokenStream>::new();

    if let Some(namespace_types) = gen.reader.namespace_types(tree.namespace) {
        for def in namespace_types {
            if let Some(tokens) = gen.define(def) {
                combine_type(&mut types, gen.reader.type_def_type_name(def), tokens);
            } else {
                if !gen.sys {
                    for method in gen.reader.type_def_methods(def) {
                        combine(&mut types, gen.reader.method_def_name(method), functions::gen(gen, method));
                    }
                }
                for field in gen.reader.type_def_fields(def) {
                    combine(&mut types, gen.reader.field_name(field), constants::gen(gen, field));
                }
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

pub fn namespace_impl(gen: &Gen, tree: &Tree) -> String {
    let mut types = BTreeMap::<&str, TokenStream>::new();

    if let Some(namespace_types) = gen.reader.namespace_types(tree.namespace) {
        for def in namespace_types {
            combine_type(&mut types, gen.reader.type_def_type_name(def), implements::gen(gen, def));
        }
    }

    let types = types.values();

    let tokens = quote! {
        #(#types)*
    };

    tokens.into_string()
}

fn combine<'a>(types: &mut BTreeMap<&'a str, TokenStream>, name: &'a str, tokens: TokenStream) {
    types.entry(name).or_default().combine(&tokens);
}

fn combine_type<'a>(types: &mut BTreeMap<&'a str, TokenStream>, type_name: TypeName<'a>, tokens: TokenStream) {
    if !CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
        types.entry(type_name.name).or_default().combine(&tokens);
    }
}
