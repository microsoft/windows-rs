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
    manifest_dependencies: BTreeSet<(String, String)>,
}

pub(super) struct Parameter {
    pub(super) name: String,
    flags: u16,
    pub(super) com_out_ptr: bool,
    array: Option<ArrayInfo>,
    pub(super) retval_candidate: bool,
    pub(super) producer_by_ref: bool,
    pub(super) ty: native::Type,
    hint: ParamHint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayInfo {
    ElementsParam(usize),
    BytesParam(usize),
    ElementsConst(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParamHint {
    None,
    Slice,
    ByteSlice,
    SliceCount(usize),
    FixedArray(usize),
    IntoParam,
    PackageIntoParam,
    Optional,
    Bool,
    ValueType,
    Blittable,
    ByRef,
}

impl Parameter {
    pub(super) fn is_input_only(&self) -> bool {
        self.flags & 0x0002 == 0
    }

    pub(super) fn is_output_only(&self) -> bool {
        self.flags & 0x0002 != 0 && self.flags & 0x0001 == 0
    }

    pub(super) fn is_optional(&self) -> bool {
        self.flags & 0x0010 != 0
    }

    pub(super) fn is_into_param(&self, layout: Layout) -> bool {
        matches!(self.hint, ParamHint::IntoParam)
            || (layout.is_package() && matches!(self.hint, ParamHint::PackageIntoParam))
    }

    pub(super) fn is_bool(&self) -> bool {
        matches!(self.hint, ParamHint::Bool)
    }

    pub(super) fn is_optional_hint(&self) -> bool {
        matches!(self.hint, ParamHint::Optional)
    }

    pub(super) fn is_by_ref(&self) -> bool {
        matches!(self.hint, ParamHint::ByRef)
    }

    pub(super) fn needs_cast(&self) -> bool {
        matches!(self.hint, ParamHint::ValueType) && !self.is_input_only()
    }

    pub(super) fn has_array_info(&self) -> bool {
        self.array.is_some()
    }

    pub(super) fn is_mutable_pointer(&self) -> bool {
        matches!(self.ty, native::Type::Pointer { mutable: true, .. })
    }

    pub(super) fn slice_element(&self) -> Option<native::Type> {
        match self.hint {
            ParamHint::ByteSlice => Some(native::Type::U8),
            ParamHint::Slice => self.ty.pointee().map(|element| {
                if element == &native::Type::Void {
                    native::Type::U8
                } else {
                    element.clone()
                }
            }),
            _ => None,
        }
    }

    pub(super) fn slice_parameter(&self) -> Option<usize> {
        match self.hint {
            ParamHint::SliceCount(position) => Some(position),
            _ => None,
        }
    }

    pub(super) fn fixed_array_len(&self) -> Option<usize> {
        match self.hint {
            ParamHint::FixedArray(len) => Some(len),
            _ => None,
        }
    }
}

impl Signature {
    pub(super) fn lower(
        database: &Database,
        dependencies: &native::DependencyCache,
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
        let mut parameters: Vec<Parameter> = parameters
            .into_iter()
            .enumerate()
            .map(|(position, ty)| {
                let parameter = parameter_rows.parameters()[position];
                let flags = parameter
                    .map(|parameter| parameter.flags())
                    .transpose()?
                    .unwrap_or(0);
                let array = parameter
                    .map(|parameter| Self::array_info(parameter, owner))
                    .transpose()?
                    .unwrap_or_default();
                let buffer = array.is_some();
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
                let com_out_ptr = parameter
                    .map(|parameter| parameter.has_attribute("ComOutPtrAttribute"))
                    .transpose()?
                    .unwrap_or(false);
                let copyable = ty.projected_traits(database, &mut BTreeSet::new())?.copy;
                let pointee_copyable = ty
                    .pointee()
                    .map(|ty| ty.projected_traits(database, &mut BTreeSet::new()))
                    .transpose()?
                    .is_none_or(|traits| traits.copy);
                let hint = if let Some(ArrayInfo::ElementsConst(len)) = array
                    && flags & 0x0002 == 0
                {
                    ParamHint::FixedArray(len)
                } else if flags & 0x0002 == 0 && (ty.is_interface() || ty.is_pcwstr()) {
                    ParamHint::IntoParam
                } else if flags & 0x0002 == 0 && ty.is_const_string() {
                    ParamHint::PackageIntoParam
                } else if flags & 0x0010 != 0 && copyable {
                    ParamHint::Optional
                } else if flags & 0x0002 == 0 && ty.is_bool() {
                    ParamHint::Bool
                } else if ty.is_primitive(database)? && pointee_copyable {
                    ParamHint::ValueType
                } else if copyable {
                    ParamHint::Blittable
                } else if flags & 0x0002 == 0 {
                    ParamHint::ByRef
                } else {
                    ParamHint::None
                };
                Ok(Parameter {
                    name: parameter
                        .map(|parameter| parameter.name())
                        .transpose()?
                        .map_or_else(|| format!("p{position}"), str::to_lowercase),
                    flags,
                    com_out_ptr,
                    array,
                    retval_candidate,
                    producer_by_ref,
                    ty,
                    hint,
                })
            })
            .collect::<Result<_, Error>>()?;
        for (position, parameter) in parameters.iter().enumerate() {
            if let Some(ArrayInfo::ElementsParam(count) | ArrayInfo::BytesParam(count)) =
                parameter.array
                && (count >= parameters.len() || count == position)
            {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "native array count parameter index is invalid",
                });
            }
        }
        let mut references = vec![0usize; parameters.len()];
        for parameter in &parameters {
            if let Some(ArrayInfo::ElementsParam(count) | ArrayInfo::BytesParam(count)) =
                parameter.array
            {
                references[count] += 1;
            }
        }
        for position in 0..parameters.len() {
            let Some(ArrayInfo::ElementsParam(count) | ArrayInfo::BytesParam(count)) =
                parameters[position].array
            else {
                continue;
            };
            if references[count] != 1
                || parameters[position].is_output_only()
                || !parameters[count].is_input_only()
                || parameters[count].is_optional()
                || !matches!(
                    parameters[count].ty,
                    native::Type::U32 | native::Type::USize
                )
                || !matches!(parameters[position].ty, native::Type::Pointer { .. })
            {
                continue;
            }
            if matches!(parameters[position].array, Some(ArrayInfo::BytesParam(_))) {
                if !matches!(
                    parameters[position].ty.pointee(),
                    Some(native::Type::I8 | native::Type::U8)
                ) {
                    continue;
                }
                parameters[position].hint = ParamHint::ByteSlice;
            } else {
                parameters[position].hint = ParamHint::Slice;
            }
            parameters[count].hint = ParamHint::SliceCount(position);
        }
        let return_type =
            native::Type::lower(database, method.entity().file(), owner, return_type)?;
        let indirect_return = return_type.is_indirect_return(database)?;
        let mut package_dependencies = return_type.package_dependencies(database, dependencies)?;
        let mut manifest_dependencies = return_type.manifest_dependencies(database)?;
        for parameter in &parameters {
            package_dependencies.extend(parameter.ty.package_dependencies(database, dependencies)?);
            manifest_dependencies.extend(parameter.ty.manifest_dependencies(database)?);
        }
        Ok(Self {
            flags,
            parameters,
            return_type,
            indirect_return,
            package_dependencies,
            manifest_dependencies,
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

    pub(super) fn manifest_dependencies(&self) -> &BTreeSet<(String, String)> {
        &self.manifest_dependencies
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
    ) -> Result<Option<ArrayInfo>, Error> {
        let Some(attribute) = parameter
            .find_attribute("NativeArrayInfoAttribute")?
            .or(parameter.find_attribute("MemorySizeAttribute")?)
        else {
            return Ok(None);
        };
        let mut result = None;
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
                Some("CountConst" | "SizeConst") => result = Some(ArrayInfo::ElementsConst(value)),
                Some("CountParamIndex") | None => result = Some(ArrayInfo::ElementsParam(value)),
                Some("BytesParamIndex" | "SizeParamIndex") => {
                    result = Some(ArrayInfo::BytesParam(value))
                }
                _ => {}
            }
        }
        Ok(result)
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
