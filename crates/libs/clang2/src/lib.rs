#![doc = include_str!("../readme.md")]

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;
use windows_metadata as metadata;

mod agreement;
mod capture;
mod layout;
mod output;
mod record;

pub use capture::Compiler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    X86,
    X64,
    Arm64,
}

impl Target {
    fn triple(self) -> &'static str {
        match self {
            Self::X86 => "i686-pc-windows-msvc",
            Self::X64 => "x86_64-pc-windows-msvc",
            Self::Arm64 => "aarch64-pc-windows-msvc",
        }
    }

    pub fn pointer_bits(self) -> u32 {
        if self == Self::X86 { 32 } else { 64 }
    }
}

/// One explicit source profile and required declaration scope.
pub struct Input {
    pub name: String,
    pub source: String,
    pub target: Target,
    pub arguments: Vec<String>,
    pub roots: Vec<PathBuf>,
    pub required: BTreeSet<String>,
}

impl Input {
    pub fn new(name: impl Into<String>, source: impl Into<String>, target: Target) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
            target,
            arguments: Vec::new(),
            roots: Vec::new(),
            required: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Source,
    Scope,
    Capture,
    Agreement,
    Representation,
}

#[derive(Debug)]
pub struct Error {
    pub stage: Stage,
    pub message: String,
}

