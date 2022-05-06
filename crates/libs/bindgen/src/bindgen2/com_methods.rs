use super::*;

pub fn gen(gen: &Gen, def: TypeDef, kind: InterfaceKind, method: MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames, base_count: usize) -> TokenStream {
    let signature = gen.reader.method_def_signature(method, &[]);
    let name = method_names.add(gen, method);
    let vname = virtual_names.add(gen, method);
    let constraints = gen.param_constraints(&signature.params);
    let mut cfg = gen.reader.method_def_cfg(method);
    cfg.add_feature(gen.reader.type_def_namespace(def));
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    if kind == InterfaceKind::None {
        return quote! {};
    }

    let mut bases = quote! {};

    for _ in 0..base_count {
        bases.combine(&quote! { .base__ });
    }

    match gen.reader.signature_kind(&signature) {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = gen.win32_args(leading_params);
            let params = gen.win32_params(leading_params);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = gen.win32_args(leading_params);
            let params = gen.win32_params(leading_params);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = gen.win32_args(leading_params);
            let params = gen.win32_params(leading_params);
            let return_type = type_deref(&signature.params[signature.params.len() - 1].ty);
            let return_type_tokens = gen.type_name(&return_type);
            let abi_return_type_tokens = gen.type_abi_name(&return_type);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let mut result__: #abi_return_type_tokens = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args ::core::mem::transmute(&mut result__))
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let args = gen.win32_args(&signature.params);
            let params = gen.win32_params(&signature.params);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let args = gen.win32_args(&signature.params);
            let params = gen.win32_params(&signature.params);
            // TODO: why is this using gen_abi_element_name?
            let return_type = gen.type_name(&signature.return_type.unwrap());

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_type {
                    let mut result__: #return_type = :: core::mem::zeroed();
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), &mut result__, #args);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let args = gen.win32_args(&signature.params);
            let params = gen.win32_params(&signature.params);
            // TODO: why gen_return_sig exists? Don't we always know it will be not ReturnVoid?
            let return_type = gen.return_sig(&signature);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) #return_type {
                    ::core::mem::transmute((::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args))
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let args = gen.win32_args(&signature.params);
            let params = gen.win32_params(&signature.params);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#constraints>(&self, #params) {
                    (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args)
                }
            }
        }
    }
}
