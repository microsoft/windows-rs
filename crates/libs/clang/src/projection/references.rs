use super::*;
use windows_metadata::reader::{HasAttributes, Index, MethodDef, TypeCategory, TypeDef};
use windows_metadata::{AsRow, MethodImplAttributes, ParamAttributes, TypeAttributes, Value};

/// Authoritative metadata and explicitly configured native namespace wrappers.
///
/// An empty prefix maps the metadata namespace directly; `"ABI"` maps its SDK ABI wrapper.
/// Fully captured definitions support scalar method validation. Explicitly external definitions
/// retain native identity and use-site checks; their member contracts belong to this metadata.
pub struct References<'a> {
    pub metadata: &'a Index,
    /// Metadata namespace subtree owning unqualified native COM names.
    pub global_namespace: Option<&'a str>,
    pub native_prefixes: &'a [&'a str],
    /// A configured prefix on native delegate type names, such as the SDK's `"I"`.
    pub delegate_prefix: Option<&'a str>,
}

impl<'a> References<'a> {
    fn is_global_namespace(&self, namespace: &str) -> bool {
        self.global_namespace.is_some_and(|root| {
            namespace == root
                || namespace
                    .strip_prefix(root)
                    .is_some_and(|tail| tail.starts_with('.'))
        })
    }

    /// Native COM names whose definitions can be owned by this metadata during capture.
    pub fn native_interfaces(&self) -> Result<BTreeSet<String>, Error> {
        let mut names = BTreeSet::new();
        for definition in self
            .metadata
            .types()
            .filter(|definition| com_type(*definition))
        {
            let namespace = definition.namespace().replace('.', "::");
            if self.is_global_namespace(definition.namespace()) {
                let native = definition.name().to_string();
                if self.get(&native)? != Some(definition) {
                    return Err(Error(format!(
                        "external metadata identity differs for `{native}`"
                    )));
                }
                names.insert(native);
            }
            let mut leaves = vec![definition.name().to_string()];
            if definition.category() == TypeCategory::Delegate
                && let Some(prefix) = self.delegate_prefix
            {
                leaves.push(format!("{prefix}{}", definition.name()));
            }
            for prefix in self.native_prefixes {
                for leaf in &leaves {
                    let native = if prefix.is_empty() {
                        format!("{namespace}::{leaf}")
                    } else {
                        format!("{prefix}::{namespace}::{leaf}")
                    };
                    if self.get(&native)? != Some(definition) {
                        return Err(Error(format!(
                            "external metadata identity differs for `{native}`"
                        )));
                    }
                    names.insert(native);
                }
            }
        }
        Ok(names)
    }

