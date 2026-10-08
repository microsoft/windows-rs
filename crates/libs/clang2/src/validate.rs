use super::*;

pub(super) fn validate(snapshot: &Snapshot) -> Result<Resolved<'_>, Error> {
    let mut groups: BTreeMap<&str, Vec<Id>> = BTreeMap::new();
    for (index, declaration) in snapshot.declarations.iter().enumerate() {
        available(declaration)?;
        groups
            .entry(&declaration.candidate)
            .or_default()
            .push(Id(index));
    }
    validate_groups(snapshot, groups)
}

pub(super) fn assess(snapshot: &Snapshot) -> Result<Assessment<'_>, Error> {
    let mut groups: BTreeMap<&str, Vec<Id>> = BTreeMap::new();
    let mut unavailable = BTreeMap::new();
    let mut dependents: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (index, declaration) in snapshot.declarations.iter().enumerate() {
        let group = declaration.candidate.as_str();
        groups.entry(group).or_default().push(Id(index));
        if let Err(error) = available(declaration) {
            unavailable
                .entry(group)
                .or_insert_with(|| error.to_string());
        }
        let mut pending = declaration.data.types();
        while let Some(ty) = pending.pop() {
            if let TypeKind::Named(id) = ty.kind {
                dependents
                    .entry(&snapshot.declarations[id.0].candidate)
                    .or_default()
                    .insert(group);
            }
            pending.extend(ty.children());
        }
    }
    let mut pending: VecDeque<_> = unavailable.keys().copied().collect();
    while let Some(group) = pending.pop_front() {
        for dependent in dependents.get(group).into_iter().flatten() {
            if !unavailable.contains_key(dependent) {
                unavailable.insert(*dependent, unavailable[group].clone());
                pending.push_back(*dependent);
            }
        }
    }
    let mut rejected = BTreeMap::new();
    for id in &snapshot.roots {
        let declaration = &snapshot.declarations[id.0];
        if let Some(reason) = unavailable.get(declaration.candidate.as_str()) {
            rejected.insert(
                declaration.name.clone(),
                format!("native closure for `{}`: {reason}", declaration.name),
            );
        }
    }
    let dependency_edges = dependents.values().map(BTreeSet::len).sum();
    let unavailable_groups = unavailable.len();
    let groups = groups
        .into_iter()
        .filter(|(group, _)| !unavailable.contains_key(group))
        .collect();
    let mut resolved = validate_groups(snapshot, groups)?;
    resolved
        .roots
        .retain(|name, _| !rejected.contains_key(*name));
    let resolved = (!resolved.roots.is_empty()).then_some(resolved);
    Ok(Assessment {
        resolved,
        rejected,
        dependency_edges,
        unavailable_groups,
    })
}

