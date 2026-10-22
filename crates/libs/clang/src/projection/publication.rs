use super::*;

pub(super) fn values<'a>(
    projector: &Projector<'a, '_>,
    target: Target,
    imports: &Imports,
    selected_aliases: Option<&BTreeMap<Target, BTreeSet<String>>>,
    declarations: &mut BTreeMap<PublicName, Declaration>,
    constants: &mut values::Constants<'a>,
    rejected: &mut Option<&mut partial::Issues>,
) -> Result<HashSet<&'a str>, Error> {
    let resolution = projector.resolution;
    let mut unpublished_values = HashSet::new();
    let native_values: HashMap<_, _> = resolution
        .values
        .iter()
        .filter_map(|value| {
            let first = value.observations[0];
            let observation = &resolution.snapshots[first.snapshot].values[first.value.0]
                .observations[first.observation];
            (!observation.kind.is_macro()).then_some((value.name.as_str(), &observation.kind))
        })
        .collect();
    let macro_constants: HashSet<_> = resolution
        .values
        .iter()
        .filter_map(|value| {
            let first = value.observations[0];
            let observation = &resolution.snapshots[first.snapshot].values[first.value.0]
                .observations[first.observation];
            matches!(
                observation.kind,
                ValueKind::Constant {
                    origin: ConstantOrigin::Macro,
                    value: Some(_),
                    ..
                }
            )
            .then_some(value.name.as_str())
        })
        .collect();
    for value in &resolution.values {
        let keys: Vec<_> = value
            .observations
            .iter()
            .map(|reference| {
                partial::Key::of(
                    resolution.snapshots[reference.snapshot],
                    DeclarationId::Value(reference.value),
                )
            })
            .collect();
        let first = value.observations[0];
        let snapshot = resolution.snapshots[first.snapshot];
        let observation = &snapshot.values[first.value.0].observations[first.observation];
        if matches!(observation.kind, ValueKind::Function { alias: Some(_), .. })
            && selected_aliases.is_some_and(|selected| {
                !selected
                    .get(&target)
                    .is_some_and(|names| names.contains(&value.name))
            })
        {
            if !native_values.contains_key(value.name.as_str()) {
                unpublished_values.insert(value.name.as_str());
            }
            continue;
        }
        if observation.kind.is_macro()
            && let Some(native) = native_values.get(value.name.as_str())
            && !matches!(
                (&observation.kind, native),
                (
                    ValueKind::Constant { .. },
                    ValueKind::Constant { value: None, .. }
                )
            )
        {
            let kind = match observation.kind {
                ValueKind::Function { .. } => "function alias",
                ValueKind::Constant { .. } => "constant macro",
            };
            reject(
                rejected,
                keys,
                Error(format!(
                    "{kind} `{}` collides with a native declaration",
                    value.name
                )),
            )?;
            continue;
        }
        let definition = match &observation.kind {
            ValueKind::Constant { value, .. } => value.as_ref(),
            ValueKind::Function { .. } => None,
        };
        let kind = match &observation.kind {
            ValueKind::Function { .. } => PublishedKind::Function,
            ValueKind::Constant { value: number, .. } => {
                if let Some(number) = number {
                    values::check_constant(
                        constants,
                        target,
                        &value.name,
                        &snapshot.types[observation.canonical.0],
                        number.clone(),
                    )?;
                }
                PublishedKind::Constant
            }
        };
        let mut headers = BTreeSet::new();
        let bodies = value
            .observations
            .iter()
            .map(|reference| {
                let snapshot = resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.values[reference.value.0].observations[reference.observation];
                projector.import(reference.snapshot, observation, &value.name, imports)
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|resolved| {
                let missing_import =
                    matches!(kind, PublishedKind::Function) && resolved.iter().any(Option::is_none);
                let mut bodies = value
                    .observations
                    .iter()
                    .zip(resolved)
                    .map(|(reference, import)| {
                        let snapshot = resolution.snapshots[reference.snapshot];
                        let observation =
                            &snapshot.values[reference.value.0].observations[reference.observation];
                        let location = match &observation.kind {
                            ValueKind::Function {
                                alias: Some(alias), ..
                            } => &alias.location,
                            _ => &observation.location,
                        };
                        headers.insert(snapshot.files[location.file.0].clone());
                        projector.value(
                            reference.snapshot,
                            observation,
                            &value.name,
                            if missing_import { None } else { import },
                            definition,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if matches!(kind, PublishedKind::Function) {
                    projector.agree_functions(value, target, &mut bodies)?;
                } else {
                    projector.agree_constants(value, target, definition, &mut bodies)?;
                }
                Ok((bodies, missing_import))
            });
        match bodies {
            Ok((_, true)) => {
                unpublished_values.insert(value.name.as_str());
                let error = format!(
                    "function `{}` has no configured DLL/export ownership",
                    value.name
                );
                if let Some(rejected) = rejected.as_deref_mut() {
                    for key in keys {
                        rejected
                            .messages
                            .insert(format!("{key:?}: {target:?}: projection: {error}"));
                    }
                } else {
                    return Err(Error(format!("{target:?}: {error}")));
                }
            }
            Ok((bodies, false)) => {
                publish(
                    declarations,
                    target,
                    &value.name,
                    bodies.into_iter().next().unwrap(),
                    headers,
                    keys,
                    kind,
                )?;
                unpublished_values.remove(value.name.as_str());
            }
            Err(error) => {
                unpublished_values.insert(value.name.as_str());
                if matches!(observation.kind, ValueKind::Constant { value: None, .. })
                    && macro_constants.contains(value.name.as_str())
                    && let Some(rejected) = rejected.as_deref_mut()
                {
                    // Keep the declaration as evidence, not as the macro's definition.
                    for key in keys {
                        rejected
                            .messages
                            .insert(format!("{key:?}: {target:?}: projection: {error}"));
                    }
                } else {
                    reject(rejected, keys, error)?;
                }
            }
        }
    }
    Ok(unpublished_values)
}

pub(super) fn uuid_tags(
    projector: &Projector<'_, '_>,
    target: Target,
    unpublished_values: &HashSet<&str>,
    declarations: &mut BTreeMap<PublicName, Declaration>,
    rejected: &mut Option<&mut partial::Issues>,
) -> Result<(), Error> {
    let resolution = projector.resolution;
    for group in &resolution.groups {
        let Some(uuid) = projector.uuid_tag(group) else {
            continue;
        };
        let name = resolution.name(group);
        let key = partial::Key::Tag(name.into());
        let mut headers = BTreeSet::new();
        let result = (|| {
            if unpublished_values.contains(name) {
                return Err(Error(format!(
                    "UUID tag `{name}` collides with an unpublishable native value"
                )));
            }
            let mut bodies = Vec::new();
            for reference in &group.records {
                let snapshot = resolution.snapshots[reference.snapshot];
                for id in &snapshot.records[reference.record.0].observations {
                    let observation = &snapshot.observations[id.0];
                    headers.insert(snapshot.files[observation.location.file.0].clone());
                    if observation.attributes.iter().any(|attribute| {
                        !matches!(
                            attribute.kind,
                            AttributeKind::Uuid(_)
                                | AttributeKind::Sal(_)
                                | AttributeKind::Deprecated(_)
                        )
                    }) {
                        return Err(Error(format!(
                            "unsupported UUID tag attributes on `{name}`"
                        )));
                    }
                    let name = ident(public_name(name))?;
                    let guid = values::write_guid(uuid);
                    bodies.push(Body {
                        attributes: annotations::write(&observation.attributes)?,
                        declaration: quote! { const #name: GUID = #guid; },
                    });
                }
            }
            agree(name, target, &bodies)?;
            Ok(bodies.into_iter().next().unwrap())
        })();
        match result {
            Ok(body) => publish(
                declarations,
                target,
                name,
                body,
                headers,
                vec![key],
                PublishedKind::Constant,
            )?,
            Err(error) => reject(rejected, [key], error)?,
        }
    }
    Ok(())
}

pub(super) fn records<'a>(
    projector: &Projector<'a, '_>,
    target: Target,
    typedef_attributes: &HashMap<&str, TokenStream>,
    published: &mut HashSet<&'a str>,
    declarations: &mut BTreeMap<PublicName, Declaration>,
    rejected: &mut Option<&mut partial::Issues>,
) -> Result<(), Error> {
    let resolution = projector.resolution;
    let plan = projector.plan;
    let external = projector.external;
    for (id, family) in plan.families.iter().enumerate() {
        if family.is_empty() || plan.owned[id] || external.records.contains_key(&id) {
            continue;
        }
        if projector.uuid_tag(&resolution.groups[family[0]]).is_some() {
            continue;
        }
        let name = &plan.names[id];
        if !published.insert(name.as_str()) {
            return Err(Error(format!("duplicate projected name `{name}`")));
        }
        let keys: Vec<_> = family
            .iter()
            .map(|group| {
                let reference = resolution.groups[*group].records[0];
                partial::Key::of(
                    resolution.snapshots[reference.snapshot],
                    DeclarationId::Record(reference.record),
                )
            })
            .collect();
        let mut bodies = Vec::new();
        for &group in family {
            for reference in &resolution.groups[group].records {
                let snapshot = resolution.snapshots[reference.snapshot];
                for &observation in &snapshot.records[reference.record.0].observations {
                    let observation = &snapshot.observations[observation.0];
                    if observation.definition.is_some()
                        || resolution.groups[group].definition.is_none()
                    {
                        bodies.push(projector.record(
                            reference.snapshot,
                            observation,
                            &ident(public_name(name))?,
                            false,
                        ));
                    }
                }
            }
        }
        let bodies = bodies.into_iter().collect::<Result<Vec<_>, _>>();
        if let Ok(bodies) = &bodies {
            agree(name, target, bodies)?;
        }
        let result = bodies.and_then(|bodies| {
            let mut body = bodies
                .into_iter()
                .next()
                .ok_or_else(|| Error(format!("record `{name}` is incomplete")))?;
            if let Some(attributes) = typedef_attributes.get(name.as_str()) {
                body.attributes.extend(attributes.clone());
            }
            Ok((body, plan.record_headers(resolution, id)?))
        });
        match result {
            Ok((body, header)) => {
                publish(
                    declarations,
                    target,
                    name,
                    body,
                    header,
                    keys,
                    if family.iter().any(|group| {
                        resolution.groups[*group]
                            .definition
                            .is_some_and(|reference| {
                                let observation = &resolution.snapshots[reference.snapshot]
                                    .observations[reference.observation.0];
                                projector.interface_record(reference.snapshot, observation.record)
                            })
                    }) {
                        PublishedKind::Interface
                    } else {
                        PublishedKind::Record
                    },
                )?;
            }
            Err(error) => reject(rejected, keys, error)?,
        }
    }
    Ok(())
}

pub(super) fn enums<'a>(
    projector: &Projector<'a, '_>,
    target: Target,
    typedef_attributes: &HashMap<&str, TokenStream>,
    published: &mut HashSet<&'a str>,
    constants: &mut values::Constants<'a>,
    declarations: &mut BTreeMap<PublicName, Declaration>,
    rejected: &mut Option<&mut partial::Issues>,
) -> Result<(), Error> {
    let resolution = projector.resolution;
    let plan = projector.plan;
    for (id, group) in resolution.enums.iter().enumerate() {
        if projector.external.enums.contains_key(&id) {
            continue;
        }
        let name = &plan.enum_names[id];
        if plan.enum_constants(resolution, id) {
            let first = group.enums[0];
            let key = partial::Key::Tag(
                resolution.snapshots[first.snapshot].enums[first.enumeration.0]
                    .name
                    .clone(),
            );
            let Some(reference) = group.definition else {
                reject(
                    rejected,
                    [key],
                    Error(format!("enum `{name}` is incomplete")),
                )?;
                continue;
            };
            let snapshot = resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.enums[reference.enumeration.0].observations[reference.observation];
            if !resolve::sal(&observation.attributes).is_empty()
                || !resolve::deprecations(&observation.attributes).is_empty()
            {
                reject(
                    rejected,
                    [key],
                    Error(format!(
                        "annotated anonymous enum `{name}` has no public annotation owner"
                    )),
                )?;
                continue;
            }
            let result = (|| {
                Ok((
                    projector.enum_backing(reference.snapshot, observation, name)?,
                    projector.enum_fields(observation, name)?,
                    plan.enum_headers(resolution, id)?,
                ))
            })();
            let (repr, fields, headers) = match result {
                Ok(value) => value,
                Err(error) => {
                    reject(rejected, [key], error)?;
                    continue;
                }
            };
            for (member, (attributes, member_name, value)) in
                observation.members.as_ref().unwrap().iter().zip(fields)
            {
                let member_identity = name.rsplit_once("::").map_or_else(
                    || member.name.clone(),
                    |(scope, _)| format!("{scope}::{}", member.name),
                );
                values::check_constant(
                    constants,
                    target,
                    member_identity.clone(),
                    &snapshot.types[observation.canonical.0],
                    ConstantValue::Integer(member.value),
                )?;
                publish(
                    declarations,
                    target,
                    &member_identity,
                    Body {
                        attributes,
                        declaration: quote! { const #member_name: #repr = #value; },
                    },
                    headers.clone(),
                    vec![key.clone()],
                    PublishedKind::Constant,
                )?;
            }
            continue;
        }
        if !published.insert(name.as_str()) {
            return Err(Error(format!("duplicate projected name `{name}`")));
        }
        let first = group.enums[0];
        let keys = vec![partial::Key::Tag(
            resolution.snapshots[first.snapshot].enums[first.enumeration.0]
                .name
                .clone(),
        )];
        let result = (|| {
            let reference = group
                .definition
                .ok_or_else(|| Error(format!("enum `{name}` is incomplete")))?;
            let observation = &resolution.snapshots[reference.snapshot].enums
                [reference.enumeration.0]
                .observations[reference.observation];
            let mut body = projector.enumeration(
                reference.snapshot,
                observation,
                &ident(public_name(name))?,
            )?;
            if let Some(attributes) = typedef_attributes.get(name.as_str()) {
                body.attributes.extend(attributes.clone());
            }
            Ok((body, plan.enum_headers(resolution, id)?))
        })();
        match result {
            Ok((body, header)) => publish(
                declarations,
                target,
                name,
                body,
                header,
                keys,
                PublishedKind::Enum,
            )?,
            Err(error) => reject(rejected, keys, error)?,
        }
    }
    Ok(())
}

pub(super) fn aliases<'a>(
    projector: &Projector<'a, '_>,
    target: Target,
    root_aliases: &HashSet<&str>,
    published: &mut HashSet<&'a str>,
    declarations: &mut BTreeMap<PublicName, Declaration>,
    rejected: &mut Option<&mut partial::Issues>,
) -> Result<(), Error> {
    let resolution = projector.resolution;
    let plan = projector.plan;
    let external = projector.external;
    for (id, alias) in resolution.aliases.iter().enumerate() {
        if external.aliases.contains_key(&alias.name) {
            continue;
        }
        if alias.observations.iter().any(|reference| {
            !resolve::alignments(
                &resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                    [reference.observation]
                    .attributes,
            )
            .is_empty()
        }) {
            reject(
                rejected,
                [partial::Key::Alias(alias.name.clone())],
                Error(format!(
                    "aligned typedef `{}` projection is unsupported",
                    alias.name
                )),
            )?;
            continue;
        }
        let first = alias_source(resolution, alias).unwrap_or(alias.observations[0]);
        let snapshot = resolution.snapshots[first.snapshot];
        let observation = &snapshot.aliases[first.alias.0].observations[first.observation];
        let canonical = observation.canonical;
        if let TypeKind::Record(record) = snapshot.types[canonical.0].kind
            && projector
                .uuid_tag(resolution.group(RecordRef {
                    snapshot: first.snapshot,
                    record,
                }))
                .is_some()
        {
            if alias.name != snapshot.records[record.0].name
                || alias.observations.iter().any(|reference| {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.aliases[reference.alias.0].observations[reference.observation];
                    !observation.attributes.is_empty()
                        || snapshot.types[observation.canonical.0].qualifiers
                            != Qualifiers::default()
                })
            {
                reject(
                    rejected,
                    [partial::Key::Alias(alias.name.clone())],
                    Error(format!(
                        "UUID tag alias `{}` has no projected type",
                        alias.name
                    )),
                )?;
            }
            continue;
        }
        if !publishable_alias(snapshot, observation)
            && alias.observations.iter().any(|reference| {
                let snapshot = resolution.snapshots[reference.snapshot];
                callback_contracts(
                    snapshot,
                    &snapshot.aliases[reference.alias.0].observations[reference.observation],
                )
            })
        {
            reject(
                rejected,
                [partial::Key::Alias(alias.name.clone())],
                Error(format!(
                    "annotated expanded typedef `{}` is unsupported",
                    alias.name
                )),
            )?;
            continue;
        }
        if observation.implicit
            || (!root_aliases.contains(alias.name.as_str())
                && !publishable_alias(snapshot, observation))
            || plan.aliases[id].is_some_and(|family| {
                plan.names[family] == alias.name && !external.records.contains_key(&family)
            })
            || plan.enum_aliases[id].is_some_and(|group| plan.enum_names[group] == alias.name)
        {
            continue;
        }
        if !published.insert(&alias.name) {
            return Err(Error(format!("duplicate projected name `{}`", alias.name)));
        }
        let keys = vec![partial::Key::Alias(alias.name.clone())];
        let attributes = annotations::covering(
            &alias.name,
            alias.observations.iter().map(|reference| {
                resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                    [reference.observation]
                    .attributes
                    .as_slice()
            }),
        )?;
        let bodies = alias
            .observations
            .iter()
            .filter(|reference| {
                !resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                    [reference.observation]
                    .implicit
            })
            .map(|reference| {
                let snapshot = resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.aliases[reference.alias.0].observations[reference.observation];
                projector.alias(reference.snapshot, observation, &alias.name, attributes)
            })
            .collect::<Result<Vec<_>, _>>();
        if let Ok(bodies) = &bodies {
            agree(&alias.name, target, bodies)?;
        }
        let result = bodies.and_then(|bodies| {
            Ok((
                bodies.into_iter().next().unwrap(),
                plan.alias_headers(resolution, alias)?,
            ))
        });
        match result {
            Ok((body, header)) => publish(
                declarations,
                target,
                &alias.name,
                body,
                header,
                keys,
                PublishedKind::Alias,
            )?,
            Err(error) => reject(rejected, keys, error)?,
        }
    }
    Ok(())
}

pub(super) fn output(
    declarations: BTreeMap<PublicName, Declaration>,
    modules: &[TokenStream],
    rejected: &mut Option<&mut partial::Issues>,
) -> Result<Output, Error> {
    let records = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Record)
        .count();
    let enums = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Enum)
        .count();
    let constants = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Constant)
        .count();
    let functions = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Function)
        .count();
    let interfaces = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Interface)
        .count();
    let aliases = declarations.len() - records - enums - constants - functions - interfaces;
    let mut items = BTreeMap::new();
    for (name, declaration) in declarations {
        if let Err(error) = insert(
            &mut items,
            &declaration.headers,
            &name,
            variants(declaration.bodies),
        ) {
            reject(rejected, declaration.keys, error)?;
        }
    }
    let mut partitions = BTreeMap::new();
    for (stem, (headers, declarations)) in items {
        let mut body: TokenStream = declarations.into_values().collect();
        for module in modules.iter().rev() {
            body = quote! { mod #module { #body } };
        }
        let text = quote! { #[win32] #body }.to_string();
        partitions.insert(
            stem,
            Partition {
                headers,
                rdl: windows_rdl::formatter::format(&text),
            },
        );
    }
    Ok(Output {
        partitions,
        records,
        enums,
        aliases,
        constants,
        functions,
        interfaces,
    })
}
