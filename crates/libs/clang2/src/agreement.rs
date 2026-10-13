use super::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Identity {
    name: String,
    usr: String,
    kind: String,
    file: String,
    owner: Option<Box<Self>>,
    field: Option<usize>,
}

impl Snapshot {
    /// Checks every consumed observation against another capture on the same compiler and target.
    pub fn agree(&self, other: &Self) -> Result<()> {
        let failure = |message| {
            Error::new(
                Stage::Agreement,
                format!("{} vs {}: {message}", self.input_name, other.input_name),
            )
        };
        if self.target != other.target
            || self.target_triple != other.target_triple
            || self.compiler != other.compiler
            || self.library != other.library
            || self
                .layout_bridge
                .as_ref()
                .map(|bridge| (&bridge.parser_sha256, bridge.abi))
                != other
                    .layout_bridge
                    .as_ref()
                    .map(|bridge| (&bridge.parser_sha256, bridge.abi))
        {
            return Err(failure(
                "native agreement requires the same compiler and target".into(),
            ));
        }
        self.check()?;
        other.check()?;
        let left = self.identities()?;
        let right = other.identities()?;
        let identities = left
            .iter()
            .chain(&right)
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(id, identity)| (identity, id))
            .collect::<BTreeMap<_, _>>();
        let right_indices = right
            .iter()
            .enumerate()
            .map(|(id, key)| (key, id))
            .collect::<BTreeMap<_, _>>();
        let left_ids = left.iter().map(|key| identities[key]).collect::<Vec<_>>();
        let right_ids = right.iter().map(|key| identities[key]).collect::<Vec<_>>();
        for (id, key) in left.iter().enumerate() {
            let decl = &self.declarations[id];
            let Some(&right_id) = right_indices.get(key) else {
                return Err(failure(format!(
                    "{}: missing original native identity",
                    decl.evidence.qualified_name
                )));
            };
            let observed = &other.declarations[right_id];
            let mut evidence = decl.evidence.clone();
            evidence.id = left_ids[id];
            let mut other_evidence = observed.evidence.clone();
            other_evidence.id = right_ids[right_id];
            if evidence != other_evidence
                || decl.names != observed.names
                || decl.anonymous != observed.anonymous
                || decl.owner.map(|id| left_ids[id]) != observed.owner.map(|id| right_ids[id])
                || remap(&decl.kind, &left_ids) != remap(&observed.kind, &right_ids)
            {
                return Err(failure(format!(
                    "{}: original native observations disagree",
                    decl.evidence.qualified_name
                )));
            }
        }
        for evidence in &self.macros {
            let candidates = other
                .macros
                .iter()
                .filter(|other| other.name == evidence.name)
                .collect::<Vec<_>>();
            let [observed] = candidates.as_slice() else {
                return Err(failure(format!(
                    "{}: missing or ambiguous selected macro",
                    evidence.name
                )));
            };
            if evidence != *observed {
                return Err(failure(format!(
                    "{}: original macro definition or compiler result disagrees",
                    evidence.name
                )));
            }
        }
        Ok(())
    }

    fn identities(&self) -> Result<Vec<Identity>> {
        let mut seen = BTreeSet::new();
        self.declarations
            .iter()
            .enumerate()
            .map(|(id, _)| {
                let identity = self.identity(id, &mut BTreeSet::new())?;
                if !seen.insert(identity.clone()) {
                    return Err(Error::new(
                        Stage::Agreement,
                        "ambiguous cross-input native identity",
                    ));
                }
                Ok(identity)
            })
            .collect()
    }

    fn identity(&self, id: Id, visiting: &mut BTreeSet<Id>) -> Result<Identity> {
        if visiting.len() >= 64 || !visiting.insert(id) {
            return Err(Error::new(
                Stage::Agreement,
                "cyclic or excessive native ownership",
            ));
        }
        let declaration = &self.declarations[id];
        let owner = declaration
            .owner
            .map(|owner| self.identity(owner, visiting).map(Box::new))
            .transpose()?;
        let field = if declaration.anonymous
            && let Some(owner) = declaration.owner
        {
            let DeclarationKind::Record { fields, .. } = &self.declarations[owner].kind else {
                return Err(Error::new(
                    Stage::Agreement,
                    "anonymous native owner is not a record",
                ));
            };
            let slots = fields
                .iter()
                .filter(
                    |field| matches!(field.written.kind, NativeKind::Record(child) if child == id),
                )
                .map(|field| field.evidence.ordinal)
                .collect::<Vec<_>>();
            let [slot] = slots.as_slice() else {
                return Err(Error::new(
                    Stage::Agreement,
                    "anonymous native identity requires one owning field",
                ));
            };
            Some(*slot)
        } else {
            None
        };
        visiting.remove(&id);
        Ok(Identity {
            name: declaration.evidence.qualified_name.clone(),
            usr: declaration.evidence.usr.clone(),
            kind: declaration.evidence.kind.clone(),
            file: declaration.evidence.spelling.file.clone(),
            owner,
            field,
        })
    }
}

fn remap(kind: &DeclarationKind, ids: &[Id]) -> DeclarationKind {
    let mut result = kind.clone();
    match &mut result {
        DeclarationKind::Alias { written, canonical } => {
            remap_type(written, ids);
            remap_type(canonical, ids);
        }
        DeclarationKind::Record { fields, .. } => {
            for field in fields {
                field.evidence.record_id = field.evidence.record_id.map(|id| ids[id]);
                remap_type(&mut field.written, ids);
                remap_type(&mut field.canonical, ids);
            }
        }
        DeclarationKind::Enum { underlying, .. } => remap_type(underlying, ids),
        DeclarationKind::Function(functions) => {
            for function in functions {
                remap_function(function, ids);
            }
        }
        DeclarationKind::Interface { base, methods, .. } => {
            *base = base.map(|id| ids[id]);
            for method in methods {
                remap_function(&mut method.function, ids);
            }
        }
        DeclarationKind::Opaque { .. } => {}
    }
    result
}

fn remap_function(function: &mut NativeFunction, ids: &[Id]) {
    remap_type(&mut function.result, ids);
    remap_type(&mut function.canonical_result, ids);
    for parameter in &mut function.params {
        remap_type(&mut parameter.written, ids);
        remap_type(&mut parameter.canonical, ids);
    }
}

fn remap_type(ty: &mut NativeType, ids: &[Id]) {
    match &mut ty.kind {
        NativeKind::Alias(id) | NativeKind::Record(id) | NativeKind::Enum(id) => *id = ids[*id],
        NativeKind::Pointer(ty) | NativeKind::Reference(ty) | NativeKind::Array(ty, _) => {
            remap_type(ty, ids);
        }
        NativeKind::Void | NativeKind::Scalar(_) => {}
    }
}
