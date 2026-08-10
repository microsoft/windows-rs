use super::*;
use std::collections::{BTreeMap, BTreeSet};
use windows_metadata2::{Type, TypeResolution};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Policy {
    Derive,
    ExplicitLayout,
    FixedArray,
    TypedefArray,
    ScopedEnum,
}

pub(super) fn classify(
    database: &Database,
    definition: TypeDefinition<'_>,
    nested: &BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
) -> Result<Policy, Error> {
    if has_explicit_layout(database, definition.entity(), nested, &mut BTreeSet::new())? {
        return Ok(Policy::ExplicitLayout);
    }

    for field in definition.fields()? {
        if field.is_literal()? {
            continue;
        }
        let ty = field.signature()?;
        if matches!(ty.kind, TypeKind::Array { .. }) {
            return Ok(Policy::FixedArray);
        }
        for entity in definitions(database, field.entity().file(), &ty)? {
            let definition = database.definition(entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum if definition.has_attribute("ScopedEnumAttribute")? => {
                    return Ok(Policy::ScopedEnum);
                }
                TypeCategory::Struct
                    if resolves_to_fixed_array(database, entity, &mut BTreeSet::new())? =>
                {
                    return Ok(Policy::TypedefArray);
                }
                _ => {}
            }
        }
    }

    Ok(Policy::Derive)
}

fn has_explicit_layout(
    database: &Database,
    entity: Entity<TypeDef>,
    nested: &BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
    visiting: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<bool, Error> {
    if !visiting.insert(entity) {
        return Ok(false);
    }
    let definition = database.definition(entity).unwrap();
    let mut result = definition
        .type_attributes()?
        .contains(TypeAttributes::EXPLICIT_LAYOUT);

    if !result {
        for child in nested.get(&entity).into_iter().flatten() {
            if has_explicit_layout(database, *child, nested, visiting)? {
                result = true;
                break;
            }
        }
    }

    if !result {
        'fields: for field in definition.fields()? {
            if field.is_literal()? {
                continue;
            }
            let ty = field.signature()?;
            if let Some(field_type) = by_value_type(&ty) {
                for field_entity in definitions(database, field.entity().file(), field_type)? {
                    if database.definition(field_entity).unwrap().category()?
                        == TypeCategory::Struct
                        && has_explicit_layout(database, field_entity, nested, visiting)?
                    {
                        result = true;
                        break 'fields;
                    }
                }
            }
        }
    }

    visiting.remove(&entity);
    Ok(result)
}

fn resolves_to_fixed_array(
    database: &Database,
    entity: Entity<TypeDef>,
    visiting: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<bool, Error> {
    if !visiting.insert(entity) {
        return Ok(false);
    }
    let definition = database.definition(entity).unwrap();
    if !definition.has_attribute("NativeTypedefAttribute")? {
        visiting.remove(&entity);
        return Ok(false);
    }
    let mut fields = Vec::new();
    for field in definition.fields()? {
        if !field.is_literal()? {
            fields.push(field);
        }
    }
    let result = if let [field] = fields.as_slice() {
        let ty = field.signature()?;
        if matches!(ty.kind, TypeKind::Array { .. }) {
            true
        } else {
            let mut result = false;
            for field_entity in definitions(database, field.entity().file(), &ty)? {
                if database.definition(field_entity).unwrap().category()? == TypeCategory::Struct
                    && resolves_to_fixed_array(database, field_entity, visiting)?
                {
                    result = true;
                    break;
                }
            }
            result
        }
    } else {
        false
    };
    visiting.remove(&entity);
    Ok(result)
}

fn by_value_type(ty: &Type) -> Option<&Type> {
    match &ty.kind {
        TypeKind::Array { element, .. } => Some(element),
        TypeKind::Value(_) | TypeKind::Class(_) => Some(ty),
        _ => None,
    }
}

fn definitions(
    database: &Database,
    file: FileId,
    ty: &Type,
) -> Result<Vec<Entity<TypeDef>>, Error> {
    let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
        return Ok(Vec::new());
    };
    Ok(match database.resolve_type(file, *id)? {
        TypeResolution::Definition(entity) => vec![entity],
        TypeResolution::Candidates(candidates) => candidates.iter().collect(),
        TypeResolution::Specification(_) => Vec::new(),
    })
}
