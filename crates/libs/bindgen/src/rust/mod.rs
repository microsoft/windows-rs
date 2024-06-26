mod cfg;
mod classes;
mod com_methods;
mod constants;
mod delegates;
mod enums;
mod extensions;
mod functions;
mod handles;
mod implements;
mod index;
mod interfaces;
mod iterators;
mod method_names;
mod standalone;
mod structs;
mod try_format;
mod winrt_methods;
mod writer;

use super::*;
use index::*;
use rayon::prelude::*;
use writer::*;

pub fn from_reader(
    reader: &'static metadata::Reader,
    mut config: std::collections::BTreeMap<&str, &str>,
    output: &str,
) -> Result<()> {
    let mut writer = Writer::new(reader, output);
    writer.package = config.remove("package").is_some();
    writer.flatten = config.remove("flatten").is_some();
    writer.sys = config.remove("sys").is_some();
    writer.implement = config.remove("implement").is_some();
    writer.minimal = config.remove("minimal").is_some();
    writer.no_inner_attributes = config.remove("no-inner-attributes").is_some();
    writer.no_bindgen_comment = config.remove("no-bindgen-comment").is_some();
    writer.vtbl = config.remove("vtbl").is_some();
    writer.rustfmt_config = if let Some(config) = config.remove("rustfmt-config") {
        config.to_string()
    } else {
        String::new()
    };

    if writer.package && writer.flatten {
        return Err(Error::new(
            "cannot combine `package` and `flatten` configuration values",
        ));
    }

    if writer.implement && writer.sys {
        return Err(Error::new(
            "cannot combine `implement` and `sys` configuration values",
        ));
    }

    config.retain(|key, value| {
        if let Some(full_name) = key.strip_prefix("prepend:") {
            if let Some(index) = full_name.rfind('.') {
                let namespace = &full_name[0..index];
                let name = &full_name[index + 1..];
                if let Some(type_def) = reader.get_type_def(namespace, name).next() {
                    writer.prepend.insert(type_def, value.to_string());
                    return false;
                }
            }
        }
        true
    });

    if let Some((key, _)) = config.first_key_value() {
        return Err(Error::new(&format!("invalid configuration value `{key}`")));
    }

    if writer.package {
        gen_package(&writer)
    } else {
        gen_file(&writer)
    }
}

fn gen_file(writer: &Writer) -> Result<()> {
    // TODO: harmonize this output code so we don't need these two wildly differnt code paths
    // there should be a simple way to generate the with or without namespaces.

    if writer.flatten {
        let tokens = standalone::standalone_imp(writer);
        write_to_file(&writer.output, try_format(writer, &tokens))
    } else {
        let mut tokens = String::new();
        let root = Tree::new(writer.reader);

        for tree in root.nested.values() {
            tokens.push_str(&namespace(writer, tree));
        }

        write_to_file(&writer.output, try_format(writer, &tokens))
    }
}

fn gen_package(writer: &Writer) -> Result<()> {
    let directory = directory(&writer.output);
    let root = Tree::new(writer.reader);
    let mut root_len = 0;

    for tree in root.nested.values() {
        root_len = tree.namespace.len();
        _ = std::fs::remove_dir_all(format!("{directory}/{}", tree.namespace));
    }

    let trees = root.flatten();

    trees.par_iter().try_for_each(|tree| {
        let directory = format!("{directory}/{}", tree.namespace.replace('.', "/"));
        let mut tokens = namespace(writer, tree);

        let tokens_impl = if !writer.sys {
            namespace_impl(writer, tree)
        } else {
            String::new()
        };

        if !writer.sys && !tokens_impl.is_empty() {
            tokens.push_str("#[cfg(feature = \"implement\")]\ncore::include!(\"impl.rs\");\n");
        }

        let output = format!("{directory}/mod.rs");
        write_to_file(&output, try_format(writer, &tokens))?;

        if !writer.sys && !tokens_impl.is_empty() {
            let output = format!("{directory}/impl.rs");
            write_to_file(&output, try_format(writer, &tokens_impl))?;
        }

        Ok::<(), Error>(())
    })?;

    let package_root = super::directory(directory);
    let cargo_toml = format!("{package_root}/Cargo.toml");
    let mut toml = String::new();

    for line in read_file_lines(&cargo_toml)? {
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
        } else if tree.namespace.starts_with("Windows.Win32")
            || tree.namespace.starts_with("Windows.Wdk")
        {
            toml.push_str(&format!("{feature} = [\"Win32_Foundation\"]\n"));
        } else if tree.namespace != "Windows.Foundation" {
            toml.push_str(&format!("{feature} = [\"Foundation\"]\n"));
        } else {
            toml.push_str(&format!("{feature} = []\n"));
        }
    }

    write_to_file(&cargo_toml, toml)?;
    write_to_file(&format!("{package_root}/features.json"), gen_index(writer))
}

