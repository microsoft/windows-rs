//! Native IR to Win32 RDL projection and a three-target pipeline entry point.

use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use windows_metadata::Type as MetadataType;
use windows_rdl::emit::{write_ident, write_type, write_value};

mod plan;
use plan::Plan;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[derive(Debug, Eq, PartialEq)]
pub struct Partition {
    pub header: String,
    pub rdl: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Output {
    pub partitions: BTreeMap<String, Partition>,
    pub records: usize,
    pub enums: usize,
    /// Published aliases, including named callback declarations.
    pub aliases: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PartialOutput {
    pub output: Output,
    pub omissions: Vec<String>,
}

/// Projects the supported subset with explicit omissions. Compiler and correspondence errors are fatal.
/// A failed entity is omitted on all targets, so failure cannot masquerade as target availability.
pub fn generate_partial(inputs: &[Input], namespace: &str) -> Result<PartialOutput, Error> {
    let captured =
        capture_partial(inputs, &TARGETS).map_err(|error| Error(format!("capture: {error}")))?;
    project_partial(captured, namespace)
}

/// Projects an existing partial capture without reparsing its headers.
pub fn project_partial(captured: PartialCapture, namespace: &str) -> Result<PartialOutput, Error> {
    let mut snapshots = captured.snapshots;
    let published_aliases: HashSet<_> = snapshots
        .iter()
        .flat_map(|snapshot| {
            snapshot
                .aliases
                .iter()
                .filter(|alias| {
                    alias.observations.iter().any(|observation| {
                        publishable_alias(snapshot, observation.canonical)
                            || !resolve::alignments(&observation.attributes).is_empty()
                    })
                })
                .map(|alias| alias.name.clone())
        })
        .collect();
    let mut issues = partial::Issues {
        messages: captured.omissions.into_iter().collect(),
        ..Default::default()
    };
    let count = |snapshot: &Snapshot| {
        snapshot.records.len()
            + snapshot.aliases.len()
            + snapshot.enums.len()
            + snapshot.roots.len()
    };
    loop {
        let remaining: usize = snapshots.iter().map(count).sum();
        let merged = merge(&snapshots).map_err(|error| Error(format!("merge: {error}")))?;
        let mut rejected = partial::Issues::default();
        let output = project_impl(&merged, namespace, Some(&mut rejected))
            .map_err(|error| Error(format!("projection: {error}")))?;
        if rejected.excluded.is_empty() {
            return Ok(PartialOutput {
                output,
                omissions: issues.finish(),
            });
        }
        for (key, reason) in rejected.excluded {
            match &key {
                partial::Key::Tag(_) => issues.reject(key, format!("projection: {reason}")),
                partial::Key::Alias(name) => {
                    if published_aliases.contains(name) {
                        issues.reject(key, format!("projection: {reason}"));
                        continue;
                    }
                    // Non-value aliases such as void are expanded at use sites.
                    for snapshot in &mut snapshots {
                        snapshot.roots.retain(|id| match id {
                            DeclarationId::Alias(id) => &snapshot.aliases[id.0].name != name,
                            DeclarationId::Record(_) | DeclarationId::Enum(_) => true,
                        });
                    }
                    issues
                        .messages
                        .insert(format!("projection {key:?}: {reason}"));
                }
            }
        }
        partial::prune(&mut snapshots, &mut issues);
        assert!(
            snapshots.iter().map(count).sum::<usize>() < remaining,
            "partial projection must remove rejected declarations or roots"
        );
    }
}

/// Captures, reconciles, and projects a configured three-target job.
pub fn generate(inputs: &[Input], namespace: &str) -> Result<Output, Error> {
    let snapshots =
        capture(inputs, &TARGETS).map_err(|error| Error(format!("capture: {error}")))?;
    let merged = merge(&snapshots).map_err(|error| Error(format!("merge: {error}")))?;
    project(&merged, namespace).map_err(|error| Error(format!("projection: {error}")))
}

/// Projects supported definitions, preserving target variants and checking metadata ABI layout.
///
/// Checked by-value alias families share a public record, without merging native identities.
/// Other reachable value names remain transparent aliases. Ambiguous header ownership is an error.
pub fn project(merged: &Merged<'_>, namespace: &str) -> Result<Output, Error> {
    project_impl(merged, namespace, None)
}

fn project_impl(
    merged: &Merged<'_>,
    namespace: &str,
    mut rejected: Option<&mut partial::Issues>,
) -> Result<Output, Error> {
    if merged.targets.keys().copied().collect::<Vec<_>>() != TARGETS {
        return Err(Error(
            "projection requires x86, x64, and ARM64 observations".into(),
        ));
    }
    let modules = namespace
        .split('.')
        .map(ident)
        .collect::<Result<Vec<_>, _>>()?;
    let root_aliases: HashSet<_> = merged
        .targets
        .values()
        .flat_map(|resolution| &resolution.snapshots)
        .flat_map(|snapshot| {
            snapshot.roots.iter().filter_map(|root| match root {
                DeclarationId::Alias(alias) => Some(snapshot.aliases[alias.0].name.as_str()),
                DeclarationId::Record(_) | DeclarationId::Enum(_) => None,
            })
        })
        .collect();
    let mut declarations: BTreeMap<String, Declaration> = BTreeMap::new();
    for (&target, resolution) in &merged.targets {
        let plan = Plan::new(resolution)?;
        let projector = Projector {
            namespace,
            resolution,
            plan: &plan,
        };
        let mut published = HashSet::new();
        for (id, family) in plan.families.iter().enumerate() {
            if family.is_empty() {
                continue;
            }
            let name = &plan.names[id];
            if !published.insert(name.as_str()) {
                return Err(Error(format!("duplicate projected name `{name}`")));
            }
            let keys: Vec<_> = family
                .iter()
                .map(|group| {
                    partial::Key::Tag(resolution.name(&resolution.groups[*group]).to_string())
                })
                .collect();
            let mut bodies = Vec::new();
            for &group in family {
                for reference in &resolution.groups[group].records {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    for &observation in &snapshot.records[reference.record.0].observations {
                        let observation = &snapshot.observations[observation.0];
                        if observation.definition.is_some() {
                            bodies.push(projector.record(
                                reference.snapshot,
                                observation,
                                &ident(name)?,
                            ));
                        }
                    }
                }
            }
            let bodies = bodies.into_iter().collect::<Result<Vec<_>, _>>();
            if let Ok(bodies) = &bodies {
                agree(name, target, bodies)?;
            }
            let result = bodies.and_then(|bodies| {
                let body = bodies
                    .into_iter()
                    .next()
                    .ok_or_else(|| Error(format!("record `{name}` is incomplete")))?;
                Ok((body, plan.record_header(resolution, id)?))
            });
            match result {
                Ok((body, header)) => {
                    publish(
                        &mut declarations,
                        target,
                        name,
                        body,
                        header,
                        keys,
                        PublishedKind::Record,
                    );
                }
                Err(error) => reject(&mut rejected, keys, error)?,
            }
        }
        for (id, group) in resolution.enums.iter().enumerate() {
            let name = &plan.enum_names[id];
            if !published.insert(name.as_str()) {
                return Err(Error(format!("duplicate projected name `{name}`")));
            }
            let first = group.enums[0];
            let keys = vec![partial::Key::Tag(
                resolution.snapshots[first.snapshot].enums[first.enumeration.0]
                    .name
                    .clone(),
            )];
            let result = (|| {
                let reference = group
                    .definition
                    .ok_or_else(|| Error(format!("enum `{name}` is incomplete")))?;
                let observation = &resolution.snapshots[reference.snapshot].enums
                    [reference.enumeration.0]
                    .observations[reference.observation];
                let body = projector.enumeration(reference.snapshot, observation, &ident(name)?)?;
                Ok((body, plan.enum_header(resolution, id)?))
            })();
            match result {
                Ok((body, header)) => publish(
                    &mut declarations,
                    target,
                    name,
                    body,
                    header,
                    keys,
                    PublishedKind::Enum,
                ),
                Err(error) => reject(&mut rejected, keys, error)?,
            }
        }
        for (id, alias) in resolution.aliases.iter().enumerate() {
            if alias.observations.iter().any(|reference| {
                !resolve::alignments(
                    &resolution.snapshots[reference.snapshot].aliases[reference.alias.0]
                        .observations[reference.observation]
                        .attributes,
                )
                .is_empty()
            }) {
                reject(
                    &mut rejected,
                    [partial::Key::Alias(alias.name.clone())],
                    Error(format!(
                        "aligned typedef `{}` projection is unsupported",
                        alias.name
                    )),
                )?;
                continue;
            }
            let first = alias.observations[0];
            let snapshot = resolution.snapshots[first.snapshot];
            let canonical =
                snapshot.aliases[first.alias.0].observations[first.observation].canonical;
            if (!root_aliases.contains(alias.name.as_str())
                && !publishable_alias(snapshot, canonical))
                || plan.aliases[id].is_some_and(|family| plan.names[family] == alias.name)
                || plan.enum_aliases[id].is_some_and(|group| plan.enum_names[group] == alias.name)
            {
                continue;
            }
            if !published.insert(&alias.name) {
                return Err(Error(format!("duplicate projected name `{}`", alias.name)));
            }
            let keys = vec![partial::Key::Alias(alias.name.clone())];
            let bodies = alias
                .observations
                .iter()
                .map(|reference| {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.aliases[reference.alias.0].observations[reference.observation];
                    projector.alias(reference.snapshot, observation, &alias.name)
                })
                .collect::<Result<Vec<_>, _>>();
            if let Ok(bodies) = &bodies {
                agree(&alias.name, target, bodies)?;
            }
            let result = bodies.and_then(|bodies| {
                Ok((
                    bodies.into_iter().next().unwrap(),
                    plan.alias_header(resolution, alias)?,
                ))
            });
            match result {
                Ok((body, header)) => publish(
                    &mut declarations,
                    target,
                    &alias.name,
                    body,
                    header,
                    keys,
                    PublishedKind::Alias,
                ),
                Err(error) => reject(&mut rejected, keys, error)?,
            }
        }
    }
    let records = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Record)
        .count();
    let enums = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Enum)
        .count();
    let aliases = declarations.len() - records - enums;
    let mut items = BTreeMap::new();
    for (name, declaration) in declarations {
        if let Err(error) = insert(
            &mut items,
            &declaration.headers,
            &name,
            variants(declaration.bodies),
        ) {
            reject(&mut rejected, declaration.keys, error)?;
        }
    }
    let mut partitions = BTreeMap::new();
    for (stem, (header, declarations)) in items {
        let mut body: TokenStream = declarations.into_values().collect();
        for module in modules.iter().rev() {
            body = quote! { mod #module { #body } };
        }
        let text = quote! { #[win32] #body }.to_string();
        partitions.insert(
            stem,
            Partition {
                header,
                rdl: windows_rdl::formatter::format(&text),
            },
        );
    }
    Ok(Output {
        partitions,
        records,
        enums,
        aliases,
    })
}

#[derive(Default)]
struct Declaration {
    headers: BTreeSet<String>,
    bodies: BTreeMap<String, (Body, Vec<Target>)>,
    keys: BTreeSet<partial::Key>,
    kind: PublishedKind,
}

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
enum PublishedKind {
    #[default]
    Alias,
    Enum,
    Record,
}

fn publish(
    items: &mut BTreeMap<String, Declaration>,
    target: Target,
    name: &str,
    body: Body,
    header: String,
    keys: Vec<partial::Key>,
    kind: PublishedKind,
) {
    let item = items.entry(name.to_string()).or_default();
    item.headers.insert(header);
    item.keys.extend(keys);
    item.kind = item.kind.max(kind);
    item.bodies
        .entry(body.key())
        .or_insert_with(|| (body, Vec::new()))
        .1
        .push(target);
}

fn reject(
    rejected: &mut Option<&mut partial::Issues>,
    keys: impl IntoIterator<Item = partial::Key>,
    error: Error,
) -> Result<(), Error> {
    if let Some(rejected) = rejected.as_deref_mut() {
        for key in keys {
            rejected.reject(key, error.to_string());
        }
        Ok(())
    } else {
        Err(error)
    }
}

struct Projector<'a, 'b> {
    namespace: &'a str,
    resolution: &'a Resolution<'b>,
    plan: &'a Plan,
}

fn publishable_alias(snapshot: &Snapshot, id: TypeId) -> bool {
    let ty = &snapshot.types[id.0];
    !ty.qualifiers.constant && (ty.layout.is_some() || matches!(ty.kind, TypeKind::Record(_)))
}

fn agree(name: &str, target: Target, bodies: &[Body]) -> Result<(), Error> {
    // Native compatibility is already checked; this compares the emitted public declarations.
    if let Some(first) = bodies.first() {
        let key = first.key();
        if bodies[1..].iter().any(|body| body.key() != key) {
            return Err(Error(format!(
                "conflicting public names for `{name}` on {target:?}"
            )));
        }
    }
    Ok(())
}

fn callback(snapshot: &Snapshot, id: TypeId) -> Option<&Function> {
    if let TypeKind::Pointer(pointee) = snapshot.types[id.0].kind
        && let TypeKind::Function(function) = &snapshot.types[pointee.0].kind
    {
        Some(function)
    } else {
        None
    }
}

impl Projector<'_, '_> {
    fn alias(
        &self,
        snapshot_id: usize,
        observation: &AliasObservation,
        name: &str,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        if !publishable_alias(snapshot, observation.canonical) {
            return Err(Error(format!(
                "typedef `{name}` is not an unqualified value type"
            )));
        }
        let name = ident(name)?;
        if let Some(function) = callback(snapshot, observation.canonical) {
            self.ty(snapshot_id, observation.canonical)?;
            let mut id = observation.underlying;
            let written = loop {
                match &snapshot.types[id.0].kind {
                    TypeKind::Alias(alias) => {
                        id = snapshot.aliases[alias.0].observations[0].underlying;
                    }
                    TypeKind::Pointer(pointee) => id = *pointee,
                    TypeKind::Attributed { modified, .. } => id = *modified,
                    TypeKind::Function(function) => break function,
                    _ => return Err(Error("callback has no written prototype".into())),
                }
            };
            return self.callback(snapshot_id, function, written, &name);
        }
        let ty = write_type(
            self.namespace,
            &self.ty(snapshot_id, observation.canonical)?,
        );
        Ok(Body {
            attributes: TokenStream::new(),
            declaration: quote! { type #name = #ty; },
        })
    }

    fn callback(
        &self,
        snapshot: usize,
        function: &Function,
        written: &Function,
        name: &TokenStream,
    ) -> Result<Body, Error> {
        if function.parameters.len() != written.parameters.len() {
            return Err(Error("written and canonical callback arity differ".into()));
        }
        let abi = match function.convention {
            CallingConvention::C => quote! { "C" },
            CallingConvention::Stdcall => TokenStream::new(),
        };
        let parameters = function
            .parameters
            .iter()
            .zip(&written.parameters)
            .enumerate()
            .map(|(index, (id, written))| {
                let name = ident(&format!("param{index}"))?;
                let ty = write_type(self.namespace, &self.written_ty(snapshot, *written, *id)?);
                Ok(quote! { #name: #ty })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let result = self.written_ty(snapshot, written.result, function.result)?;
        let result = if result == MetadataType::Void {
            TokenStream::new()
        } else {
            let result = write_type(self.namespace, &result);
            quote! { -> #result }
        };
        Ok(Body {
            attributes: TokenStream::new(),
            declaration: quote! { extern #abi fn #name(#(#parameters),*) #result; },
        })
    }

    fn written_ty(
        &self,
        snapshot_id: usize,
        written: TypeId,
        canonical: TypeId,
    ) -> Result<MetadataType, Error> {
        let value = self.ty(snapshot_id, canonical)?;
        let snapshot = self.resolution.snapshots[snapshot_id];
        Ok(match &snapshot.types[written.0].kind {
            TypeKind::Alias(id)
                if publishable_alias(
                    snapshot,
                    snapshot.aliases[id.0].observations[0].canonical,
                ) =>
            {
                let name = &snapshot.aliases[id.0].name;
                if callback(snapshot, canonical).is_some() {
                    MetadataType::class_named(self.namespace, name)
                } else {
                    MetadataType::value_named(self.namespace, name)
                }
            }
            TypeKind::Pointer(written) => {
                let TypeKind::Pointer(canonical) = snapshot.types[canonical.0].kind else {
                    return Err(Error(
                        "written pointer has a non-pointer canonical type".into(),
                    ));
                };
                if matches!(snapshot.types[canonical.0].kind, TypeKind::Function(_)) {
                    value
                } else {
                    let inner = Box::new(self.written_ty(snapshot_id, *written, canonical)?);
                    if snapshot.types[canonical.0].qualifiers.constant {
                        MetadataType::PtrConst(inner, 1)
                    } else {
                        MetadataType::PtrMut(inner, 1)
                    }
                }
            }
            TypeKind::Array {
                element: written, ..
            } => {
                let TypeKind::Array {
                    element: canonical,
                    length,
                } = snapshot.types[canonical.0].kind
                else {
                    return Err(Error("written array has a non-array canonical type".into()));
                };
                MetadataType::ArrayFixed(
                    Box::new(self.written_ty(snapshot_id, *written, canonical)?),
                    length
                        .try_into()
                        .map_err(|_| Error("array extent exceeds host size".into()))?,
                )
            }
            TypeKind::Attributed { modified, .. } => {
                self.written_ty(snapshot_id, *modified, canonical)?
            }
            _ => value,
        })
    }

    fn ty(&self, snapshot_id: usize, id: TypeId) -> Result<MetadataType, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let ty = &snapshot.types[id.0];
        if ty.qualifiers.volatile || ty.qualifiers.restrict {
            return Err(Error(
                "volatile/restrict type projection is not implemented".into(),
            ));
        }
        Ok(match &ty.kind {
            TypeKind::Void => MetadataType::Void,
            TypeKind::Scalar(scalar) => {
                let (value, size) = match scalar {
                    Scalar::Bool => (MetadataType::Bool, 1),
                    Scalar::Char { signed: true } | Scalar::SignedChar => (MetadataType::I8, 1),
                    Scalar::Char { signed: false } | Scalar::UnsignedChar => (MetadataType::U8, 1),
                    Scalar::Short => (MetadataType::I16, 2),
                    Scalar::UnsignedShort | Scalar::WChar => (MetadataType::U16, 2),
                    Scalar::Int | Scalar::Long => (MetadataType::I32, 4),
                    Scalar::UnsignedInt | Scalar::UnsignedLong => (MetadataType::U32, 4),
                    Scalar::LongLong => (MetadataType::I64, 8),
                    Scalar::UnsignedLongLong => (MetadataType::U64, 8),
                    Scalar::Float => (MetadataType::F32, 4),
                    Scalar::Double | Scalar::LongDouble => (MetadataType::F64, 8),
                };
                if ty.layout
                    != Some(Layout {
                        size,
                        alignment: size,
                    })
                {
                    return Err(Error(format!(
                        "unsupported {scalar:?} layout on {:?}",
                        snapshot.target
                    )));
                }
                value
            }
            TypeKind::Pointer(pointee) => {
                let size = if snapshot.target == Target::X86 { 4 } else { 8 };
                if ty.layout
                    != Some(Layout {
                        size,
                        alignment: size,
                    })
                {
                    return Err(Error("unsupported pointer layout".into()));
                }
                if matches!(snapshot.types[pointee.0].kind, TypeKind::Function(_)) {
                    if snapshot.types[pointee.0].qualifiers != Qualifiers::default() {
                        return Err(Error(
                            "qualified function type projection is unsupported".into(),
                        ));
                    }
                    let name = self.plan.callbacks[snapshot_id].get(&id).ok_or_else(|| {
                        Error("function pointer has no named callback declaration".into())
                    })?;
                    return Ok(MetadataType::class_named(self.namespace, name));
                }
                let inner = self.ty(snapshot_id, *pointee)?;
                let constant = snapshot.types[pointee.0].qualifiers.constant;
                if matches!(
                    (&inner, constant),
                    (MetadataType::PtrMut(..), true) | (MetadataType::PtrConst(..), false)
                ) {
                    return Err(Error(
                        "mixed const/mutable pointer chains are not representable in RDL".into(),
                    ));
                }
                let inner = Box::new(inner);
                if constant {
                    MetadataType::PtrConst(inner, 1)
                } else {
                    MetadataType::PtrMut(inner, 1)
                }
            }
            TypeKind::Array { element, length } => MetadataType::ArrayFixed(
                Box::new(self.ty(snapshot_id, *element)?),
                (*length)
                    .try_into()
                    .map_err(|_| Error("array extent exceeds host size".into()))?,
            ),
            TypeKind::Record(record) => {
                let name = &self.plan.names[self.plan.family(snapshot_id, *record)];
                MetadataType::value_named(self.namespace, name)
            }
            TypeKind::Enum(id) => {
                let name = &self.plan.enum_names[self.resolution.enum_groups[snapshot_id][id.0]];
                MetadataType::value_named(self.namespace, name)
            }
            TypeKind::Alias(_) | TypeKind::Attributed { .. } => {
                return Err(Error("expected a canonical projection type".into()));
            }
            TypeKind::Function(_) => {
                return Err(Error("bare function type projection is unsupported".into()));
            }
        })
    }

    fn enumeration(
        &self,
        snapshot_id: usize,
        observation: &EnumObservation,
        name: &TokenStream,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        if !resolve::alignments(&observation.attributes).is_empty() {
            return Err(Error(format!(
                "aligned enum `{name}` projection is unsupported"
            )));
        }
        if snapshot.types[observation.canonical.0].layout != Some(observation.layout) {
            return Err(Error(format!(
                "enum `{name}` cannot use its backing type layout"
            )));
        }
        let repr = write_type(
            self.namespace,
            &self.ty(snapshot_id, observation.canonical)?,
        );
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        for member in observation.members.as_ref().unwrap() {
            let member_name = ident(&member.name)?;
            if !names.insert(member_name.to_string()) || member.name == "value__" {
                return Err(Error(format!(
                    "duplicate or reserved enum member `{name}.{member_name}`"
                )));
            }
            if !member.attributes.is_empty() {
                return Err(Error(format!(
                    "attributed enum member `{name}.{member_name}` is unsupported"
                )));
            }
            let value = match member.value {
                EnumValue::Signed(value) => windows_metadata::Value::I64(value),
                EnumValue::Unsigned(value) => windows_metadata::Value::U64(value),
            };
            let value = write_value(self.namespace, &value);
            fields.push(quote! { #member_name = #value, });
        }
        let scoped = observation.scoped.then(|| quote! { #[scoped] });
        Ok(Body {
            attributes: quote! { #[repr(#repr)] },
            declaration: quote! { #scoped enum #name { #(#fields)* } },
        })
    }

    fn record(
        &self,
        snapshot_id: usize,
        observation: &RecordObservation,
        name: &TokenStream,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let definition = observation.definition.as_ref().unwrap();
        let attributes = &observation.attributes;
        let kind = snapshot.records[observation.record.0].kind;
        if definition.fields.is_empty() {
            return Err(Error(format!(
                "empty record `{name}` projection is not implemented"
            )));
        }
        let (packed, maximum) = resolve::packing(attributes);
        let packing = if packed {
            Some(1)
        } else {
            maximum.map(|bits| u64::from(bits / 8))
        };
        let requested = resolve::alignments(attributes)
            .into_iter()
            .map(|(bits, _)| bits / 8)
            .max();
        let forced = requested.filter(|bytes| *bytes != 0);
        if forced.is_some_and(|bytes| bytes > u32::from(u16::MAX)) {
            return Err(Error(format!(
                "record `{name}` alignment exceeds metadata limits"
            )));
        }
        if packing.is_some() && forced.is_some() {
            return Err(Error(format!(
                "record `{name}` combines packing and alignment"
            )));
        }
        if attributes
            .iter()
            .any(|attribute| matches!(attribute.kind, AttributeKind::Unresolved(_)))
        {
            return Err(Error(format!("unresolved attributes on `{name}`")));
        }
        let mut size = 0;
        let mut alignment = 1;
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        for field in &definition.fields {
            let field_name = ident(&field.name)?;
            if !names.insert(field_name.to_string()) {
                return Err(Error(format!(
                    "duplicate projected field `{field_name}` in `{name}`"
                )));
            }
            let native = &snapshot.types[field.canonical.0];
            if !field.attributes.is_empty() || native.qualifiers.constant {
                return Err(Error(format!(
                    "attributed or const field `{name}.{field_name}` is unsupported"
                )));
            }
            let layout = native
                .layout
                .ok_or_else(|| Error(format!("field `{name}.{field_name}` has no layout")))?;
            if packing.is_some()
                && self.contains_alignment(snapshot_id, field.canonical, &mut HashSet::new())
            {
                return Err(Error(format!(
                    "packed record `{name}` contains an aligned type"
                )));
            }
            let field_alignment =
                packing.map_or(layout.alignment, |packing| layout.alignment.min(packing));
            let offset = if kind == RecordKind::Union {
                0
            } else {
                round_up(size, field_alignment)
            };
            if field.offset_bits != offset * 8 {
                return Err(Error(format!(
                    "field `{name}.{field_name}` cannot use metadata layout"
                )));
            }
            size = size.max(offset + layout.size);
            alignment = alignment.max(field_alignment);
            let ty = write_type(
                self.namespace,
                &self.written_ty(snapshot_id, field.written, field.canonical)?,
            );
            fields.push(quote! { #field_name: #ty, });
        }
        alignment = alignment.max(u64::from(forced.unwrap_or(1)));
        if definition.layout
            != (Layout {
                size: round_up(size, alignment),
                alignment,
            })
        {
            return Err(Error(format!("record `{name}` cannot use metadata layout")));
        }
        let keyword = match kind {
            RecordKind::Struct => quote! { struct },
            RecordKind::Union => quote! { union },
        };
        Ok(Body {
            attributes: if let Some(packing) = packing {
                let packing = proc_macro2::Literal::u64_unsuffixed(packing);
                quote! { #[packed(#packing)] }
            } else if let Some(forced) = forced {
                let forced = proc_macro2::Literal::u32_unsuffixed(forced);
                quote! { #[align(#forced)] }
            } else {
                TokenStream::new()
            },
            declaration: quote! { #keyword #name { #(#fields)* } },
        })
    }

    fn contains_alignment(
        &self,
        snapshot_id: usize,
        id: TypeId,
        seen: &mut HashSet<RecordId>,
    ) -> bool {
        let snapshot = self.resolution.snapshots[snapshot_id];
        match snapshot.types[id.0].kind {
            TypeKind::Array { element, .. } => self.contains_alignment(snapshot_id, element, seen),
            TypeKind::Record(id) if seen.insert(id) => {
                snapshot.records[id.0].observations.iter().any(|id| {
                    let observation = &snapshot.observations[id.0];
                    resolve::alignments(&observation.attributes)
                        .iter()
                        .any(|(bits, _)| *bits != 0)
                        || observation.definition.as_ref().is_some_and(|definition| {
                            definition.fields.iter().any(|field| {
                                self.contains_alignment(snapshot_id, field.canonical, seen)
                            })
                        })
                })
            }
            _ => false,
        }
    }
}

fn round_up(value: u64, alignment: u64) -> u64 {
    value.div_ceil(alignment) * alignment
}

fn ident(value: &str) -> Result<TokenStream, Error> {
    if value.is_empty()
        || !value.chars().enumerate().all(|(index, ch)| {
            ch == '_' || ch.is_ascii_alphabetic() || (index > 0 && ch.is_ascii_digit())
        })
    {
        return Err(Error(format!("unsupported RDL identifier `{value}`")));
    }
    Ok(write_ident(value))
}

type Items = BTreeMap<String, (String, BTreeMap<String, TokenStream>)>;

fn insert(
    items: &mut Items,
    headers: &BTreeSet<String>,
    name: &str,
    body: TokenStream,
) -> Result<(), Error> {
    if headers.len() != 1 {
        return Err(Error(format!(
            "ambiguous header ownership for `{name}`: {headers:?}"
        )));
    }
    let header = headers.first().unwrap();
    let stem = std::path::Path::new(header)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| Error(format!("invalid header path `{header}`")))?;
    let stem: String = stem
        .chars()
        .filter(|ch| *ch != '.')
        .flat_map(char::to_lowercase)
        .collect();
    if stem.is_empty() {
        return Err(Error(format!("empty header partition for `{header}`")));
    }
    let (owner, declarations) = items
        .entry(stem.clone())
        .or_insert_with(|| (header.clone(), BTreeMap::new()));
    if owner != header {
        return Err(Error(format!(
            "headers `{owner}` and `{header}` collide in partition `{stem}`"
        )));
    }
    declarations.insert(name.to_string(), body);
    Ok(())
}

struct Body {
    attributes: TokenStream,
    declaration: TokenStream,
}

impl Body {
    fn key(&self) -> String {
        format!("{} {}", self.attributes, self.declaration)
    }
}

fn variants(bodies: BTreeMap<String, (Body, Vec<Target>)>) -> TokenStream {
    let mut rendered: Vec<_> = bodies
        .into_values()
        .map(|(body, targets)| {
            let arch = if targets.len() == 3 {
                TokenStream::new()
            } else {
                let names = targets.iter().map(|target| match target {
                    Target::X86 => quote! { X86 },
                    Target::X64 => quote! { X64 },
                    Target::Arm64 => quote! { Arm64 },
                });
                quote! { #[arch(#(#names)|*)] }
            };
            let Body {
                attributes,
                declaration,
            } = body;
            quote! { #attributes #arch #declaration }
        })
        .collect();
    rendered.sort_by_key(ToString::to_string);
    rendered.into_iter().collect()
}
