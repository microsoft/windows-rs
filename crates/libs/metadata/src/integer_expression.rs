#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IntegerLiteral {
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
}

impl IntegerLiteral {
    fn value(self) -> i128 {
        match self {
            Self::I32(value) => value.into(),
            Self::U32(value) => value.into(),
            Self::I64(value) => value.into(),
            Self::U64(value) => value.into(),
        }
    }

    fn arithmetic(self, right: Self, operation: Operation) -> Option<Self> {
        use IntegerLiteral::*;
        let left_value = self.value();
        let right_value = right.value();
        Some(match (self, right) {
            (U64(_), _) | (_, U64(_)) => {
                let (left, right) = (left_value as u64, right_value as u64);
                U64(match operation {
                    Operation::Add => left.wrapping_add(right),
                    Operation::Subtract => left.wrapping_sub(right),
                    Operation::Multiply => left.wrapping_mul(right),
                })
            }
            (I64(_), _) | (_, I64(_)) => {
                let (left, right) = (left_value as i64, right_value as i64);
                I64(match operation {
                    Operation::Add => left.checked_add(right)?,
                    Operation::Subtract => left.checked_sub(right)?,
                    Operation::Multiply => left.checked_mul(right)?,
                })
            }
            (U32(_), _) | (_, U32(_)) => {
                let (left, right) = (left_value as u32, right_value as u32);
                U32(match operation {
                    Operation::Add => left.wrapping_add(right),
                    Operation::Subtract => left.wrapping_sub(right),
                    Operation::Multiply => left.wrapping_mul(right),
                })
            }
            (I32(left), I32(right)) => I32(match operation {
                Operation::Add => left.checked_add(right)?,
                Operation::Subtract => left.checked_sub(right)?,
                Operation::Multiply => left.checked_mul(right)?,
            }),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IntegerReference {
    Parameter(u16),
    Dereference(u16),
    /// Null storage makes the expression unavailable, not zero.
    OptionalDereference(u16),
    Field(String),
    Return,
}

/// Integer arithmetic with C promotions and usual arithmetic conversions.
/// References obtain their types from the owning signature or record. Unsigned arithmetic wraps
/// at the resulting type's width; signed overflow is undefined, not a mathematical integer result.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IntegerExpression {
    Literal(IntegerLiteral),
    Reference(IntegerReference),
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
}

impl IntegerExpression {
    pub fn is_valid(&self) -> bool {
        self.constant_value().is_ok()
    }

    /// Returns `None` for a runtime expression and an error for invalid or undefined expressions.
    pub fn constant_value(&self) -> Result<Option<IntegerLiteral>, &'static str> {
        fn check(value: &IntegerExpression, depth: usize, remaining: &mut usize) -> bool {
            if depth > 32 || *remaining == 0 {
                return false;
            }
            *remaining -= 1;
            match value {
                IntegerExpression::Literal(_) => true,
                IntegerExpression::Reference(IntegerReference::Field(name)) => !name.is_empty(),
                IntegerExpression::Reference(_) => true,
                IntegerExpression::Add(left, right)
                | IntegerExpression::Subtract(left, right)
                | IntegerExpression::Multiply(left, right) => {
                    check(left, depth + 1, remaining) && check(right, depth + 1, remaining)
                }
            }
        }
        fn evaluate(value: &IntegerExpression) -> Result<Option<IntegerLiteral>, &'static str> {
            use IntegerExpression::*;
            match value {
                Literal(value) => Ok(Some(*value)),
                Reference(_) => Ok(None),
                Add(left, right) | Subtract(left, right) | Multiply(left, right) => {
                    let operation = match value {
                        Add(..) => Operation::Add,
                        Subtract(..) => Operation::Subtract,
                        Multiply(..) => Operation::Multiply,
                        _ => unreachable!(),
                    };
                    let left = evaluate(left)?;
                    let right = evaluate(right)?;
                    match (left, right) {
                        (Some(left), Some(right)) => left
                            .arithmetic(right, operation)
                            .map(Some)
                            .ok_or("signed overflow in integer expression"),
                        _ => Ok(None),
                    }
                }
            }
        }
        if !check(self, 0, &mut 128) {
            return Err("invalid integer expression");
        }
        evaluate(self)
    }

    pub fn references(&self) -> Option<Vec<&IntegerReference>> {
        fn visit<'a>(value: &'a IntegerExpression, result: &mut Vec<&'a IntegerReference>) {
            match value {
                IntegerExpression::Literal(_) => {}
                IntegerExpression::Reference(reference) => result.push(reference),
                IntegerExpression::Add(left, right)
                | IntegerExpression::Subtract(left, right)
                | IntegerExpression::Multiply(left, right) => {
                    visit(left, result);
                    visit(right, result);
                }
            }
        }
        if !self.is_valid() {
            return None;
        }
        let mut result = Vec::new();
        visit(self, &mut result);
        Some(result)
    }

