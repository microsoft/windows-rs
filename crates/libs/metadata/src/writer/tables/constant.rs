use super::*;

#[derive(Default)]
pub struct Constant {
    pub ty: Type,
    pub value: Value,

    pub(crate) parent_index: HasConstant,
}