    pub(super) fn get(&self, native: &str) -> Result<Option<TypeDef<'a>>, Error> {
        if !native.contains("::") {
            if self.global_namespace.is_none() {
                return Ok(None);
            }
            let mut definitions = self
                .metadata
                .namespaces()
                .filter(|namespace| self.is_global_namespace(namespace))
                .flat_map(|namespace| self.metadata.get(namespace, native))
                .filter(|def| com_type(*def));
            let result = definitions.next();
            if definitions.next().is_some() {
                return Err(Error(format!(
                    "ambiguous external metadata ownership for `{native}`"
                )));
            }
            return Ok(result);
        }
        let mut result = None;
        for prefix in self.native_prefixes {
            let qualified = if prefix.is_empty() {
                Some(native)
            } else {
                native
                    .strip_prefix(prefix)
                    .and_then(|rest| rest.strip_prefix("::"))
            };
            let Some((namespace, name)) = qualified.and_then(|name| name.rsplit_once("::")) else {
                continue;
            };
            let namespace = namespace.replace("::", ".");
            let delegates = self
                .delegate_prefix
                .and_then(|prefix| name.strip_prefix(prefix))
                .into_iter()
                .flat_map(|name| self.metadata.get(&namespace, name))
                .filter(|definition| definition.category() == TypeCategory::Delegate);
            for definition in self.metadata.get(&namespace, name).chain(delegates) {
                if result.replace(definition).is_some() {
                    return Err(Error(format!(
                        "ambiguous external metadata ownership for `{native}`"
                    )));
                }
            }
        }
        Ok(result)
    }

    pub(super) fn bind(
        &self,
        resolution: &Resolution<'_>,
        plan: &Plan,
        rejected: &mut Option<&mut partial::Issues>,
    ) -> Result<BTreeMap<usize, TypeDef<'a>>, Error> {
        for prefix in self.native_prefixes {
            if !prefix.is_empty() {
                for part in prefix.split("::") {
                    ident(part)?;
                }
            }
        }
        if let Some(prefix) = self.delegate_prefix {
            ident(prefix)?;
        }
        let mut result = BTreeMap::new();
        for group in &resolution.groups {
            let name = resolution.name(group);
            let Some(definition) = self.get(name)? else {
                continue;
            };
            let reference = group.records[0];
            let family = plan.family(reference.snapshot, reference.record);
            if plan.families[family].len() != 1 || plan.owned[family] {
                return Err(Error(format!(
                    "external type `{name}` has a local public family"
                )));
            }
            result.insert(family, definition);
        }
        for group in &resolution.enums {
            let first = group.enums[0];
            let name = &resolution.snapshots[first.snapshot].enums[first.enumeration.0].name;
            if self.get(name)?.is_some() {
                reject(
                    rejected,
                    [partial::Key::Tag(name.clone())],
                    Error(format!("unsupported external enum contract for `{name}`")),
                )?;
            }
        }
        for alias in &resolution.aliases {
            let Some(definition) = self.get(&alias.name)? else {
                continue;
            };
            for reference in &alias.observations {
                let snapshot = resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.aliases[reference.alias.0].observations[reference.observation];
                let TypeKind::Record(record) = snapshot.types[observation.canonical.0].kind else {
                    reject(
                        rejected,
                        [partial::Key::Alias(alias.name.clone())],
                        Error(format!(
                            "unsupported external alias contract for `{}`",
                            alias.name
                        )),
                    )?;
                    continue;
                };
                if result.get(&plan.family(reference.snapshot, record)) != Some(&definition) {
                    return Err(Error(format!(
                        "external alias `{}` changes metadata identity",
                        alias.name
                    )));
                }
            }
        }
        Ok(result)
    }
}

impl Projector<'_, '_> {
    pub(super) fn interface_type(&self, snapshot: usize, record: RecordId) -> MetadataType {
        let family = self.plan.family(snapshot, record);
        if let Some(definition) = self.external.get(&family) {
            MetadataType::class_named(definition.namespace(), definition.name())
        } else {
            MetadataType::class_named(self.namespace, &self.plan.names[family])
        }
    }

