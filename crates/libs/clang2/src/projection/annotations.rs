use super::*;

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

pub(super) fn write(attributes: &[Attribute]) -> TokenStream {
    write_owned(attributes, None)
}

pub(super) fn function(function: &Function, attributes: &[Attribute]) -> TokenStream {
    let noreturn = (function.noreturn
        || attributes
            .iter()
            .any(|attribute| attribute.kind == AttributeKind::Cxx11NoReturn))
    .then(|| quote! { #[noreturn] });
    let value = match function.exceptions {
        ExceptionSpecification::None => return quote! { #noreturn },
        ExceptionSpecification::Noexcept => "noexcept",
        ExceptionSpecification::NoThrow => "nothrow",
        ExceptionSpecification::DynamicNone => "throw()",
    };
    quote! { #noreturn #[Windows::Win32::Metadata::NativeExceptionSpecification(#value)] }
}

pub(super) fn callable(attributes: &[Attribute]) -> bool {
    attributes.iter().all(|attribute| {
        matches!(
            attribute.kind,
            AttributeKind::Sal(_)
                | AttributeKind::Deprecated(_)
                | AttributeKind::NoThrow
                | AttributeKind::Cxx11NoReturn
        )
    })
}

pub(super) fn write_typedef(
    resolution: &Resolution<'_>,
    alias: &AliasGroup,
) -> Result<TokenStream, Error> {
    let attributes = covering(
        &alias.name,
        alias.observations.iter().map(|reference| {
            resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                [reference.observation]
                .attributes
                .as_slice()
        }),
    )?;
    Ok(write_owned(attributes, Some(&alias.name)))
}

pub(super) fn inherited_callback(
    resolution: &Resolution<'_>,
    snapshot: &Snapshot,
    mut written: TypeId,
) -> Result<TokenStream, Error> {
    let mut attributes = TokenStream::new();
    loop {
        match snapshot.types[written.0].kind {
            TypeKind::Alias(id) => {
                let alias = &snapshot.aliases[id.0];
                let group = resolution
                    .aliases
                    .iter()
                    .find(|group| group.name == alias.name)
                    .unwrap();
                attributes.extend(write_typedef(resolution, group)?);
                written = alias.observations[0].underlying;
            }
            TypeKind::Pointer(inner)
            | TypeKind::Attributed {
                modified: inner, ..
            } => written = inner,
            _ => return Ok(attributes),
        }
    }
}

fn write_owned(attributes: &[Attribute], typedef: Option<&str>) -> TokenStream {
    write_with_sal(attributes, typedef, resolve::sal(attributes))
}

fn write_with_sal<'a>(
    attributes: &[Attribute],
    typedef: Option<&str>,
    sal: impl IntoIterator<Item = &'a SalAnnotation>,
) -> TokenStream {
    let owner = typedef.map(|name| quote! { , TypedefName = #name });
    let sal = sal.into_iter().map(|value| {
        let SalAnnotation {
            version,
            name,
            arguments,
            expanded,
        } = value;
        quote! { #[Windows::Win32::Metadata::NativeSal(#version, #name, #arguments, #expanded #owner)] }
    });
    let deprecated = resolve::deprecations(attributes).into_iter().map(|value| {
        let Deprecation {
            message,
            replacement,
        } = value;
        quote! { #[Windows::Win32::Metadata::NativeDeprecated(#message, #replacement #owner)] }
    });
    quote! { #(#sal)* #(#deprecated)* }
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

pub(super) fn parameter(
    parameter: &Parameter,
    parameters: &[Parameter],
    ty: &MetadataType,
) -> Result<TokenStream, Error> {
    let attributes = &parameter.attributes;
    if !only_contracts(attributes) {
        return Err(Error("unsupported function parameter attributes".into()));
    }
    let (mut input, mut output, mut optional) = (false, false, false);
    for value in resolve::sal(attributes) {
        if let Some((direction, argument_optional)) = sal::direction(value) {
            input |= direction != sal::Direction::Output;
            output |= direction != sal::Direction::Input;
            optional |= argument_optional;
        }
    }
    let midl = resolve::midl_parameter(parameter);
    input |= midl.input;
    output |= midl.output;
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
                sal::ParameterAnnotation::Evidence(value) => Some(*value),
                sal::ParameterAnnotation::Contract(_) | sal::ParameterAnnotation::Reserved => None,
            }),
        )
    };
    // A by-value interface reference supplies an existing object, not a replaceable pointer.
    if matches!(ty, MetadataType::ClassName(_)) {
        input |= output;
        output = false;
    }
    let input = input.then(|| quote! { #[r#in] });
    let output = output.then(|| quote! { #[out] });
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
        Some(quote! {
            #[Windows::Win32::Metadata::NativeReturnedPointer(
                MayBeNull = #may_be_null, NullOnFailure = #null_on_failure
            )]
        })
    });
    let entry = normalized.iter().filter_map(|value| {
        let sal::ParameterAnnotation::Contract(contract) = value else {
            return None;
        };
        let read_only = contract.input_read_only?;
        Some(quote! { #[Windows::Win32::Metadata::NativeInput(ReadOnly = #read_only)] })
    });
    let retval = midl.retval.then(|| quote! { #[retval] });
    Ok(quote! { #native #input #output #optional #reserved #retval #(#returned)* #(#entry)* })
}

pub(super) fn buffer(
    snapshot: &Snapshot,
    function: &Function,
    parameters: &[Parameter],
    index: usize,
    ty: &MetadataType,
) -> Result<TokenStream, Error> {
    let mut size = None;
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
    let mut initialized = None;
    let (mut on_entry, mut on_success) = (false, false);
    for value in resolve::sal(&parameters[index].attributes) {
        let contract =
            sal::parameter(value, parameters).or_else(|| sal::rpc_buffer(value, parameters));
        post_writable |= contract.is_some_and(|contract| contract.post_writable);
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
        let capacity = contract
            .and_then(|contract| contract.capacity)
            .or_else(|| sal::capacity(value, parameters));
        let termination = sal::termination(value, contract);
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
        if let Some((entry, success)) = termination {
            on_entry |= entry;
            on_success |= success;
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
        if !matches!(&snapshot.types[function.parameters[count].0].kind,
            TypeKind::Scalar(scalar) if scalar.signed_integer().is_some())
        {
            return Err(Error(format!(
                "SAL buffer capacity `{}` requires a by-value integer parameter",
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
    let mut attributes = TokenStream::new();
    if com_out {
        attributes.extend(quote! { #[iid_is] });
    }
    if let Some(capacity) = size {
        let size = count_index(capacity.parameter)?;
        attributes.extend(if post_writable {
            quote! {
                #[Windows::Win32::Metadata::NativeArrayInfo(
                    CountParamIndex = #size, PostWritable = true)]
            }
        } else if capacity.units == sal::Units::Bytes {
            quote! { #[size_param(#size)] }
        } else {
            quote! { #[len_param(#size)] }
        });
    }
    if on_entry || on_success {
        attributes.extend(quote! {
            #[Windows::Win32::Metadata::NativeNullTerminated(
                OnEntry = #on_entry, OnSuccess = #on_success)]
        });
    }
    if let Some(length) = initialized {
        let index = count_index(length.count.parameter)?;
        let bytes = length.count.units == sal::Units::Bytes;
        let dereference = length.dereference;
        let fallback = length
            .null_fallback
            .map(count_index)
            .transpose()?
            .map(|index| quote! { , NullFallbackParamIndex = #index });
        attributes.extend(quote! {
            #[Windows::Win32::Metadata::NativeInitializedLength(
                CountParamIndex = #index, IsByteCount = #bytes, Dereference = #dereference #fallback)]
        });
    }
    Ok(attributes)
}

fn count_index(index: usize) -> Result<proc_macro2::Literal, Error> {
    let index: i16 = index
        .try_into()
        .map_err(|_| Error("SAL count parameter index exceeds metadata range".into()))?;
    Ok(proc_macro2::Literal::i16_unsuffixed(index))
}
