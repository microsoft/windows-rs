use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;

pub(super) struct Signature {
    pub(super) flags: u8,
    pub(super) parameters: Vec<Parameter>,
    pub(super) return_type: native::Type,
    pub(super) indirect_return: bool,
    package_dependencies: BTreeSet<(String, String)>,
}

pub(super) struct Parameter {
    pub(super) name: String,
    flags: u16,
    pub(super) com_out_ptr: bool,
    pub(super) array_count: Option<usize>,
    pub(super) array_len: Option<usize>,
    pub(super) retval_candidate: bool,
    pub(super) producer_by_ref: bool,
    pub(super) pointer_cast: bool,
    pub(super) ty: native::Type,
}

impl Parameter {
    pub(super) fn is_input_only(&self) -> bool {
        self.flags & 0x0002 == 0
    }

    pub(super) fn is_optional(&self) -> bool {
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
        let parameters: Vec<Parameter> = parameters
            .into_iter()
            .enumerate()
            .map(|(position, ty)| {
                let parameter = parameter_rows.parameters()[position];
                let flags = parameter
                    .map(|parameter| parameter.flags())
                    .transpose()?
                    .unwrap_or(0);
                let (array_count, mut array_len) = parameter
                    .map(|parameter| Self::array_info(parameter, owner))
                    .transpose()?
                    .unwrap_or_default();
                if flags & 0x0002 != 0 && flags & 0x0001 == 0 {
                    array_len = None;
                }
                let buffer = if let Some(parameter) = parameter {
                    parameter.has_attribute("NativeArrayInfoAttribute")?
                        || parameter.has_attribute("MemorySizeAttribute")?
                } else {
                    false
                };
                let explicit_retval = parameter
                    .map(|parameter| parameter.has_attribute("RetValAttribute"))
                    .transpose()?
                    .unwrap_or(false);
                let ty = native::Type::lower_parameter(
                    database,
                    method.entity().file(),
                    owner,
                    ty,
                    flags & 0x0002 == 0,
                )?;
                let retval_candidate = if explicit_retval
                    || (flags & 0x0002 != 0
                        && flags & 0x0001 == 0
                        && flags & 0x0010 == 0
                        && !buffer)
                {
                    if let Some(pointee) = ty.pointee() {
                        explicit_retval
                            || (pointee != &native::Type::Void
                                && !pointee.exceeds_retval_limit(database)?)
                    } else {
                        false
                    }
                } else {
                    false
                };
                let producer_by_ref = flags & 0x0002 == 0 && ty.producer_by_ref(database)?;
                let pointer_cast = ty.needs_pointer_cast()
                    || (flags & 0x0002 != 0
                        && matches!(&ty, native::Type::Pointer { mutable: true, .. }));
                let com_out_ptr = parameter
                    .map(|parameter| parameter.has_attribute("ComOutPtrAttribute"))
                    .transpose()?
                    .unwrap_or(false);
                Ok(Parameter {
                    name: parameter
                        .map(|parameter| parameter.name())
                        .transpose()?
                        .map_or_else(|| format!("p{position}"), str::to_lowercase),
                    flags,
                    com_out_ptr,
                    array_count,
                    array_len,
                    retval_candidate,
                    producer_by_ref,
                    pointer_cast,
                    ty,
                })
            })
            .collect::<Result<_, Error>>()?;
        for (position, parameter) in parameters.iter().enumerate() {
            if let Some(count) = parameter.array_count
                && (count >= parameters.len() || count == position)
            {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "native array count parameter index is invalid",
                });
            }
        }
        let return_type =
            native::Type::lower(database, method.entity().file(), owner, return_type)?;
        let indirect_return = return_type.is_indirect_return(database)?;
        let mut package_dependencies = return_type.package_dependencies(database)?;
        for parameter in &parameters {
            package_dependencies.extend(parameter.ty.package_dependencies(database)?);
        }
        Ok(Self {
            flags,
            parameters,
            return_type,
            indirect_return,
            package_dependencies,
        })
    }

    pub(super) fn named_types(&self, mut add: impl FnMut(&str, &str)) {
        for parameter in &self.parameters {
            parameter.ty.named_types(&mut add);
        }
        self.return_type.named_types(add);
    }

    pub(super) fn package_dependencies(&self) -> &BTreeSet<(String, String)> {
        &self.package_dependencies
    }

    pub(super) fn write_parameters_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = parameter
                .ty
                .write_abi_projection(namespace, layout, projection);
            quote! { #name: #ty }
        });
        quote! { #(#parameters),* }
    }

    pub(super) fn write_delegate_parameters_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            if !parameter.is_input_only()
                && let Some((_, interface)) = parameter.ty.interface_out()
            {
                let ty = interface.write_public(namespace, layout);
                quote! { #name: windows_core::OutRef<#ty> }
            } else {
                let ty = parameter
                    .ty
                    .write_abi_projection(namespace, layout, projection);
                quote! { #name: #ty }
            }
        });
        quote! { #(#parameters),* }
    }

    pub(super) fn write_vtable_parameters_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let indirect_return = self.indirect_return.then(|| {
            let ty = self
                .return_type
                .write_abi_projection(namespace, layout, projection);
            quote! { , *mut #ty }
        });
        let parameters = self.parameters.iter().map(|parameter| {
            let ty = parameter
                .ty
                .write_abi_projection(namespace, layout, projection);
            quote! { #ty }
        });
        quote! { *mut core::ffi::c_void #indirect_return #(, #parameters)* }
    }

    pub(super) fn write_result(&self, namespace: &str, layout: Layout) -> TokenStream {
        self.write_vtable_result_projection(namespace, layout, Projection::Sys)
    }

    pub(super) fn write_vtable_result_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        if self.indirect_return {
            quote! {}
        } else {
            self.write_result_projection(namespace, layout, projection)
        }
    }

    fn array_info(
        parameter: windows_metadata2::ParameterDefinition<'_>,
        owner: &str,
    ) -> Result<(Option<usize>, Option<usize>), Error> {
        let attribute =
            if let Some(attribute) = parameter.find_attribute("NativeArrayInfoAttribute")? {
                attribute
            } else if let Some(attribute) = parameter.find_attribute("MemorySizeAttribute")? {
                attribute
            } else {
                return Ok((None, None));
            };
        let mut result = None;
        let mut length = None;
        for argument in attribute.arguments(&())? {
            let (name, value) = match argument {
                AttributeArgument::Named { name, value, .. } => (Some(name), value),
                AttributeArgument::Fixed { value, .. } => (None, value),
            };
            let value = match value {
                AttributeValue::I16(value) => i64::from(value),
                AttributeValue::I32(value) => i64::from(value),
                AttributeValue::U16(value) => i64::from(value),
                AttributeValue::U32(value) => i64::from(value),
                _ => continue,
            };
            let value = usize::try_from(value).map_err(|_| Error::InvalidType {
                name: owner.to_string(),
                message: "native array relationship value is negative",
            })?;
            match name.as_deref() {
                Some("CountConst" | "SizeConst") => length = Some(value),
                Some("CountParamIndex" | "SizeParamIndex") | None => result = Some(value),
                _ => {}
            }
        }
        Ok((result, length))
    }

    pub(super) fn write_result_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        if self.return_type == native::Type::Void {
            quote! {}
        } else if !projection.is_sys() && self.return_type.is_interface() {
            let ty = self.return_type.write_public(namespace, layout);
            quote! { -> Option<#ty> }
        } else {
            let ty = self
                .return_type
                .write_abi_projection(namespace, layout, projection);
            quote! { -> #ty }
        }
    }

    pub(super) fn write_vtable_parameters_named(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let indirect_return = self.indirect_return.then(|| {
            let ty = self
                .return_type
                .write_abi_projection(namespace, layout, projection);
            quote! { , result__: *mut #ty }
        });
        let parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = parameter
                .ty
                .write_abi_projection(namespace, layout, projection);
            quote! { #name: #ty }
        });
        quote! { this: *mut core::ffi::c_void #indirect_return #(, #parameters)* }
    }
}
