use super::*;

mod enums;
pub use enums::{EnumGroup, EnumObservationRef, EnumRef};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AliasObservationRef {
    pub snapshot: usize,
    pub alias: AliasId,
    pub observation: usize,
}

pub struct AliasGroup {
    pub name: String,
    /// Name-corresponding bindings, not a claim that their canonical types are identical.
    pub observations: Vec<AliasObservationRef>,
}

#[derive(Clone, Copy)]
enum Owner {
    Record(ObservationRef),
    Alias(AliasObservationRef),
    Enum(EnumObservationRef),
}

#[derive(Debug, Default)]
pub struct Work {
    pub definition_pairs: usize,
    pub type_pairs: usize,
    pub enum_pairs: usize,
}

pub struct Resolution<'a> {
    pub snapshots: Vec<&'a Snapshot>,
    pub groups: Vec<Group>,
    pub aliases: Vec<AliasGroup>,
    pub enums: Vec<EnumGroup>,
    pub work: Work,
    record_groups: Vec<Vec<usize>>,
    pub(crate) enum_groups: Vec<Vec<usize>>,
}

impl Resolution<'_> {
    pub fn enum_group(&self, reference: EnumRef) -> &EnumGroup {
        &self.enums[self.enum_groups[reference.snapshot][reference.enumeration.0]]
    }

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

type TypeRef = (usize, TypeId);
pub(crate) type TypePair = (TypeRef, TypeRef);

/// Checks all observed definitions for one target without discarding original observations.
///
/// Correspondence is restricted to global named records and enums under matching compiler arguments.
/// Names and USRs select candidates; all candidate definitions must then agree.
/// Alias groups preserve name-corresponding bindings for later public compatibility checks.
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
    for (id, snapshot) in snapshots.iter().enumerate() {
        for (observation, value) in snapshot.observations.iter().enumerate() {
            let owner = Owner::Record(ObservationRef {
                snapshot: id,
                observation: ObservationId(observation),
            });
            attributes(&snapshots, owner, &value.attributes)?;
            if let Some(definition) = &value.definition {
                for field in &definition.fields {
                    attributes(&snapshots, owner, &field.attributes)?;
                }
            }
        }
        for (alias, value) in snapshot.aliases.iter().enumerate() {
            for (observation, value) in value.observations.iter().enumerate() {
                attributes(
                    &snapshots,
                    Owner::Alias(AliasObservationRef {
                        snapshot: id,
                        alias: AliasId(alias),
                        observation,
                    }),
                    &value.attributes,
                )?;
            }
        }
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
        let first = records[0];
        let first = &snapshots[first.snapshot].records[first.record.0];
        if records.iter().any(|reference| {
            snapshots[reference.snapshot].records[reference.record.0].kind != first.kind
        }) {
            return Err(Error(format!(
                "native record kind differs for `{}`",
                first.name
            )));
        }
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
        let conflict = |reason| {
            conflict(
                &snapshots,
                Owner::Record(left),
                Owner::Record(right),
                reason,
            )
        };
        if a.layout != b.layout
            || a.fields.len() != b.fields.len()
            || packing(&snapshots[left.snapshot].observations[left.observation.0].attributes)
                != packing(&snapshots[right.snapshot].observations[right.observation.0].attributes)
        {
            return Err(conflict("record layout or field count differs"));
        }
        for (a, b) in a.fields.iter().zip(&b.fields) {
            if a.name != b.name
                || a.offset_bits != b.offset_bits
                || packing(&a.attributes) != packing(&b.attributes)
            {
                return Err(conflict("field name, order, or offset differs"));
            }
            // Typedef names are retained as evidence, but do not create nominal native types.
            let (a, b) = (a.canonical, b.canonical);
            types.push((
                (left.snapshot, a),
                (right.snapshot, b),
                Owner::Record(left),
                Owner::Record(right),
            ));
        }
    }
    let mut aliases: BTreeMap<&str, Vec<AliasObservationRef>> = BTreeMap::new();
    for (snapshot, value) in snapshots.iter().enumerate() {
        for (id, alias) in value.aliases.iter().enumerate() {
            // Clang's typedef USRs contain filenames; global typedef correspondence uses the name.
            let group = aliases.entry(&alias.name).or_default();
            for observation in 0..alias.observations.len() {
                group.push(AliasObservationRef {
                    snapshot,
                    alias: AliasId(id),
                    observation,
                });
            }
        }
    }
    let mut alias_groups = Vec::new();
    for (name, mut observations) in aliases {
        observations.sort_by_key(|reference| {
            let snapshot = snapshots[reference.snapshot];
            let location =
                &snapshot.aliases[reference.alias.0].observations[reference.observation].location;
            (
                snapshot.input.as_str(),
                snapshot.files[location.file.0].as_str(),
                location.offset,
            )
        });
        alias_groups.push(AliasGroup {
            name: name.to_string(),
            observations,
        });
    }
    // Record edges refer to groups whose definitions all have independent obligations above.
    // No recursive assumption is published unless the entire worklist succeeds.
    let (enums, enum_groups) = enums::resolve(&snapshots, &mut work)?;
    let mut seen = HashSet::new();
    while let Some((left, right, owner_left, owner_right)) = types.pop() {
        if !types_equal(&snapshots, left, right, &mut seen, &enum_groups, &|a, b| {
            membership[a.snapshot][a.record.0] == membership[b.snapshot][b.record.0]
        }) {
            return Err(conflict(
                &snapshots,
                owner_left,
                owner_right,
                "native field types differ",
            ));
        }
    }
    work.type_pairs = seen.len();
    Ok(Resolution {
        snapshots,
        groups,
        aliases: alias_groups,
        enums,
        enum_groups,
        work,
        record_groups: membership,
    })
}

