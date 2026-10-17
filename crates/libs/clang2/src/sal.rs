use super::*;

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
    pub initialized: Option<InitializedLength>,
    pub terminated: bool,
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
}

#[derive(Clone, Copy)]
enum ContractKind {
    Value,
    Buffer(Units),
    Terminated,
    TerminatedBuffer,
    InitializedBuffer(Units),
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ParameterAnnotation<'a> {
    Contract(ParameterContract),
    Reserved,
    Evidence(&'a SalAnnotation),
}

pub(crate) fn parameter(
    value: &SalAnnotation,
    parameters: &[Parameter],
) -> Option<ParameterContract> {
    let (direction, optional, kind) = shape(value)?;
    let mut contract = ParameterContract {
        direction,
        optional,
        capacity: None,
        initialized: None,
        terminated: false,
    };
    match kind {
        ContractKind::Value => {}
        ContractKind::Terminated => contract.terminated = true,
        ContractKind::Buffer(units) => {
            contract.capacity = Some(count(&value.expanded, parameters, units)?);
        }
        ContractKind::TerminatedBuffer => {
            contract.capacity = Some(count(&value.expanded, parameters, Units::Elements)?);
            contract.terminated = true;
        }
        ContractKind::InitializedBuffer(units) => {
            let (capacity, length) = value.expanded.split_once(',')?;
            contract.capacity = Some(count(capacity, parameters, units)?);
            contract.initialized = Some(initialized(length, parameters, units)?);
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
    Some(InitializedLength {
        count: count(name.strip_prefix('*').unwrap_or(name), parameters, units)?,
        dereference: name.starts_with('*'),
    })
}

#[cfg(feature = "projection")]
#[derive(Clone, Copy)]
enum RpcBufferKind {
    Capacity,
    Partial,
    Full,
    Uncounted,
}

#[cfg(feature = "projection")]
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

#[cfg(feature = "projection")]
pub(crate) fn rpc_buffer(
    value: &SalAnnotation,
    parameters: &[Parameter],
) -> Option<ParameterContract> {
    let (direction, optional, kind, terminated) = rpc_buffer_shape(value)?;
    let mut contract = ParameterContract {
        direction,
        optional,
        capacity: None,
        initialized: None,
        terminated,
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
        ContractKind::Buffer(units) => (value.expanded.as_str(), units),
        ContractKind::TerminatedBuffer => (value.expanded.as_str(), Units::Elements),
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
            "__RPC__in_string" | "__RPC__in_opt_string" | "__RPC__inout_opt_string" => {
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

#[cfg(feature = "projection")]
pub(crate) fn direction(value: &SalAnnotation) -> Option<(Direction, bool)> {
    if let Some((direction, optional, _, _)) = rpc_buffer_shape(value) {
        return Some((direction, optional));
    }
    // Partial lowering preserves readonly, inner-pointer, and failure contracts as evidence.
    if value.arguments.is_empty() && value.expanded.is_empty() {
        match (value.version.as_str(), value.name.as_str()) {
            (
                "1.2",
                "__RPC__in" | "__RPC__in_string" | "__RPC__deref_in" | "__RPC__deref_in_opt",
            ) => {
                return Some((Direction::Input, false));
            }
            (
                "1.2",
                "__RPC__in_opt"
                | "__RPC__in_opt_string"
                | "__RPC__deref_opt_in"
                | "__RPC__deref_opt_in_opt",
            ) => {
                return Some((Direction::Input, true));
            }
            ("1.2", "__RPC__deref_out" | "__RPC__deref_out_opt")
            | (
                "2",
                "_Outptr_"
                | "_Outptr_result_maybenull_"
                | "_COM_Outptr_"
                | "_COM_Outptr_result_maybenull_",
            ) => return Some((Direction::Output, false)),
            (
                "2",
                "_Outptr_opt_"
                | "_Outptr_opt_result_maybenull_"
                | "_COM_Outptr_opt_"
                | "_COM_Outptr_opt_result_maybenull_",
            ) => return Some((Direction::Output, true)),
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
    let (direction, optional, kind) = match (value.version.as_str(), value.name.as_str()) {
        ("2", "_In_") | ("1", "__in") => (Direction::Input, false, Value),
        ("2", "_In_opt_") | ("1", "__in_opt") => (Direction::Input, true, Value),
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
            (Direction::Input, false, Buffer(Units::Elements))
        }
        ("2", "_In_reads_opt_") | ("1", "__in_ecount_opt") => {
            (Direction::Input, true, Buffer(Units::Elements))
        }
        ("2", "_In_reads_bytes_") | ("1", "__in_bcount") => {
            (Direction::Input, false, Buffer(Units::Bytes))
        }
        ("2", "_In_reads_bytes_opt_") | ("1", "__in_bcount_opt") => {
            (Direction::Input, true, Buffer(Units::Bytes))
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
        ("2", "_In_z_") | ("1", "__in_z") => (Direction::Input, false, Terminated),
        ("2", "_In_opt_z_") | ("1", "__in_z_opt") => (Direction::Input, true, Terminated),
        ("2", "_Out_writes_z_") => (Direction::Output, false, TerminatedBuffer),
        ("2", "_Out_writes_opt_z_") => (Direction::Output, true, TerminatedBuffer),
        ("2", "_Inout_updates_z_") | ("1", "__inout_ecount_z") => {
            (Direction::InputOutput, false, TerminatedBuffer)
        }
        ("2", "_Inout_updates_opt_z_") => (Direction::InputOutput, true, TerminatedBuffer),
        ("2", "_Out_writes_to_") | ("1", "__out_ecount_part") => {
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
    if matches!(kind, Value | Terminated)
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
