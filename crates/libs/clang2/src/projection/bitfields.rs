use super::*;

pub(super) fn backing(snapshot: &Snapshot, field: &Field) -> Result<TypeId, Error> {
    let id = match snapshot.types[field.canonical.0].kind {
        TypeKind::Enum(id) => snapshot.enums[id.0].observations[0].canonical,
        _ => field.canonical,
    };
    let ty = &snapshot.types[id.0];
    if !matches!(&ty.kind, TypeKind::Scalar(scalar)
        if *scalar == Scalar::Bool || scalar.signed_integer().is_some())
        || ty.layout != snapshot.types[field.canonical.0].layout
    {
        return Err(Error(format!(
            "bitfield `{}` requires an integer or boolean member type with matching layout",
            field.name
        )));
    }
    Ok(id)
}

pub(super) fn groups<'a>(
    snapshot: &Snapshot,
    fields: &'a [Field],
    kind: RecordKind,
) -> Result<Vec<&'a [Field]>, Error> {
    for field in fields.iter().filter(|field| field.bit_width.is_some()) {
        let ty = &snapshot.types[backing(snapshot, field)?.0];
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
            if ty.layout != next_ty.layout {
                return Err(Error(format!(
                    "bitfield storage mixes backing layouts at `{}`",
                    next.name
                )));
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

impl Projector<'_, '_> {
    pub(super) fn bitfield(
        &self,
        snapshot_id: usize,
        group: &[Field],
        backing: &str,
    ) -> Result<(TokenStream, TokenStream, TokenStream), Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let field = &group[0];
        let first = group
            .iter()
            .find(|member| !member.name.is_empty())
            .unwrap_or(field);
        let mut ty = self.ty(snapshot_id, bitfields::backing(snapshot, first)?)?;
        if ty == MetadataType::Bool {
            ty = MetadataType::U8;
        }
        let named = group
            .iter()
            .filter(|member| !member.name.is_empty())
            .map(|member| {
                Ok((
                    member,
                    self.written_ty(snapshot_id, member.written, member.canonical)?,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let typed = named
            .iter()
            .any(|(member, member_ty)| *member_ty != ty || member.bit_width == Some(1));
        let members = group
            .iter()
            .rposition(|member| !member.name.is_empty())
            .map(|last| {
                let mut named = named.iter();
                let members = group[..=last].iter().map(|member| {
                    let member_type = if member.name.is_empty() {
                        None
                    } else {
                        let (_, ty) = named.next().unwrap();
                        typed.then_some(ty)
                    };
                    windows_rdl::emit::write_bitfield_member(
                        self.namespace,
                        (!member.name.is_empty()).then_some(member.name.as_str()),
                        member.bit_width.unwrap(),
                        member_type,
                    )
                });
                quote! { { #(#members)* } }
            });
        Ok((
            ident(backing)?,
            write_type(self.namespace, &ty),
            quote! { #members },
        ))
    }
}
