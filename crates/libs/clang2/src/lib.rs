#![doc = include_str!("../readme.md")]

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;
use windows_metadata as metadata;

mod capture;
mod output;

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

#[derive(Debug, Clone)]
pub struct Evidence {
    pub qualified_name: String,
    pub kind: String,
    pub usr: String,
    pub expansion: Location,
    pub spelling: Location,
    pub written_type: String,
    pub canonical_type: String,
    pub layout: Option<Layout>,
}

#[derive(Debug, Clone)]
pub struct Root {
    pub qualified_name: String,
    pub kind: String,
    pub required: bool,
    pub location: Location,
}

/// Owned native evidence. No compiler cursor or translation-unit handle escapes capture.
#[derive(Debug, Clone)]
pub struct Snapshot {
    target: Target,
    pub target_triple: String,
    pub compiler: String,
    pub library: PathBuf,
    pub input_name: String,
    pub arguments: Vec<String>,
    pub diagnostics: Vec<String>,
    pub roots: Vec<Root>,
    declarations: Vec<Declaration>,
}

impl Snapshot {
    pub fn target(&self) -> Target {
        self.target
    }

    pub fn evidence(&self) -> impl Iterator<Item = &Evidence> {
        self.declarations.iter().map(|decl| &decl.evidence)
    }

    pub fn rdl(&self, namespace: &str) -> Result<String> {
        output::Closed::new(self, namespace)
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
    kind: DeclarationKind,
}

#[derive(Debug, Clone)]
enum DeclarationKind {
    Alias {
        written: NativeType,
        canonical: NativeType,
    },
    Opaque {
        union: bool,
    },
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
            if let DeclarationKind::Alias { written, canonical } = &decl.kind {
                let mut expanded = self.expand(written, &mut BTreeSet::new())?;
                let mut canonical = canonical.clone();
                expanded.normalize_arrays();
                canonical.normalize_arrays();
                if expanded != canonical {
                    return Err(Error::new(
                        Stage::Agreement,
                        format!(
                            "{}: written typedef graph disagrees with compiler canonical type: \
                             expanded {expanded:?}, canonical {canonical:?}",
                            decl.evidence.qualified_name
                        ),
                    ));
                }
            }
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
