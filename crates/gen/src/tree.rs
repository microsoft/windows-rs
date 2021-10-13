use super::*;

pub fn gen_source_file(tree: &TypeTree) -> TokenStream {
    let gen = Gen::Relative(tree.namespace);

    let types = tree.types.values().map(move |t| gen_type_entry(t, &gen));

    let namespaces = tree.namespaces.keys().map(move |name| {
        let name = to_ident(name);
        quote! { pub mod #name; }
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
    let gen = Gen::Relative(tree.namespace);

    tree.types
        .iter()
        .map(move |t| gen_type_entry(t.1, &gen))
        .chain(gen_namespaces(&tree.namespaces))
}

fn gen_namespaces<'a>(
    namespaces: &'a BTreeMap<&'static str, TypeTree>,
) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        if tree.include {
            let name = to_ident(name);

            let tokens = namespace_iter(tree);

            quote! {
                // TODO: https://github.com/microsoft/windows-rs/issues/212
                // TODO: https://github.com/microsoft/win32metadata/issues/380
                #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
                pub mod #name {
                    #(#tokens)*
                }
            }
         } else {
             TokenStream::new()
         }
    })
}

fn gen_type_entry(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    if entry.include == TypeInclude::None {
        return TokenStream::new();
    }

    let mut tokens = TokenStream::new();

    // TODO: replace with regular for loop once multi-arch struct work is complete.
    for def in entry.def.first().iter() {
        // for def in &entry.def {
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
