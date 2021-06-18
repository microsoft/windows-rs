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
    pub fn dependencies(&self) -> Vec<TypeEntry> {
        self.return_type
            .iter()
            .map(|s| s.definition(TypeInclude::Minimal))
            .chain(self.params.iter().map(|p| p.signature.definition(TypeInclude::Minimal)))
            .flatten()
            .collect()
    }

    pub fn has_query_interface(&self) -> bool {
        self.return_type.as_ref().map_or(false, |signature| {
            if signature.kind == ElementType::HRESULT && self.params.len() >= 2 {
                let guid = &self.params[self.params.len() - 2];
                let object = &self.params[self.params.len() - 1];

                if guid.signature.kind == ElementType::Guid
                    && guid.is_const()
                    && object.signature.kind == ElementType::Void
                    && object.signature.pointers == 2
                {
                    return true;
                }
            }

            false
        })
    }

    pub fn gen_winrt_constraint(&self, gen: &Gen) -> TokenStream {
        let params = self.params.iter().map(|p| p.gen_winrt_produce_type(gen));

        let return_type = if let Some(return_type) = &self.return_type {
            let tokens = return_type.kind.gen_name(gen);

            if return_type.is_array {
                quote! { ::windows::Array<#tokens> }
            } else {
                tokens
            }
        } else {
            quote! { () }
        };

        quote! { F: FnMut(#(#params),*) -> ::windows::Result<#return_type> + 'static }
    }

    // All WinRT ABI methods return an HRESULT while any return type is transformed into a trailing
    // out parameter. This is unlike Win32 methods that don't require this transformation.
    pub fn gen_winrt_abi(&self, gen: &Gen) -> TokenStream {
        let params = self
            .params
            .iter()
            .map(|p| {
                let name = p.param.gen_name();
                let abi = p.signature.gen_winrt_abi(gen);

                if p.signature.is_array {
                    let abi_size_name = p.param.gen_abi_size_name();
                    if p.param.is_input() {
                        quote! { #abi_size_name: u32, #name: *const #abi }
                    } else if p.signature.by_ref {
                        quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi }
                    } else {
                        quote! { #abi_size_name: u32, #name: *mut #abi }
                    }
                } else if p.param.is_input() {
                    // WinRT only uses const to mean that structs are passed by reference.
                    if p.is_const() {
                        quote! { #name: &#abi }
                    } else {
                        quote! { #name: #abi }
                    }
                } else {
                    quote! { #name: *mut #abi }
                }
            })
            .chain(self.return_type.iter().map(|signature| {
                let abi = signature.gen_winrt_abi(gen);

                if signature.is_array {
                    quote! { result_size__: *mut u32, result__: *mut *mut #abi }
                } else {
                    quote! { result__: *mut #abi }
                }
            }));

        quote! {
            (this: ::windows::RawPtr, #(#params),*) -> ::windows::HRESULT
        }
    }

    pub fn gen_winrt_method(
        &self,
        method: &MethodInfo,
        interface: &InterfaceInfo,
        gen: &Gen,
    ) -> TokenStream {
        let params = if interface.kind == InterfaceKind::Composable
            || interface.kind == InterfaceKind::Extend
        {
            &self.params[..self.params.len() - 2]
        } else {
            &self.params
        };

        let name = self.gen_name(method, interface);

        let vtable_offset = Literal::u32_unsuffixed(method.vtable_offset);
        let constraints = self.gen_constraints(params);
        let args = params.iter().map(|p| p.gen_winrt_abi_arg());
        let params = self.gen_winrt_params(params, gen);
        let interface_name = interface.def.gen_name(gen);

        let return_type_tokens = if let Some(return_type) = &self.return_type {
            let tokens = return_type.kind.gen_name(gen);

            if return_type.is_array {
                quote! { ::windows::Array<#tokens> }
            } else {
                tokens
            }
        } else {
            quote! { () }
        };

        let return_arg = if let Some(return_type) = &self.return_type {
            if return_type.is_array {
                let return_type = return_type.kind.gen_name(gen);
                quote! { ::windows::Array::<#return_type>::set_abi_len(&mut result__), ::windows::Array::<#return_type>::set_abi(&mut result__) }
            } else {
                quote! { &mut result__ }
            }
        } else {
            quote! {}
        };

        // The ABI obviously still has the two composable parameters. Here we just pass the default in and out
        // arguments to ensure the call succeeds in the non-aggregating case.
        let composable_args = match interface.kind {
            InterfaceKind::Composable => quote! {
                ::std::ptr::null_mut(), ::windows::Abi::set_abi(&mut ::std::option::Option::<::windows::IInspectable>::None),
            },
            InterfaceKind::Extend => quote! {
                ::windows::Abi::abi(&derived__), ::windows::Abi::set_abi(base__),
            },
            _ => quote! {},
        };

        let vcall = if let Some(return_type) = &self.return_type {
            if return_type.is_array {
                quote! {
                    let mut result__: #return_type_tokens = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args,)* #composable_args #return_arg)
                        .and_then(|| result__ )
                }
            } else {
                quote! {
                    let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args,)* #composable_args #return_arg)
                            .from_abi::<#return_type_tokens>(result__ )
                }
            }
        } else {
            quote! {
                (::windows::Interface::vtable(this).#vtable_offset)(::windows::Abi::abi(this), #(#args,)* #composable_args).ok()
            }
        };

        let deprecated = if method.is_deprecated {
            quote! { #[cfg(feature = "deprecated")] }
        } else {
            quote! {}
        };

        match interface.kind {
            InterfaceKind::Default => quote! {
                #deprecated
                pub fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                    let this = self;
                    unsafe {
                        #vcall
                    }
                }
            },
            InterfaceKind::NonDefault | InterfaceKind::Overridable => {
                quote! {
                    #deprecated
                    pub fn #name<#constraints>(&self, #params) -> ::windows::Result<#return_type_tokens> {
                        let this = &::windows::Interface::cast::<#interface_name>(self).unwrap();
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Static | InterfaceKind::Composable => {
                quote! {
                    #deprecated
                    pub fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
            InterfaceKind::Extend => {
                let interface_name = to_ident(interface.def.name());
                quote! {
                    pub fn #name<#constraints>(self, #params) -> ::windows::Result<#return_type_tokens> {
                        unsafe {
                            let (derived__, base__) = ::windows::Compose::compose(self);
                            #return_type_tokens::#interface_name(|this| unsafe { #vcall })
                        }
                    }
                }
            }
        }
    }

    fn gen_name(&self, method: &MethodInfo, interface: &InterfaceInfo) -> Ident {
        if (interface.kind == InterfaceKind::Composable || interface.kind == InterfaceKind::Extend)
            && self.params.len() == 2
        {
            format_ident!("new")
        } else if method.overload > 1 {
            format_ident!("{}{}", &method.name, method.overload)
        } else {
            to_ident(&method.name)
        }
    }

    pub fn gen_constraints(&self, params: &[MethodParam]) -> TokenStream {
        if params.iter().any(|param| param.is_convertible()) {
            quote! { 'a, }
        } else {
            quote! {}
        }
    }

    pub fn gen_winrt_params(&self, params: &[MethodParam], gen: &Gen) -> TokenStream {
        params
            .iter()
            .map(|param| {
                let name = param.param.gen_name();
                let tokens = param.signature.kind.gen_name(gen);

                if param.signature.is_array {
                    if param.param.is_input() {
                        quote! { #name: &[<#tokens as ::windows::RuntimeType>::DefaultType], }
                    } else if param.signature.by_ref {
                        quote! { #name: &mut ::windows::Array<#tokens>, }
                    } else {
                        quote! { #name: &mut [<#tokens as ::windows::RuntimeType>::DefaultType], }
                    }
                } else if param.param.is_input() {
                    if param.is_convertible() {
                        let into = param.signature.kind.gen_name(gen);
                        quote! { #name: impl ::windows::IntoParam<'a, #into>, }
                    } else {
                        let mut signature = quote! {};

                        for _ in 0..param.signature.pointers {
                            if param.is_const() {
                                signature.combine(&quote! { *const });
                            } else {
                                signature.combine(&quote! { *mut });
                            }
                        }

                        signature.combine(&tokens);
                        quote! { #name: #signature, }
                    }
                } else if param.signature.kind.is_nullable() {
                    quote! { #name: &mut ::std::option::Option<#tokens>, }
                } else if let ElementType::GenericParam(_) = param.signature.kind {
                    quote! { &mut <#tokens as ::windows::RuntimeType>::DefaultType, }
                } else if param.signature.pointers > 0 {
                    let tokens = param.signature.gen_winrt_abi(gen);
                    quote! { #name: #tokens, }
                } else {
                    quote! { #name: &mut #tokens, }
                }
            })
            .collect()
    }

    pub fn gen_winrt_upcall(&self, inner: TokenStream, gen: &Gen) -> TokenStream {
        let invoke_args = self
            .params
            .iter()
            .map(|param| param.gen_winrt_invoke_arg(gen));

        match &self.return_type {
            Some(return_type) if return_type.is_array => {
                quote! {
                    match #inner(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            let (ok_data__, ok_data_len__) = ok__.into_abi();
                            *result__ = ok_data__;
                            *result_size__ = ok_data_len__;
                            ::windows::HRESULT(0)
                        }
                        ::std::result::Result::Err(err) => err.into()
                    }
                }
            }
            Some(_) => {
                quote! {
                    match #inner(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            *result__ = ::std::mem::transmute_copy(&ok__);
                            ::std::mem::forget(ok__);
                            ::windows::HRESULT(0)
                        }
                        ::std::result::Result::Err(err) => err.into()
                    }
                }
            }
            None => quote! {
                #inner(#(#invoke_args,)*).into()
            },
        }
    }

    pub fn gen_win32_params(&self, params: &[MethodParam], gen: &Gen) -> TokenStream {
        params
            .iter()
            .map(|param| {
                let name = param.param.gen_name();

                if param.is_convertible() {
                    let into = param.signature.kind.gen_name(gen);
                    quote! { #name: impl ::windows::IntoParam<'a, #into>, }
                } else {
                    let tokens = param.gen_win32(gen);
                    quote! { #name: #tokens, }
                }
            })
            .collect()
    }
}

impl MethodParam {
    fn is_convertible(&self) -> bool {
        self.param.is_input()
            && !self.signature.is_array
            && self.signature.pointers == 0
            && self.signature.kind.is_convertible()
    }

    fn is_const(&self) -> bool {
        self.signature.is_const || self.param.is_const()
    }

    fn gen_winrt_invoke_arg(&self, gen: &Gen) -> TokenStream {
        let name = self.param.gen_name();
        let kind = self.signature.kind.gen_name(gen);

        // TODO: This compiles but doesn't property handle delegates with array parameters.
        // https://github.com/microsoft/windows-rs/issues/212

        if self.signature.is_array {
            quote! { ::std::mem::transmute_copy(&#name) }
        } else if self.param.is_input() {
            if self.signature.kind.is_primitive() {
                quote! { #name }
            } else if self.is_const() {
                quote! { &*(#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::RuntimeType>::DefaultType) }
            } else {
                quote! { &*(&#name as *const <#kind as ::windows::Abi>::Abi as *const <#kind as ::windows::RuntimeType>::DefaultType) }
            }
        } else {
            quote! { ::std::mem::transmute_copy(&#name) }
        }
    }

    pub fn gen_winrt_abi_arg(&self) -> TokenStream {
        let name = self.param.gen_name();

        if self.signature.is_array {
            if self.param.is_input() {
                quote! { #name.len() as u32, ::std::mem::transmute(#name.as_ptr()) }
            } else if self.signature.by_ref {
                quote! { #name.set_abi_len(), #name.set_abi() }
            } else {
                quote! { #name.len() as u32, ::std::mem::transmute_copy(&#name) }
            }
        } else if self.param.is_input() {
            if self.is_convertible() {
                if self.is_const() {
                    quote! { &#name.into_param().abi() }
                } else {
                    quote! { #name.into_param().abi() }
                }
            } else if self.signature.kind.is_blittable() {
                quote! { #name }
            } else if self.signature.pointers == 0 {
                quote! { ::windows::Abi::abi(#name) }
            } else {
                quote! { ::std::mem::transmute(#name) }
            }
        } else if self.signature.kind.is_blittable()
            || (self.signature.pointers > 0 && !self.signature.kind.is_nullable())
        {
            quote! { #name }
        } else {
            quote! { ::windows::Abi::set_abi(#name) }
        }
    }

    pub fn gen_win32(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();
        let is_const = self.is_const();

        for _ in 0..self.signature.pointers {
            if is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        let kind = self.signature.kind.gen_name(gen);

        if self.signature.kind.is_nullable() {
            tokens.combine(&quote! {
                ::std::option::Option<#kind>
            });
        } else {
            tokens.combine(&kind)
        }

        tokens
    }

    pub fn gen_win32_abi(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();
        let is_const = self.is_const();

        for _ in 0..self.signature.pointers {
            if is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        tokens.combine(&self.signature.kind.gen_abi_type(gen));
        tokens
    }

    pub fn gen_win32_abi_param(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        for _ in 0..self.signature.pointers {
            if self.is_const() {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        tokens.combine(&self.signature.kind.gen_abi_type(gen));
        tokens
    }

    pub fn gen_win32_abi_arg(&self) -> TokenStream {
        let name = self.param.gen_name();

        if self.is_convertible() {
            quote! { #name.into_param().abi() }
        } else {
            quote! { ::std::mem::transmute(#name) }
        }
    }

    pub fn gen_winrt_produce_type(&self, gen: &Gen) -> TokenStream {
        let tokens = self.signature.kind.gen_name(gen);

        if self.signature.is_array {
            if self.param.is_input() {
                quote! { &[#tokens] }
            } else if self.signature.by_ref {
                quote! { &mut ::windows::Array<#tokens> }
            } else {
                quote! { &mut [#tokens] }
            }
        } else if self.param.is_input() {
            if let ElementType::GenericParam(_) = self.signature.kind {
                quote! { &<#tokens as ::windows::RuntimeType>::DefaultType }
            } else if self.signature.kind.is_primitive() {
                quote! { #tokens }
            } else if self.signature.kind.is_nullable() {
                quote! { &::std::option::Option<#tokens> }
            } else {
                quote! { &#tokens }
            }
        } else {
            quote! { &mut #tokens }
        }
    }
}