fn validate_groups<'a>(
    snapshot: &'a Snapshot,
    groups: BTreeMap<&str, Vec<Id>>,
) -> Result<Resolved<'a>, Error> {
    let mut completions: Vec<_> = (0..snapshot.declarations.len()).map(Id).collect();
    let mut incomplete = vec![];
    for candidates in groups.values() {
        if let Some(complete) = candidates
            .iter()
            .find(|id| snapshot.declarations[id.0].data.complete())
        {
            for id in candidates {
                if !snapshot.declarations[id.0].data.complete() {
                    completions[id.0] = *complete;
                }
            }
        } else {
            incomplete.push(snapshot.declarations[candidates[0].0].name.clone());
        }
    }
    incomplete.sort();
    let mut comparison = Comparison {
        snapshot,
        completions,
        completed: BTreeSet::new(),
        declaration_pairs: 0,
        type_pairs: 0,
    };
    for candidates in groups.values() {
        let mut uuid = None;
        for id in candidates {
            if let DeclarationData::Record {
                guid: Some(guid), ..
            } = &snapshot.declarations[id.0].data
            {
                if let Some((previous_id, previous)) = uuid
                    && previous != guid
                {
                    return Err(comparison.conflict(previous_id, *id, "interface UUIDs differ"));
                }
                uuid = Some((*id, guid));
            }
        }
        let anchor = candidates
            .iter()
            .max_by_key(|id| snapshot.declarations[id.0].data.evidence_rank())
            .copied()
            .unwrap_or(candidates[0]);
        for other in candidates {
            comparison.compare(anchor, *other)?;
        }
    }
    let report = Validation {
        declarations: snapshot
            .entities
            .iter()
            .filter(|entity| {
                entity
                    .observations
                    .iter()
                    .any(|id| groups.contains_key(snapshot.declarations[id.0].candidate.as_str()))
            })
            .count(),
        observations: groups.values().map(Vec::len).sum(),
        declaration_pairs: comparison.declaration_pairs,
        type_pairs: comparison.type_pairs,
        incomplete,
    };
    let mut representatives: Vec<_> = (0..snapshot.declarations.len()).map(Id).collect();
    let mut annotations = BTreeMap::new();
    let mut guids = BTreeMap::new();
    for candidates in groups.values() {
        let chosen = *candidates
            .iter()
            .max_by_key(|id| snapshot.declarations[id.0].data.evidence_rank())
            .unwrap();
        for id in candidates {
            representatives[id.0] = chosen;
            if let DeclarationData::Record {
                guid: Some(guid), ..
            } = &snapshot.declarations[id.0].data
            {
                guids.insert(chosen, guid.as_str());
            }
        }
        annotations.insert(chosen, resolve_annotations(snapshot, candidates)?);
    }
    let groups: BTreeMap<_, _> = groups
        .into_values()
        .map(|group| (representatives[group[0].0], group))
        .collect();
    let mut names: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for id in groups.keys() {
        names
            .entry(snapshot.declarations[id.0].name.as_str())
            .or_default()
            .push(*id);
    }
    let mut roots: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for id in &snapshot.roots {
        if !groups.contains_key(&representatives[id.0]) {
            continue;
        }
        roots
            .entry(snapshot.declarations[id.0].name.as_str())
            .or_default()
            .push(*id);
    }
    Ok(Resolved {
        snapshot,
        representatives,
        groups,
        names,
        roots,
        annotations,
        guids,
        report,
    })
}

fn available(declaration: &Declaration) -> Result<(), Error> {
    let reason = match &declaration.data {
        DeclarationData::Pending => unreachable!("capture left a pending declaration"),
        DeclarationData::Unavailable(reason) => Some(reason.clone()),
        DeclarationData::Record { unavailable, .. } if !unavailable.is_empty() => {
            Some(unavailable.join("; "))
        }
        DeclarationData::Variable {
            value: Value::Unavailable(reason),
            ..
        } => Some(reason.clone()),
        DeclarationData::Enum { variants, .. } => variants.iter().find_map(|(_, value)| {
            if let Value::Unavailable(reason) = value {
                Some(reason.clone())
            } else {
                None
            }
        }),
        _ => None,
    };
    if let Some(reason) = reason {
        return Err(unsupported(declaration, &reason));
    }
    let mut pending = declaration.data.types();
    while let Some(ty) = pending.pop() {
        if let TypeKind::Unavailable(reason) = &ty.kind {
            return Err(unsupported(declaration, reason));
        }
        pending.extend(ty.children());
    }
    Ok(())
}

fn unsupported(declaration: &Declaration, reason: &str) -> Error {
    Error(format!(
        "unsupported native evidence for `{}` at {}:{} (TU {}): {reason}",
        declaration.name, declaration.location.file, declaration.location.line, declaration.unit,
    ))
}

fn parameters_match(left: &[Parameter], right: &[Parameter]) -> bool {
    left.len() == right.len()
}

