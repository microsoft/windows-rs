use super::*;

pub struct RecordVariants {
    pub name: String,
    pub usr: String,
    /// Target-local group indices. An absent target was not observed, not proven unavailable.
    pub groups: BTreeMap<Target, usize>,
}

pub struct Merged<'a> {
    pub targets: BTreeMap<Target, Resolution<'a>>,
    pub records: Vec<RecordVariants>,
}

/// Reconciles each target, then associates native identities without equating their layouts.
///
/// Every variant retains its target's group and snapshot-local type edges. No field, layout,
/// or declaration is synthesized for a target that was not observed.
pub fn merge(snapshots: &[Snapshot]) -> Result<Merged<'_>, Error> {
    if snapshots.is_empty() {
        return Err(Error("merge requires snapshots".into()));
    }
    let mut targets: BTreeMap<Target, Vec<&Snapshot>> = BTreeMap::new();
    for snapshot in snapshots {
        targets.entry(snapshot.target).or_default().push(snapshot);
    }
    let targets = targets
        .into_iter()
        .map(|(target, snapshots)| Ok((target, resolve::resolve_snapshots(snapshots)?)))
        .collect::<Result<BTreeMap<_, _>, Error>>()?;
    let mut records: BTreeMap<(&str, &str), BTreeMap<Target, usize>> = BTreeMap::new();
    for (&target, resolution) in &targets {
        for (id, group) in resolution.groups.iter().enumerate() {
            let reference = group.records[0];
            let record = &resolution.snapshots[reference.snapshot].records[reference.record.0];
            records
                .entry((&record.name, &record.usr))
                .or_default()
                .insert(target, id);
        }
    }
    let records = records
        .into_iter()
        .map(|((name, usr), groups)| RecordVariants {
            name: name.to_string(),
            usr: usr.to_string(),
            groups,
        })
        .collect();
    Ok(Merged { targets, records })
}
