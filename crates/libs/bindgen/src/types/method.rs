use super::*;

// This is WinRT methods only
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
        let noexcept = config.noexcept || self.def.has_attribute("NoExceptionAttribute");

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
                // For copyable types the Rust type and ABI type are identical, so write
                // directly.  For non-copyable types (e.g. interfaces) the ABI type differs
                // from the Rust type, so transmute_copy is required, and the value must be
                // forgotten to prevent a double-drop.
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
                        // use `ptr::write` since `result` could be uninitialized
                        #write_result
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        match #inner(#this #(#invoke_args,)*) {
                            Ok(ok__) => {
                                // use `ptr::write` since `result` could be uninitialized
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
        let noexcept = config.noexcept || self.def.has_attribute("NoExceptionAttribute");

        let params = self.signature.params.iter().map(|p| {
            let default_type = p.write_default(config);

            let sig = if p.is_input() {
                if p.is_winrt_array() {
                    quote! { &[#default_type] }
                } else if p.is_primitive(config.reader) {
                    quote! { #default_type }
                } else if p.is_interface() {
                    let type_name = p.write_name(config);
                    quote! { windows_core::Ref<#type_name> }
                } else if matches!(&p.ty, Type::Generic(_)) {
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
            // The default parameterless composable constructor maps to `new` for the
            // non-aggregating entry and `compose` for the aggregating entry.
            (quote!(new), quote!(compose))
        } else if kind == InterfaceKind::Composable {
            let name = method_names.add(self.def);
            // Build `<name>_compose` for the aggregating variant by string-concatenating
            // `_compose` onto the existing name token. `TokenStream::join` here is the
            // bindgen string-builder helper (defined in `tokens/token_stream.rs`), not
            // a real proc-macro2 method — it appends the literal suffix.
            let compose = name.join("_compose");
            (name, compose)
        } else {
            (method_names.add(self.def), TokenStream::new())
        };

        // Per-typed-parameter argument expressions (no outer/inner pair). Reused twice for
        // composable factories (the non-aggregating and aggregating entries).
        let typed_args: Vec<TokenStream> = params.iter().map(|param|{
            let name = param.write_ident();

            if param.is_input() {
                if param.is_winrt_array() {
                    if param.is_copyable(config.reader) {
                        quote! { #name.len().try_into().unwrap(), #name.as_ptr() }
                    } else {
                        quote! { #name.len().try_into().unwrap(), core::mem::transmute(#name.as_ptr()) }
                    }
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

        // typed_args without the trailing return out-pointer; used by middleware-mode
        // helper emission where the helper supplies the out-pointer slot itself. The
        // trailing `,` is preserved when non-empty so it splices cleanly before the
        // helper-supplied `result__` argument. For Composable's primary (non-aggregating)
        // entry the two null placeholder slots (outer + inner) are spliced in here so
        // the helper-emitted call site retains the same ABI.
        let typed_args_only: TokenStream = if kind == InterfaceKind::Composable {
            quote! { #(#typed_args,)* core::ptr::null_mut(), &mut core::ptr::null_mut(), }
        } else if typed_args.is_empty() {
            TokenStream::new()
        } else {
            quote! { #(#typed_args,)* }
        };

        let args = if kind == InterfaceKind::Composable {
            // Composable factory methods take the `outer` controlling unknown and the
            // out-pointer for the `inner` non-delegating object as their last two
            // parameters before the result. The non-aggregating (`new`/named) entry passes
            // nulls for both.
            quote! {
                #(#typed_args,)* core::ptr::null_mut(), &mut core::ptr::null_mut(), #return_arg
            }
        } else {
            quote! {
                #(#typed_args,)* #return_arg
            }
        };

        // For composable factories the aggregating entry passes the outer `IInspectable`
        // (assembled by `Compose::compose`) and the writeback slot for the inner.
        let compose_args = if kind == InterfaceKind::Composable {
            quote! {
                #(#typed_args,)* core::mem::transmute_copy(&derived__), base__ as *mut _ as _, #return_arg
            }
        } else {
            TokenStream::new()
        };

        let generics: Vec<TokenStream> = params
            .iter()
            .enumerate()
            .filter_map(|(position, param)| {
                if param.is_convertible() {
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
                    if param.is_convertible() {
                        let name: TokenStream = format!("P{position}").into();
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

        // For the aggregating composable variant we splice an extra `T: windows_core::Compose`
        // bound onto the existing where clause (or introduce one if there isn't one).
        let where_clause_compose = if kind == InterfaceKind::Composable {
            let constraints: Vec<_> = params
                .iter()
                .enumerate()
                .filter_map(|(position, param)| {
                    if param.is_convertible() {
                        let name: TokenStream = format!("P{position}").into();
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

        let return_type = match &self.signature.return_type {
            Type::Void => quote! { () },
            _ => {
                let tokens = self.signature.return_type.write_name(config);
                if self.signature.return_type.is_winrt_array() {
                    quote! { windows_core::Array<#tokens> }
                } else {
                    quote! { #tokens }
                }
            }
        };

        let has_noexcept_attr = self.def.has_attribute("NoExceptionAttribute");
        let noexcept = config.noexcept || has_noexcept_attr;
        // When the method genuinely carries `NoExceptionAttribute` the metadata
        // contract guarantees success, so `debug_assert!` is sufficient. When
        // `noexcept` is only being assumed because of the `--noexcept` flag we
        // have no such guarantee, so use a real `assert!` that survives release
        // builds rather than silently returning a zeroed-out value.
        let assert_success = if has_noexcept_attr {
            quote! { debug_assert!(hresult__.0 == 0); }
        } else {
            quote! { assert!(hresult__.0 == 0); }
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

        // For Default methods the receiver is `self` directly; for all other kinds (None,
        // Base, Static, Composable) the caller binds a local `this` that the vcall uses.
        let receiver: TokenStream = if kind == InterfaceKind::Default {
            quote! { self }
        } else {
            quote! { this }
        };

        // Builds the full vtable call expression (including return-value handling) for the
        // given `args` token stream. This is wrapped in a closure so we can build a parallel
        // `_compose` variant for composable factory methods without duplicating the
        // return-type plumbing below.
        //
        // `typed_args_only` contains the in-parameter expressions without the trailing
        // out-pointer (or `null_mut()` placeholders for composable factories). When
        // middleware mode is enabled and the return shape fits one of the
        // `windows_core::imp::call_*` helpers, we emit a tail call into the helper using
        // `typed_args_only` so the helper supplies the `result__` slot itself. Otherwise
        // we fall back to the existing inline expansion using `args` (which already
        // splices in the trailing out-pointer).
        let build_vcall = |args: &TokenStream, typed_args_only: &TokenStream| -> TokenStream {
            let vcall = quote! { (windows_core::Interface::vtable(#receiver).#vname)(windows_core::Interface::as_raw(#receiver), #args) };

            // Middleware-mode helper emission. Only applies when:
            // * `--middleware` is set,
            // * the method does not use `noexcept`-style infallible return,
            // * the return is not a WinRT array.
            //
            // The helper covers all four `kind`s that go through `build_vcall`:
            //
            // * `Default` (`#receiver = self`),
            // * `None` / `Base` (`#receiver = this`, after a `cast::<…>` prelude),
            // * `Static` (`#receiver = this`, inside the `Self::IFoo(|this| { … })`
            //   factory closure),
            // * `Composable` primary (non-aggregating) entry (`#receiver = this`,
            //   inside the factory closure; the two outer/inner null placeholder
            //   slots are spliced into `typed_args_only` above).
            //
            // The `Composable` aggregating `_compose` variant goes through a
            // dedicated `call_compose` helper emitted in the `InterfaceKind::Composable`
            // arm below — it is *not* produced by this closure.
            //
            // It covers `Result<T>` interface returns (Abi = *mut c_void) and
            // `Result<T>` copyable returns (Abi = T) via the `Default`-bounded
            // `<T as Type<T>>::Abi` slot in `call_in_out`. The "transmute" branch
            // (CloneType such as HSTRING, where Abi = MaybeUninit<T>) is intentionally
            // not covered — it falls back to the inline expansion below.
            let middleware_eligible =
                config.middleware && !noexcept && !self.signature.return_type.is_winrt_array();

            if middleware_eligible {
                match &self.signature.return_type {
                    Type::Void => {
                        return quote! {
                            windows_core::imp::call_in(#receiver, |this__|
                                (windows_core::Interface::vtable(#receiver).#vname)(this__, #typed_args_only))
                        };
                    }
                    // `Type::Generic(_)` (a bare generic parameter such as `TResult`)
                    // is excluded: the helper requires `<T as Type<T>>::Abi: Default`,
                    // which isn't implied by the typical `T: RuntimeType + 'static`
                    // bound the generated impl block carries. Fall back to inline.
                    Type::Generic(_) => {}
                    return_type
                        if return_type.is_convertible()
                            || return_type.is_copyable(config.reader) =>
                    {
                        // `call_in_out` requires `<T as Type<T>>::Abi: Default`. This
                        // holds for interface (`Abi = *mut c_void`) and copyable
                        // (`Abi = T` for primitives/enums/BOOL/HANDLE) returns.
                        return quote! {
                            windows_core::imp::call_in_out(#receiver, |this__, result__|
                                (windows_core::Interface::vtable(#receiver).#vname)(this__, #typed_args_only result__))
                        };
                    }
                    _ => {} // CloneType etc. — fall through to inline expansion.
                }
            }

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

                        quote! {
                            let mut result__ = core::mem::zeroed();
                            #vcall.#map
                        }
                    }
                }
            }
        };

        let vcall = build_vcall(&args, &typed_args_only);

        match kind {
            InterfaceKind::Default => quote! {
                pub fn #name<#(#generics,)*>(&self, #(#params)*) #return_type #where_clause {
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
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                }
            }
            InterfaceKind::Composable => {
                // Composable factory methods get two entries: a non-aggregating one (the
                // caller does not provide a derived implementation) and an aggregating
                // `_compose` variant that takes a `T: Compose` and threads the outer
                // `IInspectable` and inner writeback slot into the vtable call.
                //
                // After a successful `CreateInstance(outer, &inner_out, &result_out)`:
                //
                // * `*inner_out` (here `*base__`) holds a strong ref to the **non-
                //   delegating** inner `IInspectable`. We retain ownership of it through
                //   the outer's `ComposeBase` slot — the outer drops the inner when the
                //   outer itself is destroyed.
                //
                // * `*result_out` (here `result__`) holds a strong ref to the aggregated
                //   default interface. Its IUnknown methods **delegate to the outer's
                //   controlling IUnknown**, so AddRef/Release on the returned value keeps
                //   the outer alive (which transitively keeps the inner alive). This is
                //   exactly the value the user wants back.
                //
                // We must therefore return `result__` and drop the local `derived__`
                // outer reference at end of scope. Returning `derived__.cast::<#return>()`
                // would be wrong: the outer's `QueryInterface` for `#return` falls
                // through to the non-delegating inner via the `ComposeBase` slot, giving
                // back a non-delegating pointer whose `Release` does not keep the outer
                // alive — once `derived__` drops, the outer is freed and the caller is
                // left with a dangling reference into the inner's vtables.
                let interface_name = to_ident(trim_tick(interface.unwrap().def.name()));

                // `_compose` body. In `--middleware` mode and when the return shape
                // fits (interface or copyable scalar — see same eligibility rules as
                // the `Default` arm), we collapse the body into a single tail call
                // through `windows_core::imp::call_compose`, which performs the
                // `Compose::compose` / vtable dispatch / `let _ = &derived__;`
                // keep-alive / `from_abi` sequence in one place.
                //
                // Otherwise (or when middleware mode is off), emit the original
                // inline expansion below.
                let compose_eligible = config.middleware
                    && !noexcept
                    && !self.signature.return_type.is_winrt_array()
                    && !matches!(&self.signature.return_type, Type::Generic(_))
                    && (self.signature.return_type.is_convertible()
                        || self.signature.return_type.is_copyable(config.reader));

                let compose_body = if compose_eligible {
                    quote! {
                        windows_core::imp::call_compose(this, compose, |this__, derived__, base__, result__|
                            (windows_core::Interface::vtable(this).#vname)(this__, #(#typed_args,)* derived__, base__, result__))
                    }
                } else {
                    quote! {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(#receiver).#vname)(windows_core::Interface::as_raw(#receiver), #compose_args).ok()?;
                        // Suppress unused-variable warning: `derived__` is held alive
                        // for the duration of the factory call (so the factory can
                        // store a back-pointer to the outer in the inner) and then
                        // dropped at scope end — its sole owning ref is replaced by
                        // the delegating ref baked into `result__`.
                        let _ = &derived__;
                        windows_core::Type::from_abi(result__)
                    }
                };

                quote! {
                    pub fn #name<#(#generics,)*>(#(#params)*) #return_type #where_clause {
                        Self::#interface_name(|this| unsafe { #vcall })
                    }
                    pub fn #name_compose<#(#generics,)* T>(#(#params)* compose: T) #return_type #where_clause_compose {
                        Self::#interface_name(|this| unsafe {
                            #compose_body
                        })
                    }
                }
            }
        }
    }
}