    pub(super) fn validate_references(
        &self,
        rejected: &mut Option<&mut partial::Issues>,
    ) -> Result<(), Error> {
        for (snapshot_id, snapshot) in self.resolution.snapshots.iter().enumerate() {
            for (id, record) in snapshot.records.iter().enumerate() {
                if record.external
                    && !self
                        .external
                        .contains_key(&self.plan.family(snapshot_id, RecordId(id)))
                {
                    return Err(Error(format!(
                        "externally owned interface `{}` requires authoritative metadata",
                        record.name
                    )));
                }
            }
        }
        for (&family, &metadata) in self.external {
            let group = &self.resolution.groups[self.plan.families[family][0]];
            let name = self.resolution.name(group);
            if matches!(
                metadata.category(),
                TypeCategory::Interface | TypeCategory::Delegate
            ) {
                let expected = guid(metadata)?;
                for record in &group.records {
                    let snapshot = self.resolution.snapshots[record.snapshot];
                    for id in &snapshot.records[record.record.0].observations {
                        if resolve::uuid(&snapshot.observations[id.0].attributes)
                            .is_some_and(|uuid| uuid != expected)
                        {
                            return Err(Error(format!(
                                "external interface `{name}` UUID differs from metadata"
                            )));
                        }
                    }
                }
            }
            let Some(reference) = group.definition else {
                let keys = group.records.iter().map(|reference| {
                    partial::Key::of(
                        self.resolution.snapshots[reference.snapshot],
                        DeclarationId::Record(reference.record),
                    )
                });
                reject(
                    rejected,
                    keys,
                    Error(format!(
                        "external interface `{name}` has no native definition"
                    )),
                )?;
                continue;
            };
            let snapshot = self.resolution.snapshots[reference.snapshot];
            let observation = &snapshot.observations[reference.observation.0];
            let definition = observation.definition.as_ref().unwrap();
            let external = snapshot.records[observation.record.0].external;
            if matches!(
                metadata.category(),
                TypeCategory::Interface | TypeCategory::Delegate
            ) && resolve::uuid(&observation.attributes) != Some(guid(metadata)?)
            {
                return Err(Error(format!(
                    "external interface `{name}` UUID differs from metadata"
                )));
            }
            if !(if external {
                com_type(metadata)
            } else {
                supported(metadata)
            }) {
                let keys = group.records.iter().map(|reference| {
                    partial::Key::of(
                        self.resolution.snapshots[reference.snapshot],
                        DeclarationId::Record(reference.record),
                    )
                });
                reject(
                    rejected,
                    keys,
                    Error(format!(
                        "unsupported external metadata contract for `{name}`"
                    )),
                )?;
                continue;
            }
            if !metadata.flags().contains(TypeAttributes::WindowsRuntime) {
                let native_base = definition
                    .bases
                    .first()
                    .map(|base| self.interface_type(reference.snapshot, base.record));
                let metadata_base = native_base_type(metadata)?;
                if native_base != metadata_base {
                    return Err(Error(format!(
                        "external interface `{name}` base differs from metadata"
                    )));
                }
                native_methods(metadata)?;
                self.interface(reference.snapshot, observation, &ident(public_name(name))?)?;
                continue;
            }
            let (base_name, base_uuid, first_slot) =
                if metadata.category() == TypeCategory::Delegate {
                    ("IUnknown", 0x00000000_0000_0000_c000_000000000046, 3)
                } else {
                    ("IInspectable", 0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90, 6)
                };
            // Apply the same native interface checks as local publication before using its ABI.
            self.interface(reference.snapshot, observation, &ident(public_name(name))?)?;
            let Some(base) = definition.bases.first() else {
                return Err(Error(format!(
                    "external WinRT type `{name}` requires {base_name}"
                )));
            };
            let base_group = self.resolution.group(RecordRef {
                snapshot: reference.snapshot,
                record: base.record,
            });
            let base_ref = base_group.definition.unwrap();
            let base =
                &self.resolution.snapshots[base_ref.snapshot].observations[base_ref.observation.0];
            if resolve::uuid(&base.attributes) != Some(base_uuid) {
                return Err(Error(format!(
                    "external WinRT type `{name}` requires {base_name}"
                )));
            }
            if external {
                continue;
            }
            let methods: Vec<_> = methods(metadata).collect();
            if methods.len() != definition.methods.len() {
                return Err(Error(format!(
                    "external interface `{name}` method count differs from metadata"
                )));
            }
            for (index, (native, metadata)) in definition.methods.iter().zip(methods).enumerate() {
                let signature = metadata.signature(&[]);
                let parameters =
                    metadata
                        .params_by_sequence(signature.types.len())
                        .map_err(|error| {
                            Error(format!("external method `{name}.{}`: {error}", native.name))
                        })?;
                if native.name != metadata.name()
                    || native.slot != index as u64 + first_slot
                    || native.overrides
                    || metadata
                        .impl_flags()
                        .contains(MethodImplAttributes::PreserveSig)
                {
                    return Err(Error(format!(
                        "unsupported external method mapping `{name}.{}`",
                        native.name
                    )));
                }
                let function = written_function(snapshot, native.observation.canonical)?.prototype;
                let returns_value = signature.return_type != MetadataType::Void;
                if function.parameters.len() != signature.types.len() + usize::from(returns_value)
                    || !scalar_matches(snapshot, function.result, &MetadataType::I32)
                    || function.method_qualifiers != Qualifiers::default()
                {
                    return Err(Error(format!(
                        "external method `{name}.{}` ABI differs from metadata",
                        native.name
                    )));
                }
                for (index, ty) in signature.types.iter().enumerate() {
                    let flags = parameters.params()[index]
                        .map(|parameter| parameter.flags())
                        .unwrap_or_default();
                    let output = flags.contains(ParamAttributes::Out);
                    if flags.contains(ParamAttributes::Optional)
                        || (output && flags.contains(ParamAttributes::In))
                    {
                        return Err(Error(format!(
                            "unsupported external parameter contract `{name}.{}`",
                            native.name
                        )));
                    }
                    check_parameter(snapshot, function.parameters[index], ty, output).map_err(
                        |error| Error(format!("external method `{name}.{}`: {error}", native.name)),
                    )?;
                }
                if returns_value {
                    check_parameter(
                        snapshot,
                        *function.parameters.last().unwrap(),
                        &signature.return_type,
                        true,
                    )
                    .map_err(|error| {
                        Error(format!("external method `{name}.{}`: {error}", native.name))
                    })?;
                }
            }
        }
        Ok(())
    }
}

