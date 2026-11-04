use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BufferSize {
    Constant(u64),
    Parameter(u16),
    Dereference(u16),
}

/// A simple caller-supplied buffer extent at entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferInfo {
    pub count: BufferSize,
    pub bytes: bool,
}

impl BufferInfo {
    pub fn to_values(&self) -> Vec<(String, Value)> {
        let (name, value) = match self.count {
            BufferSize::Constant(value) => ("CountConst", Value::U64(value)),
            BufferSize::Parameter(index) | BufferSize::Dereference(index) => {
                ("CountParamIndex", Value::U16(index))
            }
        };
        let mut values = vec![(name.into(), value)];
        if matches!(self.count, BufferSize::Dereference(_)) {
            values.push(("Dereference".into(), Value::Bool(true)));
        }
        if self.bytes {
            values.push(("Bytes".into(), Value::Bool(true)));
        }
        values
    }

    pub fn from_values(values: &[(String, Value)]) -> Option<Self> {
        let mut count = None;
        let mut dereference = None;
        let mut bytes = None;
        for (name, value) in values {
            match (name.as_str(), value) {
                ("CountConst", Value::U64(value)) if count.is_none() => {
                    count = Some(BufferSize::Constant(*value));
                }
                ("CountParamIndex", Value::U16(index)) if count.is_none() => {
                    count = Some(BufferSize::Parameter(*index));
                }
                ("Dereference", Value::Bool(value)) if dereference.is_none() => {
                    dereference = Some(*value);
                }
                ("Bytes", Value::Bool(value)) if bytes.is_none() => bytes = Some(*value),
                _ => return None,
            }
        }
        let count = match (count?, dereference.unwrap_or(false)) {
            (BufferSize::Parameter(index), true) => BufferSize::Dereference(index),
            (_, true) => return None,
            (count, false) => count,
        };
        Some(Self {
            count,
            bytes: bytes.unwrap_or(false),
        })
    }
}
