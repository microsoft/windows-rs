use super::*;

pub trait TypeRefExt {
    fn type_name(&self) -> TypeName;
}

impl TypeRefExt for TypeRef {
    fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), trim_tick(self.name()))
    }
}
