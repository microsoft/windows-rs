use super::*;

mod enums;
mod records;
mod values;
pub use enums::{EnumGroup, EnumObservationRef, EnumRef};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecordRef {
    pub snapshot: usize,
    pub record: RecordId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationRef {
    pub snapshot: usize,
    pub observation: ObservationId,
}

#[derive(Debug)]
pub struct Group {
    pub records: Vec<RecordRef>,
    pub definition: Option<ObservationRef>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AliasObservationRef {
    pub snapshot: usize,
    pub alias: AliasId,
    pub observation: usize,
}

pub struct AliasGroup {
    pub name: String,
    /// Name-corresponding bindings, not a claim that their canonical types are identical.
    pub observations: Vec<AliasObservationRef>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValueObservationRef {
    pub snapshot: usize,
    pub value: ValueId,
    pub observation: usize,
}

pub struct ValueGroup {
    pub name: String,
    /// Constant definitions precede declarations; remaining order is stable by source.
    pub observations: Vec<ValueObservationRef>,
}

#[derive(Clone, Copy)]
enum Owner {
    Record(ObservationRef),
    Alias(AliasObservationRef),
    Enum(EnumObservationRef),
    Value(ValueObservationRef),
}

#[derive(Debug, Default)]
pub struct Work {
    pub definition_pairs: usize,
    pub type_pairs: usize,
    pub enum_pairs: usize,
}

pub struct Resolution<'a> {
    pub snapshots: Vec<&'a Snapshot>,
    pub groups: Vec<Group>,
    pub aliases: Vec<AliasGroup>,
    pub enums: Vec<EnumGroup>,
    pub values: Vec<ValueGroup>,
    pub work: Work,
    record_groups: Vec<Vec<usize>>,
    pub(crate) enum_groups: Vec<Vec<usize>>,
}

impl Resolution<'_> {
    pub fn enum_group(&self, reference: EnumRef) -> &EnumGroup {
        &self.enums[self.enum_groups[reference.snapshot][reference.enumeration.0]]
    }

    pub fn group(&self, record: RecordRef) -> &Group {
        &self.groups[self.record_groups[record.snapshot][record.record.0]]
    }

    pub fn name(&self, group: &Group) -> &str {
        let reference = group.records[0];
        &self.snapshots[reference.snapshot].records[reference.record.0].name
    }

    pub fn definition(&self, group: &Group) -> Option<&RecordDefinition> {
        group.definition.map(|reference| {
            self.snapshots[reference.snapshot].observations[reference.observation.0]
                .definition
                .as_ref()
                .unwrap()
        })
    }
}

type TypeRef = (usize, TypeId);
type TypeCheck = (TypeRef, TypeRef, Owner, Owner);
pub(crate) type TypePair = (TypeRef, TypeRef);

/// Checks all observed definitions for one target without discarding original observations.
///
/// Correspondence uses global identities and owner-relative nested records under matching arguments.
/// Names and USRs select candidates; all candidate definitions must then agree.
/// Alias groups preserve name-corresponding bindings for later public compatibility checks.
pub fn resolve(snapshots: &[Snapshot]) -> Result<Resolution<'_>, Error> {
    resolve_snapshots(snapshots.iter().collect())
}

