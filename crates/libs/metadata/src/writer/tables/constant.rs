use super::*;

#[derive(Default)]
pub struct Constant {
    pub value: Value,
    pub(crate) parent_index: HasConstant,
}
