use super::*;
use crate::resolve::{TypePair, abi_attributes_equal, types_equal};

pub(super) struct Plan {
    pub families: Vec<Vec<usize>>,
    pub names: Vec<String>,
    pub aliases: Vec<Option<usize>>,
    pub enum_names: Vec<String>,
    pub enum_aliases: Vec<Option<usize>>,
    pub callbacks: Vec<HashMap<TypeId, String>>,
    enum_primary: Vec<Option<usize>>,
    enum_headers: Vec<BTreeSet<String>>,
    primary: Vec<Option<usize>>,
    groups: Vec<Vec<usize>>,
    parents: Vec<usize>,
    definition_headers: Vec<BTreeSet<String>>,
}

impl Plan {
    pub fn new(resolution: &Resolution<'_>) -> Result<Self, Error> {
        let count = resolution.groups.len();
        let mut groups: Vec<_> = resolution
            .snapshots
            .iter()
            .map(|snapshot| vec![0; snapshot.records.len()])
            .collect();
        let mut definition_headers = vec![BTreeSet::new(); count];
        for (id, group) in resolution.groups.iter().enumerate() {
            for reference in &group.records {
                groups[reference.snapshot][reference.record.0] = id;
                let snapshot = resolution.snapshots[reference.snapshot];
                for observation in &snapshot.records[reference.record.0].observations {
                    let observation = &snapshot.observations[observation.0];
                    if observation.definition.is_some() {
                        definition_headers[id]
                            .insert(snapshot.files[observation.location.file.0].clone());
                    }
                }
            }
        }
        let mut parents: Vec<_> = (0..count).collect();
        let mut sizes = vec![1; count];
        let mut aliases = Vec::new();
        for alias in &resolution.aliases {
            let records: Option<Vec<_>> = alias
                .observations
                .iter()
                .map(|reference| {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.aliases[reference.alias.0].observations[reference.observation];
                    let ty = &snapshot.types[observation.canonical.0];
                    if let TypeKind::Record(id) = ty.kind
                        && ty.qualifiers == Qualifiers::default()
                    {
                        Some(groups[reference.snapshot][id.0])
                    } else {
                        None
                    }
                })
                .collect();
            if let Some(records) = records {
                let first = records[0];
                for &other in &records[1..] {
                    let mut a = root(&mut parents, first);
                    let mut b = root(&mut parents, other);
                    if a != b {
                        if sizes[a] < sizes[b] {
                            std::mem::swap(&mut a, &mut b);
                        }
                        parents[b] = a;
                        sizes[a] += sizes[b];
                    }
                }
                aliases.push(Some(first));
            } else {
                aliases.push(None);
            }
        }
        for id in 0..count {
            parents[id] = root(&mut parents, id);
        }
        let mut families = vec![Vec::new(); count];
        for (id, &parent) in parents.iter().enumerate() {
            families[parent].push(id);
        }
        let mut names = vec![String::new(); count];
        let mut primary = vec![None; count];
        for (alias_id, (alias, family)) in resolution.aliases.iter().zip(&mut aliases).enumerate() {
            if let Some(id) = family {
                *id = parents[*id];
                if names[*id].is_empty() || alias.name < names[*id] {
                    names[*id].clone_from(&alias.name);
                    primary[*id] = Some(alias_id);
                }
            }
        }
        for (id, family) in families.iter().enumerate() {
            if let Some(&first) = family.first()
                && names[id].is_empty()
            {
                names[id] = resolution.name(&resolution.groups[first]).to_string();
            }
        }
        let mut enum_headers = vec![BTreeSet::new(); resolution.enums.len()];
        let mut enum_names = Vec::new();
        for (id, group) in resolution.enums.iter().enumerate() {
            let first = group.enums[0];
            enum_names.push(
                resolution.snapshots[first.snapshot].enums[first.enumeration.0]
                    .name
                    .clone(),
            );
            for reference in &group.enums {
                let snapshot = resolution.snapshots[reference.snapshot];
                for observation in &snapshot.enums[reference.enumeration.0].observations {
                    if observation.members.is_some() {
                        enum_headers[id]
                            .insert(snapshot.files[observation.location.file.0].clone());
                    }
                }
            }
        }
        let mut enum_primary: Vec<Option<usize>> = vec![None; resolution.enums.len()];
        let mut enum_aliases = vec![None; resolution.aliases.len()];
        for (alias_id, alias) in resolution.aliases.iter().enumerate() {
            let reference = alias.observations[0];
            let snapshot = resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.aliases[reference.alias.0].observations[reference.observation];
            let ty = &snapshot.types[observation.canonical.0];
            if let TypeKind::Enum(id) = ty.kind
                && ty.qualifiers == Qualifiers::default()
            {
                let group = resolution.enum_groups[reference.snapshot][id.0];
                enum_aliases[alias_id] = Some(group);
                if enum_primary[group].is_none() || alias.name < enum_names[group] {
                    enum_names[group].clone_from(&alias.name);
                    enum_primary[group] = Some(alias_id);
                }
            }
        }
        let mut plan = Self {
            families,
            names,
            aliases,
            primary,
            groups,
            parents,
            definition_headers,
            enum_names,
            enum_aliases,
            enum_primary,
            enum_headers,
            callbacks: Vec::new(),
        };
        plan.validate(resolution)?;
        plan.callbacks = plan.callback_names(resolution);
        Ok(plan)
    }

