use super::*;

pub trait ConstantExt {
    fn constant_type(&self) -> Type;
}

impl ConstantExt for Constant {
    fn constant_type(&self) -> Type {
        Type::from_metadata_type(&self.ty(), None, &[])
    }
}
