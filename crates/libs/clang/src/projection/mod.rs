//! Native IR to Win32 RDL projection and a three-target pipeline entry point.

use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::cell::RefCell;
use std::collections::BTreeSet;
use windows_metadata::Type as MetadataType;
use windows_rdl::emit::{write_ident, write_param_direction, write_value};

mod annotations;
mod bitfields;
mod callbacks;
mod integer_expression;
mod native_object;
mod plan;
mod publication;
mod records;
mod reference_values;
mod references;
mod values;
use callbacks::abis_compatible;
use plan::Plan;
pub use references::References;

#[derive(Default)]
pub struct Options<'a> {
    pub imports: Option<&'a Imports>,
    pub references: Option<&'a References<'a>>,
}

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

#[derive(Clone, Debug, Default)]
pub struct Imports {
    /// Explicit public-name mappings take precedence on every target.
    pub functions: BTreeMap<String, Import>,
    /// SDK mappings use exact compiler linker symbols within each target.
    pub symbols: BTreeMap<Target, BTreeMap<String, Import>>,
    /// Approved compiler-derived alternatives, keyed by exact candidate linker symbol.
    pub default_stdcall: BTreeMap<Target, BTreeMap<String, Import>>,
}

impl FromIterator<(String, Import)> for Imports {
    fn from_iter<T: IntoIterator<Item = (String, Import)>>(iter: T) -> Self {
        Self {
            functions: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PartialOutput {
    pub output: Output,
    pub omissions: Vec<String>,
}

/// Projects the supported subset with explicit omissions. Compiler and correspondence errors are fatal.
/// Contract failures omit an entity on all targets; unresolved imports are reported per target.
pub fn generate_partial(inputs: &[Input], namespace: &str) -> Result<PartialOutput, Error> {
    let captured =
        capture_partial(inputs, &TARGETS).map_err(|error| Error(format!("capture: {error}")))?;
    project_partial(captured, namespace)
}

/// Projects an existing partial capture without reparsing its headers.
pub fn project_partial(captured: PartialCapture, namespace: &str) -> Result<PartialOutput, Error> {
    project_partial_with_imports(captured, namespace, &Imports::default())
}

pub fn project_partial_with_imports(
    captured: PartialCapture,
    namespace: &str,
    imports: &Imports,
) -> Result<PartialOutput, Error> {
    project_partial_with_options(
        captured,
        namespace,
        &Options {
            imports: Some(imports),
            ..Default::default()
        },
    )
}

pub fn project_partial_with_options(
    captured: PartialCapture,
    namespace: &str,
    options: &Options<'_>,
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
                        publishable_alias(snapshot, observation)
                            || !resolve::alignments(&observation.attributes).is_empty()
                            || !resolve::sal(&observation.attributes).is_empty()
                            || !resolve::deprecations(&observation.attributes).is_empty()
                            || observation
                                .parameters
                                .iter()
                                .flatten()
                                .any(annotations::parameter_is_annotated)
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
        let output = project_impl(&merged, namespace, options, Some(&mut rejected))
            .map_err(|error| Error(format!("projection: {error}")))?;
        if rejected.excluded.is_empty() {
            issues.messages.extend(rejected.messages);
            return Ok(PartialOutput {
                output,
                omissions: issues.finish(),
            });
        }
        for (key, reason) in rejected.excluded {
            match &key {
                partial::Key::Tag(_)
                | partial::Key::Nested(_, _)
                | partial::Key::Value(_)
                | partial::Key::Macro(_) => {
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
/// Other reachable value names remain transparent aliases. Shared ownership uses shared.rdl.
pub fn project(merged: &Merged<'_>, namespace: &str) -> Result<Output, Error> {
    project_with_imports(merged, namespace, &Imports::default())
}

pub fn project_with_imports(
    merged: &Merged<'_>,
    namespace: &str,
    imports: &Imports,
) -> Result<Output, Error> {
    project_with_options(
        merged,
        namespace,
        &Options {
            imports: Some(imports),
            ..Default::default()
        },
    )
}

pub fn project_with_options(
    merged: &Merged<'_>,
    namespace: &str,
    options: &Options<'_>,
) -> Result<Output, Error> {
    project_impl(merged, namespace, options, None)
}

/// Whether the enum is a bit set: a `flag_enum` attribute or a header-declared `operator|`.
fn flags(observation: &EnumObservation) -> bool {
    observation.bitwise
        || observation
            .attributes
            .iter()
            .any(|attribute| attribute.kind == AttributeKind::FlagEnum)
}

fn project_impl(
    merged: &Merged<'_>,
    namespace: &str,
    options: &Options<'_>,
    mut rejected: Option<&mut partial::Issues>,
) -> Result<Output, Error> {
    let default_imports = Imports::default();
    let imports = options.imports.unwrap_or(&default_imports);
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
    let mut external_names = BTreeSet::new();
    let native_sized = native_sized_aliases(merged);
    // X86 is projected first, supplying the same ABI anchor used by variants.
    let mut x86_callbacks = HashMap::new();
    for (&target, resolution) in &merged.targets {
        let plan = Plan::new(resolution)?;
        let external = options
            .references
            .map(|references| references.bind(resolution, &plan, namespace))
            .transpose()?
            .unwrap_or_default();
        external_names.extend(
            external
                .definitions()
                .filter(|definition| definition.namespace() == namespace)
                .map(|definition| definition.name().to_string()),
        );
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
        let interface_ids = interface_ids(resolution)?;
        let projector = Projector {
            namespace,
            resolution,
            plan: &plan,
            external: &external,
            native_sized: &native_sized,
            interface_ids: &interface_ids,
            x86_callbacks: &x86_callbacks,
            callback_abis: RefCell::new(HashMap::new()),
        };
        projector.validate_references(&mut rejected)?;
        if rejected
            .as_deref()
            .is_some_and(|issues| !issues.excluded.is_empty())
        {
            continue;
        }
        let mut published = HashSet::new();
        let mut constants = HashMap::new();
        let unpublished_values = publication::values(
            &projector,
            target,
            imports,
            &mut declarations,
            &mut constants,
            &mut rejected,
        )?;
        publication::uuid_tags(
            &projector,
            target,
            &unpublished_values,
            &mut declarations,
            &mut rejected,
        )?;
        publication::records(
            &projector,
            target,
            &typedef_attributes,
            &mut published,
            &mut declarations,
            &mut rejected,
        )?;
        publication::enums(
            &projector,
            target,
            &typedef_attributes,
            &mut published,
            &mut constants,
            &mut declarations,
            &mut rejected,
        )?;
        publication::aliases(
            &projector,
            target,
            &root_aliases,
            &mut published,
            &mut declarations,
            &mut rejected,
        )?;
        if target == Target::X86 {
            x86_callbacks.extend(declarations.values().filter_map(|declaration| {
                if declaration.kind != PublishedKind::Alias {
                    return None;
                }
                let body = declaration.bodies.get(&Target::X86)?;
                Some((
                    declaration.native_name.clone(),
                    (body.contract_key(), body.abi?.convention),
                ))
            }));
        }
    }
    if rejected
        .as_deref()
        .is_none_or(|issues| issues.excluded.is_empty())
    {
        for name in external_names {
            if declarations.contains_key(&(name.clone(), NameSpace::Type)) {
                return Err(Error(format!(
                    "local type `{name}` collides with external metadata"
                )));
            }
        }
    }
    publication::output(declarations, &modules, &mut rejected)
}

#[derive(Default)]
struct Declaration {
    native_name: String,
    headers: BTreeSet<String>,
    bodies: BTreeMap<Target, Body>,
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
    mut body: Body,
    headers: BTreeSet<String>,
    keys: Vec<partial::Key>,
    kind: PublishedKind,
) -> Result<(), Error> {
    let space = if matches!(kind, PublishedKind::Constant | PublishedKind::Function) {
        NameSpace::Value
    } else {
        NameSpace::Type
    };
    let item = items
        .entry((public_name(name).to_string(), space))
        .or_default();
    if !item.native_name.is_empty() && item.native_name != name {
        return Err(Error(format!(
            "public name `{}` collides between native declarations `{}` and `{name}`",
            public_name(name),
            item.native_name
        )));
    }
    item.native_name = name.to_string();
    if let Some(previous) = item.bodies.get(&target) {
        if previous.key() != body.key() {
            return Err(Error(format!(
                "conflicting public definitions for `{name}` on {target:?}"
            )));
        }
        body.retain_abi(previous);
    }
    item.headers.extend(headers);
    item.keys.extend(keys);
    item.kind = item.kind.max(kind);
    item.bodies.insert(target, body);
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
    external: &'a references::Bindings<'a>,
    native_sized: &'a HashSet<&'a str>,
    /// GUID constants named `IID_<interface>`, keyed by interface name.
    interface_ids: &'a HashMap<&'a str, u128>,
    x86_callbacks: &'a HashMap<String, (String, CallingConvention)>,
    callback_abis: RefCell<HashMap<String, Vec<CallingConvention>>>,
}

/// Indexes `IID_<name>` GUID constants, the C spelling of an interface's IID, by interface name.
fn interface_ids<'a>(resolution: &'a Resolution<'_>) -> Result<HashMap<&'a str, u128>, Error> {
    let mut result = HashMap::new();
    for snapshot in &resolution.snapshots {
        for value in &snapshot.values {
            let Some(interface) = value.name.strip_prefix("IID_") else {
                continue;
            };
            for observation in &value.observations {
                if let ValueKind::Constant {
                    value: Some(ConstantValue::Guid(guid)),
                    ..
                } = &observation.kind
                    && result
                        .insert(interface, *guid)
                        .is_some_and(|previous| previous != *guid)
                {
                    return Err(Error(format!("conflicting values for `{}`", value.name)));
                }
            }
        }
    }
    Ok(result)
}

fn native_sized_aliases<'a>(merged: &'a Merged<'_>) -> HashSet<&'a str> {
    merged
        .aliases
        .iter()
        .filter(|variants| {
            if variants.groups.len() != TARGETS.len() {
                return false;
            }
            let mut signedness = None;
            variants.groups.iter().all(|(target, group)| {
                let resolution = &merged.targets[target];
                let alias = &resolution.aliases[*group];
                let size = if *target == Target::X86 { 4 } else { 8 };
                alias_source(resolution, alias).is_some()
                    && alias.observations.iter().all(|reference| {
                        let snapshot = resolution.snapshots[reference.snapshot];
                        let observation = &snapshot.aliases[reference.alias.0].observations
                            [reference.observation];
                        let ty = &snapshot.types[observation.canonical.0];
                        let signed = match ty.kind {
                            TypeKind::Scalar(Scalar::Int | Scalar::Long | Scalar::LongLong) => true,
                            TypeKind::Scalar(
                                Scalar::UnsignedInt
                                | Scalar::UnsignedLong
                                | Scalar::UnsignedLongLong,
                            ) => false,
                            _ => return false,
                        };
                        ty.qualifiers == Qualifiers::default()
                            && ty.layout
                                == Some(Layout {
                                    size,
                                    alignment: size,
                                })
                            && *signedness.get_or_insert(signed) == signed
                    })
            })
        })
        .map(|variants| variants.name.as_str())
        .collect()
}

fn contains_alias(snapshot: &Snapshot, id: TypeId, contains: impl Fn(&str) -> bool) -> bool {
    let mut pending = vec![id];
    let mut seen = HashSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        match snapshot.types[id.0].kind {
            TypeKind::Alias(id) => {
                let alias = &snapshot.aliases[id.0];
                if contains(&alias.name) {
                    return true;
                }
                pending.extend(alias.observations.iter().map(|value| value.underlying));
            }
            TypeKind::Pointer(inner)
            | TypeKind::Pointer64(inner)
            | TypeKind::LValueReference(inner)
            | TypeKind::Array { element: inner, .. }
            | TypeKind::Attributed {
                modified: inner, ..
            } => pending.push(inner),
            _ => {}
        }
    }
    false
}

fn publishable_alias(snapshot: &Snapshot, observation: &AliasObservation) -> bool {
    let ty = &snapshot.types[observation.canonical.0];
    !observation.implicit
        && !ty.qualifiers.constant
        && (ty.layout.is_some()
            || matches!(ty.kind, TypeKind::Record(_) | TypeKind::Function(_))
            || ty.kind == TypeKind::Void && ty.qualifiers == Qualifiers::default())
}

fn has_fixed_pointer(snapshot: &Snapshot, id: TypeId) -> bool {
    let mut pending = vec![id];
    let mut seen = HashSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        match snapshot.types[id.0].kind {
            TypeKind::Pointer64(_) => return true,
            TypeKind::Alias(alias) => pending.extend(
                snapshot.aliases[alias.0]
                    .observations
                    .iter()
                    .map(|value| value.underlying),
            ),
            TypeKind::Pointer(inner)
            | TypeKind::Array { element: inner, .. }
            | TypeKind::Attributed {
                modified: inner, ..
            } => pending.push(inner),
            _ => {}
        }
    }
    false
}

fn written_type(snapshot: &Snapshot, mut id: TypeId) -> TypeId {
    loop {
        match snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => id = snapshot.aliases[alias.0].observations[0].underlying,
            TypeKind::Attributed { modified, .. } => id = modified,
            _ => return id,
        }
    }
}

fn alias_source(resolution: &Resolution<'_>, alias: &AliasGroup) -> Option<AliasObservationRef> {
    alias.observations.iter().copied().find(|reference| {
        !resolution.snapshots[reference.snapshot].aliases[reference.alias.0].observations
            [reference.observation]
            .implicit
    })
}

fn checked_type(snapshot: &Snapshot, id: TypeId) -> Result<&Type, Error> {
    let ty = &snapshot.types[id.0];
    if ty.qualifiers.unaligned && ty.layout.is_none_or(|layout| layout.alignment != 1) {
        return Err(Error(
            "unaligned type requires naturally byte-aligned storage".into(),
        ));
    }
    if let TypeKind::Pointer(pointee) | TypeKind::Pointer64(pointee) = ty.kind {
        let size = if matches!(ty.kind, TypeKind::Pointer64(_)) || snapshot.target != Target::X86 {
            8
        } else {
            4
        };
        if ty.layout
            != Some(Layout {
                size,
                alignment: size,
            })
        {
            return Err(Error("unsupported pointer layout".into()));
        }
        if matches!(snapshot.types[pointee.0].kind, TypeKind::Function(_))
            && snapshot.types[pointee.0].qualifiers != Qualifiers::default()
        {
            return Err(Error(
                "qualified function type projection is unsupported".into(),
            ));
        }
    }
    Ok(ty)
}

fn checked_storage_type(snapshot: &Snapshot, id: TypeId) -> Result<&Type, Error> {
    let ty = checked_type(snapshot, id)?;
    // Constants and C++ references cannot preserve qualified access semantics.
    if ty.qualifiers.volatile || ty.qualifiers.restrict {
        return Err(Error(
            "volatile/restrict storage projection is not implemented".into(),
        ));
    }
    if let TypeKind::Array { element, .. } = ty.kind {
        checked_storage_type(snapshot, element)?;
    }
    Ok(ty)
}

fn function_alias_name(snapshot: &Snapshot, mut id: TypeId) -> Option<&str> {
    loop {
        match snapshot.types[id.0].kind {
            TypeKind::Attributed { modified, .. } => id = modified,
            TypeKind::Alias(alias) => {
                let alias = &snapshot.aliases[alias.0];
                let observation = alias.observations.iter().find(|value| !value.implicit)?;
                return matches!(
                    snapshot.types[observation.canonical.0].kind,
                    TypeKind::Function(_)
                )
                .then_some(alias.name.as_str());
            }
            _ => return None,
        }
    }
}

fn agree(name: &str, target: Target, bodies: &mut [Body]) -> Result<(), Error> {
    // Native compatibility is already checked; this compares the emitted public declarations.
    if let Some((first, rest)) = bodies.split_first_mut() {
        let key = first.key();
        if let Some(other) = rest.iter().find(|body| body.key() != key) {
            return Err(Error(format!(
                "conflicting public names or contracts for `{name}` on {target:?}\nfirst: {key:?}\nother: {:?}",
                other.key(),
            )));
        }
        for other in rest {
            first.retain_abi(other);
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct CallableAbi {
    convention: CallingConvention,
    inferred: bool,
}

impl CallableAbi {
    fn canonicalize(&mut self, target: Target, convention: CallingConvention) {
        if target != Target::X86 && self.inferred {
            self.convention = convention;
        }
    }
}

fn callable_abi(function: WrittenFunction<'_>) -> CallableAbi {
    // Clang can retain a written stdcall attribute that it ignores on a variadic prototype.
    let convention = if function.prototype.variadic {
        function.prototype.convention
    } else {
        function.convention
    };
    CallableAbi {
        convention,
        inferred: !function.explicit_convention && !function.prototype.variadic,
    }
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

#[derive(Clone, Copy)]
struct WrittenFunction<'a> {
    prototype: &'a Function,
    convention: CallingConvention,
    explicit_convention: bool,
}

impl WrittenFunction<'_> {
    fn conventions(
        self,
        snapshot: &Snapshot,
        aliases: &impl Fn(AliasId) -> Result<Option<Vec<CallableAbi>>, Error>,
    ) -> Result<Vec<CallableAbi>, Error> {
        fn append(
            snapshot: &Snapshot,
            id: TypeId,
            values: &mut Vec<CallableAbi>,
            aliases: &impl Fn(AliasId) -> Result<Option<Vec<CallableAbi>>, Error>,
        ) -> Result<(), Error> {
            match snapshot.types[id.0].kind {
                TypeKind::Alias(alias) => {
                    if let Some(conventions) = aliases(alias)? {
                        values.extend(conventions);
                    } else {
                        append(
                            snapshot,
                            snapshot.aliases[alias.0].observations[0].underlying,
                            values,
                            aliases,
                        )?;
                    }
                }
                TypeKind::Pointer(inner)
                | TypeKind::Pointer64(inner)
                | TypeKind::LValueReference(inner)
                | TypeKind::Array { element: inner, .. } => {
                    append(snapshot, inner, values, aliases)?;
                }
                TypeKind::Function(_) | TypeKind::Attributed { .. } => {
                    values.extend(written_function(snapshot, id)?.conventions(snapshot, aliases)?);
                }
                _ => {}
            }
            Ok(())
        }
        let mut values = vec![callable_abi(self)];
        append(snapshot, self.prototype.result, &mut values, aliases)?;
        for &parameter in &self.prototype.parameters {
            append(snapshot, parameter, &mut values, aliases)?;
        }
        Ok(values)
    }
}

fn written_function(snapshot: &Snapshot, mut id: TypeId) -> Result<WrittenFunction<'_>, Error> {
    let mut convention = None;
    loop {
        match &snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => id = snapshot.aliases[alias.0].observations[0].underlying,
            TypeKind::Pointer(pointee) => id = *pointee,
            TypeKind::Attributed {
                convention: requested,
                modified,
                ..
            } => {
                if convention.is_some_and(|convention| convention != *requested) {
                    return Err(Error("conflicting written calling conventions".into()));
                }
                convention.get_or_insert(*requested);
                id = *modified;
            }
            TypeKind::Function(function) => {
                return Ok(WrittenFunction {
                    prototype: function,
                    convention: convention.unwrap_or(function.convention),
                    explicit_convention: convention.is_some(),
                });
            }
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

fn parameter_names(parameters: &[Parameter], named: bool) -> Vec<String> {
    parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| {
            if named && !parameter.name.is_empty() {
                parameter.name.clone()
            } else {
                format!("param{index}")
            }
        })
        .collect()
}

fn callback_contracts(snapshot: &Snapshot, observation: &AliasObservation) -> bool {
    if !observation.attributes.is_empty()
        || callback_parameters(snapshot, observation)
            .is_some_and(|parameters| parameters.iter().any(annotations::parameter_is_annotated))
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
        if !publishable_alias(snapshot, observation) {
            return Err(Error(format!(
                "typedef `{name}` is not an unqualified value type"
            )));
        }
        let canonical = checked_type(snapshot, observation.canonical)?;
        let function = match &canonical.kind {
            TypeKind::Function(function) => Some(function.as_ref()),
            _ => callback(snapshot, observation.canonical),
        };
        if let Some(function) = function {
            let written = written_function(snapshot, observation.underlying)?;
            let contracts = annotations::function(written.prototype, attributes);
            let annotations::CallableAttributes {
                declaration,
                result,
            } = annotations::write_callback(
                self.resolution,
                snapshot,
                observation.underlying,
                function.result,
                attributes,
                callback_parameters(snapshot, observation).unwrap_or_default(),
            )?;
            return self.callback(
                snapshot_id,
                function,
                written,
                name,
                callback_parameters(snapshot, observation),
                annotations::CallableAttributes {
                    declaration: quote! { #contracts #declaration },
                    result,
                },
            );
        }
        let native_sized = self.native_sized.contains(name);
        let name = ident(public_name(name))?;
        if observation
            .parameters
            .as_ref()
            .is_some_and(|parameters| parameters.iter().any(annotations::parameter_is_annotated))
        {
            return Err(Error(format!(
                "nested callback contract on `{name}` has no metadata owner"
            )));
        }
        let inherited =
            annotations::value_alias_attributes(self.resolution, snapshot, observation.underlying)?;
        let ty = if let TypeKind::Record(record) = snapshot.types[observation.canonical.0].kind
            && self.interface_record(snapshot_id, record)
        {
            self.interface_type(snapshot_id, record)
        } else if callback_parameters(snapshot, observation).is_some()
            || has_fixed_pointer(snapshot, observation.underlying)
            || self
                .external
                .contains_alias(snapshot, observation.underlying)
            || (!matches!(canonical.kind, TypeKind::Scalar(_))
                && contains_alias(snapshot, observation.underlying, |name| {
                    self.native_sized.contains(name)
                }))
            || !inherited.is_empty()
        {
            self.written_ty(snapshot_id, observation.underlying, observation.canonical)?
        } else {
            self.ty(snapshot_id, observation.canonical)?
        };
        let ty = match ty {
            MetadataType::I32 | MetadataType::I64 if native_sized => MetadataType::ISize,
            MetadataType::U32 | MetadataType::U64 if native_sized => MetadataType::USize,
            ty => ty,
        };
        let ty = write_type(self.namespace, &ty);
        Ok(Body {
            abi: None,
            attributes: annotations::write_value_typedef(
                snapshot,
                observation.canonical,
                attributes,
                &inherited,
            )?,
            declaration: quote! { type #name = #ty; },
        })
    }

    fn callback(
        &self,
        snapshot: usize,
        function: &Function,
        written: WrittenFunction<'_>,
        name: &str,
        declarations: Option<&[Parameter]>,
        attributes: annotations::CallableAttributes,
    ) -> Result<Body, Error> {
        let signature = self.signature(snapshot, function, Some(written))?;
        let abi = callable_abi(written);
        let name = ident(public_name(name))?;
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
        let result_attributes = attributes.result;
        let result = if result == MetadataType::Void {
            TokenStream::new()
        } else {
            let result = write_type(self.namespace, &result);
            quote! { -> #result_attributes #result }
        };
        Ok(Body {
            abi: Some(abi),
            attributes: attributes.declaration,
            declaration: quote! { fn #name(#(#parameters),*) #result; },
        })
    }

    fn signature(
        &self,
        snapshot: usize,
        function: &Function,
        written: Option<WrittenFunction<'_>>,
    ) -> Result<windows_metadata::Signature, Error> {
        if function.variadic {
            return Err(Error("variadic callbacks are unsupported".into()));
        }
        self.check_function_abi(snapshot, function, written.map(|value| value.prototype))?;
        if function.method_qualifiers != Qualifiers::default() {
            return Err(Error(
                "method qualifiers are unsupported on callbacks".into(),
            ));
        }
        if written
            .is_some_and(|written| function.parameters.len() != written.prototype.parameters.len())
        {
            return Err(Error("written and canonical callback arity differ".into()));
        }
        let flags = match written.map_or(function.convention, |written| written.convention) {
            CallingConvention::C => windows_metadata::MethodCallAttributes::C,
            CallingConvention::Stdcall => windows_metadata::MethodCallAttributes::STDCALL,
        };
        let types = function
            .parameters
            .iter()
            .enumerate()
            .map(|(index, id)| {
                let written = written.map_or(*id, |written| written.prototype.parameters[index]);
                self.parameter_ty(snapshot, written, *id)
            })
            .collect::<Result<_, _>>()?;
        let return_type = if let Some(written) = written {
            self.result_ty(snapshot, written.prototype.result, function.result)?
        } else {
            self.ty(snapshot, function.result)?
        };
        Ok(windows_metadata::Signature {
            flags,
            return_type,
            types,
        })
    }

    fn check_function_abi(
        &self,
        snapshot: usize,
        function: &Function,
        written: Option<&Function>,
    ) -> Result<(), Error> {
        if written.is_some_and(|written| written.parameters.len() != function.parameters.len()) {
            return Err(Error("written and canonical callable arity differ".into()));
        }
        let source = self.resolution.snapshots[snapshot];
        for (index, &ty) in function.parameters.iter().enumerate() {
            self.check_value_abi(snapshot, ty, false, &mut HashSet::new())?;
            if let Some(written) = written {
                let written = written_type(source, written.parameters[index]);
                // Array and function parameters pass the compiler-adjusted pointer, not a value.
                if matches!(source.types[ty.0].kind, TypeKind::Pointer(_))
                    && matches!(
                        source.types[written.0].kind,
                        TypeKind::Array { .. } | TypeKind::Function(_)
                    )
                {
                    continue;
                }
                self.check_value_abi(snapshot, written, false, &mut HashSet::new())?;
            }
        }
        self.check_value_abi(snapshot, function.result, true, &mut HashSet::new())?;
        if let Some(written) = written {
            self.check_value_abi(snapshot, written.result, true, &mut HashSet::new())?;
        }
        Ok(())
    }

    fn check_value_abi(
        &self,
        snapshot: usize,
        ty: TypeId,
        result: bool,
        seen: &mut HashSet<(usize, TypeId)>,
    ) -> Result<(), Error> {
        if !seen.insert((snapshot, ty)) {
            return Ok(());
        }
        match self.resolution.snapshots[snapshot].types[ty.0].kind {
            TypeKind::Alias(alias) => {
                for observation in
                    &self.resolution.snapshots[snapshot].aliases[alias.0].observations
                {
                    self.check_value_abi(snapshot, observation.underlying, result, seen)?;
                }
                Ok(())
            }
            TypeKind::Attributed { modified, .. } => {
                self.check_value_abi(snapshot, modified, result, seen)
            }
            TypeKind::Pointer64(_) => Err(Error(
                "fixed-width pointer values require a calling-ABI contract".into(),
            )),
            TypeKind::Array { element, .. } => {
                self.check_value_abi(snapshot, element, result, seen)
            }
            TypeKind::Record(record) => {
                let record = RecordRef { snapshot, record };
                let reference = self.resolution.group(record).definition.ok_or_else(|| {
                    Error(format!(
                        "incomplete record `{}` cannot be used by value",
                        self.resolution.snapshots[snapshot].records[record.record.0].name
                    ))
                })?;
                let observation = &self.resolution.snapshots[reference.snapshot].observations
                    [reference.observation.0];
                let definition = observation.definition.as_ref().unwrap();
                self.check_opaque_storage(snapshot, ty)?;
                let (packed, maximum) = resolve::packing(&observation.attributes);
                if (packed || maximum.is_some())
                    && self.required_alignment(snapshot, ty, &mut HashSet::new()) != 0
                {
                    return Err(Error(
                        "by-value mixed packing/alignment requires a calling-ABI contract".into(),
                    ));
                }
                if !definition.bases.is_empty() {
                    return Err(Error(
                        "by-value C++ inheritance requires a calling-ABI contract".into(),
                    ));
                }
                let semantics = definition.semantics;
                if !self.interface_record(snapshot, record.record)
                    && (!semantics.can_pass_in_registers
                        || semantics.non_trivial_destructor
                        || (result
                            && (semantics.non_trivial_copy_assignment
                                || semantics.deleted_copy_assignment
                                || semantics.user_provided_constructor
                                || semantics.constructor_template)))
                {
                    return Err(Error(
                        "by-value C++ special members require a calling-ABI contract".into(),
                    ));
                }
                for field in &definition.fields {
                    if annotations::field_alignment(&field.attributes)?.is_some() {
                        return Err(Error(
                            "by-value member alignment requires a calling-ABI contract".into(),
                        ));
                    }
                    self.check_value_abi(reference.snapshot, field.canonical, false, seen)?;
                    self.check_value_abi(reference.snapshot, field.written, false, seen)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Projects a pointer or reference to a COM interface object as the interface reference.
    fn interface_pointee(
        &self,
        snapshot_id: usize,
        pointee: TypeId,
    ) -> Result<Option<MetadataType>, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let TypeKind::Record(record) = snapshot.types[pointee.0].kind else {
            return Ok(None);
        };
        if !self.interface_record(snapshot_id, record) {
            return Ok(None);
        }
        checked_storage_type(snapshot, pointee)?;
        if snapshot.types[pointee.0].qualifiers.constant {
            let reference = self
                .resolution
                .group(RecordRef {
                    snapshot: snapshot_id,
                    record,
                })
                .definition
                .unwrap();
            let observation = &self.resolution.snapshots[reference.snapshot].observations
                [reference.observation.0];
            if resolve::uuid(&observation.attributes).is_none() {
                return Err(Error(
                    "const GUID-less interface pointers require const-preserving metadata".into(),
                ));
            }
        }
        // COM references do not express C++ object constness. Outer pointer qualifiers still
        // describe pointer storage and are projected normally.
        Ok(Some(self.interface_type(snapshot_id, record)))
    }

    fn parameter_ty(
        &self,
        snapshot_id: usize,
        written: TypeId,
        canonical: TypeId,
    ) -> Result<MetadataType, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let source = written_type(snapshot, written);
        if matches!(snapshot.types[source.0].kind, TypeKind::Function(_))
            && let Some(function) = callback(snapshot, canonical)
        {
            checked_type(snapshot, canonical)?;
            if let Some(name) = function_alias_name(snapshot, written) {
                return Ok(self.callback_type(name));
            }
            return self.function_pointer(
                snapshot_id,
                canonical,
                function,
                Some(written_function(snapshot, written)?),
            );
        }
        if let TypeKind::Array { element, .. } = snapshot.types[source.0].kind
            && let TypeKind::Pointer(pointee) = snapshot.types[canonical.0].kind
        {
            checked_type(snapshot, canonical)?;
            let inner = Box::new(self.written_ty(snapshot_id, element, pointee)?);
            return Ok(if snapshot.types[pointee.0].qualifiers.constant {
                MetadataType::PtrConst(inner, 1)
            } else {
                MetadataType::PtrMut(inner, 1)
            });
        }
        if let TypeKind::LValueReference(pointee) = snapshot.types[canonical.0].kind {
            // An interface reference passes the same object pointer as `I*`.
            if let Some(interface) = self.interface_pointee(snapshot_id, pointee)? {
                return Ok(interface);
            }
            checked_storage_type(snapshot, pointee)?;
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

    fn result_ty(
        &self,
        snapshot_id: usize,
        written: TypeId,
        canonical: TypeId,
    ) -> Result<MetadataType, Error> {
        let ty = self.written_ty(snapshot_id, written, canonical)?;
        // A void typedef still denotes no result, not an opaque pointee value.
        if self.resolution.snapshots[snapshot_id].types[canonical.0].kind == TypeKind::Void {
            Ok(MetadataType::Void)
        } else {
            Ok(ty)
        }
    }

    fn written_ty(
        &self,
        snapshot_id: usize,
        written: TypeId,
        canonical: TypeId,
    ) -> Result<MetadataType, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let native = checked_type(snapshot, canonical)?;
        let fixed = matches!(snapshot.types[written.0].kind, TypeKind::Pointer64(_))
            || matches!(native.kind, TypeKind::Pointer64(_));
        Ok(match &snapshot.types[written.0].kind {
            TypeKind::Alias(id)
                if self
                    .external
                    .aliases
                    .contains_key(&snapshot.aliases[id.0].name)
                    || snapshot.aliases[id.0]
                        .observations
                        .iter()
                        .any(|observation| publishable_alias(snapshot, observation)) =>
            {
                let alias = &snapshot.aliases[id.0];
                if let Some(definition) = self.external.aliases.get(&alias.name) {
                    if let TypeKind::Record(record) = native.kind
                        && self.interface_record(snapshot_id, record)
                    {
                        return Err(Error("interface values require a native pointer".into()));
                    }
                    if matches!(native.kind, TypeKind::Function(_)) {
                        return Err(Error("function values require a native pointer".into()));
                    }
                    return Ok(references::metadata_type(*definition));
                }
                if callback_parameters(snapshot, &alias.observations[0]).is_none() {
                    self.ty(snapshot_id, canonical)?;
                }
                let name = &alias.name;
                if callback(snapshot, canonical).is_some() {
                    MetadataType::class_named(self.namespace, name)
                } else if matches!(native.kind, TypeKind::Function(_)) {
                    return Err(Error("function values require a native pointer".into()));
                } else {
                    MetadataType::value_named(self.namespace, name)
                }
            }
            TypeKind::Pointer(written) | TypeKind::Pointer64(written) => {
                let (TypeKind::Pointer(pointee) | TypeKind::Pointer64(pointee)) = native.kind
                else {
                    return Err(Error(
                        "written pointer has a non-pointer canonical type".into(),
                    ));
                };
                if fixed && matches!(snapshot.types[pointee.0].kind, TypeKind::Function(_)) {
                    return Err(Error(
                        "fixed-width function pointers are unsupported".into(),
                    ));
                }
                if fixed {
                    if let TypeKind::Record(record) = snapshot.types[pointee.0].kind
                        && self.interface_record(snapshot_id, record)
                    {
                        return Err(Error(
                            "fixed-width interface pointers are unsupported".into(),
                        ));
                    }
                    let inner = Box::new(self.written_ty(snapshot_id, *written, pointee)?);
                    return Ok(MetadataType::Ptr64(
                        inner,
                        snapshot.types[pointee.0].qualifiers.constant,
                    ));
                }
                if let TypeKind::Record(record) = snapshot.types[pointee.0].kind
                    && self.interface_record(snapshot_id, record)
                {
                    return self.ty(snapshot_id, canonical);
                }
                if let TypeKind::Function(function) = &snapshot.types[pointee.0].kind {
                    if let Some(name) = function_alias_name(snapshot, *written) {
                        return Ok(self.callback_type(name));
                    }
                    self.function_pointer(
                        snapshot_id,
                        canonical,
                        function,
                        Some(written_function(snapshot, *written)?),
                    )?
                } else {
                    let inner = Box::new(self.written_ty(snapshot_id, *written, pointee)?);
                    if snapshot.types[pointee.0].qualifiers.constant {
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
            _ => self.ty(snapshot_id, canonical)?,
        })
    }

    fn function_pointer(
        &self,
        snapshot: usize,
        id: TypeId,
        function: &Function,
        written: Option<WrittenFunction<'_>>,
    ) -> Result<MetadataType, Error> {
        let source = self.resolution.snapshots[snapshot];
        let prototype = written.unwrap_or(WrittenFunction {
            prototype: function,
            convention: function.convention,
            explicit_convention: false,
        });
        if let Some(candidates) = self.plan.callbacks[snapshot].get(&id) {
            let conventions = prototype.conventions(source, &|_| Ok(None))?;
            let required = self.required_abis(snapshot, prototype)?;
            // Prefer the same spelling, but require compatibility with the published ABI.
            for preferred in [true, false] {
                for (name, declared) in candidates {
                    if (declared == &conventions) != preferred
                        || declared.len() != conventions.len()
                        || declared
                            .iter()
                            .zip(&conventions)
                            .any(|(a, b)| a.convention != b.convention)
                    {
                        continue;
                    }
                    if abis_compatible(source.target, &required, &self.callback_abis(name)?) {
                        return Ok(self.callback_type(name));
                    }
                }
            }
        }
        if function.noreturn {
            Err(Error(
                "noreturn function pointer requires a named callback to retain its contract".into(),
            ))
        } else {
            Ok(MetadataType::FnPtr(Box::new(
                self.signature(snapshot, function, written)?,
            )))
        }
    }

    fn ty(&self, snapshot_id: usize, id: TypeId) -> Result<MetadataType, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let ty = checked_type(snapshot, id)?;
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
            TypeKind::Pointer64(pointee) => {
                if matches!(snapshot.types[pointee.0].kind, TypeKind::Function(_)) {
                    return Err(Error(
                        "fixed-width function pointers are unsupported".into(),
                    ));
                }
                MetadataType::Ptr64(
                    Box::new(self.ty(snapshot_id, *pointee)?),
                    snapshot.types[pointee.0].qualifiers.constant,
                )
            }
            TypeKind::Pointer(pointee) => {
                if let TypeKind::Function(function) = &snapshot.types[pointee.0].kind {
                    return self.function_pointer(snapshot_id, id, function, None);
                }
                if let Some(interface) = self.interface_pointee(snapshot_id, *pointee)? {
                    return Ok(interface);
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
                if let Some(definition) = self
                    .external
                    .records
                    .get(&self.plan.family(snapshot_id, *record))
                    && !self.interface_record(snapshot_id, *record)
                {
                    return Ok(references::metadata_type(*definition));
                }
                if self
                    .uuid_tag(self.resolution.group(RecordRef {
                        snapshot: snapshot_id,
                        record: *record,
                    }))
                    .is_some()
                {
                    return Err(Error("UUID tag has no projected type".into()));
                }
                if self.interface_record(snapshot_id, *record) {
                    return Err(Error("interface values require a native pointer".into()));
                }
                let name = &self.plan.names[self.plan.family(snapshot_id, *record)];
                MetadataType::value_named(self.namespace, name)
            }
            TypeKind::Enum(id) => {
                let group = self.resolution.enum_groups[snapshot_id][id.0];
                if snapshot.enums[id.0].owner.is_some() {
                    if snapshot.enums[id.0].access != FieldAccess::Public {
                        return Err(Error("non-public member enum has no projected type".into()));
                    }
                    if !self.plan.enum_constants(self.resolution, group) {
                        return Err(Error("nested enums are not supported".into()));
                    }
                }
                if let Some(definition) = self.external.enums.get(&group) {
                    return Ok(references::metadata_type(*definition));
                }
                if self.plan.enum_constants(self.resolution, group) {
                    // Without a public type name, uses share the constants' backing type.
                    let reference = self.resolution.enums[group]
                        .definition
                        .ok_or_else(|| Error("anonymous enum has no captured definition".into()))?;
                    let observation = &self.resolution.snapshots[reference.snapshot].enums
                        [reference.enumeration.0]
                        .observations[reference.observation];
                    return self.enum_backing_type(
                        reference.snapshot,
                        observation,
                        &snapshot.enums[id.0].name,
                    );
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
        let flags = flags(observation).then(|| quote! { #[flags] });
        let native = annotations::write(&observation.attributes)?;
        Ok(Body {
            abi: None,
            attributes: quote! { #native #[repr(#repr)] },
            declaration: quote! { #flags #scoped enum #name { #(#fields)* } },
        })
    }

    fn enum_backing(
        &self,
        snapshot_id: usize,
        observation: &EnumObservation,
        name: &str,
    ) -> Result<TokenStream, Error> {
        Ok(write_type(
            self.namespace,
            &self.enum_backing_type(snapshot_id, observation, name)?,
        ))
    }

    fn enum_backing_type(
        &self,
        snapshot_id: usize,
        observation: &EnumObservation,
        name: &str,
    ) -> Result<MetadataType, Error> {
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
        self.ty(snapshot_id, observation.canonical)
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
            fields.push((annotations::write(&member.attributes)?, member_name, value));
        }
        Ok(fields)
    }
}

fn public_name(value: &str) -> &str {
    value.rsplit_once("::").map_or(value, |(_, name)| name)
}

fn write_type(namespace: &str, ty: &MetadataType) -> TokenStream {
    fn flatten(namespace: &str, ty: &mut MetadataType) {
        match ty {
            MetadataType::ClassName(name) | MetadataType::ValueName(name) => {
                if name.namespace == namespace && name.name.contains("::") {
                    name.name = public_name(&name.name).to_string();
                }
                for ty in &mut name.generics {
                    flatten(namespace, ty);
                }
            }
            MetadataType::Array(ty)
            | MetadataType::ArrayFixed(ty, _)
            | MetadataType::PtrMut(ty, _)
            | MetadataType::PtrConst(ty, _)
            | MetadataType::Ptr64(ty, _)
            | MetadataType::RefMut(ty)
            | MetadataType::RefConst(ty) => flatten(namespace, ty),
            MetadataType::FnPtr(signature) => {
                flatten(namespace, &mut signature.return_type);
                for ty in &mut signature.types {
                    flatten(namespace, ty);
                }
            }
            MetadataType::Void
            | MetadataType::Bool
            | MetadataType::Char
            | MetadataType::I8
            | MetadataType::U8
            | MetadataType::I16
            | MetadataType::U16
            | MetadataType::I32
            | MetadataType::U32
            | MetadataType::I64
            | MetadataType::U64
            | MetadataType::F32
            | MetadataType::F64
            | MetadataType::ISize
            | MetadataType::USize
            | MetadataType::String
            | MetadataType::Object
            | MetadataType::Generic(..) => {}
        }
    }
    let mut ty = ty.clone();
    flatten(namespace, &mut ty);
    windows_rdl::emit::write_type(namespace, &ty)
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
        "shared".to_string()
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
        if stem == "shared" {
            return Err(Error(format!(
                "header `{header}` collides with reserved shared partition"
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
    abi: Option<CallableAbi>,
    attributes: TokenStream,
    declaration: TokenStream,
}

impl Body {
    fn key(&self) -> String {
        self.tokens().to_string()
    }

    fn contract_key(&self) -> String {
        format!("{} {}", self.attributes, self.declaration)
    }

    fn retain_abi(&mut self, other: &Self) {
        if let Some(abi) = &mut self.abi {
            abi.inferred &= other.abi.unwrap().inferred;
        }
    }

    fn canonicalize_abi(&mut self, target: Target, convention: CallingConvention) {
        if let Some(abi) = &mut self.abi {
            abi.canonicalize(target, convention);
        }
    }

    fn tokens(&self) -> TokenStream {
        let Self {
            attributes,
            declaration,
            abi,
        } = self;
        let abi = abi.map(|abi| match abi.convention {
            CallingConvention::C => quote! { extern "C" },
            CallingConvention::Stdcall => quote! { extern },
        });
        quote! { #attributes #abi #declaration }
    }
}

fn variants(bodies: BTreeMap<Target, Body>) -> TokenStream {
    let x86 = bodies
        .get(&Target::X86)
        .and_then(|body| Some((body.contract_key(), body.abi?.convention)));
    let mut grouped: BTreeMap<String, (Body, Vec<Target>)> = BTreeMap::new();
    for (target, mut body) in bodies {
        if let Some((key, convention)) = &x86
            && body.contract_key() == *key
        {
            // Same-target conflicts and ownership have already been checked.
            body.canonicalize_abi(target, *convention);
        }
        grouped
            .entry(body.key())
            .or_insert_with(|| (body, Vec::new()))
            .1
            .push(target);
    }
    let mut rendered: Vec<_> = grouped
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
            let body = body.tokens();
            quote! { #arch #body }
        })
        .collect();
    rendered.sort_by_cached_key(|tokens| windows_rdl::formatter::format(&tokens.to_string()));
    rendered.into_iter().collect()
}
