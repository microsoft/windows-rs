use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub def: MethodParam,
    pub ty: Type,
}

impl Param {
    pub fn is_convertible(&self) -> bool {
        !self.def.flags().contains(ParamAttributes::Out)
            && self.ty.is_convertible()
    }
}