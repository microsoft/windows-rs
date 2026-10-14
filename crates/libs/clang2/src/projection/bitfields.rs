use super::*;

pub(super) fn groups<'a>(
    snapshot: &Snapshot,
    fields: &'a [Field],
    kind: RecordKind,
) -> Result<Vec<&'a [Field]>, Error> {
    for field in fields.iter().filter(|field| field.bit_width.is_some()) {
        let ty = &snapshot.types[field.canonical.0];
        if !matches!(&ty.kind, TypeKind::Scalar(scalar) if scalar.signed_integer().is_some()) {
            return Err(Error(format!(
                "bitfield `{}` requires an integer backing type",
                field.name
            )));
        }
        if ty
            .layout
            .is_none_or(|layout| u64::from(field.bit_width.unwrap()) > layout.size * 8)
        {
            return Err(Error(format!(
                "bitfield `{}` exceeds its backing type",
                field.name
            )));
        }
    }
    let mut groups = Vec::new();
    let mut index = 0;
    while index < fields.len() {
        let start = index;
        let field = &fields[start];
        index += 1;
        let Some(width) = field.bit_width else {
            groups.push(&fields[start..index]);
            continue;
        };
        if width == 0 {
            continue;
        }
        let ty = &snapshot.types[field.canonical.0];
        let mut backing = (!field.name.is_empty()).then_some(ty);
        let limit = field.offset_bits + ty.layout.unwrap().size * 8;
        let mut end = field.offset_bits + u64::from(width);
        while kind != RecordKind::Union && index < fields.len() {
            let next = &fields[index];
            let Some(width) = next.bit_width else {
                break;
            };
            if width == 0 || next.offset_bits >= limit {
                break;
            }
            let next_ty = &snapshot.types[next.canonical.0];
            let signed = |ty: &Type| match &ty.kind {
                TypeKind::Scalar(scalar) => scalar.signed_integer(),
                _ => None,
            };
            if ty.layout != next_ty.layout
                || (!next.name.is_empty()
                    && backing.is_some_and(|backing| signed(backing) != signed(next_ty)))
            {
                return Err(Error(format!(
                    "bitfield storage mixes backing types at `{}`",
                    next.name
                )));
            }
            if !next.name.is_empty() {
                backing = Some(next_ty);
            }
            if next.offset_bits != end || end + u64::from(width) > limit {
                return Err(Error(format!(
                    "bitfield `{}` cannot use contiguous metadata storage",
                    next.name
                )));
            }
            end += u64::from(width);
            index += 1;
        }
        groups.push(&fields[start..index]);
    }
    Ok(groups)
}
