mod associations;
mod attributes;
mod layouts;
mod members;
mod methods;

use crate::reader::{self, AsRow, HasAttributes, RowId};
use std::collections::HashMap;

/// A metadata validation failure associated with one row and, when applicable, a related row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    category: ValidationCategory,
    message: String,
    row: RowId,
    related: Option<RowId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationCategory {
    Duplicate,
    Invalid,
}

impl ValidationError {
    pub fn category(&self) -> ValidationCategory {
        self.category
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn row(&self) -> RowId {
        self.row
    }

    pub fn related(&self) -> Option<RowId> {
        self.related
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for ValidationError {}

/// Configures validation for authored metadata and its external definitions.
pub struct Validator<'a> {
    index: &'a reader::Index,
    references: Option<&'a reader::Index>,
}

impl<'a> Validator<'a> {
    pub fn new(index: &'a reader::Index) -> Self {
        Self {
            index,
            references: None,
        }
    }

    pub fn references(mut self, references: &'a reader::Index) -> Self {
        self.references = Some(references);
        self
    }

    pub fn validate(self) -> Vec<ValidationError> {
        Context::new(self.index, self.references).validate()
    }
}

/// Validates metadata identities and associations exposed by [`reader::Index`].
pub fn validate(index: &reader::Index) -> Vec<ValidationError> {
    Validator::new(index).validate()
}

struct Context<'a> {
    index: &'a reader::Index,
    references: Option<&'a reader::Index>,
    errors: Vec<ValidationError>,
}

impl<'a> Context<'a> {
    fn new(index: &'a reader::Index, references: Option<&'a reader::Index>) -> Self {
        Self {
            index,
            references,
            errors: vec![],
        }
    }

    fn validate(mut self) -> Vec<ValidationError> {
        attributes::validate(&mut self);
        associations::validate_maps(&mut self);
        layouts::validate(&mut self);

        let mut types: Vec<_> = self.index.types().collect();
        types.sort_by(|a, b| {
            (a.namespace(), a.name(), a.row_id()).cmp(&(b.namespace(), b.name(), b.row_id()))
        });

        let mut names = HashMap::<(&str, &str), Vec<reader::TypeDef<'_>>>::new();
        for ty in types {
            let previous = names
                .entry((ty.namespace(), ty.name()))
                .or_default()
                .iter()
                .find(|previous| arches_overlap(previous.arches(), ty.arches()));
            if let Some(previous) = previous {
                self.duplicate(
                    ty.row_id(),
                    previous.row_id(),
                    format!("duplicate type `{}.{}`", ty.namespace(), ty.name()),
                );
            }
            names
                .entry((ty.namespace(), ty.name()))
                .or_default()
                .push(ty);

            self.validate_type(ty);
            for nested in self.index.nested_recursive(ty) {
                self.validate_type(nested);
            }
        }

        self.errors
    }

    fn validate_type(&mut self, ty: reader::TypeDef<'a>) {
        members::validate(self, ty);
        associations::validate_type(self, ty);
        methods::validate(self, ty);
    }

    fn invalid(&mut self, row: RowId, related: Option<RowId>, message: String) {
        self.errors.push(ValidationError {
            category: ValidationCategory::Invalid,
            message,
            row,
            related,
        });
    }

    fn duplicate(&mut self, row: RowId, related: RowId, message: String) {
        self.duplicate_optional(row, Some(related), message);
    }

    fn duplicate_optional(&mut self, row: RowId, related: Option<RowId>, message: String) {
        self.errors.push(ValidationError {
            category: ValidationCategory::Duplicate,
            message,
            row,
            related,
        });
    }
}

fn invalid_signature_type(ty: &crate::Type) -> bool {
    match ty {
        crate::Type::Void => true,
        crate::Type::Array(element)
        | crate::Type::ArrayFixed(element, _)
        | crate::Type::RefMut(element)
        | crate::Type::RefConst(element) => invalid_signature_type(element),
        crate::Type::PtrMut(_, _) | crate::Type::PtrConst(_, _) => false,
        _ => false,
    }
}

fn type_name(ty: &crate::Type) -> String {
    match ty {
        crate::Type::ClassName(name) | crate::Type::ValueName(name) => {
            format!("{}.{}", name.namespace, name.name)
        }
        crate::Type::Array(element) => format!("{}[]", type_name(element)),
        _ => format!("{ty:?}"),
    }
}

fn generics(ty: reader::TypeDef) -> Vec<crate::Type> {
    ty.generic_params()
        .map(|param| crate::Type::Generic(param.name().to_string(), param.sequence()))
        .collect()
}

fn same_identity(left: &crate::Signature, right: &crate::Signature) -> bool {
    left.flags == right.flags && left.types == right.types
}

fn arches_overlap(left: i32, right: i32) -> bool {
    left == 0 || right == 0 || left & right != 0
}
