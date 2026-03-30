use super::*;

pub trait ConstantExt {
    fn constant_type(&self, reader: &Reader) -> Type;
}

impl ConstantExt for Constant {
    fn constant_type(&self, reader: &Reader) -> Type {
        Type::from_metadata_type(&self.ty(), None, &[], reader)
    }
}
