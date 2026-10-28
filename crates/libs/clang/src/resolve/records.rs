use super::*;

pub(super) fn groups(
    snapshots: &[&Snapshot],
    membership: &mut [Vec<usize>],
    pending: &mut Vec<(ObservationRef, ObservationRef)>,
) -> Result<Vec<Group>, Error> {
    let mut candidates = BTreeMap::new();
    for (snapshot, value) in snapshots.iter().enumerate() {
        for id in 0..value.records.len() {
            candidates
                .entry(value.record_identity(RecordId(id)))
                .or_insert_with(Vec::new)
                .push(RecordRef {
                    snapshot,
                    record: RecordId(id),
                });
        }
    }
    let mut groups = Vec::new();
    for records in candidates.into_values() {
        let first = records[0];
        let first = &snapshots[first.snapshot].records[first.record.0];
        if records.iter().any(|reference| {
            !snapshots[reference.snapshot].records[reference.record.0]
                .kind
                .compatible(first.kind)
        }) {
            return Err(Error(format!(
                "native record kind differs for `{}`",
                first.name
            )));
        }
        let mut definitions = Vec::new();
        if records.iter().any(|reference| {
            snapshots[reference.snapshot].records[reference.record.0].external != first.external
        }) {
            return Err(Error(format!(
                "native record ownership differs for `{}`",
                first.name
            )));
        }
        let mut identified: Option<(u128, ObservationRef)> = None;
        for reference in &records {
            membership[reference.snapshot][reference.record.0] = groups.len();
            let snapshot = &snapshots[reference.snapshot];
            for &id in &snapshot.records[reference.record.0].observations {
                if let Some(value) = uuid(&snapshot.observations[id.0].attributes) {
                    let current = ObservationRef {
                        snapshot: reference.snapshot,
                        observation: id,
                    };
                    if let Some((previous, owner)) = identified
                        && previous != value
                    {
                        return Err(conflict(
                            snapshots,
                            Owner::Record(owner),
                            Owner::Record(current),
                            "record UUID differs",
                        ));
                    }
                    identified = Some((value, current));
                }
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
            let attributes =
                &snapshots[first.snapshot].observations[first.observation.0].attributes;
            let required = alignments(attributes);
            let visible = default_type_visibility(attributes);
            for reference in &records {
                let snapshot = &snapshots[reference.snapshot];
                for &id in &snapshot.records[reference.record.0].observations {
                    let observation = &snapshot.observations[id.0];
                    if observation.definition.is_some() {
                        continue;
                    }
                    let reason = if alignments(&observation.attributes)
                        .iter()
                        .any(|request| !required.contains(request))
                    {
                        "forward alignment request differs"
                    } else if default_type_visibility(&observation.attributes) && !visible {
                        "forward type visibility request differs"
                    } else if no_trivial_auto_var_init(&observation.attributes)
                        && !no_trivial_auto_var_init(attributes)
                    {
                        "forward auto-initialization policy differs"
                    } else if uuid(&observation.attributes)
                        .is_some_and(|value| Some(value) != uuid(attributes))
                    {
                        "forward UUID differs"
                    } else {
                        continue;
                    };
                    return Err(conflict(
                        snapshots,
                        Owner::Record(ObservationRef {
                            snapshot: reference.snapshot,
                            observation: id,
                        }),
                        Owner::Record(first),
                        reason,
                    ));
                }
            }
            for &other in &definitions[1..] {
                pending.push((first, other));
            }
        }
        groups.push(Group {
            records,
            definition: definitions.first().copied(),
        });
    }
    Ok(groups)
}

pub(super) fn definitions(
    snapshots: &[&Snapshot],
    membership: &[Vec<usize>],
    pending: Vec<(ObservationRef, ObservationRef)>,
    work: &mut Work,
    types: &mut Vec<TypeCheck>,
) -> Result<(), Error> {
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
        let conflict =
            |reason| conflict(snapshots, Owner::Record(left), Owner::Record(right), reason);
        if a.semantics != b.semantics {
            return Err(conflict("record special-member semantics differ"));
        }
        if a.layout != b.layout
            || a.fields.len() != b.fields.len()
            || a.enums.len() != b.enums.len()
            || a.bases.len() != b.bases.len()
            || a.methods.len() != b.methods.len()
            || !attributes_equal(
                &snapshots[left.snapshot].observations[left.observation.0].attributes,
                &snapshots[right.snapshot].observations[right.observation.0].attributes,
            )
        {
            return Err(conflict("record layout or field count differs"));
        }
        for (&a, &b) in a.enums.iter().zip(&b.enums) {
            let a = &snapshots[left.snapshot].enums[a.0];
            let b = &snapshots[right.snapshot].enums[b.0];
            if a.name != b.name || a.access != b.access {
                return Err(conflict("record enum identities or access differ"));
            }
        }
        for (&a, &b) in a.bases.iter().zip(&b.bases) {
            if membership[left.snapshot][a.record.0] != membership[right.snapshot][b.record.0]
                || a.offset_bits != b.offset_bits
            {
                return Err(conflict("native base identities, order, or offsets differ"));
            }
        }
        for (a, b) in a.methods.iter().zip(&b.methods) {
            if a.pure != b.pure {
                return Err(conflict("native method purity differs"));
            }
            if a.name != b.name
                || a.slot != b.slot
                || a.overrides != b.overrides
                || midl_property(a) != midl_property(b)
                || !value_shape_equal(&a.observation, &b.observation, false)
            {
                return Err(conflict("native method names, slots, or attributes differ"));
            }
            types.push((
                (left.snapshot, a.observation.canonical),
                (right.snapshot, b.observation.canonical),
                Owner::Record(left),
                Owner::Record(right),
            ));
        }
        for (x, y) in a.fields.iter().zip(&b.fields) {
            if x.name != y.name
                || x.access != y.access
                || x.offset_bits != y.offset_bits
                || x.bit_width != y.bit_width
                || !field_attributes_equal(x, &a.fields, y, &b.fields)
            {
                return Err(conflict(
                    "field name, order, offset, bit width, or attributes differ",
                ));
            }
            // Typedef names are retained as evidence, but do not create nominal native types.
            let (a, b) = (x.canonical, y.canonical);
            types.push((
                (left.snapshot, a),
                (right.snapshot, b),
                Owner::Record(left),
                Owner::Record(right),
            ));
        }
    }
    Ok(())
}
