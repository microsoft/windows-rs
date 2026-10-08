#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) struct Id(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) struct EntityId(pub usize);

#[derive(Debug)]
pub(super) struct Entity {
    pub observations: Vec<Id>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Location {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

#[derive(Debug)]
pub(super) struct Declaration {
    pub entity: EntityId,
    pub name: String,
    pub identity: String,
    pub candidate: String,
    pub unit: String,
    pub location: Location,
    pub owner: String,
    pub annotations: Vec<Annotation>,
    pub data: DeclarationData,
}

#[derive(Debug)]
pub(super) enum DeclarationData {
    Pending,
    Record {
        kind: String,
        unnamed: bool,
        complete: bool,
        layout: Option<Layout>,
        fields: Vec<Field>,
        bases: Vec<Type>,
        methods: Vec<Method>,
        /// A `__declspec(uuid(...))` attribute, lowercase and without surrounding braces.
        guid: Option<String>,
        unavailable: Vec<String>,
    },
    Alias {
        target: Type,
        canonical: Type,
        parameters: Vec<Parameter>,
    },
    Enum {
        complete: bool,
        scoped: bool,
        flags: bool,
        repr: Type,
        variants: Vec<(String, Value)>,
        annotations: Vec<Vec<Annotation>>,
    },
    Function {
        inline: bool,
        ty: Type,
        canonical: Type,
        parameters: Vec<Parameter>,
        link_name: String,
    },
    Variable {
        ty: Type,
        canonical: Type,
        value: Value,
    },
    Unavailable(String),
}

impl DeclarationData {
    pub fn complete(&self) -> bool {
        match self {
            Self::Record { complete, .. } | Self::Enum { complete, .. } => *complete,
            _ => true,
        }
    }

    pub fn types(&self) -> Vec<&Type> {
        match self {
            Self::Record {
                fields,
                bases,
                methods,
                ..
            } => fields
                .iter()
                .map(|field| &field.ty)
                .chain(bases)
                .chain(
                    methods
                        .iter()
                        .flat_map(|method| [&method.ty, &method.canonical]),
                )
                .collect(),
            Self::Enum { repr: ty, .. } => vec![ty],
            Self::Variable { ty, canonical, .. } => vec![ty, canonical],
            Self::Alias {
                target, canonical, ..
            } => vec![target, canonical],
            Self::Function { ty, canonical, .. } => vec![ty, canonical],
            Self::Pending | Self::Unavailable(_) => vec![],
        }
    }

    pub fn evidence_rank(&self) -> (u8, bool) {
        let rank = match self {
            Self::Variable {
                value: Value::None, ..
            } => 0,
            _ => u8::from(self.complete()) + 1,
        };
        (rank, matches!(self, Self::Record { guid: Some(_), .. }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Layout {
    pub size: i64,
    pub align: i64,
}

#[derive(Debug)]
pub(super) struct Field {
    pub name: String,
    pub ty: Type,
    pub offset: i64,
    pub bit_width: Option<i32>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub(super) struct Method {
    pub name: String,
    pub property: Option<String>,
    pub ty: Type,
    pub canonical: Type,
    pub parameters: Vec<Parameter>,
    pub virtual_method: bool,
    pub static_method: bool,
    pub const_method: bool,
    pub ref_qualifier: i32,
    pub pure: bool,
    pub overrides: Vec<String>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Parameter {
    pub name: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum AnnotationSource {
    Sal,
    Midl,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Annotation {
    pub source: AnnotationSource,
    pub text: String,
    pub context: std::sync::Arc<[String]>,
    pub location: Location,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct SourceAnnotations {
    pub sal: Vec<String>,
    pub midl: Vec<String>,
}

impl SourceAnnotations {
    pub fn lowered(&self) -> Vec<String> {
        let mut result = self.sal.clone();
        if !result
            .iter()
            .any(|text| Annotation::direction(text).is_some())
        {
            let direction = self
                .midl
                .iter()
                .filter_map(|text| Annotation::midl_direction(text))
                .fold(0, |a, b| a | b);
            match direction {
                1 => result.push("_In_".into()),
                2 => result.push("_Out_".into()),
                3 => result.push("_Inout_".into()),
                _ => {}
            }
        }
        result
    }
}

#[derive(Default)]
pub(super) struct ResolvedAnnotations {
    pub own: SourceAnnotations,
    pub fields: Vec<SourceAnnotations>,
    pub methods: Vec<SourceAnnotations>,
    pub parameters: std::collections::BTreeMap<usize, Vec<SourceAnnotations>>,
}

impl Annotation {
    pub fn direction(text: &str) -> Option<u8> {
        if text.starts_with("_Inout_") {
            Some(3)
        } else if text.starts_with("_In_") {
            Some(1)
        } else if text.starts_with("_Out_")
            || text.starts_with("_Outptr_")
            || text.starts_with("_COM_Outptr_")
        {
            Some(2)
        } else {
            None
        }
    }

    pub(super) fn midl_direction(text: &str) -> Option<u8> {
        let mut direction = 0;
        for attribute in Self::midl_attributes(text)? {
            direction |= match attribute {
                "in" => 1,
                "out" => 2,
                _ => 0,
            };
        }
        (direction != 0).then_some(direction)
    }

    pub(super) fn midl_attributes(text: &str) -> Option<Vec<&str>> {
        let mut text = text.trim();
        if let Some(comment) = text.strip_prefix("/*") {
            text = comment.strip_suffix("*/")?.trim();
        }
        let mut attributes = vec![];
        while !text.is_empty() {
            let group = text.strip_prefix('[')?;
            let mut delimiters = vec![];
            let mut start = 0;
            let mut index = 0;
            loop {
                if let Some(length) = literal_length(&group[index..]) {
                    index += length;
                    continue;
                }
                let ch = group[index..].chars().next()?;
                match ch {
                    '(' | '[' | '{' => delimiters.push(ch),
                    ']' if delimiters.is_empty() => {
                        attributes.push(group[start..index].trim());
                        text = group[index + 1..].trim();
                        break;
                    }
                    ')' | ']' | '}' => {
                        let open = delimiters.pop()?;
                        if !matches!((open, ch), ('(', ')') | ('[', ']') | ('{', '}')) {
                            return None;
                        }
                    }
                    ',' if delimiters.is_empty() => {
                        attributes.push(group[start..index].trim());
                        start = index + 1;
                    }
                    _ => {}
                }
                index += ch.len_utf8();
            }
        }
        if attributes.is_empty()
            || attributes.iter().any(|attribute| {
                !attribute
                    .chars()
                    .next()
                    .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
            })
        {
            return None;
        }
        Some(attributes)
    }
    pub fn bound_text(&self) -> String {
        let mut result = String::new();
        let mut depth = 0usize;
        let mut start = 0;
        while start < self.text.len() {
            if let Some(length) = literal_length(&self.text[start..]) {
                result.push_str(&self.text[start..start + length]);
                start += length;
                continue;
            }
            let character = self.text[start..].chars().next().unwrap();
            if character.is_ascii_digit()
                || (character == '.'
                    && self
                        .text
                        .as_bytes()
                        .get(start + 1)
                        .is_some_and(u8::is_ascii_digit))
            {
                let mut end = start + 1;
                while let Some(&ch) = self.text.as_bytes().get(end) {
                    if ch.is_ascii_alphanumeric()
                        || matches!(ch, b'_' | b'.' | b'\'')
                        || (matches!(ch, b'+' | b'-')
                            && matches!(self.text.as_bytes()[end - 1], b'e' | b'E' | b'p' | b'P'))
                    {
                        end += 1;
                    } else {
                        break;
                    }
                }
                result.push_str(&self.text[start..end]);
                start = end;
            } else if character.is_ascii_alphabetic() || character == '_' {
                let end = start
                    + self.text[start..]
                        .bytes()
                        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == b'_')
                        .count();
                let word = &self.text[start..end];
                let before = self.text[..start].trim_end();
                let after = self.text[end..].trim_start();
                if depth != 0
                    && !before.ends_with('.')
                    && !before.ends_with("::")
                    && !before.ends_with("->")
                    && !after.starts_with("::")
                    && let Some(index) = self.context.iter().position(|name| name == word)
                {
                    result.push_str(&format!("${index}"));
                } else {
                    result.push_str(word);
                }
                start = end;
            } else {
                if character == '(' {
                    depth += 1;
                } else if character == ')' {
                    depth = depth.saturating_sub(1);
                }
                result.push(character);
                start += character.len_utf8();
            }
        }
        result
    }
}

fn literal_length(text: &str) -> Option<usize> {
    for prefix in ["u8R\"", "uR\"", "UR\"", "LR\"", "R\""] {
        if let Some(rest) = text.strip_prefix(prefix)
            && let Some((delimiter, _)) = rest.split_once('(')
            && delimiter.len() <= 16
            && !delimiter
                .chars()
                .any(|ch| ch.is_whitespace() || matches!(ch, ')' | '\\'))
        {
            let suffix = format!("){delimiter}\"");
            let body = prefix.len() + delimiter.len() + 1;
            return Some(
                text[body..]
                    .find(&suffix)
                    .map_or(text.len(), |end| body + end + suffix.len()),
            );
        }
    }
    let prefix = [
        "u8\"", "u\"", "U\"", "L\"", "\"", "u8'", "u'", "U'", "L'", "'",
    ]
    .into_iter()
    .find(|prefix| text.starts_with(prefix))?;
    let quote = prefix.as_bytes()[prefix.len() - 1];
    let mut escaped = false;
    for (offset, ch) in text[prefix.len()..].bytes().enumerate() {
        if escaped {
            escaped = false;
        } else if ch == b'\\' {
            escaped = true;
        } else if ch == quote {
            return Some(prefix.len() + offset + 1);
        }
    }
    Some(text.len())
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Value {
    None,
    Integer(u64),
    Float(u64),
    String {
        encoding: StringEncoding,
        units: Vec<u32>,
    },
    Aggregate(Vec<Self>),
    Unavailable(String),
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum StringEncoding {
    Narrow,
    Utf8,
    Utf16,
    Utf32,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct Qualifiers {
    pub constant: bool,
    pub volatile: bool,
    pub restrict: bool,
}

#[derive(Debug)]
pub(super) struct Type {
    pub qualifiers: Qualifiers,
    pub kind: TypeKind,
}

#[derive(Debug)]
pub(super) enum TypeKind {
    Builtin {
        kind: String,
        layout: Option<Layout>,
    },
    Named(Id),
    Pointer(Box<Type>),
    LValueReference(Box<Type>),
    RValueReference(Box<Type>),
    Array {
        length: Option<u64>,
        element: Box<Type>,
    },
    Function {
        prototype: bool,
        convention: i32,
        exception_specification: i32,
        variadic: bool,
        result: Box<Type>,
        parameters: Vec<Type>,
    },
    Unavailable(String),
}

impl Type {
    pub fn children(&self) -> Vec<&Self> {
        match &self.kind {
            TypeKind::Pointer(target)
            | TypeKind::LValueReference(target)
            | TypeKind::RValueReference(target)
            | TypeKind::Array {
                element: target, ..
            } => vec![target],
            TypeKind::Function {
                result, parameters, ..
            } => std::iter::once(result.as_ref()).chain(parameters).collect(),
            _ => vec![],
        }
    }
}