pub(super) fn resolve_snapshots(snapshots: Vec<&Snapshot>) -> Result<Resolution<'_>, Error> {
    let Some(first) = snapshots.first() else {
        return Err(Error("resolve requires snapshots".into()));
    };
    if snapshots.iter().any(|snapshot| {
        snapshot.target != first.target
            || snapshot.compiler != first.compiler
            || snapshot.arguments != first.arguments
    }) {
        return Err(Error(
            "resolve requires the same target, compiler, and arguments".into(),
        ));
    }
    validate_observation_attributes(&snapshots)?;
    let mut membership: Vec<Vec<usize>> = snapshots
        .iter()
        .map(|snapshot| vec![0; snapshot.records.len()])
        .collect();
    let mut pending = Vec::new();
    let groups = records::groups(&snapshots, &mut membership, &mut pending)?;
    let mut work = Work::default();
    let mut types = Vec::new();
    records::definitions(&snapshots, &membership, pending, &mut work, &mut types)?;
    let aliases = alias_groups(&snapshots);
    // Record edges refer to groups whose definitions all have independent obligations above.
    // No recursive assumption is published unless the entire worklist succeeds.
    let (enums, enum_groups) = enums::resolve(&snapshots, &mut work)?;
    let values = values::resolve(&snapshots, &mut types)?;
    let mut seen = HashSet::new();
    while let Some((left, right, owner_left, owner_right)) = types.pop() {
        if !types_equal(&snapshots, left, right, &mut seen, &enum_groups, &|a, b| {
            membership[a.snapshot][a.record.0] == membership[b.snapshot][b.record.0]
        }) {
            return Err(conflict(
                &snapshots,
                owner_left,
                owner_right,
                "native field types differ",
            ));
        }
    }
    work.type_pairs = seen.len();
    Ok(Resolution {
        snapshots,
        groups,
        aliases,
        enums,
        values,
        enum_groups,
        work,
        record_groups: membership,
    })
}

fn validate_observation_attributes(snapshots: &[&Snapshot]) -> Result<(), Error> {
    for (id, snapshot) in snapshots.iter().enumerate() {
        for (observation, value) in snapshot.observations.iter().enumerate() {
            let owner = Owner::Record(ObservationRef {
                snapshot: id,
                observation: ObservationId(observation),
            });
            attributes(snapshots, owner, &value.attributes)?;
            if let Some(definition) = &value.definition {
                for field in &definition.fields {
                    attributes(snapshots, owner, &field.attributes)?;
                }
                for method in &definition.methods {
                    value_attributes(snapshots, owner, &method.observation)?;
                }
            }
        }
        for (alias, value) in snapshot.aliases.iter().enumerate() {
            for (observation, value) in value.observations.iter().enumerate() {
                for parameter in value.parameters.iter().flatten() {
                    attributes(
                        snapshots,
                        Owner::Alias(AliasObservationRef {
                            snapshot: id,
                            alias: AliasId(alias),
                            observation,
                        }),
                        &parameter.attributes,
                    )?;
                }
                attributes(
                    snapshots,
                    Owner::Alias(AliasObservationRef {
                        snapshot: id,
                        alias: AliasId(alias),
                        observation,
                    }),
                    &value.attributes,
                )?;
            }
        }
    }
    Ok(())
}

fn alias_groups(snapshots: &[&Snapshot]) -> Vec<AliasGroup> {
    let mut aliases: BTreeMap<&str, Vec<AliasObservationRef>> = BTreeMap::new();
    for (snapshot, value) in snapshots.iter().enumerate() {
        for (id, alias) in value.aliases.iter().enumerate() {
            // Clang's typedef USRs contain filenames; global typedef correspondence uses the name.
            let group = aliases.entry(&alias.name).or_default();
            for observation in 0..alias.observations.len() {
                group.push(AliasObservationRef {
                    snapshot,
                    alias: AliasId(id),
                    observation,
                });
            }
        }
    }
    let mut alias_groups = Vec::new();
    for (name, mut observations) in aliases {
        observations.sort_by_key(|reference| {
            let snapshot = snapshots[reference.snapshot];
            let location =
                &snapshot.aliases[reference.alias.0].observations[reference.observation].location;
            (
                snapshot.input.as_str(),
                snapshot.files[location.file.0].as_str(),
                location.offset,
            )
        });
        alias_groups.push(AliasGroup {
            name: name.to_string(),
            observations,
        });
    }
    alias_groups
}

