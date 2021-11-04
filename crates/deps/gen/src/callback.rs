use super::*;

pub fn gen_callback(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let signature = def.invoke_method().signature(&[]);

    // Note that callbacks are C-style function pointers so the code gen will only use ABI types
    // to ensure the the ABI is faithfully preserved. Other types generally provide an abstraction
    // over the ABI but in this case that is not practical.

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_win32_abi_param(p, gen);
        quote! { #name: #tokens }
    });

    let return_sig = gen_win32_return_sig(&signature, gen);
    let cfg = gen.gen_function_cfg(def.attributes(), &signature);

    quote! {
        #cfg
        pub type #name = unsafe extern "system" fn(#(#params),*) #return_sig;
    }
}