fn resolve_annotations(
    snapshot: &Snapshot,
    candidates: &[Id],
) -> Result<ResolvedAnnotations, Error> {
    let mut result = ResolvedAnnotations::default();
    let mut names: BTreeMap<usize, Vec<BTreeSet<String>>> = BTreeMap::new();
    for id in candidates {
        let declaration = &snapshot.declarations[id.0];
        merge_annotations(
            &mut result.own,
            &declaration.annotations,
            declaration,
            "declaration",
        )?;
        if let DeclarationData::Enum { annotations, .. } = &declaration.data {
            if result.fields.is_empty() {
                result
                    .fields
                    .resize_with(annotations.len(), SourceAnnotations::default);
            }
            for (index, annotations) in annotations.iter().enumerate() {
                merge_annotations(
                    &mut result.fields[index],
                    annotations,
                    declaration,
                    &format!("variant {index}"),
                )?;
            }
        }
        if let DeclarationData::Record {
            fields, methods, ..
        } = &declaration.data
        {
            if result.fields.is_empty() {
                result
                    .fields
                    .resize_with(fields.len(), SourceAnnotations::default);
            }
            if result.methods.is_empty() {
                result
                    .methods
                    .resize_with(methods.len(), SourceAnnotations::default);
            }
            for (index, field) in fields.iter().enumerate() {
                merge_annotations(
                    &mut result.fields[index],
                    &field.annotations,
                    declaration,
                    &format!("field {index}"),
                )?;
            }
            for (index, method) in methods.iter().enumerate() {
                merge_annotations(
                    &mut result.methods[index],
                    &method.annotations,
                    declaration,
                    &format!("method {index}"),
                )?;
            }
        }
        let callables: Vec<_> = match &declaration.data {
            DeclarationData::Function { parameters, .. }
            | DeclarationData::Alias { parameters, .. } => vec![(0, parameters)],
            DeclarationData::Record { methods, .. } => methods
                .iter()
                .enumerate()
                .map(|(index, method)| (index + 1, &method.parameters))
                .collect(),
            _ => vec![],
        };
        for (slot, parameters) in callables {
            let candidates = names
                .entry(slot)
                .or_insert_with(|| vec![BTreeSet::new(); parameters.len()]);
            let resolved = result.parameters.entry(slot).or_insert_with(|| {
                (0..parameters.len())
                    .map(|_| SourceAnnotations::default())
                    .collect()
            });
            if resolved.len() != parameters.len() {
                return Err(unsupported(
                    declaration,
                    "callable parameter counts disagree",
                ));
            }
            for (index, (evidence, parameter)) in resolved.iter_mut().zip(parameters).enumerate() {
                if valid_parameter_name(&parameter.name) {
                    candidates[index].insert(parameter.name.clone());
                }
                merge_annotations(
                    evidence,
                    &parameter.annotations,
                    declaration,
                    &format!("parameter {index}"),
                )?;
            }
        }
    }
    result.parameter_names = names
        .into_iter()
        .map(|(slot, candidates)| {
            let preferred: Vec<_> = candidates
                .into_iter()
                .map(|names| names.into_iter().next())
                .collect();
            let reserved: BTreeSet<_> = preferred.iter().flatten().cloned().collect();
            let mut used = BTreeSet::new();
            let names = preferred
                .into_iter()
                .enumerate()
                .map(|(index, name)| {
                    if let Some(name) = name
                        && used.insert(name.clone())
                    {
                        return name;
                    }
                    let mut name = format!("p{index}");
                    while reserved.contains(&name) || !used.insert(name.clone()) {
                        name.push('_');
                    }
                    name
                })
                .collect();
            (slot, names)
        })
        .collect();
    Ok(result)
}

