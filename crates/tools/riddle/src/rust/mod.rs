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
mod standalone;
mod structs;
mod try_format;
mod winrt_methods;
use crate::{Error, Result, Tree};
use rayon::prelude::*;

pub fn from_reader(
    reader: &metadata::Reader,
    filter: &metadata::Filter,
    mut config: std::collections::BTreeMap<&str, &str>,
    output: &str,
) -> Result<()> {
    let mut gen = Gen::new(reader, filter, output);
    gen.package = config.remove("PACKAGE").is_some();
    gen.flatten = config.remove("FLATTEN").is_some();
    gen.std = config.remove("STD").is_some();
    gen.sys = gen.std || config.remove("SYS").is_some();
    gen.implement = config.remove("IMPLEMENT").is_some();
    gen.minimal = config.remove("MINIMAL").is_some();

    // TODO: get rid of this hack so it can work with any metadata
    if gen.flatten {
        gen.namespace = "Windows.";
    }

    if gen.package && gen.flatten {
        return Err(Error::new(
            "cannot combine PACKAGE and FLATTEN configuration values",
        ));
    }

    if gen.implement && gen.sys {
        return Err(Error::new(
            "cannot combine IMPLEMENT and SYS configuration values",
        ));
    }

    if let Some((key, _)) = config.first_key_value() {
        return Err(Error::new(&format!("invalid configuration value: `{key}`")));
    }

    if gen.package {
        gen_package(&gen)
    } else {
        gen_file(&gen)
    }
}

fn gen_file(gen: &Gen) -> Result<()> {
    // TODO: harmonize this output code so we don't need these two wildly differnt code paths
    // there should be a simple way to generate the with or without namespaces.

    if gen.flatten {
        let tokens = standalone::standalone_imp(gen, gen.filter.includes());
        crate::write_to_file(gen.output, try_format(tokens))
    } else {
        let mut tokens = String::new();
        let root = Tree::new(gen.reader, gen.filter);

        for tree in root.nested.values() {
            tokens.push_str(&namespace(gen, tree));
        }

        crate::write_to_file(gen.output, try_format(tokens))
    }
}

fn gen_package(gen: &Gen) -> Result<()> {
    let directory = crate::directory(gen.output);
    let root = Tree::new(gen.reader, gen.filter);
    let mut root_len = 0;

    for tree in root.nested.values() {
        root_len = tree.namespace.len();
        _ = std::fs::remove_dir_all(format!("{directory}/{}", tree.namespace));
    }

    let trees = root.flatten();

    trees.par_iter().try_for_each(|tree| {
        let directory = format!("{directory}/{}", tree.namespace.replace('.', "/"));
        let mut tokens = namespace(gen, tree);
        if !gen.sys {
            tokens.push_str("#[cfg(feature = \"implement\")]\n::core::include!(\"impl.rs\");\n");
        }
        let output = format!("{directory}/mod.rs");
        crate::write_to_file(&output, try_format(tokens))?;
        if !gen.sys {
            let tokens = namespace_impl(gen, tree);
            let output = format!("{directory}/impl.rs");
            crate::write_to_file(&output, try_format(tokens))?;
        }
        Ok::<(), Error>(())
    })?;

    let cargo_toml = format!("{}/Cargo.toml", crate::directory(directory));
    let mut toml = String::new();

    for line in crate::read_file_lines(&cargo_toml)? {
        toml.push_str(&line);
        toml.push('\n');

        if line == "# generated features" {
            break;
        }
    }

    for tree in trees.iter().skip(1) {
        let feature = tree.namespace[root_len + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            toml.push_str(&format!("{feature} = [\"{dependency}\"]\n"));
        } else {
            toml.push_str(&format!("{feature} = []\n"));
        }
    }

    crate::write_to_file(&cargo_toml, toml)
}

use crate::tokens::*;
use gen::*;
use metadata::*;
use method_names::*;
use std::collections::*;
use std::fmt::Write;
use try_format::*;

fn namespace(gen: &Gen, tree: &Tree) -> String {
    let gen = &mut gen.clone();
    gen.namespace = tree.namespace;
    let mut tokens = TokenStream::new();

    if tree.namespace == "Windows" || !tree.namespace.starts_with("Windows.") {
        tokens.combine(&allow());
    }

    for (name, tree) in &tree.nested {
        let name = to_ident(name);
        let namespace_feature =
            tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        if gen.package {
            tokens.combine(&quote! {
                #[cfg(feature = #namespace_feature)]
                pub mod #name;
            });
        } else {
            tokens.combine(&quote! { pub mod #name });
            tokens.push_str("{");
            tokens.push_str(&namespace(gen, tree));
            tokens.push_str("}");
        }
    }

    let mut functions = BTreeMap::<&str, TokenStream>::new();
    let mut types = BTreeMap::<TypeKind, BTreeMap<&str, TokenStream>>::new();

    for method in gen.reader.namespace_functions(tree.namespace) {
        let name = gen.reader.method_def_name(method);
        functions
            .entry(name)
            .or_default()
            .combine(&functions::gen(gen, method));
    }

    for field in gen.reader.namespace_constants(tree.namespace) {
        let name = gen.reader.field_name(field);
        types
            .entry(TypeKind::Class)
            .or_default()
            .entry(name)
            .or_default()
            .combine(&constants::gen(gen, field));
    }

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
                    .contains(TypeAttributes::WindowsRuntime)
                {
                    types
                        .entry(kind)
                        .or_default()
                        .insert(name, classes::gen(gen, def));
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

    if gen.implement {
        tokens.push_str(&namespace_impl(gen, tree));
    }

    tokens.into_string()
}

fn namespace_impl(gen: &Gen, tree: &Tree) -> String {
    let gen = &mut gen.clone();
    gen.namespace = tree.namespace;
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
