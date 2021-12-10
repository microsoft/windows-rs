use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());

    let method = def.invoke_method();
    let signature = method.signature(&[]);
    let return_sig = gen_return_sig(&signature, gen);
    let cfg = gen.type_cfg(def).gen_with_doc(gen);

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_param_sig(p, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #cfg
        pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
    }
}
