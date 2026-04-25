use super::*;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: metadata::Type,
}

/// A function or method parameter with an optional SAL annotation.
///
/// Unlike [`Field`], which represents a struct field and carries no direction
/// metadata, `Param` also records the [`ParamAnnotation`] extracted from any
/// SAL attribute on the parameter declaration.
#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    /// SAL-derived direction/optional flags.  All fields are `false` when no
    /// SAL annotation was detected for this parameter.
    pub annotation: ParamAnnotation,
}
