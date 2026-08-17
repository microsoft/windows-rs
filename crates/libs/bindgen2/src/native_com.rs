use super::*;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub(super) enum ReturnKind<'a> {
    HResult,
    Void,
    Direct {
        ty: &'a native::Type,
        interface: bool,
    },
    Indirect(&'a native::Type),
    Retval {
        position: usize,
        ty: &'a native::Type,
        conversion: native_signature::ResultConversion,
    },
    VoidInterface {
        position: usize,
        ty: &'a native::Type,
        conversion: native_signature::ResultConversion,
    },
    VoidValue {
        position: usize,
        ty: &'a native::Type,
        conversion: native_signature::ResultConversion,
    },
    Query {
        guid: usize,
        object: usize,
        optional: bool,
    },
}

#[derive(Clone, Copy)]
pub(super) struct FunctionContext<'a> {
    pub(super) namespace: &'a str,
    pub(super) layout: Layout,
    pub(super) name: &'a str,
    pub(super) module: &'a str,
    pub(super) abi: &'a str,
    pub(super) import_name: Option<&'a str>,
}

pub(super) fn write_slice_parameter(
    parameter: &native_signature::Parameter,
    namespace: &str,
    layout: Layout,
) -> Option<(TokenStream, TokenStream)> {
    let (element, transmute) = parameter.slice_plan()?;
    let is_interface = element.is_interface();
    let element = element.write_public(namespace, layout);
    let element = if is_interface {
        quote! { Option<#element> }
    } else {
        element
    };
    let name = tokens::ident(&parameter.name);
    let mutable = parameter.is_mutable_pointer();
    let (parameter, pointer) = if parameter.is_optional() && mutable {
        (
            quote! { #name: Option<&mut [#element]> },
            quote! {
                #name.as_deref().map_or(
                    core::ptr::null_mut(),
                    |slice| slice.as_ptr().cast_mut()
                )
            },
        )
    } else if parameter.is_optional() {
        (
            quote! { #name: Option<&[#element]> },
            quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) },
        )
    } else if mutable {
        (
            quote! { #name: &mut [#element] },
            quote! { #name.as_mut_ptr() },
        )
    } else {
        (quote! { #name: &[#element] }, quote! { #name.as_ptr() })
    };
    let pointer = if is_interface || transmute {
        quote! { core::mem::transmute(#pointer) }
    } else {
        pointer
    };
    Some((parameter, pointer))
}

pub(super) fn write_slice_count(
    signature: &native_signature::Signature,
    parameter: &native_signature::Parameter,
    namespace: &str,
    layout: Layout,
) -> Option<TokenStream> {
    let (slice, newtype) = parameter.slice_parameter()?;
    let slice = &signature.parameters[slice];
    let name = tokens::ident(&slice.name);
    let wrap = |value| {
        if newtype {
            let ty = parameter.ty.write_public(namespace, layout);
            quote! { #ty(#value) }
        } else {
            value
        }
    };
    Some(if slice.is_optional() {
        let zero = wrap(quote! { 0 });
        let len = wrap(quote! { slice.len().try_into().unwrap() });
        if slice.is_mutable_pointer() {
            quote! {
                #name.as_deref().map_or(#zero, |slice| #len)
            }
        } else {
            quote! {
                #name.map_or(#zero, |slice| #len)
            }
        }
    } else {
        let len = wrap(quote! { #name.len().try_into().unwrap() });
        quote! { #len }
    })
}

impl native_signature::Signature {
    pub(super) fn supports_implementation(&self) -> bool {
        matches!(
            self.return_kind(false),
            ReturnKind::HResult
                | ReturnKind::Void
                | ReturnKind::VoidInterface { .. }
                | ReturnKind::VoidValue { .. }
                | ReturnKind::Direct { .. }
                | ReturnKind::Indirect(_)
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
        let context = FunctionContext {
            namespace,
            layout,
            name,
            module,
            abi,
            import_name,
        };
        let return_kind = self.return_kind(layout.is_package());
        if matches!(return_kind, ReturnKind::HResult | ReturnKind::Query { .. }) {
            return Some(write_rich_com_function(self, context, return_kind));
        }
        if !matches!(return_kind, ReturnKind::Retval { .. }) {
            return self.write_native_function(context);
        }

        fn write_rich_com_function(
            signature: &native_signature::Signature,
            context: FunctionContext<'_>,
            return_kind: ReturnKind<'_>,
        ) -> TokenStream {
            let FunctionContext {
                namespace,
                layout,
                name,
                module,
                abi,
                import_name,
            } = context;
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
                    arguments.push(quote! { result__ as *mut _ as *mut _ });
                    continue;
                }
                let name = tokens::ident(&parameter.name);
                let consumer = parameter.consumer_plan(layout.is_package());
                if let Some((projected, argument)) =
                    write_slice_parameter(parameter, namespace, layout)
                {
                    parameters.push(quote! { #projected, });
                    arguments.push(argument);
                    continue;
                }
                if let Some(argument) =
                    write_slice_count(signature, parameter, context.namespace, layout)
                {
                    arguments.push(argument);
                    continue;
                }
                if matches!(consumer, native_signature::ConsumerPlan::Bool) {
                    parameters.push(quote! { #name: bool, });
                    arguments.push(quote! { #name.into() });
                    continue;
                }
                if let Some((len, element, indirect)) = parameter.fixed_array_plan() {
                    let element = element.write_public(namespace, layout);
                    let len = proc_macro2::Literal::usize_unsuffixed(len);
                    let mutable =
                        matches!(&parameter.ty, native::Type::Pointer { mutable: true, .. });
                    if parameter.is_optional() && mutable {
                        parameters.push(quote! { #name: Option<&mut [#element; #len]>, });
                        let argument = quote! {
                            #name.as_deref().map_or(
                                core::ptr::null_mut(),
                                |slice| slice.as_ptr().cast_mut()
                            )
                        };
                        arguments.push(if indirect {
                            quote! { core::mem::transmute(#argument) }
                        } else {
                            argument
                        });
                    } else if parameter.is_optional() {
                        parameters.push(quote! { #name: Option<&[#element; #len]>, });
                        let argument =
                            quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) };
                        arguments.push(if indirect {
                            quote! { core::mem::transmute(#argument) }
                        } else {
                            argument
                        });
                    } else if mutable {
                        parameters.push(quote! { #name: &mut [#element; #len], });
                        let argument = quote! { #name.as_mut_ptr() };
                        arguments.push(if indirect {
                            quote! { core::mem::transmute(#argument) }
                        } else {
                            argument
                        });
                    } else {
                        parameters.push(quote! { #name: &[#element; #len], });
                        let argument = quote! { #name.as_ptr() };
                        arguments.push(if indirect {
                            quote! { core::mem::transmute(#argument) }
                        } else {
                            argument
                        });
                    }
                    continue;
                }
                if matches!(consumer, native_signature::ConsumerPlan::StringRef) {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: &#ty, });
                    arguments.push(quote! { core::mem::transmute_copy(#name) });
                    continue;
                }
                if let native_signature::ConsumerPlan::StringPointer { optional } = consumer {
                    let ty = parameter.ty.write_public(namespace, layout);
                    if parameter.is_optional() || optional {
                        parameters.push(quote! { #name: Option<#ty>, });
                        arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                    } else {
                        parameters.push(quote! { #name: #ty, });
                        arguments.push(quote! { core::mem::transmute(#name) });
                    }
                    continue;
                }
                if let native_signature::ConsumerPlan::InterfacePointer { deep, optional } =
                    consumer
                {
                    let pointer = parameter
                        .ty
                        .write_interface_pointer(namespace, layout, None)
                        .unwrap();
                    if parameter.is_optional() || optional {
                        parameters.push(quote! { #name: Option<#pointer>, });
                        arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                    } else {
                        parameters.push(quote! { #name: #pointer, });
                        arguments.push(if deep {
                            quote! { #name as _ }
                        } else {
                            quote! { core::mem::transmute(#name) }
                        });
                    }
                    continue;
                }
                if matches!(consumer, native_signature::ConsumerPlan::InterfaceOutput) {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: &Option<#ty>, });
                    arguments.push(quote! { core::mem::transmute_copy(#name) });
                } else if matches!(consumer, native_signature::ConsumerPlan::IntoParam) {
                    let generic = tokens::ident(&format!("P{position}"));
                    let ty = parameter.ty.write_public(namespace, layout);
                    generic_parameters.push(generic.clone());
                    constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                    parameters.push(quote! { #name: #generic, });
                    arguments.push(quote! { #name.param().abi() });
                } else if matches!(consumer, native_signature::ConsumerPlan::Optional) {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: Option<#ty>, });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else if matches!(consumer, native_signature::ConsumerPlan::ByRef) {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: &#ty, });
                    arguments.push(quote! { core::mem::transmute_copy(#name) });
                } else {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: #ty, });
                    if parameter.casts_abi_argument() {
                        arguments.push(quote! { #name as _ });
                    } else {
                        arguments.push(quote! { #name });
                    }
                }
            }

            if matches!(return_kind, ReturnKind::Query { optional: true, .. }) {
                parameters.push(quote! { result__: *mut Option<T>, });
            }

            let function_generics = if matches!(return_kind, ReturnKind::Query { .. }) {
                quote! { <#(#generic_parameters,)* T> }
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
                        #(#constraints)*
                        #query_constraint
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
            conversion,
        } = return_kind
        else {
            unreachable!()
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
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();
        for (position, parameter) in self.parameters.iter().enumerate() {
            if position == retval {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            let name = tokens::ident(&parameter.name);
            let consumer = parameter.consumer_plan(layout.is_package());
            if let Some((projected, argument)) = write_slice_parameter(parameter, namespace, layout)
            {
                parameters.push(projected);
                arguments.push(argument);
            } else if let Some(argument) = write_slice_count(self, parameter, namespace, layout) {
                arguments.push(argument);
            } else if let Some((len, element, indirect)) = parameter.fixed_array_plan() {
                let element = element.write_public(namespace, layout);
                let len = proc_macro2::Literal::usize_unsuffixed(len);
                if parameter.is_optional() && parameter.is_mutable_pointer() {
                    parameters.push(quote! { #name: Option<&mut [#element; #len]> });
                    let argument = quote! {
                        #name.as_deref().map_or(
                            core::ptr::null_mut(),
                            |slice| slice.as_ptr().cast_mut()
                        )
                    };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                } else if parameter.is_optional() {
                    parameters.push(quote! { #name: Option<&[#element; #len]> });
                    let argument =
                        quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                } else if parameter.is_mutable_pointer() {
                    parameters.push(quote! { #name: &mut [#element; #len] });
                    let argument = quote! { #name.as_mut_ptr() };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                } else {
                    parameters.push(quote! { #name: &[#element; #len] });
                    let argument = quote! { #name.as_ptr() };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                }
            } else if let native_signature::ConsumerPlan::InterfacePointer { deep, optional } =
                consumer
            {
                let pointer = parameter
                    .ty
                    .write_interface_pointer(namespace, layout, None)
                    .unwrap();
                if parameter.is_optional() || optional {
                    parameters.push(quote! { #name: Option<#pointer> });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    parameters.push(quote! { #name: #pointer });
                    arguments.push(if deep {
                        quote! { #name as _ }
                    } else {
                        quote! { core::mem::transmute(#name) }
                    });
                }
            } else if matches!(consumer, native_signature::ConsumerPlan::IntoParam) {
                let generic = tokens::ident(&format!("P{position}"));
                let ty = parameter.ty.write_public(namespace, layout);
                generic_parameters.push(generic.clone());
                constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                parameters.push(quote! { #name: #generic });
                arguments.push(quote! { #name.param().abi() });
            } else if matches!(consumer, native_signature::ConsumerPlan::Bool) {
                parameters.push(quote! { #name: bool });
                arguments.push(quote! { #name.into() });
            } else if matches!(consumer, native_signature::ConsumerPlan::StringRef) {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: &#ty });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
            } else if let native_signature::ConsumerPlan::StringPointer { optional } = consumer {
                let ty = parameter.ty.write_public(namespace, layout);
                if parameter.is_optional() || optional {
                    parameters.push(quote! { #name: Option<#ty> });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    parameters.push(quote! { #name: #ty });
                    arguments.push(quote! { core::mem::transmute(#name) });
                }
            } else if matches!(consumer, native_signature::ConsumerPlan::Optional) {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: Option<#ty> });
                arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
            } else if matches!(consumer, native_signature::ConsumerPlan::ByRef) {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: &#ty });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
            } else {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: #ty });
                if parameter.casts_abi_argument() {
                    arguments.push(quote! { #name as _ });
                } else {
                    arguments.push(quote! { #name });
                }
            }
        }
        let function_generics = if generic_parameters.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_parameters),*> }
        };
        let where_clause = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { where #(#constraints)* }
        };
        let result = ty.write_public(namespace, layout);
        let body = match conversion {
            native_signature::ResultConversion::FromAbi => quote! {
                #method(#(#arguments),*)
                    .and_then(|| windows_core::Type::from_abi(result__))
            },
            native_signature::ResultConversion::Transmute => quote! {
                #method(#(#arguments),*)
                    .map(|| core::mem::transmute(result__))
            },
            native_signature::ResultConversion::Identity => {
                quote! { #method(#(#arguments),*).map(|| result__) }
            }
        };
        Some(quote! {
            #[inline]
            pub unsafe fn #method #function_generics(#(#parameters),*) -> windows_core::Result<#result>
            #where_clause
            {
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

    pub(super) fn write_com_method(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        name: &str,
        owner: &str,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let visibility = if projection.is_minimal() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        let return_kind = self.return_kind(layout.is_package());
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();

        let retval = match return_kind {
            ReturnKind::Retval { position, ty, .. }
            | ReturnKind::VoidInterface { position, ty, .. }
            | ReturnKind::VoidValue { position, ty, .. } => Some((position, ty)),
            ReturnKind::HResult
            | ReturnKind::Void
            | ReturnKind::Direct { .. }
            | ReturnKind::Indirect(_) => None,
            ReturnKind::Query { .. } => None,
        };
        let cast_value_outputs = self.requires_abi_argument_cast();

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
                arguments.push(quote! { result__ as *mut _ as *mut _ });
                continue;
            }
            let name = tokens::ident(&parameter.name);
            let consumer = parameter.consumer_plan(layout.is_package());
            if retval.is_some_and(|(retval, _)| retval == position) {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            if let Some((projected, argument)) = write_slice_parameter(parameter, namespace, layout)
            {
                parameters.push(quote! { #projected, });
                arguments.push(argument);
                continue;
            }
            if let Some(argument) = write_slice_count(self, parameter, namespace, layout) {
                arguments.push(argument);
                continue;
            }
            if matches!(consumer, native_signature::ConsumerPlan::Bool) {
                parameters.push(quote! { #name: bool, });
                arguments.push(quote! { #name.into() });
                continue;
            }
            if let Some((len, element, indirect)) = parameter.fixed_array_plan() {
                let element = element.write_public_with_owner(namespace, layout, Some(owner));
                let len = proc_macro2::Literal::usize_unsuffixed(len);
                let mutable = matches!(&parameter.ty, native::Type::Pointer { mutable: true, .. });
                if parameter.is_optional() && mutable {
                    parameters.push(quote! { #name: Option<&mut [#element; #len]>, });
                    let argument = quote! {
                        #name.as_deref().map_or(
                            core::ptr::null_mut(),
                            |slice| slice.as_ptr().cast_mut()
                        )
                    };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                } else if parameter.is_optional() {
                    parameters.push(quote! { #name: Option<&[#element; #len]>, });
                    let argument =
                        quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                } else if mutable {
                    parameters.push(quote! { #name: &mut [#element; #len], });
                    let argument = quote! { #name.as_mut_ptr() };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                } else {
                    parameters.push(quote! { #name: &[#element; #len], });
                    let argument = quote! { #name.as_ptr() };
                    arguments.push(if indirect {
                        quote! { core::mem::transmute(#argument) }
                    } else {
                        argument
                    });
                }
                continue;
            }
            if matches!(consumer, native_signature::ConsumerPlan::StringRef) {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                parameters.push(quote! { #name: &#ty, });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
                continue;
            }
            if let native_signature::ConsumerPlan::StringPointer { optional } = consumer {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                if parameter.is_optional() || optional {
                    parameters.push(quote! { #name: Option<#ty>, });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    parameters.push(quote! { #name: #ty, });
                    arguments.push(quote! { core::mem::transmute(#name) });
                }
                continue;
            }
            if let native_signature::ConsumerPlan::InterfacePointer { deep, optional } = consumer {
                let pointer = parameter
                    .ty
                    .write_interface_pointer(namespace, layout, Some(owner))
                    .unwrap();
                if parameter.is_optional() || optional {
                    parameters.push(quote! { #name: Option<#pointer>, });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    parameters.push(quote! { #name: #pointer, });
                    arguments.push(if deep {
                        quote! { #name as _ }
                    } else {
                        quote! { core::mem::transmute(#name) }
                    });
                }
                continue;
            }
            if matches!(consumer, native_signature::ConsumerPlan::InterfaceOutput) {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                parameters.push(quote! { #name: &Option<#ty>, });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
            } else if matches!(consumer, native_signature::ConsumerPlan::IntoParam) {
                let generic = tokens::ident(&format!("P{position}"));
                let ty = parameter.ty.write_param(namespace, layout, owner);
                generic_parameters.push(generic.clone());
                constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                parameters.push(quote! { #name: #generic, });
                arguments.push(quote! { #name.param().abi() });
            } else if matches!(consumer, native_signature::ConsumerPlan::Optional) {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                parameters.push(quote! { #name: Option<#ty>, });
                arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
            } else if matches!(consumer, native_signature::ConsumerPlan::ByRef) {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                parameters.push(quote! { #name: &#ty, });
                arguments.push(quote! { core::mem::transmute_copy(#name) });
            } else if parameter.ty.pointee().is_some() {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                parameters.push(quote! { #name: #ty, });
                if parameter.casts_com_abi_argument(
                    layout.is_package()
                        || matches!(return_kind, ReturnKind::Query { .. })
                        || cast_value_outputs,
                ) {
                    arguments.push(quote! { #name as _ });
                } else {
                    arguments.push(quote! { #name });
                }
            } else {
                let ty = parameter
                    .ty
                    .write_public_with_owner(namespace, layout, Some(owner));
                parameters.push(quote! { #name: #ty, });
                if parameter.casts_method_argument() {
                    arguments.push(quote! { #name as _ });
                } else {
                    arguments.push(quote! { #name });
                }
            }
        }

        if matches!(return_kind, ReturnKind::Query { optional: true, .. }) {
            parameters.push(quote! { result__: *mut Option<T>, });
        }

        if matches!(
            return_kind,
            ReturnKind::Query {
                optional: false,
                ..
            }
        ) {
            let generics = quote! { <#(#generic_parameters,)* T> };
            return Ok(quote! {
                #visibility unsafe fn #method #generics(
                    &self,
                    #(#parameters)*
                ) -> windows_core::Result<T>
                where
                    #(#constraints)*
                    T: windows_core::Interface,
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
            let generics = quote! { <#(#generic_parameters,)* T> };
            return Ok(quote! {
                #visibility unsafe fn #method #generics(
                    &self,
                    #(#parameters)*
                ) -> windows_core::Result<()>
                where
                    #(#constraints)*
                    T: windows_core::Interface,
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
            ReturnKind::Retval {
                position: _,
                ty,
                conversion,
            } => {
                let public = ty.write_public_with_owner(namespace, layout, Some(owner));
                let body = match conversion {
                    native_signature::ResultConversion::FromAbi => quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call.and_then(|| windows_core::Type::from_abi(result__))
                        }
                    },
                    native_signature::ResultConversion::Transmute => quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call.map(|| core::mem::transmute(result__))
                        }
                    },
                    native_signature::ResultConversion::Identity => quote! {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #call.map(|| result__)
                        }
                    },
                };
                (quote! { -> windows_core::Result<#public> }, body)
            }
            ReturnKind::VoidInterface { ty, .. } => {
                let public = ty.write_public_with_owner(namespace, layout, Some(owner));
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
                let public = ty.write_public_with_owner(namespace, layout, Some(owner));
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
            ReturnKind::Direct { ty, interface } => {
                let public = ty.write_public_with_owner(namespace, layout, Some(owner));
                let public = if interface {
                    quote! { Option<#public> }
                } else {
                    public
                };
                (quote! { -> #public }, quote! { unsafe { #call } })
            }
            ReturnKind::Indirect(ty) => {
                let ty = ty.write_public_with_owner(namespace, layout, Some(owner));
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

    pub(super) fn return_kind(&self, package: bool) -> ReturnKind<'_> {
        match self.return_plan(package) {
            native_signature::ReturnPlan::HResult => ReturnKind::HResult,
            native_signature::ReturnPlan::Void => ReturnKind::Void,
            native_signature::ReturnPlan::VoidInterface {
                position,
                conversion,
            } => ReturnKind::VoidInterface {
                position,
                ty: self.parameters[position].ty.pointee().unwrap(),
                conversion,
            },
            native_signature::ReturnPlan::VoidValue {
                position,
                conversion,
            } => ReturnKind::VoidValue {
                position,
                ty: self.parameters[position].ty.pointee().unwrap(),
                conversion,
            },
            native_signature::ReturnPlan::Direct { interface } => ReturnKind::Direct {
                ty: &self.return_type,
                interface,
            },
            native_signature::ReturnPlan::Indirect => ReturnKind::Indirect(&self.return_type),
            native_signature::ReturnPlan::Retval {
                position,
                conversion,
            } => ReturnKind::Retval {
                position,
                ty: self.parameters[position].ty.pointee().unwrap(),
                conversion,
            },
            native_signature::ReturnPlan::Query {
                guid,
                object,
                optional,
            } => ReturnKind::Query {
                guid,
                object,
                optional,
            },
        }
    }
}
