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
    pub data: DeclarationData,
}

#[derive(Debug)]
pub(super) enum DeclarationData {
    Pending,
    Record {
        kind: String,
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
        repr: Type,
        variants: Vec<(String, Value)>,
    },
    Function {
        ty: Type,
        canonical: Type,
        parameters: Vec<Parameter>,
        link_name: String,
    },
    Variable {
        ty: Type,
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
            Self::Enum { repr: ty, .. } | Self::Variable { ty, .. } => vec![ty],
            Self::Alias {
                target, canonical, ..
            } => vec![target, canonical],
            Self::Function { ty, canonical, .. } => vec![ty, canonical],
            Self::Pending | Self::Unavailable(_) => vec![],
        }
    }

    pub fn evidence_rank(&self) -> u8 {
        match self {
            Self::Variable {
                value: Value::None, ..
            } => 0,
            _ => u8::from(self.complete()) + 1,
        }
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
}

#[derive(Debug)]
pub(super) struct Method {
    pub name: String,
    pub ty: Type,
    pub canonical: Type,
    pub parameters: Vec<Parameter>,
    pub virtual_method: bool,
    pub static_method: bool,
    pub const_method: bool,
    pub ref_qualifier: i32,
    pub pure: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Parameter {
    pub name: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Annotation {
    pub text: String,
    pub context: Vec<String>,
    pub location: Location,
}

impl Annotation {
    pub fn bound_text(&self) -> String {
        let mut result = String::new();
        let mut word = String::new();
        let mut arguments = false;
        let flush = |word: &mut String, result: &mut String, arguments: bool| {
            if arguments && let Some(index) = self.context.iter().position(|name| name == word) {
                result.push_str(&format!("${index}"));
            } else {
                result.push_str(word);
            }
            word.clear();
        };
        for character in self.text.chars() {
            if character.is_ascii_alphanumeric() || character == '_' {
                word.push(character);
            } else {
                flush(&mut word, &mut result, arguments);
                arguments |= character == '(';
                result.push(character);
            }
        }
        flush(&mut word, &mut result, arguments);
        result
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Value {
    None,
    Integer(u64),
    Float(u64),
    Unavailable(String),
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
