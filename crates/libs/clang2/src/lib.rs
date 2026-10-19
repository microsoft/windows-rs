#![doc = include_str!("../readme.md")]

mod capture;
mod merge;
mod native;
mod partial;
#[cfg(feature = "projection")]
pub mod projection;
mod resolve;
mod runtime;
mod sal;
pub use merge::{Merged, Variants, merge};
pub use resolve::{
    AliasGroup, AliasObservationRef, EnumGroup, EnumObservationRef, EnumRef, Group, ObservationRef,
    RecordRef, Resolution, ValueGroup, ValueObservationRef, Work, resolve,
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
    /// Global declaration roots and their dependencies. Empty selects all declarations.
    pub roots: Vec<String>,
    /// Object-like macros to evaluate as constant expressions after this input's declarations.
    pub macros: Vec<String>,
    /// Header filenames in which to discover object-like macros at the end of this input.
    /// Non-expression candidates are errors in strict capture and omissions in partial capture.
    pub macro_headers: Vec<String>,
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
pub struct ValueId(pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeclarationId {
    Record(RecordId),
    Alias(AliasId),
    Enum(EnumId),
    Value(ValueId),
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
    pub unaligned: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scalar {
    Bool,
    Char { signed: bool },
    SignedChar,
    UnsignedChar,
    WChar,
    Char16,
    Char32,
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
            | Self::Char16
            | Self::Char32
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
    Pointer64(TypeId),
    LValueReference(TypeId),
    Array {
        element: TypeId,
        length: Option<u64>,
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

/// Decoded native function prototypes; unknown function properties remain unsupported.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Function {
    pub convention: CallingConvention,
    pub exceptions: ExceptionSpecification,
    pub noreturn: bool,
    pub variadic: bool,
    pub method_qualifiers: Qualifiers,
    pub result: TypeId,
    pub parameters: Vec<TypeId>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExceptionSpecification {
    None,
    Noexcept,
    NoThrow,
    DynamicNone,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub qualifiers: Qualifiers,
    /// Object layout; void, functions, references, and incomplete types have no layout.
    pub layout: Option<Layout>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub location: Location,
    pub written: TypeId,
    pub canonical: TypeId,
    pub offset_bits: u64,
    /// Evaluated bit width, including zero-width allocation barriers.
    pub bit_width: Option<u32>,
    pub attributes: Vec<Attribute>,
}

/// Native attribute evidence; unresolved kinds remain unsuitable for reconciliation.
#[derive(Debug, Eq, PartialEq)]
pub enum AttributeKind {
    Packed,
    DefaultTypeVisibility,
    /// Compiler auto-initialization opt-out, not a layout or caller initialization contract.
    NoTrivialAutoVarInit,
    /// CodeView heap-allocation profiling, not an ownership or optimizer guarantee.
    MSAllocator,
    Uuid(u128),
    NoVTable,
    DllImport,
    DllExport,
    NoThrow,
    Cxx11NoReturn,
    SelectAny,
    Sal(Box<SalAnnotation>),
    Deprecated(Box<Deprecation>),
    MaxFieldAlignment {
        bits: u32,
    },
    Aligned {
        bits: u32,
        kind: AlignmentKind,
    },
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
    pub location: Option<Location>,
}

/// SAL source-hook evidence, including macro substitutions at the declaration site.
/// Expressions are retained, not evaluated or interpreted as native type syntax.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SalAnnotation {
    pub version: String,
    pub name: String,
    /// Arguments supplied to the hook, possibly already substituted by outer SAL macros.
    pub arguments: String,
    /// Arguments after further expansion, including nested annotation transport.
    pub expanded: String,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Deprecation {
    pub message: String,
    pub replacement: String,
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
    /// Compiler-provided declaration rather than an authored public typedef.
    pub implicit: bool,
    /// The direct typedef type specifier defines the record or enum, rather than referencing it.
    pub defines_tag: bool,
    pub attributes: Vec<Attribute>,
    /// Parameters of a prototype written in this declaration, not part of ABI type identity.
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Record {
    pub name: String,
    pub usr: String,
    pub owner: Option<RecordOwner>,
    pub kind: RecordKind,
    pub observations: Vec<ObservationId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecordOwner {
    pub record: RecordId,
    /// First field whose type contains this unnamed record; None for named declarations.
    pub field: Option<usize>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum RecordMember {
    Named(String),
    Anonymous(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    Struct,
    Class,
    Union,
}

impl RecordKind {
    pub(crate) fn compatible(self, other: Self) -> bool {
        // C++ permits class and struct tags to redeclare the same non-union record.
        (self == Self::Union) == (other == Self::Union)
    }
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
    /// Direct public, non-virtual bases, in declaration order.
    pub bases: Vec<Base>,
    /// Direct public virtual methods, in vtable order.
    pub methods: Vec<Method>,
    /// Members without vtable slots. Retained as provenance, not ABI comparisons.
    pub non_vtable_members: Vec<NonVTableMember>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NonVTableMember {
    pub name: String,
    pub usr: String,
    pub location: Location,
    pub kind: NonVTableMemberKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Base {
    pub record: RecordId,
    pub offset_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonVTableMemberKind {
    Template,
    Method,
    Using,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Method {
    pub name: String,
    pub slot: u64,
    pub overrides: bool,
    pub pure: bool,
    pub observation: ValueObservation,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub written: TypeId,
    pub location: Location,
    pub attributes: Vec<Attribute>,
    pub default: Option<DefaultArgument>,
    pub comments: Vec<ParameterComment>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParameterComment {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DefaultArgument {
    /// Compiler-printed expression, not evaluated or treated as an ABI parameter.
    pub expression: String,
    pub location: Location,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValueDeclaration {
    pub name: String,
    pub usr: String,
    pub observations: Vec<ValueObservation>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValueObservation {
    pub location: Location,
    pub written: TypeId,
    pub canonical: TypeId,
    pub attributes: Vec<Attribute>,
    pub kind: ValueKind,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionAlias {
    pub name: String,
    pub usr: String,
    pub location: Location,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValueKind {
    Function {
        alias: Option<Box<FunctionAlias>>,
        parameters: Vec<Parameter>,
        mangling: String,
        language: LanguageLinkage,
        convention_origin: CallingConventionOrigin,
        stdcall_candidate: Option<Box<FunctionCandidate>>,
    },
    Constant {
        /// None only for a native declaration, never for a failed initializer.
        value: Option<ConstantValue>,
        origin: ConstantOrigin,
        linkage: ConstantLinkage,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallingConventionOrigin {
    Unknown,
    Default,
    Explicit,
}

/// Compiler-derived alternative to an implicit convention, not a native declaration.
#[derive(Debug, Eq, PartialEq)]
pub struct FunctionCandidate {
    pub written: TypeId,
    pub canonical: TypeId,
    pub mangling: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConstantLinkage {
    Internal,
    External {
        language: LanguageLinkage,
        mangling: String,
    },
}

/// Scalar constant values; floating-point payloads retain their IEEE bits, including signed zero.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstantValue {
    Integer(EnumValue),
    /// Numeric address bits sign-extended from the target pointer width; no relocation.
    Pointer(i64),
    Boolean(bool),
    Float32(u32),
    Float64(u64),
    Guid(u128),
    PropertyKey {
        format: u128,
        id: u32,
    },
    String {
        value: String,
        wide: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LanguageLinkage {
    C,
    Cxx,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConstantOrigin {
    Declaration,
    /// The observation location is the actual macro definition referenced by the probe expansion.
    Macro,
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
    pub anonymous: bool,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnumValue {
    Signed(i64),
    Unsigned(u64),
}

#[derive(Debug, Eq, PartialEq)]
pub struct InlineFunction {
    pub name: String,
    pub usr: String,
    pub location: Location,
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
    pub values: Vec<ValueDeclaration>,
    /// C++ inline overload observations beside selected C declarations, outside the exported ABI.
    pub inline_functions: Vec<InlineFunction>,
    pub observations: Vec<RecordObservation>,
    pub types: Vec<Type>,
}

#[derive(Debug)]
pub struct Statistics {
    pub records: usize,
    pub aliases: usize,
    pub enums: usize,
    pub values: usize,
    pub observations: usize,
    pub types: usize,
    pub fields: usize,
    /// Retained Rust capacities, excluding allocator overhead and native compiler memory.
    pub owned_bytes: usize,
}

impl Snapshot {
    pub(crate) fn record_identity(&self, id: RecordId) -> (&str, &str, Vec<RecordMember>) {
        let (root, path) = self.record_path(id);
        let root = &self.records[root.0];
        (&root.name, &root.usr, path)
    }

    pub(crate) fn record_label(&self, id: RecordId) -> String {
        let (root, _, path) = self.record_identity(id);
        let mut name = root.to_string();
        for member in path {
            name.push_str("::");
            match member {
                RecordMember::Named(member) => name.push_str(&member),
                RecordMember::Anonymous(field) => name.push_str(&format!("<field {field}>")),
            }
        }
        name
    }

    pub(crate) fn record_path(&self, mut id: RecordId) -> (RecordId, Vec<RecordMember>) {
        let mut path = Vec::new();
        while let Some(owner) = self.records[id.0].owner {
            path.push(match owner.field {
                Some(field) => RecordMember::Anonymous(field),
                None => RecordMember::Named(self.records[id.0].name.clone()),
            });
            id = owner.record;
        }
        path.reverse();
        (id, path)
    }

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
                    .map(|value| match &value.kind {
                        AttributeKind::Packed
                        | AttributeKind::DefaultTypeVisibility
                        | AttributeKind::NoTrivialAutoVarInit
                        | AttributeKind::MSAllocator
                        | AttributeKind::Uuid(_)
                        | AttributeKind::NoVTable
                        | AttributeKind::DllImport
                        | AttributeKind::DllExport
                        | AttributeKind::NoThrow
                        | AttributeKind::Cxx11NoReturn
                        | AttributeKind::SelectAny
                        | AttributeKind::MaxFieldAlignment { .. }
                        | AttributeKind::Aligned { .. } => 0,
                        AttributeKind::Unresolved(kind) => kind.capacity(),
                        AttributeKind::Sal(value) => {
                            size_of::<SalAnnotation>()
                                + value.version.capacity()
                                + value.name.capacity()
                                + value.arguments.capacity()
                                + value.expanded.capacity()
                        }
                        AttributeKind::Deprecated(value) => {
                            size_of::<Deprecation>()
                                + value.message.capacity()
                                + value.replacement.capacity()
                        }
                    })
                    .sum::<usize>()
        }
        fn parameters_bytes(parameters: &Vec<Parameter>) -> usize {
            vector_bytes(parameters)
                + parameters
                    .iter()
                    .map(|parameter| {
                        parameter.name.capacity()
                            + attributes_bytes(&parameter.attributes)
                            + vector_bytes(&parameter.comments)
                            + parameter
                                .comments
                                .iter()
                                .map(|value| value.text.capacity())
                                .sum::<usize>()
                            + parameter
                                .default
                                .as_ref()
                                .map_or(0, |value| value.expression.capacity())
                    })
                    .sum::<usize>()
        }
        fn value_bytes(value: &ValueObservation) -> usize {
            attributes_bytes(&value.attributes)
                + match &value.kind {
                    ValueKind::Function {
                        alias,
                        parameters,
                        mangling,
                        stdcall_candidate,
                        ..
                    } => {
                        mangling.capacity()
                            + alias.as_ref().map_or(0, |alias| {
                                size_of::<FunctionAlias>()
                                    + alias.name.capacity()
                                    + alias.usr.capacity()
                            })
                            + parameters_bytes(parameters)
                            + stdcall_candidate.as_ref().map_or(0, |candidate| {
                                size_of::<FunctionCandidate>() + candidate.mangling.capacity()
                            })
                    }
                    ValueKind::Constant { value, linkage, .. } => {
                        let text = match value {
                            Some(ConstantValue::String { value, .. }) => value.capacity(),
                            _ => 0,
                        };
                        text + match linkage {
                            ConstantLinkage::Internal => 0,
                            ConstantLinkage::External { mangling, .. } => mangling.capacity(),
                        }
                    }
                }
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
            + vector_bytes(&self.values)
            + vector_bytes(&self.inline_functions)
            + self
                .inline_functions
                .iter()
                .map(|function| function.name.capacity() + function.usr.capacity())
                .sum::<usize>()
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
                    .map(|value| {
                        attributes_bytes(&value.attributes)
                            + value.parameters.as_ref().map_or(0, parameters_bytes)
                    })
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
        for value in &self.values {
            owned_bytes += value.name.capacity()
                + value.usr.capacity()
                + vector_bytes(&value.observations)
                + value.observations.iter().map(value_bytes).sum::<usize>();
        }
        for observation in &self.observations {
            owned_bytes += attributes_bytes(&observation.attributes);
            if let Some(definition) = &observation.definition {
                fields += definition.fields.len();
                owned_bytes += vector_bytes(&definition.fields)
                    + vector_bytes(&definition.bases)
                    + vector_bytes(&definition.methods)
                    + vector_bytes(&definition.non_vtable_members)
                    + definition
                        .non_vtable_members
                        .iter()
                        .map(|member| member.name.capacity() + member.usr.capacity())
                        .sum::<usize>()
                    + definition
                        .methods
                        .iter()
                        .map(|method| method.name.capacity() + value_bytes(&method.observation))
                        .sum::<usize>()
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
            values: self.values.len(),
            observations: self.observations.len()
                + self
                    .values
                    .iter()
                    .map(|value| value.observations.len())
                    .sum::<usize>()
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
