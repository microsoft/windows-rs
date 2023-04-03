/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

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
mod structs;
mod winrt_methods;
use metadata::reader::*;
use method_names::*;
use std::collections::*;
use std::fmt::Write;
use tokens::*;
mod standalone;
pub use standalone::*;

#[doc(hidden)]
pub use gen::*;

#[doc(hidden)]
pub fn namespace(gen: &Gen, tree: &Tree) -> String {
    let mut tokens = TokenStream::new();

    if tree.namespace == "Windows" || !tree.namespace.starts_with("Windows.") {
        tokens.combine(&allow());
    }

    for (name, tree) in &tree.nested {
        let name = to_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        if gen.cfg {
            tokens.combine(&quote! { #[cfg(feature = #namespace)] });
        }
        tokens.combine(&quote! { pub mod #name; });
    }

    let mut functions = BTreeMap::<&str, TokenStream>::new();
    let mut types = BTreeMap::<TypeKind, BTreeMap<&str, TokenStream>>::new();

    for def in gen
        .reader
        .namespace_types(tree.namespace, &Default::default())
    {
        let type_name = gen.reader.type_def_type_name(def);
        if REMAP_TYPES.iter().any(|(x, _)| x == &type_name) {
            continue;
        }
        if CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
            continue;
        }
        let name = type_name.name;
        let kind = gen.reader.type_def_kind(def);
        match kind {
            TypeKind::Class => {
                if gen
                    .reader
                    .type_def_flags(def)
                    .contains(TypeAttributes::WINRT)
                {
                    types
                        .entry(kind)
                        .or_default()
                        .insert(name, classes::gen(gen, def));
                } else {
                    for method in gen.reader.type_def_methods(def) {
                        let name = gen.reader.method_def_name(method);
                        functions
                            .entry(name)
                            .or_default()
                            .combine(&functions::gen(gen, method));
                    }
                    for field in gen.reader.type_def_fields(def) {
                        let name = gen.reader.field_name(field);
                        types
                            .entry(kind)
                            .or_default()
                            .entry(name)
                            .or_default()
                            .combine(&constants::gen(gen, field));
                    }
                }
            }
            TypeKind::Interface => types
                .entry(kind)
                .or_default()
                .entry(name)
                .or_default()
                .combine(&interfaces::gen(gen, def)),
            TypeKind::Enum => types
                .entry(kind)
                .or_default()
                .entry(name)
                .or_default()
                .combine(&enums::gen(gen, def)),
            TypeKind::Struct => {
                if gen.reader.type_def_fields(def).next().is_none() {
                    if let Some(guid) = gen.reader.type_def_guid(def) {
                        let ident = to_ident(name);
                        let value = gen.guid(&guid);
                        let guid = gen.type_name(&Type::GUID);
                        let cfg = gen.reader.type_def_cfg(def, &[]);
                        let doc = gen.cfg_doc(&cfg);
                        let constant = quote! {
                            #doc
                            pub const #ident: #guid = #value;
                        };
                        types
                            .entry(TypeKind::Class)
                            .or_default()
                            .entry(name)
                            .or_default()
                            .combine(&constant);
                        continue;
                    }
                }
                types
                    .entry(kind)
                    .or_default()
                    .entry(name)
                    .or_default()
                    .combine(&structs::gen(gen, def));
            }
            TypeKind::Delegate => types
                .entry(kind)
                .or_default()
                .entry(name)
                .or_default()
                .combine(&delegates::gen(gen, def)),
        }
    }

    for function in functions.values() {
        tokens.combine(function);
    }

    for ty in types.values().flat_map(|v| v.values()) {
        tokens.combine(ty);
    }

    tokens.combine(&extensions::gen_mod(gen, tree.namespace));
    tokens.into_string()
}

#[doc(hidden)]
pub fn namespace_impl(gen: &Gen, tree: &Tree) -> String {
    let mut types = BTreeMap::<&str, TokenStream>::new();

    for def in gen
        .reader
        .namespace_types(tree.namespace, &Default::default())
    {
        let type_name = gen.reader.type_def_type_name(def);
        if CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
            continue;
        }
        if gen.reader.type_def_kind(def) != TypeKind::Interface {
            continue;
        }
        let tokens = implements::gen(gen, def);

        if !tokens.is_empty() {
            types.insert(type_name.name, tokens);
        }
    }

    let types = types.values();

    let mut tokens = quote! {
        #(#types)*
    };

    tokens.combine(&extensions::gen_impl(tree.namespace));
    tokens.into_string()
}

/// Generates bindings for a specific component namespace.
pub fn component(namespace: &str, files: &[File]) -> String {
    let reader = &Reader::new(files);
    let tree = reader.tree(namespace, &Default::default());
    let mut gen = Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.component = true;
    let mut bindings = crate::namespace(&gen, &tree);
    bindings.push_str(&namespace_impl(&gen, &tree));
    bindings
}

fn allow() -> TokenStream {
    quote! {
        #![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
    }
}

/// Expand a possibly empty generics list with a new generic
fn expand_generics(generics: TokenStream, new: TokenStream) -> TokenStream {
    if generics.is_empty() {
        quote!(#new)
    } else {
        quote!(#generics, #new)
    }
}

/// Expand a possibly emppty where clause with a new generic constraint
fn expand_where_clause(where_clause: TokenStream, generic: TokenStream) -> TokenStream {
    if where_clause.is_empty() {
        quote!(where #generic)
    } else {
        quote!(#where_clause #generic)
    }
}