fn com_type(definition: TypeDef<'_>) -> bool {
    (definition.category() == TypeCategory::Interface
        || (definition.category() == TypeCategory::Delegate
            && definition.flags().contains(TypeAttributes::WindowsRuntime)))
        && definition.generic_params().next().is_none()
}

fn supported(definition: TypeDef<'_>) -> bool {
    com_type(definition)
        && definition.flags().contains(TypeAttributes::WindowsRuntime)
        && definition.interface_impls().next().is_none()
        && (definition.category() != TypeCategory::Delegate
            || (methods(definition).count() == 1
                && definition
                    .methods()
                    .all(|method| matches!(method.name(), "Invoke" | ".ctor"))))
        && methods(definition).all(|method| {
            let signature = method.signature(&[]);
            !method
                .impl_flags()
                .contains(MethodImplAttributes::PreserveSig)
                && (signature.return_type == MetadataType::Void
                    || scalar_type(&signature.return_type))
                && signature.types.iter().all(scalar_type)
        })
}

fn native_base_type(definition: TypeDef<'_>) -> Result<Option<MetadataType>, Error> {
    let mut bases = definition.interface_impls();
    let base = bases.next().map(|base| base.interface(&[]));
    if bases.next().is_some()
        || base
            .as_ref()
            .is_some_and(|base| !matches!(base, MetadataType::ClassName(_)))
    {
        return Err(Error(format!(
            "external interface `{}` requires at most one interface base",
            definition.name()
        )));
    }
    Ok(base)
}

pub(super) fn native_methods(mut definition: TypeDef<'_>) -> Result<Vec<&str>, Error> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    loop {
        if !seen.insert((definition.namespace(), definition.name())) {
            return Err(Error("cyclic external interface inheritance".into()));
        }
        if definition.category() != TypeCategory::Interface
            || definition.flags().contains(TypeAttributes::WindowsRuntime)
            || definition.generic_params().next().is_some()
        {
            return Err(Error(format!(
                "native inheritance from externally owned interface `{}` is unsupported",
                definition.name()
            )));
        }
        chain.push(definition);
        let Some(MetadataType::ClassName(base)) = native_base_type(definition)? else {
            break;
        };
        let mut definitions = definition.index().get(&base.namespace, &base.name);
        definition = definitions
            .next()
            .ok_or_else(|| Error(format!("missing external interface base `{base:?}`")))?;
        if definitions.next().is_some() {
            return Err(Error(format!(
                "ambiguous external interface base `{base:?}`"
            )));
        }
    }
    let mut result = Vec::new();
    for definition in chain.into_iter().rev() {
        for method in definition.methods() {
            if method
                .find_attribute("NativeOverrideSlotAttribute")
                .is_some()
            {
                return Err(Error(format!(
                    "external interface slot overrides are unsupported on `{}.{}`",
                    definition.name(),
                    method.name()
                )));
            }
            result.push(method.name());
        }
    }
    Ok(result)
}

