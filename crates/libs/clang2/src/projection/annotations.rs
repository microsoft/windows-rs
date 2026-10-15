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

pub(super) fn function(function: &Function) -> TokenStream {
    let value = match function.exceptions {
        ExceptionSpecification::None => return TokenStream::new(),
        ExceptionSpecification::Noexcept => "noexcept",
        ExceptionSpecification::NoThrow => "nothrow",
        ExceptionSpecification::DynamicNone => "throw()",
    };
    quote! { #[Windows::Win32::Metadata::NativeExceptionSpecification(#value)] }
}

pub(super) fn callable(attributes: &[Attribute]) -> bool {
    attributes.iter().all(|attribute| {
        matches!(
            attribute.kind,
            AttributeKind::Sal(_) | AttributeKind::Deprecated(_) | AttributeKind::NoThrow
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
    let owner = typedef.map(|name| quote! { , TypedefName = #name });
    let sal = resolve::sal(attributes).into_iter().map(|value| {
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

pub(super) fn parameter(attributes: &[Attribute], ty: &MetadataType) -> Result<TokenStream, Error> {
    if !only_contracts(attributes) {
        return Err(Error("unsupported function parameter attributes".into()));
    }
    let (mut input, mut output, mut optional) = (false, false, false);
    for value in resolve::sal(attributes) {
        let direction = match value.name.as_str() {
            "_In_"
            | "_In_opt_"
            | "_In_z_"
            | "_In_opt_z_"
            | "_In_reads_"
            | "_In_reads_opt_"
            | "_In_reads_bytes_"
            | "_In_reads_bytes_opt_"
            | "__in"
            | "__in_opt" => Some((true, false)),
            "_Out_"
            | "_Out_opt_"
            | "_Out_writes_"
            | "_Out_writes_opt_"
            | "_Out_writes_to_"
            | "_Out_writes_to_opt_"
            | "_Out_writes_bytes_"
            | "_Out_writes_bytes_opt_"
            | "_Out_writes_bytes_to_"
            | "_Out_writes_bytes_to_opt_"
            | "_Outptr_"
            | "_Outptr_opt_"
            | "_Outptr_result_maybenull_"
            | "_Outptr_opt_result_maybenull_"
            | "_COM_Outptr_"
            | "_COM_Outptr_opt_"
            | "_COM_Outptr_result_maybenull_"
            | "_COM_Outptr_opt_result_maybenull_"
            | "__out"
            | "__out_opt" => Some((false, true)),
            "_Inout_" | "_Inout_opt_" | "__inout" | "__inout_opt" => Some((true, true)),
            _ => None,
        };
        if let Some((read, write)) = direction {
            input |= read;
            output |= write;
            optional |= value.name.contains("_opt_") || value.name.ends_with("_opt");
        }
    }
    // A by-value interface reference supplies an existing object, not a replaceable pointer.
    if matches!(ty, MetadataType::ClassName(_)) {
        input |= output;
        output = false;
    }
    let input = input.then(|| quote! { #[r#in] });
    let output = output.then(|| quote! { #[out] });
    let optional = optional.then(|| quote! { #[opt] });
    let native = write(attributes);
    Ok(quote! { #native #input #output #optional })
}
