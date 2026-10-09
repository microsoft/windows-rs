use super::*;

pub(super) fn attributes(
    annotation: &str,
    ty: &ProjectedType,
    parameters: &[ProjectedType],
    all_annotations: &[Vec<String>],
    sized: bool,
) -> Result<Option<String>, Error> {
    let Some((name, argument)) = annotation
        .split_once('(')
        .and_then(|(name, rest)| rest.strip_suffix(')').map(|argument| (name, argument)))
    else {
        return Ok(None);
    };
    let (name, capacity, written) = match name {
        "_Out_writes_bytes_all_" => ("_Out_writes_bytes_", argument, Some(argument)),
        "_Out_writes_bytes_all_opt_" => ("_Out_writes_bytes_opt_", argument, Some(argument)),
        "_Out_writes_bytes_to_" | "_Out_writes_bytes_to_opt_" => {
            let (capacity, written) = argument
                .split_once(',')
                .ok_or_else(|| Error(format!("invalid output byte relationship: {annotation}")))?;
            (
                if name.ends_with("_opt_") {
                    "_Out_writes_bytes_opt_"
                } else {
                    "_Out_writes_bytes_"
                },
                capacity,
                Some(written),
            )
        }
        _ => (name, argument, None),
    };
    let (direction, bytes, optional, output) = match name {
        "_In_reads_" => ("#[in] ", false, false, false),
        "_Out_writes_" => ("#[out] ", false, false, true),
        "_Inout_updates_" => ("#[in] #[out] ", false, false, true),
        "_In_reads_bytes_" => ("#[in] ", true, false, false),
        "_Out_writes_bytes_" => ("#[out] ", true, false, true),
        "_Inout_updates_bytes_" => ("#[in] #[out] ", true, false, true),
        "_In_reads_opt_" => ("#[in] ", false, true, false),
        "_Out_writes_opt_" => ("#[out] ", false, true, true),
        "_Inout_updates_opt_" => ("#[in] #[out] ", false, true, true),
        "_In_reads_bytes_opt_" => ("#[in] ", true, true, false),
        "_Out_writes_bytes_opt_" => ("#[out] ", true, true, true),
        "_Inout_updates_bytes_opt_" => ("#[in] #[out] ", true, true, true),
        _ => return Ok(None),
    };
    if sized {
        return Err(Error(
            "multiple buffer-length annotations are not supported".into(),
        ));
    }
    let (mutable, void) = match ty {
        ProjectedType::Pointer {
            mutable,
            depth,
            target,
        } => (
            *mutable,
            *depth == 1 && matches!(target.contract(), ProjectedType::Void),
        ),
        ProjectedType::PointerReference {
            mutable,
            string: Some(_),
            ..
        } => (*mutable, false),
        _ => return Err(Error(format!("{name} requires a buffer pointer"))),
    };
    if output && !mutable {
        return Err(Error(format!("{name} requires a writable buffer")));
    }
    if !bytes && void {
        return Err(Error(format!("{name} requires a non-void element type")));
    }
    let mut attributes = direction.to_string();
    if optional {
        attributes.push_str("#[opt] ");
    }
    if let Some(length) = length(capacity.trim(), bytes, parameters)? {
        attributes.push_str(&length);
    }
    if let Some(written) = written
        && let Some(written) = written_bytes(written.trim(), parameters, all_annotations)?
    {
        attributes.push_str(&written);
    }
    Ok(Some(attributes))
}

fn written_bytes(
    argument: &str,
    parameters: &[ProjectedType],
    annotations: &[Vec<String>],
) -> Result<Option<String>, Error> {
    let (argument, dereference) = argument
        .strip_prefix('*')
        .map_or((argument, false), |value| (value.trim(), true));
    let Some(index) = argument
        .strip_prefix('$')
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|value| value.parse::<i16>().ok())
    else {
        return Ok(None);
    };
    let ty = parameters
        .get(index as usize)
        .ok_or_else(|| Error("written byte count parameter is out of range".into()))?
        .contract();
    let scalar = if dereference {
        let ProjectedType::Pointer {
            mutable: true,
            depth: 1,
            target,
        } = ty
        else {
            return Err(Error(
                "written byte count requires a writable integer pointer".into(),
            ));
        };
        let count_annotations = &annotations[index as usize];
        let required = count_annotations
            .iter()
            .any(|value| matches!(value.as_str(), "_In_" | "_Out_" | "_Inout_"));
        let optional = count_annotations
            .iter()
            .any(|value| value.contains("_opt_"));
        let output = count_annotations.iter().any(|value| {
            matches!(
                value.as_str(),
                "_Out_" | "_Inout_" | "_Out_opt_" | "_Inout_opt_"
            )
        });
        if !output {
            return Err(Error(
                "written byte count requires an output parameter".into(),
            ));
        }
        if required && optional {
            return Err(Error("conflicting written byte count nullability".into()));
        }
        target.scalar_kind()
    } else {
        ty.scalar_kind()
    };
    if !matches!(
        scalar,
        Some("i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize")
    ) {
        return Err(Error("written byte count must be an integer".into()));
    }
    Ok(Some(format!(
        "#[written_bytes(BytesParamIndex = {index}, Dereference = {dereference})] "
    )))
}

fn length(
    argument: &str,
    bytes: bool,
    parameters: &[ProjectedType],
) -> Result<Option<String>, Error> {
    if let Some(index) = argument
        .strip_prefix('$')
        .filter(|value| value.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|value| value.parse::<i16>().ok())
    {
        let Some("i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize") =
            usize::try_from(index)
                .ok()
                .and_then(|index| parameters.get(index))
                .and_then(ProjectedType::scalar_kind)
        else {
            return Err(Error(format!(
                "buffer length `{argument}` must refer to an integer parameter"
            )));
        };
        let attribute = if bytes { "size_param" } else { "len_param" };
        return Ok(Some(format!("#[{attribute}({index})] ")));
    }
    if !bytes
        && argument.bytes().all(|byte| byte.is_ascii_digit())
        && (argument == "0" || !argument.starts_with('0'))
        && let Ok(value) = argument.parse::<i32>()
    {
        return Ok(Some(format!("#[len_const({value})] ")));
    }
    Ok(None)
}
