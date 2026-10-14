use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Key {
    Tag(String),
    Alias(String),
}

impl Key {
    pub fn of(snapshot: &Snapshot, id: DeclarationId) -> Self {
        match id {
            DeclarationId::Record(id) => Self::Tag(snapshot.records[id.0].name.clone()),
            DeclarationId::Alias(id) => Self::Alias(snapshot.aliases[id.0].name.clone()),
            DeclarationId::Enum(id) => Self::Tag(snapshot.enums[id.0].name.clone()),
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

/// Removes failed entities and their dependents on every target, not just the failing target.
pub(crate) fn prune(snapshots: &mut [Snapshot], issues: &mut Issues) {
    let mut dependents: HashMap<Key, BTreeSet<Key>> = HashMap::new();
    for snapshot in snapshots.iter() {
        let types = type_dependencies(snapshot);
        let mut edge = |owner: Key, ty: TypeId| {
            if let Some(dependency) = types[ty.0] {
                dependents
                    .entry(Key::of(snapshot, dependency))
                    .or_default()
                    .insert(owner);
            }
        };
        for observation in &snapshot.observations {
            if let Some(definition) = &observation.definition {
                for field in &definition.fields {
                    let key = Key::of(snapshot, DeclarationId::Record(observation.record));
                    edge(key.clone(), field.written);
                    edge(key, field.canonical);
                }
            }
        }
        for alias in &snapshot.aliases {
            for observation in &alias.observations {
                edge(Key::Alias(alias.name.clone()), observation.underlying);
                edge(Key::Alias(alias.name.clone()), observation.canonical);
            }
        }
        for enumeration in &snapshot.enums {
            for observation in &enumeration.observations {
                edge(Key::Tag(enumeration.name.clone()), observation.underlying);
                edge(Key::Tag(enumeration.name.clone()), observation.canonical);
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
    for snapshot in snapshots {
        compact(snapshot, &issues.excluded);
    }
}

fn type_dependencies(snapshot: &Snapshot) -> Vec<Option<DeclarationId>> {
    let mut result = Vec::with_capacity(snapshot.types.len());
    for ty in &snapshot.types {
        result.push(match ty.kind {
            TypeKind::Record(id) => Some(DeclarationId::Record(id)),
            TypeKind::Alias(id) => Some(DeclarationId::Alias(id)),
            TypeKind::Enum(id) => Some(DeclarationId::Enum(id)),
            TypeKind::Pointer(id) | TypeKind::Array { element: id, .. } => result[id.0],
            TypeKind::Void | TypeKind::Scalar(_) => None,
        });
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

fn compact(snapshot: &mut Snapshot, excluded: &BTreeMap<Key, String>) {
    let records = remap(
        snapshot
            .records
            .iter()
            .map(|value| !excluded.contains_key(&Key::Tag(value.name.clone()))),
    );
    let aliases = remap(
        snapshot
            .aliases
            .iter()
            .map(|value| !excluded.contains_key(&Key::Alias(value.name.clone()))),
    );
    let enums = remap(
        snapshot
            .enums
            .iter()
            .map(|value| !excluded.contains_key(&Key::Tag(value.name.clone()))),
    );
    let declaration = |id| match id {
        DeclarationId::Record(id) => records[id.0].map(|id| DeclarationId::Record(RecordId(id))),
        DeclarationId::Alias(id) => aliases[id.0].map(|id| DeclarationId::Alias(AliasId(id))),
        DeclarationId::Enum(id) => enums[id.0].map(|id| DeclarationId::Enum(EnumId(id))),
    };
    let types = remap(
        type_dependencies(snapshot)
            .into_iter()
            .map(|dependency| dependency.is_none_or(|id| declaration(id).is_some())),
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
    retain(&mut snapshot.observations, &observations);
    retain(&mut snapshot.types, &types);
    for record in &mut snapshot.records {
        for id in &mut record.observations {
            id.0 = observations[id.0].unwrap();
        }
    }
    for alias in &mut snapshot.aliases {
        for observation in &mut alias.observations {
            observation.underlying.0 = types[observation.underlying.0].unwrap();
            observation.canonical.0 = types[observation.canonical.0].unwrap();
        }
    }
    for observation in &mut snapshot.observations {
        observation.record.0 = records[observation.record.0].unwrap();
        if let Some(definition) = &mut observation.definition {
            for field in &mut definition.fields {
                field.written.0 = types[field.written.0].unwrap();
                field.canonical.0 = types[field.canonical.0].unwrap();
            }
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
            TypeKind::Pointer(id) | TypeKind::Array { element: id, .. } => {
                id.0 = types[id.0].unwrap();
            }
            TypeKind::Void | TypeKind::Scalar(_) => {}
        }
    }
}
