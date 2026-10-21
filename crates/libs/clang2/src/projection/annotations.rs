use super::*;
use windows_rdl::emit::{SuccessCondition, write_success};

pub(super) fn covering<'a>(
    name: &str,
    observations: impl Iterator<Item = &'a [Attribute]>,
) -> Result<&'a [Attribute], Error> {
    let sets: Vec<_> = observations
        .map(|attrs| (attrs, resolve::sal(attrs)))
        .collect();
    let Some((attributes, largest)) = sets.iter().max_by_key(|(_, sal)| sal.len()) else {
        return Err(Error(format!("no annotation observations for `{name}`")));
    };
    if sets.iter().any(|(_, sal)| {
        sal.iter()
            .any(|value| largest.binary_search(value).is_err())
    }) {
        return Err(Error(format!(
            "no observed SAL annotation set contains every contract for `{name}`"
        )));
    }
    Ok(attributes)
}

pub(super) fn write(attributes: &[Attribute]) -> Result<TokenStream, Error> {
    write_owned(attributes, None)
}

pub(super) fn write_callable(
    snapshot: &Snapshot,
    result_type: TypeId,
    attributes: &[Attribute],
) -> Result<TokenStream, Error> {
    let mut must_use = false;
    let success = success_condition(snapshot, result_type, [attributes], false);
    let native = write_result_owned(attributes, None, &mut must_use, success.is_some())?;
    let result = must_use.then(|| quote! { #[must_use] });
    let success = success.map(write_success);
    Ok(quote! { #native #result #success })
}

fn write_result_owned(
    attributes: &[Attribute],
    typedef: Option<&str>,
    must_use: &mut bool,
    success: bool,
) -> Result<TokenStream, Error> {
    let sal = resolve::sal(attributes);
    *must_use |= sal.iter().any(result_check);
    write_with_sal(
        attributes,
        typedef,
        sal.into_iter()
            .filter(|value| !result_check(value) && !(success && is_success(value.value))),
    )
}

fn is_success(value: &SalAnnotation) -> bool {
    matches!(
        value.name.as_str(),
        "_Success_" | "__success" | "_Return_type_success_"
    )
}

fn success_condition<'a>(
    snapshot: &Snapshot,
    mut result_type: TypeId,
    attributes: impl IntoIterator<Item = &'a [Attribute]>,
    default: bool,
) -> Option<SuccessCondition> {
    use SuccessCondition::*;
    let kind = loop {
        match &snapshot.types[result_type.0].kind {
            TypeKind::Alias(id) => result_type = snapshot.aliases[id.0].observations[0].canonical,
            TypeKind::Enum(id) if !default && !snapshot.enums[id.0].observations[0].scoped => {
                result_type = snapshot.enums[id.0].observations[0].canonical;
            }
            kind => break kind,
        }
    };
    let mut result = None;
    for attributes in attributes {
        for annotation in resolve::sal(attributes)
            .iter()
            .filter(|value| is_success(value.value))
        {
            let value = annotation.unscoped()?;
            let supported = if default {
                value.version == "2" && value.name == "_Return_type_success_"
            } else {
                value.version == "2" && value.name == "_Success_"
                    || value.version == "1.1" && value.name == "__success"
            };
            if !supported || value.arguments != value.expanded {
                return None;
            }
            let expression: TokenStream = value.arguments.parse().ok()?;
            let condition = SuccessCondition::from_expression(&expression).or_else(|| {
                match expression.to_string().as_str() {
                    "return" | "0 != return" | "return != false" => Some(NotEqualZero),
                    "0 == return" | "return == 0L" => Some(EqualZero),
                    _ => None,
                }
            })?;
            let valid = match kind {
                TypeKind::Scalar(Scalar::Bool) => {
                    matches!(condition, EqualZero | NotEqualZero | EqualOne)
                }
                TypeKind::Scalar(scalar) => scalar
                    .signed_integer()
                    .is_some_and(|signed| condition != Nonnegative || signed),
                TypeKind::Pointer(inner) => {
                    !matches!(snapshot.types[inner.0].kind, TypeKind::Function(_))
                        && matches!(condition, EqualZero | NotEqualZero)
                }
                _ => false,
            };
            if !valid || result.is_some_and(|previous| previous != condition) {
                return None;
            }
            result = Some(condition);
        }
    }
    result
}

pub(super) fn write_value_typedef(
    snapshot: &Snapshot,
    canonical: TypeId,
    attributes: &[Attribute],
    inherited: &[&[Attribute]],
) -> Result<TokenStream, Error> {
    let success = success_condition(snapshot, canonical, [attributes], true).filter(|condition| {
        success_condition(
            snapshot,
            canonical,
            std::iter::once(attributes).chain(inherited.iter().copied()),
            true,
        ) == Some(*condition)
    });
    let native = write_with_sal(
        attributes,
        None,
        resolve::sal(attributes)
            .into_iter()
            .filter(|value| !(success.is_some() && is_success(value.value))),
    )?;
    let success = success.map(write_success);
    Ok(quote! { #native #success })
}

pub(super) fn value_alias_attributes<'a>(
    resolution: &'a Resolution<'_>,
    snapshot: &Snapshot,
    mut written: TypeId,
) -> Result<Vec<&'a [Attribute]>, Error> {
    let mut result = Vec::new();
    while let TypeKind::Alias(id) = snapshot.types[written.0].kind {
        let alias = &snapshot.aliases[id.0];
        let group = resolution
            .aliases
            .iter()
            .find(|group| group.name == alias.name)
            .unwrap();
        let attributes = typedef_attributes(resolution, group)?;
        if resolve::sal(attributes)
            .iter()
            .any(|value| value.value.name == "_Return_type_success_")
        {
            result.push(attributes);
        }
        written = alias.observations[0].underlying;
    }
    Ok(result)
}

fn result_check(value: &sal::Annotation<'_>) -> bool {
    value.unscoped().is_some_and(|value| {
        value.version == "2"
            && matches!(
                value.name.as_str(),
                "_Check_return_" | "_Must_inspect_result_"
            )
            && value.arguments.is_empty()
            && value.expanded.is_empty()
    })
}

pub(super) fn field(
    snapshot: &Snapshot,
    field: &Field,
    fields: &[Field],
) -> Result<TokenStream, Error> {
    let normalized = sal::fields(&field.attributes, fields);
    let mut result = write_with_sal(
        &field.attributes,
        None,
        normalized.iter().filter_map(|value| match value {
            sal::FieldAnnotation::Evidence(value) => Some(value.clone()),
            sal::FieldAnnotation::Contract(_) => None,
        }),
    )?;
    if let Some(alignment) = field_alignment(&field.attributes)? {
        let alignment = proc_macro2::Literal::u16_unsuffixed(alignment);
        result.extend(quote! { #[align(#alignment)] });
    }
    for value in normalized {
        let sal::FieldAnnotation::Contract(contract) = value else {
            continue;
        };
        match snapshot.types[field.canonical.0].kind {
            TypeKind::Pointer(inner)
                if !matches!(snapshot.types[inner.0].kind, TypeKind::Function(_)) => {}
            TypeKind::Array { .. } => {}
            _ => {
                return Err(Error(format!(
                    "SAL field buffer `{}` requires data pointer or array storage",
                    field.name
                )));
            }
        }
        for index in std::iter::once(contract.capacity).chain(contract.readable) {
            if !matches!(&snapshot.types[fields[index].canonical.0].kind,
                TypeKind::Scalar(scalar) if scalar.signed_integer().is_some())
            {
                return Err(Error(format!(
                    "SAL field buffer count `{}` requires an integer field",
                    fields[index].name
                )));
            }
        }
        for (access, index) in
            std::iter::once((windows_metadata::BufferAccess::Write, contract.capacity)).chain(
                contract
                    .readable
                    .map(|index| (windows_metadata::BufferAccess::Read, index)),
            )
        {
            let mut extent = windows_metadata::BufferExtent::new(
                access,
                windows_metadata::BufferPhase::Value,
                extent_units(contract.units),
                windows_metadata::BufferCount::Field(fields[index].name.clone()),
            );
            extent.nullable = contract.optional;
            result.extend(windows_rdl::write_buffer_extent(&extent, &[]).unwrap());
        }
    }
    Ok(result)
}

pub(super) fn field_alignment(attributes: &[Attribute]) -> Result<Option<u16>, Error> {
    resolve::alignments(attributes)
        .into_iter()
        .map(|(bits, _)| bits / 8)
        .filter(|bytes| *bytes != 0)
        .max()
        .map(|bytes| {
            bytes
                .try_into()
                .map_err(|_| Error("field alignment exceeds metadata limits".into()))
        })
        .transpose()
}

pub(super) fn function(function: &Function, attributes: &[Attribute]) -> TokenStream {
    let noreturn = (function.noreturn
        || attributes
            .iter()
            .any(|attribute| attribute.kind == AttributeKind::Cxx11NoReturn))
    .then(|| quote! { #[noreturn] });
    quote! { #noreturn }
}

pub(super) fn callable(attributes: &[Attribute]) -> bool {
    attributes.iter().all(|attribute| {
        matches!(
            attribute.kind,
            AttributeKind::Sal(_)
                | AttributeKind::Deprecated(_)
                | AttributeKind::NoThrow
                | AttributeKind::Cxx11NoReturn
                | AttributeKind::MSAllocator
        )
    })
}

pub(super) fn write_typedef(
    resolution: &Resolution<'_>,
    alias: &AliasGroup,
) -> Result<TokenStream, Error> {
    write_owned(typedef_attributes(resolution, alias)?, Some(&alias.name))
}

fn typedef_attributes<'a>(
    resolution: &'a Resolution<'_>,
    alias: &AliasGroup,
) -> Result<&'a [Attribute], Error> {
    covering(
        &alias.name,
        alias.observations.iter().map(|reference| {
            resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                [reference.observation]
                .attributes
                .as_slice()
        }),
    )
}

pub(super) fn write_callback(
    resolution: &Resolution<'_>,
    snapshot: &Snapshot,
    mut written: TypeId,
    result_type: TypeId,
    attributes: &[Attribute],
) -> Result<TokenStream, Error> {
    let mut owners = vec![(attributes, None)];
    loop {
        match snapshot.types[written.0].kind {
            TypeKind::Alias(id) => {
                let alias = &snapshot.aliases[id.0];
                let group = resolution
                    .aliases
                    .iter()
                    .find(|group| group.name == alias.name)
                    .unwrap();
                owners.push((
                    typedef_attributes(resolution, group)?,
                    Some(group.name.as_str()),
                ));
                written = alias.observations[0].underlying;
            }
            TypeKind::Pointer(inner)
            | TypeKind::Attributed {
                modified: inner, ..
            } => written = inner,
            _ => break,
        }
    }
    let success = success_condition(
        snapshot,
        result_type,
        owners.iter().map(|(attributes, _)| *attributes),
        false,
    );
    let mut must_use = false;
    let mut attributes = TokenStream::new();
    for (owner, typedef) in owners {
        attributes.extend(write_result_owned(
            owner,
            typedef,
            &mut must_use,
            success.is_some(),
        )?);
    }
    let result = must_use.then(|| quote! { #[must_use] });
    let success = success.map(write_success);
    Ok(quote! { #attributes #result #success })
}

fn write_owned(attributes: &[Attribute], typedef: Option<&str>) -> Result<TokenStream, Error> {
    write_with_sal(attributes, typedef, resolve::sal(attributes))
}

fn write_with_sal<'a>(
    attributes: &[Attribute],
    typedef: Option<&str>,
    sal: impl IntoIterator<Item = sal::Annotation<'a>>,
) -> Result<TokenStream, Error> {
    let owner = typedef.map(|name| quote! { , TypedefName = #name });
    let sal = sal.into_iter().map(|value| {
        let modifiers = (!value.modifiers.is_empty()).then(|| {
            let names = value.modifiers.iter().map(|value| value.name.as_str()).collect::<Vec<_>>().join(" ");
            quote! { , Modifiers = #names }
        });
        let SalAnnotation {
            version,
            name,
            arguments,
            expanded,
        } = value.value;
        let arguments = sal::transport::expression(arguments)?;
        let expanded = sal::transport::expression(expanded)?;
        Ok(quote! { #[Windows::Win32::Metadata::NativeSal(#version, #name, #arguments, #expanded #owner #modifiers)] })
    }).collect::<Result<Vec<_>, Error>>()?;
    let deprecated = resolve::deprecations(attributes).into_iter().map(|value| {
        let Deprecation {
            message,
            replacement,
        } = value;
        quote! { #[Windows::Win32::Metadata::NativeDeprecated(#message, #replacement #owner)] }
    });
    Ok(quote! { #(#sal)* #(#deprecated)* })
}

pub(super) fn only_contracts(attributes: &[Attribute]) -> bool {
    attributes.iter().all(|attribute| {
        matches!(
            attribute.kind,
            AttributeKind::Sal(_) | AttributeKind::Deprecated(_)
        )
    })
}

pub(super) fn parameter_is_annotated(parameter: &Parameter) -> bool {
    !parameter.attributes.is_empty()
        || resolve::midl_parameter(parameter) != resolve::MidlParameter::default()
}

fn parameter_direction(
    snapshot: &Snapshot,
    function: &Function,
    parameter: &Parameter,
    parameters: &[Parameter],
) -> Result<(bool, bool, bool), Error> {
    let attributes = &parameter.attributes;
    let (mut input, mut output, mut optional) = (false, false, false);
    for value in resolve::sal(attributes) {
        if let Some((direction, argument_optional)) = value.unscoped().and_then(sal::direction) {
            input |= direction != sal::Direction::Output;
            output |= direction != sal::Direction::Input;
            optional |= argument_optional;
        }
    }
    if let Some((common_input, common_output, common_optional)) =
        sal::common_direction(snapshot, function, attributes, parameters)?
    {
        input |= common_input;
        output |= common_output;
        optional |= common_optional;
    }
    let midl = resolve::midl_parameter(parameter);
    input |= midl.input;
    output |= midl.output;
    Ok((input, output, optional))
}

pub(super) fn parameter(
    snapshot: &Snapshot,
    function: &Function,
    parameter: &Parameter,
    parameters: &[Parameter],
    ty: &MetadataType,
) -> Result<TokenStream, Error> {
    let attributes = &parameter.attributes;
    if !only_contracts(attributes) {
        return Err(Error("unsupported function parameter attributes".into()));
    }
    let (mut input, mut output, optional) =
        parameter_direction(snapshot, function, parameter, parameters)?;
    let midl = resolve::midl_parameter(parameter);
    if midl.retval && (!output || input || matches!(ty, MetadataType::ClassName(_))) {
        return Err(Error("MIDL result marker requires output storage".into()));
    }
    let normalized = sal::parameters(attributes, parameters);
    let preserve = normalized.iter().any(|value| {
        matches!(
            value,
            sal::ParameterAnnotation::Contract(contract)
                if matches!(ty, MetadataType::ClassName(_))
                    && contract.direction != sal::Direction::Input
        )
    });
    let native = if preserve {
        write(attributes)
    } else {
        write_with_sal(
            attributes,
            None,
            normalized.iter().filter_map(|value| match value {
                sal::ParameterAnnotation::Evidence(value) => Some(value.clone()),
                sal::ParameterAnnotation::Contract(_)
                | sal::ParameterAnnotation::Termination(_)
                | sal::ParameterAnnotation::Reserved => None,
            }),
        )
    }?;
    // A by-value interface reference supplies an existing object, not a replaceable pointer.
    if matches!(ty, MetadataType::ClassName(_)) {
        input |= output;
        output = false;
    }
    let direction = write_param_direction(input, output);
    let optional = optional.then(|| quote! { #[opt] });
    let reserved = normalized
        .contains(&sal::ParameterAnnotation::Reserved)
        .then(|| quote! { #[reserved] });
    let returned = normalized.iter().filter_map(|value| {
        let sal::ParameterAnnotation::Contract(contract) = value else {
            return None;
        };
        let sal::ReturnedPointer {
            may_be_null,
            null_on_failure,
            ..
        } = contract.returned_pointer?;
        Some(windows_rdl::emit::write_returned_pointer(
            may_be_null,
            null_on_failure,
        ))
    });
    let retval = midl.retval.then(|| quote! { #[retval] });
    Ok(quote! { #native #direction #optional #reserved #retval #(#returned)* })
}

pub(super) fn buffer(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    index: usize,
    ty: &MetadataType,
    canonical_ty: &MetadataType,
    names: &[&str],
) -> Result<TokenStream, Error> {
    let mut size = None;
    let mut attributes = TokenStream::new();
    for value in sal::parameters(&parameters[index].attributes, parameters) {
        if let sal::ParameterAnnotation::Contract(contract) = value
            && let Some(buffer) = contract.returned_buffer
        {
            attributes.extend(returned_buffer(
                snapshot,
                function,
                parameters,
                index,
                buffer,
                canonical_ty,
                names,
            )?);
        }
    }
    let midl = resolve::midl_parameter(&parameters[index]);
    let mut com_out = midl.iid_is && midl.output && !midl.input;
    if (midl.retval || com_out)
        && !matches!(
            snapshot.types[function.parameters[index].0].kind,
            TypeKind::Pointer(_)
        )
    {
        return Err(Error("MIDL result marker requires a pointer".into()));
    }
    if com_out {
        let TypeKind::Pointer(inner) = snapshot.types[function.parameters[index].0].kind else {
            unreachable!()
        };
        if !matches!(snapshot.types[inner.0].kind, TypeKind::Pointer(_)) {
            return Err(Error(
                "MIDL iid_is requires pointer-to-pointer storage".into(),
            ));
        }
    }
    let mut post_writable = false;
    let mut entry_readable = false;
    let mut entry_writable = false;
    let mut initialized = None;
    let (mut on_entry, mut on_success) = (false, false);
    let (mut double_entry, mut double_success) = (false, false);
    for value in resolve::sal(&parameters[index].attributes) {
        let contract = value.unscoped().and_then(|value| {
            sal::parameter(value, parameters).or_else(|| sal::rpc_buffer(value, parameters))
        });
        if contract.is_some_and(|contract| contract.returned_buffer.is_some()) {
            continue;
        }
        post_writable |= contract.is_some_and(|contract| contract.post_writable);
        let input_partial = value.value.version == "1.2"
            && matches!(
                value.value.name.as_str(),
                "__RPC__in_ecount_part"
                    | "__RPC__in_ecount_part_opt"
                    | "__RPC__in_xcount_part"
                    | "__RPC__in_xcount_part_opt"
            );
        if let Some(contract) = contract {
            entry_readable |= contract.direction == sal::Direction::Input && !input_partial
                || contract.direction == sal::Direction::InputOutput
                    && contract.initialized.is_some();
            entry_writable |= contract.direction != sal::Direction::Input || input_partial;
        } else if let Some((direction, _)) = value.unscoped().and_then(sal::direction) {
            entry_readable |= direction == sal::Direction::Input && !input_partial;
            entry_writable |= direction != sal::Direction::Input;
        }
        if let Some(pointer) = contract.and_then(|contract| contract.returned_pointer) {
            let TypeKind::Pointer(inner) = snapshot.types[function.parameters[index].0].kind else {
                return Err(Error(
                    "SAL returned pointer requires pointer-to-pointer storage".into(),
                ));
            };
            let TypeKind::Pointer(pointee) = snapshot.types[inner.0].kind else {
                return Err(Error(
                    "SAL returned pointer requires pointer-to-pointer storage".into(),
                ));
            };
            com_out |= pointer.com
                && matches!(snapshot.types[pointee.0].kind, TypeKind::Void)
                && snapshot.types[inner.0].qualifiers == Qualifiers::default()
                && snapshot.types[pointee.0].qualifiers == Qualifiers::default();
        }
        let capacity = contract.and_then(|contract| contract.capacity).or_else(|| {
            value
                .unscoped()
                .and_then(|value| sal::capacity(value, parameters))
        });
        let termination = sal::termination(&value, contract);
        let length = contract.and_then(|contract| contract.initialized);
        if capacity.is_none() && termination.is_none() && length.is_none() {
            continue;
        }
        if !matches!(
            snapshot.types[function.parameters[index].0].kind,
            TypeKind::Pointer(_)
        ) || matches!(ty, MetadataType::ClassName(_) | MetadataType::FnPtr(_))
        {
            return Err(Error(
                "SAL buffer contract requires a buffer pointer".into(),
            ));
        }
        if let Some(termination) = termination {
            on_entry |= termination.on_entry;
            on_success |= termination.on_success;
            double_entry |= termination.double && termination.on_entry;
            double_success |= termination.double && termination.on_success;
        }
        if let Some(length) = length {
            if let Some(fallback) = length.null_fallback
                && !matches!(&snapshot.types[function.parameters[fallback].0].kind,
                    TypeKind::Scalar(scalar) if scalar.signed_integer().is_some())
            {
                return Err(Error(format!(
                    "SAL initialized length fallback `{}` requires a by-value integer parameter",
                    parameters[fallback].name,
                )));
            }
            let count = length.count.parameter;
            let mut kind = &snapshot.types[function.parameters[count].0].kind;
            if length.dereference
                && let TypeKind::Pointer(inner) = kind
            {
                kind = &snapshot.types[inner.0].kind;
            } else if length.dereference {
                return Err(Error(format!(
                    "SAL initialized length `{}` requires a pointer to an integer parameter",
                    parameters[count].name,
                )));
            }
            if !matches!(kind,
                    TypeKind::Scalar(scalar) if scalar.signed_integer().is_some())
            {
                return Err(Error(format!(
                    "SAL initialized length `{}` requires an integer count",
                    parameters[count].name,
                )));
            }
            if initialized.is_some_and(|previous| previous != length) {
                return Err(Error(
                    "conflicting SAL initialized length parameters, units, or indirection".into(),
                ));
            }
            initialized = Some(length);
        }
        let Some(capacity) = capacity else {
            continue;
        };
        let count = capacity.parameter;
        let mut kind = &snapshot.types[function.parameters[count].0].kind;
        if capacity.dereference {
            let TypeKind::Pointer(inner) = kind else {
                return Err(Error(
                    "SAL buffer capacity requires a pointer to an integer parameter".into(),
                ));
            };
            kind = &snapshot.types[inner.0].kind;
        }
        if !matches!(kind,
            TypeKind::Scalar(scalar) if scalar.signed_integer().is_some())
        {
            return Err(Error(format!(
                "SAL buffer capacity `{}` requires an integer parameter",
                parameters[count].name,
            )));
        }
        if size.is_some_and(|previous| previous != capacity) {
            return Err(Error(
                "conflicting SAL buffer capacity parameters or units".into(),
            ));
        }
        size = Some(capacity);
    }
    if com_out {
        attributes.extend(quote! { #[iid_is] });
    }
    if let Some(capacity) = size {
        for (access, phase, enabled) in [
            (
                windows_metadata::BufferAccess::Read,
                windows_metadata::BufferPhase::Entry,
                entry_readable,
            ),
            (
                windows_metadata::BufferAccess::Write,
                windows_metadata::BufferPhase::Entry,
                entry_writable,
            ),
            (
                windows_metadata::BufferAccess::Write,
                windows_metadata::BufferPhase::Success,
                post_writable,
            ),
        ] {
            if enabled {
                let count = extent_count(capacity.parameter, capacity.dereference)?;
                let mut extent = windows_metadata::BufferExtent::new(
                    access,
                    phase,
                    extent_units(capacity.units),
                    count,
                );
                count_optional(snapshot, function, parameters, &mut extent)?;
                attributes.extend(write_extent(&extent, names)?);
            }
        }
    }
    if on_entry || on_success {
        attributes.extend(windows_rdl::emit::write_null_terminated(
            on_entry.then_some(double_entry),
            on_success.then_some(double_success),
        ));
    }
    if let Some(length) = initialized {
        let mut extent = windows_metadata::BufferExtent::new(
            windows_metadata::BufferAccess::Read,
            windows_metadata::BufferPhase::Success,
            extent_units(length.count.units),
            extent_count(length.count.parameter, length.dereference)?,
        );
        extent.count_on_entry = length.on_entry;
        extent.null_fallback = length
            .null_fallback
            .map(|index| {
                u16::try_from(index)
                    .map_err(|_| Error("SAL count parameter index exceeds metadata range".into()))
            })
            .transpose()?;
        count_optional(snapshot, function, parameters, &mut extent)?;
        attributes.extend(write_extent(&extent, names)?);
    }
    Ok(attributes)
}

fn count_optional(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    extent: &mut windows_metadata::BufferExtent,
) -> Result<(), Error> {
    if extent.null_fallback.is_none()
        && let windows_metadata::BufferCount::Dereference(index) = extent.count
    {
        extent.count_optional =
            parameter_direction(snapshot, function, &parameters[index as usize], parameters)?.2;
    }
    Ok(())
}

fn returned_buffer(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    index: usize,
    buffer: sal::ReturnedBuffer,
    ty: &MetadataType,
    names: &[&str],
) -> Result<TokenStream, Error> {
    let TypeKind::Pointer(inner) = snapshot.types[function.parameters[index].0].kind else {
        return Err(Error(
            "SAL returned buffer requires pointer-to-pointer storage".into(),
        ));
    };
    if !matches!(snapshot.types[inner.0].kind, TypeKind::Pointer(_))
        || !matches!(ty, MetadataType::PtrMut(inner, depth) | MetadataType::PtrConst(inner, depth)
            if *depth >= 2 || matches!(**inner, MetadataType::PtrMut(..) | MetadataType::PtrConst(..)))
    {
        return Err(Error(
            "SAL returned buffer requires pointer-to-pointer data storage".into(),
        ));
    }
    let mut result = TokenStream::new();
    for (access, count) in [
        (windows_metadata::BufferAccess::Write, buffer.writable),
        (windows_metadata::BufferAccess::Read, buffer.readable),
    ] {
        let Some(count) = count else { continue };
        let mut kind = &snapshot.types[function.parameters[count.parameter].0].kind;
        if count.dereference {
            let TypeKind::Pointer(inner) = kind else {
                return Err(Error(
                    "SAL returned-buffer count requires a pointer to an integer".into(),
                ));
            };
            kind = &snapshot.types[inner.0].kind;
        }
        if !matches!(kind, TypeKind::Scalar(scalar) if scalar.signed_integer().is_some()) {
            return Err(Error(
                "SAL returned-buffer count requires an integer".into(),
            ));
        }
        let mut extent = windows_metadata::BufferExtent::new(
            access,
            windows_metadata::BufferPhase::Success,
            extent_units(count.units),
            extent_count(count.parameter, count.dereference)?,
        );
        extent.indirect = true;
        extent.nullable = buffer.nullable;
        count_optional(snapshot, function, parameters, &mut extent)?;
        result.extend(write_extent(&extent, names)?);
    }
    Ok(result)
}

fn extent_units(units: sal::Units) -> windows_metadata::BufferUnits {
    match units {
        sal::Units::Elements => windows_metadata::BufferUnits::Elements,
        sal::Units::Bytes => windows_metadata::BufferUnits::Bytes,
    }
}

fn extent_count(index: usize, dereference: bool) -> Result<windows_metadata::BufferCount, Error> {
    let index = u16::try_from(index)
        .map_err(|_| Error("SAL count parameter index exceeds metadata range".into()))?;
    Ok(if dereference {
        windows_metadata::BufferCount::Dereference(index)
    } else {
        windows_metadata::BufferCount::Parameter(index)
    })
}

fn write_extent(
    extent: &windows_metadata::BufferExtent,
    names: &[&str],
) -> Result<TokenStream, Error> {
    windows_rdl::write_buffer_extent(extent, names)
        .ok_or_else(|| Error("invalid SAL buffer extent".into()))
}
