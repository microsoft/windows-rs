use super::*;

pub trait ConstantExt {
    fn constant_type(&self) -> Type;
    fn constant_value(&self) -> Value;
}

impl ConstantExt for Constant {
    fn constant_type(&self) -> Type {
        Type::from_metadata_type(&self.ty(), None, &[])
    }

    fn constant_value(&self) -> Value {
        self.value()
    }
}
