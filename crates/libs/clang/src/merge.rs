use super::*;

pub struct Variants {
    pub name: String,
    /// Target-local group indices. An absent target was not observed, not proven unavailable.
    pub groups: BTreeMap<Target, usize>,
}

pub struct Merged<'a> {
    pub targets: BTreeMap<Target, Resolution<'a>>,
    pub records: Vec<Variants>,
    pub aliases: Vec<Variants>,
    pub enums: Vec<Variants>,
    pub values: Vec<Variants>,
}

/// Reconciles each target, then associates native identities without equating their layouts.
///
/// Every variant retains its target's group and snapshot-local type edges. No field, layout,
/// or declaration is synthesized for a target that was not observed.
/// Alias variants associate names, not proven-equivalent canonical types.
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
    let mut records = BTreeMap::new();
    let mut aliases: BTreeMap<&str, BTreeMap<Target, usize>> = BTreeMap::new();
    let mut enums: BTreeMap<(&str, &str), BTreeMap<Target, usize>> = BTreeMap::new();
    let mut values: BTreeMap<(bool, &str), BTreeMap<Target, usize>> = BTreeMap::new();
    for (&target, resolution) in &targets {
        for (id, group) in resolution.groups.iter().enumerate() {
            let reference = group.records[0];
            let snapshot = resolution.snapshots[reference.snapshot];
            records
                .entry(snapshot.record_identity(reference.record))
                .or_insert_with(|| (snapshot.record_label(reference.record), BTreeMap::new()))
                .1
                .insert(target, id);
        }
        for (id, alias) in resolution.aliases.iter().enumerate() {
            aliases.entry(&alias.name).or_default().insert(target, id);
        }
        for (id, value) in resolution.values.iter().enumerate() {
            let first = value.observations[0];
            let observation = &resolution.snapshots[first.snapshot].values[first.value.0]
                .observations[first.observation];
            values
                .entry((observation.kind.is_macro(), &value.name))
                .or_default()
                .insert(target, id);
        }
        for (id, group) in resolution.enums.iter().enumerate() {
            let reference = group.enums[0];
            let enumeration =
                &resolution.snapshots[reference.snapshot].enums[reference.enumeration.0];
            enums
                .entry((&enumeration.name, &enumeration.usr))
                .or_default()
                .insert(target, id);
        }
    }
    let records = records
        .into_values()
        .map(|(name, groups)| Variants { name, groups })
        .collect();
    let aliases = variants(aliases.into_iter());
    let enums = variants(enums.into_iter().map(|((name, _), groups)| (name, groups)));
    let values = variants(values.into_iter().map(|((_, name), groups)| (name, groups)));
    Ok(Merged {
        targets,
        records,
        aliases,
        enums,
        values,
    })
}

fn variants<'a>(values: impl Iterator<Item = (&'a str, BTreeMap<Target, usize>)>) -> Vec<Variants> {
    values
        .map(|(name, groups)| Variants {
            name: name.to_string(),
            groups,
        })
        .collect()
}