use method_names::*;
use std::fmt::Write;
use tokens::*;
use try_format::*;

fn namespace(writer: &Writer, tree: &Tree) -> String {
    let writer = &mut writer.clone();
    writer.namespace = tree.namespace;
    let mut tokens = TokenStream::new();

    for (name, tree) in &tree.nested {
        let name = to_ident(name);
        let feature = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        if writer.package {
            tokens.combine(&quote! {
                #[cfg(feature = #feature)]
                pub mod #name;
            });
        } else {
            tokens.combine(&quote! { pub mod #name });
            tokens.push_str("{");
            tokens.push_str(&namespace(writer, tree));
            tokens.push_str("}");
        }
    }

    let mut functions = std::collections::BTreeMap::<&str, TokenStream>::new();
    let mut types = std::collections::BTreeMap::<
        metadata::TypeKind,
        std::collections::BTreeMap<&str, TokenStream>,
    >::new();

    for item in writer.reader.namespace_items(writer.namespace) {
        match item {
            metadata::Item::Type(def) => {
                let type_name = def.type_name();
                if writer.reader.remap_type(type_name).is_some() {
                    continue;
                }
                if writer.reader.core_type(type_name).is_some() {
                    continue;
                }
                types
                    .entry(def.kind())
                    .or_default()
                    .entry(type_name.name())
                    .or_default()
                    .combine(&writer.type_def(def));
            }
            metadata::Item::Fn(def, namespace) => {
                let name = def.name();
                functions
                    .entry(name)
                    .or_default()
                    .combine(&functions::writer(writer, namespace, def));
            }
            metadata::Item::Const(def) => {
                let name = def.name();
                types
                    .entry(metadata::TypeKind::Class)
                    .or_default()
                    .entry(name)
                    .or_default()
                    .combine(&constants::writer(writer, def));
            }
        }
    }

    for function in functions.values() {
        tokens.combine(function);
    }

    for ty in types.values().flat_map(|v| v.values()) {
        tokens.combine(ty);
    }

    tokens.combine(&extensions::gen_mod(writer, tree.namespace));

    if writer.implement {
        tokens.push_str(&namespace_impl(writer, tree));
    }

    tokens.into_string()
}

fn namespace_impl(writer: &Writer, tree: &Tree) -> String {
    let writer = &mut writer.clone();
    writer.namespace = tree.namespace;
    let mut types = std::collections::BTreeMap::new();

    for item in writer.reader.namespace_items(tree.namespace) {
        if let metadata::Item::Type(def) = item {
            let type_name = def.type_name();
            if writer.reader.core_type(type_name).is_some() {
                continue;
            }
            if def.kind() != metadata::TypeKind::Interface {
                continue;
            }
            let tokens = implements::writer(writer, def);

            if !tokens.is_empty() {
                types.insert(type_name.name(), tokens);
            }
        }
    }

    let types = types.values();

    let mut tokens = quote! {
        #(#types)*
    };

    tokens.combine(&extensions::gen_impl(tree.namespace));
    tokens.into_string()
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