fn methods(definition: TypeDef<'_>) -> impl Iterator<Item = MethodDef<'_>> {
    let delegate = definition.category() == TypeCategory::Delegate;
    definition
        .methods()
        .filter(move |method| !delegate || method.name() == "Invoke")
}

fn scalar_type(metadata: &MetadataType) -> bool {
    matches!(
        metadata,
        MetadataType::I8
            | MetadataType::U8
            | MetadataType::I16
            | MetadataType::U16
            | MetadataType::I32
            | MetadataType::U32
            | MetadataType::I64
            | MetadataType::U64
            | MetadataType::F32
            | MetadataType::F64
            | MetadataType::Bool
            | MetadataType::Char
    )
}

fn check_parameter(
    snapshot: &Snapshot,
    mut native: TypeId,
    metadata: &MetadataType,
    output: bool,
) -> Result<(), Error> {
    if !scalar_type(metadata) {
        return Err(Error(format!(
            "unsupported external parameter type `{metadata:?}`"
        )));
    }
    if output {
        let TypeKind::Pointer(pointee) = checked_type(snapshot, native)?.kind else {
            return Err(Error(
                "external output parameter requires a native pointer".into(),
            ));
        };
        if snapshot.types[pointee.0].qualifiers != Qualifiers::default() {
            return Err(Error(
                "external output parameter has qualified storage".into(),
            ));
        }
        native = pointee;
    }
    if !scalar_matches(snapshot, native, metadata) {
        return Err(Error(format!(
            "external parameter ABI differs from `{metadata:?}`"
        )));
    }
    Ok(())
}

fn scalar_matches(snapshot: &Snapshot, native: TypeId, metadata: &MetadataType) -> bool {
    let ty = &snapshot.types[native.0];
    if ty.qualifiers.volatile || ty.qualifiers.restrict || ty.qualifiers.unaligned {
        return false;
    }
    let TypeKind::Scalar(scalar) = &ty.kind else {
        return false;
    };
    match metadata {
        MetadataType::F32 => *scalar == Scalar::Float,
        MetadataType::F64 => *scalar == Scalar::Double,
        MetadataType::Bool => matches!(scalar, Scalar::Bool | Scalar::UnsignedChar),
        MetadataType::Char => matches!(
            scalar,
            Scalar::WChar | Scalar::Char16 | Scalar::UnsignedShort
        ),
        _ => {
            let (signed, size) = match metadata {
                MetadataType::I8 => (true, 1),
                MetadataType::U8 => (false, 1),
                MetadataType::I16 => (true, 2),
                MetadataType::U16 => (false, 2),
                MetadataType::I32 => (true, 4),
                MetadataType::U32 => (false, 4),
                MetadataType::I64 => (true, 8),
                MetadataType::U64 => (false, 8),
                _ => return false,
            };
            scalar.signed_integer() == Some(signed)
                && ty.layout.is_some_and(|layout| layout.size == size)
        }
    }
}

fn guid(definition: TypeDef<'_>) -> Result<u128, Error> {
    let attribute = definition
        .find_attribute("GuidAttribute")
        .ok_or_else(|| Error(format!("external interface `{definition:?}` has no GUID")))?;
    let values: Vec<_> = attribute
        .value()
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    let [
        Value::U32(first),
        Value::U16(second),
        Value::U16(third),
        tail @ ..,
    ] = values.as_slice()
    else {
        return Err(Error(format!("invalid external GUID on `{definition:?}`")));
    };
    if tail.len() != 8 {
        return Err(Error(format!("invalid external GUID on `{definition:?}`")));
    }
    let mut guid = (u128::from(*first) << 32) | (u128::from(*second) << 16) | u128::from(*third);
    for value in tail {
        let Value::U8(value) = value else {
            return Err(Error(format!("invalid external GUID on `{definition:?}`")));
        };
        guid = (guid << 8) | u128::from(*value);
    }
    Ok(guid)
}
