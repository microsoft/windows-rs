use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;

pub(super) struct Signature {
    pub(super) flags: u8,
    pub(super) parameters: Vec<Parameter>,
    pub(super) return_type: native::Type,
    pub(super) indirect_return: bool,
    pub(super) no_return: bool,
    return_plan: ReturnPlan,
    package_return_plan_override: Option<ReturnPlan>,
    package_dependencies: BTreeSet<(String, String)>,
}

pub(super) struct Parameter {
    pub(super) name: String,
    direction: Direction,
    optional: bool,
    pub(super) com_out_ptr: bool,
    array: Option<ArrayInfo>,
    pub(super) retval_candidate: bool,
    pub(super) explicit_retval: bool,
    pub(super) retval_transmute: bool,
    pub(super) producer_by_ref: bool,
    pub(super) ty: native::Type,
    pointer_alias_cast: bool,
    hint: ParamHint,
    pointer_cast: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayInfo {
    ElementsParam(usize),
    BytesParam(usize),
    ElementsConst(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Input,
    Output,
    InputOutput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParamHint {
    None,
    Slice {
        element: native::Type,
        transmute: bool,
    },
    SliceCount {
        position: usize,
        newtype: bool,
    },
    FixedArray {
        len: usize,
        element: native::Type,
        indirect: bool,
    },
    IntoParam,
    PackageIntoParam,
    Optional,
    Bool,
    ValueType,
    Blittable,
    ByRef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReturnPlan {
    HResult,
    Void,
    VoidInterface {
        position: usize,
    },
    VoidValue {
        position: usize,
    },
    Direct,
    Indirect,
    Retval {
        position: usize,
    },
    Query {
        guid: usize,
        object: usize,
        optional: bool,
    },
}

impl Parameter {
    pub(super) fn is_input_only(&self) -> bool {
        matches!(self.direction, Direction::Input)
    }

    pub(super) fn is_output_only(&self) -> bool {
        matches!(self.direction, Direction::Output)
    }

    pub(super) fn is_interface_output(&self) -> bool {
        !self.is_input_only() && self.ty.is_interface()
    }

    pub(super) fn is_optional(&self) -> bool {
        self.optional
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
        self.pointer_cast || (matches!(self.hint, ParamHint::ValueType) && !self.is_input_only())
    }

    pub(super) fn needs_method_cast(&self) -> bool {
        self.needs_cast() || self.pointer_alias_cast
    }

    pub(super) fn has_array_info(&self) -> bool {
        self.array.is_some()
    }

    pub(super) fn array_count(&self) -> Option<usize> {
        match self.array {
            Some(ArrayInfo::ElementsParam(position) | ArrayInfo::BytesParam(position)) => {
                Some(position)
            }
            _ => None,
        }
    }

    pub(super) fn is_mutable_pointer(&self) -> bool {
        matches!(self.ty, native::Type::Pointer { mutable: true, .. })
            || self.ty.mutable_string_pointer()
    }

    pub(super) fn slice_plan(&self) -> Option<(&native::Type, bool)> {
        match &self.hint {
            ParamHint::Slice { element, transmute } => Some((element, *transmute)),
            _ => None,
        }
    }

    pub(super) fn slice_parameter(&self) -> Option<(usize, bool)> {
        match self.hint {
            ParamHint::SliceCount { position, newtype } => Some((position, newtype)),
            _ => None,
        }
    }

    pub(super) fn fixed_array_plan(&self) -> Option<(usize, &native::Type, bool)> {
        match &self.hint {
            ParamHint::FixedArray {
                len,
                element,
                indirect,
            } => Some((*len, element, *indirect)),
            _ => None,
        }
    }
}

fn slice_plan(
    ty: &native::Type,
    pointer_alias: Option<&native::Type>,
    bytes: bool,
) -> Option<ParamHint> {
    let element = if bytes {
        native::Type::U8
    } else if ty.is_pcstr() {
        native::Type::U8
    } else if ty.is_pcwstr() {
        native::Type::U16
    } else if ty.mutable_string_pointer() {
        if matches!(ty, native::Type::Named { name, .. } if name == "PSTR") {
            native::Type::U8
        } else {
            native::Type::U16
        }
    } else if let Some(element) = ty.pointee() {
        if element == &native::Type::Void {
            native::Type::U8
        } else {
            element.clone()
        }
    } else if pointer_alias.is_some() {
        ty.clone()
    } else {
        return None;
    };
    let transmute = ty.is_const_string()
        || ty.mutable_string_pointer()
        || pointer_alias.is_some()
        || ty
            .pointee()
            .is_some_and(|abi| abi == &native::Type::Void || abi != &element);
    Some(ParamHint::Slice { element, transmute })
}

fn fixed_array_plan(ty: &native::Type, len: usize) -> ParamHint {
    let pointee = ty.pointee();
    let element = if ty.mutable_string_pointer() {
        if matches!(ty, native::Type::Named { name, .. } if name == "PSTR") {
            native::Type::U8
        } else {
            native::Type::U16
        }
    } else {
        pointee.cloned().unwrap_or_else(|| ty.clone())
    };
    ParamHint::FixedArray {
        len,
        element,
        indirect: pointee.is_none(),
    }
}

fn core_manifest_dependencies(ty: &native::Type) -> BTreeSet<(String, String)> {
    match ty {
        native::Type::Array { element, .. } | native::Type::Pointer { element, .. } => {
            core_manifest_dependencies(element)
        }
        native::Type::Interface {
            namespace,
            name,
            arguments,
        } if arguments.is_empty() && native::is_core_projection(namespace, name) => {
            BTreeSet::from([(namespace.clone(), name.clone())])
        }
        native::Type::Named { namespace, name } if native::is_core_projection(namespace, name) => {
            BTreeSet::from([(namespace.clone(), name.clone())])
        }
        _ => BTreeSet::new(),
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
        let returns_void = matches!(return_type.kind, TypeKind::Void);
        let parameter_rows = method.parameters_by_sequence()?;
        let lowered: Vec<(Parameter, Option<native::Type>)> = parameters
            .into_iter()
            .enumerate()
            .map(|(position, ty)| {
                let parameter = parameter_rows.parameters()[position];
                let flags = parameter
                    .map(|parameter| parameter.flags())
                    .transpose()?
                    .unwrap_or(0);
                let direction = if flags & 0x0002 == 0 {
                    Direction::Input
                } else if flags & 0x0001 == 0 {
                    Direction::Output
                } else {
                    Direction::InputOutput
                };
                let optional = flags & 0x0010 != 0;
                let array = parameter
                    .map(|parameter| Self::array_info(parameter, owner))
                    .transpose()?
                    .unwrap_or_default();
                let buffer = array.is_some();
                let explicit_retval = parameter
                    .map(|parameter| parameter.has_attribute("RetValAttribute"))
                    .transpose()?
                    .unwrap_or(false);
                let reserved = parameter
                    .map(|parameter| parameter.has_attribute("ReservedAttribute"))
                    .transpose()?
                    .unwrap_or(false);
                let metadata_retval_too_large = match &ty.kind {
                    TypeKind::Pointer(element) if returns_void => {
                        native::metadata_exceeds_retval_limit(
                            database,
                            method.entity().file(),
                            element,
                        )?
                    }
                    TypeKind::Pointer(element) => native::metadata_has_oversized_member(
                        database,
                        method.entity().file(),
                        element,
                    )?,
                    _ => false,
                };
                let ty = native::Type::lower_parameter(
                    database,
                    method.entity().file(),
                    owner,
                    ty,
                    flags & 0x0002 == 0,
                )?;
                let resolved_pointer_alias = ty.resolved_pointer_alias(database)?;
                let pointer_alias_cast = if let Some(native::Type::Pointer {
                    mutable: true,
                    element,
                }) = &resolved_pointer_alias
                {
                    !element.is_primitive(database)?
                        && !ty.resolves_to_delegate(database, &mut BTreeSet::new())?
                } else {
                    false
                };
                let pointer_alias = array.is_some().then_some(resolved_pointer_alias).flatten();
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
                                && !metadata_retval_too_large
                    } else {
                        false
                    }
                } else {
                    false
                };
                let producer_by_ref = flags & 0x0002 == 0
                    && (ty.is_const_string() || ty.producer_by_ref(database)?);
                let pointer_cast = flags & 0x0002 != 0
                    && ty.needs_output_pointer_cast(database, &mut BTreeSet::new())?;
                let com_out_ptr = parameter
                    .map(|parameter| parameter.has_attribute("ComOutPtrAttribute"))
                    .transpose()?
                    .unwrap_or(false);
                let copyable = ty.projected_traits(database, &mut BTreeSet::new())?.copy;
                let delegate = ty.is_delegate(database)?;
                let pointee_copyable = ty
                    .pointee()
                    .map(|ty| ty.projected_traits(database, &mut BTreeSet::new()))
                    .transpose()?
                    .is_none_or(|traits| traits.copy);
                let retval_transmute = if let Some(ty) = ty.pointee() {
                    ty.is_interface()
                        || ty.is_bstr()
                        || ty.is_hstring()
                        || (!ty.is_void_alias(database)? && !pointee_copyable)
                } else {
                    false
                };
                let hint = if let Some(ArrayInfo::ElementsConst(len)) = array
                    && (flags & 0x0002 == 0 || flags & 0x0001 != 0)
                {
                    fixed_array_plan(&ty, len)
                } else if flags & 0x0002 == 0 && (ty.is_interface() || ty.is_pcwstr()) {
                    ParamHint::IntoParam
                } else if flags & 0x0002 == 0 && ty.is_const_string() {
                    ParamHint::PackageIntoParam
                } else if (flags & 0x0010 != 0 || reserved) && copyable && !delegate {
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
                Ok((
                    Parameter {
                        name: parameter
                            .map(|parameter| parameter.name())
                            .transpose()?
                            .map_or_else(|| format!("p{position}"), str::to_lowercase),
                        direction,
                        optional,
                        com_out_ptr,
                        array,
                        retval_candidate,
                        explicit_retval,
                        retval_transmute,
                        producer_by_ref,
                        ty,
                        pointer_alias_cast,
                        hint,
                        pointer_cast,
                    },
                    pointer_alias,
                ))
            })
            .collect::<Result<_, Error>>()?;
        let (mut parameters, pointer_aliases): (Vec<_>, Vec<_>) = lowered.into_iter().unzip();
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
                || !parameters[count].ty.is_integer(database)?
                || (!matches!(parameters[position].ty, native::Type::Pointer { .. })
                    && pointer_aliases[position].is_none()
                    && !parameters[position].ty.is_const_string()
                    && !parameters[position].ty.mutable_string_pointer())
            {
                continue;
            }
            let bytes = matches!(parameters[position].array, Some(ArrayInfo::BytesParam(_)));
            if bytes {
                if parameters[position].ty.is_pcwstr()
                    || (!parameters[position].ty.is_const_string()
                        && !matches!(
                            parameters[position].ty.pointee(),
                            Some(native::Type::I8 | native::Type::U8)
                        ))
                {
                    continue;
                }
            }
            parameters[position].hint = slice_plan(
                &parameters[position].ty,
                pointer_aliases[position].as_ref(),
                bytes,
            )
            .unwrap();
            parameters[count].hint = ParamHint::SliceCount {
                position,
                newtype: parameters[count].ty.is_newtype(database)?,
            };
        }
        let return_type =
            native::Type::lower(database, method.entity().file(), owner, return_type)?;
        let indirect_return = return_type.is_indirect_return(database)?;
        let no_return = return_type == native::Type::Void
            && method.find_attribute("DoesNotReturnAttribute")?.is_some();
        let return_plan = Self::classify_return(&parameters, &return_type, indirect_return, false);
        let package_return_plan =
            Self::classify_return(&parameters, &return_type, indirect_return, true);
        let package_return_plan_override =
            (package_return_plan != return_plan).then_some(package_return_plan);
        let mut package_dependencies = return_type.package_dependencies(database, dependencies)?;
        for parameter in &parameters {
            package_dependencies.extend(parameter.ty.package_dependencies(database, dependencies)?);
        }
        Ok(Self {
            flags,
            parameters,
            return_type,
            indirect_return,
            no_return,
            return_plan,
            package_return_plan_override,
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

    pub(super) fn uses_winrt_projection(&self) -> bool {
        self.return_type.uses_winrt_projection()
            || self
                .parameters
                .iter()
                .any(|parameter| parameter.ty.uses_winrt_projection())
    }

    pub(super) fn manifest_dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = core_manifest_dependencies(&self.return_type);
        for parameter in &self.parameters {
            dependencies.extend(core_manifest_dependencies(&parameter.ty));
        }
        dependencies
    }

    pub(super) fn return_plan(&self, package: bool) -> ReturnPlan {
        if package {
            self.package_return_plan_override
                .unwrap_or(self.return_plan)
        } else {
            self.return_plan
        }
    }

    fn classify_return(
        parameters: &[Parameter],
        return_type: &native::Type,
        indirect_return: bool,
        package: bool,
    ) -> ReturnPlan {
        if !(return_type.is_hresult() || (package && return_type.is_hresult_package())) {
            if return_type == &native::Type::Void {
                if let Some((position, ty)) = Self::retval_parameter(parameters) {
                    return if ty.is_interface() {
                        ReturnPlan::VoidInterface { position }
                    } else {
                        ReturnPlan::VoidValue { position }
                    };
                }
                return ReturnPlan::Void;
            }
            if indirect_return {
                return ReturnPlan::Indirect;
            }
            return ReturnPlan::Direct;
        }
        if let Some((guid, object)) = Self::query_parameters(parameters) {
            return ReturnPlan::Query {
                guid,
                object,
                optional: parameters[object].is_optional(),
            };
        }
        if let Some((position, ty)) = Self::retval_parameter(parameters) {
            if ty
                .pointee()
                .is_some_and(|pointee| pointee == &native::Type::Void)
                && !parameters[position].explicit_retval
            {
                ReturnPlan::HResult
            } else {
                ReturnPlan::Retval { position }
            }
        } else {
            ReturnPlan::HResult
        }
    }

    fn retval_parameter(parameters: &[Parameter]) -> Option<(usize, &native::Type)> {
        let (parameter, preceding) = parameters.split_last()?;
        if parameter.retval_candidate
            && (parameter.explicit_retval || preceding.iter().all(Parameter::is_input_only))
        {
            Some((preceding.len(), parameter.ty.pointee()?))
        } else {
            None
        }
    }

    fn query_parameters(parameters: &[Parameter]) -> Option<(usize, usize)> {
        let guid = parameters.iter().rposition(|parameter| {
            parameter.is_input_only()
                && matches!(
                    &parameter.ty,
                    native::Type::Pointer {
                        mutable: false,
                        element,
                    } if element.is_guid()
                )
        })?;
        let object = parameters.iter().rposition(|parameter| {
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
            } else if parameter.is_input_only() && parameter.ty.is_interface() {
                let ty = parameter.ty.write_public(namespace, layout);
                quote! { #name: windows_core::Ref<#ty> }
            } else if parameter.is_input_only() && parameter.ty.interface_out().is_some() {
                let ty = parameter
                    .ty
                    .write_interface_pointer(namespace, layout, None)
                    .unwrap();
                quote! { #name: #ty }
            } else if parameter.is_input_only() && parameter.ty.is_hstring() {
                let ty = parameter.ty.write_public(namespace, layout);
                quote! { #name: windows_core::Ref<#ty> }
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

    pub(super) fn write_function_result_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        if projection.is_sys() {
            return self.write_result_projection(namespace, layout, projection);
        }
        if self.return_type == native::Type::Void {
            quote! {}
        } else if self.return_type.is_interface() {
            let ty = self.return_type.write_public(namespace, layout);
            quote! { -> Option<#ty> }
        } else {
            let ty = self.return_type.write_public(namespace, layout);
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
