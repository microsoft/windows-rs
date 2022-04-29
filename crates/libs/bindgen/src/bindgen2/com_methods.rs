use super::*;

pub fn gen(gen: &Gen, def: TypeDef, kind: InterfaceKind, method: MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames, base_count: usize) -> TokenStream {
    // let signature = method.signature(&def.generics);
    // let name = method_names.add(method);
    // let vname = virtual_names.add(method);
    // let constraints = gen_param_constraints(&signature.params, gen);
    // let mut cfg = method.cfg();
    // cfg.add_feature(def.namespace());
    // let doc = gen.doc(&cfg);
    // let features = gen.cfg(&cfg);

    // if kind == InterfaceKind::NonDefault {
    //     return quote! {};
    // }

    // let mut bases = quote! {};

    // for _ in 0..base_count {
    //     bases.combine(&quote! { .base__ });
    // }

    // match signature.kind() {
    //     SignatureKind::Query => {
    //         let leading_params = &signature.params[..signature.params.len() - 2];
    //         let args = gen_win32_args(leading_params);
    //         let params = gen_win32_params(leading_params, gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
    //                 let mut result__ = ::core::option::Option::None;
    //                 (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    //             }
    //         }
    //     }
    //     SignatureKind::QueryOptional => {
    //         let leading_params = &signature.params[..signature.params.len() - 2];
    //         let args = gen_win32_args(leading_params);
    //         let params = gen_win32_params(leading_params, gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
    //                 (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    //             }
    //         }
    //     }
    //     SignatureKind::ResultValue => {
    //         let leading_params = &signature.params[..signature.params.len() - 1];
    //         let args = gen_win32_args(leading_params);
    //         let params = gen_win32_params(leading_params, gen);

    //         let return_type = signature.params[signature.params.len() - 1].ty.deref();
    //         let return_type_tokens = gen_element_name(&return_type, gen);
    //         let abi_return_type_tokens = gen_abi_element_name(&return_type, gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
    //                 let mut result__: #abi_return_type_tokens = ::core::mem::zeroed();
    //                 (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args ::core::mem::transmute(&mut result__))
    //                 .from_abi::<#return_type_tokens>(result__ )
    //             }
    //         }
    //     }
    //     SignatureKind::ResultVoid => {
    //         let args = gen_win32_args(&signature.params);
    //         let params = gen_win32_params(&signature.params, gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
    //                 (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args).ok()
    //             }
    //         }
    //     }
    //     SignatureKind::ReturnStruct => {
    //         let args = gen_win32_args(&signature.params);
    //         let params = gen_win32_params(&signature.params, gen);
    //         // TODO: why is this using gen_abi_element_name?
    //         let return_type = gen_abi_element_name(&signature.return_type.unwrap(), gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints>(&self, #params) -> #return_type {
    //                 let mut result__: #return_type = :: core::mem::zeroed();
    //                 (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), &mut result__, #args);
    //                 result__
    //             }
    //         }
    //     }
    //     SignatureKind::PreserveSig => {
    //         let args = gen_win32_args(&signature.params);
    //         let params = gen_win32_params(&signature.params, gen);
    //         // TODO: why gen_return_sig exists? Don't we always know it will be not ReturnVoid?
    //         let return_type = gen_return_sig(&signature, gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints>(&self, #params) #return_type {
    //                 ::core::mem::transmute((::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args))
    //             }
    //         }
    //     }
    //     SignatureKind::ReturnVoid => {
    //         let args = gen_win32_args(&signature.params);
    //         let params = gen_win32_params(&signature.params, gen);

    //         quote! {
    //             #doc
    //             #features
    //             pub unsafe fn #name<#constraints>(&self, #params) {
    //                 (::windows::core::Interface::vtable(self)#bases.#vname)(::core::mem::transmute_copy(self), #args)
    //             }
    //         }
    //     }
    // }

    " ".into()
}

// pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
//     let mut tokens = quote! {};

//     for (position, param) in params.iter().enumerate() {
//         let name = gen_param_name(&param.def);

//         if let Some(ArrayInfo::Fixed(fixed)) = param.array_info {
//             if fixed > 0 && param.def.free_with().is_none() {
//                 let ty = param.ty.deref();
//                 let ty = gen_default_type(&ty, gen);
//                 let len = Literal::u32_unsuffixed(fixed as _);

//                 let ty = if param.def.flags().output() {
//                     quote! { &mut [#ty; #len] }
//                 } else {
//                     quote! { &[#ty; #len] }
//                 };

//                 tokens.combine(&quote! { #name: #ty, });
//                 continue;
//             }
//         }

//         if let Some(ArrayInfo::RelativeLen(_)) = param.array_info {
//             let ty = param.ty.deref();
//             let ty = gen_default_type(&ty, gen);
//             let ty = if param.def.flags().output() {
//                 quote! { &mut [#ty] }
//             } else {
//                 quote! { &[#ty] }
//             };

//             tokens.combine(&quote! { #name: #ty, });
//             continue;
//         }

//         if let Some(ArrayInfo::RelativePtr(_)) = param.array_info {
//             continue;
//         }

//         if param.is_convertible() {
//             let kind: TokenStream = format!("Param{}", position).into();
//             tokens.combine(&quote! { #name: #kind, });
//             continue;
//         }

//         let kind = gen_default_type(&param.ty, gen);
//         tokens.combine(&quote! { #name: #kind, });
//     }

//     tokens
// }

// pub fn gen_win32_args(params: &[MethodParam]) -> TokenStream {
//     let mut tokens = quote! {};

//     for param in params {
//         let name = gen_param_name(&param.def);

//         if let Some(ArrayInfo::Fixed(fixed)) = param.array_info {
//             if fixed > 0 && param.def.free_with().is_none() {
//                 let signature = if param.def.flags().output() {
//                     quote! { ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(#name)), }
//                 } else {
//                     quote! { ::core::mem::transmute(::windows::core::as_ptr_or_null(#name)), }
//                 };

//                 tokens.combine(&signature);
//                 continue;
//             }
//         }

//         if let Some(ArrayInfo::RelativeLen(_)) = param.array_info {
//             let signature = if param.def.flags().output() {
//                 quote! { ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(#name)), }
//             } else {
//                 quote! { ::core::mem::transmute(::windows::core::as_ptr_or_null(#name)), }
//             };

//             tokens.combine(&signature);
//             continue;
//         }

//         if let Some(ArrayInfo::RelativePtr(relative)) = param.array_info {
//             let name = gen_param_name(&params[relative].def);
//             tokens.combine(&quote! { #name.len() as _, });
//             continue;
//         }

//         if param.is_convertible() {
//             tokens.combine(&quote! { #name.into_param().abi(), });
//             continue;
//         }

//         tokens.combine(&quote! { ::core::mem::transmute(#name), });
//     }

//     tokens
// }
