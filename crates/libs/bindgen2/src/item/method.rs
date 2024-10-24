use super::*;

// This is WinRT methods only
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub def: MethodDef,
    pub generics: Vec<Type>,
    pub signature: Signature,
    pub dependencies: Dependencies,
}

impl Method {
    pub fn new(def: MethodDef, generics: &[Type]) -> Self {
        let signature = def.signature(generics);

        let mut dependencies = Dependencies::new();
        signature.dependencies(&mut dependencies);

        Self {
            def,
            generics: generics.to_vec(),
            signature,
            dependencies,
        }
    }

    // pub fn included(&self, filter: &NameTree) -> bool {
    //     self.dependencies.included(filter)
    // }

    pub fn write_vtbl(&self, writer: &Writer, named_params: bool, virtual_names: &mut MethodNames) -> TokenStream {
        let name = virtual_names.add(self.def);

        let args = {
            let args = self.signature.params.iter().map(|param| {
                let name = to_ident(&param.1.name());
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
                ty =>  {
                    let ty = ty.write_abi(writer);
                    if named_params {
                        quote! { result__: *mut #ty }
                    } else {
                        quote! { *mut #ty }
                    }
                }
            };

            quote! {
                #(#args,)* #return_arg
            }
        };

        quote! { 
            pub #name: unsafe extern "system" fn(*mut core::ffi::c_void, #args) -> windows_core::HRESULT,
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
        // TODO: for config.package the dependencies need to be used to generate [cfg] and for
        // whether to include the method at all based on filtering/exclusions

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
                let name = to_ident(&param.1.name());

                if param.1.flags().contains(ParamAttributes::In) {
                    if param.0.is_winrt_array() {
                        if param.0.is_blittable() {
                            quote! { #name.len().try_into().unwrap(), #name.as_ptr() }
                        } else {
                            quote! { #name.len().try_into().unwrap(), core::mem::transmute(#name.as_ptr()) }
                        }
                    } else if param.0.is_borrowed() {
                        quote! { #name.param().abi() }
                    } else if param.0.is_blittable() {
                        if param.0.is_const_ref() {
                            quote! { &#name }
                        } else {
                            quote! { #name }
                        }
                    } else {
                        quote! { core::mem::transmute_copy(#name) }
                    }
                } else if param.0.is_winrt_array() {
                    if param.0.is_blittable() {
                        quote! { #name.len().try_into().unwrap(), #name.as_mut_ptr() }
                    } else {
                        quote! { #name.len().try_into().unwrap(), core::mem::transmute_copy(&#name) }
                    }
                } else if param.0.is_winrt_array_ref() {
                    quote! { #name.set_abi_len(), #name as *mut _ as _ }
                } else if param.0.is_blittable() {
                    quote! { #name }
                } else {
                    quote! { #name as *mut _ as _ }
                }
            });

            let return_arg = match &self.signature.return_type.0 {
                Type::Void => quote! {},
                ty => if ty.is_winrt_array() {
                    let ty = ty.write(writer);
                    quote! { windows_core::Array::<#ty>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _ }
                } else {
                    quote! { &mut result__ }
                },
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

        let params = params.iter().enumerate().map(|(position, param)| {
            let name = to_ident(&param.1.name());
            let kind = param.0.write(writer);
            let default_type = param.0.write_default(writer);

            if param.1.flags().contains(ParamAttributes::In) {
                if param.0.is_winrt_array() {
                    quote! { #name: &[#default_type], }
                } else if is_convertible(&param.0, param.1) {
                    let kind: TokenStream = format!("P{position}").into();
                    quote! { #name: #kind, }
                } else if param.0.is_blittable() {
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
                let tokens = self.signature.return_type.0.write(writer);
                if self.signature.return_type.0.is_winrt_array() {
                    quote! { windows_core::Array<#tokens> }
                } else {
                    quote! { #tokens }
                }
            }
        };

        let noexcept = self.def.has_attribute("NoExceptionAttribute");
    
        let return_type = if noexcept {
            if self.signature.return_type.0.is_nullable() {
                quote! { -> Option<#return_type> }
            } else if self.signature.return_type.0 == Type::Void {
                quote! {}
            } else {
                quote! { -> #return_type }
            }
        } else {
            quote! { -> windows_core::Result<#return_type> }
        };

        // TODO: have test for difference between name and vname
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
                    if self.signature.return_type.0.is_blittable() {
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
                } else if self.signature.return_type.0.is_blittable() {
                    quote! {
                    let mut result__ = core::mem::zeroed();
                    #vcall
                    .map(||result__) }
                } else {
                    quote! { let mut result__ = core::mem::zeroed();
                    #vcall.and_then(|| windows_core::Type::from_abi(result__)) }
                }
            }
        };

        let features = quote!{};
        let generics = quote!{};
        let where_clause = quote!{};

        match kind {
            InterfaceKind::Default => quote! {
                #features
                pub fn #name<#generics>(&self, #(#params)*) #return_type #where_clause {
                    let this = self;
                    unsafe {
                        #vcall
                    }
                }
            },
            InterfaceKind::None
            | InterfaceKind::Base => {
                let unwrap = if noexcept {
                    quote! { .unwrap() }
                } else {
                    quote! { ? }
                };
    
                quote! {
                    #features
                    pub fn #name<#generics>(&self, #(#params)*) #return_type #where_clause {
                        let this = &windows_core::Interface::cast::<#interface_name>(self)#unwrap;
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Static | InterfaceKind::Composable => {
                quote! {
                    #features
                    pub fn #name<#generics>(#(#params)*) #return_type #where_clause {
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
        }
    }
}

fn is_convertible(_ty: &Type, _param: Param) -> bool {
    false
}