    /// A versioned, typed postfix encoding, not source-language expression text.
    pub fn encode(&self) -> Option<String> {
        fn write(value: &IntegerExpression, result: &mut String) {
            use IntegerExpression::*;
            use std::fmt::Write;
            match value {
                Literal(value) => match value {
                    IntegerLiteral::I32(value) => write!(result, " i32:{value}").unwrap(),
                    IntegerLiteral::U32(value) => write!(result, " u32:{value}").unwrap(),
                    IntegerLiteral::I64(value) => write!(result, " i64:{value}").unwrap(),
                    IntegerLiteral::U64(value) => write!(result, " u64:{value}").unwrap(),
                },
                Reference(reference) => match reference {
                    IntegerReference::Parameter(index) => write!(result, " p:{index}").unwrap(),
                    IntegerReference::Dereference(index) => write!(result, " d:{index}").unwrap(),
                    IntegerReference::OptionalDereference(index) => {
                        write!(result, " o:{index}").unwrap();
                    }
                    IntegerReference::Return => result.push_str(" return"),
                    IntegerReference::Field(name) => {
                        result.push_str(" f:");
                        for byte in name.as_bytes() {
                            write!(result, "{byte:02x}").unwrap();
                        }
                    }
                },
                Add(left, right) | Subtract(left, right) | Multiply(left, right) => {
                    write(left, result);
                    write(right, result);
                    result.push_str(match value {
                        Add(..) => " add",
                        Subtract(..) => " sub",
                        Multiply(..) => " mul",
                        _ => unreachable!(),
                    });
                }
            }
        }
        if !self.is_valid() {
            return None;
        }
        let mut result = "v1".to_string();
        write(self, &mut result);
        Some(result)
    }

    /// Unknown versions, operators, types and noncanonical payloads retain their full spelling.
    pub fn decode(text: &str) -> Option<Self> {
        use IntegerLiteral::*;
        let mut tokens = text.split_whitespace();
        if tokens.next()? != "v1" {
            return None;
        }
        let mut stack = Vec::new();
        for (count, token) in tokens.enumerate() {
            if count >= 128 {
                return None;
            }
            let value = match token {
                "add" | "sub" | "mul" => {
                    let right = Box::new(stack.pop()?);
                    let left = Box::new(stack.pop()?);
                    match token {
                        "add" => Self::Add(left, right),
                        "sub" => Self::Subtract(left, right),
                        "mul" => Self::Multiply(left, right),
                        _ => unreachable!(),
                    }
                }
                "return" => Self::Reference(IntegerReference::Return),
                _ => {
                    let (kind, value) = token.split_once(':')?;
                    match kind {
                        "i32" => Self::Literal(I32(value.parse().ok()?)),
                        "u32" => Self::Literal(U32(value.parse().ok()?)),
                        "i64" => Self::Literal(I64(value.parse().ok()?)),
                        "u64" => Self::Literal(U64(value.parse().ok()?)),
                        "p" => Self::Reference(IntegerReference::Parameter(value.parse().ok()?)),
                        "d" => Self::Reference(IntegerReference::Dereference(value.parse().ok()?)),
                        "o" => Self::Reference(IntegerReference::OptionalDereference(
                            value.parse().ok()?,
                        )),
                        "f" => {
                            if !value.is_ascii() || !value.len().is_multiple_of(2) {
                                return None;
                            }
                            let bytes: Option<Vec<_>> = (0..value.len())
                                .step_by(2)
                                .map(|index| u8::from_str_radix(&value[index..index + 2], 16).ok())
                                .collect();
                            Self::Reference(IntegerReference::Field(
                                String::from_utf8(bytes?).ok()?,
                            ))
                        }
                        _ => return None,
                    }
                }
            };
            stack.push(value);
        }
        let value = stack.pop()?;
        if !stack.is_empty() || value.encode()?.as_str() != text {
            return None;
        }
        Some(value)
    }
}
