use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub def: MethodParam,
    pub ty: Type,
}
