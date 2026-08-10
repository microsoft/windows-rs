use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) struct Signature {
    pub(super) flags: u8,
    parameters: Vec<Parameter>,
    return_type: native::Type,
}

struct Parameter {
    name: String,
    flags: u16,
    ty: native::Type,
}

impl Parameter {
    fn is_input_only(&self) -> bool {
        self.flags & 0x0002 == 0
    }

    fn is_output_only(&self) -> bool {
        self.flags & 0x0002 != 0 && self.flags & 0x0001 == 0
    }

    fn is_optional(&self) -> bool {
        self.flags & 0x0010 != 0
    }
}

impl Signature {
    pub(super) fn lower(
        database: &Database,
        method: windows_metadata2::MethodDefinition<'_>,
        owner: &str,
    ) -> Result<Self, Error> {
        let MethodSignature {
            flags,
            return_type,
            parameters,
            ..
        } = method.signature()?;
        let parameter_rows = method.parameters_by_sequence()?;
        let parameters = parameters
            .into_iter()
            .enumerate()
            .map(|(position, ty)| {
                let parameter = parameter_rows.parameters()[position];
                let flags = parameter
                    .map(|parameter| parameter.flags())
                    .transpose()?
                    .unwrap_or(0);
                Ok(Parameter {
                    name: parameter
                        .map(|parameter| parameter.name())
                        .transpose()?
                        .map_or_else(|| format!("p{position}"), str::to_lowercase),
                    flags,
                    ty: native::Type::lower_parameter(
                        database,
                        method.entity().file(),
                        owner,
                        ty,
                        flags & 0x0002 == 0,
                    )?,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self {
            flags,
            parameters,
            return_type: native::Type::lower(database, method.entity().file(), owner, return_type)?,
        })
    }

    pub(super) fn named_types(&self, mut add: impl FnMut(&str, &str)) {
        for parameter in &self.parameters {
            parameter.ty.named_types(&mut add);
        }
        self.return_type.named_types(add);
    }

    pub(super) fn write_parameters_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = parameter.ty.write_projection(namespace, layout, projection);
            quote! { #name: #ty }
        });
        quote! { #(#parameters),* }
    }

    pub(super) fn write_vtable_parameters(&self, namespace: &str, layout: Layout) -> TokenStream {
        self.write_vtable_parameters_projection(namespace, layout, Projection::Sys)
    }

    pub(super) fn write_vtable_parameters_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let parameters = self.parameters.iter().map(|parameter| {
            let ty = parameter.ty.write_projection(namespace, layout, projection);
            quote! { #ty }
        });
        quote! { *mut core::ffi::c_void #(, #parameters)* }
    }

    pub(super) fn write_result(&self, namespace: &str, layout: Layout) -> TokenStream {
        self.write_result_projection(namespace, layout, Projection::Sys)
    }

    pub(super) fn write_result_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        if self.return_type == native::Type::Void {
            quote! {}
        } else {
            let ty = self
                .return_type
                .write_projection(namespace, layout, projection);
            quote! { -> #ty }
        }
    }

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
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let mut parameters = Vec::new();
        let mut arguments = Vec::new();
        let retval_position = self
            .parameters
            .last()
            .filter(|parameter| {
                parameter.is_output_only()
                    && !parameter.is_optional()
                    && parameter.ty.pointee().is_some()
                    && self.parameters[..self.parameters.len() - 1]
                        .iter()
                        .all(Parameter::is_input_only)
            })
            .map(|_| self.parameters.len() - 1);

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
            } else {
                if parameter.is_optional() && parameter.ty.pointee().is_some() {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: Option<#ty>, });
                    arguments.push(quote! { #name.unwrap_or(core::mem::zeroed()) as _ });
                } else {
                    let ty = parameter.ty.write_public(namespace, layout);
                    parameters.push(quote! { #name: #ty, });
                    arguments.push(quote! { #name });
                }
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
        let body;
        let result;
        if let Some(position) = retval_position {
            let retval = &self.parameters[position];
            let Some(ty) = retval.ty.pointee() else {
                return Err(Error::InvalidType {
                    name: method.to_string(),
                    message: "native COM retval parameter is not a pointer",
                });
            };
            let public = ty.write_public(namespace, layout);
            result = quote! { -> windows_core::Result<#public> };
            body = if ty.is_interface() {
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
        } else {
            result = quote! { -> windows_core::HRESULT };
            body = quote! { unsafe { #call } };
        }

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
}
