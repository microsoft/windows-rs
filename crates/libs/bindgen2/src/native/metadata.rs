use super::*;
use proc_macro2::TokenStream;
use std::collections::BTreeSet;

pub(super) fn metadata_has_oversized_member(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
) -> Result<bool, Error> {
    metadata_type_has_oversized_member(database, file, ty, None, &mut BTreeSet::new())
}

pub(super) fn metadata_exceeds_retval_limit(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
) -> Result<bool, Error> {
    let layout = metadata_type_layout(database, file, ty, None, &mut BTreeSet::new())?;
    Ok(layout.0 > 16)
}

fn metadata_type_layout(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: Option<Entity<TypeDef>>,
    stack: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<(usize, usize), Error> {
    Ok(match &ty.kind {
        TypeKind::I8 | TypeKind::U8 => (1, 1),
        TypeKind::I16 | TypeKind::U16 => (2, 2),
        TypeKind::I64 | TypeKind::U64 | TypeKind::F64 => (8, 8),
        TypeKind::Array {
            element,
            rank,
            sizes,
            lower_bounds,
        } if *rank == 1 && sizes.len() == 1 && lower_bounds.iter().all(|bound| *bound == 0) => {
            let (size, align) = metadata_type_layout(database, file, element, owner, stack)?;
            (
                size.saturating_mul(sizes[0] as usize),
                align.saturating_mul(sizes[0] as usize).max(1),
            )
        }
        TypeKind::Value(id) => {
            let (namespace, name) =
                database
                    .type_name(file, *id)?
                    .ok_or_else(|| Error::InvalidType {
                        name: "retval".to_string(),
                        message: "native retval type has no name",
                    })?;
            let mut definitions = match database.resolve_type(file, *id)? {
                TypeResolution::Definition(definition) => vec![definition],
                TypeResolution::Candidates(candidates) => candidates.iter().collect(),
                TypeResolution::Specification(_) => Vec::new(),
            };
            if definitions.is_empty()
                && namespace.is_empty()
                && let Some(owner) = owner
            {
                definitions.extend(
                    database
                        .nested_types_of(owner)
                        .filter(|definition| {
                            definition.name().is_ok_and(|candidate| candidate == name)
                        })
                        .map(|definition| definition.entity()),
                );
            }
            let mut result = None::<(usize, usize)>;
            for entity in definitions {
                if !stack.insert(entity) {
                    continue;
                }
                let definition = database.definition(entity).unwrap();
                if definition.category()? == TypeCategory::Struct {
                    let explicit = definition
                        .type_attributes()?
                        .contains(TypeAttributes::EXPLICIT_LAYOUT);
                    let packing = definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .filter(|packing| *packing != 0)
                        .map(usize::from);
                    let mut layout = (0usize, 1usize);
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            continue;
                        }
                        let (field_size, mut field_align) = metadata_type_layout(
                            database,
                            field.entity().file(),
                            &field.signature()?,
                            Some(entity),
                            stack,
                        )?;
                        if let Some(packing) = packing {
                            field_align = field_align.min(packing);
                        }
                        if explicit {
                            layout.0 = layout.0.max(field_size);
                        } else {
                            layout.0 = align_up(layout.0, field_align);
                            layout.0 = layout.0.saturating_add(field_size);
                        }
                        layout.1 = layout.1.max(field_align);
                    }
                    result = Some(result.map_or(layout, |result| {
                        (result.0.max(layout.0), result.1.max(layout.1))
                    }));
                }
                stack.remove(&entity);
            }
            result.unwrap_or((4, 4))
        }
        _ => (4, 4),
    })
}

