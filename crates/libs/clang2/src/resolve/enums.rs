use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumRef {
    pub snapshot: usize,
    pub enumeration: EnumId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumObservationRef {
    pub snapshot: usize,
    pub enumeration: EnumId,
    pub observation: usize,
}

pub struct EnumGroup {
    pub enums: Vec<EnumRef>,
    pub definition: Option<EnumObservationRef>,
}

pub(super) fn resolve(
    snapshots: &[&Snapshot],
    work: &mut Work,
) -> Result<(Vec<EnumGroup>, Vec<Vec<usize>>), Error> {
    let mut candidates = BTreeMap::new();
    for (snapshot, value) in snapshots.iter().enumerate() {
        for (id, enumeration) in value.enums.iter().enumerate() {
            candidates
                .entry((&enumeration.name, &enumeration.usr))
                .or_insert_with(Vec::new)
                .push(EnumRef {
                    snapshot,
                    enumeration: EnumId(id),
                });
        }
    }
    let mut membership: Vec<_> = snapshots
        .iter()
        .map(|value| vec![0; value.enums.len()])
        .collect();
    let mut groups = Vec::new();
    for enums in candidates.into_values() {
        let mut observations = Vec::new();
        for reference in &enums {
            membership[reference.snapshot][reference.enumeration.0] = groups.len();
            for observation in 0..snapshots[reference.snapshot].enums[reference.enumeration.0]
                .observations
                .len()
            {
                observations.push(EnumObservationRef {
                    snapshot: reference.snapshot,
                    enumeration: reference.enumeration,
                    observation,
                });
            }
        }
        observations.sort_by_key(|reference| {
            let snapshot = snapshots[reference.snapshot];
            let value =
                &snapshot.enums[reference.enumeration.0].observations[reference.observation];
            (
                snapshot.input.as_str(),
                snapshot.files[value.location.file.0].as_str(),
                value.location.offset,
            )
        });
        let first = observations[0];
        let a =
            &snapshots[first.snapshot].enums[first.enumeration.0].observations[first.observation];
        let mut definition: Option<EnumObservationRef> = None;
        for reference in observations {
            let snapshot = snapshots[reference.snapshot];
            let b = &snapshot.enums[reference.enumeration.0].observations[reference.observation];
            let owner = Owner::Enum(reference);
            attributes(snapshots, owner, &b.attributes)?;
            if reference != first {
                work.enum_pairs += 1;
                if a.scoped != b.scoped
                    || a.layout != b.layout
                    || packed(&a.attributes) != packed(&b.attributes)
                    || snapshots[first.snapshot].types[a.canonical.0]
                        != snapshot.types[b.canonical.0]
                {
                    return Err(conflict(
                        snapshots,
                        Owner::Enum(first),
                        owner,
                        "enum backing type, scope, attributes, or layout differs",
                    ));
                }
            }
            if let Some(members) = &b.members {
                for member in members {
                    attributes(snapshots, owner, &member.attributes)?;
                }
                if let Some(previous) = definition {
                    let previous_value = &snapshots[previous.snapshot].enums
                        [previous.enumeration.0]
                        .observations[previous.observation];
                    let old = previous_value.members.as_ref().unwrap();
                    if old.len() != members.len()
                        || old.iter().zip(members).any(|(a, b)| {
                            a.name != b.name
                                || a.value != b.value
                                || packed(&a.attributes) != packed(&b.attributes)
                        })
                    {
                        return Err(conflict(
                            snapshots,
                            Owner::Enum(previous),
                            owner,
                            "enum members differ",
                        ));
                    }
                } else {
                    definition = Some(reference);
                }
            }
        }
        groups.push(EnumGroup { enums, definition });
    }
    Ok((groups, membership))
}
