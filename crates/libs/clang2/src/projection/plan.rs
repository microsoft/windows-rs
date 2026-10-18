use super::*;

// MIDL supplies reserved, numbered tags for anonymous IDL enums.
fn midl_enum_name(name: &str) -> bool {
    name.starts_with("__MIDL_")
        && name.rsplit_once('_').is_some_and(|(_, suffix)| {
            suffix.len() >= 4 && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
}
use crate::resolve::{TypePair, abi_attributes_equal, attributes_equal, types_equal};

type CallbackNames = HashMap<(TypeId, Vec<CallingConvention>), String>;

pub(super) struct Plan {
    pub families: Vec<Vec<usize>>,
    pub names: Vec<String>,
    pub aliases: Vec<Option<usize>>,
    pub enum_names: Vec<String>,
    pub enum_aliases: Vec<Option<usize>>,
    pub callbacks: Vec<CallbackNames>,
    pub owned: Vec<bool>,
    pub children: Vec<Vec<Vec<RecordId>>>,
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
                        && snapshot.records[id.0].owner.is_none()
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
                    join(&mut parents, &mut sizes, first, other);
                }
                aliases.push(Some(first));
            } else {
                aliases.push(None);
            }
        }
        let mut owned_groups = BTreeMap::new();
        let mut owned = vec![false; count];
        let mut children = Vec::new();
        for (snapshot_id, snapshot) in resolution.snapshots.iter().enumerate() {
            let mut nested = vec![Vec::new(); snapshot.records.len()];
            for (id, record) in snapshot.records.iter().enumerate() {
                let Some(owner) = record.owner else { continue };
                nested[owner.record.0].push(RecordId(id));
                let group = groups[snapshot_id][id];
                owned[group] = true;
                let (owner, path) = snapshot.record_path(RecordId(id));
                let family = root(&mut parents, groups[snapshot_id][owner.0]);
                if let Some(other) = owned_groups.insert((family, path), group) {
                    join(&mut parents, &mut sizes, group, other);
                }
            }
            children.push(nested);
        }
        for id in 0..count {
            parents[id] = root(&mut parents, id);
        }
        let mut families = vec![Vec::new(); count];
        for (id, &parent) in parents.iter().enumerate() {
            families[parent].push(id);
        }
        let mut names = vec![String::new(); count];
        let primary = vec![None; count];
        for id in aliases.iter_mut().flatten() {
            *id = parents[*id];
        }
        for (id, family) in families.iter().enumerate() {
            if let Some(&first) = family.first()
                && names[id].is_empty()
            {
                let reference = resolution.groups[first].records[0];
                names[id] = resolution.snapshots[reference.snapshot].record_label(reference.record);
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
        let enum_primary: Vec<Option<usize>> = vec![None; resolution.enums.len()];
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
            owned,
            children,
        };
        plan.validate(resolution)?;
        plan.primary_names(resolution)?;
        for (id, family) in plan.families.iter().enumerate() {
            if !plan.owned[id] || family.is_empty() {
                continue;
            }
            let reference = resolution.groups[family[0]].records[0];
            let snapshot = resolution.snapshots[reference.snapshot];
            let (owner, path) = snapshot.record_path(reference.record);
            let mut name = plan.names[plan.family(reference.snapshot, owner)].clone();
            for member in path {
                let leaf = match member {
                    RecordMember::Named(name) => name,
                    RecordMember::Anonymous(field) => {
                        format!("{}_{field}", name.rsplit('/').next().unwrap())
                    }
                };
                name.push('/');
                name.push_str(&leaf);
            }
            plan.names[id] = name;
        }
        for (snapshot, children) in plan.children.iter_mut().enumerate() {
            for children in children {
                children.sort_by_key(|record| {
                    &plan.names[plan.parents[plan.groups[snapshot][record.0]]]
                });
            }
        }
        plan.callbacks = plan.callback_names(resolution)?;
        Ok(plan)
    }

    fn primary_names(&mut self, resolution: &Resolution<'_>) -> Result<(), Error> {
        let mut common = vec![Vec::new(); self.families.len()];
        let mut defining = vec![Vec::new(); self.families.len()];
        let mut enum_defining = vec![Vec::new(); self.enum_names.len()];
        for (id, alias) in resolution.aliases.iter().enumerate() {
            if alias_source(resolution, alias).is_none() {
                continue;
            }
            let defines = alias.observations.iter().any(|reference| {
                resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                    [reference.observation]
                    .defines_tag
            });
            if let Some(family) = self.aliases[id] {
                let bindings: BTreeSet<_> = alias
                    .observations
                    .iter()
                    .map(|reference| {
                        let snapshot = resolution.snapshots[reference.snapshot];
                        let observation = &snapshot.aliases[reference.alias.0].observations
                            [reference.observation];
                        let TypeKind::Record(record) = snapshot.types[observation.canonical.0].kind
                        else {
                            unreachable!()
                        };
                        self.groups[reference.snapshot][record.0]
                    })
                    .collect();
                if bindings.len() == self.families[family].len() {
                    common[family].push(id);
                    if defines {
                        defining[family].push(id);
                    }
                }
            } else if let Some(group) = self.enum_aliases[id]
                && defines
            {
                enum_defining[group].push(id);
            }
        }
        for (id, family) in self.families.iter().enumerate() {
            if family.is_empty() || self.owned[id] {
                continue;
            }
            let candidates = if defining[id].is_empty() && family.len() > 1 {
                &common[id]
            } else {
                &defining[id]
            };
            if candidates.len() != 1 && family.len() == 1 {
                continue;
            }
            let alias = primary_name(resolution, &self.names[id], candidates)?;
            self.primary[id] = Some(alias);
            self.names[id].clone_from(&resolution.aliases[alias].name);
        }
        for (id, candidates) in enum_defining.iter().enumerate() {
            let aliases;
            let candidates = if candidates.is_empty() && midl_enum_name(&self.enum_names[id]) {
                aliases = self
                    .enum_aliases
                    .iter()
                    .enumerate()
                    .filter_map(|(alias, group)| (*group == Some(id)).then_some(alias))
                    .collect::<Vec<_>>();
                &aliases
            } else {
                candidates
            };
            if let [alias] = candidates.as_slice() {
                self.enum_primary[id] = Some(*alias);
                self.enum_names[id].clone_from(&resolution.aliases[*alias].name);
            } else if !candidates.is_empty() && midl_enum_name(&self.enum_names[id]) {
                return Err(Error(format!(
                    "MIDL enum `{}` has multiple authored typedef names",
                    self.enum_names[id]
                )));
            }
        }
        Ok(())
    }

    fn callback_names(&self, resolution: &Resolution<'_>) -> Result<Vec<CallbackNames>, Error> {
        let mut candidates = HashMap::<_, Vec<_>>::new();
        for alias in &resolution.aliases {
            if alias.observations.iter().any(|reference| {
                let snapshot = resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.aliases[reference.alias.0].observations[reference.observation];
                observation.implicit
                    || observation.parameters.is_none()
                    || callback_contracts(snapshot, observation)
            }) {
                continue;
            }
            let reference = alias.observations[0];
            let snapshot = resolution.snapshots[reference.snapshot];
            let id =
                snapshot.aliases[reference.alias.0].observations[reference.observation].canonical;
            if let Some(function) = callback(snapshot, id) {
                let conventions = written_function(
                    snapshot,
                    snapshot.aliases[reference.alias.0].observations[reference.observation]
                        .underlying,
                )?
                .conventions(snapshot)?;
                candidates
                    .entry((function.convention, function.parameters.len()))
                    .or_default()
                    .push(((reference.snapshot, id), &alias.name, conventions));
            }
        }
        let mut seen = HashSet::new();
        Ok(resolution
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
                    for (candidate, name, conventions) in candidates {
                        // A failed comparison must not leave assumed pairs in the next attempt.
                        seen.clear();
                        if names.contains_key(&(id, conventions.clone())) {
                            continue;
                        }
                        if self.same_type(resolution, (snapshot_id, id), *candidate, &mut seen) {
                            names.insert((id, conventions.clone()), (*name).clone());
                        }
                    }
                }
                names
            })
            .collect())
    }

    pub fn family(&self, snapshot: usize, record: RecordId) -> usize {
        self.parents[self.groups[snapshot][record.0]]
    }

    pub fn alias_headers(
        &self,
        resolution: &Resolution<'_>,
        alias: &AliasGroup,
    ) -> Result<BTreeSet<String>, Error> {
        let mut headers = BTreeSet::new();
        let mut definitions = BTreeSet::new();
        for reference in &alias.observations {
            let snapshot = resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.aliases[reference.alias.0].observations[reference.observation];
            if observation.implicit {
                continue;
            }
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
            Ok(definitions)
        } else {
            owners(&alias.name, headers)
        }
    }

    pub fn record_headers(
        &self,
        resolution: &Resolution<'_>,
        family: usize,
    ) -> Result<BTreeSet<String>, Error> {
        if let Some(alias) = self.primary[family] {
            return self.alias_headers(resolution, &resolution.aliases[alias]);
        }
        let mut headers: BTreeSet<_> = self.families[family]
            .iter()
            .flat_map(|id| self.definition_headers[*id].iter().cloned())
            .collect();
        if headers.is_empty() {
            for &group in &self.families[family] {
                for reference in &resolution.groups[group].records {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    for &id in &snapshot.records[reference.record.0].observations {
                        let observation = &snapshot.observations[id.0];
                        headers.insert(snapshot.files[observation.location.file.0].clone());
                    }
                }
            }
        }
        owners(&self.names[family], headers)
    }

    pub fn enum_headers(
        &self,
        resolution: &Resolution<'_>,
        id: usize,
    ) -> Result<BTreeSet<String>, Error> {
        if let Some(alias) = self.enum_primary[id] {
            self.alias_headers(resolution, &resolution.aliases[alias])
        } else {
            owners(&self.enum_names[id], self.enum_headers[id].clone())
        }
    }

    pub fn enum_constants(&self, resolution: &Resolution<'_>, id: usize) -> bool {
        let reference = resolution.enums[id].enums[0];
        let enumeration = &resolution.snapshots[reference.snapshot].enums[reference.enumeration.0];
        let observation = &enumeration.observations[0];
        self.enum_primary[id].is_none()
            && (observation.anonymous || (!observation.scoped && midl_enum_name(&enumeration.name)))
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
                if !resolution.snapshots[a.snapshot].records[left.record.0]
                    .kind
                    .compatible(resolution.snapshots[b.snapshot].records[right.record.0].kind)
                    || x.layout != y.layout
                    || x.fields.len() != y.fields.len()
                    || x.bases.len() != y.bases.len()
                    || x.methods.len() != y.methods.len()
                    || !attributes_equal(&left.attributes, &right.attributes)
                    || !x.bases.iter().zip(&y.bases).all(|(&x, &y)| {
                        x.offset_bits == y.offset_bits
                            && self.family(a.snapshot, x.record)
                                == self.family(b.snapshot, y.record)
                    })
                    || !x.methods.iter().zip(&y.methods).all(|(x, y)| {
                        x.name == y.name
                            && x.slot == y.slot
                            && x.overrides == y.overrides
                            && resolve::value_shape_equal(&x.observation, &y.observation, false)
                            && self.same_type(
                                resolution,
                                (a.snapshot, x.observation.canonical),
                                (b.snapshot, y.observation.canonical),
                                &mut seen,
                            )
                    })
                    || !x.fields.iter().zip(&y.fields).all(|(left, right)| {
                        left.name == right.name
                            && left.offset_bits == right.offset_bits
                            && left.bit_width == right.bit_width
                            && resolve::field_attributes_equal(left, &x.fields, right, &y.fields)
                            && self.same_type(
                                resolution,
                                (a.snapshot, left.canonical),
                                (b.snapshot, right.canonical),
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
                if let (Some(a), Some(b)) = (
                    callback_parameters(resolution.snapshots[first.snapshot], a),
                    callback_parameters(resolution.snapshots[other.snapshot], b),
                ) && !resolve::parameters_equal(a, b)
                {
                    return Err(Error(format!(
                        "conflicting public names or contracts for `{}`: callback parameter annotations differ",
                        alias.name
                    )));
                }
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

fn join(parents: &mut [usize], sizes: &mut [usize], a: usize, b: usize) {
    let mut a = root(parents, a);
    let mut b = root(parents, b);
    if a != b {
        if sizes[a] < sizes[b] {
            std::mem::swap(&mut a, &mut b);
        }
        parents[b] = a;
        sizes[a] += sizes[b];
    }
}

fn owners(name: &str, headers: BTreeSet<String>) -> Result<BTreeSet<String>, Error> {
    if headers.is_empty() {
        Err(Error(format!("no header ownership for `{name}`")))
    } else {
        Ok(headers)
    }
}

fn primary_name(
    resolution: &Resolution<'_>,
    tag: &str,
    candidates: &[usize],
) -> Result<usize, Error> {
    if let [alias] = candidates {
        Ok(*alias)
    } else {
        let names: Vec<_> = candidates
            .iter()
            .map(|id| resolution.aliases[*id].name.as_str())
            .collect();
        Err(Error(format!(
            "ambiguous primary name for `{tag}`: {names:?}"
        )))
    }
}
