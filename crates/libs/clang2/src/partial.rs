use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Key {
    Tag(String),
    Alias(String),
    Value(String),
    /// Failed macro probes do not exclude native functions with the same spelling.
    Macro(String),
    Nested(Box<Self>, RecordMember),
}

impl Key {
    pub fn of(snapshot: &Snapshot, id: DeclarationId) -> Self {
        match id {
            DeclarationId::Record(id) => {
                let (root, path) = snapshot.record_path(id);
                path.into_iter().fold(
                    Self::Tag(snapshot.records[root.0].name.clone()),
                    |owner, member| Self::Nested(Box::new(owner), member),
                )
            }
            DeclarationId::Alias(id) => Self::Alias(snapshot.aliases[id.0].name.clone()),
            DeclarationId::Enum(id) => Self::Tag(snapshot.enums[id.0].name.clone()),
            DeclarationId::Value(id) => {
                let value = &snapshot.values[id.0];
                if value.observations.iter().any(|observation| {
                    matches!(
                        observation.kind,
                        ValueKind::Constant {
                            origin: ConstantOrigin::Macro,
                            ..
                        } | ValueKind::Function { alias: Some(_), .. }
                    )
                }) {
                    Self::Macro(value.name.clone())
                } else {
                    Self::Value(value.name.clone())
                }
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct Issues {
    pub excluded: BTreeMap<Key, String>,
    pub messages: BTreeSet<String>,
}

impl Issues {
    pub fn reject(&mut self, key: Key, reason: String) {
        self.excluded
            .entry(key)
            .and_modify(|current| {
                if reason < *current {
                    current.clone_from(&reason);
                }
            })
            .or_insert(reason);
    }

    pub fn finish(mut self) -> Vec<String> {
        self.messages.extend(
            self.excluded
                .into_iter()
                .map(|(key, reason)| format!("{key:?}: {reason}")),
        );
        self.messages.into_iter().collect()
    }

    pub fn extend(&mut self, other: Self) {
        self.messages.extend(other.messages);
        for (key, reason) in other.excluded {
            self.reject(key, reason);
        }
    }
}

/// Removes failures, their dependents, and dependencies unreachable from surviving roots.
pub(crate) fn prune(snapshots: &mut [Snapshot], issues: &mut Issues) {
    let mut dependents: HashMap<Key, BTreeSet<Key>> = HashMap::new();
    for snapshot in snapshots.iter() {
        for value in &snapshot.values {
            for observation in &value.observations {
                if let ValueKind::Function {
                    alias: Some(alias), ..
                } = &observation.kind
                {
                    dependents
                        .entry(Key::Value(alias.name.clone()))
                        .or_default()
                        .insert(Key::Macro(value.name.clone()));
                }
            }
        }
        for (id, record) in snapshot.records.iter().enumerate() {
            if let Some(owner) = record.owner {
                dependents
                    .entry(Key::of(snapshot, DeclarationId::Record(owner.record)))
                    .or_default()
                    .insert(Key::of(snapshot, DeclarationId::Record(RecordId(id))));
            }
        }
        for observation in &snapshot.observations {
            if let Some(definition) = &observation.definition {
                for &base in &definition.bases {
                    dependents
                        .entry(Key::of(snapshot, DeclarationId::Record(base.record)))
                        .or_default()
                        .insert(Key::of(snapshot, DeclarationId::Record(observation.record)));
                }
            }
        }
        let types = type_dependencies(snapshot);
        let mut edge = |owner: &Key, ty: TypeId| {
            for &dependency in &types[ty.0] {
                dependents
                    .entry(Key::of(snapshot, dependency))
                    .or_default()
                    .insert(owner.clone());
            }
        };
        for observation in &snapshot.observations {
            if let Some(definition) = &observation.definition {
                for field in &definition.fields {
                    let key = Key::of(snapshot, DeclarationId::Record(observation.record));
                    edge(&key, field.written);
                    edge(&key, field.canonical);
                }
            }
        }
        for alias in &snapshot.aliases {
            for observation in &alias.observations {
                edge(&Key::Alias(alias.name.clone()), observation.underlying);
                edge(&Key::Alias(alias.name.clone()), observation.canonical);
                for parameter in observation.parameters.iter().flatten() {
                    edge(&Key::Alias(alias.name.clone()), parameter.written);
                }
            }
        }
        for enumeration in &snapshot.enums {
            for observation in &enumeration.observations {
                edge(&Key::Tag(enumeration.name.clone()), observation.underlying);
                edge(&Key::Tag(enumeration.name.clone()), observation.canonical);
            }
        }
        let values = snapshot.values.iter().enumerate().flat_map(|(id, value)| {
            let key = Key::of(snapshot, DeclarationId::Value(ValueId(id)));
            value
                .observations
                .iter()
                .map(move |observation| (key.clone(), observation))
        });
        let methods = snapshot.observations.iter().flat_map(|observation| {
            observation.definition.iter().flat_map(move |definition| {
                definition.methods.iter().map(move |method| {
                    (
                        Key::of(snapshot, DeclarationId::Record(observation.record)),
                        &method.observation,
                    )
                })
            })
        });
        for (key, observation) in values.chain(methods) {
            edge(&key, observation.written);
            edge(&key, observation.canonical);
            if let ValueKind::Function {
                parameters,
                stdcall_candidate,
                ..
            } = &observation.kind
            {
                if let Some(candidate) = stdcall_candidate {
                    edge(&key, candidate.written);
                    edge(&key, candidate.canonical);
                }
                for parameter in parameters {
                    edge(&key, parameter.written);
                }
            }
        }
    }
    let mut pending: Vec<_> = issues.excluded.keys().cloned().collect();
    while let Some(key) = pending.pop() {
        for dependent in dependents.remove(&key).unwrap_or_default() {
            if !issues.excluded.contains_key(&dependent) {
                issues.reject(dependent.clone(), format!("depends on omitted {key:?}"));
                pending.push(dependent);
            }
        }
    }
    let mut dependencies: HashMap<Key, BTreeSet<Key>> = HashMap::new();
    for (dependency, owners) in dependents {
        for owner in owners {
            dependencies
                .entry(owner)
                .or_default()
                .insert(dependency.clone());
        }
    }
    let mut pending = Vec::new();
    // Share reachability across inputs and targets to retain every required native observation.
    for snapshot in snapshots.iter() {
        pending.extend(snapshot.roots.iter().map(|id| Key::of(snapshot, *id)));
        // Nested declarations belong to the public owner even without a field using them.
        for (id, record) in snapshot.records.iter().enumerate() {
            if let Some(owner) = record.owner {
                dependencies
                    .entry(Key::of(snapshot, DeclarationId::Record(owner.record)))
                    .or_default()
                    .insert(Key::of(snapshot, DeclarationId::Record(RecordId(id))));
            }
        }
    }
    let mut reachable = HashSet::new();
    while let Some(key) = pending.pop() {
        if !issues.excluded.contains_key(&key) && reachable.insert(key.clone()) {
            pending.extend(dependencies.remove(&key).unwrap_or_default());
        }
    }
    for snapshot in snapshots {
        compact(snapshot, &issues.excluded, &reachable);
    }
}

fn type_dependencies(snapshot: &Snapshot) -> Vec<Vec<DeclarationId>> {
    let mut result: Vec<Vec<DeclarationId>> = Vec::with_capacity(snapshot.types.len());
    for ty in &snapshot.types {
        let mut dependencies = match &ty.kind {
            TypeKind::Record(id) => vec![DeclarationId::Record(*id)],
            TypeKind::Alias(id) => vec![DeclarationId::Alias(*id)],
            TypeKind::Enum(id) => vec![DeclarationId::Enum(*id)],
            TypeKind::Pointer(id)
            | TypeKind::Pointer64(id)
            | TypeKind::LValueReference(id)
            | TypeKind::Array { element: id, .. } => result[id.0].clone(),
            TypeKind::Function(function) => std::iter::once(&function.result)
                .chain(&function.parameters)
                .flat_map(|id| result[id.0].iter().copied())
                .collect(),
            TypeKind::Attributed {
                modified,
                equivalent,
                ..
            } => result[modified.0]
                .iter()
                .chain(&result[equivalent.0])
                .copied()
                .collect(),
            TypeKind::Void | TypeKind::Scalar(_) => Vec::new(),
        };
        dependencies.sort_unstable();
        dependencies.dedup();
        result.push(dependencies);
    }
    result
}

fn remap(keep: impl Iterator<Item = bool>) -> Vec<Option<usize>> {
    let mut next = 0;
    keep.map(|keep| {
        keep.then(|| {
            let id = next;
            next += 1;
            id
        })
    })
    .collect()
}

fn retain<T>(values: &mut Vec<T>, map: &[Option<usize>]) {
    let mut index = 0;
    values.retain(|_| {
        let keep = map[index].is_some();
        index += 1;
        keep
    });
}

fn compact(snapshot: &mut Snapshot, excluded: &BTreeMap<Key, String>, reachable: &HashSet<Key>) {
    let records = remap(snapshot.records.iter().enumerate().map(|(id, _)| {
        let key = Key::of(snapshot, DeclarationId::Record(RecordId(id)));
        !excluded.contains_key(&key) && reachable.contains(&key)
    }));
    let aliases = remap(snapshot.aliases.iter().map(|value| {
        let key = Key::Alias(value.name.clone());
        !excluded.contains_key(&key) && reachable.contains(&key)
    }));
    let enums = remap(snapshot.enums.iter().map(|value| {
        let key = Key::Tag(value.name.clone());
        !excluded.contains_key(&key) && reachable.contains(&key)
    }));
    let values = remap(snapshot.values.iter().enumerate().map(|(id, value)| {
        reachable.contains(&Key::of(snapshot, DeclarationId::Value(ValueId(id))))
            && !excluded.contains_key(&Key::Value(value.name.clone()))
            && (!excluded.contains_key(&Key::Macro(value.name.clone()))
                || value.observations.iter().any(|observation| {
                    matches!(observation.kind, ValueKind::Function { alias: None, .. })
                }))
    }));
    let declaration = |id| match id {
        DeclarationId::Record(id) => records[id.0].map(|id| DeclarationId::Record(RecordId(id))),
        DeclarationId::Alias(id) => aliases[id.0].map(|id| DeclarationId::Alias(AliasId(id))),
        DeclarationId::Enum(id) => enums[id.0].map(|id| DeclarationId::Enum(EnumId(id))),
        DeclarationId::Value(id) => values[id.0].map(|id| DeclarationId::Value(ValueId(id))),
    };
    let types = remap(
        type_dependencies(snapshot)
            .into_iter()
            .map(|dependencies| dependencies.into_iter().all(|id| declaration(id).is_some())),
    );
    let observations = remap(
        snapshot
            .observations
            .iter()
            .map(|value| records[value.record.0].is_some()),
    );
    snapshot.roots = snapshot
        .roots
        .iter()
        .filter_map(|id| declaration(*id))
        .collect();
    retain(&mut snapshot.records, &records);
    retain(&mut snapshot.aliases, &aliases);
    retain(&mut snapshot.enums, &enums);
    retain(&mut snapshot.values, &values);
    retain(&mut snapshot.observations, &observations);
    retain(&mut snapshot.types, &types);
    for record in &mut snapshot.records {
        if let Some(owner) = &mut record.owner {
            owner.record.0 = records[owner.record.0].unwrap();
        }
        for id in &mut record.observations {
            id.0 = observations[id.0].unwrap();
        }
    }
    for alias in &mut snapshot.aliases {
        for observation in &mut alias.observations {
            observation.underlying.0 = types[observation.underlying.0].unwrap();
            observation.canonical.0 = types[observation.canonical.0].unwrap();
            for parameter in observation.parameters.iter_mut().flatten() {
                parameter.written.0 = types[parameter.written.0].unwrap();
            }
        }
    }
    for observation in &mut snapshot.observations {
        observation.record.0 = records[observation.record.0].unwrap();
        if let Some(definition) = &mut observation.definition {
            for base in &mut definition.bases {
                base.record.0 = records[base.record.0].unwrap();
            }
            for method in &mut definition.methods {
                remap_value(&mut method.observation, &types);
            }
            for field in &mut definition.fields {
                field.written.0 = types[field.written.0].unwrap();
                field.canonical.0 = types[field.canonical.0].unwrap();
            }
        }
    }
    for value in &mut snapshot.values {
        for observation in &mut value.observations {
            remap_value(observation, &types);
        }
    }
    for enumeration in &mut snapshot.enums {
        for observation in &mut enumeration.observations {
            observation.underlying.0 = types[observation.underlying.0].unwrap();
            observation.canonical.0 = types[observation.canonical.0].unwrap();
        }
    }
    for ty in &mut snapshot.types {
        match &mut ty.kind {
            TypeKind::Record(id) => id.0 = records[id.0].unwrap(),
            TypeKind::Alias(id) => id.0 = aliases[id.0].unwrap(),
            TypeKind::Enum(id) => id.0 = enums[id.0].unwrap(),
            TypeKind::Pointer(id)
            | TypeKind::Pointer64(id)
            | TypeKind::LValueReference(id)
            | TypeKind::Array { element: id, .. } => {
                id.0 = types[id.0].unwrap();
            }

            TypeKind::Function(function) => {
                function.result.0 = types[function.result.0].unwrap();
                for id in &mut function.parameters {
                    id.0 = types[id.0].unwrap();
                }
            }
            TypeKind::Attributed {
                modified,
                equivalent,
                ..
            } => {
                modified.0 = types[modified.0].unwrap();
                equivalent.0 = types[equivalent.0].unwrap();
            }
            TypeKind::Void | TypeKind::Scalar(_) => {}
        }
    }
}

fn remap_value(value: &mut ValueObservation, types: &[Option<usize>]) {
    value.written.0 = types[value.written.0].unwrap();
    value.canonical.0 = types[value.canonical.0].unwrap();
    if let ValueKind::Function {
        parameters,
        stdcall_candidate,
        ..
    } = &mut value.kind
    {
        if let Some(candidate) = stdcall_candidate {
            candidate.written.0 = types[candidate.written.0].unwrap();
            candidate.canonical.0 = types[candidate.canonical.0].unwrap();
        }
        for parameter in parameters {
            parameter.written.0 = types[parameter.written.0].unwrap();
        }
    }
}
