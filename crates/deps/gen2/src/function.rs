use super::*;

pub fn gen_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
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

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        let function = gen_function_decl(def, gen);

        quote! {
            #[link(name = "windows")]
            extern "system" {
                #function
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
            tokens.combine(&gen_function_decl(def, gen));
        }
    }

    tokens
}

fn gen_function_decl(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let return_type = gen_return_sig(&signature, gen);
    let arch_cfg = gen.arch_cfg(def.attributes());
    let feature_cfg = gen.method_cfg(def).0;

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_param_sig(p, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #arch_cfg
        #feature_cfg
        pub fn #name(#(#params),*) #return_type;
    }
}