/// Record relations supply independent definition checks; a failed comparison invalidates the pass.
pub(crate) fn types_equal(
    snapshots: &[&Snapshot],
    mut a: TypeRef,
    mut b: TypeRef,
    seen: &mut HashSet<TypePair>,
    enums: &[Vec<usize>],
    records: &impl Fn(RecordRef, RecordRef) -> bool,
) -> bool {
    loop {
        if !seen.insert(if a <= b { (a, b) } else { (b, a) }) {
            return true;
        }
        let x = &snapshots[a.0].types[a.1.0];
        let y = &snapshots[b.0].types[b.1.0];
        if x.qualifiers != y.qualifiers {
            return false;
        }
        if let (TypeKind::Record(x), TypeKind::Record(y)) = (&x.kind, &y.kind) {
            return records(
                RecordRef {
                    snapshot: a.0,
                    record: *x,
                },
                RecordRef {
                    snapshot: b.0,
                    record: *y,
                },
            );
        }
        if let (TypeKind::Enum(x), TypeKind::Enum(y)) = (&x.kind, &y.kind) {
            return enums[a.0][x.0] == enums[b.0][y.0];
        }
        if x.layout != y.layout {
            return false;
        }
        match (&x.kind, &y.kind) {
            (TypeKind::Function(x), TypeKind::Function(y)) => {
                return x.convention == y.convention
                    && x.exceptions == y.exceptions
                    && x.noreturn == y.noreturn
                    && x.variadic == y.variadic
                    && x.method_qualifiers == y.method_qualifiers
                    && x.parameters.len() == y.parameters.len()
                    && types_equal(
                        snapshots,
                        (a.0, x.result),
                        (b.0, y.result),
                        seen,
                        enums,
                        records,
                    )
                    && x.parameters.iter().zip(&y.parameters).all(|(x, y)| {
                        types_equal(snapshots, (a.0, *x), (b.0, *y), seen, enums, records)
                    });
            }
            (TypeKind::Void, TypeKind::Void) => return true,
            (TypeKind::Scalar(x), TypeKind::Scalar(y)) => return x == y,
            (TypeKind::Pointer(x), TypeKind::Pointer(y))
            | (TypeKind::Pointer64(x), TypeKind::Pointer64(y))
            | (TypeKind::LValueReference(x), TypeKind::LValueReference(y)) => {
                a.1 = *x;
                b.1 = *y;
            }
            (
                TypeKind::Array {
                    element: x,
                    length: n,
                },
                TypeKind::Array {
                    element: y,
                    length: m,
                },
            ) if n == m => {
                a.1 = *x;
                b.1 = *y;
            }
            _ => return false,
        }
    }
}

fn conflict(snapshots: &[&Snapshot], left: Owner, right: Owner, reason: &str) -> Error {
    Error(format!(
        "{reason}: {} vs {}",
        describe(snapshots, left),
        describe(snapshots, right)
    ))
}

fn describe(snapshots: &[&Snapshot], owner: Owner) -> String {
    let (snapshot, name, location) = match owner {
        Owner::Value(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let value = &snapshot.values[reference.value.0];
            (
                snapshot,
                value.name.clone(),
                &value.observations[reference.observation].location,
            )
        }
        Owner::Enum(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let enumeration = &snapshot.enums[reference.enumeration.0];
            (
                snapshot,
                enumeration.name.clone(),
                &enumeration.observations[reference.observation].location,
            )
        }
        Owner::Record(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let observation = &snapshot.observations[reference.observation.0];
            (
                snapshot,
                snapshot.record_label(observation.record),
                &observation.location,
            )
        }
        Owner::Alias(reference) => {
            let snapshot = snapshots[reference.snapshot];
            let alias = &snapshot.aliases[reference.alias.0];
            (
                snapshot,
                alias.name.clone(),
                &alias.observations[reference.observation].location,
            )
        }
    };
    format!(
        "{}: {name} [{}] {}:{}:{}",
        snapshot.target.triple(),
        snapshot.input,
        snapshot.files[location.file.0],
        location.line,
        location.column
    )
}

pub(crate) fn packing(attributes: &[Attribute]) -> (bool, Option<u32>) {
    let packed = attributes
        .iter()
        .any(|attribute| attribute.kind == AttributeKind::Packed);
    let maximum = attributes
        .iter()
        .find_map(|attribute| match attribute.kind {
            AttributeKind::MaxFieldAlignment { bits } => Some(bits),
            _ => None,
        });
    (packed, maximum)
}

pub(crate) fn alignments(attributes: &[Attribute]) -> Vec<(u32, AlignmentKind)> {
    let mut result: Vec<_> = attributes
        .iter()
        .filter_map(|attribute| {
            if let AttributeKind::Aligned { bits, kind } = attribute.kind {
                Some((bits, kind))
            } else {
                None
            }
        })
        .collect();
    result.sort_unstable();
    result.dedup();
    result
}

