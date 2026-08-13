use super::*;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
enum ReturnKind<'a> {
    HResult,
    Void,
    Direct(&'a native::Type),
    Indirect(&'a native::Type),
    Retval {
        position: usize,
        ty: &'a native::Type,
    },
    VoidInterface {
        position: usize,
        ty: &'a native::Type,
    },
    VoidValue {
        position: usize,
        ty: &'a native::Type,
    },
    Query {
        guid: usize,
        object: usize,
        optional: bool,
    },
}

impl native_signature::Signature {
    pub(super) fn supports_implementation(&self) -> bool {
        matches!(
            self.return_kind(false),
            ReturnKind::HResult
                | ReturnKind::Void
                | ReturnKind::VoidInterface { .. }
                | ReturnKind::VoidValue { .. }
                | ReturnKind::Direct(_)
                | ReturnKind::Retval { .. }
                | ReturnKind::Query { .. }
        )
    }

    pub(super) fn write_com_function(
        &self,
        namespace: &str,
        layout: Layout,
        name: &str,
        module: &str,
        abi: &str,
        import_name: Option<&str>,
    ) -> Option<TokenStream> {
        let return_kind = self.return_kind(layout == Layout::Package);
        if matches!(return_kind, ReturnKind::HResult | ReturnKind::Query { .. }) {
            return Some(write_rich_com_function(
                self,
                namespace,
                layout,
                name,
                module,
                abi,
                import_name,
                return_kind,
            ));
        }
        if !matches!(return_kind, ReturnKind::Retval { .. }) {
            return self.write_native_function(namespace, layout, name, module, abi, import_name);
        }

        fn write_rich_com_function(
            signature: &native_signature::Signature,
            namespace: &str,
            layout: Layout,
            name: &str,
            module: &str,
            abi: &str,
            import_name: Option<&str>,
            return_kind: ReturnKind<'_>,
        ) -> TokenStream {
            let method = tokens::ident(name);
            let symbol = import_name.map(|name| quote! { #name });
            let raw_parameters = signature.parameters.iter().map(|parameter| {
                let name = tokens::ident(&parameter.name);
                let ty = parameter
                    .ty
                    .write_abi_projection(namespace, layout, Projection::Default);
                quote! { #name: #ty }
            });
            let mut generic_parameters = Vec::new();
            let mut constraints = Vec::new();
            let mut parameters = Vec::new();
            let mut arguments = Vec::new();
            let (slices, slice_counts) = signature.slice_parameters();

            for (position, parameter) in signature.parameters.iter().enumerate() {
                if matches!(return_kind, ReturnKind::Query { guid, .. } if position == guid) {
                    arguments.push(quote! { &T::IID });
                    continue;
                }
                if matches!(return_kind, ReturnKind::Query { object, optional: false, .. } if position == object)
                {
                    arguments.push(quote! { &mut result__ });
                    continue;
                }
                if matches!(return_kind, ReturnKind::Query { object, optional: true, .. } if position == object)
                {
                    let name = tokens::ident(&parameter.name);
                    let interface = quote! { *mut Option<T> };
                    parameters.push(quote! { #name: #interface, });
                    arguments.push(quote! { #name as *mut _ as *mut _ });
                    continue;
                }
                let name = tokens::ident(&parameter.name);
                if let Some(element) = &slices[position] {
                    let is_interface = element.is_interface();
                    let is_byte_buffer = parameter
                        .ty
                        .pointee()
                        .is_some_and(|element| element == &native::Type::Void);
                    let element = element.write_public(namespace, layout);
                    let element = if is_interface {
                        quote! { Option<#element> }
                    } else {
                        element
                    };
                    if parameter.is_optional() {
                        parameters.push(quote! { #name: Option<&[#element]>, });
                        if is_interface || is_byte_buffer {
                            arguments.push(quote! {
                                core::mem::transmute(
                                    #name.map_or(core::ptr::null(), |slice| slice.as_ptr())
                                )
                            });
                        } else {
                            arguments.push(
                                quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) },
                            );
                        }
                    } else {
                        parameters.push(quote! { #name: &[#element], });
                        if is_interface || is_byte_buffer {
                            arguments.push(quote! { core::mem::transmute(#name.as_ptr()) });
                        } else {
                            arguments.push(quote! { #name.as_ptr() });
                        }
                    }
                    continue;
                }
                if let Some(slice) = slice_counts[position] {
                    let name = tokens::ident(&signature.parameters[slice].name);
                    if signature.parameters[slice].is_optional() {
                        arguments.push(
                            quote! { #name.map_or(0, |slice| slice.len().try_into().unwrap()) },
                        );
                    } else {
                        arguments.push(quote! { #name.len().try_into().unwrap() });
                    }
                    continue;
                }
                if parameter.is_input_only() && parameter.ty.is_bool() {
                    parameters.push(quote! { #name: bool, });
                    arguments.push(quote! { #name.into() });
                    continue;
                }
                if let Some(len) = parameter.array_len
                    && let Some(element) = parameter.ty.pointee()
                {
                    let element = element.write_public(namespace, layout);
                    let len = proc_macro2::Literal::usize_unsuffixed(len);
                    let mutable =
                        matches!(&parameter.ty, native::Type::Pointer { mutable: true, .. });
                    if parameter.is_optional() && mutable {
                        parameters.push(quote! { #name: Option<&mut [#element; #len]>, });
                        arguments.push(
                            quote! { #name.map_or(core::ptr::null_mut(), |array| array.as_mut_ptr()) },
                        );
                    } else if parameter.is_optional() {
                        parameters.push(quote! { #name: Option<&[#element; #len]>, });
                        arguments.push(
                            quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) },
                        );
                    } else if mutable {
                        parameters.push(quote! { #name: &mut [#element; #len], });
                        arguments.push(quote! { #name.as_mut_ptr() });
                    } else {
                        parameters.push(quote! { #name: &[#element; #len], });
                        arguments.push(quote! { #name.as_ptr() });
                    }
                    continue;
                }
                if parameter.ty.is_bstr() {
                    parameters.push(quote! { #name: &windows_core::BSTR, });
                    arguments.push(quote! { core::mem::transmute_copy(#name) });
                    continue;
                }
                if parameter.ty.pointee().is_some_and(native::Type::is_bstr) {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: #ty, });
                    arguments.push(quote! { core::mem::transmute(#name) });
                    continue;
                }
                if let Some((_, interface)) = parameter.ty.interface_out() {
                    let interface = interface.write_public(namespace, layout);
                    let pointer = quote! { *mut Option<#interface> };
                    if parameter.is_optional() {
                        parameters.push(quote! { #name: Option<#pointer>, });
                        arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                    } else {
                        parameters.push(quote! { #name: #pointer, });
                        arguments.push(quote! { core::mem::transmute(#name) });
                    }
                    continue;
                }
                if parameter.ty.is_interface() && !parameter.is_input_only() {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: &Option<#ty>, });
                    arguments.push(quote! { core::mem::transmute_copy(#name) });
                } else if (parameter.is_input_only() && parameter.ty.is_interface())
                    || (parameter.is_input_only()
                        && (parameter.ty.is_pcwstr()
                            || (layout == Layout::Package && parameter.ty.is_const_string())))
                {
                    let generic = tokens::ident(&format!("P{position}"));
                    let ty = parameter.ty.write_public(namespace, layout);
                    generic_parameters.push(generic.clone());
                    constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                    parameters.push(quote! { #name: #generic, });
                    arguments.push(quote! { #name.param().abi() });
                } else if parameter.is_optional() && parameter.ty.pointee().is_some() {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: Option<#ty>, });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: #ty, });
                    if parameter.pointer_cast {
                        arguments.push(quote! { #name as _ });
                    } else {
                        arguments.push(quote! { #name });
                    }
                }
            }

            let function_generics = if matches!(return_kind, ReturnKind::Query { .. }) {
                quote! { <T, #(#generic_parameters),*> }
            } else if generic_parameters.is_empty() {
                quote! {}
            } else {
                quote! { <#(#generic_parameters),*> }
            };
            let query_constraint = matches!(return_kind, ReturnKind::Query { .. })
                .then(|| quote! { T: windows_core::Interface, });
            let where_clause = (!constraints.is_empty() || query_constraint.is_some()).then(|| {
                quote! {
                    where
                        #query_constraint
                        #(#constraints)*
                }
            });
            let prelude = matches!(
                return_kind,
                ReturnKind::Query {
                    optional: false,
                    ..
                }
            )
            .then(|| quote! { let mut result__ = core::ptr::null_mut(); });
            let body = if matches!(
                return_kind,
                ReturnKind::Query {
                    optional: false,
                    ..
                }
            ) {
                quote! {
                    #method(#(#arguments),*)
                        .and_then(|| windows_core::Type::from_abi(result__))
                }
            } else if matches!(return_kind, ReturnKind::Query { optional: true, .. }) {
                quote! { #method(#(#arguments),*).ok() }
            } else {
                quote! { #method(#(#arguments),*) }
            };
            let result = if matches!(
                return_kind,
                ReturnKind::Query {
                    optional: false,
                    ..
                }
            ) {
                quote! { windows_core::Result<T> }
            } else if matches!(return_kind, ReturnKind::Query { optional: true, .. }) {
                quote! { windows_core::Result<()> }
            } else {
                quote! { windows_core::HRESULT }
            };
            quote! {
                #[inline]
                pub unsafe fn #method #function_generics(#(#parameters)*) -> #result
                #where_clause
                {
                    windows_core::link!(
                        #module #abi #symbol fn #method(#(#raw_parameters),*) -> windows_core::HRESULT
                    );
                    #prelude
                    unsafe { #body }
                }
            }
        }
        let ReturnKind::Retval {
            position: retval,
            ty,
        } = return_kind
        else {
            unreachable!()
        };
        let method = tokens::ident(name);
        let symbol = import_name.map(|name| quote! { #name });
        let raw_parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = if parameter.is_input_only() {
                parameter.ty.write_public_input(namespace, layout)
            } else {
                parameter
                    .ty
                    .write_abi_projection(namespace, layout, Projection::Default)
            };
            quote! { #name: #ty }
        });
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| *position != retval)
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                let ty = parameter.ty.write_public_input(namespace, layout);
                quote! { #name: #ty }
            });
        let arguments = self
            .parameters
            .iter()
            .enumerate()
            .map(|(position, parameter)| {
                if position == retval {
                    quote! { &mut result__ }
                } else {
                    let name = tokens::ident(&parameter.name);
                    quote! { #name }
                }
            });
        let result = ty.write_public(namespace, layout);
        let body = if ty.is_interface() {
            quote! {
                #method(#(#arguments),*)
                    .and_then(|| windows_core::Type::from_abi(result__))
            }
        } else {
            quote! { #method(#(#arguments),*).map(|| result__) }
        };
        Some(quote! {
            #[inline]
            pub unsafe fn #method(#(#parameters),*) -> windows_core::Result<#result> {
                windows_core::link!(
                    #module #abi #symbol fn #method(#(#raw_parameters),*) -> windows_core::HRESULT
                );
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #body
                }
            }
        })
    }

    fn write_native_function(
        &self,
        namespace: &str,
        layout: Layout,
        name: &str,
        module: &str,
        abi: &str,
        import_name: Option<&str>,
    ) -> Option<TokenStream> {
        let return_kind = self.return_kind(layout == Layout::Package);
        let output = if matches!(return_kind, ReturnKind::Void) {
            let outputs = self
                .parameters
                .iter()
                .enumerate()
                .filter(|(_, parameter)| !parameter.is_input_only())
                .collect::<Vec<_>>();
            match outputs.as_slice() {
                [] => None,
                [(position, parameter)] => Some((*position, parameter.ty.pointee()?)),
                _ => return None,
            }
        } else {
            None
        };
        let direct = match return_kind {
            ReturnKind::Direct(ty) => Some(ty),
            ReturnKind::Void => None,
            _ => return None,
        };
        let method = tokens::ident(name);
        let symbol = import_name.map(|name| quote! { #name });
        let raw_parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = parameter
                .ty
                .write_abi_projection(namespace, layout, Projection::Default);
            quote! { #name: #ty }
        });
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| output.is_none_or(|(output, _)| output != *position))
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                if parameter.ty.is_bstr() {
                    quote! { #name: &windows_core::BSTR }
                } else {
                    let ty = parameter.ty.write_public_input(namespace, layout);
                    quote! { #name: #ty }
                }
            });
        let arguments = self
            .parameters
            .iter()
            .enumerate()
            .map(|(position, parameter)| {
                if output.is_some_and(|(output, _)| output == position) {
                    quote! { &mut result__ }
                } else {
                    let name = tokens::ident(&parameter.name);
                    if parameter.ty.is_bstr() {
                        quote! { core::mem::transmute_copy(#name) }
                    } else {
                        quote! { #name }
                    }
                }
            });
        let result = output.map_or_else(
            || {
                direct
                    .map(|ty| ty.write_public(namespace, layout))
                    .unwrap_or_default()
            },
            |(_, ty)| ty.write_public(namespace, layout),
        );
        let return_type = (!result.is_empty()).then(|| quote! { -> #result });
        let raw_return_type = direct.map(|ty| {
            let ty = ty.write_abi_projection(namespace, layout, Projection::Default);
            quote! { -> #ty }
        });
        let body = if output.is_some() {
            quote! {
                let mut result__ = core::mem::zeroed();
                #method(#(#arguments),*);
                result__
            }
        } else {
            quote! { #method(#(#arguments),*) }
        };
        Some(quote! {
            #[inline]
            pub unsafe fn #method(#(#parameters),*) #return_type {
                windows_core::link!(
                    #module #abi #symbol fn #method(#(#raw_parameters),*) #raw_return_type
                );
                unsafe { #body }
            }
        })
    }

    pub(super) fn write_com_method(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        name: &str,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let visibility = if projection.is_minimal() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        let return_kind = self.return_kind(layout == Layout::Package);
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();
        let (slices, slice_counts) = self.slice_parameters();
        let retval = match return_kind {
            ReturnKind::Retval { position, ty }
            | ReturnKind::VoidInterface { position, ty }
            | ReturnKind::VoidValue { position, ty } => Some((position, ty)),
            ReturnKind::HResult
            | ReturnKind::Void
            | ReturnKind::Direct(_)
            | ReturnKind::Indirect(_) => None,
            ReturnKind::Query { .. } => None,
        };

        for (position, parameter) in self.parameters.iter().enumerate() {
            if matches!(return_kind, ReturnKind::Query { guid, .. } if position == guid) {
                arguments.push(quote! { &T::IID });
                continue;
            }
            if matches!(return_kind, ReturnKind::Query { object, optional: false, .. } if position == object)
            {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            if matches!(return_kind, ReturnKind::Query { object, optional: true, .. } if position == object)
            {
                let interface = quote! { *mut Option<T> };
                parameters.push(quote! { result__: #interface, });
                arguments.push(quote! { result__ as *mut _ as *mut _ });
                continue;
            }
            let name = tokens::ident(&parameter.name);
            if retval.is_some_and(|(retval, _)| retval == position) {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            if let Some(element) = &slices[position] {
                let is_interface = element.is_interface();
                let is_byte_buffer = parameter
                    .ty
                    .pointee()
                    .is_some_and(|element| element == &native::Type::Void);
                let element = element.write_public(namespace, layout);
                let element = if is_interface {
                    quote! { Option<#element> }
                } else {
                    element
                };
                if parameter.is_optional() {
                    parameters.push(quote! { #name: Option<&[#element]>, });
                    if is_interface || is_byte_buffer {
                        arguments.push(quote! {
                            core::mem::transmute(
                                #name.map_or(core::ptr::null(), |slice| slice.as_ptr())
                            )
                        });
                    } else {
                        arguments.push(
                            quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) },
                        );
                    }
                } else {
                    parameters.push(quote! { #name: &[#element], });
                    if is_interface || is_byte_buffer {
                        arguments.push(quote! { core::mem::transmute(#name.as_ptr()) });
                    } else {
                        arguments.push(quote! { #name.as_ptr() });
                    }
                }
                continue;
            }
            if let Some(slice) = slice_counts[position] {
                let name = tokens::ident(&self.parameters[slice].name);
                if self.parameters[slice].is_optional() {
                    arguments
                        .push(quote! { #name.map_or(0, |slice| slice.len().try_into().unwrap()) });
                } else {
                    arguments.push(quote! { #name.len().try_into().unwrap() });
                }
                continue;
            }
            if parameter.is_input_only() && parameter.ty.is_bool() {
                parameters.push(quote! { #name: bool, });
                arguments.push(quote! { #name.into() });
                continue;
            }
            if let Some(len) = parameter.array_len
                && let Some(element) = parameter.ty.pointee()
            {
                let element = element.write_public(namespace, layout);
                let len = proc_macro2::Literal::usize_unsuffixed(len);
                let mutable = matches!(&parameter.ty, native::Type::Pointer { mutable: true, .. });
                if parameter.is_optional() && mutable {
                    parameters.push(quote! { #name: Option<&mut [#element; #len]>, });
                    arguments.push(
                        quote! { #name.map_or(core::ptr::null_mut(), |array| array.as_mut_ptr()) },
                    );
                } else if parameter.is_optional() {
                    parameters.push(quote! { #name: Option<&[#element; #len]>, });
                    arguments
                        .push(quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) });
                } else if mutable {
                    parameters.push(quote! { #name: &mut [#element; #len], });
                    arguments.push(quote! { #name.as_mut_ptr() });
                } else {
                    parameters.push(quote! { #name: &[#element; #len], });
                    arguments.push(quote! { #name.as_ptr() });
                }
                continue;
            }
            if parameter.ty.is_bstr() {
                parameters.push(quote! { #name: &windows_core::BSTR, });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
                continue;
            }
            if parameter.ty.pointee().is_some_and(native::Type::is_bstr) {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: #ty, });
                arguments.push(quote! { core::mem::transmute(#name) });
                continue;
            }
            if let Some((mutable, interface)) = parameter.ty.interface_out() {
                let interface = interface.write_public(namespace, layout);
                let pointer = if mutable {
                    quote! { *mut Option<#interface> }
                } else {
                    quote! { *const Option<#interface> }
                };
                if parameter.is_optional() {
                    parameters.push(quote! { #name: Option<#pointer>, });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    parameters.push(quote! { #name: #pointer, });
                    arguments.push(quote! { core::mem::transmute(#name) });
                }
                continue;
            }
            if parameter.ty.is_interface() && !parameter.is_input_only() {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: &Option<#ty>, });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
            } else if (parameter.is_input_only() && parameter.ty.is_interface())
                || (parameter.is_input_only()
                    && (parameter.ty.is_pcwstr()
                        || (layout == Layout::Package && parameter.ty.is_const_string())))
            {
                let generic = tokens::ident(&format!("P{position}"));
                let ty = parameter.ty.write_public(namespace, layout);
                generic_parameters.push(generic.clone());
                constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                parameters.push(quote! { #name: #generic, });
                arguments.push(quote! { #name.param().abi() });
            } else if parameter.is_optional() && parameter.ty.mutable_string_pointer() {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: Option<#ty>, });
                arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
            } else if parameter.is_optional() && parameter.ty.pointee().is_some() {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: Option<#ty>, });
                arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
            } else if parameter.ty.pointee().is_some() {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: #ty, });
                if parameter.pointer_cast {
                    arguments.push(quote! { #name as _ });
                } else {
                    arguments.push(quote! { #name });
                }
            } else {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: #ty, });
                arguments.push(quote! { #name });
            }
        }

        if matches!(
            return_kind,
            ReturnKind::Query {
                optional: false,
                ..
            }
        ) {
            let generics =
                (!generic_parameters.is_empty()).then(|| quote! { , #(#generic_parameters),* });
            return Ok(quote! {
                #visibility unsafe fn #method<T #generics>(
                    &self,
                    #(#parameters)*
                ) -> windows_core::Result<T>
                where
                    T: windows_core::Interface,
                    #(#constraints)*
                {
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        (windows_core::Interface::vtable(self).#method)(
                            windows_core::Interface::as_raw(self),
                            #(#arguments),*
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            });
        }
        if matches!(return_kind, ReturnKind::Query { optional: true, .. }) {
            let generics =
                (!generic_parameters.is_empty()).then(|| quote! { , #(#generic_parameters),* });
            return Ok(quote! {
                #visibility unsafe fn #method<T #generics>(
                    &self,
                    #(#parameters)*
                ) -> windows_core::Result<()>
                where
                    T: windows_core::Interface,
                    #(#constraints)*
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).#method)(
                            windows_core::Interface::as_raw(self),
                            #(#arguments),*
                        )
                        .ok()
                    }
                }
            });
        }

        let generics =
            (!generic_parameters.is_empty()).then(|| quote! { <#(#generic_parameters),*> });
        let where_clause = (!constraints.is_empty()).then(|| quote! { where #(#constraints)* });
        let indirect =
            matches!(return_kind, ReturnKind::Indirect(_)).then(|| quote! { &mut result__, });
        let call = quote! {
            (windows_core::Interface::vtable(self).#method)(
                windows_core::Interface::as_raw(self),
                #indirect
                #(#arguments),*
            )
        };
        let (result, body) = match return_kind {
            ReturnKind::Retval { ty, .. } => {
                let public = ty.write_public(namespace, layout);
                let body = if ty.is_interface() {
                    quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call.and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                } else {
                    quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call.map(|| result__)
                        }
                    }
                };
                (quote! { -> windows_core::Result<#public> }, body)
            }
            ReturnKind::VoidInterface { ty, .. } => {
                let public = ty.write_public(namespace, layout);
                (
                    quote! { -> windows_core::Result<#public> },
                    quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call;
                            windows_core::Type::from_abi(result__)
                        }
                    },
                )
            }
            ReturnKind::VoidValue { ty, .. } => {
                let public = ty.write_public(namespace, layout);
                (
                    quote! { -> #public },
                    quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call;
                            result__
                        }
                    },
                )
            }
            ReturnKind::HResult => (
                quote! { -> windows_core::HRESULT },
                quote! { unsafe { #call } },
            ),
            ReturnKind::Void => (quote! {}, quote! { unsafe { #call; } }),
            ReturnKind::Direct(ty) => {
                let ty = ty.write_public(namespace, layout);
                (quote! { -> #ty }, quote! { unsafe { #call } })
            }
            ReturnKind::Indirect(ty) => {
                let ty = ty.write_public(namespace, layout);
                (
                    quote! { -> #ty },
                    quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call;
                            result__
                        }
                    },
                )
            }
            ReturnKind::Query { .. } => unreachable!(),
        };

        Ok(quote! {
            #visibility unsafe fn #method #generics(
                &self,
                #(#parameters)*
            ) #result
            #where_clause
            {
                #body
            }
        })
    }

    pub(super) fn write_impl_method(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        name: &str,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let return_kind = self.return_kind(layout == Layout::Package);
        if matches!(return_kind, ReturnKind::Query { .. }) {
            let parameters = self.parameters.iter().map(|parameter| {
                let name = tokens::ident(&parameter.name);
                let ty = parameter
                    .ty
                    .write_abi_projection(namespace, layout, projection);
                quote! { #name: #ty, }
            });
            return Ok(quote! {
                fn #method(&self, #(#parameters)*) -> windows_core::Result<()>;
            });
        }
        let retval = match return_kind {
            ReturnKind::Retval { position, ty } => Some((position, ty)),
            ReturnKind::HResult
            | ReturnKind::Void
            | ReturnKind::VoidInterface { .. }
            | ReturnKind::VoidValue { .. }
            | ReturnKind::Direct(_) => None,
            ReturnKind::Indirect(_) | ReturnKind::Query { .. } => {
                return Err(Error::UnsupportedType {
                    name: name.to_string(),
                    shape: "native COM producer method does not return HRESULT".to_string(),
                });
            }
        };
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| retval.is_none_or(|(retval, _)| *position != retval))
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                let ty = parameter.ty.write_public(namespace, layout);
                if parameter.array_count.is_none()
                    && parameter.array_len.is_none()
                    && let Some((mutable, interface)) = parameter.ty.interface_out()
                {
                    let interface = interface.write_public(namespace, layout);
                    if mutable {
                        quote! { #name: windows_core::OutRef<#interface>, }
                    } else {
                        quote! { #name: *const Option<#interface>, }
                    }
                } else if parameter.is_input_only() && parameter.ty.is_interface() {
                    quote! { #name: windows_core::Ref<#ty>, }
                } else if parameter.array_count.is_some() || parameter.array_len.is_some() {
                    let ty = parameter.ty.write_public_pointer(namespace, layout);
                    quote! { #name: #ty, }
                } else if parameter.producer_by_ref {
                    quote! { #name: &#ty, }
                } else {
                    quote! { #name: #ty, }
                }
            });
        let result = match return_kind {
            ReturnKind::Retval { ty, .. } => {
                let ty = ty.write_public(namespace, layout);
                quote! { -> windows_core::Result<#ty> }
            }
            ReturnKind::HResult => quote! { -> windows_core::Result<()> },
            ReturnKind::Void | ReturnKind::VoidInterface { .. } | ReturnKind::VoidValue { .. } => {
                quote! {}
            }
            ReturnKind::Direct(ty) => {
                let ty = ty.write_public(namespace, layout);
                quote! { -> #ty }
            }
            _ => unreachable!(),
        };
        Ok(quote! {
            fn #method(&self, #(#parameters)*) #result;
        })
    }

    pub(super) fn write_impl_upcall(
        &self,
        impl_name: &TokenStream,
        name: &str,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let return_kind = self.return_kind(layout == Layout::Package);
        let retval_position = match return_kind {
            ReturnKind::Retval { position, .. } => Some(position),
            ReturnKind::HResult
            | ReturnKind::Void
            | ReturnKind::VoidInterface { .. }
            | ReturnKind::VoidValue { .. }
            | ReturnKind::Direct(_)
            | ReturnKind::Query { .. } => None,
            ReturnKind::Indirect(_) => {
                return Err(Error::UnsupportedType {
                    name: name.to_string(),
                    shape: "native COM producer method does not return HRESULT".to_string(),
                });
            }
        };
        let arguments = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| retval_position != Some(*position))
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                if parameter.producer_by_ref {
                    quote! { core::mem::transmute(&#name) }
                } else {
                    quote! { core::mem::transmute_copy(&#name) }
                }
            });
        if matches!(return_kind, ReturnKind::Direct(_)) {
            return Ok(quote! {
                #impl_name::#method(this, #(#arguments),*)
            });
        }
        if matches!(
            return_kind,
            ReturnKind::Void | ReturnKind::VoidInterface { .. } | ReturnKind::VoidValue { .. }
        ) {
            return Ok(quote! { #impl_name::#method(this, #(#arguments),*); });
        }
        let ReturnKind::Retval {
            position,
            ty: pointee,
        } = return_kind
        else {
            return Ok(quote! {
                #impl_name::#method(this, #(#arguments),*).into()
            });
        };
        let result = tokens::ident(&self.parameters[position].name);
        let write = if pointee.is_interface() {
            quote! { #result.write(core::mem::transmute(ok__)); }
        } else {
            quote! { #result.write(ok__); }
        };
        Ok(quote! {
            match #impl_name::#method(this, #(#arguments),*) {
                Ok(ok__) => {
                    #write
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        })
    }

    fn return_kind(&self, package: bool) -> ReturnKind<'_> {
        if !(self.return_type.is_hresult() || (package && self.return_type.is_hresult_package())) {
            if self.return_type == native::Type::Void {
                if let Some((position, ty)) = self.retval_parameter() {
                    return if ty.is_interface() {
                        ReturnKind::VoidInterface { position, ty }
                    } else {
                        ReturnKind::VoidValue { position, ty }
                    };
                }
                return ReturnKind::Void;
            }
            if self.indirect_return {
                return ReturnKind::Indirect(&self.return_type);
            }
            return ReturnKind::Direct(&self.return_type);
        }
        if let Some((guid, object)) = self.query_parameters() {
            return ReturnKind::Query {
                guid,
                object,
                optional: self.parameters[object].is_optional(),
            };
        }
        if let Some((position, ty)) = self.retval_parameter() {
            if ty
                .pointee()
                .is_some_and(|pointee| pointee == &native::Type::Void)
            {
                ReturnKind::HResult
            } else {
                ReturnKind::Retval { position, ty }
            }
        } else {
            ReturnKind::HResult
        }
    }

    fn retval_parameter(&self) -> Option<(usize, &native::Type)> {
        let (parameter, preceding) = self.parameters.split_last()?;
        if parameter.retval_candidate
            && preceding
                .iter()
                .all(native_signature::Parameter::is_input_only)
        {
            Some((preceding.len(), parameter.ty.pointee()?))
        } else {
            None
        }
    }

    fn slice_parameters(&self) -> (Vec<Option<native::Type>>, Vec<Option<usize>>) {
        let mut references = vec![0usize; self.parameters.len()];
        for parameter in &self.parameters {
            if let Some(count) = parameter.array_count {
                references[count] += 1;
            }
        }

        let mut slices = vec![None; self.parameters.len()];
        let mut counts = vec![None; self.parameters.len()];
        for (position, parameter) in self.parameters.iter().enumerate() {
            let Some(count) = parameter.array_count else {
                continue;
            };
            if references[count] != 1
                || !parameter.is_input_only()
                || !self.parameters[count].is_input_only()
                || self.parameters[count].is_optional()
                || !matches!(
                    self.parameters[count].ty,
                    native::Type::U32 | native::Type::USize
                )
            {
                continue;
            }
            let native::Type::Pointer {
                mutable: false,
                element,
            } = &parameter.ty
            else {
                continue;
            };
            slices[position] = Some(if element.as_ref() == &native::Type::Void {
                native::Type::U8
            } else {
                element.as_ref().clone()
            });
            counts[count] = Some(position);
        }
        (slices, counts)
    }

    fn query_parameters(&self) -> Option<(usize, usize)> {
        let guid = self.parameters.iter().rposition(|parameter| {
            parameter.is_input_only()
                && matches!(
                    &parameter.ty,
                    native::Type::Pointer {
                        mutable: false,
                        element,
                    } if matches!(
                        element.as_ref(),
                        native::Type::Named { name, .. } if name == "GUID"
                    )
                )
        })?;
        let object = self.parameters.iter().rposition(|parameter| {
            parameter.com_out_ptr
                && matches!(
                    &parameter.ty,
                    native::Type::Pointer {
                        mutable: true,
                        element,
                    } if matches!(
                        element.as_ref(),
                        native::Type::Pointer {
                            mutable: true,
                            element,
                        } if matches!(element.as_ref(), native::Type::Void)
                    )
                )
        })?;
        Some((guid, object))
    }
}
