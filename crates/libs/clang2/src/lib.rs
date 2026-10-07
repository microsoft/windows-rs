#![doc = include_str!("../readme.md")]

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::{Display, Formatter, Write};

mod native;
use native::*;

mod capture;
pub use capture::{DeclarationInfo, capture, discover};

mod project;
mod validate;
pub use project::{
    FunctionImport, ImportTarget, Plan, ProjectionOptions, ReferenceKind, StringKind, TypeReference,
};

/// One translation unit. All inputs to a capture use the same compiler arguments.
#[derive(Clone, Debug)]
pub struct Input {
    pub name: String,
    pub source: String,
}

impl Input {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
        }
    }
}

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// Owned native evidence. No libclang handles or projected metadata types are retained.
pub struct Snapshot {
    entities: Vec<Entity>,
    declarations: Vec<Declaration>,
    roots: Vec<Id>,
    target: String,
    pointer_size: i64,
    arguments: Vec<String>,
    diagnostics: Vec<String>,
}

/// Work performed while checking the captured evidence.
#[derive(Debug)]
pub struct Validation {
    pub declarations: usize,
    pub observations: usize,
    pub declaration_pairs: usize,
    pub type_pairs: usize,
    /// Named records or enums for which no complete definition was captured.
    pub incomplete: Vec<String>,
}

/// Checked native groups and completions, borrowing immutable captured evidence.
pub struct Resolved<'a> {
    snapshot: &'a Snapshot,
    representatives: Vec<Id>,
    groups: Vec<Vec<Id>>,
    annotations: BTreeMap<Id, BTreeMap<usize, Vec<Vec<String>>>>,
    report: Validation,
}

impl Resolved<'_> {
    pub fn report(&self) -> &Validation {
        &self.report
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    pub fn resolved_observations(&self) -> usize {
        self.representatives.len()
    }
}

impl Snapshot {
    /// Checks agreement of available definitions, not whole-language C++ ODR equivalence.
    ///
    /// An incomplete record is reported separately, never counted as a complete definition.
    /// Unsupported evidence is an error even when there is only one observation.
    pub fn validate(&self) -> Result<Validation, Error> {
        Ok(self.resolve()?.report)
    }

    pub fn resolve(&self) -> Result<Resolved<'_>, Error> {
        validate::validate(self)
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    /// Returns a diagnostic view of the native graph. This format is experimental.
    pub fn dump(&self) -> String {
        let mut result = format!("target: {}\n", self.target);
        for (index, declaration) in self.declarations.iter().enumerate() {
            writeln!(
                result,
                "[{index}] {}{} ({}:{}:{}, TU {})",
                declaration.name,
                if self.roots.contains(&Id(index)) {
                    " [root]"
                } else {
                    ""
                },
                declaration.location.file,
                declaration.location.line,
                declaration.location.column,
                declaration.unit,
            )
            .unwrap();
            writeln!(result, "USR: {}", declaration.identity).unwrap();
            writeln!(result, "entity: {}", declaration.entity.0).unwrap();
            writeln!(result, "{:#?}", declaration.data).unwrap();
        }
        result
    }
}
