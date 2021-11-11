use super::*;

pub fn gen_sys(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let functions = tree.types.iter().map(|(name, def)| gen_function(name, def));

    quote! {
        #[link(name = "windows")]
        extern "system" {
            #(#functions)*
        }
    }
}

fn gen_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
}

fn gen_function(name: &str, entry: &TypeEntry) -> TokenStream {
    let mut tokens = TokenStream::new();

    for def in &entry.def {
        if let ElementType::MethodDef(def) = def {
            let name = to_ident(name);
            tokens.combine(&quote! {
                fn #name();
            });
        }
    }

    tokens
}
