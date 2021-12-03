use super::*;

pub fn gen_sys_file(root: &'static str, tree: &TypeTree, ignore_windows_features: bool) -> TokenStream {
    let gen = Gen { relative: tree.namespace, root, ignore_windows_features, docs: false, build: false };
    let types = gen_sys(tree, &gen);

    let namespaces = tree.namespaces.iter().filter_map(move |(name, tree)| {
        if !tree.include {
            return None;
        }

        let name = to_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        Some(quote! {
            #[cfg(feature = #namespace)] pub mod #name;
        })
    });

    quote! {
        #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
        #(#namespaces)*
        #types
    }
}

pub fn gen_source_file(root: &'static str, tree: &TypeTree, ignore_windows_features: bool) -> TokenStream {
    let gen = Gen { relative: tree.namespace, root, ignore_windows_features, docs: false, build: false };

    let types = tree.types.values().map(move |t| gen_type_entry(t, &gen));

    let namespaces = tree.namespaces.iter().filter_map(move |(name, tree)| {
        if !tree.include {
            return None;
        }

        let name = to_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        Some(quote! {
            #[cfg(feature = #namespace)] pub mod #name;
        })
    });

    quote! {
        #![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
        #(#namespaces)*
        #(#types)*
    }
}

pub fn gen_source_tree() -> TokenStream {
    let reader = TypeReader::get();

    namespace_iter(&reader.types).fold(TokenStream::new(), |mut accum, n| {
        accum.combine(&n);
        accum
    })
}

pub fn namespace_iter(tree: &TypeTree) -> impl Iterator<Item = TokenStream> + '_ {
    let gen = Gen::relative(tree.namespace);

    tree.types.iter().map(move |t| gen_type_entry(t.1, &gen)).chain(gen_namespaces(&tree.namespaces))
}

fn gen_namespaces<'a>(namespaces: &'a BTreeMap<&'static str, TypeTree>) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        if tree.include {
            // TODO: https://github.com/microsoft/windows-rs/issues/212
            // TODO: https://github.com/microsoft/win32metadata/issues/380

            let allow = if name == &tree.namespace {
                quote! { #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)] }
            } else {
                quote! {}
            };

            let name = to_ident(name);
            let tokens = namespace_iter(tree);

            quote! {
                #allow
                pub mod #name {
                    #(#tokens)*
                }
            }
        } else {
            TokenStream::new()
        }
    })
}

pub fn gen_type_entry(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    if entry.include == TypeInclude::None {
        return TokenStream::new();
    }

    let mut tokens = TokenStream::new();

    for def in &entry.def {
        tokens.combine(&match def {
            ElementType::TypeDef(def) => gen_type(&def.clone().with_generics(), gen, entry.include),
            ElementType::MethodDef(def) => gen_function(def, gen),
            ElementType::Field(def) => gen_constant(def, gen),
            _ => unimplemented!(),
        });
    }

    tokens
}

fn gen_type(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    match def.kind() {
        TypeKind::Interface => {
            if def.is_winrt() {
                gen_interface(&def.clone().with_generics(), gen, include)
            } else {
                gen_com_interface(def, gen, include)
            }
        }
        TypeKind::Class => Class(def.clone().with_generics()).gen(gen, include),
        TypeKind::Enum => gen_enum(def, gen, include),
        TypeKind::Struct => gen_struct(def, gen),
        TypeKind::Delegate => {
            if def.is_winrt() {
                gen_delegate(def, gen)
            } else {
                gen_callback(def, gen)
            }
        }
    }
}
