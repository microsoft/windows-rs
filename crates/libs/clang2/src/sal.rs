use super::*;

pub(crate) mod transport;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Direction {
    Input,
    Output,
    InputOutput,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ParameterContract {
    pub direction: Direction,
    pub optional: bool,
    pub capacity: Option<Count>,
    pub post_writable: bool,
    pub initialized: Option<InitializedLength>,
    pub terminated: bool,
    pub returned_pointer: Option<ReturnedPointer>,
    pub input_read_only: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ReturnedPointer {
    pub may_be_null: bool,
    pub null_on_failure: bool,
    pub com: bool,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Units {
    Elements,
    Bytes,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Count {
    pub parameter: usize,
    pub units: Units,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct InitializedLength {
    pub count: Count,
    pub dereference: bool,
    pub null_fallback: Option<usize>,
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
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ParameterAnnotation<'a> {
    Contract(ParameterContract),
    Reserved,
    Evidence(&'a SalAnnotation),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct FieldContract {
    pub capacity: usize,
    pub readable: Option<usize>,
    pub units: Units,
    pub optional: bool,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum FieldAnnotation<'a> {
    Contract(FieldContract),
    Evidence(&'a SalAnnotation),
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
    let count = |name: &str| {
        fields.iter().position(|field| {
            !field.name.is_empty() && field.name == name.trim() && field.bit_width.is_none()
        })
    };
    let (capacity, readable) = if partial {
        let (capacity, readable) = value.expanded.split_once(',')?;
        (count(capacity)?, Some(count(readable)?))
    } else {
        let capacity = count(&value.expanded)?;
        (capacity, full.then_some(capacity))
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
    let mut contracts = values.iter().filter_map(|value| field(value, fields));
    let first = contracts.next();
    let distinct = contracts.any(|value| Some(value) != first);
    let mut result: Vec<_> = values
        .into_iter()
        .map(|value| {
            if !distinct && let Some(contract) = field(value, fields) {
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
        contract.capacity?;
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
        input_read_only: None,
    };
    match kind {
        ContractKind::Value => {}
        ContractKind::Input(read_only) => contract.input_read_only = Some(read_only),
        ContractKind::OutPointer(pointer) => contract.returned_pointer = Some(pointer),
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
            contract.capacity = Some(count);
            if matches!(kind, ContractKind::FullBuffer(_)) {
                contract.initialized = Some(InitializedLength {
                    count,
                    dereference: false,
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
    let name = name.trim();
    let parameter = parameters
        .iter()
        .position(|parameter| !parameter.name.is_empty() && parameter.name == name)?;
    Some(Count { parameter, units })
}

fn initialized(name: &str, parameters: &[Parameter], units: Units) -> Option<InitializedLength> {
    let name = name.trim();
    if let Some((condition, branches)) = name.split_once('?') {
        let (present, absent) = branches.split_once(':')?;
        let guard = count(condition, parameters, units)?;
        let present = count(present.trim().strip_prefix('*')?, parameters, units)?;
        if guard != present {
            return None;
        }
        return Some(InitializedLength {
            count: present,
            dereference: true,
            null_fallback: Some(count(absent, parameters, units)?.parameter),
        });
    }
    Some(InitializedLength {
        count: count(name.strip_prefix('*').unwrap_or(name), parameters, units)?,
        dereference: name.starts_with('*'),
        null_fallback: None,
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
    value: &SalAnnotation,
    contract: Option<ParameterContract>,
) -> Option<(bool, bool)> {
    if value.version == "1.2" && value.arguments.is_empty() && value.expanded.is_empty() {
        match value.name.as_str() {
            "__RPC__inout_opt_string" => {
                return Some((true, false));
            }
            "__RPC__inout_string" => return Some((true, true)),
            _ => {}
        }
    }
    let contract = contract?;
    contract.terminated.then_some((
        contract.direction != Direction::Output,
        contract.direction != Direction::Input,
    ))
}

pub(crate) fn direction(value: &SalAnnotation) -> Option<(Direction, bool)> {
    if let Some((direction, optional, _, _)) = rpc_buffer_shape(value) {
        return Some((direction, optional));
    }
    if value.version == "2" && !value.arguments.is_empty() && !value.expanded.is_empty() {
        match value.name.as_str() {
            "_Outptr_result_buffer_"
            | "_Outptr_result_buffer_to_"
            | "_Outptr_result_buffer_all_"
            | "_Outptr_result_buffer_maybenull_"
            | "_Outptr_result_buffer_to_maybenull_"
            | "_Outptr_result_buffer_all_maybenull_"
            | "_Outptr_result_bytebuffer_"
            | "_Outptr_result_bytebuffer_to_"
            | "_Outptr_result_bytebuffer_all_"
            | "_Outptr_result_bytebuffer_maybenull_"
            | "_Outptr_result_bytebuffer_to_maybenull_"
            | "_Outptr_result_bytebuffer_all_maybenull_" => {
                return Some((Direction::Output, false));
            }
            "_Outptr_opt_result_buffer_"
            | "_Outptr_opt_result_buffer_to_"
            | "_Outptr_opt_result_buffer_all_"
            | "_Outptr_opt_result_buffer_maybenull_"
            | "_Outptr_opt_result_buffer_to_maybenull_"
            | "_Outptr_opt_result_buffer_all_maybenull_"
            | "_Outptr_opt_result_bytebuffer_"
            | "_Outptr_opt_result_bytebuffer_to_"
            | "_Outptr_opt_result_bytebuffer_all_"
            | "_Outptr_opt_result_bytebuffer_maybenull_"
            | "_Outptr_opt_result_bytebuffer_to_maybenull_"
            | "_Outptr_opt_result_bytebuffer_all_maybenull_" => {
                return Some((Direction::Output, true));
            }
            _ => {}
        }
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

fn shape(value: &SalAnnotation) -> Option<(Direction, bool, ContractKind)> {
    use ContractKind::*;
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
        ("1.2", "__RPC__deref_out" | "__RPC__deref_out_opt")
        | ("1", "__deref_out")
        | ("2", "_Outptr_") => (
            Direction::Output,
            false,
            OutPointer(ReturnedPointer {
                may_be_null: false,
                null_on_failure: false,
                com: false,
            }),
        ),
        ("2", "_Outptr_opt_") | ("1", "__deref_opt_out") => (
            Direction::Output,
            true,
            OutPointer(ReturnedPointer {
                may_be_null: false,
                null_on_failure: false,
                com: false,
            }),
        ),
        ("2", "_Outptr_result_maybenull_" | "_Outptr_opt_result_maybenull_")
        | ("1", "__deref_out_opt" | "__deref_opt_out_opt") => (
            Direction::Output,
            matches!(
                value.name.as_str(),
                "_Outptr_opt_result_maybenull_" | "__deref_opt_out_opt"
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
        ("2", "_In_reads_") | ("1", "__in_ecount") => {
            (Direction::Input, false, InputBuffer(Units::Elements, true))
        }
        ("2", "_In_reads_opt_") | ("1", "__in_ecount_opt") => {
            (Direction::Input, true, InputBuffer(Units::Elements, true))
        }
        ("2", "_In_reads_bytes_") | ("1", "__in_bcount") => {
            (Direction::Input, false, InputBuffer(Units::Bytes, true))
        }
        ("2", "_In_reads_bytes_opt_") | ("1", "__in_bcount_opt") => {
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
        ("2", "_Out_writes_to_") | ("1", "__out_ecount_part") => {
            (Direction::Output, false, InitializedBuffer(Units::Elements))
        }
        ("1.2", "__RPC__out_ecount_part")
            if !value.arguments.trim().is_empty() && !value.expanded.trim().is_empty() =>
        {
            (Direction::Output, false, InitializedBuffer(Units::Elements))
        }
        ("2", "_Out_writes_to_opt_") => {
            (Direction::Output, true, InitializedBuffer(Units::Elements))
        }
        ("2", "_Out_writes_bytes_to_") | ("1", "__out_bcount_part") => {
            (Direction::Output, false, InitializedBuffer(Units::Bytes))
        }
        ("2", "_Out_writes_bytes_to_opt_") => {
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
    if matches!(kind, Value | Input(_) | Terminated(_) | OutPointer(_))
        && (!value.arguments.is_empty() || !value.expanded.is_empty())
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
    let mut contracts = values
        .iter()
        .filter_map(|value| parameter(value, parameters));
    let first = contracts.next();
    let distinct = contracts.any(|value| Some(value) != first);
    let mut result: Vec<_> = values
        .into_iter()
        .map(|value| {
            // Do not replace several written contracts with their permissive union.
            if !distinct && let Some(contract) = parameter(value, parameters) {
                ParameterAnnotation::Contract(contract)
            } else if matches!(
                (value.version.as_str(), value.name.as_str()),
                ("2", "_Reserved_") | ("1", "__reserved")
            ) && value.arguments.is_empty()
                && value.expanded.is_empty()
            {
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