/// Record relations supply independent definition checks; a failed comparison invalidates the pass.
pub(crate) fn types_equal(
    snapshots: &[&Snapshot],
    mut a: TypeRef,
    mut b: TypeRef,
    seen: &mut HashSet<TypePair>,
    enums: &[Vec<usize>],
    records: &impl Fn(RecordRef, RecordRef) -> bool,
) -> bool {
    loop {
        if !seen.insert(if a <= b { (a, b) } else { (b, a) }) {
            return true;
        }
        let x = &snapshots[a.0].types[a.1.0];
        let y = &snapshots[b.0].types[b.1.0];
        if x.qualifiers != y.qualifiers {
            return false;
        }
        if let (TypeKind::Record(x), TypeKind::Record(y)) = (&x.kind, &y.kind) {
            return records(
                RecordRef {
                    snapshot: a.0,
                    record: *x,
                },
                RecordRef {
                    snapshot: b.0,
                    record: *y,
                },
            );
        }
        if let (TypeKind::Enum(x), TypeKind::Enum(y)) = (&x.kind, &y.kind) {
            return enums[a.0][x.0] == enums[b.0][y.0];
        }
        if x.layout != y.layout {
            return false;
        }
        match (&x.kind, &y.kind) {
            (TypeKind::Function(x), TypeKind::Function(y)) => {
                return x.convention == y.convention
                    && x.parameters.len() == y.parameters.len()
                    && types_equal(
                        snapshots,
                        (a.0, x.result),
                        (b.0, y.result),
                        seen,
                        enums,
                        records,
                    )
                    && x.parameters.iter().zip(&y.parameters).all(|(x, y)| {
                        types_equal(snapshots, (a.0, *x), (b.0, *y), seen, enums, records)
                    });
            }
            (TypeKind::Void, TypeKind::Void) => return true,
            (TypeKind::Scalar(x), TypeKind::Scalar(y)) => return x == y,
            (TypeKind::Pointer(x), TypeKind::Pointer(y)) => {
                a.1 = *x;
                b.1 = *y;
            }
            (
                TypeKind::Array {
                    element: x,
                    length: n,
                },
                TypeKind::Array {
                    element: y,
                    length: m,
                },
            ) if n == m => {
                a.1 = *x;
                b.1 = *y;
            }
            _ => return false,
        }
    }
}

fn conflict(snapshots: &[&Snapshot], left: Owner, right: Owner, reason: &str) -> Error {
    Error(format!(
        "{reason}: {} vs {}",
        describe(snapshots, left),
        describe(snapshots, right)
    ))
}

fn describe(snapshots: &[&Snapshot], owner: Owner) -> String {
    let (snapshot, name, location) = match owner {
        Owner::Enum(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let enumeration = &snapshot.enums[reference.enumeration.0];
            (
                snapshot,
                &enumeration.name,
                &enumeration.observations[reference.observation].location,
            )
        }
        Owner::Record(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let observation = &snapshot.observations[reference.observation.0];
            (
                snapshot,
                &snapshot.records[observation.record.0].name,
                &observation.location,
            )
        }
        Owner::Alias(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let alias = &snapshot.aliases[reference.alias.0];
            (
                snapshot,
                &alias.name,
                &alias.observations[reference.observation].location,
            )
        }
    };
    format!(
        "{}: {name} [{}] {}:{}:{}",
        snapshot.target.triple(),
        snapshot.input,
        snapshot.files[location.file.0],
        location.line,
        location.column
    )
}

pub(crate) fn packing(attributes: &[Attribute]) -> (bool, Option<u32>) {
    let packed = attributes
        .iter()
        .any(|attribute| attribute.kind == AttributeKind::Packed);
    let maximum = attributes
        .iter()
        .find_map(|attribute| match attribute.kind {
            AttributeKind::MaxFieldAlignment { bits } => Some(bits),
            _ => None,
        });
    (packed, maximum)
}

fn attributes(
    snapshots: &[&Snapshot],
    owner: Owner,
    attributes: &[Attribute],
) -> Result<(), Error> {
    for attribute in attributes {
        if let AttributeKind::MaxFieldAlignment { bits } = attribute.kind
            && (!matches!(bits, 8 | 16 | 32 | 64 | 128) || packing(attributes).1 != Some(bits))
        {
            return Err(Error(format!(
                "{}: unsupported packing {bits} bits",
                describe(snapshots, owner)
            )));
        }
        if let AttributeKind::Unresolved(kind) = &attribute.kind {
            return Err(Error(format!(
                "{}: cannot verify {kind} attribute (spelling {:?}, tokens {:?})",
                describe(snapshots, owner),
                attribute.spelling,
                attribute.tokens
            )));
        }
    }
    Ok(())
}
