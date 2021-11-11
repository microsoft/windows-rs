use super::*;

pub fn gen_sys(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let functions = gen_functions(tree, gen);

    quote! {
        #functions
    }
}

fn gen_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut functions = tree.types.values().map(|entry| gen_function_if(entry, gen)).peekable();

    if functions.peek().is_some() {
        quote! {
            #[link(name = "windows")]
            extern "system" {
                #(#functions)*
            }
        }
    } else {
        quote! {}
    }
}

fn gen_function_if(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for def in &entry.def {
        if let ElementType::MethodDef(def) = def {
            tokens.combine(&gen_function(def, gen));
        }
    }

    tokens
}

fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = to_ident(def.name());
    let signature = def.signature(&[]);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = gen_win32_return_sig(&signature, gen);

    let cfg = gen.gen_function_cfg(def.attributes(), &signature);


    quote! {
        #cfg
        pub fn #name();
    }
}
