use super::*;

// This is WinRT methods only
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub def: MethodDef,
    pub signature: Signature,
    pub dependencies: TypeMap,
}

impl Method {
    pub fn new(def: MethodDef, generics: &[Type]) -> Self {
        let signature = def.signature("", generics);

        let mut dependencies = TypeMap::new();
        signature.dependencies(&mut dependencies);

        Self {
            def,
            signature,
            dependencies,
        }
    }

    pub fn write_upcall(&self, inner: TokenStream, this: bool) -> TokenStream {
        let noexcept = self.def.has_attribute("NoExceptionAttribute");

        let invoke_args = self.signature
        .params
        .iter()
        .map(|param| {
            let name = to_ident(&param.1.name().to_lowercase());
            let abi_size_name: TokenStream = format!("{}_array_size", param.1.name().to_lowercase()).into();

            if param.1.flags().contains(ParamAttributes::In) {
                if param.0.is_winrt_array() {
                    quote! { core::slice::from_raw_parts(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
                } else if param.0.is_primitive() {
                    quote! { #name }
                } else if param.0.is_const_ref() || param.0.is_interface() {
                    quote! { core::mem::transmute_copy(&#name) }
                } else {
                    quote! { core::mem::transmute(&#name) }
                }
            } else if param.0.is_winrt_array() {
                quote! { core::slice::from_raw_parts_mut(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
            } else if param.0.is_winrt_array_ref() {
                quote! { windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
            } else {
                quote! { core::mem::transmute_copy(&#name) }
            }
        });

        let this = if this {
            quote! { this, }
        } else {
            quote! {}
        };

        match &self.signature.return_type.0 {
            Type::Void => {
                if noexcept {
                    quote! {
                        #inner(#this #(#invoke_args,)*);
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        #inner(#this #(#invoke_args,)*).into()
                    }
                }
            }
            _ if self.signature.return_type.0.is_winrt_array() => {
                if noexcept {
                    quote! {
                        let ok__ = #inner(#this #(#invoke_args,)*);
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        match #inner(#this #(#invoke_args,)*) {
                            Ok(ok__) => {
                                let (ok_data__, ok_data_len__) = ok__.into_abi();
                                // use `ptr::write` since `result` could be uninitialized
                                result__.write(core::mem::transmute(ok_data__));
                                result_size__.write(ok_data_len__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into()
                        }
                    }
                }
            }
            _ => {
                let forget = if self.signature.return_type.0.is_copyable() {
                    quote! {}
                } else {
                    quote! { core::mem::forget(ok__); }
                };

                if noexcept {
                    quote! {
                        let ok__ = #inner(#this #(#invoke_args,)*);
                        // use `ptr::write` since `result` could be uninitialized
                        result__.write(core::mem::transmute_copy(&ok__));
                        #forget
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        match #inner(#this #(#invoke_args,)*) {
                            Ok(ok__) => {
                                // use `ptr::write` since `result` could be uninitialized
                                result__.write(core::mem::transmute_copy(&ok__));
                                #forget
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into()
                        }
                    }
                }
            }
        }
    }

    pub fn write_impl_signature(
        &self,
        writer: &Writer,
        named_params: bool,
        has_this: bool,
    ) -> TokenStream {
        let noexcept = self.def.has_attribute("NoExceptionAttribute");

        let params = self.signature.params.iter().map(|p| {
            let default_type = p.0.write_default(writer);

            let sig = if p.1.flags().contains(ParamAttributes::In) {
                if p.0.is_winrt_array() {
                    quote! { &[#default_type] }
                } else if p.0.is_primitive() {
                    quote! { #default_type }
                } else if p.0.is_interface() {
                    let type_name = p.0.write_name(writer);
                    quote! { windows_core::Ref<'_, #type_name> }
                } else {
                    quote! { &#default_type }
                }
            } else if p.0.is_winrt_array() {
                quote! { &mut [#default_type] }
            } else if p.0.is_winrt_array_ref() {
                let kind = p.0.write_name(writer);
                quote! { &mut windows_core::Array<#kind> }
            } else if p.0.is_interface() {
                let type_name = p.0.write_name(writer);
                quote! { windows_core::OutRef<'_, #type_name> }
            } else {
                quote! { &mut #default_type }
            };

            if named_params {
                let name = to_ident(p.1.name());
                quote! { #name: #sig }
            } else {
                sig
            }
        });

        let return_type_tokens = if self.signature.return_type.0 == Type::Void {
            quote! { () }
        } else {
            let tokens = self.signature.return_type.0.write_name(writer);

            if self.signature.return_type.0.is_winrt_array() {
                quote! { windows_core::Array<#tokens> }
            } else {
                tokens
            }
        };

        let return_type_tokens = if noexcept {
            if self.signature.return_type.0.is_interface() {
                quote! { -> Option<#return_type_tokens> }
            } else if self.signature.return_type.0 == Type::Void {
                quote! {}
            } else {
                quote! { -> #return_type_tokens }
            }
        } else {
            quote! { -> windows_core::Result<#return_type_tokens> }
        };

        if has_this {
            quote! {
               (&self, #(#params),*) #return_type_tokens
            }
        } else {
            quote! {
                (#(#params),*) #return_type_tokens
            }
        }
    }

    pub fn write_abi(&self, writer: &Writer, named_params: bool) -> TokenStream {
        let args = self.signature.params.iter().map(|param| {
            let name = to_ident(&param.1.name().to_lowercase());
            let abi = param.0.write_abi(writer);
            let abi_size_name: TokenStream = format!("{}_array_size", name.as_str()).into();

            if param.1.flags().contains(ParamAttributes::In) {
                if param.0.is_winrt_array() {
                    if named_params {
                        quote! { #abi_size_name: u32, #name: *const #abi }
                    } else {
                        quote! { u32, *const #abi }
                    }
                } else if param.0.is_const_ref() {
                    if named_params {
                        quote! { #name: &#abi }
                    } else {
                        quote! { &#abi }
                    }
                } else if named_params {
                    quote! { #name: #abi }
                } else {
                    quote! { #abi }
                }
            } else if param.0.is_winrt_array() {
                if named_params {
                    quote! { #abi_size_name: u32, #name: *mut #abi }
                } else {
                    quote! { u32, *mut #abi }
                }
            } else if param.0.is_winrt_array_ref() {
                if named_params {
                    quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi }
                } else {
                    quote! { *mut u32, *mut *mut #abi }
                }
            } else if named_params {
                quote! { #name: *mut #abi }
            } else {
                quote! { *mut #abi }
            }
        });

        let return_arg = match &self.signature.return_type.0 {
            Type::Void => quote! {},
            Type::Array(ty) => {
                let ty = ty.write_abi(writer);
                if named_params {
                    quote! { result_size__: *mut u32, result__: *mut *mut #ty }
                } else {
                    quote! { *mut u32, *mut *mut #ty }
                }
            }
            ty => {
                let ty = ty.write_abi(writer);
                if named_params {
                    quote! { result__: *mut #ty }
                } else {
                    quote! { *mut #ty }
                }
            }
        };

        if named_params {
            quote! {
                this: *mut core::ffi::c_void, #(#args,)* #return_arg
            }
        } else {
            quote! {
                *mut core::ffi::c_void, #(#args,)* #return_arg
            }
        }
    }

    pub fn write(
        &self,
        writer: &Writer,
        interface_name: TokenStream,
        kind: InterfaceKind,
        method_names: &mut MethodNames,
        virtual_names: &mut MethodNames,
    ) -> TokenStream {
        let params = if kind == InterfaceKind::Composable {
            &self.signature.params[..self.signature.params.len() - 2]
        } else {
            &self.signature.params
        };

        let name = if kind == InterfaceKind::Composable && params.is_empty() {
            quote!(new)
        } else {
            method_names.add(self.def)
        };

        let args = {
            let args = params.iter().map(|param|{
                let name = to_ident(&param.1.name().to_lowercase());

                if param.1.flags().contains(ParamAttributes::In) {
                    if param.0.is_winrt_array() {
                        if param.0.is_copyable() {
                            quote! { #name.len().try_into().unwrap(), #name.as_ptr() }
                        } else {
                            quote! { #name.len().try_into().unwrap(), core::mem::transmute(#name.as_ptr()) }
                        }
                    } else if param.0.is_convertible() {
                        quote! { #name.param().abi() }
                    } else if param.0.is_copyable() {
                        if param.0.is_const_ref() {
                            quote! { &#name }
                        } else {
                            quote! { #name }
                        }
                    } else {
                        quote! { core::mem::transmute_copy(#name) }
                    }
                } else if param.0.is_winrt_array() {
                    if param.0.is_copyable() {
                        quote! { #name.len().try_into().unwrap(), #name.as_mut_ptr() }
                    } else {
                        quote! { #name.len().try_into().unwrap(), core::mem::transmute_copy(&#name) }
                    }
                } else if param.0.is_winrt_array_ref() {
                    quote! { #name.set_abi_len(), #name as *mut _ as _ }
                } else if param.0.is_copyable() {
                    quote! { #name }
                } else {
                    quote! { #name as *mut _ as _ }
                }
            });

            let return_arg = match &self.signature.return_type.0 {
                Type::Void => quote! {},
                ty => {
                    if ty.is_winrt_array() {
                        let ty = ty.write_name(writer);
                        quote! { windows_core::Array::<#ty>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _ }
                    } else {
                        quote! { &mut result__ }
                    }
                }
            };

            if kind == InterfaceKind::Composable {
                quote! {
                    #(#args,)* core::ptr::null_mut(), &mut core::ptr::null_mut(), #return_arg
                }
            } else {
                quote! {
                    #(#args,)* #return_arg
                }
            }
        };

        let generics = params
            .iter()
            .enumerate()
            .filter_map(|(position, (ty, def))| {
                if is_convertible(ty, *def) {
                    let name: TokenStream = format!("P{position}").into();
                    Some(name)
                } else {
                    None
                }
            });

        let where_clause = {
            let constraints: Vec<_> = params
                .iter()
                .enumerate()
                .filter_map(|(position, (ty, def))| {
                    if is_convertible(ty, *def) {
                        let name: TokenStream = format!("P{position}").into();
                        let ty = ty.write_name(writer);

                        Some(quote! { #name: windows_core::Param<#ty>, })
                    } else {
                        None
                    }
                })
                .collect();

            if constraints.is_empty() {
                quote! {}
            } else {
                quote! { where #(#constraints)* }
            }
        };

        let params = params.iter().enumerate().map(|(position, param)| {
            let name = to_ident(&param.1.name().to_lowercase());
            let kind = param.0.write_name(writer);
            let default_type = param.0.write_default(writer);

            if param.1.flags().contains(ParamAttributes::In) {
                if param.0.is_winrt_array() {
                    quote! { #name: &[#default_type], }
                } else if is_convertible(&param.0, param.1) {
                    let kind: TokenStream = format!("P{position}").into();
                    quote! { #name: #kind, }
                } else if param.0.is_copyable() {
                    quote! { #name: #kind, }
                } else {
                    quote! { #name: &#kind, }
                }
            } else if param.0.is_winrt_array() {
                quote! { #name: &mut [#default_type], }
            } else if param.0.is_winrt_array_ref() {
                quote! { #name: &mut windows_core::Array<#kind>, }
            } else {
                quote! { #name: &mut #default_type, }
            }
        });

        let return_type = match &self.signature.return_type.0 {
            Type::Void => quote! { () },
            _ => {
                let tokens = self.signature.return_type.0.write_name(writer);
                if self.signature.return_type.0.is_winrt_array() {
                    quote! { windows_core::Array<#tokens> }
                } else {
                    quote! { #tokens }
                }
            }
        };

        let noexcept = self.def.has_attribute("NoExceptionAttribute");

        let return_type = if noexcept {
            if self.signature.return_type.0.is_interface() {
                quote! { -> Option<#return_type> }
            } else if self.signature.return_type.0 == Type::Void {
                quote! {}
            } else {
                quote! { -> #return_type }
            }
        } else {
            quote! { -> windows_core::Result<#return_type> }
        };

        let vname = virtual_names.add(self.def);
        let vcall = quote! { (windows_core::Interface::vtable(this).#vname)(windows_core::Interface::as_raw(this), #args) };

        let vcall = match &self.signature.return_type.0 {
            Type::Void => {
                if noexcept {
                    quote! {
                        let hresult__ = #vcall;
                        debug_assert!(hresult__.0 == 0);
                    }
                } else {
                    quote! {
                        #vcall.ok()
                    }
                }
            }
            _ if self.signature.return_type.0.is_winrt_array() => {
                if noexcept {
                    quote! {
                        let mut result__ = core::mem::MaybeUninit::zeroed();
                        let hresult__ = #vcall;
                        debug_assert!(hresult__.0 == 0);
                        result__.assume_init()
                    }
                } else {
                    quote! {
                        let mut result__ = core::mem::MaybeUninit::zeroed();
                        #vcall
                            .map(|| result__.assume_init())
                    }
                }
            }
            _ => {
                if noexcept {
                    if self.signature.return_type.0.is_copyable() {
                        quote! {
                        let mut result__ = core::mem::zeroed();
                        let hresult__ = #vcall;
                        debug_assert!(hresult__.0 == 0);
                        result__ }
                    } else {
                        quote! {
                        let mut result__ = core::mem::zeroed();
                        let hresult__ = #vcall;
                        debug_assert!(hresult__.0 == 0);
                        core::mem::transmute(result__) }
                    }
                } else if !self.signature.return_type.0.is_convertible() {
                    if self.signature.return_type.0.is_primitive() {
                        quote! {
                        let mut result__ = core::mem::zeroed();
                        #vcall
                        .map(||result__) }
                    } else {
                        quote! {
                        let mut result__ = core::mem::zeroed();
                        #vcall
                        .map(||core::mem::transmute(result__)) }
                    }
                } else {
                    quote! { let mut result__ = core::mem::zeroed();
                    #vcall.and_then(|| windows_core::Type::from_abi(result__)) }
                }
            }
        };

        match kind {
            InterfaceKind::Default => quote! {
                pub fn #name<#(#generics,)*>(&self, #(#params)*) #return_type #where_clause {
                    let this = self;
                    unsafe {
                        #vcall
                    }
                }
            },
            InterfaceKind::None | InterfaceKind::Base => {
                let unwrap = if noexcept {
                    quote! { .unwrap() }
                } else {
                    quote! { ? }
                };

                quote! {
                    pub fn #name<#(#generics,)*>(&self, #(#params)*) #return_type #where_clause {
                        let this = &windows_core::Interface::cast::<#interface_name>(self)#unwrap;
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Static | InterfaceKind::Composable => {
                quote! {
                    pub fn #name<#(#generics,)*>(#(#params)*) #return_type #where_clause {
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
        }
    }
}

fn is_convertible(ty: &Type, param: Param) -> bool {
    param.flags().contains(ParamAttributes::In) && ty.is_convertible()
}
