use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    if gen.reader.type_def_flags(def).winrt() {
        gen_delegate(gen, def)
    } else {
        gen_callback(gen, def)
    }
}

fn gen_callback(gen: &Gen, def: TypeDef) -> TokenStream {
    let name = to_ident(gen.reader.type_def_name(def));

    let method = gen.reader.type_def_invoke_method(def);
    let signature = gen.reader.method_def_signature(method, &[]);
    let return_type = gen.return_sig(&signature);
    let cfg = gen.reader.type_def_cfg(def, &[]); // TODO: why not just use method_def_cfg?
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    let params = signature.params.iter().map(|p| {
        let name = gen.param_name(p.def);
        let tokens = gen.type_default_name(&p.ty);
        quote! { #name: #tokens }
    });

    quote! {
        #doc
        #features
        pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_type>;
    }
}

fn gen_delegate(gen: &Gen, def: TypeDef) -> TokenStream {
    " ".into()
}

