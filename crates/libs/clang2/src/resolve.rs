use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecordRef {
    pub snapshot: usize,
    pub record: RecordId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationRef {
    pub snapshot: usize,
    pub observation: ObservationId,
}

#[derive(Debug)]
pub struct Group {
    pub records: Vec<RecordRef>,
    pub definition: Option<ObservationRef>,
}

#[derive(Debug, Default)]
pub struct Work {
    pub definition_pairs: usize,
    pub type_pairs: usize,
}

pub struct Resolution<'a> {
    pub snapshots: Vec<&'a Snapshot>,
    pub groups: Vec<Group>,
    pub work: Work,
    record_groups: Vec<Vec<usize>>,
}

impl Resolution<'_> {
    pub fn group(&self, record: RecordRef) -> &Group {
        &self.groups[self.record_groups[record.snapshot][record.record.0]]
    }

    pub fn name(&self, group: &Group) -> &str {
        let reference = group.records[0];
        &self.snapshots[reference.snapshot].records[reference.record.0].name
    }

    pub fn definition(&self, group: &Group) -> Option<&RecordDefinition> {
        group.definition.map(|reference| {
            self.snapshots[reference.snapshot].observations[reference.observation.0]
                .definition
                .as_ref()
                .unwrap()
        })
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TypeRef {
    snapshot: usize,
    ty: TypeId,
}

/// Checks all observed definitions for one target without discarding original observations.
///
/// Correspondence is restricted to global named structs under matching compiler arguments.
/// Names and USRs select candidates; all candidate definitions must then agree.
pub fn resolve(snapshots: &[Snapshot]) -> Result<Resolution<'_>, Error> {
    resolve_snapshots(snapshots.iter().collect())
}

pub(super) fn resolve_snapshots(snapshots: Vec<&Snapshot>) -> Result<Resolution<'_>, Error> {
    let Some(first) = snapshots.first() else {
        return Err(Error("resolve requires snapshots".into()));
    };
    if snapshots.iter().any(|snapshot| {
        snapshot.target != first.target
            || snapshot.compiler != first.compiler
            || snapshot.arguments != first.arguments
    }) {
        return Err(Error(
            "resolve requires the same target, compiler, and arguments".into(),
        ));
    }
    let mut candidates = BTreeMap::new();
    for (snapshot, value) in snapshots.iter().enumerate() {
        for (id, record) in value.records.iter().enumerate() {
            candidates
                .entry((record.name.as_str(), record.usr.as_str()))
                .or_insert_with(Vec::new)
                .push(RecordRef {
                    snapshot,
                    record: RecordId(id),
                });
        }
    }
    let mut membership: Vec<Vec<usize>> = snapshots
        .iter()
        .map(|snapshot| vec![0; snapshot.records.len()])
        .collect();
    let mut groups = Vec::new();
    let mut work = Work::default();
    let mut pending = Vec::new();
    for records in candidates.into_values() {
        let mut definitions = Vec::new();
        for reference in &records {
            membership[reference.snapshot][reference.record.0] = groups.len();
            let snapshot = &snapshots[reference.snapshot];
            for &id in &snapshot.records[reference.record.0].observations {
                if snapshot.observations[id.0].definition.is_some() {
                    definitions.push(ObservationRef {
                        snapshot: reference.snapshot,
                        observation: id,
                    });
                }
            }
        }
        definitions.sort_by_key(|reference| {
            let snapshot = &snapshots[reference.snapshot];
            let location = &snapshot.observations[reference.observation.0].location;
            (
                snapshot.input.as_str(),
                snapshot.files[location.file.0].as_str(),
                location.offset,
            )
        });
        if let Some(&first) = definitions.first() {
            for &other in &definitions[1..] {
                pending.push((first, other));
            }
        }
        groups.push(Group {
            records,
            definition: definitions.first().copied(),
        });
    }
    let mut types = Vec::new();
    for (left, right) in pending {
        work.definition_pairs += 1;
        let a = snapshots[left.snapshot].observations[left.observation.0]
            .definition
            .as_ref()
            .unwrap();
        let b = snapshots[right.snapshot].observations[right.observation.0]
            .definition
            .as_ref()
            .unwrap();
        let conflict = |reason| conflict(&snapshots, left, right, reason);
        if a.layout != b.layout || a.fields.len() != b.fields.len() {
            return Err(conflict("record layout or field count differs"));
        }
        for (a, b) in a.fields.iter().zip(&b.fields) {
            if a.name != b.name || a.offset_bits != b.offset_bits {
                return Err(conflict("field name, order, or offset differs"));
            }
            for (a, b) in [(a.written, b.written), (a.canonical, b.canonical)] {
                types.push((
                    TypeRef {
                        snapshot: left.snapshot,
                        ty: a,
                    },
                    TypeRef {
                        snapshot: right.snapshot,
                        ty: b,
                    },
                    left,
                    right,
                ));
            }
        }
    }
    // Record edges refer to groups whose definitions all have independent obligations above.
    // No recursive assumption is published unless the entire worklist succeeds.
    let mut seen = HashSet::new();
    while let Some((left, right, owner_left, owner_right)) = types.pop() {
        let pair = if left <= right {
            (left, right)
        } else {
            (right, left)
        };
        if !seen.insert(pair) {
            continue;
        }
        work.type_pairs += 1;
        let a = &snapshots[left.snapshot].types[left.ty.0];
        let b = &snapshots[right.snapshot].types[right.ty.0];
        let conflict = || {
            conflict(
                &snapshots,
                owner_left,
                owner_right,
                "native field types differ",
            )
        };
        if a.qualifiers != b.qualifiers {
            return Err(conflict());
        }
        let child = match (&a.kind, &b.kind) {
            (TypeKind::Record(a), TypeKind::Record(b)) => {
                if membership[left.snapshot][a.0] != membership[right.snapshot][b.0] {
                    return Err(conflict());
                }
                continue;
            }
            (TypeKind::Void, TypeKind::Void) => None,
            (TypeKind::Scalar(a), TypeKind::Scalar(b)) if a == b => None,
            (TypeKind::Pointer(a), TypeKind::Pointer(b)) => Some((*a, *b)),
            (
                TypeKind::Array {
                    element: a,
                    length: x,
                },
                TypeKind::Array {
                    element: b,
                    length: y,
                },
            ) if x == y => Some((*a, *b)),
            _ => return Err(conflict()),
        };
        if a.layout != b.layout {
            return Err(conflict());
        }
        if let Some((a, b)) = child {
            types.push((
                TypeRef { ty: a, ..left },
                TypeRef { ty: b, ..right },
                owner_left,
                owner_right,
            ));
        }
    }
    Ok(Resolution {
        snapshots,
        groups,
        work,
        record_groups: membership,
    })
}

fn conflict(
    snapshots: &[&Snapshot],
    left: ObservationRef,
    right: ObservationRef,
    reason: &str,
) -> Error {
    let describe = |reference: ObservationRef| {
        let snapshot = &snapshots[reference.snapshot];
        let observation = &snapshot.observations[reference.observation.0];
        let location = &observation.location;
        format!(
            "{} [{}] {}:{}:{}",
            snapshot.records[observation.record.0].name,
            snapshot.input,
            snapshot.files[location.file.0],
            location.line,
            location.column
        )
    };
    Error(format!(
        "{}: {reason}: {} vs {}",
        snapshots[left.snapshot].target.triple(),
        describe(left),
        describe(right)
    ))
}
