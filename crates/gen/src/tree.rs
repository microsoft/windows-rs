use super::*;

pub fn gen_tree(tree: &TypeTree) -> impl Iterator<Item = TokenStream> + '_ {
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

            let tokens = gen_tree(tree);

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

    match &entry.def {
        ElementType::TypeDef(def) => gen_type(&def.clone().with_generics(), gen, entry.include),
        ElementType::MethodDef(def) => gen_function(def, gen),
        ElementType::Field(def) => gen_constant(def, gen),
        _ => unimplemented!(),
    }
}
