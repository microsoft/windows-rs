use super::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Expression {
    Constant(i32),
    Parameter(usize),
    Dereference(usize),
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
}

impl Expression {
    pub fn references(&self) -> Vec<(usize, bool)> {
        fn visit(value: &Expression, result: &mut Vec<(usize, bool)>) {
            match value {
                Expression::Constant(_) => {}
                Expression::Parameter(index) => result.push((*index, false)),
                Expression::Dereference(index) => result.push((*index, true)),
                Expression::Add(left, right)
                | Expression::Subtract(left, right)
                | Expression::Multiply(left, right) => {
                    visit(left, result);
                    visit(right, result);
                }
            }
        }
        let mut result = Vec::new();
        visit(self, &mut result);
        result
    }

    pub fn parse(text: &str, names: &[&str]) -> Option<Self> {
        Self::parse_inner(text, names, 0, &mut 128)
    }

    fn parse_inner(
        text: &str,
        names: &[&str],
        depth: usize,
        remaining: &mut usize,
    ) -> Option<Self> {
        if depth > 32 || *remaining == 0 {
            return None;
        }
        *remaining -= 1;
        let text = count_group(text);
        if text.is_empty() {
            return None;
        }
        let mut nesting = 0i32;
        let mut operator = None;
        for (index, byte) in text.bytes().enumerate() {
            match byte {
                b'(' => nesting += 1,
                b')' => nesting -= 1,
                b'+' | b'-' | b'*' if nesting == 0 => {
                    if byte != b'*' && text.as_bytes().get(index + 1) == Some(&byte) {
                        return None;
                    }
                    if text[..index]
                        .trim_end()
                        .ends_with(|ch: char| ch.is_ascii_alphanumeric() || matches!(ch, ')' | '_'))
                        && (byte != b'*' || operator.is_none_or(|(_, previous)| previous == b'*'))
                    {
                        operator = Some((index, byte));
                    }
                }
                _ => {}
            }
            if nesting < 0 {
                return None;
            }
        }
        if nesting != 0 {
            return None;
        }
        if let Some((index, op)) = operator {
            let left = Self::parse_inner(&text[..index], names, depth + 1, remaining)?;
            let right = Self::parse_inner(&text[index + 1..], names, depth + 1, remaining)?;
            return Self::arithmetic(left, right, op);
        }
        if let Some(value) = text.strip_prefix('-') {
            return Self::arithmetic(
                Self::Constant(0),
                Self::parse_inner(value, names, depth + 1, remaining)?,
                b'-',
            );
        }
        if let Some(value) = text.strip_prefix('+') {
            return Self::arithmetic(
                Self::Constant(0),
                Self::parse_inner(value, names, depth + 1, remaining)?,
                b'+',
            );
        }
        if let Some(value) = count_literal(text) {
            return Some(Self::Constant(value.try_into().ok()?));
        }
        let (name, dereference) = if let Some(name) = text.strip_prefix('*') {
            (count_group(name), true)
        } else {
            (text, false)
        };
        let mut matches = names
            .iter()
            .enumerate()
            .filter(|(_, candidate)| !candidate.is_empty() && **candidate == name);
        let (index, _) = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        Some(if dereference {
            Self::Dereference(index)
        } else {
            Self::Parameter(index)
        })
    }

    fn arithmetic(left: Self, right: Self, operation: u8) -> Option<Self> {
        if let (Self::Constant(left), Self::Constant(right)) = (&left, &right) {
            return Some(Self::Constant(match operation {
                b'+' => left.checked_add(*right)?,
                b'-' => left.checked_sub(*right)?,
                b'*' => left.checked_mul(*right)?,
                _ => unreachable!(),
            }));
        }
        Some(match operation {
            b'+' => Self::Add(Box::new(left), Box::new(right)),
            b'-' => Self::Subtract(Box::new(left), Box::new(right)),
            b'*' => Self::Multiply(Box::new(left), Box::new(right)),
            _ => unreachable!(),
        })
    }
}
