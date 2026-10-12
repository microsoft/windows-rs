use super::*;

#[derive(Debug)]
pub struct NativeRecordError(String);

impl std::fmt::Display for NativeRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for NativeRecordError {}

fn descriptor<'a>(
    row: impl HasAttributes<'a>,
    name: &str,
) -> Result<Option<Attribute<'a>>, NativeRecordError> {
    let mut attributes = row
        .attributes()
        .filter(|attr| attr.namespace() == "Windows.Win32.Metadata" && attr.name() == name);
    let result = attributes.next();
    if attributes.next().is_some() {
        return Err(NativeRecordError(format!("duplicate {name}")));
    }
    Ok(result)
}

impl TypeDef<'_> {
    /// Returns the native record owning this named nested type or typedef.
    pub fn native_owner(&self) -> Result<Option<TypeName>, NativeRecordError> {
        let Some(attr) = descriptor(*self, "NativeTypeOwnerAttribute")? else {
            return Ok(None);
        };
        match attr.value().as_slice() {
            [(name, Value::TypeName(owner))] if name.is_empty() && owner.generics.is_empty() => {
                Ok(Some(owner.clone()))
            }
            _ => Err(NativeRecordError(
                "NativeTypeOwnerAttribute requires one record type".to_string(),
            )),
        }
    }
}

impl Field<'_> {
    /// Returns whether this field is an implicit unnamed native record member.
    pub fn is_native_anonymous(&self) -> Result<bool, NativeRecordError> {
        let Some(attr) = descriptor(*self, "NativeAnonymousFieldAttribute")? else {
            return Ok(false);
        };
        if !attr.value().is_empty() {
            return Err(NativeRecordError(
                "NativeAnonymousFieldAttribute takes no arguments".to_string(),
            ));
        }
        Ok(true)
    }
}

impl Index {
    /// Checks native record ownership and unnamed fields without inferring either from names.
    pub fn validate_native_records(
        &self,
        reference: Option<&Self>,
    ) -> Result<(), NativeRecordError> {
        for attr in self.all_attributes().filter(|attr| {
            attr.namespace() == "Windows.Win32.Metadata"
                && matches!(
                    attr.name(),
                    "NativeTypeOwnerAttribute" | "NativeAnonymousFieldAttribute"
                )
        }) {
            match (attr.name(), attr.parent()) {
                ("NativeTypeOwnerAttribute", HasAttribute::TypeDef(ty)) => {
                    if ty.category() != TypeCategory::Struct
                        || ty.flags().contains(TypeAttributes::WindowsRuntime)
                        || ty.generic_params().next().is_some()
                    {
                        return Err(NativeRecordError(
                            "native type owner requires a native record or typedef".to_string(),
                        ));
                    }
                    let owner = ty.native_owner()?.unwrap();
                    self.check_native_owner(
                        &owner,
                        ty.arches(),
                        reference,
                        &mut vec![ty.full_name()],
                    )?;
                }
                ("NativeAnonymousFieldAttribute", HasAttribute::Field(field)) => {
                    field.is_native_anonymous()?;
                    let parent: TypeDef<'_> = field.parent_row(4);
                    let Type::ValueName(name) = field.ty() else {
                        return Err(NativeRecordError(
                            "unnamed native field requires a direct nested record".to_string(),
                        ));
                    };
                    if parent.category() != TypeCategory::Struct
                        || parent.flags().contains(TypeAttributes::WindowsRuntime)
                        || field.flags().contains(FieldAttributes::Static)
                        || !self.nested(parent).any(|child| {
                            child.full_name() == name
                                && !child.is_opaque()
                                && child.category() == TypeCategory::Struct
                                && !child.flags().contains(TypeAttributes::WindowsRuntime)
                        })
                    {
                        return Err(NativeRecordError(
                            "unnamed native field requires storage owned by its enclosing record"
                                .to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(NativeRecordError(
                        "native record descriptor has an invalid owner".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn check_native_owner(
        &self,
        name: &TypeName,
        arches: i32,
        reference: Option<&Self>,
        visiting: &mut Vec<TypeName>,
    ) -> Result<(), NativeRecordError> {
        if visiting.contains(name) || visiting.len() >= 64 {
            return Err(NativeRecordError(
                "cyclic or excessive native record ownership".to_string(),
            ));
        }
        let definitions: Vec<_> = self
            .get(&name.namespace, &name.name)
            .chain(
                reference
                    .into_iter()
                    .flat_map(|index| index.get(&name.namespace, &name.name)),
            )
            .filter(|owner| arches == 0 || owner.arches() == 0 || arches & owner.arches() != 0)
            .collect();
        if definitions.is_empty() {
            return Err(NativeRecordError(
                "native record owner is unavailable".to_string(),
            ));
        }
        visiting.push(name.clone());
        for owner in definitions {
            if owner.category() != TypeCategory::Struct
                || owner.flags().contains(TypeAttributes::WindowsRuntime)
                || owner.generic_params().next().is_some()
                || owner.attributes().any(|attr| {
                    attr.namespace() == "Windows.Win32.Metadata"
                        && attr.name() == "NativeTypedefAttribute"
                })
            {
                return Err(NativeRecordError(
                    "native type owner must identify a record, not a typedef".to_string(),
                ));
            }
            if let Some(parent) = owner.native_owner()? {
                let selected = if arches == 0 {
                    owner.arches()
                } else if owner.arches() == 0 {
                    arches
                } else {
                    arches & owner.arches()
                };
                self.check_native_owner(&parent, selected, reference, visiting)?;
            }
        }
        visiting.pop();
        Ok(())
    }
}
