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
        if !config.bindgen.layout.is_package() {
            return quote! {};
        }

        parent
            .difference(&self.dependencies, config)
            .write(config, not)
    }

    pub fn write_upcall(&self, inner: TokenStream, this: bool, config: &Config) -> TokenStream {
        let reader = config.reader;
        let noexcept = self.def.has_attribute("NoExceptionAttribute");

        let invoke_args = self.write_upcall_args(reader);

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
            Type::Array(element_type) => {
                // For copyable element types the Rust and ABI types are identical,
                // so the transmute would be from a type to itself.
                let write_result = if element_type.is_copyable(reader) {
                    quote! { result__.write(ok_data__); }
                } else {
                    quote! { result__.write(core::mem::transmute(ok_data__)); }
                };

                if noexcept {
                    quote! {
                        let ok__ = #inner(#this #(#invoke_args,)*);
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        #write_result
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        match #inner(#this #(#invoke_args,)*) {
                            Ok(ok__) => {
                                let (ok_data__, ok_data_len__) = ok__.into_abi();
                                #write_result
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

    /// Like `write_upcall`, but assumes the inner closure returns nothing and
    /// the boxed `Invoke` should simply return `S_OK`. Used by event-handler
    /// delegates generated under `--minimal`.
    pub fn write_upcall_no_return(
        &self,
        inner: TokenStream,
        this: bool,
        config: &Config,
    ) -> TokenStream {
        let invoke_args = self.write_upcall_args(config.reader);

        let this = if this {
            quote! { this, }
        } else {
            quote! {}
        };

        quote! {
            #inner(#this #(#invoke_args,)*);
            windows_core::HRESULT(0)
        }
    }

    fn write_upcall_args(&self, reader: &Reader) -> Vec<TokenStream> {
        self.signature
            .params
            .iter()
            .map(|param| {
                let name = param.write_ident();
                let abi_size_name: TokenStream =
                    format!("{}_array_size", param.write_ident()).parse().unwrap();

                if param.is_input() {
                    if param.is_winrt_array() {
                        quote! { core::slice::from_raw_parts(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
                    } else if param.is_primitive(reader) {
                        quote! { #name }
                    } else if param.is_const_ref()
                        || param.is_interface()
                        || matches!(&param.ty, Type::Generic(_))
                    {
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
            })
            .collect()
    }

    pub fn write_impl_signature(
        &self,
        config: &Config,
        named_params: bool,
        has_this: bool,
    ) -> TokenStream {
        let noexcept = self.def.has_attribute("NoExceptionAttribute");
        let params = self.write_impl_params(config, named_params);

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

    /// Returns the parenthesized parameter signature without a return type,
    /// suitable for use as the input portion of an `Fn(...)` bound when the
    /// caller doesn't need a return value (currently event-handler closures
    /// under `--minimal`).
    pub fn write_impl_signature_no_return(&self, config: &Config) -> TokenStream {
        let params = self.write_impl_params(config, false);
        quote! { (#(#params),*) }
    }

    fn write_impl_params(&self, config: &Config, named_params: bool) -> Vec<TokenStream> {
        self.signature
            .params
            .iter()
            .map(|p| {
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
            })
            .collect()
    }

    pub fn write_abi(&self, config: &Config, named_params: bool) -> TokenStream {
        let args = self.signature.params.iter().map(|param| {
            let name = param.write_ident();
            let abi = param.write_abi(config);
            let abi_size_name: TokenStream = format!("{name}_array_size").parse().unwrap();

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
                    let local: TokenStream = format!("{name}__").parse().unwrap();
                    quote! { windows_core::Param::param(#local.as_ref()).abi() }
                } else if param.is_convertible() {
                    quote! { #name.param().abi() }
                } else if config.bindgen.style.is_minimal() && param.is_input() && matches!(param.ty, Type::String) {
                    // In minimal mode, string params accept &str directly.
                    // Convert to HSTRING and pass its abi.
                    quote! { core::mem::transmute_copy(&windows_core::HSTRING::from(#name)) }
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
                #(#typed_args,)* core::ptr::null_mut(), core::ptr::null_mut(), #return_arg
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

        // In minimal mode, HSTRING input params accept `&str` directly — the generated
        // method body handles the conversion to HSTRING internally.
        let is_string_param = |param: &Param| -> bool {
            config.bindgen.style.is_minimal()
                && param.is_input()
                && matches!(param.ty, Type::String)
        };

        let generics: Vec<TokenStream> = params
            .iter()
            .enumerate()
            .filter_map(|(position, param)| {
                if is_ireference_string(param)
                    || (param.is_convertible() && param.ireference_inner(config.reader).is_none())
                {
                    let name: TokenStream = format!("P{position}").parse().unwrap();
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
                    let name: TokenStream = format!("P{position}").parse().unwrap();
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
                    let name: TokenStream = format!("P{position}").parse().unwrap();
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
                    let local: TokenStream = format!("{name}__").parse().unwrap();
                    let iref = param.ty.write_name(config);
                    if is_ireference_string(param) {
                        let pname: TokenStream = format!("P{position}").parse().unwrap();
                        Some(quote! { let #local = #name.map(<#pname as Into<#iref>>::into); })
                    } else {
                        Some(quote! { let #local = #name.map(<#iref as From<_>>::from); })
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
                        let pname: TokenStream = format!("P{position}").parse().unwrap();
                        quote! { #name: Option<#pname>, }
                    } else if let Some(inner) = param.ireference_inner(config.reader) {
                        let inner_name = inner.write_name(config);
                        quote! { #name: Option<#inner_name>, }
                    } else if is_string_param(param) {
                        quote! { #name: &str, }
                    } else if param.is_convertible() {
                        let kind: TokenStream = format!("P{position}").parse().unwrap();
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

        let return_type = if self.signature.return_type == Type::Void {
            quote! { () }
        } else {
            let tokens = if config.bindgen.style.is_minimal()
                && matches!(self.signature.return_type, Type::String)
            {
                quote! { String }
            } else if let Some(inner) = return_unwrap_inner {
                inner.write_name(config)
            } else {
                self.signature.return_type.write_name(config)
            };
            if self.signature.return_type.is_winrt_array() {
                quote! { windows_core::Array<#tokens> }
            } else {
                quote! { #tokens }
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
                        } else if config.bindgen.style.is_minimal()
                            && matches!(self.signature.return_type, Type::String)
                        {
                            quote! {
                                let mut result__ = core::mem::zeroed();
                                let hresult__ = #vcall;
                                #assert_success
                                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                                hstring.to_string_lossy()
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
                        } else if config.bindgen.style.is_minimal()
                            && matches!(self.signature.return_type, Type::String)
                        {
                            // In minimal mode, return String instead of HSTRING.
                            quote! {
                                let mut result__ = core::mem::zeroed();
                                #vcall.map(|| {
                                    let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                                    hstring.to_string_lossy()
                                })
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

        // minimal: suppress remove_* methods and replace add_* methods
        // with a combined wrapper returning EventRevoker.
        let raw_method_name = self.def.name();
        let is_event_add = !noexcept
            && config.bindgen.style.is_minimal()
            && self.def.flags().contains(MethodAttributes::SpecialName)
            && raw_method_name.starts_with("add_");
        let is_event_remove = config.bindgen.style.is_minimal()
            && self.def.flags().contains(MethodAttributes::SpecialName)
            && raw_method_name.starts_with("remove_");
        let suppress_event_remove = is_event_remove
            && kind != InterfaceKind::Composable
            && {
                let paired_event_add = format!("add_{}", &raw_method_name[7..]);
                matches!(self.def.parent(), MemberRefParent::TypeDef(def) if def.methods().any(|method| {
                    method.flags().contains(MethodAttributes::SpecialName)
                        && !method.has_attribute("NoExceptionAttribute")
                        && method.name() == paired_event_add
                }))
            };

        if suppress_event_remove {
            return quote! {};
        }

        if is_event_add && kind != InterfaceKind::Composable {
            // The event part (e.g. "Click" from "add_Click") determines the
            // vtable field name for the paired remove method.
            let event_part = &raw_method_name[4..];
            let remove_vname = if config.bindgen.style.is_minimal() {
                // Raw mode: vtbl field is "remove_Click"
                to_ident(&format!("remove_{event_part}"))
            } else {
                // Default mode: vtbl field is "RemoveClick"
                to_ident(&format!("Remove{event_part}"))
            };

            // Promote the delegate parameter (typically the only input) to a
            // generic closure so callers can pass a closure directly without
            // first constructing the delegate. `params` has been shadowed into
            // a `Vec<TokenStream>` of rendered declarations, so reach for
            // `self.signature.params` when inspecting the original `Param`s.
            let sig_params = &self.signature.params[..];
            let delegate_idx = sig_params
                .iter()
                .position(|p| matches!(&p.ty, Type::Delegate(_)));

            let (event_generics, event_where_clause, event_params_decl, event_args, event_prelude) =
                if let Some(idx) = delegate_idx {
                    let dp = &sig_params[idx];
                    let Type::Delegate(d) = &dp.ty else {
                        unreachable!()
                    };
                    let invoke = d.method(config.reader);
                    let delegate_name = d.write_name(config);
                    let pname = dp.write_ident();

                    // The wrapper accepts a closure constrained to the
                    // delegate's parameter list but with no return type.
                    let fn_sig_no_return = invoke.write_impl_signature_no_return(config);

                    // If the delegate is being generated by this same bindgen
                    // run under `--minimal`, its `new` accepts the void-
                    // returning closure directly and returns S_OK internally,
                    // so we can pass `handler` straight through. Otherwise
                    // (the delegate comes from a referenced crate built
                    // without `--minimal`), wrap the handler to satisfy its
                    // `Fn(...) -> Result<()>` constraint.
                    let delegate_is_local_minimal =
                        config.references.contains(d.type_name()).is_none();

                    // Rebuild typed_args, replacing the delegate slot with a raw
                    // interface pointer for the locally-constructed delegate.
                    let event_typed_args: Vec<TokenStream> = typed_args
                        .iter()
                        .enumerate()
                        .map(|(i, ts)| {
                            if i == idx {
                                quote! { windows_core::Interface::as_raw(&#pname) }
                            } else {
                                ts.clone()
                            }
                        })
                        .collect();
                    let event_args = quote! { #(#event_typed_args,)* &mut result__ };

                    // Keep all P{n} generics for non-delegate params; add `F` for
                    // the closure that replaces the delegate parameter.
                    let mut event_generics: Vec<TokenStream> = sig_params
                        .iter()
                        .enumerate()
                        .filter_map(|(position, param)| {
                            if position == idx {
                                return None;
                            }
                            if is_ireference_string(param)
                                || (param.is_convertible()
                                    && param.ireference_inner(config.reader).is_none())
                            {
                                let name: TokenStream = format!("P{position}").parse().unwrap();
                                Some(name)
                            } else {
                                None
                            }
                        })
                        .collect();
                    event_generics.push(quote! { F });

                    let other_constraints: Vec<TokenStream> = sig_params
                        .iter()
                        .enumerate()
                        .filter_map(|(position, param)| {
                            if position == idx {
                                return None;
                            }
                            let name: TokenStream = format!("P{position}").parse().unwrap();
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

                    let event_where_clause = {
                        // Drop Send when the delegate is locally generated
                        // under minimal mode — we inline the DelegateBox and
                        // bypass the delegate's new() bounds. When the delegate
                        // is referenced (external crate), its new() still
                        // requires Send so we must keep the bound here too.
                        if delegate_is_local_minimal {
                            quote! {
                                where #(#other_constraints)* F: Fn #fn_sig_no_return + 'static,
                            }
                        } else {
                            quote! {
                                where #(#other_constraints)* F: Fn #fn_sig_no_return + Send + 'static,
                            }
                        }
                    };

                    // Reuse the rendered declarations for non-delegate params and
                    // replace the delegate slot with `handler: F`.
                    let event_params_decl: Vec<TokenStream> = params
                        .iter()
                        .enumerate()
                        .map(|(i, rendered)| {
                            if i == idx {
                                let pname = sig_params[i].write_ident();
                                quote! { #pname: F, }
                            } else {
                                rendered.clone()
                            }
                        })
                        .collect();

                    let event_prelude = if delegate_is_local_minimal {
                        // Inline DelegateBox construction directly instead of
                        // calling Delegate::new(). This decouples the event
                        // wrapper's Send bound from the delegate constructor's
                        // bound — the wrapper's own where clause is the sole
                        // authority on whether F must be Send.
                        let boxed_name: TokenStream =
                            format!("{}Box", trim_tick(d.def.name())).parse().unwrap();
                        let generic_names = d.generics.iter().map(|ty| ty.write_name(config));
                        let generic_names = quote! { #(#generic_names,)* };
                        quote! {
                            #prelude
                            let #pname: #delegate_name = {
                                let com = windows_core::imp::DelegateBox::<#delegate_name, F>::new(&#boxed_name::<#generic_names F>::VTABLE, #pname);
                                unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                            };
                        }
                    } else {
                        // Synthetic argument names for the wrapping closure
                        // that injects `Ok(())`. These only exist inside the
                        // inner closure so they don't collide with any
                        // user-visible parameter names.
                        let invoke_arg_idents: Vec<TokenStream> =
                            (0..invoke.signature.params.len())
                                .map(|i| format!("a{i}").parse().unwrap())
                                .collect();
                        quote! {
                            #prelude
                            let #pname = <#delegate_name>::new(move |#(#invoke_arg_idents),*| {
                                #pname(#(#invoke_arg_idents),*);
                                Ok(())
                            });
                        }
                    };

                    (
                        event_generics,
                        event_where_clause,
                        event_params_decl,
                        event_args,
                        event_prelude,
                    )
                } else {
                    // No delegate parameter detected: fall back to the existing signature.
                    (generics, where_clause, params, args, prelude)
                };

            // Raw vtable call that maps the HRESULT into Result<i64> via `?`.
            let raw_vcall = quote! {
                (windows_core::Interface::vtable(#receiver).#vname)(
                    windows_core::Interface::as_raw(#receiver),
                    #event_args
                )
            };

            let core = config.write_core();
            let result = config.write_result();

            // Body shared across all kinds: materialise the token then wrap it.
            let event_body = quote! {
                let mut result__ = core::mem::zeroed();
                let token__ = #raw_vcall.map(|| result__)?;
                Ok(#core EventRevoker::new(
                    #receiver.clone(),
                    token__,
                    windows_core::Interface::vtable(#receiver).#remove_vname,
                ))
            };

            return match kind {
                InterfaceKind::Default => quote! {
                    pub fn #name<#(#event_generics,)*>(&self, #(#event_params_decl)*) -> #result Result<#core EventRevoker> #event_where_clause {
                        #event_prelude
                        unsafe {
                            #event_body
                        }
                    }
                },
                InterfaceKind::None | InterfaceKind::Base => {
                    let interface_name = interface.unwrap().write_name(config);
                    quote! {
                        pub fn #name<#(#event_generics,)*>(&self, #(#event_params_decl)*) -> #result Result<#core EventRevoker> #event_where_clause {
                            let this = &windows_core::Interface::cast::<#interface_name>(self)?;
                            #event_prelude
                            unsafe {
                                #event_body
                            }
                        }
                    }
                }
                InterfaceKind::Static => {
                    let factory_name = to_ident(trim_tick(interface.unwrap().def.name()));
                    quote! {
                        pub fn #name<#(#event_generics,)*>(#(#event_params_decl)*) -> #result Result<#core EventRevoker> #event_where_clause {
                            #event_prelude
                            Self::#factory_name(|this| unsafe {
                                #event_body
                            })
                        }
                    }
                }
                // Composable is excluded by the `kind != Composable` guard above.
                InterfaceKind::Composable => unreachable!(),
            };
        }

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
