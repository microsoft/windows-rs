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
    Query {
        guid: usize,
        object: usize,
    },
}

impl native_signature::Signature {
    pub(super) fn write_com_method(
        &self,
        namespace: &str,
        layout: Layout,
        name: &str,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let return_kind = self.return_kind();
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();
        let (slices, slice_counts) = self.slice_parameters();
        let retval = match return_kind {
            ReturnKind::Retval { position, ty } | ReturnKind::VoidInterface { position, ty } => {
                Some((position, ty))
            }
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
            if matches!(return_kind, ReturnKind::Query { object, .. } if position == object) {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            let name = tokens::ident(&parameter.name);
            if retval.is_some_and(|(retval, _)| retval == position) {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            if let Some(element) = slices[position] {
                let element = element.write_public(namespace, layout);
                if parameter.is_optional() {
                    parameters.push(quote! { #name: Option<&[#element]>, });
                    arguments
                        .push(quote! { #name.map_or(core::ptr::null(), |slice| slice.as_ptr()) });
                } else {
                    parameters.push(quote! { #name: &[#element], });
                    arguments.push(quote! { #name.as_ptr() });
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
            if parameter.ty.is_interface()
                || (parameter.is_input_only() && parameter.ty.is_pcwstr())
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
                arguments.push(quote! { #name });
            }
        }

        if matches!(return_kind, ReturnKind::Query { .. }) {
            let generics =
                (!generic_parameters.is_empty()).then(|| quote! { , #(#generic_parameters),* });
            return Ok(quote! {
                pub(crate) unsafe fn #method<T #generics>(
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
            pub(crate) unsafe fn #method #generics(
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
        let return_kind = self.return_kind();
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
            ReturnKind::HResult => None,
            ReturnKind::Void
            | ReturnKind::Direct(_)
            | ReturnKind::Indirect(_)
            | ReturnKind::VoidInterface { .. }
            | ReturnKind::Query { .. } => {
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
                if parameter.is_input_only() && parameter.ty.is_interface() {
                    quote! { #name: windows_core::Ref<#ty>, }
                } else {
                    quote! { #name: #ty, }
                }
            });
        let result = if let Some((_, ty)) = retval {
            let ty = ty.write_public(namespace, layout);
            quote! { windows_core::Result<#ty> }
        } else {
            quote! { windows_core::Result<()> }
        };
        Ok(quote! {
            fn #method(&self, #(#parameters)*) -> #result;
        })
    }

    pub(super) fn write_impl_upcall(
        &self,
        impl_name: &TokenStream,
        name: &str,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let return_kind = self.return_kind();
        let retval_position = match return_kind {
            ReturnKind::Retval { position, .. } => Some(position),
            ReturnKind::HResult | ReturnKind::Query { .. } => None,
            ReturnKind::Void
            | ReturnKind::Direct(_)
            | ReturnKind::Indirect(_)
            | ReturnKind::VoidInterface { .. } => {
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
                quote! { core::mem::transmute_copy(&#name) }
            });
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

    fn return_kind(&self) -> ReturnKind<'_> {
        if !self.return_type.is_hresult() {
            if self.return_type == native::Type::Void {
                if let Some((position, ty)) = self.retval_parameter()
                    && ty.is_interface()
                {
                    return ReturnKind::VoidInterface { position, ty };
                }
                return ReturnKind::Void;
            }
            if self.indirect_return {
                return ReturnKind::Indirect(&self.return_type);
            }
            return ReturnKind::Direct(&self.return_type);
        }
        if let Some((guid, object)) = self.query_parameters() {
            return ReturnKind::Query { guid, object };
        }
        if let Some((position, ty)) = self.retval_parameter() {
            ReturnKind::Retval { position, ty }
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

    fn slice_parameters(&self) -> (Vec<Option<&native::Type>>, Vec<Option<usize>>) {
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
                || self.parameters[count].ty != native::Type::U32
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
            if element.is_interface() || element.as_ref() == &native::Type::Void {
                continue;
            }
            slices[position] = Some(element.as_ref());
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
