use super::*;

mod expression;
pub(crate) mod transport;
pub(crate) use expression::Expression;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Annotation<'a> {
    pub value: &'a SalAnnotation,
    pub modifiers: Vec<&'a SalAnnotation>,
}

impl<'a> Annotation<'a> {
    pub fn unscoped(&self) -> Option<&'a SalAnnotation> {
        self.modifiers.is_empty().then_some(self.value)
    }
}

pub(crate) fn modifier(value: &SalAnnotation) -> bool {
    value.version == "2"
        && value.arguments.is_empty()
        && value.expanded.is_empty()
        && matches!(
            value.name.as_str(),
            "_Pre_" | "_Post_" | "_Notref_" | "_Pre_defensive_" | "_Post_defensive_"
        )
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Termination {
    pub on_entry: bool,
    pub on_success: bool,
    pub double: bool,
}

fn scoped_termination(value: &Annotation<'_>) -> Option<Termination> {
    let [modifier] = value.modifiers.as_slice() else {
        return None;
    };
    let on_entry = match modifier.name.as_str() {
        "_Pre_" => true,
        "_Post_" => false,
        _ => return None,
    };
    let value = value.value;
    if value.version != "2" || !value.arguments.is_empty() || !value.expanded.is_empty() {
        return None;
    }
    let double = match value.name.as_str() {
        "_Null_terminated_" => false,
        "_NullNull_terminated_" => true,
        _ => return None,
    };
    Some(Termination {
        on_entry,
        on_success: !on_entry,
        double,
    })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Direction {
    Input,
    Output,
    InputOutput,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ParameterContract {
    pub direction: Direction,
    pub optional: bool,
    pub capacity: Option<Count>,
    pub post_writable: bool,
    pub initialized: Option<InitializedLength>,
    pub terminated: bool,
    pub returned_pointer: Option<ReturnedPointer>,
    pub returned_buffer: Option<ReturnedBuffer>,
    pub returned_string_on_entry: bool,
    pub returned_nullable_on_entry: bool,
    // Native agreement retains SAL/RPC distinctions even when publication uses the same direction.
    pub input_read_only: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ReturnedPointer {
    pub may_be_null: bool,
    pub null_on_failure: bool,
    pub com: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ReturnedBuffer {
    pub writable: Option<Count>,
    pub readable: Option<Count>,
    pub nullable: bool,
}

#[derive(Clone, Copy)]
enum ReturnedBufferKind {
    Capacity,
    Partial,
    Full,
    FullWritable(bool),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Units {
    Elements,
    Bytes,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Count {
    pub source: CountSource,
    pub units: Units,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum CountSource {
    Constant(u64),
    Parameter(usize),
    Dereference(usize),
    Expression(Expression),
}

impl Count {
    pub fn references(&self) -> Vec<(usize, bool)> {
        match &self.source {
            CountSource::Constant(_) => Vec::new(),
            CountSource::Parameter(index) => vec![(*index, false)],
            CountSource::Dereference(index) => vec![(*index, true)],
            CountSource::Expression(value) => value.references(),
        }
    }

    #[cfg(feature = "projection")]
    pub fn parameter(&self) -> Option<(usize, bool)> {
        match &self.source {
            CountSource::Constant(_) => None,
            CountSource::Parameter(index) => Some((*index, false)),
            CountSource::Dereference(index) => Some((*index, true)),
            CountSource::Expression(_) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct InitializedLength {
    pub count: Count,
    pub null_fallback: Option<usize>,
    pub on_entry: bool,
}

#[derive(Clone, Copy)]
enum ContractKind {
    Value,
    Input(bool),
    InputBuffer(Units, bool),
    Buffer(Units),
    FullBuffer(Units),
    Terminated(bool),
    InputTerminatedBuffer(bool),
    TerminatedBuffer,
    InitializedBuffer(Units),
    OutPointer(ReturnedPointer),
    OutString {
        nullable: bool,
        on_entry: bool,
    },
    OutBuffer {
        units: Units,
        kind: ReturnedBufferKind,
        nullable: bool,
        nullable_on_entry: bool,
    },
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ParameterAnnotation<'a> {
    Contract(ParameterContract),
    Termination(Termination),
    Reserved,
    Evidence(Annotation<'a>),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct FieldContract {
    pub capacity: Expression,
    pub readable: Option<Expression>,
    pub units: Units,
    pub optional: bool,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum FieldAnnotation<'a> {
    Contract(FieldContract),
    Evidence(Annotation<'a>),
}

fn field(value: &SalAnnotation, fields: &[Field]) -> Option<FieldContract> {
    if value.version != "2" || value.arguments.trim().is_empty() || value.expanded.trim().is_empty()
    {
        return None;
    }
    let (units, optional, full, partial) = match value.name.as_str() {
        "_Field_size_" => (Units::Elements, false, false, false),
        "_Field_size_opt_" => (Units::Elements, true, false, false),
        "_Field_size_full_" => (Units::Elements, false, true, false),
        "_Field_size_full_opt_" => (Units::Elements, true, true, false),
        "_Field_size_part_" => (Units::Elements, false, false, true),
        "_Field_size_part_opt_" => (Units::Elements, true, false, true),
        "_Field_size_bytes_" => (Units::Bytes, false, false, false),
        "_Field_size_bytes_opt_" => (Units::Bytes, true, false, false),
        "_Field_size_bytes_full_" => (Units::Bytes, false, true, false),
        "_Field_size_bytes_full_opt_" => (Units::Bytes, true, true, false),
        "_Field_size_bytes_part_" => (Units::Bytes, false, false, true),
        "_Field_size_bytes_part_opt_" => (Units::Bytes, true, false, true),
        _ => return None,
    };
    let names: Vec<_> = fields.iter().map(|field| field.name.as_str()).collect();
    let count = |text: &str| {
        let expression = Expression::parse(text, &names)?;
        expression
            .references()
            .iter()
            .all(|&(index, dereference)| !dereference && fields[index].bit_width.is_none())
            .then_some(expression)
    };
    let (capacity, readable) = if partial {
        let (capacity, readable) = value.expanded.split_once(',')?;
        (count(capacity)?, Some(count(readable)?))
    } else {
        let capacity = count(&value.expanded)?;
        let readable = full.then(|| capacity.clone());
        (capacity, readable)
    };
    Some(FieldContract {
        capacity,
        readable,
        units,
        optional,
    })
}

pub(crate) fn fields<'a>(
    attributes: &'a [Attribute],
    fields: &[Field],
) -> Vec<FieldAnnotation<'a>> {
    let values = resolve::sal(attributes);
    let mut contracts = values
        .iter()
        .filter_map(|value| value.unscoped().and_then(|value| field(value, fields)));
    let first = contracts.next();
    let distinct = contracts.any(|value| Some(&value) != first.as_ref());
    let mut result: Vec<_> = values
        .into_iter()
        .map(|value| {
            if !distinct
                && let Some(contract) = value.unscoped().and_then(|value| field(value, fields))
            {
                FieldAnnotation::Contract(contract)
            } else {
                FieldAnnotation::Evidence(value)
            }
        })
        .collect();
    result.sort_unstable();
    result.dedup();
    result
}

pub(crate) fn parameter(
    value: &SalAnnotation,
    parameters: &[Parameter],
) -> Option<ParameterContract> {
    if let Some((direction, _, RpcBufferKind::Capacity | RpcBufferKind::Full, _)) =
        rpc_buffer_shape(value)
        && direction != Direction::Input
    {
        let contract = rpc_buffer(value, parameters)?;
        contract.capacity.as_ref()?;
        return Some(contract);
    }
    let (direction, optional, kind) = shape(value)?;
    let mut contract = ParameterContract {
        direction,
        optional,
        capacity: None,
        post_writable: false,
        initialized: None,
        terminated: false,
        returned_pointer: None,
        returned_buffer: None,
        returned_string_on_entry: false,
        returned_nullable_on_entry: false,
        input_read_only: None,
    };
    match kind {
        ContractKind::Value => {}
        ContractKind::Input(read_only) => contract.input_read_only = Some(read_only),
        ContractKind::OutPointer(pointer) => contract.returned_pointer = Some(pointer),
        ContractKind::OutString { nullable, on_entry } => {
            contract.returned_pointer = Some(ReturnedPointer {
                may_be_null: nullable,
                null_on_failure: false,
                com: false,
            });
            contract.terminated = true;
            contract.returned_string_on_entry = on_entry;
        }
        ContractKind::OutBuffer {
            units,
            kind,
            nullable,
            nullable_on_entry,
        } => {
            contract.terminated = matches!(kind, ReturnedBufferKind::FullWritable(true));
            contract.returned_nullable_on_entry = nullable_on_entry;
            let (writable, readable) = match kind {
                ReturnedBufferKind::Capacity => {
                    (Some(count(&value.expanded, parameters, units)?), None)
                }
                ReturnedBufferKind::Full => {
                    (None, Some(count(&value.expanded, parameters, units)?))
                }
                ReturnedBufferKind::FullWritable(_) => {
                    let count = count(&value.expanded, parameters, units)?;
                    (Some(count.clone()), Some(count))
                }
                ReturnedBufferKind::Partial => {
                    let (capacity, length) = value.expanded.split_once(',')?;
                    (
                        Some(count(capacity, parameters, units)?),
                        Some(count(length, parameters, units)?),
                    )
                }
            };
            contract.returned_buffer = Some(ReturnedBuffer {
                writable,
                readable,
                nullable,
            });
        }
        ContractKind::Terminated(read_only) => {
            contract.input_read_only = Some(read_only);
            contract.terminated = true;
        }
        ContractKind::InputBuffer(units, read_only) => {
            contract.input_read_only = Some(read_only);
            contract.capacity = Some(count(&value.expanded, parameters, units)?);
        }
        ContractKind::InputTerminatedBuffer(read_only) => {
            contract.input_read_only = Some(read_only);
            contract.capacity = Some(count(&value.expanded, parameters, Units::Elements)?);
            contract.terminated = true;
        }
        ContractKind::Buffer(units) | ContractKind::FullBuffer(units) => {
            let count = count(&value.expanded, parameters, units)?;
            contract.capacity = Some(count.clone());
            if matches!(kind, ContractKind::FullBuffer(_)) {
                contract.initialized = Some(InitializedLength {
                    on_entry: count
                        .references()
                        .iter()
                        .any(|(_, dereference)| *dereference),
                    count,
                    null_fallback: None,
                });
            }
        }
        ContractKind::TerminatedBuffer => {
            contract.capacity = Some(count(&value.expanded, parameters, Units::Elements)?);
            contract.terminated = true;
        }
        ContractKind::InitializedBuffer(units) => {
            let (capacity, length) = value.expanded.split_once(',')?;
            contract.capacity = Some(count(capacity, parameters, units)?);
            contract.initialized = Some(initialized(length, parameters, units)?);
            contract.post_writable =
                value.version == "1.2" && value.name == "__RPC__out_ecount_part";
        }
    }
    Some(contract)
}

fn count(name: &str, parameters: &[Parameter], units: Units) -> Option<Count> {
    let names: Vec<_> = parameters.iter().map(|value| value.name.as_str()).collect();
    let source = match Expression::parse(name, &names)? {
        Expression::Constant(value) => CountSource::Constant(value.try_into().ok()?),
        Expression::Parameter(index) => CountSource::Parameter(index),
        Expression::Dereference(index) => CountSource::Dereference(index),
        value => CountSource::Expression(value),
    };
    Some(Count { source, units })
}

// Recognize a bounded count vocabulary, not general C constant expressions.
fn count_literal(text: &str) -> Option<u64> {
    let text = count_group(text);
    if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        if hex.is_empty() || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return None;
        }
        return u64::from_str_radix(hex, 16)
            .ok()
            .filter(|value| *value <= i32::MAX as u64);
    }
    if text.is_empty()
        || text.len() > 1 && text.starts_with('0')
        || !text.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    text.parse().ok().filter(|value| *value <= i32::MAX as u64)
}

fn count_group(mut text: &str) -> &str {
    text = text.trim();
    while text.starts_with('(') && text.ends_with(')') {
        let mut depth = 0;
        let wraps = text.bytes().enumerate().all(|(index, byte)| {
            match byte {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            depth > 0 || depth == 0 && index == text.len() - 1
        });
        if !wraps || depth != 0 {
            break;
        }
        text = text[1..text.len() - 1].trim();
    }
    text
}

fn initialized(name: &str, parameters: &[Parameter], units: Units) -> Option<InitializedLength> {
    let direct = |name| {
        let count = count(name, parameters, units)?;
        let CountSource::Parameter(index) = count.source else {
            return None;
        };
        Some(index)
    };
    let name = count_group(name);
    if let Some((condition, branches)) = name.split_once('?') {
        let (present, absent) = branches.split_once(':')?;
        let guard = direct(condition)?;
        let present = count(present, parameters, units)?;
        if !matches!(present.references().as_slice(), [(index, true)] if *index == guard) {
            return None;
        }
        return Some(InitializedLength {
            count: present,
            null_fallback: Some(direct(absent)?),
            on_entry: false,
        });
    }
    Some(InitializedLength {
        count: count(name, parameters, units)?,
        null_fallback: None,
        on_entry: false,
    })
}

#[derive(Clone, Copy)]
enum RpcBufferKind {
    Capacity,
    Partial,
    Full,
    Uncounted,
}

fn rpc_buffer_shape(value: &SalAnnotation) -> Option<(Direction, bool, RpcBufferKind, bool)> {
    use Direction::*;
    use RpcBufferKind::*;
    if value.version != "1.2"
        || value.arguments.trim().is_empty()
        || value.expanded.trim().is_empty()
    {
        return None;
    }
    Some(match value.name.as_str() {
        "__RPC__in_ecount"
        | "__RPC__in_ecount_full"
        | "__RPC__in_xcount"
        | "__RPC__in_xcount_full" => (Input, false, Capacity, false),
        "__RPC__in_ecount_full_string" | "__RPC__in_xcount_full_string" => {
            (Input, false, Capacity, true)
        }
        "__RPC__in_ecount_opt"
        | "__RPC__in_ecount_full_opt"
        | "__RPC__in_xcount_opt"
        | "__RPC__in_xcount_full_opt" => (Input, true, Capacity, false),
        "__RPC__in_ecount_opt_string"
        | "__RPC__in_ecount_full_opt_string"
        | "__RPC__in_xcount_opt_string"
        | "__RPC__in_xcount_full_opt_string" => (Input, true, Capacity, true),
        "__RPC__in_ecount_part" | "__RPC__in_xcount_part" => (Input, false, Partial, false),
        "__RPC__in_ecount_part_opt" | "__RPC__in_xcount_part_opt" => (Input, true, Partial, false),
        "__RPC__out_ecount" => (Output, false, Capacity, false),
        "__RPC__out_ecount_string" => (Output, false, Capacity, true),
        "__RPC__out_ecount_part" => (Output, false, Partial, false),
        "__RPC__out_ecount_full" => (Output, false, Full, false),
        "__RPC__out_ecount_full_string" => (Output, false, Full, true),
        "__RPC__inout_ecount_full" => (InputOutput, false, Full, false),
        "__RPC__inout_ecount_full_string" => (InputOutput, false, Full, true),
        "__RPC__inout_ecount_full_opt" => (InputOutput, true, Full, false),
        "__RPC__inout_ecount_full_opt_string" => (InputOutput, true, Full, true),
        "__RPC__out_xcount" | "__RPC__out_xcount_part" | "__RPC__out_xcount_full" => {
            (Output, false, Uncounted, false)
        }
        "__RPC__out_xcount_string" | "__RPC__out_xcount_full_string" => {
            (Output, false, Uncounted, true)
        }
        "__RPC__inout_xcount" | "__RPC__inout_xcount_part" | "__RPC__inout_xcount_full" => {
            (InputOutput, false, Uncounted, false)
        }
        "__RPC__inout_xcount_full_string" => (InputOutput, false, Uncounted, true),
        "__RPC__inout_xcount_opt"
        | "__RPC__inout_xcount_part_opt"
        | "__RPC__inout_xcount_full_opt" => (InputOutput, true, Uncounted, false),
        "__RPC__inout_xcount_full_opt_string" => (InputOutput, true, Uncounted, true),
        _ => return None,
    })
}

pub(crate) fn rpc_buffer(
    value: &SalAnnotation,
    parameters: &[Parameter],
) -> Option<ParameterContract> {
    let (direction, optional, kind, terminated) = rpc_buffer_shape(value)?;
    let mut contract = ParameterContract {
        direction,
        optional,
        capacity: None,
        post_writable: direction == Direction::Output && !matches!(kind, RpcBufferKind::Uncounted),
        initialized: None,
        terminated,
        returned_pointer: None,
        returned_buffer: None,
        returned_string_on_entry: false,
        returned_nullable_on_entry: false,
        input_read_only: None,
    };
    match kind {
        RpcBufferKind::Uncounted => {}
        RpcBufferKind::Capacity | RpcBufferKind::Full => {
            contract.capacity = count(&value.expanded, parameters, Units::Elements);
            if matches!(kind, RpcBufferKind::Full) {
                contract.initialized = initialized(&value.expanded, parameters, Units::Elements);
            }
        }
        RpcBufferKind::Partial => {
            if let Some((capacity, length)) = value.expanded.split_once(',') {
                contract.capacity = count(capacity, parameters, Units::Elements);
                // Input partial lengths are entry conditions, not successful-output lengths.
                if direction != Direction::Input {
                    contract.initialized = initialized(length, parameters, Units::Elements);
                }
            }
        }
    }
    Some(contract)
}

#[cfg(feature = "projection")]
pub(crate) fn capacity(value: &SalAnnotation, parameters: &[Parameter]) -> Option<Count> {
    let (_, _, kind) = shape(value)?;
    let (name, units) = match kind {
        ContractKind::InputBuffer(units, _)
        | ContractKind::Buffer(units)
        | ContractKind::FullBuffer(units) => (value.expanded.as_str(), units),
        ContractKind::InputTerminatedBuffer(_) | ContractKind::TerminatedBuffer => {
            (value.expanded.as_str(), Units::Elements)
        }
        ContractKind::InitializedBuffer(units) => (value.expanded.split_once(',')?.0, units),
        _ => return None,
    };
    count(name, parameters, units)
}

#[cfg(feature = "projection")]
pub(crate) fn termination(
    value: &Annotation<'_>,
    contract: Option<&ParameterContract>,
) -> Option<Termination> {
    if let Some(termination) = scoped_termination(value) {
        return Some(termination);
    }
    let value = value.unscoped()?;
    if value.version == "1.2" && value.arguments.is_empty() && value.expanded.is_empty() {
        match value.name.as_str() {
            "__RPC__inout_opt_string" => {
                return Some(Termination {
                    on_entry: true,
                    on_success: false,
                    double: false,
                });
            }
            "__RPC__inout_string" => {
                return Some(Termination {
                    on_entry: true,
                    on_success: true,
                    double: false,
                });
            }
            _ => {}
        }
    }
    let contract = contract?;
    if contract.returned_pointer.is_some() || contract.returned_buffer.is_some() {
        return None;
    }
    contract.terminated.then_some(Termination {
        on_entry: contract.direction != Direction::Output,
        on_success: contract.direction != Direction::Input,
        double: false,
    })
}

#[cfg(feature = "projection")]
pub(crate) fn common_direction(
    snapshot: &Snapshot,
    function: &Function,
    attributes: &[Attribute],
    parameters: &[Parameter],
) -> Option<(bool, bool, bool)> {
    let values = resolve::sal(attributes);
    let conditions: Vec<_> = values
        .iter()
        .filter(|value| value.value.name == "_When_")
        .collect();
    let [first, second] = conditions.as_slice() else {
        return None;
    };
    let branch = |value: &Annotation<'_>| {
        let value = value.unscoped()?;
        if value.version != "2" || value.arguments.is_empty() {
            return None;
        }
        let (condition, annotation) = value.expanded.split_once(',')?;
        let (input, output, optional) = match annotation.trim() {
            "_In_" => (true, false, false),
            "_In_opt_" => (true, false, true),
            "_Out_" => (false, true, false),
            "_Out_opt_" => (false, true, true),
            "_Inout_" => (true, true, false),
            "_Inout_opt_" => (true, true, true),
            _ => return None,
        };
        let (name, right, equal) = if let Some((name, right)) = condition.split_once("==") {
            (name.trim(), right.trim(), true)
        } else if let Some((name, right)) = condition.split_once("!=") {
            (name.trim(), right.trim(), false)
        } else {
            return None;
        };
        if right != "0" {
            return None;
        }
        let position = parameters
            .iter()
            .position(|parameter| !name.is_empty() && parameter.name == name)?;
        let mut ty = function.parameters[position];
        while let TypeKind::Alias(id) = snapshot.types[ty.0].kind {
            ty = snapshot.aliases[id.0].observations[0].canonical;
        }
        if !matches!(snapshot.types[ty.0].kind, TypeKind::Pointer(_)) {
            return None;
        }
        Some((position, equal, input, output, optional))
    };
    let first = branch(first)?;
    let second = branch(second)?;
    if first.0 != second.0 || first.1 == second.1 {
        return None;
    }
    // Complementary pointer-null tests cover every entry state. Keep both source conditions;
    // only their shared facts can be published without a conditional contract.
    let required = values.iter().any(|value| {
        value
            .unscoped()
            .and_then(direction)
            .is_some_and(|(_, optional)| !optional)
    });
    Some((
        first.2 && second.2,
        first.3 && second.3,
        first.4 && second.4 && !required,
    ))
}

#[cfg(feature = "projection")]
pub(crate) fn direction(value: &SalAnnotation) -> Option<(Direction, bool)> {
    if let Some((direction, optional, _, _)) = rpc_buffer_shape(value) {
        return Some((direction, optional));
    }
    // Partial lowering preserves inner-pointer and failure contracts as evidence.
    if value.arguments.is_empty() && value.expanded.is_empty() {
        match (value.version.as_str(), value.name.as_str()) {
            ("1.2", "__RPC__deref_in" | "__RPC__deref_in_opt") => {
                return Some((Direction::Input, false));
            }
            ("1.2", "__RPC__deref_opt_in" | "__RPC__deref_opt_in_opt") => {
                return Some((Direction::Input, true));
            }
            ("1.2", "__RPC__inout_string" | "__RPC__deref_inout" | "__RPC__deref_inout_opt") => {
                return Some((Direction::InputOutput, false));
            }
            (
                "1.2",
                "__RPC__inout_opt_string" | "__RPC__deref_opt_inout" | "__RPC__deref_opt_inout_opt",
            ) => {
                return Some((Direction::InputOutput, true));
            }
            _ => {}
        }
    }
    let (direction, optional, _) = shape(value)?;
    Some((direction, optional))
}

fn returned_buffer_shape(value: &SalAnnotation) -> Option<(bool, ContractKind)> {
    use ReturnedBufferKind::*;
    use Units::*;
    if value.version == "1.2"
        && !value.arguments.trim().is_empty()
        && !value.expanded.trim().is_empty()
    {
        let (kind, nullable) = match value.name.as_str() {
            "__RPC__deref_out_ecount" => (Capacity, false),
            "__RPC__deref_out_ecount_part" => (Partial, false),
            "__RPC__deref_out_ecount_full" => (FullWritable(false), false),
            "__RPC__deref_out_ecount_full_string" => (FullWritable(true), false),
            "__RPC__deref_out_ecount_opt" => (Capacity, true),
            "__RPC__deref_out_ecount_part_opt" => (Partial, true),
            "__RPC__deref_out_ecount_full_opt" => (FullWritable(false), true),
            "__RPC__deref_out_ecount_full_opt_string" => (FullWritable(true), true),
            _ => return None,
        };
        return Some((
            false,
            ContractKind::OutBuffer {
                units: Elements,
                kind,
                nullable,
                nullable_on_entry: nullable,
            },
        ));
    }
    if value.version != "2" || value.arguments.trim().is_empty() || value.expanded.trim().is_empty()
    {
        return None;
    }
    let (name, optional) = if let Some(name) = value.name.strip_prefix("_Outptr_result_") {
        (name, false)
    } else {
        (value.name.strip_prefix("_Outptr_opt_result_")?, true)
    };
    let (units, kind, nullable) = match name {
        "buffer_" => (Elements, Capacity, false),
        "buffer_to_" => (Elements, Partial, false),
        "buffer_all_" => (Elements, Full, false),
        "buffer_maybenull_" => (Elements, Capacity, true),
        "buffer_to_maybenull_" => (Elements, Partial, true),
        "buffer_all_maybenull_" => (Elements, Full, true),
        "bytebuffer_" => (Bytes, Capacity, false),
        "bytebuffer_to_" => (Bytes, Partial, false),
        "bytebuffer_all_" => (Bytes, Full, false),
        "bytebuffer_maybenull_" => (Bytes, Capacity, true),
        "bytebuffer_to_maybenull_" => (Bytes, Partial, true),
        "bytebuffer_all_maybenull_" => (Bytes, Full, true),
        _ => return None,
    };
    Some((
        optional,
        ContractKind::OutBuffer {
            units,
            kind,
            nullable,
            nullable_on_entry: false,
        },
    ))
}

fn shape(value: &SalAnnotation) -> Option<(Direction, bool, ContractKind)> {
    use ContractKind::*;
    if let Some((optional, kind)) = returned_buffer_shape(value) {
        return Some((Direction::Output, optional, kind));
    }
    if let Some((Direction::Input, optional, RpcBufferKind::Capacity, terminated)) =
        rpc_buffer_shape(value)
    {
        return Some((
            Direction::Input,
            optional,
            if terminated {
                InputTerminatedBuffer(false)
            } else {
                InputBuffer(Units::Elements, false)
            },
        ));
    }
    let (direction, optional, kind) = match (value.version.as_str(), value.name.as_str()) {
        ("1.2", "__RPC__deref_out_string") | ("2", "_Outptr_result_z_") => (
            Direction::Output,
            false,
            OutString {
                nullable: false,
                on_entry: false,
            },
        ),
        ("1.2", "__RPC__deref_out_opt_string") => (
            Direction::Output,
            false,
            OutString {
                nullable: true,
                on_entry: true,
            },
        ),
        ("2", "_Outptr_opt_result_z_") => (
            Direction::Output,
            true,
            OutString {
                nullable: false,
                on_entry: false,
            },
        ),
        ("2", "_Outptr_result_maybenull_z_" | "_Outptr_opt_result_maybenull_z_") => (
            Direction::Output,
            value.name == "_Outptr_opt_result_maybenull_z_",
            OutString {
                nullable: true,
                on_entry: false,
            },
        ),
        ("1.2", "__RPC__deref_out" | "__RPC__deref_out_opt")
        | ("1", "__deref_out")
        | ("1.1", "_Deref_out_")
        | ("2", "_Outptr_") => (
            Direction::Output,
            false,
            OutPointer(ReturnedPointer {
                may_be_null: false,
                null_on_failure: false,
                com: false,
            }),
        ),
        ("2", "_Outptr_opt_") | ("1", "__deref_opt_out") | ("1.1", "_Deref_opt_out_") => (
            Direction::Output,
            true,
            OutPointer(ReturnedPointer {
                may_be_null: false,
                null_on_failure: false,
                com: false,
            }),
        ),
        ("2", "_Outptr_result_maybenull_" | "_Outptr_opt_result_maybenull_")
        | ("1", "__deref_out_opt" | "__deref_opt_out_opt")
        | ("1.1", "_Deref_out_opt_" | "_Deref_opt_out_opt_") => (
            Direction::Output,
            matches!(
                value.name.as_str(),
                "_Outptr_opt_result_maybenull_" | "__deref_opt_out_opt" | "_Deref_opt_out_opt_"
            ),
            OutPointer(ReturnedPointer {
                may_be_null: true,
                null_on_failure: false,
                com: false,
            }),
        ),
        ("2", "_COM_Outptr_" | "_COM_Outptr_opt_") => (
            Direction::Output,
            value.name == "_COM_Outptr_opt_",
            OutPointer(ReturnedPointer {
                may_be_null: false,
                null_on_failure: true,
                com: true,
            }),
        ),
        ("2", "_Outptr_result_nullonfailure_" | "_Outptr_opt_result_nullonfailure_") => (
            Direction::Output,
            value.name == "_Outptr_opt_result_nullonfailure_",
            OutPointer(ReturnedPointer {
                may_be_null: false,
                null_on_failure: true,
                com: false,
            }),
        ),
        ("2", "_COM_Outptr_result_maybenull_" | "_COM_Outptr_opt_result_maybenull_") => (
            Direction::Output,
            value.name == "_COM_Outptr_opt_result_maybenull_",
            OutPointer(ReturnedPointer {
                may_be_null: true,
                null_on_failure: true,
                com: true,
            }),
        ),
        ("2", "_In_") | ("1", "__in") => (Direction::Input, false, Input(true)),
        ("2", "_In_opt_") | ("1", "__in_opt") => (Direction::Input, true, Input(true)),
        ("1.2", "__RPC__in") => (Direction::Input, false, Input(false)),
        ("1.2", "__RPC__in_opt") => (Direction::Input, true, Input(false)),
        ("2", "_Out_") | ("1", "__out") | ("1.2", "__RPC__out") => {
            (Direction::Output, false, Value)
        }
        ("2", "_Out_opt_") | ("1", "__out_opt") => (Direction::Output, true, Value),
        ("2", "_Inout_") | ("1", "__inout") | ("1.2", "__RPC__inout") => {
            (Direction::InputOutput, false, Value)
        }
        ("2", "_Inout_opt_") | ("1", "__inout_opt") | ("1.2", "__RPC__inout_opt") => {
            (Direction::InputOutput, true, Value)
        }
        ("2", "_In_reads_") | ("1", "__in_ecount") | ("1.1", "_In_count_") => {
            (Direction::Input, false, InputBuffer(Units::Elements, true))
        }
        ("2", "_In_reads_opt_") | ("1", "__in_ecount_opt") | ("1.1", "_In_opt_count_") => {
            (Direction::Input, true, InputBuffer(Units::Elements, true))
        }
        ("2", "_In_reads_bytes_") | ("1", "__in_bcount") | ("1.1", "_In_bytecount_") => {
            (Direction::Input, false, InputBuffer(Units::Bytes, true))
        }
        ("2", "_In_reads_bytes_opt_")
        | ("1", "__in_bcount_opt")
        | ("1.1", "_In_opt_bytecount_") => {
            (Direction::Input, true, InputBuffer(Units::Bytes, true))
        }
        ("2", "_Out_writes_") | ("1", "__out_ecount") => {
            (Direction::Output, false, Buffer(Units::Elements))
        }
        ("2", "_Out_writes_opt_") | ("1", "__out_ecount_opt") => {
            (Direction::Output, true, Buffer(Units::Elements))
        }
        ("2", "_Out_writes_bytes_") | ("1", "__out_bcount") => {
            (Direction::Output, false, Buffer(Units::Bytes))
        }
        ("2", "_Out_writes_bytes_opt_") | ("1", "__out_bcount_opt") => {
            (Direction::Output, true, Buffer(Units::Bytes))
        }
        ("2", "_Out_writes_all_") => (Direction::Output, false, FullBuffer(Units::Elements)),
        ("2", "_Out_writes_all_opt_") => (Direction::Output, true, FullBuffer(Units::Elements)),
        ("2", "_Out_writes_bytes_all_") => (Direction::Output, false, FullBuffer(Units::Bytes)),
        ("2", "_Out_writes_bytes_all_opt_") => (Direction::Output, true, FullBuffer(Units::Bytes)),
        ("2", "_Inout_updates_") | ("1", "__inout_ecount") | ("1.2", "__RPC__inout_ecount") => {
            (Direction::InputOutput, false, Buffer(Units::Elements))
        }
        ("2", "_Inout_updates_opt_") | ("1.2", "__RPC__inout_ecount_opt") => {
            (Direction::InputOutput, true, Buffer(Units::Elements))
        }
        ("2", "_Inout_updates_bytes_") | ("1", "__inout_bcount") => {
            (Direction::InputOutput, false, Buffer(Units::Bytes))
        }
        ("2", "_Inout_updates_bytes_opt_") => (Direction::InputOutput, true, Buffer(Units::Bytes)),
        ("2", "_Inout_updates_all_") => {
            (Direction::InputOutput, false, FullBuffer(Units::Elements))
        }
        ("2", "_Inout_updates_all_opt_") => {
            (Direction::InputOutput, true, FullBuffer(Units::Elements))
        }
        ("2", "_Inout_updates_bytes_all_") => {
            (Direction::InputOutput, false, FullBuffer(Units::Bytes))
        }
        ("2", "_Inout_updates_bytes_all_opt_") => {
            (Direction::InputOutput, true, FullBuffer(Units::Bytes))
        }
        ("2", "_In_z_") | ("1", "__in_z") => (Direction::Input, false, Terminated(true)),
        ("2", "_In_opt_z_") | ("1", "__in_z_opt") => (Direction::Input, true, Terminated(true)),
        ("1.2", "__RPC__in_string") => (Direction::Input, false, Terminated(false)),
        ("1.2", "__RPC__in_opt_string") => (Direction::Input, true, Terminated(false)),
        ("2", "_Out_writes_z_") => (Direction::Output, false, TerminatedBuffer),
        ("2", "_Out_writes_opt_z_") => (Direction::Output, true, TerminatedBuffer),
        ("2", "_Inout_updates_z_") | ("1", "__inout_ecount_z") => {
            (Direction::InputOutput, false, TerminatedBuffer)
        }
        ("2", "_Inout_updates_opt_z_") => (Direction::InputOutput, true, TerminatedBuffer),
        ("2", "_Out_writes_to_") | ("1", "__out_ecount_part") | ("1.1", "_Out_cap_post_count_") => {
            (Direction::Output, false, InitializedBuffer(Units::Elements))
        }
        ("1.2", "__RPC__out_ecount_part")
            if !value.arguments.trim().is_empty() && !value.expanded.trim().is_empty() =>
        {
            (Direction::Output, false, InitializedBuffer(Units::Elements))
        }
        ("2", "_Out_writes_to_opt_") | ("1.1", "_Out_opt_cap_post_count_") => {
            (Direction::Output, true, InitializedBuffer(Units::Elements))
        }
        ("2", "_Out_writes_bytes_to_") | ("1", "__out_bcount_part") => {
            (Direction::Output, false, InitializedBuffer(Units::Bytes))
        }
        ("2", "_Out_writes_bytes_to_opt_") | ("1.1", "_Out_opt_bytecap_post_bytecount_") => {
            (Direction::Output, true, InitializedBuffer(Units::Bytes))
        }
        ("2", "_Inout_updates_to_")
        | ("1", "__inout_ecount_part")
        | ("1.2", "__RPC__inout_ecount_part") => (
            Direction::InputOutput,
            false,
            InitializedBuffer(Units::Elements),
        ),
        ("2", "_Inout_updates_to_opt_") | ("1.2", "__RPC__inout_ecount_part_opt") => (
            Direction::InputOutput,
            true,
            InitializedBuffer(Units::Elements),
        ),
        ("2", "_Inout_updates_bytes_to_") | ("1", "__inout_bcount_part") => (
            Direction::InputOutput,
            false,
            InitializedBuffer(Units::Bytes),
        ),
        ("2", "_Inout_updates_bytes_to_opt_") => (
            Direction::InputOutput,
            true,
            InitializedBuffer(Units::Bytes),
        ),
        _ => return None,
    };
    if matches!(
        kind,
        Value | Input(_) | Terminated(_) | OutPointer(_) | OutString { .. }
    ) && (!value.arguments.is_empty() || !value.expanded.is_empty())
    {
        return None;
    }
    Some((direction, optional, kind))
}

pub(crate) fn parameters<'a>(
    attributes: &'a [Attribute],
    parameters: &[Parameter],
) -> Vec<ParameterAnnotation<'a>> {
    let values = resolve::sal(attributes);
    let mut contracts = values.iter().filter_map(|value| {
        value
            .unscoped()
            .and_then(|value| parameter(value, parameters))
    });
    let first = contracts.next();
    let has_contract = first.is_some();
    let mut combined =
        first.and_then(|first| contracts.try_fold(first, combine_returned_contracts));
    let distinct = has_contract && combined.is_none();
    // `_Result_nullonfailure_` adds a failure postcondition to the returned pointer contract.
    let mut null_on_failure = false;
    if values.iter().any(result_null_on_failure)
        && let Some(pointer) = combined
            .as_mut()
            .and_then(|contract| contract.returned_pointer.as_mut())
    {
        pointer.null_on_failure = true;
        null_on_failure = true;
    }
    let mut result: Vec<_> = values
        .into_iter()
        .filter(|value| !value.unscoped().is_some_and(analysis_only))
        .map(|value| {
            // Only matching returned-buffer/pointer facts can share a combined contract.
            if !distinct
                && (value
                    .unscoped()
                    .and_then(|value| parameter(value, parameters))
                    .is_some()
                    || null_on_failure && result_null_on_failure(&value))
            {
                ParameterAnnotation::Contract(combined.clone().unwrap())
            } else if let Some(termination) = scoped_termination(&value) {
                ParameterAnnotation::Termination(termination)
            } else if value.unscoped().is_some_and(|value| {
                matches!(
                    (value.version.as_str(), value.name.as_str()),
                    ("2", "_Reserved_") | ("1", "__reserved")
                ) && value.arguments.is_empty()
                    && value.expanded.is_empty()
            }) {
                ParameterAnnotation::Reserved
            } else {
                ParameterAnnotation::Evidence(value)
            }
        })
        .collect();
    result.sort_unstable();
    result.dedup();
    result
}

// Static-analysis directives that state no contract a caller can observe.
fn analysis_only(value: &SalAnnotation) -> bool {
    let arguments = !value.arguments.trim().is_empty();
    match (value.version.as_str(), value.name.as_str()) {
        ("L", "__in_data_source" | "__out_data_source")
        | ("2", "_Printf_format_string_params_") => arguments,
        (
            "2",
            "_Printf_format_string_"
            | "_Scanf_format_string_"
            | "_Scanf_s_format_string_"
            | "_IRQL_uses_cancel_"
            | "_Strict_type_match",
        ) => !arguments && value.expanded.trim().is_empty(),
        _ => false,
    }
}

fn result_null_on_failure(value: &Annotation<'_>) -> bool {
    value.unscoped().is_some_and(|value| {
        value.version == "2"
            && value.name == "_Result_nullonfailure_"
            && value.arguments.is_empty()
            && value.expanded.is_empty()
    })
}

fn combine_returned_contracts(
    mut left: ParameterContract,
    mut right: ParameterContract,
) -> Option<ParameterContract> {
    if left == right {
        return Some(left);
    }
    if [&left, &right]
        .iter()
        .any(|value| value.returned_pointer.is_none() && value.returned_buffer.is_none())
    {
        return None;
    }
    let pointer = left.returned_pointer.or(right.returned_pointer)?;
    let buffer = left
        .returned_buffer
        .as_ref()
        .or(right.returned_buffer.as_ref())?;
    if pointer.com
        || pointer.may_be_null != buffer.nullable
        || [&left, &right].iter().any(|value| {
            value.returned_pointer.is_some_and(|value| value != pointer)
                || value
                    .returned_buffer
                    .as_ref()
                    .is_some_and(|value| value != buffer)
        })
    {
        return None;
    }
    let buffer = buffer.clone();
    left.returned_pointer = None;
    left.returned_buffer = None;
    right.returned_pointer = None;
    right.returned_buffer = None;
    let terminated = left.terminated || right.terminated;
    left.terminated = false;
    right.terminated = false;
    let returned_string_on_entry = left.returned_string_on_entry || right.returned_string_on_entry;
    left.returned_string_on_entry = false;
    right.returned_string_on_entry = false;
    let returned_nullable_on_entry =
        left.returned_nullable_on_entry || right.returned_nullable_on_entry;
    left.returned_nullable_on_entry = false;
    right.returned_nullable_on_entry = false;
    if left != right {
        return None;
    }
    left.returned_pointer = Some(pointer);
    left.returned_buffer = Some(buffer);
    left.terminated = terminated;
    left.returned_string_on_entry = returned_string_on_entry;
    left.returned_nullable_on_entry = returned_nullable_on_entry;
    Some(left)
}
