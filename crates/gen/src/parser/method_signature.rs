use super::*;

#[derive(Debug)]
pub struct MethodSignature {
    pub params: Vec<MethodParam>,
    pub return_type: Option<Signature>,
}

#[derive(Debug)]
pub struct MethodParam {
    pub param: tables::Param,
    pub signature: Signature,
}

impl MethodSignature {
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.return_type
            .iter()
            .filter_map(|s| s.definition())
            .chain(self.params.iter().filter_map(|p| p.signature.definition()))
            .collect()
    }

    pub fn gen_constraint(&self, gen: &Gen) -> TokenStream {
        let params = self.params.iter().map(|p| p.gen_produce_type(gen));

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.gen(gen)
        } else {
            quote! { () }
        };

        quote! { FnMut(#(#params),*) -> ::windows::Result<#return_type> + 'static }
    }

    // All WinRT ABI methods return an HRESULT while any return type is transformed into a trailing
    // out parameter. This is unlike Win32 methods that don't require this transformation.
    pub fn gen_winrt_abi(&self, gen: &Gen) -> TokenStream {
        let params = self
            .params
            .iter()
            .map(|p| {
                let name = p.param.gen_name();
                let abi = p.signature.gen_abi(gen);

                if p.param.is_input() {
                    // WinRT only uses const to mean that structs are passed by reference.
                    if p.signature.is_const {
                        quote! { #name: &#abi }
                    } else {
                        quote! { #name: #abi }
                    }
                } else {
                    quote! { #name: *mut #abi }
                }
            })
            .chain(self.return_type.iter().map(|p| {
                let abi = p.gen_abi(gen);
                quote! { result__: *mut #abi }
            }));

        quote! {
            (this: ::windows::RawPtr, #(#params),*) -> ::windows::ErrorCode
        }
    }

    pub fn gen_winrt_method(&self, gen: &MethodGen) -> TokenStream {
        let params = if gen.kind == InterfaceKind::Composable {
            &self.params[..self.params.len() - 2]
        } else {
            &self.params
        };

        let name = self.gen_name(gen);
        let windows = gen.gen.windows();
        let vtable_offset = Literal::u32_unsuffixed(gen.vtable_offset);
        let constraints = self.gen_constraints(params, gen);
        let args = params.iter().map(|p| p.gen_abi_arg());
        let params = self.gen_params(params, gen);
        let interface = gen.interface.gen_name(&gen.gen);

        let return_type_tokens = if let Some(return_type) = &self.return_type {
            let tokens = return_type.gen(&gen.gen);

            if return_type.is_array {
                quote! { #windows Array<#tokens> }
            } else {
                tokens
            }
        } else {
            quote! { () }
        };

        let return_arg = if let Some(return_type) = &self.return_type {
            if return_type.is_array {
                let return_type = return_type.gen(&gen.gen);
                quote! { #windows Array::<#return_type>::set_abi_len(&mut result__), #windows Array::<#return_type>::set_abi(&mut result__), }
            } else {
                quote! { &mut result__ }
            }
        } else {
            quote! {}
        };

        // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
        // arguments to ensure the call succeeds in the non-aggregating case.
        let composable_args = if gen.kind == InterfaceKind::Composable {
            quote! {
                ::std::ptr::null_mut(), #windows Abi::set_abi(&mut ::std::option::Option::<#windows Object>::None),
            }
        } else {
            quote! {}
        };
        
        let vcall = if let Some(return_type) = &self.return_type {
            if return_type.is_array {
                quote! {
                    let mut result__: #return_type_tokens = ::std::mem::zeroed();
                    (#windows Interface::vtable(this).#vtable_offset)(#windows Abi::abi(this), #(#args),* #composable_args #return_arg)
                        .and_then(|| result__ )
                }
            } else {
                quote! {
                    let mut result__: <#return_type_tokens as #windows Abi>::Abi = ::std::mem::zeroed();
                        (#windows Interface::vtable(this).#vtable_offset)(#windows Abi::abi(this), #(#args),* #composable_args #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                }
            }
        } else {
            quote! {
                (#windows Interface::vtable(this).#vtable_offset)(#windows Abi::abi(this), #(#args),* #composable_args).ok()
            }
        };

        match gen.kind {
            InterfaceKind::Default => quote! {
                pub fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                    let this = self;
                    unsafe {
                        #vcall
                    }
                }
            },
            InterfaceKind::NonDefault | InterfaceKind::Overrides => {
                quote! {
                    pub fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                        let this = &::windows::Interface::cast::<#interface>(self).unwrap();
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Statics | InterfaceKind::Composable => {
                quote! {
                    pub fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                        Self::#interface(|this| unsafe { #vcall })
                    }
                }
            }
        }
    }

    fn gen_name(&self, gen: &MethodGen) -> Ident {
        if gen.kind == InterfaceKind::Composable && self.params.len() == 2 {
            format_ident!("new")
        } else if gen.overload > 1 {
            format_ident!("{}{}", &gen.name, gen.overload)
        } else {
            to_ident(&gen.name)
        }
    }

    fn gen_constraints(&self, params: &[MethodParam], gen: &MethodGen) -> TokenStream {
        let mut tokens = Vec::new();

        for (index, param) in params.iter().enumerate() {
            if param.param.is_input()
                && !param.signature.is_array
                && param.signature.kind.is_convertible()
            {
                let name = squote::format_ident!("T{}__", index);
                let into = param.signature.kind.gen_name(&gen.gen);
                tokens.push(quote! { #name: ::std::convert::Into<::windows::Param<'a, #into>>, });
            }
        }

        if !tokens.is_empty() {
            tokens.insert(0, quote! { 'a, });
        }

        TokenStream::from_iter(tokens)
    }

    fn gen_params(&self, params: &[MethodParam], gen: &MethodGen) -> TokenStream {
        let windows = gen.gen.windows();

        TokenStream::from_iter(params.iter().enumerate().map(|(index, param)| {
            let name = param.param.gen_name();
            let tokens = param.signature.gen(&gen.gen);

            if param.signature.is_array {
                if param.param.is_input() {
                    quote! { #name: &[<#tokens as #windows RuntimeType>::DefaultType], }
                } else if param.signature.by_ref {
                    quote! { #name: &mut #windows Array<#tokens>, }
                } else {
                    quote! { #name: &mut [<#tokens as #windows RuntimeType>::DefaultType], }
                }
            } else if param.param.is_input() {
                if param.signature.kind.is_convertible() {
                    let tokens = squote::format_ident!("T{}__", index);
                    quote! { #name: #tokens, }
                } else {
                    quote! { #name: #tokens, }
                }
            } else if param.signature.kind.is_nullable() {
                quote! { #name: &mut ::std::option::Option<#tokens>, }
            } else if let ElementType::GenericParam(_) = param.signature.kind {
                quote! { &mut <#tokens as ::windows::RuntimeType>::DefaultType, }
            } else {
                quote! { #name: &mut #tokens, }
            }
        }))
    }
}

impl MethodParam {
    pub fn gen_abi_arg(&self) -> TokenStream {
        let name = self.param.gen_name();

        if self.signature.kind.is_blittable() {
            quote! { #name }
        } else {
            if self.param.is_input() {
                if self.signature.kind.is_convertible() {
                    // TODO: deal with const structs
                    quote! { #name.into().abi() }
                } else {
                    quote! { ::windows::Abi::abi(#name) }
                }
                
            } else {
                quote! { ::windows::Abi::set_abi(#name) }
            }
        }
    }

    pub fn gen_produce_type(&self, gen: &Gen) -> TokenStream {
        let tokens = self.signature.gen(gen);

        if self.param.is_input() {
            if self.signature.kind.is_primitive() {
                quote! { #tokens }
            } else {
                quote! { &#tokens }
            }
        } else {
            quote! { &mut #tokens }
        }
    }
}
