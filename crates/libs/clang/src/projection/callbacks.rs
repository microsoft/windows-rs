use super::*;
use windows_metadata::reader::AsRow;

impl Projector<'_, '_> {
    pub(super) fn alias_bodies(
        &self,
        alias: &AliasGroup,
        attributes: &[Attribute],
    ) -> Result<Vec<Body>, Error> {
        alias
            .observations
            .iter()
            .filter(|reference| {
                !self.resolution.snapshots[reference.snapshot].aliases[reference.alias.0]
                    .observations[reference.observation]
                    .implicit
            })
            .map(|reference| {
                let snapshot = self.resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.aliases[reference.alias.0].observations[reference.observation];
                self.alias(reference.snapshot, observation, &alias.name, attributes)
            })
            .collect()
    }

    pub(super) fn callback_abis(&self, name: &str) -> Result<Vec<CallingConvention>, Error> {
        if let Some(abis) = self.callback_abis.borrow().get(name) {
            return Ok(abis.clone());
        }
        let ty = if let Some(definition) = self.external.aliases.get(name) {
            reference_values::expand_type(
                MetadataType::FnPtr(Box::new(reference_values::callback_signature(*definition)?)),
                definition.index(),
                self.resolution.snapshots[0].target,
                &mut HashSet::new(),
                false,
            )?
        } else {
            let alias = self
                .resolution
                .aliases
                .iter()
                .find(|alias| alias.name == name)
                .ok_or_else(|| Error(format!("missing callback `{name}`")))?;
            let attributes = annotations::covering(
                name,
                alias.observations.iter().map(|reference| {
                    self.resolution.snapshots[reference.snapshot].aliases[reference.alias.0]
                        .observations[reference.observation]
                        .attributes
                        .as_slice()
                }),
            )?;
            let mut bodies = self.alias_bodies(alias, attributes)?;
            let target = self.resolution.snapshots[0].target;
            agree(name, target, &mut bodies)?;
            let mut body = bodies.into_iter().next().unwrap();
            if let Some((key, convention)) = self.x86_callbacks.get(name)
                && body.contract_key() == *key
            {
                body.canonicalize_abi(target, *convention);
            }
            let reference = alias_source(self.resolution, alias).unwrap_or(alias.observations[0]);
            let snapshot = self.resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.aliases[reference.alias.0].observations[reference.observation];
            let function = match &snapshot.types[observation.canonical.0].kind {
                TypeKind::Function(function) => function,
                _ => callback(snapshot, observation.canonical).unwrap(),
            };
            let mut signature = self.signature(
                reference.snapshot,
                function,
                Some(written_function(snapshot, observation.underlying)?),
            )?;
            signature.flags = match body.abi.unwrap().convention {
                CallingConvention::C => windows_metadata::MethodCallAttributes::C,
                CallingConvention::Stdcall => windows_metadata::MethodCallAttributes::STDCALL,
            };
            MetadataType::FnPtr(Box::new(signature))
        };
        let abis = self.type_callback_abis(&ty)?;
        self.callback_abis
            .borrow_mut()
            .insert(name.to_string(), abis.clone());
        Ok(abis)
    }

    pub(super) fn type_callback_abis(
        &self,
        ty: &MetadataType,
    ) -> Result<Vec<CallingConvention>, Error> {
        let mut abis = Vec::new();
        match ty {
            MetadataType::FnPtr(signature) => {
                abis.push(match signature.flags {
                    windows_metadata::MethodCallAttributes::C => CallingConvention::C,
                    windows_metadata::MethodCallAttributes::STDCALL => CallingConvention::Stdcall,
                    _ => return Err(Error("unsupported callback calling convention".into())),
                });
                abis.extend(self.type_callback_abis(&signature.return_type)?);
                for ty in &signature.types {
                    abis.extend(self.type_callback_abis(ty)?);
                }
            }
            MetadataType::PtrMut(inner, _)
            | MetadataType::PtrConst(inner, _)
            | MetadataType::Ptr64(inner, _)
            | MetadataType::ArrayFixed(inner, _) => {
                abis.extend(self.type_callback_abis(inner)?);
            }
            MetadataType::ClassName(name) | MetadataType::ValueName(name) => {
                if let Some(definition) = self.external.aliases.values().find(|definition| {
                    definition.namespace() == name.namespace && definition.name() == name.name
                }) {
                    let expanded = reference_values::expand_type(
                        ty.clone(),
                        definition.index(),
                        self.resolution.snapshots[0].target,
                        &mut HashSet::new(),
                        false,
                    )?;
                    if expanded != *ty {
                        abis.extend(self.type_callback_abis(&expanded)?);
                    }
                } else if name.namespace == self.namespace
                    && let Some(alias) = self
                        .resolution
                        .aliases
                        .iter()
                        .find(|alias| alias.name == name.name)
                {
                    let reference = alias_source(self.resolution, alias).unwrap();
                    let snapshot = self.resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.aliases[reference.alias.0].observations[reference.observation];
                    if callback(snapshot, observation.canonical).is_some()
                        || matches!(
                            snapshot.types[observation.canonical.0].kind,
                            TypeKind::Function(_)
                        )
                    {
                        abis.extend(self.callback_abis(&alias.name)?);
                    } else if matches!(
                        snapshot.types[observation.canonical.0].kind,
                        TypeKind::Pointer(_)
                            | TypeKind::Pointer64(_)
                            | TypeKind::LValueReference(_)
                            | TypeKind::Array { .. }
                    ) {
                        let ty = self.written_ty(
                            reference.snapshot,
                            observation.underlying,
                            observation.canonical,
                        )?;
                        if ty != MetadataType::ValueName(name.clone()) {
                            abis.extend(self.type_callback_abis(&ty)?);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(abis)
    }
}
