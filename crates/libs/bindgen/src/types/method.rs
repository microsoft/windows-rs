use super::*;

// WinRT methods only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub def: MethodDef,
    pub signature: Signature,
    pub dependencies: TypeMap,
}

impl Method {
    pub fn new(def: MethodDef, generics: &[Type], reader: &Reader) -> Self {
        let signature = def.method_signature("", generics, reader);
        let dependencies = signature.dependencies(reader);

        Self {
            def,
            signature,
            dependencies,
        }
    }

    pub fn write_cfg(&self, config: &Config, parent: &Cfg, not: bool) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        parent
            .difference(&self.dependencies, config)
            .write(config, not)
    }

    pub fn write_upcall(&self, inner: TokenStream, this: bool, config: &Config) -> TokenStream {
        let reader = config.reader;
        let noexcept = self.def.has_attribute("NoExceptionAttribute");

        let invoke_args = self.signature
        .params
        .iter()
        .map(|param| {
            let name = param.write_ident();
            let abi_size_name: TokenStream = format!("{}_array_size", param.write_ident()).into();

            if param.is_input() {
                if param.is_winrt_array() {
                    quote! { core::slice::from_raw_parts(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
                } else if param.is_primitive(reader) {
                    quote! { #name }
                } else if param.is_const_ref() || param.is_interface() || matches!(&param.ty, Type::Generic(_))  {
                    quote! { core::mem::transmute_copy(&#name) }
                } else {
                    quote! { core::mem::transmute(&#name) }
                }
            } else if param.is_winrt_array() {
                quote! { core::slice::from_raw_parts_mut(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
            } else if param.is_winrt_array_ref() {
                quote! { &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&#name), #abi_size_name) }
            } else {
                quote! { core::mem::transmute_copy(&#name) }
            }
        });

        let this = if this {
            quote! { this, }
        } else {
            quote! {}
        };

        match &self.signature.return_type {
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
            _ if self.signature.return_type.is_winrt_array() => {
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
                // For copyable types the Rust and ABI types are identical. For non-copyable
                // types the ABI type differs, requiring transmute_copy and forget to avoid
                // a double-drop.
                let write_result = if self.signature.return_type.is_copyable(reader) {
                    quote! { result__.write(ok__); }
                } else {
                    quote! {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                    }
                };

                if noexcept {
                    quote! {
                        let ok__ = #inner(#this #(#invoke_args,)*);
                        #write_result
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        match #inner(#this #(#invoke_args,)*) {
                            Ok(ok__) => {
                                #write_result
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
        config: &Config,
        named_params: bool,
        has_this: bool,
    ) -> TokenStream {
        let noexcept = self.def.has_attribute("NoExceptionAttribute");

        let params = self.signature.params.iter().map(|p| {
            let default_type = p.write_default(config);

            let sig = if p.is_input() {
                if p.is_winrt_array() {
                    quote! { &[#default_type] }
                } else if p.is_primitive(config.reader) {
                    quote! { #default_type }
                } else if p.is_interface() || matches!(&p.ty, Type::Generic(_)) {
                    let type_name = p.write_name(config);
                    quote! { windows_core::Ref<#type_name> }
                } else {
                    quote! { &#default_type }
                }
            } else if p.is_winrt_array() {
                quote! { &mut [#default_type] }
            } else if p.is_winrt_array_ref() {
                let kind = p.write_name(config);
                quote! { &mut windows_core::Array<#kind> }
            } else if p.is_interface() {
                let type_name = p.write_name(config);
                quote! { windows_core::OutRef<#type_name> }
            } else {
                quote! { &mut #default_type }
            };

            if named_params {
                let name = to_ident(p.def.name());
                quote! { #name: #sig }
            } else {
                sig
            }
        });

        let return_type_tokens = if self.signature.return_type == Type::Void {
            quote! { () }
        } else {
            let tokens = self.signature.return_type.write_name(config);

            if self.signature.return_type.is_winrt_array() {
                quote! { windows_core::Array<#tokens> }
            } else {
                tokens
            }
        };

        let return_type_tokens = if noexcept {
            if self.signature.return_type.is_interface() {
                quote! { -> Option<#return_type_tokens> }
            } else if self.signature.return_type == Type::Void {
                quote! {}
            } else {
                quote! { -> #return_type_tokens }
            }
        } else {
            let result = config.write_result();
            quote! { -> #result Result<#return_type_tokens> }
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

    pub fn write_abi(&self, config: &Config, named_params: bool) -> TokenStream {
        let args = self.signature.params.iter().map(|param| {
            let name = param.write_ident();
            let abi = param.write_abi(config);
            let abi_size_name: TokenStream = format!("{}_array_size", name.as_str()).into();

            if param.is_input() {
                if param.is_winrt_array() {
                    if named_params {
                        quote! { #abi_size_name: u32, #name: *const #abi }
                    } else {
                        quote! { u32, *const #abi }
                    }
                } else if param.is_const_ref() {
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
            } else if param.is_winrt_array() {
                if named_params {
                    quote! { #abi_size_name: u32, #name: *mut #abi }
                } else {
                    quote! { u32, *mut #abi }
                }
            } else if param.is_winrt_array_ref() {
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

        let return_arg = match &self.signature.return_type {
            Type::Void => quote! {},
            Type::Array(ty) => {
                let ty = ty.write_abi(config);
                if named_params {
                    quote! { result_size__: *mut u32, result__: *mut *mut #ty }
                } else {
                    quote! { *mut u32, *mut *mut #ty }
                }
            }
            ty => {
                let ty = ty.write_abi(config);
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
        config: &Config,
        interface: Option<&Interface>,
        kind: InterfaceKind,
        method_names: &mut MethodNames,
        virtual_names: &mut MethodNames,
    ) -> TokenStream {
        let params = if kind == InterfaceKind::Composable {
            &self.signature.params[..self.signature.params.len() - 2]
        } else {
            &self.signature.params
        };

        let (name, name_compose) = if kind == InterfaceKind::Composable && params.is_empty() {
            // Default parameterless composable constructor: `new` for the non-aggregating
            // entry, `compose` for the aggregating entry.
            (quote!(new), quote!(compose))
        } else if kind == InterfaceKind::Composable {
            let name = method_names.add(self.def);
            let compose = name.join("_compose");
            (name, compose)
        } else {
            (method_names.add(self.def), TokenStream::new())
        };

        let typed_args: Vec<TokenStream> = params.iter().map(|param|{
            let name = param.write_ident();

            if param.is_input() {
                if param.is_winrt_array() {
                    if param.is_copyable(config.reader) {
                        quote! { #name.len().try_into().unwrap(), #name.as_ptr() }
                    } else {
                        quote! { #name.len().try_into().unwrap(), core::mem::transmute(#name.as_ptr()) }
                    }
                } else if param.ireference_inner(config.reader).is_some() {
                    // Sugared `Option<T>` -> materialized `Option<IReference<T>>` in
                    // the local `name__` (see prelude below). `Param<T> for Option<&T>`
                    // turns `None` into a null abi pointer.
                    let local: TokenStream = format!("{}__", name.as_str()).into();
                    quote! { windows_core::Param::param(#local.as_ref()).abi() }
                } else if param.is_convertible() {
                    quote! { #name.param().abi() }
                } else if param.is_copyable(config.reader) {
                    if param.is_const_ref() {
                        quote! { &#name }
                    } else {
                        quote! { #name }
                    }
                } else {
                    quote! { core::mem::transmute_copy(#name) }
                }
            } else if param.is_winrt_array() {
                if param.is_copyable(config.reader) {
                    quote! { #name.len().try_into().unwrap(), #name.as_mut_ptr() }
                } else {
                    quote! { #name.len().try_into().unwrap(), core::mem::transmute_copy(&#name) }
                }
            } else if param.is_winrt_array_ref() {
                quote! { #name.set_abi_len(), #name as *mut _ as _ }
            } else if param.is_copyable(config.reader) {
                quote! { #name }
            } else {
                quote! { #name as *mut _ as _ }
            }
        }).collect();

        let return_arg = match &self.signature.return_type {
            Type::Void => quote! {},
            ty => {
                if ty.is_winrt_array() {
                    let ty = ty.write_name(config);
                    quote! { windows_core::Array::<#ty>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _ }
                } else {
                    quote! { &mut result__ }
                }
            }
        };

        let args = if kind == InterfaceKind::Composable {
            // Composable factories take `outer` and an out-pointer for `inner` as the last
            // two parameters before the result. The non-aggregating entry passes nulls.
            quote! {
                #(#typed_args,)* core::ptr::null_mut(), &mut core::ptr::null_mut(), #return_arg
            }
        } else {
            quote! {
                #(#typed_args,)* #return_arg
            }
        };

        let compose_args = if kind == InterfaceKind::Composable {
            quote! {
                #(#typed_args,)* core::mem::transmute_copy(&derived__), base__ as *mut _ as _, #return_arg
            }
        } else {
            TokenStream::new()
        };

        // Helper: does this param need a generic `P{n}: Into<IReference<HSTRING>>`?
        // (only IReference<HSTRING> sugar; value-like inner doesn't need a generic).
        let is_ireference_string = |param: &Param| -> bool {
            matches!(param.ireference_inner(config.reader), Some(Type::String))
        };

        let generics: Vec<TokenStream> = params
            .iter()
            .enumerate()
            .filter_map(|(position, param)| {
                if is_ireference_string(param)
                    || (param.is_convertible() && param.ireference_inner(config.reader).is_none())
                {
                    let name: TokenStream = format!("P{position}").into();
                    Some(name)
                } else {
                    None
                }
            })
            .collect();

        let where_clause = {
            let constraints: Vec<_> = params
                .iter()
                .enumerate()
                .filter_map(|(position, param)| {
                    let name: TokenStream = format!("P{position}").into();
                    if is_ireference_string(param) {
                        let iref = param.ty.write_name(config);
                        Some(quote! { #name: core::convert::Into<#iref>, })
                    } else if param.is_convertible()
                        && param.ireference_inner(config.reader).is_none()
                    {
                        let ty = param.write_name(config);
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

        let where_clause_compose = if kind == InterfaceKind::Composable {
            let constraints: Vec<_> = params
                .iter()
                .enumerate()
                .filter_map(|(position, param)| {
                    let name: TokenStream = format!("P{position}").into();
                    if is_ireference_string(param) {
                        let iref = param.ty.write_name(config);
                        Some(quote! { #name: core::convert::Into<#iref>, })
                    } else if param.is_convertible()
                        && param.ireference_inner(config.reader).is_none()
                    {
                        let ty = param.write_name(config);
                        Some(quote! { #name: windows_core::Param<#ty>, })
                    } else {
                        None
                    }
                })
                .collect();
            quote! { where #(#constraints)* T: windows_core::Compose, }
        } else {
            TokenStream::new()
        };

        // Prelude statements that materialize IReference<T> wrappers for sugared
        // `Option<T>` parameters. Bound just outside the unsafe block so the
        // wrapper is dropped *after* the vtable call returns.
        let prelude = {
            let lines: Vec<_> = params
                .iter()
                .enumerate()
                .filter_map(|(position, param)| {
                    param.ireference_inner(config.reader)?;
                    let name = param.write_ident();
                    let local: TokenStream = format!("{}__", name.as_str()).into();
                    let iref = param.ty.write_name(config);
                    if is_ireference_string(param) {
                        let pname: TokenStream = format!("P{position}").into();
                        Some(quote! { let #local = #name.map(<#pname as core::convert::Into<#iref>>::into); })
                    } else {
                        Some(quote! { let #local = #name.map(<#iref as core::convert::From<_>>::from); })
                    }
                })
                .collect();
            quote! { #(#lines)* }
        };

        let params: Vec<TokenStream> = params
            .iter()
            .enumerate()
            .map(|(position, param)| {
                let name = param.write_ident();
                let kind = param.write_name(config);
                let default_type = param.write_default(config);

                if param.is_input() {
                    if param.is_winrt_array() {
                        quote! { #name: &[#default_type], }
                    } else if is_ireference_string(param) {
                        let pname: TokenStream = format!("P{position}").into();
                        quote! { #name: Option<#pname>, }
                    } else if let Some(inner) = param.ireference_inner(config.reader) {
                        let inner_name = inner.write_name(config);
                        quote! { #name: Option<#inner_name>, }
                    } else if param.is_convertible() {
                        let kind: TokenStream = format!("P{position}").into();
                        quote! { #name: #kind, }
                    } else if param.is_copyable(config.reader) {
                        quote! { #name: #kind, }
                    } else {
                        quote! { #name: &#kind, }
                    }
                } else if param.is_winrt_array() {
                    quote! { #name: &mut [#default_type], }
                } else if param.is_winrt_array_ref() {
                    quote! { #name: &mut windows_core::Array<#kind>, }
                } else {
                    quote! { #name: &mut #default_type, }
                }
            })
            .collect();

        let noexcept = self.def.has_attribute("NoExceptionAttribute");
        let assert_success = quote! { debug_assert!(hresult__.0 == 0); };

        // Detect return-position `IReference<T>` sugar: when the return type is a
        // sugarable `Windows.Foundation.IReference<T>`, unwrap to `Result<T>`
        // (or `Result<HSTRING>`) by chaining `.Value()` after `from_abi`.
        // Limited to non-noexcept, non-array returns.
        let return_unwrap_inner = if !noexcept && !self.signature.return_type.is_winrt_array() {
            self.signature
                .return_type
                .ireference_inner_for_sugar(config.reader)
        } else {
            None
        };

        let return_type = match &self.signature.return_type {
            Type::Void => quote! { () },
            _ => {
                let tokens = if let Some(inner) = return_unwrap_inner {
                    inner.write_name(config)
                } else {
                    self.signature.return_type.write_name(config)
                };
                if self.signature.return_type.is_winrt_array() {
                    quote! { windows_core::Array<#tokens> }
                } else {
                    quote! { #tokens }
                }
            }
        };

        let return_type = if noexcept {
            if self.signature.return_type.is_interface() {
                quote! { -> Option<#return_type> }
            } else if self.signature.return_type == Type::Void {
                quote! {}
            } else {
                quote! { -> #return_type }
            }
        } else {
            let result = config.write_result();
            quote! { -> #result Result<#return_type> }
        };

        let vname = virtual_names.add(self.def);

        let receiver: TokenStream = if kind == InterfaceKind::Default {
            quote! { self }
        } else {
            quote! { this }
        };

        let build_vcall = |args: &TokenStream| -> TokenStream {
            let vcall = quote! { (windows_core::Interface::vtable(#receiver).#vname)(windows_core::Interface::as_raw(#receiver), #args) };

            match &self.signature.return_type {
                Type::Void => {
                    if noexcept {
                        quote! {
                            let hresult__ = #vcall;
                            #assert_success
                        }
                    } else {
                        quote! {
                            #vcall.ok()
                        }
                    }
                }
                _ if self.signature.return_type.is_winrt_array() => {
                    if noexcept {
                        quote! {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            let hresult__ = #vcall;
                            #assert_success
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
                        if self.signature.return_type.is_copyable(config.reader) {
                            quote! {
                                let mut result__ = core::mem::zeroed();
                                let hresult__ = #vcall;
                                #assert_success
                                result__
                            }
                        } else {
                            quote! {
                                let mut result__ = core::mem::zeroed();
                                let hresult__ = #vcall;
                                #assert_success
                                core::mem::transmute(result__)
                            }
                        }
                    } else {
                        let map = self.signature.return_type.write_result_map(config.reader);

                        if return_unwrap_inner.is_some() {
                            // Sugared `Result<IReference<T>>` -> `Result<T>`:
                            // materialize the IReference and then call `.Value()`.
                            // Explicit type annotation pins `from_abi`'s generic.
                            let iref = self.signature.return_type.write_name(config);
                            quote! {
                                let mut result__ = core::mem::zeroed();
                                #vcall.#map.and_then(|r__: #iref| r__.Value())
                            }
                        } else {
                            quote! {
                                let mut result__ = core::mem::zeroed();
                                #vcall.#map
                            }
                        }
                    }
                }
            }
        };

        let vcall = build_vcall(&args);

        match kind {
            InterfaceKind::Default => quote! {
                pub fn #name<#(#generics,)*>(&self, #(#params)*) #return_type #where_clause {
                    #prelude
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

                let interface_name = interface.unwrap().write_name(config);

                quote! {
                    pub fn #name<#(#generics,)*>(&self, #(#params)*) #return_type #where_clause {
                        let this = &windows_core::Interface::cast::<#interface_name>(self)#unwrap;
                        #prelude
                        unsafe {
                            #vcall
                        }
                    }
                }
            }
            InterfaceKind::Static => {
                let interface_name = to_ident(trim_tick(interface.unwrap().def.name()));

                quote! {
                    pub fn #name<#(#generics,)*>(#(#params)*) #return_type #where_clause {
                        #prelude
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
            InterfaceKind::Composable => {
                // Composable factories emit two entries: a non-aggregating `name` and an
                // aggregating `name_compose` that takes a `T: Compose`. We return `result__`
                // (the delegating default interface) rather than `derived__` so that
                // AddRef/Release on the returned value keeps the outer alive.
                let interface_name = to_ident(trim_tick(interface.unwrap().def.name()));

                quote! {
                    pub fn #name<#(#generics,)*>(#(#params)*) #return_type #where_clause {
                        #prelude
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                    pub fn #name_compose<#(#generics,)* T>(#(#params)* compose: T) #return_type #where_clause_compose {
                        #prelude
                        Self::#interface_name(|this| unsafe {
                            let (derived__, base__) = windows_core::Compose::compose(compose);
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(#receiver).#vname)(windows_core::Interface::as_raw(#receiver), #compose_args).ok()?;
                            // Keep `derived__` alive until the factory returns; its owning
                            // ref is replaced by the delegating ref in `result__`.
                            let _ = &derived__;
                            windows_core::Type::from_abi(result__)
                        })
                    }
                }
            }
        }
    }
}
