use super::*;

pub(super) fn resolve(
    snapshots: &[&Snapshot],
    types: &mut Vec<TypeCheck>,
) -> Result<Vec<ValueGroup>, Error> {
    let mut values: BTreeMap<(bool, String), Vec<ValueObservationRef>> = BTreeMap::new();
    for (snapshot, source) in snapshots.iter().enumerate() {
        for (value, declaration) in source.values.iter().enumerate() {
            for observation in 0..declaration.observations.len() {
                values
                    .entry((
                        declaration.observations[observation].kind.is_macro(),
                        declaration.name.clone(),
                    ))
                    .or_default()
                    .push(ValueObservationRef {
                        snapshot,
                        value: ValueId(value),
                        observation,
                    });
            }
        }
    }
    for observations in values.values_mut() {
        observations.sort_by_key(|reference| {
            let snapshot = snapshots[reference.snapshot];
            let observation =
                &snapshot.values[reference.value.0].observations[reference.observation];
            let location = &observation.location;
            (
                // A declaration must not hide disagreements between definitions.
                matches!(observation.kind, ValueKind::Constant { value: None, .. }),
                snapshot.input.as_str(),
                snapshot.files[location.file.0].as_str(),
                location.offset,
            )
        });
        let first = observations[0];
        let a = &snapshots[first.snapshot].values[first.value.0].observations[first.observation];
        let mut declaration_linkage = None;
        for &reference in observations.iter() {
            let b = &snapshots[reference.snapshot].values[reference.value.0].observations
                [reference.observation];
            value_attributes(snapshots, Owner::Value(reference), b)?;
            if let ValueKind::Constant {
                origin: ConstantOrigin::Declaration,
                linkage,
                ..
            } = &b.kind
            {
                if let Some((previous, expected)) = declaration_linkage {
                    if expected != linkage {
                        return Err(conflict(
                            snapshots,
                            Owner::Value(previous),
                            Owner::Value(reference),
                            "native constant linkages differ",
                        ));
                    }
                } else {
                    declaration_linkage = Some((reference, linkage));
                }
            }
            if !value_shape_equal(a, b, true) {
                return Err(conflict(
                    snapshots,
                    Owner::Value(first),
                    Owner::Value(reference),
                    "native value declarations differ",
                ));
            }
            types.push((
                (first.snapshot, a.canonical),
                (reference.snapshot, b.canonical),
                Owner::Value(first),
                Owner::Value(reference),
            ));
        }
    }
    let values = values
        .into_iter()
        .map(|((_, name), observations)| ValueGroup { name, observations })
        .collect();
    Ok(values)
}