pub(crate) fn abi_attributes_equal(a: &[Attribute], b: &[Attribute]) -> bool {
    packing(a) == packing(b)
        && a.iter().any(|value| value.kind == AttributeKind::FlagEnum)
            == b.iter().any(|value| value.kind == AttributeKind::FlagEnum)
        && alignments(a) == alignments(b)
        && default_type_visibility(a) == default_type_visibility(b)
        && deprecations(a) == deprecations(b)
        && uuid(a) == uuid(b)
        && no_trivial_auto_var_init(a) == no_trivial_auto_var_init(b)
        && a.iter()
            .any(|value| value.kind == AttributeKind::MSAllocator)
            == b.iter()
                .any(|value| value.kind == AttributeKind::MSAllocator)
        && a.iter().any(|value| value.kind == AttributeKind::NoVTable)
            == b.iter().any(|value| value.kind == AttributeKind::NoVTable)
        && a.iter().any(|value| value.kind == AttributeKind::DllImport)
            == b.iter().any(|value| value.kind == AttributeKind::DllImport)
        && a.iter().any(|value| value.kind == AttributeKind::DllExport)
            == b.iter().any(|value| value.kind == AttributeKind::DllExport)
        && a.iter().any(|value| value.kind == AttributeKind::NoThrow)
            == b.iter().any(|value| value.kind == AttributeKind::NoThrow)
        && a.iter()
            .any(|value| value.kind == AttributeKind::Cxx11NoReturn)
            == b.iter()
                .any(|value| value.kind == AttributeKind::Cxx11NoReturn)
}

pub(crate) fn attributes_equal(a: &[Attribute], b: &[Attribute]) -> bool {
    abi_attributes_equal(a, b) && sal(a) == sal(b)
}

pub(crate) fn field_attributes_equal(
    a: &Field,
    left: &[Field],
    b: &Field,
    right: &[Field],
) -> bool {
    abi_attributes_equal(&a.attributes, &b.attributes)
        && sal::fields(&a.attributes, left) == sal::fields(&b.attributes, right)
}

pub(crate) fn parameters_equal(a: &[Parameter], b: &[Parameter]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(left, right)| {
            abi_attributes_equal(&left.attributes, &right.attributes)
                && sal::parameters(&left.attributes, a) == sal::parameters(&right.attributes, b)
                && midl_parameter(left) == midl_parameter(right)
                && left.default.as_ref().map(|value| &value.expression)
                    == right.default.as_ref().map(|value| &value.expression)
        })
}

#[derive(Default, Eq, PartialEq)]
pub(crate) struct MidlParameter {
    pub input: bool,
    pub output: bool,
    pub retval: bool,
    pub iid_is: bool,
}

pub(crate) fn midl_parameter(parameter: &Parameter) -> MidlParameter {
    let mut result = MidlParameter::default();
    for comment in &parameter.comments {
        let Some(mut text) = comment
            .text
            .strip_prefix("/*")
            .and_then(|s| s.strip_suffix("*/"))
        else {
            continue;
        };
        let mut value = MidlParameter::default();
        loop {
            text = text.trim();
            if text.is_empty() {
                result.input |= value.input;
                result.output |= value.output;
                result.retval |= value.retval;
                result.iid_is |= value.iid_is;
                break;
            }
            let Some((name, rest)) = text.strip_prefix('[').and_then(|s| s.split_once(']')) else {
                break;
            };
            let name = name.trim();
            if name.is_empty() || !name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_') {
                break;
            }
            match name {
                "in" => value.input = true,
                "out" => value.output = true,
                "retval" => value.retval = true,
                "iid_is" => value.iid_is = true,
                _ => {}
            }
            text = rest;
        }
    }
    if !result.retval && !result.iid_is {
        return result;
    }
    for value in sal(&parameter.attributes) {
        if let Some((direction, _)) = value.unscoped().and_then(sal::direction) {
            result.input |= direction != sal::Direction::Output;
            result.output |= direction != sal::Direction::Input;
        }
    }
    result
}

