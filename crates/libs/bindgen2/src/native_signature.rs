use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) struct Signature {
    pub(super) flags: u8,
    pub(super) parameters: Vec<Parameter>,
    pub(super) return_type: native::Type,
    pub(super) indirect_return: bool,
}

pub(super) struct Parameter {
    pub(super) name: String,
    flags: u16,
    pub(super) com_out_ptr: bool,
    pub(super) array_count: Option<usize>,
    pub(super) retval_candidate: bool,
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
                let array_count = parameter
                    .map(|parameter| Self::array_count(parameter, owner))
                    .transpose()?
                    .flatten();
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
                let retval_candidate =
                    if flags & 0x0002 != 0 && flags & 0x0001 == 0 && flags & 0x0010 == 0 && !buffer
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
                Ok(Parameter {
                    name: parameter
                        .map(|parameter| parameter.name())
                        .transpose()?
                        .map_or_else(|| format!("p{position}"), str::to_lowercase),
                    flags,
                    com_out_ptr: parameter
                        .map(|parameter| parameter.has_attribute("ComOutPtrAttribute"))
                        .transpose()?
                        .unwrap_or(false),
                    array_count,
                    retval_candidate,
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
        Ok(Self {
            flags,
            parameters,
            return_type,
            indirect_return,
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
            let ty = parameter
                .ty
                .write_abi_projection(namespace, layout, projection);
            quote! { #name: #ty }
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

    fn array_count(
        parameter: windows_metadata2::ParameterDefinition<'_>,
        owner: &str,
    ) -> Result<Option<usize>, Error> {
        let Some(attribute) = parameter.find_attribute("NativeArrayInfoAttribute")? else {
            return Ok(None);
        };
        let mut result = None;
        let mut other_relationship = false;
        for argument in attribute.arguments(&())? {
            let AttributeArgument::Named { name, value, .. } = argument else {
                continue;
            };
            if name == "CountConst" {
                other_relationship = true;
                continue;
            }
            if name != "CountParamIndex" {
                continue;
            }
            let AttributeValue::I16(value) = value else {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "native array count parameter index is not an i16",
                });
            };
            let value = usize::try_from(value).map_err(|_| Error::InvalidType {
                name: owner.to_string(),
                message: "native array count parameter index is negative",
            })?;
            if result.replace(value).is_some() {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "native array has multiple count parameter indexes",
                });
            }
        }
        Ok((!other_relationship).then_some(result).flatten())
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
