use super::*;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: metadata::Type,
}

/// `Param` is the same layout as `Field`; it exists as a semantic alias for
/// function / method parameter positions.
pub type Param = Field;
