use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerBound {
    pub expression: IntegerExpression,
    pub exclusive: bool,
}

/// Each bound compares the subject using C integer promotions and usual arithmetic conversions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerRange {
    pub phase: BufferPhase,
    pub indirect: bool,
    pub lower: Option<IntegerBound>,
    pub upper: Option<IntegerBound>,
}

impl IntegerRange {
    pub fn is_valid(&self) -> bool {
        (self.lower.is_some() || self.upper.is_some())
            && (!self.indirect || self.phase != BufferPhase::Value)
            && self.lower.iter().chain(&self.upper).all(|bound| {
                bound.expression.references().is_some_and(|references| {
                    references.into_iter().all(|reference| match reference {
                        IntegerReference::Field(_) => self.phase == BufferPhase::Value,
                        IntegerReference::Return => self.phase == BufferPhase::Success,
                        _ => self.phase != BufferPhase::Value,
                    })
                })
            })
    }

    pub fn to_values(&self) -> Vec<(String, Value)> {
        let mut values = vec![(
            String::new(),
            Value::EnumValue(
                TypeName::named("Windows.Win32.Metadata", "BufferPhase"),
                Box::new(Value::I32(self.phase as i32)),
            ),
        )];
        if self.indirect {
            values.push(("Indirect".into(), Value::Bool(true)));
        }
        for (name, bound) in [("Lower", &self.lower), ("Upper", &self.upper)] {
            if let Some(bound) = bound {
                values.push((name.into(), Value::Utf8(bound.expression.encode().unwrap())));
                if bound.exclusive {
                    values.push((format!("{name}Exclusive"), Value::Bool(true)));
                }
            }
        }
        values
    }

    pub fn from_values(values: &[(String, Value)]) -> Option<Self> {
        let [(name, Value::EnumValue(ty, phase)), properties @ ..] = values else {
            return None;
        };
        if !name.is_empty() || ty != ("Windows.Win32.Metadata", "BufferPhase") {
            return None;
        }
        let phase = match phase.as_ref() {
            Value::I32(0) => BufferPhase::Entry,
            Value::I32(1) => BufferPhase::Success,
            Value::I32(2) => BufferPhase::Value,
            _ => return None,
        };
        let mut result = Self {
            phase,
            indirect: false,
            lower: None,
            upper: None,
        };
        let (mut lower_exclusive, mut upper_exclusive) = (false, false);
        let mut seen = HashSet::new();
        for (name, value) in properties {
            if !seen.insert(name) {
                return None;
            }
            match (name.as_str(), value) {
                ("Indirect", Value::Bool(true)) => result.indirect = true,
                ("Lower" | "Upper", Value::Utf8(value)) => {
                    let bound = Some(IntegerBound {
                        expression: IntegerExpression::decode(value)?,
                        exclusive: false,
                    });
                    if name == "Lower" {
                        result.lower = bound;
                    } else {
                        result.upper = bound;
                    }
                }
                ("LowerExclusive", Value::Bool(true)) => lower_exclusive = true,
                ("UpperExclusive", Value::Bool(true)) => upper_exclusive = true,
                _ => return None,
            }
        }
        if lower_exclusive {
            result.lower.as_mut()?.exclusive = true;
        }
        if upper_exclusive {
            result.upper.as_mut()?.exclusive = true;
        }
        result.is_valid().then_some(result)
    }
}
