use super::*;

pub(super) fn groups<'a>(
    resolution: &'a Resolution<'_>,
) -> Result<HashMap<&'a str, &'a EnumAnnotation>, Error> {
    let mut groups = HashMap::new();
    let mut members = HashMap::new();
    for alias in &resolution.aliases {
        let Some(group) = alias.observations.iter().find_map(|reference| {
            resolve::enum_annotation(
                &resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                    [reference.observation]
                    .attributes,
            )
        }) else {
            continue;
        };
        groups.insert(alias.name.as_str(), group);
        for member in &group.members {
            let native = member
                .native
                .clone()
                .unwrap_or_else(|| member_name(&alias.name, &member.name));
            if members.insert(member.name.as_str(), native).is_some() {
                return Err(Error(format!(
                    "constant `{}` belongs to multiple enum groups",
                    member.name
                )));
            }
        }
    }
    for value in &resolution.values {
        if members
            .get(public_name(&value.name))
            .is_some_and(|name| *name != value.name)
        {
            return Err(Error(format!(
                "enum group member collides with native declaration `{}`",
                value.name
            )));
        }
    }
    for snapshot in &resolution.snapshots {
        for enumeration in &snapshot.enums {
            for observation in &enumeration.observations {
                if observation.scoped {
                    continue;
                }
                for member in observation.members.iter().flatten() {
                    if members.contains_key(member.name.as_str()) {
                        return Err(Error(format!(
                            "enum group member `{}` already belongs to a native enum",
                            member.name
                        )));
                    }
                }
            }
        }
    }
    Ok(groups)
}

pub(super) fn integer(value: EnumValue) -> i128 {
    match value {
        EnumValue::Signed(value) => i128::from(value),
        EnumValue::Unsigned(value) => i128::from(value),
    }
}

pub(super) fn member_name(group: &str, member: &str) -> String {
    group.rsplit_once("::").map_or_else(
        || member.into(),
        |(namespace, _)| format!("{namespace}::{member}"),
    )
}

impl Projector<'_, '_> {
    pub(super) fn logical_enum(
        &self,
        snapshot_id: usize,
        observation: &AliasObservation,
        name: &str,
        group: &EnumAnnotation,
        attributes: &[Attribute],
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let ty = &snapshot.types[observation.canonical.0];
        let TypeKind::Scalar(scalar) = &ty.kind else {
            return Err(Error(format!(
                "enum group `{name}` requires integer storage"
            )));
        };
        let signed = scalar
            .signed_integer()
            .ok_or_else(|| Error(format!("enum group `{name}` requires integer storage")))?;
        let bits = ty
            .layout
            .ok_or_else(|| Error(format!("enum group `{name}` has no storage layout")))?
            .size
            * 8;
        if !matches!(bits, 8 | 16 | 32 | 64) || ty.qualifiers != Qualifiers::default() {
            return Err(Error(format!(
                "unsupported enum group storage for `{name}`"
            )));
        }
        let (minimum, maximum) = if signed {
            (-(1_i128 << (bits - 1)), (1_i128 << (bits - 1)) - 1)
        } else {
            (0, (1_i128 << bits) - 1)
        };
        let fields = group.members.iter().map(|member| {
            let value = integer(member.value);
            if !(minimum..=maximum).contains(&value) {
                return Err(Error(format!("enum group `{name}` member `{}` does not fit its storage; use an explicit native conversion", member.name)));
            }
            let value = if signed {
                windows_metadata::Value::I64(value.try_into().unwrap())
            } else {
                windows_metadata::Value::U64(value.try_into().unwrap())
            };
            let value = write_value(self.namespace, &value);
            let attributes = if let Some(native) = &member.native {
                let value = self.resolution.values.iter().find(|value| &value.name == native).unwrap();
                let attributes = annotations::covering(native, value.observations.iter().map(|reference| {
                    self.resolution.snapshots[reference.snapshot].values[reference.value.0]
                        .observations[reference.observation].attributes.as_slice()
                }))?;
                annotations::write(attributes)?
            } else { TokenStream::new() };
            let member = ident(&member.name)?;
            Ok(quote! { #attributes #member = #value, })
        }).collect::<Result<Vec<_>, Error>>()?;
        let repr = write_type(
            self.namespace,
            &self.ty(snapshot_id, observation.canonical)?,
        );
        let name = ident(public_name(name))?;
        let flags = group.flags.then(|| quote! { #[flags] });
        let native = annotations::write(attributes)?;
        Ok(Body {
            abi: None,
            attributes: quote! { #native #[repr(#repr)] },
            declaration: quote! { #flags enum #name { #(#fields)* } },
        })
    }

    pub(super) fn associated_enum(
        &self,
        snapshot_id: usize,
        attributes: &[Attribute],
        mut subject: TypeId,
    ) -> Result<TokenStream, Error> {
        let Some(association) = resolve::associated_enum(attributes) else {
            return Ok(TokenStream::new());
        };
        let snapshot = self.resolution.snapshots[snapshot_id];
        loop {
            match snapshot.types[subject.0].kind {
                TypeKind::Alias(id) => {
                    if self
                        .enum_groups
                        .contains_key(snapshot.aliases[id.0].name.as_str())
                    {
                        return Err(Error(
                            "AssociatedEnum is redundant on an enum-group-typed use".into(),
                        ));
                    }
                    subject = snapshot.aliases[id.0].observations[0].underlying;
                }
                TypeKind::Pointer(inner)
                | TypeKind::Pointer64(inner)
                | TypeKind::LValueReference(inner)
                | TypeKind::Array { element: inner, .. } => subject = inner,
                TypeKind::Scalar(ref scalar) if scalar.signed_integer().is_some() => break,
                _ => return Err(Error(
                    "AssociatedEnum requires native integer storage, not an already enum-typed use"
                        .into(),
                )),
            }
        }
        let mut target = association.ty;
        let ty = loop {
            match snapshot.types[target.0].kind {
                TypeKind::Alias(id) => {
                    let alias = &snapshot.aliases[id.0];
                    if self.enum_groups.contains_key(alias.name.as_str()) {
                        if let Some(definition) = self.external.aliases.get(&alias.name) {
                            break references::metadata_type(*definition);
                        }
                        break MetadataType::value_named(self.namespace, &alias.name);
                    }
                    target = alias.observations[0].underlying;
                }
                TypeKind::Enum(id) => {
                    let group = self.resolution.enum_groups[snapshot_id][id.0];
                    if self.plan.enum_constants(self.resolution, group) {
                        return Err(Error("AssociatedEnum requires a named public enum".into()));
                    }
                    break self.ty(snapshot_id, target)?;
                }
                _ => {
                    return Err(Error(format!(
                        "AssociatedEnum target `{}` is not an enum or enum group",
                        association.name
                    )));
                }
            }
        };
        let ty = write_type(self.namespace, &ty);
        Ok(quote! { #[associated_enum(#ty)] })
    }
}