    fn callback_names(&self, resolution: &Resolution<'_>) -> Vec<HashMap<TypeId, String>> {
        let mut candidates = HashMap::<_, Vec<_>>::new();
        for alias in &resolution.aliases {
            let reference = alias.observations[0];
            let snapshot = resolution.snapshots[reference.snapshot];
            let id =
                snapshot.aliases[reference.alias.0].observations[reference.observation].canonical;
            if let Some(function) = callback(snapshot, id) {
                candidates
                    .entry((function.convention, function.parameters.len()))
                    .or_default()
                    .push(((reference.snapshot, id), &alias.name));
            }
        }
        let mut seen = HashSet::new();
        resolution
            .snapshots
            .iter()
            .enumerate()
            .map(|(snapshot_id, snapshot)| {
                let mut names = HashMap::new();
                for id in (0..snapshot.types.len()).map(TypeId) {
                    let Some(function) = callback(snapshot, id) else {
                        continue;
                    };
                    let Some(candidates) =
                        candidates.get(&(function.convention, function.parameters.len()))
                    else {
                        continue;
                    };
                    for &(candidate, name) in candidates {
                        // A failed comparison must not leave assumed pairs in the next attempt.
                        seen.clear();
                        if self.same_type(resolution, (snapshot_id, id), candidate, &mut seen) {
                            names.insert(id, name.clone());
                            break;
                        }
                    }
                }
                names
            })
            .collect()
    }

    pub fn family(&self, snapshot: usize, record: RecordId) -> usize {
        self.parents[self.groups[snapshot][record.0]]
    }

    pub fn alias_header(
        &self,
        resolution: &Resolution<'_>,
        alias: &AliasGroup,
    ) -> Result<String, Error> {
        let mut headers = BTreeSet::new();
        let mut definitions = BTreeSet::new();
        for reference in &alias.observations {
            let snapshot = resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.aliases[reference.alias.0].observations[reference.observation];
            let header = &snapshot.files[observation.location.file.0];
            headers.insert(header.clone());
            let mut ty = observation.canonical;
            loop {
                match snapshot.types[ty.0].kind {
                    TypeKind::Pointer(child) | TypeKind::Array { element: child, .. } => ty = child,
                    TypeKind::Record(record) => {
                        if self.definition_headers[self.groups[reference.snapshot][record.0]]
                            .contains(header)
                        {
                            definitions.insert(header.clone());
                        }
                        break;
                    }
                    TypeKind::Enum(id) => {
                        if self.enum_headers[resolution.enum_groups[reference.snapshot][id.0]]
                            .contains(header)
                        {
                            definitions.insert(header.clone());
                        }
                        break;
                    }
                    _ => break,
                }
            }
        }
        if definitions.len() == 1 {
            Ok(definitions.into_iter().next().unwrap())
        } else {
            owner(&alias.name, headers)
        }
    }

    pub fn record_header(
        &self,
        resolution: &Resolution<'_>,
        family: usize,
    ) -> Result<String, Error> {
        if let Some(alias) = self.primary[family] {
            return self.alias_header(resolution, &resolution.aliases[alias]);
        }
        owner(
            &self.names[family],
            self.families[family]
                .iter()
                .flat_map(|id| self.definition_headers[*id].iter().cloned())
                .collect(),
        )
    }