pub(crate) fn sal(attributes: &[Attribute]) -> Vec<sal::Annotation<'_>> {
    let mut result = Vec::new();
    let mut modifiers = Vec::new();
    for value in attributes.iter().filter_map(|attribute| {
        if let AttributeKind::Sal(value) = &attribute.kind {
            Some(value.as_ref())
        } else {
            None
        }
    }) {
        if sal::modifier(value) {
            modifiers.push(value);
        } else {
            result.push(sal::Annotation {
                value,
                modifiers: std::mem::take(&mut modifiers),
            });
        }
    }
    // Retain an unattached qualifier as evidence rather than assigning it to another owner.
    if let Some(value) = modifiers.pop() {
        result.push(sal::Annotation { value, modifiers });
    }
    result.sort_unstable();
    result.dedup();
    result
}

pub(crate) fn deprecations(attributes: &[Attribute]) -> Vec<&Deprecation> {
    let mut values: Vec<_> = attributes
        .iter()
        .filter_map(|attribute| {
            if let AttributeKind::Deprecated(value) = &attribute.kind {
                Some(value.as_ref())
            } else {
                None
            }
        })
        .collect();
    values.sort_unstable();
    values.dedup();
    values
}

pub(crate) fn uuid(attributes: &[Attribute]) -> Option<u128> {
    attributes
        .iter()
        .find_map(|attribute| match attribute.kind {
            AttributeKind::Uuid(value) => Some(value),
            _ => None,
        })
}

pub(crate) fn value_shape_equal(a: &ValueObservation, b: &ValueObservation, symbol: bool) -> bool {
    attributes_equal(&a.attributes, &b.attributes)
        && match (&a.kind, &b.kind) {
            (
                ValueKind::Function {
                    alias: xa,
                    parameters: a,
                    language: x,
                    mangling: xm,
                    ..
                },
                ValueKind::Function {
                    alias: ya,
                    parameters: b,
                    language: y,
                    mangling: ym,
                    ..
                },
            ) => {
                x == y
                    && (!symbol || xm == ym)
                    && xa.as_ref().map(|alias| (&alias.name, &alias.usr))
                        == ya.as_ref().map(|alias| (&alias.name, &alias.usr))
                    && parameters_equal(a, b)
            }
            (
                ValueKind::Constant {
                    value: a,
                    linkage: x,
                    origin: xo,
                },
                ValueKind::Constant {
                    value: b,
                    linkage: y,
                    origin: yo,
                },
            ) => {
                // Probe storage is synthetic, not the native symbol's linkage.
                xo == yo
                    && (xo == &ConstantOrigin::Macro || x == y)
                    && (a.is_none() || b.is_none() || a == b)
            }
            _ => false,
        }
}

fn value_attributes(
    snapshots: &[&Snapshot],
    owner: Owner,
    value: &ValueObservation,
) -> Result<(), Error> {
    attributes(snapshots, owner, &value.attributes)?;
    if let ValueKind::Function { parameters, .. } = &value.kind {
        for parameter in parameters {
            attributes(snapshots, owner, &parameter.attributes)?;
        }
    }
    Ok(())
}

fn no_trivial_auto_var_init(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|value| value.kind == AttributeKind::NoTrivialAutoVarInit)
}

fn default_type_visibility(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.kind == AttributeKind::DefaultTypeVisibility)
}

fn attributes(
    snapshots: &[&Snapshot],
    owner: Owner,
    attributes: &[Attribute],
) -> Result<(), Error> {
    for attribute in attributes {
        if let AttributeKind::Aligned { bits, .. } = attribute.kind
            && bits != 0
            && (bits < 8 || !bits.is_power_of_two())
        {
            return Err(Error(format!(
                "{}: unsupported alignment {bits} bits",
                describe(snapshots, owner)
            )));
        }
        if let AttributeKind::MaxFieldAlignment { bits } = attribute.kind
            && (!matches!(bits, 8 | 16 | 32 | 64 | 128) || packing(attributes).1 != Some(bits))
        {
            return Err(Error(format!(
                "{}: unsupported packing {bits} bits",
                describe(snapshots, owner)
            )));
        }
        if let AttributeKind::Unresolved(kind) = &attribute.kind {
            return Err(Error(format!(
                "{}: cannot verify {kind} attribute",
                describe(snapshots, owner)
            )));
        }
    }
    Ok(())
}