impl Error {
    fn new(stage: Stage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.stage, self.message)
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub size: u64,
    pub alignment: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecordLayoutContract {
    pub packing: Option<u16>,
    pub alignment: Option<u16>,
    pub required_alignment: u32,
    pub type_visibility: Option<TypeVisibility>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeVisibility {
    Default,
    Hidden,
    Protected,
}

#[derive(Debug, Clone)]
pub struct LayoutBridgeEvidence {
    pub abi: u32,
    pub library: PathBuf,
    pub parser_sha256: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
    /// Identity within this snapshot, assigned from canonical compiler cursor equality.
    pub id: usize,
    pub qualified_name: String,
    pub kind: String,
    pub usr: String,
    pub expansion: Location,
    pub spelling: Location,
    pub written_type: String,
    pub canonical_type: String,
    pub layout: Option<Layout>,
    pub annotations: Vec<AnnotationEvidence>,
}

#[derive(Debug, Clone)]
pub struct Root {
    pub qualified_name: String,
    pub kind: String,
    pub required: bool,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroEvidence {
    pub name: String,
    pub tokens: Vec<String>,
    pub expansion: Location,
    pub spelling: Location,
    pub kind: MacroKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroKind {
    Integer {
        written_type: String,
        canonical_type: String,
        value: metadata::Value,
    },
    Alias {
        qualified_name: String,
        kind: String,
        usr: String,
        expansion: Location,
        spelling: Location,
    },
}

#[derive(Debug, Clone)]
pub struct MacroProbe {
    pub source: String,
    pub arguments: Vec<String>,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldEvidence {
    pub name: String,
    pub ordinal: usize,
    pub anonymous: bool,
    pub record_id: Option<usize>,
    pub expansion: Location,
    pub spelling: Location,
    pub written_type: String,
    pub canonical_type: String,
    pub layout: Option<Layout>,
    pub offset: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    C,
    Stdcall,
    Fastcall,
    Win64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionEvidence {
    pub linker_name: String,
    pub convention: CallingConvention,
    pub variadic: bool,
    pub dll_import: bool,
    pub written_result: String,
    pub canonical_result: String,
    pub annotations: Vec<AnnotationEvidence>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterEvidence {
    pub name: String,
    pub ordinal: usize,
    pub expansion: Location,
    pub spelling: Location,
    pub written_type: String,
    pub canonical_type: String,
    pub contract: ParameterContract,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParameterContract {
    pub direction: Option<Direction>,
    pub optional: bool,
    pub com_outptr: bool,
    pub annotations: Vec<AnnotationEvidence>,
    pub extents: Vec<BufferExtent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationEvidence {
    pub source: String,
    pub ordinal: usize,
    pub payload: String,
    pub location: Location,
    pub bindings: Vec<(String, usize)>,
    pub resolution: Option<AnnotationResolution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationResolution {
    pub payload: String,
    pub definitions: Vec<MacroDefinitionEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDefinitionEvidence {
    pub name: String,
    pub location: Location,
    pub function_like: bool,
    pub variadic: bool,
    pub parameters: Vec<String>,
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferExtent {
    pub phase: String,
    pub units: String,
    pub parameter: usize,
    pub dereference: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumMemberEvidence {
    pub name: String,
    pub value: metadata::Value,
    pub expansion: Location,
    pub spelling: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceEvidence {
    pub guid: String,
    pub vtable_size: u32,
    pub inherited: u32,
    pub no_vtable: bool,
    pub packing: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodEvidence {
    pub name: String,
    pub slot: u32,
    pub expansion: Location,
    pub spelling: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HelperEvidence {
    pub qualified_name: String,
    pub kind: String,
    pub usr: String,
    pub expansion: Location,
    pub spelling: Location,
    pub written_type: String,
    pub canonical_type: String,
}

/// Explicit named-export contracts keyed by the compiler's native linker symbol.
#[derive(Debug, Clone)]
pub struct Import {
    pub dll: String,
    pub export: String,
}

/// Owned native evidence. No compiler cursor or translation-unit handle escapes capture.
#[derive(Debug, Clone)]
pub struct Snapshot {
    target: Target,
    pub target_triple: String,
    pub compiler: String,
    pub library: PathBuf,
    pub layout_bridge: Option<LayoutBridgeEvidence>,
    pub input_name: String,
    pub source_sha256: String,
    pub arguments: Vec<String>,
    pub diagnostics: Vec<String>,
    pub roots: Vec<Root>,
    macros: Vec<MacroEvidence>,
    pub macro_probe: Option<MacroProbe>,
    declarations: Vec<Declaration>,
}

impl Snapshot {
    pub fn macros(&self) -> impl Iterator<Item = &MacroEvidence> {
        self.macros.iter()
    }

    pub fn target(&self) -> Target {
        self.target
    }

    pub fn evidence(&self) -> impl Iterator<Item = &Evidence> {
        self.declarations.iter().map(|decl| &decl.evidence)
    }

    pub fn ownerships(&self) -> impl Iterator<Item = (&Evidence, &Evidence, bool)> {
        self.declarations.iter().filter_map(|decl| {
            decl.owner.map(|owner| {
                (
                    &decl.evidence,
                    &self.declarations[owner].evidence,
                    decl.anonymous,
                )
            })
        })
    }

    pub fn fields(&self) -> impl Iterator<Item = (&Evidence, &FieldEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let fields = match &decl.kind {
                DeclarationKind::Record { fields, .. } => fields.as_slice(),
                _ => &[],
            };
            fields
                .iter()
                .map(move |field| (&decl.evidence, &field.evidence))
        })
    }

    pub fn record_layouts(&self) -> impl Iterator<Item = (&Evidence, &RecordLayoutContract)> {
        self.declarations
            .iter()
            .filter_map(|decl| match &decl.kind {
                DeclarationKind::Record {
                    contract: Some(contract),
                    ..
                } => Some((&decl.evidence, contract)),
                _ => None,
            })
    }

    pub fn functions(&self) -> impl Iterator<Item = (&Evidence, &FunctionEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let observations = match &decl.kind {
                DeclarationKind::Function(observations) => observations.as_slice(),
                _ => &[],
            };
            observations
                .iter()
                .map(move |function| (&decl.evidence, &function.evidence))
        })
    }

    pub fn interfaces(&self) -> impl Iterator<Item = (&Evidence, &InterfaceEvidence)> {
        self.declarations
            .iter()
            .filter_map(|decl| match &decl.kind {
                DeclarationKind::Interface { evidence, .. } => Some((&decl.evidence, evidence)),
                _ => None,
            })
    }

    pub fn methods(&self) -> impl Iterator<Item = (&Evidence, &MethodEvidence, &FunctionEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let methods = match &decl.kind {
                DeclarationKind::Interface { methods, .. } => methods.as_slice(),
                _ => &[],
            };
            methods
                .iter()
                .map(move |method| (&decl.evidence, &method.evidence, &method.function.evidence))
        })
    }

    pub fn interface_helpers(&self) -> impl Iterator<Item = (&Evidence, &HelperEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let helpers = match &decl.kind {
                DeclarationKind::Interface { helpers, .. } => helpers.as_slice(),
                _ => &[],
            };
            helpers.iter().map(move |helper| (&decl.evidence, helper))
        })
    }

    pub fn parameters(&self) -> impl Iterator<Item = (&Evidence, &ParameterEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let observations = match &decl.kind {
                DeclarationKind::Function(observations) => observations.as_slice(),
                _ => &[],
            };
            observations.iter().flat_map(move |function| {
                function
                    .params
                    .iter()
                    .map(move |param| (&decl.evidence, &param.evidence))
            })
        })
    }

    pub fn enum_members(&self) -> impl Iterator<Item = (&Evidence, &EnumMemberEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let members = match &decl.kind {
                DeclarationKind::Enum { members, .. } => members.as_slice(),
                _ => &[],
            };
            members.iter().map(move |member| (&decl.evidence, member))
        })
    }

    pub fn method_parameters(
        &self,
    ) -> impl Iterator<Item = (&Evidence, &MethodEvidence, &ParameterEvidence)> {
        self.declarations.iter().flat_map(|decl| {
            let methods = match &decl.kind {
                DeclarationKind::Interface { methods, .. } => methods.as_slice(),
                _ => &[],
            };
            methods.iter().flat_map(move |method| {
                method
                    .function
                    .params
                    .iter()
                    .map(move |parameter| (&decl.evidence, &method.evidence, &parameter.evidence))
            })
        })
    }

    pub fn rdl(&self, namespace: &str) -> Result<String> {
        self.rdl_with_imports(namespace, &BTreeMap::new())
    }

    pub fn rdl_with_imports(
        &self,
        namespace: &str,
        imports: &BTreeMap<String, Import>,
    ) -> Result<String> {
        output::Closed::new(self, namespace, imports)
            .and_then(|output| output.rdl())
            .map_err(|mut error| {
                error.message = format!(
                    "{} [{}]: {}",
                    self.input_name,
                    self.target.triple(),
                    error.message
                );
                error
            })
    }
}

type Id = usize;

#[derive(Debug, Clone)]
struct Declaration {
    evidence: Evidence,
    names: Vec<String>,
    owner: Option<Id>,
    anonymous: bool,
    kind: DeclarationKind,
}

#[derive(Debug, Clone, PartialEq)]
enum DeclarationKind {
    Alias {
        written: NativeType,
        canonical: NativeType,
    },
    Opaque {
        union: bool,
    },
    Record {
        union: bool,
        fields: Vec<NativeField>,
        contract: Option<RecordLayoutContract>,
    },
    Enum {
        underlying: NativeType,
        scoped: bool,
        members: Vec<EnumMemberEvidence>,
    },
    Function(Vec<NativeFunction>),
    Interface {
        evidence: InterfaceEvidence,
        base: Option<Id>,
        methods: Vec<NativeMethod>,
        helpers: Vec<HelperEvidence>,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct NativeMethod {
    evidence: MethodEvidence,
    function: NativeFunction,
}

#[derive(Debug, Clone, PartialEq)]
struct NativeFunction {
    evidence: FunctionEvidence,
    result: NativeType,
    canonical_result: NativeType,
    params: Vec<NativeParameter>,
}

#[derive(Debug, Clone, PartialEq)]
struct NativeParameter {
    evidence: ParameterEvidence,
    written: NativeType,
    canonical: NativeType,
}

#[derive(Debug, Clone, PartialEq)]
struct NativeField {
    evidence: FieldEvidence,
    written: NativeType,
    canonical: NativeType,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Qualifiers {
    constant: bool,
    volatile: bool,
    restrict: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NativeType {
    kind: NativeKind,
    qualifiers: Qualifiers,
    layout: Option<Layout>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NativeKind {
    Void,
    Scalar(i32),
    Alias(Id),
    Record(Id),
    Enum(Id),
    Pointer(Box<NativeType>),
    Reference(Box<NativeType>),
    Array(Box<NativeType>, usize),
}

impl Snapshot {
    fn expand(&self, ty: &NativeType, visiting: &mut BTreeSet<Id>) -> Result<NativeType> {
        let mut result = ty.clone();
        result.kind = match &ty.kind {
            NativeKind::Alias(id) => {
                if !visiting.insert(*id) {
                    return Err(Error::new(Stage::Agreement, "cyclic native typedef"));
                }
                let DeclarationKind::Alias { written, .. } = &self.declarations[*id].kind else {
                    return Err(Error::new(
                        Stage::Agreement,
                        "typedef edge targets a non-alias",
                    ));
                };
                let mut expanded = self.expand(written, visiting)?;
                visiting.remove(id);
                expanded.qualify(&ty.qualifiers);
                return Ok(expanded);
            }
            NativeKind::Pointer(ty) => NativeKind::Pointer(Box::new(self.expand(ty, visiting)?)),
            NativeKind::Reference(ty) => {
                NativeKind::Reference(Box::new(self.expand(ty, visiting)?))
            }
            NativeKind::Array(ty, count) => {
                NativeKind::Array(Box::new(self.expand(ty, visiting)?), *count)
            }
            kind => kind.clone(),
        };
        Ok(result)
    }

    fn check(&self) -> Result<()> {
        for decl in &self.declarations {
            match &decl.kind {
                DeclarationKind::Alias { written, canonical } => {
                    self.check_type(&decl.evidence.qualified_name, written, canonical)?;
                }
                DeclarationKind::Record { fields, .. } => {
                    for field in fields {
                        self.check_type(
                            &format!("{}.{}", decl.evidence.qualified_name, field.evidence.name),
                            &field.written,
                            &field.canonical,
                        )?;
                    }
                }
                DeclarationKind::Enum { underlying, .. } => {
                    if !matches!(underlying.kind, NativeKind::Scalar(_))
                        || decl.evidence.layout != underlying.layout
                    {
                        return Err(Error::new(
                            Stage::Agreement,
                            "enum storage disagrees with its native underlying type",
                        ));
                    }
                }
                DeclarationKind::Function(observations) => {
                    for function in observations {
                        self.check_function(&decl.evidence.qualified_name, function)?;
                    }
                }
                DeclarationKind::Interface { methods, .. } => {
                    for method in methods {
                        self.check_function(
                            &format!("{}::{}", decl.evidence.qualified_name, method.evidence.name),
                            &method.function,
                        )?;
                    }
                }
                DeclarationKind::Opaque { .. } => {}
            }
        }
        Ok(())
    }

    fn check_type(&self, name: &str, written: &NativeType, canonical: &NativeType) -> Result<()> {
        let mut expanded = self.expand(written, &mut BTreeSet::new())?;
        let mut canonical = canonical.clone();
        expanded.normalize_arrays();
        canonical.normalize_arrays();
        if expanded != canonical {
            return Err(Error::new(
                Stage::Agreement,
                format!(
                    "{name}: written type graph disagrees with compiler canonical type: \
                 expanded {expanded:?}, canonical {canonical:?}"
                ),
            ));
        }
        Ok(())
    }

    fn check_function(&self, name: &str, function: &NativeFunction) -> Result<()> {
        self.check_type(name, &function.result, &function.canonical_result)?;
        for param in &function.params {
            self.check_type(
                &format!("{name}[{}]", param.evidence.ordinal),
                &param.written,
                &param.canonical,
            )?;
        }
        Ok(())
    }
}

impl NativeType {
    fn normalize_arrays(&mut self) {
        match &mut self.kind {
            NativeKind::Array(element, _) => {
                element.qualify(&self.qualifiers);
                self.qualifiers = Qualifiers::default();
                element.normalize_arrays();
            }
            NativeKind::Pointer(element) | NativeKind::Reference(element) => {
                element.normalize_arrays();
            }
            _ => {}
        }
    }

    fn qualify(&mut self, qualifiers: &Qualifiers) {
        self.qualifiers.constant |= qualifiers.constant;
        self.qualifiers.volatile |= qualifiers.volatile;
        self.qualifiers.restrict |= qualifiers.restrict;
    }
}
