use super::*;
use proc_macro2::TokenStream;
use quote::quote;

impl native_signature::Signature {
    pub(super) fn write_com_method(
        &self,
        namespace: &str,
        layout: Layout,
        name: &str,
    ) -> Result<TokenStream, Error> {
        if !self.return_type.is_hresult() {
            return Err(Error::UnsupportedType {
                name: name.to_string(),
                shape: "native COM method does not return HRESULT".to_string(),
            });
        }

        let method = tokens::ident(name);
        if let Some(query) = self.query_parameters() {
            if query != (0, 1) || self.parameters.len() != 2 {
                return Err(Error::UnsupportedType {
                    name: name.to_string(),
                    shape: "native COM query method with additional parameters".to_string(),
                });
            }
            return Ok(quote! {
                pub(crate) unsafe fn #method<T>(&self) -> windows_core::Result<T>
                where
                    T: windows_core::Interface,
                {
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        (windows_core::Interface::vtable(self).#method)(
                            windows_core::Interface::as_raw(self),
                            &T::IID,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            });
        }

        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();
        let retval_position = self.retval_position();

        for (position, parameter) in self.parameters.iter().enumerate() {
            let name = tokens::ident(&parameter.name);
            if retval_position == Some(position) {
                arguments.push(quote! { &mut result__ });
                continue;
            }
            if parameter.ty.is_interface() {
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

        let generics =
            (!generic_parameters.is_empty()).then(|| quote! { <#(#generic_parameters),*> });
        let where_clause = (!constraints.is_empty()).then(|| quote! { where #(#constraints)* });
        let call = quote! {
            (windows_core::Interface::vtable(self).#method)(
                windows_core::Interface::as_raw(self),
                #(#arguments),*
            )
        };
        let (result, body) = if let Some(position) = retval_position {
            let ty = self.parameters[position].ty.pointee().unwrap();
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
        } else {
            (
                quote! { -> windows_core::HRESULT },
                quote! { unsafe { #call } },
            )
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
        if !self.return_type.is_hresult() {
            return Err(Error::UnsupportedType {
                name: name.to_string(),
                shape: "native COM implementation method does not return HRESULT".to_string(),
            });
        }
        let method = tokens::ident(name);
        if self.query_parameters().is_some() {
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
        let retval = self.retval_position();
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| Some(*position) != retval)
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                let ty = parameter.ty.write_public(namespace, layout);
                quote! { #name: #ty, }
            });
        let result = if let Some(position) = retval {
            let ty = self.parameters[position].ty.pointee().unwrap();
            let ty = ty.write_public(namespace, layout);
            quote! { windows_core::Result<#ty> }
        } else {
            quote! { windows_core::Result<()> }
        };
        Ok(quote! {
            fn #method(&self, #(#parameters)*) -> #result;
        })
    }

    pub(super) fn write_impl_upcall(&self, impl_name: &TokenStream, name: &str) -> TokenStream {
        let method = tokens::ident(name);
        let query = self.query_parameters().is_some();
        let retval = if query { None } else { self.retval_position() };
        let arguments = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| Some(*position) != retval)
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                quote! { core::mem::transmute_copy(&#name) }
            });
        if query || retval.is_none() {
            return quote! {
                #impl_name::#method(this, #(#arguments),*).into()
            };
        }

        let position = retval.unwrap();
        let result = tokens::ident(&self.parameters[position].name);
        let pointee = self.parameters[position].ty.pointee().unwrap();
        let write = if pointee.is_interface() {
            quote! { #result.write(core::mem::transmute(ok__)); }
        } else {
            quote! { #result.write(ok__); }
        };
        quote! {
            match #impl_name::#method(this, #(#arguments),*) {
                Ok(ok__) => {
                    #write
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
    }

    fn retval_position(&self) -> Option<usize> {
        self.parameters
            .last()
            .filter(|parameter| {
                parameter.is_output_only()
                    && !parameter.is_optional()
                    && parameter.ty.pointee().is_some()
                    && self.parameters[..self.parameters.len() - 1]
                        .iter()
                        .all(native_signature::Parameter::is_input_only)
            })
            .map(|_| self.parameters.len() - 1)
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
