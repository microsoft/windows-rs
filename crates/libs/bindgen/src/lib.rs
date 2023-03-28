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
pub use gen::*;
use metadata::reader::*;
use method_names::*;
use std::collections::*;
use std::fmt::Write;
use tokens::*;

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

pub fn standalone(names: &[&str]) -> String {
    let files = &File::with_default(&[]).unwrap();
    let reader = &Reader::new(files);
    let mut gen = &mut Gen::new(reader);
    gen.standalone = true;
    gen.sys = true;
    let mut tokens: TokenStream = format!(
        r#"// Bindings generated by `windows-bindgen` {}

"#,
        std::env!("CARGO_PKG_VERSION")
    )
    .into();

    tokens.combine(&allow());

    tokens.combine(&quote! {
        pub type HRESULT = i32;
        pub type HSTRING = *mut ::core::ffi::c_void;
        pub type IUnknown = *mut ::core::ffi::c_void;
        pub type IInspectable = *mut ::core::ffi::c_void;
        pub type PSTR = *mut u8;
        pub type PWSTR = *mut u16;
        pub type PCSTR = *const u8;
        pub type PCWSTR = *const u16;
        pub type BSTR = *const u16;
        #[repr(C)]
        pub struct GUID {
            pub data1: u32,
            pub data2: u16,
            pub data3: u16,
            pub data4: [u8; 8],
        }
        impl GUID {
            pub const fn from_u128(uuid: u128) -> Self {
                Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
            }
        }
        impl ::core::marker::Copy for GUID {}
        impl ::core::clone::Clone for GUID {
            fn clone(&self) -> Self {
                *self
            }
        }
    });

    for name in names {
        let type_name = TypeName::parse(name);
        let mut found = false;

        for def in reader.get(type_name) {
            found = true;
            let kind = gen.reader.type_def_kind(def);

            match kind {
                TypeKind::Class | TypeKind::Interface => unimplemented!(),
                TypeKind::Enum => tokens.combine(&enums::gen(gen, def)),
                TypeKind::Struct => {
                    if gen.reader.type_def_fields(def).next().is_none() {
                        if let Some(guid) = gen.reader.type_def_guid(def) {
                            let ident = to_ident(type_name.name);
                            let value = gen.guid(&guid);
                            let guid = gen.type_name(&Type::GUID);
                            let cfg = gen.reader.type_def_cfg(def, &[]);
                            let doc = gen.cfg_doc(&cfg);
                            let constant = quote! {
                                #doc
                                pub const #ident: #guid = #value;
                            };
                            tokens.combine(&constant);
                            continue;
                        }
                    }
                    tokens.combine(&structs::gen(gen, def));
                }
                TypeKind::Delegate => tokens.combine(&delegates::gen(gen, def)),
            }
        }

        if !found {
            if let Some(def) = reader
                .get(TypeName::new(type_name.namespace, "Apis"))
                .next()
            {
                for method in gen.reader.type_def_methods(def) {
                    if found {
                        break;
                    }
                    let name = gen.reader.method_def_name(method);
                    if name == type_name.name {
                        found = true;
                        tokens.combine(&functions::gen(gen, method));
                    }
                }
                for field in gen.reader.type_def_fields(def) {
                    if found {
                        break;
                    }
                    let name = gen.reader.field_name(field);
                    if name == type_name.name {
                        found = true;
                        tokens.combine(&constants::gen(gen, field));
                    }
                }
            }
        }
    }

    try_format(tokens.into_string())
}

fn try_format(tokens: String) -> String {
    use std::io::Write;

    let Ok(mut child) = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn() else {
        return tokens;
    };

    let Some(mut stdin) = child.stdin.take() else {
        return tokens;
    };

    if stdin.write_all(tokens.as_bytes()).is_err() {
        return tokens;
    }

    drop(stdin);

    let Ok(output) = child.wait_with_output() else {
        return tokens;
    };

    if !output.status.success() {
        return tokens;
    }

    if let Ok(result) = String::from_utf8(output.stdout) {
        result
    } else {
        tokens
    }
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