fn valid_parameter_name(value: &str) -> bool {
    let mut chars = value.chars();
    !matches!(value, "_" | "self" | "Self" | "super" | "crate")
        && chars
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn merge_annotations(
    target: &mut SourceAnnotations,
    annotations: &[Annotation],
    declaration: &Declaration,
    scope: &str,
) -> Result<(), Error> {
    for (source, previous) in [
        (AnnotationSource::Sal, &mut target.sal),
        (AnnotationSource::Midl, &mut target.midl),
    ] {
        let current: Vec<_> = annotations
            .iter()
            .filter(|annotation| annotation.source == source)
            .map(Annotation::bound_text)
            .collect();
        if !previous.is_empty() && !current.is_empty() && *previous != current {
            return Err(unsupported(
                declaration,
                &format!("conflicting annotations for {scope} ({source:?})"),
            ));
        }
        if previous.is_empty() {
            *previous = current;
        }
    }
    Ok(())
}

fn exception_specification(ty: &Type) -> Option<i32> {
    if let TypeKind::Function {
        exception_specification,
        ..
    } = &ty.kind
    {
        Some(*exception_specification)
    } else {
        None
    }
}

struct Comparison<'a> {
    snapshot: &'a Snapshot,
    completions: Vec<Id>,
    completed: BTreeSet<(Id, Id)>,
    declaration_pairs: usize,
    type_pairs: usize,
}

enum Obligation<'a> {
    Declaration(Id, Id),
    Type(&'a Type, &'a Type, Id, Id),
}

