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

pub(super) struct CallableAttributes {
    pub declaration: TokenStream,
    pub result: TokenStream,
}

pub(super) fn write_callable(
    snapshot: &Snapshot,
    result_type: TypeId,
    attributes: &[Attribute],
    parameters: &[Parameter],
    names: &[&str],
) -> Result<CallableAttributes, Error> {
    let mut must_use = false;
    let success = success_condition(snapshot, result_type, [attributes], false);
    let nullable = nullable_result(snapshot, result_type, [attributes]);
    let owned = write_result_owned(
        attributes,
        None,
        &mut must_use,
        success.is_some(),
        nullable,
        |value| integer_expression::return_range(value, result_type, parameters, snapshot),
        names,
    )?;
    let native = owned.declaration;
    let mut result_attributes = owned.result;
    if nullable {
        result_attributes.extend(quote! { #[nullable] });
    }
    let result = must_use.then(|| quote! { #[must_use] });
    let success = success.map(write_success);
    Ok(CallableAttributes {
        declaration: quote! { #native #result #success },
        result: result_attributes,
    })
}

fn returned_nullable(value: &sal::Annotation<'_>) -> bool {
    value.unscoped().is_some_and(|value| {
        value.version == "2"
            && value.name == "_Ret_maybenull_"
            && value.arguments.is_empty()
            && value.expanded.is_empty()
    })
}

fn data_pointer(snapshot: &Snapshot, mut ty: TypeId) -> bool {
    while let TypeKind::Alias(id) = snapshot.types[ty.0].kind {
        ty = snapshot.aliases[id.0].observations[0].canonical;
    }
    matches!(snapshot.types[ty.0].kind, TypeKind::Pointer(inner)
        if !matches!(snapshot.types[inner.0].kind, TypeKind::Function(_)))
}

/// Storage for a returned pointer whose target type is known, excluding untyped `void**`.
fn typed_out_pointer(snapshot: &Snapshot, ty: TypeId) -> bool {
    let TypeKind::Pointer(inner) = snapshot.types[ty.0].kind else {
        return false;
    };
    let TypeKind::Pointer(target) = snapshot.types[inner.0].kind else {
        return false;
    };
    !matches!(
        snapshot.types[target.0].kind,
        TypeKind::Void | TypeKind::Function(_)
    )
}

fn nullable_result<'a>(
    snapshot: &Snapshot,
    result: TypeId,
    owners: impl IntoIterator<Item = &'a [Attribute]>,
) -> bool {
    if !data_pointer(snapshot, result) {
        return false;
    }
    let mut nullable = false;
    for value in owners.into_iter().flat_map(resolve::sal) {
        if value.value.name == "_Ret_notnull_" {
            return false;
        }
        nullable |= returned_nullable(&value);
    }
    nullable
}

fn write_result_owned(
    attributes: &[Attribute],
    typedef: Option<&str>,
    must_use: &mut bool,
    success: bool,
    nullable: bool,
    range: impl Fn(&sal::Annotation<'_>) -> Option<windows_metadata::IntegerRange>,
    names: &[&str],
) -> Result<CallableAttributes, Error> {
    let sal = resolve::sal(attributes);
    *must_use |= sal.iter().any(result_check);
    let mut native = Vec::new();
    let mut result = TokenStream::new();
    for value in sal {
        if let Some(range) = range(&value) {
            result.extend(
                windows_rdl::write_integer_range(&range, names)
                    .ok_or_else(|| Error("cannot render return integer range".into()))?,
            );
        } else {
            native.push(value);
        }
    }
    let declaration = write_with_sal(
        attributes,
        typedef,
        native.into_iter().filter(|value| {
            !result_check(value)
                && !(success && is_success(value.value))
                && !(nullable && returned_nullable(value))
        }),
    )?;
    Ok(CallableAttributes {
        declaration,
        result,
    })
}

/// The success condition SAL applies to a callable: its own `_Success_` when present, otherwise
/// the `_Return_type_success_` inherited from its written result typedefs.
pub(super) fn callable_success(
    resolution: &Resolution<'_>,
    snapshot: &Snapshot,
    written_result: TypeId,
    result_type: TypeId,
    attributes: &[Attribute],
) -> Result<Option<SuccessCondition>, Error> {
    if resolve::sal(attributes)
        .iter()
        .any(|value| is_success(value.value))
    {
        return Ok(success_condition(
            snapshot,
            result_type,
            [attributes],
            false,
        ));
    }
    let inherited = value_alias_attributes(resolution, snapshot, written_result)?;
    Ok(success_condition(snapshot, result_type, inherited, true))
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
    interface: bool,
) -> Result<TokenStream, Error> {
    let mut normalized = sal::fields(&field.attributes, fields);
    let buffer = !interface
        && (data_pointer(snapshot, field.canonical)
            || matches!(
                snapshot.types[field.canonical.0].kind,
                TypeKind::Array { .. }
            ));
    if normalized.iter().any(|value| {
        let sal::FieldAnnotation::Contract(contract) = value else {
            return false;
        };
        std::iter::once(&contract.capacity)
            .chain(&contract.readable)
            .any(|count| {
                !matches!(count, sal::Expression::Parameter(_))
                    && (!buffer || field_count(count, fields, snapshot).is_none())
            })
    }) {
        normalized = resolve::sal(&field.attributes)
            .into_iter()
            .map(sal::FieldAnnotation::Evidence)
            .collect();
    }
    let nullable = data_pointer(snapshot, field.canonical)
        && normalized.iter().all(|value| match value {
            sal::FieldAnnotation::Contract(_) => false,
            sal::FieldAnnotation::Evidence(value) => value.value.name != "_Notnull_",
        })
        && normalized.iter().any(
            |value| matches!(value, sal::FieldAnnotation::Evidence(value) if field_nullable(value)),
        );
    let mut result = write_with_sal(
        &field.attributes,
        None,
        normalized.iter().filter_map(|value| match value {
            sal::FieldAnnotation::Evidence(value) => ((!nullable || !field_nullable(value))
                && integer_expression::field_range(value, field, fields, snapshot).is_none())
            .then(|| value.clone()),
            sal::FieldAnnotation::Contract(_) => None,
        }),
    )?;
    // Inline array storage cannot be null, so `_opt_` only applies to pointer fields.
    let optional = data_pointer(snapshot, field.canonical)
        && normalized.iter().any(
            |value| matches!(value, sal::FieldAnnotation::Contract(contract) if contract.optional),
        );
    if nullable || optional {
        result.extend(quote! { #[nullable] });
    }
    for value in resolve::sal(&field.attributes) {
        if let Some(range) = integer_expression::field_range(&value, field, fields, snapshot) {
            result.extend(
                windows_rdl::write_integer_range(&range, &[])
                    .ok_or_else(|| Error("cannot render field integer range".into()))?,
            );
        }
    }
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
        for (index, _) in std::iter::once(&contract.capacity)
            .chain(&contract.readable)
            .flat_map(sal::Expression::references)
        {
            if !matches!(&snapshot.types[fields[index].canonical.0].kind,
                TypeKind::Scalar(scalar) if scalar.signed_integer().is_some())
            {
                return Err(Error(format!(
                    "SAL field buffer count `{}` requires an integer field",
                    fields[index].name
                )));
            }
        }
        for (access, count) in
            std::iter::once((windows_metadata::BufferAccess::Write, &contract.capacity)).chain(
                contract
                    .readable
                    .as_ref()
                    .map(|count| (windows_metadata::BufferAccess::Read, count)),
            )
        {
            let extent = windows_metadata::BufferExtent::new(
                access,
                windows_metadata::BufferPhase::Value,
                extent_units(contract.units),
                field_count(count, fields, snapshot)
                    .ok_or_else(|| Error("unsupported field buffer count".into()))?,
            );
            result.extend(windows_rdl::write_buffer_extent(&extent, &[]).unwrap());
        }
    }
    Ok(result)
}

fn field_count(
    count: &sal::Expression,
    fields: &[Field],
    snapshot: &Snapshot,
) -> Option<windows_metadata::BufferCount> {
    use windows_metadata::BufferCount;
    Some(match count {
        sal::Expression::Constant(value) => BufferCount::Constant((*value).try_into().ok()?),
        sal::Expression::Parameter(index) => BufferCount::Field(fields.get(*index)?.name.clone()),
        _ => BufferCount::Expression(integer_expression::field_expression(
            count, fields, snapshot,
        )?),
    })
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
    parameters: &[Parameter],
) -> Result<CallableAttributes, Error> {
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
    let nullable = nullable_result(
        snapshot,
        result_type,
        owners.iter().map(|(attributes, _)| *attributes),
    );
    let mut attributes = TokenStream::new();
    let mut result_attributes = TokenStream::new();
    let names = parameter_names(parameters, true);
    let names: Vec<_> = names.iter().map(String::as_str).collect();
    for (owner, typedef) in owners {
        let owned = write_result_owned(
            owner,
            typedef,
            &mut must_use,
            success.is_some(),
            nullable,
            |value| integer_expression::return_range(value, result_type, parameters, snapshot),
            &names,
        )?;
        attributes.extend(owned.declaration);
        result_attributes.extend(owned.result);
    }
    if nullable {
        result_attributes.extend(quote! { #[nullable] });
    }
    let result = must_use.then(|| quote! { #[must_use] });
    let success = success.map(write_success);
    Ok(CallableAttributes {
        declaration: quote! { #attributes #result #success },
        result: result_attributes,
    })
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
    let deprecations = resolve::deprecations(attributes);
    if deprecations.len() > 1 {
        return Err(Error(format!(
            "conflicting deprecation messages: {:?}",
            deprecations
                .iter()
                .map(|value| value.message.as_str())
                .collect::<Vec<_>>()
        )));
    }
    let deprecated = deprecations.into_iter().map(|Deprecation { message }| {
        let message = (!message.is_empty()).then(|| quote! { #message });
        let owner = typedef.map(|name| quote! { TypedefName = #name });
        let arguments: Vec<_> = message.into_iter().chain(owner).collect();
        if arguments.is_empty() {
            quote! { #[deprecated] }
        } else {
            quote! { #[deprecated(#(#arguments),*)] }
        }
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
    // Only a complete COM returned-pointer contract supersedes MIDL's incoming-pointer flag.
    let com_output = matches!(
        sal::parameters(attributes, parameters).as_slice(),
        [sal::ParameterAnnotation::Contract(contract)]
            if contract.direction == sal::Direction::Output
                && contract.returned_pointer.is_some_and(|pointer| pointer.com)
    );
    let midl = resolve::midl_parameter(parameter);
    if !com_output {
        input |= midl.input;
        output |= midl.output;
    }
    optional |= midl.optional;
    Ok((input, output, optional))
}

pub(super) fn parameter(
    snapshot: &Snapshot,
    function: &Function,
    parameter: &Parameter,
    parameters: &[Parameter],
    canonical: &MetadataType,
    success: Option<SuccessCondition>,
) -> Result<TokenStream, Error> {
    let attributes = &parameter.attributes;
    if !only_contracts(attributes) {
        return Err(Error("unsupported function parameter attributes".into()));
    }
    let (mut input, mut output, optional) =
        parameter_direction(snapshot, function, parameter, parameters)?;
    let midl = resolve::midl_parameter(parameter);
    if midl.retval && (!output || input || matches!(canonical, MetadataType::ClassName(_))) {
        return Err(Error("MIDL result marker requires output storage".into()));
    }
    let normalized = sal::parameters(attributes, parameters);
    let interface_output = matches!(canonical, MetadataType::PtrMut(target, 1)
        if matches!(**target, MetadataType::ClassName(_) | MetadataType::Object));
    let com_result = success == Some(SuccessCondition::Nonnegative)
        && parameters
            .last()
            .is_some_and(|last| std::ptr::eq(last, parameter))
        && output
        && !input
        && !optional
        && !midl.iid_is
        && !matches!(canonical, MetadataType::ClassName(_))
        && typed_out_pointer(snapshot, function.parameters[parameters.len() - 1])
        && matches!(
            normalized.as_slice(),
            [sal::ParameterAnnotation::Contract(contract)]
                if contract.returned_pointer.is_some_and(|pointer| {
                    !pointer.may_be_null && (pointer.com || interface_output)
                })
        );
    let preserve = normalized.iter().any(|value| {
        matches!(
            value,
            sal::ParameterAnnotation::Contract(contract)
                if matches!(canonical, MetadataType::ClassName(_))
                    && contract.direction != sal::Direction::Input
                    || !count_expressions_supported(snapshot, function, contract)
        )
    });
    let mut residual = Vec::new();
    for value in &normalized {
        if let sal::ParameterAnnotation::Evidence(value) = value
            && integer_expression::range(value, parameter, parameters, snapshot).is_none()
            && pointee_readable(value, parameter, parameters)?.is_none()
        {
            residual.push(value.clone());
        }
    }
    let native = if preserve {
        write(attributes)
    } else {
        write_with_sal(attributes, None, residual)
    }?;
    // A by-value interface reference supplies an existing object, not a replaceable pointer.
    if matches!(canonical, MetadataType::ClassName(_)) {
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
        Some(windows_rdl::emit::write_returned_pointer(
            contract.returned_pointer?.null_on_failure,
        ))
    });
    // A required non-null returned pointer (`_COM_Outptr_`, or `_Outptr_` to an interface) on the
    // trailing output of a `return >= 0` callable meets the MIDL `[retval]` contract: the callee
    // writes a non-null object on every success, and callers read it only on success.
    let retval = (midl.retval || com_result).then(|| quote! { #[retval] });
    let names: Vec<_> = parameters
        .iter()
        .map(|parameter| parameter.name.as_str())
        .collect();
    let mut ranges = TokenStream::new();
    if !preserve {
        for value in resolve::sal(attributes) {
            if let Some(range) = integer_expression::range(&value, parameter, parameters, snapshot)
            {
                ranges.extend(
                    windows_rdl::write_integer_range(&range, &names)
                        .ok_or_else(|| Error("cannot render integer range".into()))?,
                );
            }
        }
    }
    Ok(quote! { #native #direction #optional #reserved #retval #(#returned)* #ranges })
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
    for value in resolve::sal(&parameters[index].attributes) {
        if let Some(units) = pointee_readable(&value, &parameters[index], parameters)? {
            returned_storage(snapshot, function, index, canonical_ty)?;
            let mut extent = windows_metadata::BufferExtent::new(
                windows_metadata::BufferAccess::Read,
                windows_metadata::BufferPhase::Success,
                units,
                windows_metadata::BufferCount::Unknown,
            );
            extent.indirect = true;
            attributes.extend(write_extent(&extent, names)?);
        }
    }
    let mut nullable = false;
    for value in sal::parameters(&parameters[index].attributes, parameters) {
        let sal::ParameterAnnotation::Contract(contract) = value else {
            continue;
        };
        let may_be_null = contract
            .returned_pointer
            .is_some_and(|pointer| pointer.may_be_null);
        nullable |= may_be_null;
        if !count_expressions_supported(snapshot, function, &contract) {
            continue;
        }
        nullable |= contract.returned_nullable_on_entry;
        if let Some(buffer) = &contract.returned_buffer {
            nullable |= buffer.nullable;
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
        if contract.terminated
            && (contract.returned_pointer.is_some() || contract.returned_buffer.is_some())
        {
            returned_storage(snapshot, function, index, canonical_ty)?;
            if contract.returned_string_on_entry {
                let mut extent = windows_metadata::BufferExtent::new(
                    windows_metadata::BufferAccess::Read,
                    windows_metadata::BufferPhase::Entry,
                    windows_metadata::BufferUnits::Elements,
                    windows_metadata::BufferCount::Constant(1),
                );
                extent.indirect = true;
                attributes.extend(write_extent(&extent, names)?);
            }
            attributes.extend(windows_rdl::emit::write_null_terminated(
                contract.returned_string_on_entry.then_some(false),
                Some(false),
                true,
            ));
        }
    }
    if nullable {
        attributes.extend(quote! { #[nullable] });
    }
    let midl = resolve::midl_parameter(&parameters[index]);
    let (input, output, _) =
        parameter_direction(snapshot, function, &parameters[index], parameters)?;
    let mut com_out = midl.iid_is && output && !input;
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
        let contract = value
            .unscoped()
            .and_then(|value| {
                sal::parameter(value, parameters).or_else(|| sal::rpc_buffer(value, parameters))
            })
            .filter(|contract| count_expressions_supported(snapshot, function, contract));
        if contract
            .as_ref()
            .is_some_and(|contract| contract.returned_buffer.is_some())
        {
            continue;
        }
        post_writable |= contract
            .as_ref()
            .is_some_and(|contract| contract.post_writable);
        let input_partial = value.value.version == "1.2"
            && matches!(
                value.value.name.as_str(),
                "__RPC__in_ecount_part"
                    | "__RPC__in_ecount_part_opt"
                    | "__RPC__in_xcount_part"
                    | "__RPC__in_xcount_part_opt"
            );
        if let Some(contract) = &contract {
            entry_readable |= contract.direction == sal::Direction::Input && !input_partial
                || contract.direction == sal::Direction::InputOutput
                    && contract.initialized.is_some();
            entry_writable |= contract.direction != sal::Direction::Input || input_partial;
        } else if let Some((direction, _)) = value.unscoped().and_then(sal::direction) {
            entry_readable |= direction == sal::Direction::Input && !input_partial;
            entry_writable |= direction != sal::Direction::Input;
        }
        if let Some(pointer) = contract
            .as_ref()
            .and_then(|contract| contract.returned_pointer)
        {
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
        let capacity = contract
            .as_ref()
            .and_then(|contract| contract.capacity.clone())
            .or_else(|| {
                value
                    .unscoped()
                    .and_then(|value| sal::capacity(value, parameters))
                    .filter(|count| count_expression_supported(snapshot, function, count))
            });
        let termination = sal::termination(&value, contract.as_ref());
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
            validate_count(
                snapshot,
                function,
                parameters,
                &length.count,
                "SAL initialized length",
            )?;
            if initialized
                .as_ref()
                .is_some_and(|previous| previous != &length)
            {
                return Err(Error(
                    "conflicting SAL initialized length parameters, units, or indirection".into(),
                ));
            }
            initialized = Some(length);
        }
        let Some(capacity) = capacity else {
            continue;
        };
        validate_count(
            snapshot,
            function,
            parameters,
            &capacity,
            "SAL buffer capacity",
        )?;
        if size.as_ref().is_some_and(|previous| previous != &capacity) {
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
                let mut extent = buffer_extent(&capacity, access, phase)?;
                count_optional(snapshot, function, parameters, &mut extent)?;
                attributes.extend(write_extent(&extent, names)?);
            }
        }
    }
    if on_entry || on_success {
        attributes.extend(windows_rdl::emit::write_null_terminated(
            on_entry.then_some(double_entry),
            on_success.then_some(double_success),
            false,
        ));
    }
    if let Some(length) = initialized {
        let mut extent = buffer_extent(
            &length.count,
            windows_metadata::BufferAccess::Read,
            windows_metadata::BufferPhase::Success,
        )?;
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

fn field_nullable(value: &sal::Annotation<'_>) -> bool {
    value.unscoped().is_some_and(|value| {
        value.version == "2"
            && value.name == "_Maybenull_"
            && value.arguments.is_empty()
            && value.expanded.is_empty()
    })
}

fn count_optional(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    extent: &mut windows_metadata::BufferExtent,
) -> Result<(), Error> {
    if extent.null_fallback.is_some() {
        return Ok(());
    }
    fn expression(
        value: &mut windows_metadata::IntegerExpression,
        snapshot: &Snapshot,
        function: &Function,
        parameters: &[Parameter],
    ) -> Result<(), Error> {
        use windows_metadata::{IntegerExpression as E, IntegerReference as R};
        match value {
            E::Reference(R::Dereference(index)) => {
                if parameter_direction(
                    snapshot,
                    function,
                    &parameters[*index as usize],
                    parameters,
                )?
                .2
                {
                    *value = E::Reference(R::OptionalDereference(*index));
                }
            }
            E::Add(left, right) | E::Subtract(left, right) | E::Multiply(left, right) => {
                expression(left, snapshot, function, parameters)?;
                expression(right, snapshot, function, parameters)?;
            }
            _ => {}
        }
        Ok(())
    }
    if let windows_metadata::BufferCount::Expression(value) = &mut extent.count {
        expression(value, snapshot, function, parameters)?;
    }
    if let windows_metadata::BufferCount::Dereference(index) = extent.count {
        extent.count_optional =
            parameter_direction(snapshot, function, &parameters[index as usize], parameters)?.2;
    }
    Ok(())
}

fn count_expression_supported(
    snapshot: &Snapshot,
    function: &Function,
    count: &sal::Count,
) -> bool {
    if !matches!(count.source, sal::CountSource::Expression(_)) {
        return true;
    }
    count.references().into_iter().all(|(index, dereference)| {
        let mut ty = function.parameters[index];
        if dereference {
            let TypeKind::Pointer(inner) = snapshot.types[ty.0].kind else {
                return false;
            };
            ty = inner;
        }
        integer_expression::integer(snapshot, ty)
    })
}

fn count_expressions_supported(
    snapshot: &Snapshot,
    function: &Function,
    contract: &sal::ParameterContract,
) -> bool {
    contract
        .capacity
        .iter()
        .chain(contract.initialized.iter().map(|value| &value.count))
        .chain(
            contract
                .returned_buffer
                .iter()
                .flat_map(|value| value.writable.iter().chain(&value.readable)),
        )
        .all(|count| count_expression_supported(snapshot, function, count))
}

fn pointee_readable(
    annotation: &sal::Annotation<'_>,
    parameter: &Parameter,
    parameters: &[Parameter],
) -> Result<Option<windows_metadata::BufferUnits>, Error> {
    use proc_macro2::{Delimiter, TokenTree};
    use windows_metadata::BufferUnits;

    fn call(tokens: TokenStream, name: &str) -> Option<TokenStream> {
        let mut tokens = tokens.into_iter();
        match (tokens.next()?, tokens.next()?, tokens.next()) {
            (TokenTree::Ident(ident), TokenTree::Group(group), None)
                if ident == name && group.delimiter() == Delimiter::Parenthesis =>
            {
                Some(group.stream())
            }
            _ => None,
        }
    }

    let Some(value) = annotation.unscoped() else {
        return Ok(None);
    };
    if value.version != "2" || value.name != "_At_" {
        return Ok(None);
    }
    let text = sal::transport::expression(&value.expanded)?;
    let Ok(tokens) = text.parse::<TokenStream>() else {
        return Ok(None);
    };
    let tokens: Vec<_> = tokens.into_iter().collect();
    let Some(comma) = tokens
        .iter()
        .position(|token| matches!(token, TokenTree::Punct(value) if value.as_char() == ','))
    else {
        return Ok(None);
    };
    let subject = tokens[..comma].iter().cloned().collect::<TokenStream>();
    let names: Vec<_> = parameters.iter().map(|value| value.name.as_str()).collect();
    let Some(sal::Expression::Dereference(index)) =
        sal::Expression::parse(&subject.to_string(), &names)
    else {
        return Ok(None);
    };
    if parameters[index].name != parameter.name {
        return Ok(None);
    }
    let postcondition: TokenStream = tokens[comma + 1..].iter().cloned().collect();
    for (name, units) in [
        ("_Post_readable_size_", BufferUnits::Elements),
        ("_Post_readable_byte_size_", BufferUnits::Bytes),
    ] {
        if let Some(count) = call(postcondition.clone(), name)
            && call(count, "_Inexpressible_").is_some_and(|reason| !reason.is_empty())
        {
            return Ok(Some(units));
        }
    }
    Ok(None)
}

fn returned_storage(
    snapshot: &Snapshot,
    function: &Function,
    index: usize,
    ty: &MetadataType,
) -> Result<(), Error> {
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
    Ok(())
}

fn returned_buffer(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    index: usize,
    buffer: &sal::ReturnedBuffer,
    ty: &MetadataType,
    names: &[&str],
) -> Result<TokenStream, Error> {
    returned_storage(snapshot, function, index, ty)?;
    let mut result = TokenStream::new();
    for (access, count) in [
        (windows_metadata::BufferAccess::Write, &buffer.writable),
        (windows_metadata::BufferAccess::Read, &buffer.readable),
    ] {
        let Some(count) = count else { continue };
        validate_count(
            snapshot,
            function,
            parameters,
            count,
            "SAL returned-buffer count",
        )?;
        let mut extent = buffer_extent(count, access, windows_metadata::BufferPhase::Success)?;
        extent.indirect = true;
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

fn validate_count(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    count: &sal::Count,
    description: &str,
) -> Result<(), Error> {
    for (index, dereference) in count.references() {
        let name = &parameters[index].name;
        let mut kind = &snapshot.types[function.parameters[index].0].kind;
        if dereference {
            let TypeKind::Pointer(inner) = kind else {
                return Err(Error(format!(
                    "{description} `{name}` requires a pointer to an integer parameter"
                )));
            };
            kind = &snapshot.types[inner.0].kind;
        }
        if !matches!(kind, TypeKind::Scalar(scalar) if scalar.signed_integer().is_some()) {
            return Err(Error(format!(
                "{description} `{name}` requires an integer parameter"
            )));
        }
    }
    Ok(())
}

fn buffer_extent(
    count: &sal::Count,
    access: windows_metadata::BufferAccess,
    phase: windows_metadata::BufferPhase,
) -> Result<windows_metadata::BufferExtent, Error> {
    let source = if let sal::CountSource::Expression(value) = &count.source {
        windows_metadata::BufferCount::Expression(
            integer_expression::project(value)
                .ok_or_else(|| Error("invalid SAL integer expression".into()))?,
        )
    } else if let sal::CountSource::Constant(value) = count.source {
        windows_metadata::BufferCount::Constant(value)
    } else {
        let (index, dereference) = count.parameter().unwrap();
        let index = u16::try_from(index)
            .map_err(|_| Error("SAL count parameter index exceeds metadata range".into()))?;
        if dereference {
            windows_metadata::BufferCount::Dereference(index)
        } else {
            windows_metadata::BufferCount::Parameter(index)
        }
    };
    Ok(windows_metadata::BufferExtent::new(
        access,
        phase,
        extent_units(count.units),
        source,
    ))
}

fn write_extent(
    extent: &windows_metadata::BufferExtent,
    names: &[&str],
) -> Result<TokenStream, Error> {
    windows_rdl::write_buffer_extent(extent, names)
        .ok_or_else(|| Error("invalid SAL buffer extent".into()))
}
