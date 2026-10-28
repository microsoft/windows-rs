use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum BufferAccess {
    Read = 0,
    Write = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum BufferPhase {
    Entry = 0,
    Success = 1,
    Value = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum BufferUnits {
    Elements = 0,
    Bytes = 1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BufferCount {
    Unknown,
    Constant(u64),
    Parameter(u16),
    Dereference(u16),
    Field(String),
    Return,
    Expression(IntegerExpression),
}

impl BufferCount {
    /// The single dereferenced count guarded by a null fallback.
    pub fn null_guard(&self) -> Option<u16> {
        match self {
            Self::Dereference(index) => Some(*index),
            Self::Expression(value) => match value.references()?.as_slice() {
                [IntegerReference::Dereference(index)] => Some(*index),
                _ => None,
            },
            _ => None,
        }
    }
}

/// An accessible extent, not an allocation size or an ownership transfer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferExtent {
    pub access: BufferAccess,
    pub phase: BufferPhase,
    pub units: BufferUnits,
    pub count: BufferCount,
    pub count_on_entry: bool,
    /// A null count pointer leaves the extent unknown, without weakening subject guarantees.
    pub count_optional: bool,
    /// Legacy count adjustment. New arithmetic uses `BufferCount::Expression`.
    pub offset: u64,
    pub null_fallback: Option<u16>,
    /// Selects initialized pointer storage at the contract phase, including returned storage.
    pub indirect: bool,
    pub nullable: bool,
}

impl BufferExtent {
    pub fn new(
        access: BufferAccess,
        phase: BufferPhase,
        units: BufferUnits,
        count: BufferCount,
    ) -> Self {
        Self {
            access,
            phase,
            units,
            count,
            count_on_entry: false,
            count_optional: false,
            offset: 0,
            null_fallback: None,
            indirect: false,
            nullable: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        (!self.count_on_entry
            || self.phase == BufferPhase::Success
                && matches!(
                    self.count,
                    BufferCount::Parameter(_)
                        | BufferCount::Dereference(_)
                        | BufferCount::Expression(_)
                ))
            && (self.null_fallback.is_none() || self.count.null_guard().is_some())
            && (!self.count_optional
                || matches!(self.count, BufferCount::Dereference(_))
                    && self.null_fallback.is_none())
            && (!self.indirect || self.phase != BufferPhase::Value)
            && (!self.nullable || self.indirect || self.phase == BufferPhase::Value)
            && match &self.count {
                BufferCount::Unknown => self.offset == 0,
                BufferCount::Field(name) => {
                    self.phase == BufferPhase::Value && !name.is_empty() && !self.indirect
                }
                BufferCount::Return => self.phase == BufferPhase::Success,
                BufferCount::Parameter(_) | BufferCount::Dereference(_) => {
                    self.phase != BufferPhase::Value
                }
                BufferCount::Constant(value) => value.checked_add(self.offset).is_some(),
                BufferCount::Expression(value) => {
                    self.offset == 0
                        && value.constant_value().is_ok_and(|value| {
                            !matches!(value,
                        Some(IntegerLiteral::I32(value)) if value < 0)
                                && !matches!(value, Some(IntegerLiteral::I64(value)) if value < 0)
                        })
                        && value.references().is_some_and(|references| {
                            (!self.count_on_entry || !references.is_empty())
                                && references.into_iter().all(|reference| match reference {
                                    IntegerReference::Field(_) => self.phase == BufferPhase::Value,
                                    IntegerReference::Return => {
                                        self.phase == BufferPhase::Success && !self.count_on_entry
                                    }
                                    _ => self.phase != BufferPhase::Value,
                                })
                        })
                }
            }
    }

    pub fn to_values(&self) -> Vec<(String, Value)> {
        let mut result = vec![
            (
                String::new(),
                buffer_enum("BufferAccess", self.access as i32),
            ),
            (String::new(), buffer_enum("BufferPhase", self.phase as i32)),
            (String::new(), buffer_enum("BufferUnits", self.units as i32)),
        ];
        let (name, value) = match &self.count {
            BufferCount::Unknown => ("CountUnknown", Value::Bool(true)),
            BufferCount::Constant(value) => ("CountConst", Value::U64(*value)),
            BufferCount::Parameter(index) | BufferCount::Dereference(index) => {
                ("CountParamIndex", Value::U16(*index))
            }
            BufferCount::Field(name) => ("CountFieldName", Value::Utf8(name.clone())),
            BufferCount::Return => ("CountReturn", Value::Bool(true)),
            BufferCount::Expression(value) => {
                ("CountExpression", Value::Utf8(value.encode().unwrap()))
            }
        };
        result.push((name.into(), value));
        if matches!(self.count, BufferCount::Dereference(_)) {
            result.push(("Dereference".into(), Value::Bool(true)));
        }
        if self.count_on_entry {
            result.push(("CountOnEntry".into(), Value::Bool(true)));
        }
        if self.count_optional {
            result.push(("CountOptional".into(), Value::Bool(true)));
        }
        if self.offset != 0 {
            result.push(("Offset".into(), Value::U64(self.offset)));
        }
        if let Some(index) = self.null_fallback {
            result.push(("NullFallbackParamIndex".into(), Value::U16(index)));
        }
        if self.indirect {
            result.push(("Indirect".into(), Value::Bool(true)));
        }
        if self.nullable {
            result.push(("Nullable".into(), Value::Bool(true)));
        }
        result
    }

    /// Unknown values or properties retain their full metadata spelling.
    pub fn from_values(values: &[(String, Value)]) -> Option<Self> {
        let [(a, access), (b, phase), (c, units), properties @ ..] = values else {
            return None;
        };
        if !a.is_empty() || !b.is_empty() || !c.is_empty() {
            return None;
        }
        let access = match buffer_enum_value(access, "BufferAccess")? {
            0 => BufferAccess::Read,
            1 => BufferAccess::Write,
            _ => return None,
        };
        let phase = match buffer_enum_value(phase, "BufferPhase")? {
            0 => BufferPhase::Entry,
            1 => BufferPhase::Success,
            2 => BufferPhase::Value,
            _ => return None,
        };
        let units = match buffer_enum_value(units, "BufferUnits")? {
            0 => BufferUnits::Elements,
            1 => BufferUnits::Bytes,
            _ => return None,
        };
        let mut count = None;
        let mut result = Self::new(access, phase, units, BufferCount::Constant(0));
        let mut dereference = false;
        let mut seen = HashSet::new();
        for (name, value) in properties {
            if !seen.insert(name) {
                return None;
            }
            let source = match (name.as_str(), value) {
                ("CountUnknown", Value::Bool(true)) => Some(BufferCount::Unknown),
                ("CountConst", Value::U64(value)) => Some(BufferCount::Constant(*value)),
                ("CountParamIndex", Value::U16(value)) => Some(BufferCount::Parameter(*value)),
                ("CountFieldName", Value::Utf8(value)) => Some(BufferCount::Field(value.clone())),
                ("CountReturn", Value::Bool(true)) => Some(BufferCount::Return),
                ("CountExpression", Value::Utf8(value)) => {
                    Some(BufferCount::Expression(IntegerExpression::decode(value)?))
                }
                ("Dereference", Value::Bool(true)) => {
                    dereference = true;
                    None
                }
                ("CountOnEntry", Value::Bool(true)) => {
                    result.count_on_entry = true;
                    None
                }
                ("CountOptional", Value::Bool(true)) => {
                    result.count_optional = true;
                    None
                }
                ("Offset", Value::U64(value)) if *value != 0 => {
                    result.offset = *value;
                    None
                }
                ("NullFallbackParamIndex", Value::U16(value)) => {
                    result.null_fallback = Some(*value);
                    None
                }
                ("Indirect", Value::Bool(true)) => {
                    result.indirect = true;
                    None
                }
                ("Nullable", Value::Bool(true)) => {
                    result.nullable = true;
                    None
                }
                _ => return None,
            };
            if let Some(source) = source
                && count.replace(source).is_some()
            {
                return None;
            }
        }
        result.count = count?;
        if dereference {
            let BufferCount::Parameter(index) = result.count else {
                return None;
            };
            result.count = BufferCount::Dereference(index);
        }
        result.is_valid().then_some(result)
    }
}

fn buffer_enum(name: &str, value: i32) -> Value {
    Value::EnumValue(
        TypeName::named("Windows.Win32.Metadata", name),
        Box::new(Value::I32(value)),
    )
}

fn buffer_enum_value(value: &Value, name: &str) -> Option<i32> {
    let Value::EnumValue(ty, value) = value else {
        return None;
    };
    if ty != ("Windows.Win32.Metadata", name) {
        return None;
    }
    let Value::I32(value) = value.as_ref() else {
        return None;
    };
    Some(*value)
}