    pub fn enum_header(&self, resolution: &Resolution<'_>, id: usize) -> Result<String, Error> {
        if let Some(alias) = self.enum_primary[id] {
            self.alias_header(resolution, &resolution.aliases[alias])
        } else {
            owner(&self.enum_names[id], self.enum_headers[id].clone())
        }
    }

    fn validate(&self, resolution: &Resolution<'_>) -> Result<(), Error> {
        let mut seen = HashSet::new();
        for (id, family) in self.families.iter().enumerate() {
            let Some(&first) = family.first() else {
                continue;
            };
            for &other in &family[1..] {
                let definition = |group: usize| {
                    resolution.groups[group].definition.ok_or_else(|| Error(format!(
                        "cannot establish public record `{}` from distinct incomplete native records", self.names[id]
                    )))
                };
                let a = definition(first)?;
                let b = definition(other)?;
                let left = &resolution.snapshots[a.snapshot].observations[a.observation.0];
                let right = &resolution.snapshots[b.snapshot].observations[b.observation.0];
                let x = left.definition.as_ref().unwrap();
                let y = right.definition.as_ref().unwrap();
                if resolution.snapshots[a.snapshot].records[left.record.0].kind
                    != resolution.snapshots[b.snapshot].records[right.record.0].kind
                    || x.layout != y.layout
                    || x.fields.len() != y.fields.len()
                    || !abi_attributes_equal(&left.attributes, &right.attributes)
                    || !x.fields.iter().zip(&y.fields).all(|(x, y)| {
                        x.name == y.name
                            && x.offset_bits == y.offset_bits
                            && abi_attributes_equal(&x.attributes, &y.attributes)
                            && self.same_type(
                                resolution,
                                (a.snapshot, x.canonical),
                                (b.snapshot, y.canonical),
                                &mut seen,
                            )
                    })
                {
                    return Err(Error(format!(
                        "incompatible native definitions for public record `{}` on {:?}: {}:{} vs {}:{}",
                        self.names[id],
                        resolution.snapshots[0].target,
                        resolution.snapshots[a.snapshot].files[left.location.file.0],
                        left.location.line,
                        resolution.snapshots[b.snapshot].files[right.location.file.0],
                        right.location.line,
                    )));
                }
            }
        }
        for alias in &resolution.aliases {
            let first = alias.observations[0];
            let a = &resolution.snapshots[first.snapshot].aliases[first.alias.0].observations
                [first.observation];
            for other in &alias.observations[1..] {
                let b = &resolution.snapshots[other.snapshot].aliases[other.alias.0].observations
                    [other.observation];
                if !abi_attributes_equal(&a.attributes, &b.attributes)
                    || !self.same_type(
                        resolution,
                        (first.snapshot, a.canonical),
                        (other.snapshot, b.canonical),
                        &mut seen,
                    )
                {
                    return Err(Error(format!(
                        "incompatible native bindings for alias `{}` on {:?}: {}:{} vs {}:{}",
                        alias.name,
                        resolution.snapshots[0].target,
                        resolution.snapshots[first.snapshot].files[a.location.file.0],
                        a.location.line,
                        resolution.snapshots[other.snapshot].files[b.location.file.0],
                        b.location.line,
                    )));
                }
            }
        }
        Ok(())
    }

    fn same_type(
        &self,
        resolution: &Resolution<'_>,
        a: (usize, TypeId),
        b: (usize, TypeId),
        seen: &mut HashSet<TypePair>,
    ) -> bool {
        types_equal(
            &resolution.snapshots,
            a,
            b,
            seen,
            &resolution.enum_groups,
            &|a, b| self.family(a.snapshot, a.record) == self.family(b.snapshot, b.record),
        )
    }
}

fn root(parents: &mut [usize], mut id: usize) -> usize {
    while parents[id] != id {
        parents[id] = parents[parents[id]];
        id = parents[id];
    }
    id
}

fn owner(name: &str, headers: BTreeSet<String>) -> Result<String, Error> {
    if headers.len() == 1 {
        Ok(headers.into_iter().next().unwrap())
    } else {
        Err(Error(format!(
            "ambiguous header ownership for `{name}`: {headers:?}"
        )))
    }
}
