use super::*;
use native_com::{FunctionContext, ReturnKind, write_slice_count, write_slice_parameter};
use proc_macro2::TokenStream;
use quote::quote;

impl native_signature::Signature {
    pub(super) fn write_native_function(
        &self,
        context: FunctionContext<'_>,
    ) -> Option<TokenStream> {
        let FunctionContext {
            namespace,
            layout,
            name,
            module,
            abi,
            import_name,
        } = context;
        let return_kind = self.return_kind(layout.is_package());
        let (output, direct) = match return_kind {
            ReturnKind::VoidValue {
                position,
                ty,
                conversion,
            }
            | ReturnKind::VoidInterface {
                position,
                ty,
                conversion,
            } => (
                Some((
                    position,
                    ty,
                    !matches!(conversion, native_signature::ResultConversion::Identity),
                )),
                None,
            ),
            ReturnKind::Direct { ty, interface } => (None, Some((ty, interface))),
            ReturnKind::Indirect(ty) => (None, Some((ty, false))),
            ReturnKind::Void => (None, None),
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
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();
        for (position, parameter) in self.parameters.iter().enumerate() {
            if output.is_some_and(|(output, _, _)| output == position) {
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
            } else if matches!(
                consumer,
                native_signature::ConsumerPlan::StringPointer { .. }
            ) {
                let ty = parameter.ty.write_public(namespace, layout);
                parameters.push(quote! { #name: #ty });
                arguments.push(quote! { core::mem::transmute(#name) });
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
        let result = output.map_or_else(
            || {
                direct
                    .map(|(ty, interface)| {
                        let public = ty.write_public(namespace, layout);
                        if interface {
                            quote! { Option<#public> }
                        } else {
                            public
                        }
                    })
                    .unwrap_or_default()
            },
            |(_, ty, _)| ty.write_public(namespace, layout),
        );
        let return_type = if self.no_return {
            Some(quote! { -> ! })
        } else {
            (!result.is_empty()).then(|| quote! { -> #result })
        };
        let raw_return_type = if self.no_return {
            Some(quote! { -> ! })
        } else {
            direct.map(|(ty, interface)| {
                let public = ty.write_public(namespace, layout);
                if interface {
                    quote! { -> Option<#public> }
                } else {
                    quote! { -> #public }
                }
            })
        };
        let body = if let Some((_, _, transmute)) = output {
            let result = if transmute {
                quote! { core::mem::transmute(result__) }
            } else {
                quote! { result__ }
            };
            quote! {
                let mut result__ = core::mem::zeroed();
                #method(#(#arguments),*);
                #result
            }
        } else {
            quote! { #method(#(#arguments),*) }
        };
        Some(quote! {
            #[inline]
            pub unsafe fn #method #function_generics(#(#parameters),*) #return_type #where_clause {
                windows_core::link!(
                    #module #abi #symbol fn #method(#(#raw_parameters),*) #raw_return_type
                );
                unsafe { #body }
            }
        })
    }
}
