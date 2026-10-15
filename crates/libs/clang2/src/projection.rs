//! Native IR to Win32 RDL projection and a three-target pipeline entry point.

use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use windows_metadata::Type as MetadataType;
use windows_rdl::emit::{write_ident, write_type, write_value};

mod annotations;
mod bitfields;
mod plan;
mod values;
use plan::Plan;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[derive(Debug, Eq, PartialEq)]
pub struct Partition {
    pub headers: BTreeSet<String>,
    pub rdl: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Output {
    pub partitions: BTreeMap<String, Partition>,
    pub records: usize,
    pub enums: usize,
    /// Published aliases, including named callback declarations.
    pub aliases: usize,
    pub constants: usize,
    pub functions: usize,
    pub interfaces: usize,
}

/// DLL/export ownership is publication input, not a property inferred from a header or C symbol.
#[derive(Clone, Debug)]
pub struct Import {
    pub library: String,
    pub name: Option<String>,
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
    project_partial_with_imports(captured, namespace, &BTreeMap::new())
}

pub fn project_partial_with_imports(
    captured: PartialCapture,
    namespace: &str,
    imports: &BTreeMap<String, Import>,
) -> Result<PartialOutput, Error> {
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
                            || !resolve::sal(&observation.attributes).is_empty()
                            || !resolve::deprecations(&observation.attributes).is_empty()
                            || observation
                                .parameters
                                .iter()
                                .flatten()
                                .any(|parameter| !parameter.attributes.is_empty())
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
            + snapshot.values.len()
            + snapshot.roots.len()
    };
    loop {
        let remaining: usize = snapshots.iter().map(count).sum();
        let merged = merge(&snapshots).map_err(|error| Error(format!("merge: {error}")))?;
        let mut rejected = partial::Issues::default();
        let output = project_impl(&merged, namespace, imports, Some(&mut rejected))
            .map_err(|error| Error(format!("projection: {error}")))?;
        if rejected.excluded.is_empty() {
            return Ok(PartialOutput {
                output,
                omissions: issues.finish(),
            });
        }
        for (key, reason) in rejected.excluded {
            match &key {
                partial::Key::Tag(_) | partial::Key::Nested(_, _) | partial::Key::Value(_) => {
                    issues.reject(key, format!("projection: {reason}"));
                }
                partial::Key::Alias(name) => {
                    if published_aliases.contains(name) {
                        issues.reject(key, format!("projection: {reason}"));
                        continue;
                    }
                    // Non-value aliases such as void are expanded at use sites.
                    for snapshot in &mut snapshots {
                        snapshot.roots.retain(|id| match id {
                            DeclarationId::Alias(id) => &snapshot.aliases[id.0].name != name,
                            DeclarationId::Record(_)
                            | DeclarationId::Enum(_)
                            | DeclarationId::Value(_) => true,
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
/// Other reachable value names remain transparent aliases. Shared ownership uses foundation.rdl.
pub fn project(merged: &Merged<'_>, namespace: &str) -> Result<Output, Error> {
    project_with_imports(merged, namespace, &BTreeMap::new())
}

pub fn project_with_imports(
    merged: &Merged<'_>,
    namespace: &str,
    imports: &BTreeMap<String, Import>,
) -> Result<Output, Error> {
    project_impl(merged, namespace, imports, None)
}

fn project_impl(
    merged: &Merged<'_>,
    namespace: &str,
    imports: &BTreeMap<String, Import>,
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
                DeclarationId::Record(_) | DeclarationId::Enum(_) | DeclarationId::Value(_) => None,
            })
        })
        .collect();
    let mut declarations = BTreeMap::new();
    for (&target, resolution) in &merged.targets {
        let plan = Plan::new(resolution)?;
        let typedef_attributes = resolution
            .aliases
            .iter()
            .enumerate()
            .filter(|(id, alias)| {
                plan.aliases[*id].is_some_and(|family| plan.names[family] == alias.name)
                    || plan.enum_aliases[*id]
                        .is_some_and(|group| plan.enum_names[group] == alias.name)
            })
            .map(|(_, alias)| {
                Ok((
                    alias.name.as_str(),
                    annotations::write_typedef(resolution, alias)?,
                ))
            })
            .collect::<Result<HashMap<_, _>, Error>>()?;
        let projector = Projector {
            namespace,
            resolution,
            plan: &plan,
        };
        let mut published = HashSet::new();
        let mut constants = HashMap::new();
        for value in &resolution.values {
            let key = partial::Key::Value(value.name.clone());
            let first = value.observations[0];
            let snapshot = resolution.snapshots[first.snapshot];
            let observation = &snapshot.values[first.value.0].observations[first.observation];
            let kind = match &observation.kind {
                ValueKind::Function { .. } => PublishedKind::Function,
                ValueKind::Constant { value: number, .. } => {
                    values::check_constant(
                        &mut constants,
                        target,
                        &value.name,
                        &snapshot.types[observation.canonical.0],
                        number.clone(),
                    )?;
                    PublishedKind::Constant
                }
            };
            let mut headers = BTreeSet::new();
            let bodies = value
                .observations
                .iter()
                .map(|reference| {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.values[reference.value.0].observations[reference.observation];
                    headers.insert(snapshot.files[observation.location.file.0].clone());
                    projector.value(reference.snapshot, observation, &value.name, imports)
                })
                .collect::<Result<Vec<_>, _>>()
                .and_then(|bodies| {
                    agree(&value.name, target, &bodies)?;
                    Ok(bodies)
                });
            match bodies {
                Ok(bodies) => {
                    publish(
                        &mut declarations,
                        target,
                        &value.name,
                        bodies.into_iter().next().unwrap(),
                        headers,
                        vec![key],
                        kind,
                    )?;
                }
                Err(error) => reject(&mut rejected, [key], error)?,
            }
        }
        for (id, family) in plan.families.iter().enumerate() {
            if family.is_empty() || plan.owned[id] {
                continue;
            }
            let name = &plan.names[id];
            if !published.insert(name.as_str()) {
                return Err(Error(format!("duplicate projected name `{name}`")));
            }
            let keys: Vec<_> = family
                .iter()
                .map(|group| {
                    let reference = resolution.groups[*group].records[0];
                    partial::Key::of(
                        resolution.snapshots[reference.snapshot],
                        DeclarationId::Record(reference.record),
                    )
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
                                false,
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
                let mut body = bodies
                    .into_iter()
                    .next()
                    .ok_or_else(|| Error(format!("record `{name}` is incomplete")))?;
                if let Some(attributes) = typedef_attributes.get(name.as_str()) {
                    body.attributes.extend(attributes.clone());
                }
                Ok((body, plan.record_headers(resolution, id)?))
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
                        if family.iter().any(|group| {
                            resolution.groups[*group]
                                .definition
                                .is_some_and(|reference| {
                                    let observation = &resolution.snapshots[reference.snapshot]
                                        .observations[reference.observation.0];
                                    projector
                                        .interface_record(reference.snapshot, observation.record)
                                })
                        }) {
                            PublishedKind::Interface
                        } else {
                            PublishedKind::Record
                        },
                    )?;
                }
                Err(error) => reject(&mut rejected, keys, error)?,
            }
        }
        for (id, group) in resolution.enums.iter().enumerate() {
            let name = &plan.enum_names[id];
            if plan.enum_constants(resolution, id) {
                let first = group.enums[0];
                let key = partial::Key::Tag(
                    resolution.snapshots[first.snapshot].enums[first.enumeration.0]
                        .name
                        .clone(),
                );
                let Some(reference) = group.definition else {
                    reject(
                        &mut rejected,
                        [key],
                        Error(format!("enum `{name}` is incomplete")),
                    )?;
                    continue;
                };
                let snapshot = resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.enums[reference.enumeration.0].observations[reference.observation];
                if !annotations::write(&observation.attributes).is_empty() {
                    reject(
                        &mut rejected,
                        [key],
                        Error(format!(
                            "annotated anonymous enum `{name}` has no public annotation owner"
                        )),
                    )?;
                    continue;
                }
                let result = (|| {
                    Ok((
                        projector.enum_backing(reference.snapshot, observation, name)?,
                        projector.enum_fields(observation, name)?,
                        plan.enum_headers(resolution, id)?,
                    ))
                })();
                let (repr, fields, headers) = match result {
                    Ok(value) => value,
                    Err(error) => {
                        reject(&mut rejected, [key], error)?;
                        continue;
                    }
                };
                for (member, (attributes, member_name, value)) in
                    observation.members.as_ref().unwrap().iter().zip(fields)
                {
                    values::check_constant(
                        &mut constants,
                        target,
                        &member.name,
                        &snapshot.types[observation.canonical.0],
                        ConstantValue::Integer(member.value),
                    )?;
                    publish(
                        &mut declarations,
                        target,
                        &member.name,
                        Body {
                            attributes,
                            declaration: quote! { const #member_name: #repr = #value; },
                        },
                        headers.clone(),
                        vec![key.clone()],
                        PublishedKind::Constant,
                    )?;
                }
                continue;
            }
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
                let mut body =
                    projector.enumeration(reference.snapshot, observation, &ident(name)?)?;
                if let Some(attributes) = typedef_attributes.get(name.as_str()) {
                    body.attributes.extend(attributes.clone());
                }
                Ok((body, plan.enum_headers(resolution, id)?))
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
                )?,
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
            if !publishable_alias(snapshot, canonical)
                && alias.observations.iter().any(|reference| {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    callback_contracts(
                        snapshot,
                        &snapshot.aliases[reference.alias.0].observations[reference.observation],
                    )
                })
            {
                reject(
                    &mut rejected,
                    [partial::Key::Alias(alias.name.clone())],
                    Error(format!(
                        "annotated expanded typedef `{}` is unsupported",
                        alias.name
                    )),
                )?;
                continue;
            }
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
            let attributes = annotations::covering(
                &alias.name,
                alias.observations.iter().map(|reference| {
                    resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
                        [reference.observation]
                        .attributes
                        .as_slice()
                }),
            )?;
            let bodies = alias
                .observations
                .iter()
                .map(|reference| {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.aliases[reference.alias.0].observations[reference.observation];
                    projector.alias(reference.snapshot, observation, &alias.name, attributes)
                })
                .collect::<Result<Vec<_>, _>>();
            if let Ok(bodies) = &bodies {
                agree(&alias.name, target, bodies)?;
            }
            let result = bodies.and_then(|bodies| {
                Ok((
                    bodies.into_iter().next().unwrap(),
                    plan.alias_headers(resolution, alias)?,
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
                )?,
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
    let constants = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Constant)
        .count();
    let functions = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Function)
        .count();
    let interfaces = declarations
        .values()
        .filter(|item| item.kind == PublishedKind::Interface)
        .count();
    let aliases = declarations.len() - records - enums - constants - functions - interfaces;
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
    for (stem, (headers, declarations)) in items {
        let mut body: TokenStream = declarations.into_values().collect();
        for module in modules.iter().rev() {
            body = quote! { mod #module { #body } };
        }
        let text = quote! { #[win32] #body }.to_string();
        partitions.insert(
            stem,
            Partition {
                headers,
                rdl: windows_rdl::formatter::format(&text),
            },
        );
    }
    Ok(Output {
        partitions,
        records,
        enums,
        aliases,
        constants,
        functions,
        interfaces,
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
    Constant,
    Function,
    Interface,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum NameSpace {
    Type,
    Value,
}

type PublicName = (String, NameSpace);

fn publish(
    items: &mut BTreeMap<PublicName, Declaration>,
    target: Target,
    name: &str,
    body: Body,
    headers: BTreeSet<String>,
    keys: Vec<partial::Key>,
    kind: PublishedKind,
) -> Result<(), Error> {
    let space = if matches!(kind, PublishedKind::Constant | PublishedKind::Function) {
        NameSpace::Value
    } else {
        NameSpace::Type
    };
    let item = items.entry((name.to_string(), space)).or_default();
    let key = body.key();
    if item
        .bodies
        .iter()
        .any(|(old, (_, targets))| old != &key && targets.contains(&target))
    {
        return Err(Error(format!(
            "conflicting public definitions for `{name}` on {target:?}"
        )));
    }
    item.headers.extend(headers);
    item.keys.extend(keys);
    item.kind = item.kind.max(kind);
    let targets = &mut item
        .bodies
        .entry(key)
        .or_insert_with(|| (body, Vec::new()))
        .1;
    if !targets.contains(&target) {
        targets.push(target);
    }
    Ok(())
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
        if let Some(other) = bodies[1..].iter().find(|body| body.key() != key) {
            return Err(Error(format!(
                "conflicting public names or contracts for `{name}` on {target:?}\nfirst: {key:?}\nother: {:?}",
                other.key(),
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

fn written_function(snapshot: &Snapshot, mut id: TypeId) -> Result<&Function, Error> {
    loop {
        match &snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => id = snapshot.aliases[alias.0].observations[0].underlying,
            TypeKind::Pointer(pointee) => id = *pointee,
            TypeKind::Attributed { modified, .. } => id = *modified,
            TypeKind::Function(function) => return Ok(function),
            _ => return Err(Error("callback has no written prototype".into())),
        }
    }
}

fn callback_parameters<'a>(
    snapshot: &'a Snapshot,
    observation: &'a AliasObservation,
) -> Option<&'a [Parameter]> {
    if let Some(parameters) = &observation.parameters {
        return Some(parameters);
    }
    let mut id = observation.underlying;
    loop {
        match snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => {
                return callback_parameters(snapshot, &snapshot.aliases[alias.0].observations[0]);
            }
            TypeKind::Pointer(inner)
            | TypeKind::Attributed {
                modified: inner, ..
            } => id = inner,
            _ => return None,
        }
    }
}

fn callback_contracts(snapshot: &Snapshot, observation: &AliasObservation) -> bool {
    if !observation.attributes.is_empty()
        || callback_parameters(snapshot, observation).is_some_and(|parameters| {
            parameters
                .iter()
                .any(|parameter| !parameter.attributes.is_empty())
        })
    {
        return true;
    }
    let mut id = observation.underlying;
    loop {
        match snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => {
                return callback_contracts(snapshot, &snapshot.aliases[alias.0].observations[0]);
            }
            TypeKind::Pointer(inner)
            | TypeKind::Attributed {
                modified: inner, ..
            } => id = inner,
            _ => return false,
        }
    }
}

impl Projector<'_, '_> {
    fn alias(
        &self,
        snapshot_id: usize,
        observation: &AliasObservation,
        name: &str,
        attributes: &[Attribute],
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
            let written = written_function(snapshot, observation.underlying)?;
            let mut body = self.callback(
                snapshot_id,
                function,
                written,
                &name,
                callback_parameters(snapshot, observation),
            )?;
            body.attributes.extend(annotations::write(attributes));
            body.attributes.extend(annotations::inherited_callback(
                self.resolution,
                snapshot,
                observation.underlying,
            )?);
            return Ok(body);
        }
        if observation.parameters.as_ref().is_some_and(|parameters| {
            parameters
                .iter()
                .any(|parameter| !parameter.attributes.is_empty())
        }) {
            return Err(Error(format!(
                "nested callback contract on `{name}` has no metadata owner"
            )));
        }
        let ty = if let TypeKind::Record(record) = snapshot.types[observation.canonical.0].kind
            && self.interface_record(snapshot_id, record)
        {
            MetadataType::class_named(
                self.namespace,
                &self.plan.names[self.plan.family(snapshot_id, record)],
            )
        } else if callback_parameters(snapshot, observation).is_some()
            && callback_contracts(snapshot, observation)
        {
            self.written_ty(snapshot_id, observation.underlying, observation.canonical)?
        } else {
            self.ty(snapshot_id, observation.canonical)?
        };
        let ty = write_type(self.namespace, &ty);
        Ok(Body {
            attributes: annotations::write(attributes),
            declaration: quote! { type #name = #ty; },
        })
    }

    fn callback(
        &self,
        snapshot: usize,
        function: &Function,
        written: &Function,
        name: &TokenStream,
        declarations: Option<&[Parameter]>,
    ) -> Result<Body, Error> {
        let signature = self.signature(snapshot, function, Some(written))?;
        let abi = match function.convention {
            CallingConvention::C => quote! { "C" },
            CallingConvention::Stdcall => TokenStream::new(),
        };
        let parameters = if let Some(declarations) = declarations {
            self.parameters(snapshot, function, declarations)?
        } else {
            signature
                .types
                .iter()
                .enumerate()
                .map(|(index, ty)| {
                    let name = ident(&format!("param{index}"))?;
                    let ty = write_type(self.namespace, ty);
                    Ok(quote! { #name: #ty })
                })
                .collect::<Result<Vec<_>, Error>>()?
        };
        let result = signature.return_type;
        let result = if result == MetadataType::Void {
            TokenStream::new()
        } else {
            let result = write_type(self.namespace, &result);
            quote! { -> #result }
        };
        Ok(Body {
            attributes: annotations::function(written),
            declaration: quote! { extern #abi fn #name(#(#parameters),*) #result; },
        })
    }

    fn signature(
        &self,
        snapshot: usize,
        function: &Function,
        written: Option<&Function>,
    ) -> Result<windows_metadata::Signature, Error> {
        self.check_function_abi(snapshot, function)?;
        if function.method_qualifiers != Qualifiers::default() {
            return Err(Error(
                "method qualifiers are unsupported on callbacks".into(),
            ));
        }
        if written.is_some_and(|written| function.parameters.len() != written.parameters.len()) {
            return Err(Error("written and canonical callback arity differ".into()));
        }
        let flags = match function.convention {
            CallingConvention::C => windows_metadata::MethodCallAttributes::C,
            CallingConvention::Stdcall => windows_metadata::MethodCallAttributes::STDCALL,
        };
        let types = function
            .parameters
            .iter()
            .enumerate()
            .map(|(index, id)| {
                if let Some(written) = written {
                    self.parameter_ty(snapshot, written.parameters[index], *id)
                } else {
                    self.ty(snapshot, *id)
                }
            })
            .collect::<Result<_, _>>()?;
        let return_type = if let Some(written) = written {
            self.written_ty(snapshot, written.result, function.result)?
        } else {
            self.ty(snapshot, function.result)?
        };
        Ok(windows_metadata::Signature {
            flags,
            return_type,
            types,
        })
    }

    fn check_function_abi(&self, snapshot: usize, function: &Function) -> Result<(), Error> {
        for &ty in function
            .parameters
            .iter()
            .chain(std::iter::once(&function.result))
        {
            if self.inherits_by_value(snapshot, ty, &mut HashSet::new()) {
                return Err(Error(
                    "by-value C++ inheritance requires a calling-ABI contract".into(),
                ));
            }
        }
        Ok(())
    }

    fn inherits_by_value(
        &self,
        snapshot: usize,
        ty: TypeId,
        seen: &mut HashSet<RecordRef>,
    ) -> bool {
        match self.resolution.snapshots[snapshot].types[ty.0].kind {
            TypeKind::Array { element, .. } => self.inherits_by_value(snapshot, element, seen),
            TypeKind::Record(record) => {
                let record = RecordRef { snapshot, record };
                seen.insert(record)
                    && self
                        .resolution
                        .group(record)
                        .definition
                        .is_some_and(|reference| {
                            let definition = self.resolution.snapshots[reference.snapshot]
                                .observations[reference.observation.0]
                                .definition
                                .as_ref()
                                .unwrap();
                            !definition.bases.is_empty()
                                || definition.fields.iter().any(|field| {
                                    self.inherits_by_value(
                                        reference.snapshot,
                                        field.canonical,
                                        seen,
                                    )
                                })
                        })
            }
            _ => false,
        }
    }

    fn parameter_ty(
        &self,
        snapshot_id: usize,
        written: TypeId,
        canonical: TypeId,
    ) -> Result<MetadataType, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let mut source = written;
        loop {
            match snapshot.types[source.0].kind {
                TypeKind::Alias(alias) => {
                    source = snapshot.aliases[alias.0].observations[0].underlying;
                }
                TypeKind::Attributed { modified, .. } => source = modified,
                _ => break,
            }
        }
        if let TypeKind::Function(written) = &snapshot.types[source.0].kind
            && let Some(function) = callback(snapshot, canonical)
        {
            let value = self.ty(snapshot_id, canonical)?;
            return if matches!(value, MetadataType::FnPtr(_)) {
                Ok(MetadataType::FnPtr(Box::new(self.signature(
                    snapshot_id,
                    function,
                    Some(written),
                )?)))
            } else {
                Ok(value)
            };
        }
        if let TypeKind::Array { element, .. } = snapshot.types[source.0].kind
            && let TypeKind::Pointer(pointee) = snapshot.types[canonical.0].kind
        {
            self.ty(snapshot_id, canonical)?;
            let inner = Box::new(self.written_ty(snapshot_id, element, pointee)?);
            return Ok(if snapshot.types[pointee.0].qualifiers.constant {
                MetadataType::PtrConst(inner, 1)
            } else {
                MetadataType::PtrMut(inner, 1)
            });
        }
        if let TypeKind::LValueReference(pointee) = snapshot.types[canonical.0].kind {
            let TypeKind::LValueReference(written) = snapshot.types[source.0].kind else {
                return Err(Error(
                    "written reference has a non-reference canonical type".into(),
                ));
            };
            let inner = Box::new(self.written_ty(snapshot_id, written, pointee)?);
            return Ok(if snapshot.types[pointee.0].qualifiers.constant {
                MetadataType::RefConst(inner)
            } else {
                MetadataType::RefMut(inner)
            });
        }
        self.written_ty(snapshot_id, written, canonical)
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
                if let TypeKind::Record(record) = snapshot.types[canonical.0].kind
                    && self.interface_record(snapshot_id, record)
                {
                    return Ok(value);
                }
                if let TypeKind::Function(function) = &snapshot.types[canonical.0].kind {
                    if matches!(value, MetadataType::FnPtr(_)) {
                        MetadataType::FnPtr(Box::new(self.signature(
                            snapshot_id,
                            function,
                            Some(written_function(snapshot, *written)?),
                        )?))
                    } else {
                        value
                    }
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
                        .ok_or_else(|| {
                            Error("incomplete arrays require parameter pointer adjustment".into())
                        })?
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
                    Scalar::UnsignedShort | Scalar::WChar | Scalar::Char16 => {
                        (MetadataType::U16, 2)
                    }
                    Scalar::Int | Scalar::Long => (MetadataType::I32, 4),
                    Scalar::UnsignedInt | Scalar::UnsignedLong | Scalar::Char32 => {
                        (MetadataType::U32, 4)
                    }
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
                if let TypeKind::Function(function) = &snapshot.types[pointee.0].kind {
                    if snapshot.types[pointee.0].qualifiers != Qualifiers::default() {
                        return Err(Error(
                            "qualified function type projection is unsupported".into(),
                        ));
                    }
                    return if let Some(name) = self.plan.callbacks[snapshot_id].get(&id) {
                        Ok(MetadataType::class_named(self.namespace, name))
                    } else if function.exceptions != ExceptionSpecification::None {
                        Err(Error(
                            "noexcept function pointer requires a named callback to retain its contract"
                                .into(),
                        ))
                    } else {
                        Ok(MetadataType::FnPtr(Box::new(self.signature(
                            snapshot_id,
                            function,
                            None,
                        )?)))
                    };
                }
                if let TypeKind::Record(record) = snapshot.types[pointee.0].kind
                    && self.interface_record(snapshot_id, record)
                {
                    let qualifiers = &snapshot.types[pointee.0].qualifiers;
                    if qualifiers.volatile || qualifiers.restrict {
                        return Err(Error(
                            "volatile/restrict interface projection is not implemented".into(),
                        ));
                    }
                    // COM references do not express C++ object constness. Outer pointer
                    // qualifiers still describe pointer storage and are projected normally.
                    return Ok(MetadataType::class_named(
                        self.namespace,
                        &self.plan.names[self.plan.family(snapshot_id, record)],
                    ));
                }
                let inner = self.ty(snapshot_id, *pointee)?;
                let constant = snapshot.types[pointee.0].qualifiers.constant;
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
                    .ok_or_else(|| {
                        Error("incomplete arrays require parameter pointer adjustment".into())
                    })?
                    .try_into()
                    .map_err(|_| Error("array extent exceeds host size".into()))?,
            ),
            TypeKind::Record(record) => {
                if self.interface_record(snapshot_id, *record) {
                    return Err(Error("interface values require a native pointer".into()));
                }
                let name = &self.plan.names[self.plan.family(snapshot_id, *record)];
                MetadataType::value_named(self.namespace, name)
            }
            TypeKind::Enum(id) => {
                let group = self.resolution.enum_groups[snapshot_id][id.0];
                if self.plan.enum_constants(self.resolution, group) {
                    return Err(Error("anonymous enum has no public type name".into()));
                }
                let name = &self.plan.enum_names[group];
                MetadataType::value_named(self.namespace, name)
            }
            TypeKind::Alias(_) | TypeKind::Attributed { .. } => {
                return Err(Error("expected a canonical projection type".into()));
            }
            TypeKind::Function(_) => {
                return Err(Error("bare function type projection is unsupported".into()));
            }
            TypeKind::LValueReference(_) => {
                return Err(Error(
                    "C++ references are supported only in parameter positions".into(),
                ));
            }
        })
    }

    fn enumeration(
        &self,
        snapshot_id: usize,
        observation: &EnumObservation,
        name: &TokenStream,
    ) -> Result<Body, Error> {
        if observation
            .members
            .as_ref()
            .unwrap()
            .iter()
            .any(|member| member.name == "value__")
        {
            return Err(Error(format!("reserved enum member `{name}.value__`")));
        }
        let repr = self.enum_backing(snapshot_id, observation, &name.to_string())?;
        let fields = self
            .enum_fields(observation, &name.to_string())?
            .into_iter()
            .map(|(attributes, name, value)| quote! { #attributes #name = #value, });
        let scoped = observation.scoped.then(|| quote! { #[scoped] });
        let native = annotations::write(&observation.attributes);
        Ok(Body {
            attributes: quote! { #native #[repr(#repr)] },
            declaration: quote! { #scoped enum #name { #(#fields)* } },
        })
    }

    fn enum_backing(
        &self,
        snapshot_id: usize,
        observation: &EnumObservation,
        name: &str,
    ) -> Result<TokenStream, Error> {
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
        Ok(write_type(
            self.namespace,
            &self.ty(snapshot_id, observation.canonical)?,
        ))
    }

    fn enum_fields(
        &self,
        observation: &EnumObservation,
        name: &str,
    ) -> Result<Vec<(TokenStream, TokenStream, TokenStream)>, Error> {
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        for member in observation.members.as_ref().unwrap() {
            let member_name = ident(&member.name)?;
            if !names.insert(member_name.to_string()) {
                return Err(Error(format!(
                    "duplicate enum member `{name}.{member_name}`"
                )));
            }
            if !annotations::only_contracts(&member.attributes) {
                return Err(Error(format!(
                    "attributed enum member `{name}.{member_name}` is unsupported"
                )));
            }
            let value = match member.value {
                EnumValue::Signed(value) => windows_metadata::Value::I64(value),
                EnumValue::Unsigned(value) => windows_metadata::Value::U64(value),
            };
            let value = write_value(self.namespace, &value);
            fields.push((annotations::write(&member.attributes), member_name, value));
        }
        Ok(fields)
    }

    fn record(
        &self,
        snapshot_id: usize,
        observation: &RecordObservation,
        name: &TokenStream,
        inline_record: bool,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let definition = observation.definition.as_ref().unwrap();
        let attributes = &observation.attributes;
        let kind = snapshot.records[observation.record.0].kind;
        if !definition.methods.is_empty() || self.interface_record(snapshot_id, observation.record)
        {
            return self.interface(snapshot_id, observation, name);
        }
        if attributes.iter().any(|attribute| {
            matches!(
                attribute.kind,
                AttributeKind::Uuid(_) | AttributeKind::NoVTable | AttributeKind::DllImport
            )
        }) {
            return Err(Error(format!(
                "unsupported ordinary record attributes on `{name}`"
            )));
        }
        if definition.fields.is_empty() && definition.bases.is_empty() {
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
            if !field.name.is_empty() && !names.insert(ident(&field.name)?.to_string()) {
                return Err(Error(format!(
                    "duplicate projected field `{}` in `{name}`",
                    field.name
                )));
            }
            let native = &snapshot.types[field.canonical.0];
            if !annotations::only_contracts(&field.attributes)
                || native.qualifiers.constant
                || field.bit_width.is_some() && !field.attributes.is_empty()
            {
                return Err(Error(format!(
                    "attributed or const field `{name}.{}` is unsupported",
                    field.name
                )));
            }
            if packing.is_some()
                && self.contains_alignment(snapshot_id, field.canonical, &mut HashSet::new())
            {
                return Err(Error(format!(
                    "packed record `{name}` contains an aligned type"
                )));
            }
        }
        if !definition.bases.is_empty() {
            let [base] = definition.bases.as_slice() else {
                return Err(Error(format!("record `{name}` has multiple value bases")));
            };
            if base.offset_bits != 0 || self.interface_record(snapshot_id, base.record) {
                return Err(Error(format!(
                    "record `{name}` requires a value base at offset zero"
                )));
            }
            if !names.insert("base".into()) {
                return Err(Error(format!("base field name collides in `{name}`")));
            }
            let reference = self
                .resolution
                .group(RecordRef {
                    snapshot: snapshot_id,
                    record: base.record,
                })
                .definition
                .ok_or_else(|| Error(format!("record `{name}` has an incomplete base")))?;
            let base_observation = &self.resolution.snapshots[reference.snapshot].observations
                [reference.observation.0];
            if packing.is_some()
                && self.record_contains_alignment(
                    reference.snapshot,
                    base_observation.record,
                    &mut HashSet::new(),
                )
            {
                return Err(Error(format!(
                    "packed record `{name}` contains an aligned base"
                )));
            }
            let layout = base_observation.definition.as_ref().unwrap().layout;
            size = layout.size;
            alignment = packing.map_or(layout.alignment, |packing| layout.alignment.min(packing));
            let ty = write_type(
                self.namespace,
                &MetadataType::value_named(
                    self.namespace,
                    &self.plan.names[self.plan.family(snapshot_id, base.record)],
                ),
            );
            fields.push(quote! { #[Windows::Win32::Metadata::NativeBase] base: #ty, });
        }
        let groups = bitfields::groups(snapshot, &definition.fields, kind)?;
        let mut nested = BTreeMap::new();
        for &record in &self.plan.children[snapshot_id][observation.record.0] {
            let full_name = &self.plan.names[self.plan.family(snapshot_id, record)];
            let leaf = full_name.rsplit('/').next().unwrap();
            if nested.contains_key(leaf) {
                return Err(Error(format!(
                    "nested type name collision in `{name}`: {leaf}"
                )));
            }
            let child = snapshot.records[record.0]
                .observations
                .iter()
                .find(|id| snapshot.observations[id.0].definition.is_some())
                .map(|&observation| ObservationRef {
                    snapshot: snapshot_id,
                    observation,
                })
                .or_else(|| {
                    self.resolution
                        .group(RecordRef {
                            snapshot: snapshot_id,
                            record,
                        })
                        .definition
                })
                .ok_or_else(|| Error(format!("nested record `{full_name}` is incomplete")))?;
            nested.insert(leaf, (record, child));
        }
        let mut direct_uses: HashMap<RecordId, Vec<usize>> = HashMap::new();
        for (position, group) in groups.iter().enumerate() {
            let field = &group[0];
            if field.bit_width.is_none()
                && let TypeKind::Record(record) = snapshot.types[field.canonical.0].kind
                && snapshot.records[record.0]
                    .owner
                    .is_some_and(|owner| owner.record == observation.record)
                && self.written_ty(snapshot_id, field.written, field.canonical)?
                    == self.ty(snapshot_id, field.canonical)?
            {
                direct_uses.entry(record).or_default().push(position);
            }
        }
        let inline: HashSet<_> = nested
            .iter()
            .filter_map(|(leaf, (record, _))| {
                if let Some(positions) = direct_uses.get(record)
                    && let [position] = positions.as_slice()
                    && *leaf
                        == format!(
                            "{}_{position}",
                            self.plan.names[self.plan.family(snapshot_id, observation.record)]
                                .rsplit('/')
                                .next()
                                .unwrap()
                        )
                {
                    Some(*record)
                } else {
                    None
                }
            })
            .collect();
        let declarations = nested
            .iter()
            .filter(|(_, (record, _))| !inline.contains(record))
            .map(|(leaf, (_, reference))| {
                let observation = &self.resolution.snapshots[reference.snapshot].observations
                    [reference.observation.0];
                let body = self.record(reference.snapshot, observation, &ident(leaf)?, false)?;
                let Body {
                    attributes,
                    declaration,
                } = body;
                Ok(quote! { #attributes #declaration })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let bitfield_count = groups
            .iter()
            .filter(|group| group[0].bit_width.is_some())
            .count();
        let mut bitfield_index = 0;
        let mut anonymous = 0;
        for (position, group) in groups.into_iter().enumerate() {
            let field = &group[0];
            let (field_name, ty, members) = if field.bit_width.is_some() {
                bitfield_index += 1;
                let backing = if bitfield_count == 1 {
                    "_bitfield".to_string()
                } else {
                    format!("_bitfield{bitfield_index}")
                };
                if !names.insert(backing.clone()) {
                    return Err(Error(format!(
                        "bitfield backing `{backing}` collides in `{name}`"
                    )));
                }
                let members = group
                    .iter()
                    .rposition(|member| !member.name.is_empty())
                    .map(|last| {
                        let members = group[..=last].iter().map(|member| {
                            let member_name = if member.name.is_empty() {
                                quote! { _ }
                            } else {
                                ident(&member.name).unwrap()
                            };
                            let width =
                                proc_macro2::Literal::u32_unsuffixed(member.bit_width.unwrap());
                            quote! { #member_name: #width, }
                        });
                        quote! { { #(#members)* } }
                    });
                (
                    ident(&backing)?,
                    write_type(
                        self.namespace,
                        &self.ty(
                            snapshot_id,
                            group
                                .iter()
                                .find(|member| !member.name.is_empty())
                                .unwrap_or(field)
                                .canonical,
                        )?,
                    ),
                    quote! { #members },
                )
            } else {
                let field_name = if field.name.is_empty() {
                    anonymous += 1;
                    let field_name = if anonymous == 1 {
                        "Anonymous".to_string()
                    } else {
                        format!("Anonymous{anonymous}")
                    };
                    if !names.insert(field_name.clone()) {
                        return Err(Error(format!(
                            "anonymous field name collision in `{name}`: {field_name}"
                        )));
                    }
                    ident(&field_name)?
                } else {
                    ident(&field.name)?
                };
                let ty = if let TypeKind::Record(record) = snapshot.types[field.canonical.0].kind
                    && inline.contains(&record)
                    && direct_uses[&record][0] == position
                {
                    let (_, reference) = nested.values().find(|(id, _)| *id == record).unwrap();
                    let child = &self.resolution.snapshots[reference.snapshot].observations
                        [reference.observation.0];
                    let full_name = &self.plan.names[self.plan.family(snapshot_id, record)];
                    let leaf = ident(full_name.rsplit('/').next().unwrap())?;
                    let body = self.record(reference.snapshot, child, &leaf, true)?;
                    let Body {
                        attributes,
                        declaration,
                    } = body;
                    quote! { #attributes #declaration }
                } else {
                    write_type(
                        self.namespace,
                        &self.written_ty(snapshot_id, field.written, field.canonical)?,
                    )
                };
                (field_name, ty, TokenStream::new())
            };
            let layout = snapshot.types[field.canonical.0]
                .layout
                .ok_or_else(|| Error(format!("field `{name}.{field_name}` has no layout")))?;
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
            let attributes = annotations::write(&field.attributes);
            fields.push(quote! { #attributes #field_name: #ty #members, });
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
        let declaration_name = (!inline_record).then_some(name);
        let native_attributes = annotations::write(attributes);
        let layout_attributes = if let Some(packing) = packing {
            let packing = proc_macro2::Literal::u64_unsuffixed(packing);
            quote! { #[packed(#packing)] }
        } else if let Some(forced) = forced {
            let forced = proc_macro2::Literal::u32_unsuffixed(forced);
            quote! { #[align(#forced)] }
        } else {
            TokenStream::new()
        };
        Ok(Body {
            attributes: quote! { #native_attributes #layout_attributes },
            declaration: quote! { #keyword #declaration_name { #(#declarations)* #(#fields)* } },
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
            TypeKind::Record(id) => self.record_contains_alignment(snapshot_id, id, seen),
            _ => false,
        }
    }

    fn record_contains_alignment(
        &self,
        snapshot_id: usize,
        id: RecordId,
        seen: &mut HashSet<RecordId>,
    ) -> bool {
        let snapshot = self.resolution.snapshots[snapshot_id];
        seen.insert(id)
            && snapshot.records[id.0].observations.iter().any(|id| {
                let observation = &snapshot.observations[id.0];
                resolve::alignments(&observation.attributes)
                    .iter()
                    .any(|(bits, _)| *bits != 0)
                    || observation.definition.as_ref().is_some_and(|definition| {
                        definition.fields.iter().any(|field| {
                            self.contains_alignment(snapshot_id, field.canonical, seen)
                        }) || definition.bases.iter().any(|base| {
                            self.record_contains_alignment(snapshot_id, base.record, seen)
                        })
                    })
            })
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

type Items = BTreeMap<String, (BTreeSet<String>, BTreeMap<PublicName, TokenStream>)>;

fn insert(
    items: &mut Items,
    headers: &BTreeSet<String>,
    name: &PublicName,
    body: TokenStream,
) -> Result<(), Error> {
    if headers.is_empty() {
        return Err(Error(format!("no header ownership for `{}`", name.0)));
    }
    let shared = headers.len() > 1;
    let stem = if shared {
        "foundation".to_string()
    } else {
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
        if stem == "foundation" {
            return Err(Error(format!(
                "header `{header}` collides with reserved foundation partition"
            )));
        }
        stem
    };
    let (owners, declarations) = items
        .entry(stem.clone())
        .or_insert_with(|| (headers.clone(), BTreeMap::new()));
    if !shared && owners != headers {
        return Err(Error(format!(
            "headers {owners:?} and {headers:?} collide in partition `{stem}`"
        )));
    }
    owners.extend(headers.iter().cloned());
    declarations.insert(name.clone(), body);
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
