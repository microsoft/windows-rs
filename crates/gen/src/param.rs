use crate::*;

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub kind: TypeKind,
    pub array: bool,
    pub input: bool,
    pub by_ref: bool,
    pub is_const: bool,
}