impl<'a> Comparison<'a> {
    fn compare(&mut self, left: Id, right: Id) -> Result<(), Error> {
        let mut pending = vec![Obligation::Declaration(left, right)];
        let mut visited = BTreeSet::new();
        while let Some(obligation) = pending.pop() {
            match obligation {
                Obligation::Declaration(left, right) => {
                    let pair = if left < right {
                        (left, right)
                    } else {
                        (right, left)
                    };
                    if left == right || self.completed.contains(&pair) || !visited.insert(pair) {
                        continue;
                    }
                    self.declaration_pairs += 1;
                    let a = &self.snapshot.declarations[left.0];
                    let b = &self.snapshot.declarations[right.0];
                    if a.candidate != b.candidate {
                        return Err(self.conflict(left, right, "native identities differ"));
                    }
                    let mut types = vec![];
                    let matches = match (&a.data, &b.data) {
                        (
                            DeclarationData::Record {
                                kind: ak,
                                unnamed: aa,
                                complete: ac,
                                layout: al,
                                fields: af,
                                bases: ab,
                                methods: am,
                                guid: ag,
                                ..
                            },
                            DeclarationData::Record {
                                kind: bk,
                                unnamed: ba,
                                complete: bc,
                                layout: bl,
                                fields: bf,
                                bases: bb,
                                methods: bm,
                                guid: bg,
                                ..
                            },
                        ) => {
                            if *ac && *bc {
                                types.extend(af.iter().zip(bf).map(|(a, b)| (&a.ty, &b.ty)));
                                types.extend(ab.iter().zip(bb));
                                types.extend(
                                    am.iter().zip(bm).map(|(a, b)| (&a.canonical, &b.canonical)),
                                );
                                ak == bk
                                    && aa == ba
                                    && al == bl
                                    && ag == bg
                                    && af.len() == bf.len()
                                    && ab.len() == bb.len()
                                    && am.len() == bm.len()
                                    && am.iter().zip(bm).all(|(a, b)| {
                                        a.name == b.name
                                            && a.property == b.property
                                            && a.virtual_method == b.virtual_method
                                            && a.static_method == b.static_method
                                            && a.const_method == b.const_method
                                            && a.ref_qualifier == b.ref_qualifier
                                            && a.pure == b.pure
                                            && a.overrides == b.overrides
                                            && parameters_match(&a.parameters, &b.parameters)
                                            && exception_specification(&a.ty)
                                                == exception_specification(&b.ty)
                                    })
                                    && af.iter().zip(bf).all(|(a, b)| {
                                        a.name == b.name
                                            && a.offset == b.offset
                                            && a.bit_width == b.bit_width
                                    })
                            } else {
                                ak == bk
                            }
                        }
                        (
                            DeclarationData::Alias {
                                canonical: a,
                                parameters: ap,
                                ..
                            },
                            DeclarationData::Alias {
                                canonical: b,
                                parameters: bp,
                                ..
                            },
                        ) => {
                            types.push((a, b));
                            parameters_match(ap, bp)
                        }
                        (
                            DeclarationData::Enum {
                                complete: ac,
                                scoped: asc,
                                flags: af,
                                repr: ar,
                                variants: av,
                                ..
                            },
                            DeclarationData::Enum {
                                complete: bc,
                                scoped: bsc,
                                flags: bf,
                                repr: br,
                                variants: bv,
                                ..
                            },
                        ) => {
                            types.push((ar, br));
                            asc == bsc && af == bf && (!(*ac && *bc) || av == bv)
                        }
                        (
                            DeclarationData::Function {
                                ty: aw,
                                canonical: at,
                                parameters: ap,
                                link_name: al,
                                ..
                            },
                            DeclarationData::Function {
                                ty: bw,
                                canonical: bt,
                                parameters: bp,
                                link_name: bl,
                                ..
                            },
                        ) => {
                            types.push((at, bt));
                            al == bl
                                && parameters_match(ap, bp)
                                && exception_specification(aw) == exception_specification(bw)
                        }
                        (
                            DeclarationData::Variable {
                                canonical: at,
                                value: av,
                                ..
                            },
                            DeclarationData::Variable {
                                canonical: bt,
                                value: bv,
                                ..
                            },
                        ) => {
                            types.push((at, bt));
                            *av == Value::None || *bv == Value::None || av == bv
                        }
                        _ => false,
                    };
                    if !matches {
                        return Err(self.conflict(
                            left,
                            right,
                            "declaration shape or value differs",
                        ));
                    }
                    pending.extend(
                        types
                            .into_iter()
                            .map(|(a, b)| Obligation::Type(a, b, left, right)),
                    );
                }
                Obligation::Type(a, b, left, right) => {
                    self.type_pairs += 1;
                    if a.qualifiers != b.qualifiers {
                        return Err(self.conflict(left, right, "native type qualifiers differ"));
                    }
                    let matches = match (&a.kind, &b.kind) {
                        (
                            TypeKind::Builtin {
                                kind: ak,
                                layout: al,
                            },
                            TypeKind::Builtin {
                                kind: bk,
                                layout: bl,
                            },
                        ) => ak == bk && al == bl,
                        (TypeKind::Named(a), TypeKind::Named(b)) => {
                            pending.push(Obligation::Declaration(
                                self.completions[a.0],
                                self.completions[b.0],
                            ));
                            true
                        }
                        (TypeKind::Pointer(_), TypeKind::Pointer(_))
                        | (TypeKind::LValueReference(_), TypeKind::LValueReference(_))
                        | (TypeKind::RValueReference(_), TypeKind::RValueReference(_)) => true,
                        (TypeKind::Array { length: a, .. }, TypeKind::Array { length: b, .. }) => {
                            a == b
                        }
                        (
                            TypeKind::Function {
                                prototype: ap,
                                convention: ac,
                                exception_specification: ae,
                                variadic: av,
                                parameters: aa,
                                ..
                            },
                            TypeKind::Function {
                                prototype: bp,
                                convention: bc,
                                exception_specification: be,
                                variadic: bv,
                                parameters: ba,
                                ..
                            },
                        ) => ap == bp && ac == bc && ae == be && av == bv && aa.len() == ba.len(),
                        _ => false,
                    };
                    if !matches {
                        return Err(self.conflict(left, right, "native type shape differs"));
                    }

                    pending.extend(
                        a.children()
                            .into_iter()
                            .zip(b.children())
                            .map(|(a, b)| Obligation::Type(a, b, left, right)),
                    );
                }
            }
        }
        // Recursive assumptions become reusable only after every obligation has succeeded.
        self.completed.extend(visited);
        Ok(())
    }

    fn conflict(&self, left: Id, right: Id, reason: &str) -> Error {
        let left = &self.snapshot.declarations[left.0];
        let right = &self.snapshot.declarations[right.0];
        Error(format!(
            "conflicting native declarations `{}` at {}:{} (TU {}) and `{}` at {}:{} (TU {}): {reason}",
            left.name,
            left.location.file,
            left.location.line,
            left.unit,
            right.name,
            right.location.file,
            right.location.line,
            right.unit,
        ))
    }
}
