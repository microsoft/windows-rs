#![doc = include_str!("../readme.md")]

mod capture;
mod merge;
mod native;
mod partial;
#[cfg(feature = "projection")]
pub mod projection;
mod resolve;
mod runtime;
pub use merge::{Merged, Variants, merge};
pub use resolve::{
    AliasGroup, AliasObservationRef, EnumGroup, EnumObservationRef, EnumRef, Group, ObservationRef,
    RecordRef, Resolution, Work, resolve,
};
pub use runtime::{PartialCapture, capture, capture_partial};

use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target {
    X86,
    X64,
    Arm64,
}

impl Target {
    pub fn triple(self) -> &'static str {
        match self {
            Self::X86 => "i686-pc-windows-msvc",
            Self::X64 => "x86_64-pc-windows-msvc",
            Self::Arm64 => "aarch64-pc-windows-msvc",
        }
    }
}

#[derive(Default)]
pub struct Input {
    pub name: String,
    pub source: String,
    /// Additional compiler arguments, such as include paths and macro definitions.
    pub arguments: Vec<String>,
    /// Global record, enum, or alias roots and their dependencies. Empty selects all declarations.
    pub roots: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeId(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecordId(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AliasId(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnumId(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeclarationId {
    Record(RecordId),
    Alias(AliasId),
    Enum(EnumId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObservationId(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FileId(pub usize);

/// A source occurrence, using the outer expansion site for macro-generated declarations.
#[derive(Debug, Eq, PartialEq)]
pub struct Location {
    pub file: FileId,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Layout {
    pub size: u64,
    pub alignment: u64,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Qualifiers {
    pub constant: bool,
    pub volatile: bool,
    pub restrict: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scalar {
    Bool,
    Char { signed: bool },
    SignedChar,
    UnsignedChar,
    WChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
}

impl Scalar {
    pub(crate) fn signed_integer(&self) -> Option<bool> {
        match self {
            Self::Char { signed } => Some(*signed),
            Self::SignedChar | Self::Short | Self::Int | Self::Long | Self::LongLong => Some(true),
            Self::UnsignedChar
            | Self::WChar
            | Self::UnsignedShort
            | Self::UnsignedInt
            | Self::UnsignedLong
            | Self::UnsignedLongLong => Some(false),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TypeKind {
    Void,
    Scalar(Scalar),
    Pointer(TypeId),
    Array {
        element: TypeId,
        length: u64,
    },
    Record(RecordId),
    Alias(AliasId),
    Enum(EnumId),
    Function(Box<Function>),
    Attributed {
        convention: CallingConvention,
        modified: TypeId,
        equivalent: TypeId,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CallingConvention {
    C,
    Stdcall,
}

/// Accepted prototypes have no variadic, exception, or supplementary function properties.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Function {
    pub convention: CallingConvention,
    pub result: TypeId,
    pub parameters: Vec<TypeId>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub qualifiers: Qualifiers,
    /// Observed byte size and alignment; void, functions, and incomplete records have no layout.
    pub layout: Option<Layout>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub location: Location,
    pub written: TypeId,
    pub canonical: TypeId,
    pub offset_bits: u64,
    pub attributes: Vec<Attribute>,
}

/// Native attribute evidence; unresolved kinds remain unsuitable for reconciliation.
#[derive(Debug, Eq, PartialEq)]
pub enum AttributeKind {
    Packed,
    DefaultTypeVisibility,
    MaxFieldAlignment { bits: u32 },
    Aligned { bits: u32, kind: AlignmentKind },
    Unresolved(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AlignmentKind {
    Gnu,
    Standard,
    Declspec,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Attribute {
    pub kind: AttributeKind,
    pub spelling: String,
    pub tokens: Vec<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Alias {
    pub name: String,
    pub usr: String,
    pub observations: Vec<AliasObservation>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AliasObservation {
    pub location: Location,
    pub underlying: TypeId,
    pub canonical: TypeId,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Record {
    pub name: String,
    pub usr: String,
    pub kind: RecordKind,
    pub observations: Vec<ObservationId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    Struct,
    Union,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RecordObservation {
    pub record: RecordId,
    pub location: Location,
    pub definition: Option<RecordDefinition>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RecordDefinition {
    pub layout: Layout,
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Enum {
    pub name: String,
    pub usr: String,
    pub observations: Vec<EnumObservation>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnumObservation {
    pub location: Location,
    pub underlying: TypeId,
    pub canonical: TypeId,
    pub layout: Layout,
    pub scoped: bool,
    pub attributes: Vec<Attribute>,
    /// None denotes a forward declaration, not an empty enum definition.
    pub members: Option<Vec<EnumMember>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnumMember {
    pub name: String,
    pub location: Location,
    pub value: EnumValue,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EnumValue {
    Signed(i64),
    Unsigned(u64),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub input: String,
    pub target: Target,
    pub compiler: String,
    pub arguments: Vec<String>,
    pub diagnostics: Vec<String>,
    /// Selected declarations; other entities were reached through dependencies.
    pub roots: Vec<DeclarationId>,
    pub files: Vec<String>,
    pub records: Vec<Record>,
    pub aliases: Vec<Alias>,
    pub enums: Vec<Enum>,
    pub observations: Vec<RecordObservation>,
    pub types: Vec<Type>,
}

#[derive(Debug)]
pub struct Statistics {
    pub records: usize,
    pub aliases: usize,
    pub enums: usize,
    pub observations: usize,
    pub types: usize,
    pub fields: usize,
    /// Retained Rust capacities, excluding allocator overhead and native compiler memory.
    pub owned_bytes: usize,
}

impl Snapshot {
    pub fn statistics(&self) -> Statistics {
        fn vector_bytes<T>(values: &Vec<T>) -> usize {
            values.capacity() * size_of::<T>()
        }
        fn strings_bytes(values: &Vec<String>) -> usize {
            vector_bytes(values) + values.iter().map(String::capacity).sum::<usize>()
        }
        fn attributes_bytes(values: &Vec<Attribute>) -> usize {
            vector_bytes(values)
                + values
                    .iter()
                    .map(|value| {
                        (match &value.kind {
                            AttributeKind::Packed
                            | AttributeKind::DefaultTypeVisibility
                            | AttributeKind::MaxFieldAlignment { .. }
                            | AttributeKind::Aligned { .. } => 0,
                            AttributeKind::Unresolved(kind) => kind.capacity(),
                        }) + value.spelling.capacity()
                            + strings_bytes(&value.tokens)
                    })
                    .sum::<usize>()
        }
        let mut fields = 0;
        let mut owned_bytes = size_of::<Self>()
            + self.input.capacity()
            + self.compiler.capacity()
            + strings_bytes(&self.arguments)
            + strings_bytes(&self.diagnostics)
            + vector_bytes(&self.roots)
            + strings_bytes(&self.files)
            + vector_bytes(&self.records)
            + vector_bytes(&self.aliases)
            + vector_bytes(&self.enums)
            + vector_bytes(&self.observations)
            + vector_bytes(&self.types);
        for ty in &self.types {
            if let TypeKind::Function(function) = &ty.kind {
                owned_bytes += size_of::<Function>() + vector_bytes(&function.parameters);
            }
        }
        for record in &self.records {
            owned_bytes +=
                record.name.capacity() + record.usr.capacity() + vector_bytes(&record.observations);
        }
        for alias in &self.aliases {
            owned_bytes += alias.name.capacity()
                + alias.usr.capacity()
                + vector_bytes(&alias.observations)
                + alias
                    .observations
                    .iter()
                    .map(|value| attributes_bytes(&value.attributes))
                    .sum::<usize>();
        }
        for enumeration in &self.enums {
            owned_bytes += enumeration.name.capacity()
                + enumeration.usr.capacity()
                + vector_bytes(&enumeration.observations);
            for observation in &enumeration.observations {
                owned_bytes += attributes_bytes(&observation.attributes);
                if let Some(members) = &observation.members {
                    owned_bytes += vector_bytes(members)
                        + members
                            .iter()
                            .map(|member| {
                                member.name.capacity() + attributes_bytes(&member.attributes)
                            })
                            .sum::<usize>();
                }
            }
        }
        for observation in &self.observations {
            owned_bytes += attributes_bytes(&observation.attributes);
            if let Some(definition) = &observation.definition {
                fields += definition.fields.len();
                owned_bytes += vector_bytes(&definition.fields)
                    + definition
                        .fields
                        .iter()
                        .map(|field| field.name.capacity() + attributes_bytes(&field.attributes))
                        .sum::<usize>();
            }
        }
        Statistics {
            records: self.records.len(),
            aliases: self.aliases.len(),
            enums: self.enums.len(),
            observations: self.observations.len()
                + self
                    .aliases
                    .iter()
                    .map(|alias| alias.observations.len())
                    .sum::<usize>()
                + self
                    .enums
                    .iter()
                    .map(|value| value.observations.len())
                    .sum::<usize>(),
            types: self.types.len(),
            fields,
            owned_bytes,
        }
    }
}

#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Error {}