fn metadata_type_has_oversized_member(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: Option<Entity<TypeDef>>,
    stack: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<bool, Error> {
    match &ty.kind {
        TypeKind::Array {
            element,
            rank,
            sizes,
            lower_bounds,
        } if *rank == 1 && sizes.len() == 1 && lower_bounds.iter().all(|bound| *bound == 0) => {
            let element = Type::lower(database, file, "retval", (**element).clone())?;
            Ok(element
                .abi_layout(database, &mut BTreeSet::new())?
                .0
                .saturating_mul(sizes[0] as usize)
                > 16)
        }
        TypeKind::Value(id) => {
            let (namespace, name) =
                database
                    .type_name(file, *id)?
                    .ok_or_else(|| Error::InvalidType {
                        name: "retval".to_string(),
                        message: "native retval type has no name",
                    })?;
            let mut definitions = match database.resolve_type(file, *id)? {
                TypeResolution::Definition(definition) => vec![definition],
                TypeResolution::Candidates(candidates) => candidates.iter().collect(),
                TypeResolution::Specification(_) => Vec::new(),
            };
            if definitions.is_empty()
                && namespace.is_empty()
                && let Some(owner) = owner
            {
                definitions.extend(
                    database
                        .nested_types_of(owner)
                        .filter(|definition| {
                            definition.name().is_ok_and(|candidate| candidate == name)
                        })
                        .map(|definition| definition.entity()),
                );
            }
            for entity in definitions {
                if !stack.insert(entity) {
                    continue;
                }
                let definition = database.definition(entity).unwrap();
                if definition.category()? == TypeCategory::Struct {
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            continue;
                        }
                        if metadata_type_has_oversized_member(
                            database,
                            field.entity().file(),
                            &field.signature()?,
                            Some(entity),
                            stack,
                        )? {
                            stack.remove(&entity);
                            return Ok(true);
                        }
                    }
                }
                stack.remove(&entity);
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

pub(super) fn is_core_projection(namespace: &str, name: &str) -> bool {
    core_projection(namespace, name).is_some()
}

pub(super) fn named_traits(
    database: &Database,
    namespace: &str,
    name: &str,
    stack: &mut BTreeSet<(String, String)>,
) -> Result<TraitSupport, Error> {
    let key = (namespace.to_string(), name.to_string());
    if !stack.insert(key.clone()) {
        return Ok(TraitSupport::NONE);
    }
    let mut result = TraitSupport::ALL;
    let mut definitions = database.type_definitions(namespace, name).to_vec();
    if definitions.is_empty() {
        definitions = projected_nested_definitions(database, namespace, name);
    }
    if definitions.is_empty() {
        result = TraitSupport::NONE;
    }
    for entity in definitions {
        let definition = database.definition(entity).unwrap();
        let traits = match definition.category()? {
            TypeCategory::Enum => TraitSupport::ALL,
            TypeCategory::Delegate => TraitSupport {
                copy: true,
                debug: true,
                partial_eq: false,
                eq: false,
            },
            TypeCategory::Struct => {
                if definition
                    .type_attributes()?
                    .contains(TypeAttributes::EXPLICIT_LAYOUT)
                    || definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .is_some()
                {
                    TraitSupport {
                        copy: false,
                        ..TraitSupport::NONE
                    }
                } else {
                    let nested = database
                        .nested_types_of(entity)
                        .enumerate()
                        .map(|(index, definition)| {
                            Ok((definition.name()?.to_string(), format!("{name}_{index}")))
                        })
                        .collect::<Result<Vec<_>, Error>>()?;
                    let substitutions = nested
                        .iter()
                        .map(|(metadata, projected)| (metadata.as_str(), projected.as_str()))
                        .collect::<Vec<_>>();
                    let projected = nested
                        .iter()
                        .map(|(_, projected)| projected.as_str())
                        .collect::<BTreeSet<_>>();
                    let mut fields = TraitSupport::ALL;
                    for field in definition.fields()? {
                        if !field.is_literal()? {
                            let ty = Type::lower_with_nested(
                                database,
                                field.entity().file(),
                                name,
                                field.signature()?,
                                &substitutions,
                            )?
                            .qualify_projected_nested(namespace, &projected);
                            fields.combine(ty.projected_traits(database, stack)?);
                        }
                    }
                    fields
                }
            }
            _ => TraitSupport::NONE,
        };
        result.combine(traits);
    }
    stack.remove(&key);
    Ok(result)
}

pub(super) fn named_copyable(
    database: &Database,
    namespace: &str,
    name: &str,
    stack: &mut BTreeSet<(String, String)>,
) -> Result<bool, Error> {
    let key = (namespace.to_string(), name.to_string());
    if !stack.insert(key.clone()) {
        return Ok(false);
    }
    let mut definitions = database.type_definitions(namespace, name).to_vec();
    if definitions.is_empty() {
        definitions = projected_nested_definitions(database, namespace, name);
    }
    if definitions.is_empty() {
        stack.remove(&key);
        return Ok(false);
    }
    for entity in definitions {
        let definition = database.definition(entity).unwrap();
        let copyable = match definition.category()? {
            TypeCategory::Enum | TypeCategory::Delegate => true,
            TypeCategory::Struct => {
                let nested = database
                    .nested_types_of(entity)
                    .enumerate()
                    .map(|(index, definition)| {
                        Ok((definition.name()?.to_string(), format!("{name}_{index}")))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                let substitutions = nested
                    .iter()
                    .map(|(metadata, projected)| (metadata.as_str(), projected.as_str()))
                    .collect::<Vec<_>>();
                let projected = nested
                    .iter()
                    .map(|(_, projected)| projected.as_str())
                    .collect::<BTreeSet<_>>();
                let mut copyable = true;
                for field in definition.fields()? {
                    if !field.is_literal()? {
                        let ty = Type::lower_with_nested(
                            database,
                            field.entity().file(),
                            name,
                            field.signature()?,
                            &substitutions,
                        )?
                        .qualify_projected_nested(namespace, &projected);
                        if !ty.projected_copyable(database, stack)? {
                            copyable = false;
                            break;
                        }
                    }
                }
                copyable
            }
            _ => false,
        };
        if !copyable {
            stack.remove(&key);
            return Ok(false);
        }
    }
    stack.remove(&key);
    Ok(true)
}

pub(super) fn named_has_explicit_layout(
    database: &Database,
    namespace: &str,
    name: &str,
    stack: &mut BTreeSet<(String, String)>,
) -> Result<bool, Error> {
    let key = (namespace.to_string(), name.to_string());
    if !stack.insert(key.clone()) {
        return Ok(false);
    }
    let mut definitions = database.type_definitions(namespace, name).to_vec();
    if definitions.is_empty() {
        definitions = projected_nested_definitions(database, namespace, name);
    }
    for entity in definitions {
        let definition = database.definition(entity).unwrap();
        if definition
            .type_attributes()?
            .contains(TypeAttributes::EXPLICIT_LAYOUT)
        {
            stack.remove(&key);
            return Ok(true);
        }
        if definition.category()? != TypeCategory::Struct {
            continue;
        }
        let nested = database
            .nested_types_of(entity)
            .enumerate()
            .map(|(index, definition)| {
                Ok((definition.name()?.to_string(), format!("{name}_{index}")))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let substitutions = nested
            .iter()
            .map(|(metadata, projected)| (metadata.as_str(), projected.as_str()))
            .collect::<Vec<_>>();
        let projected = nested
            .iter()
            .map(|(_, projected)| projected.as_str())
            .collect::<BTreeSet<_>>();
        for field in definition.fields()? {
            if field.is_literal()? {
                continue;
            }
            let ty = Type::lower_with_nested(
                database,
                field.entity().file(),
                name,
                field.signature()?,
                &substitutions,
            )?
            .qualify_projected_nested(namespace, &projected);
            if ty.projected_has_explicit_layout(database, stack)? {
                stack.remove(&key);
                return Ok(true);
            }
        }
    }
    stack.remove(&key);
    Ok(false)
}

pub(super) fn projected_nested_definitions(
    database: &Database,
    namespace: &str,
    name: &str,
) -> Vec<Entity<TypeDef>> {
    let mut parent = name;
    let mut indices = Vec::new();
    while let Some((candidate, index)) = parent.rsplit_once('_') {
        let Ok(index) = index.parse::<usize>() else {
            break;
        };
        indices.push(index);
        parent = candidate;
        let roots = database.type_definitions(namespace, parent);
        if roots.is_empty() {
            continue;
        }
        let mut definitions = Vec::new();
        for root in roots {
            let mut current = *root;
            let mut found = true;
            for index in indices.iter().rev() {
                let Some(nested) = database.nested_types_of(current).nth(*index) else {
                    found = false;
                    break;
                };
                current = nested.entity();
            }
            if found {
                definitions.push(current);
            }
        }
        if !definitions.is_empty() {
            return definitions;
        }
    }
    Vec::new()
}

pub(super) fn core_projection(namespace: &str, name: &str) -> Option<TokenStream> {
    let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
    if !win32 {
        return None;
    }
    if let Some(canonical) = canonical::type_from_name(namespace, name)
        .or_else(|| canonical::native_core_from_name(namespace, name))
    {
        return Some(canonical.write());
    }
    None
}
