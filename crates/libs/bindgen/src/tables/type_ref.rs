use super::*;

pub trait TypeRefExt {
    fn type_name(&self) -> TypeName;
}

impl TypeRefExt for TypeRef {
    fn type_name(&self) -> TypeName {
        // Safety: TypeRef<'static> references data in a 'static TypeIndex.
        let ns: &'static str = unsafe { std::mem::transmute(self.namespace()) };
        let name: &'static str = unsafe { std::mem::transmute(self.name()) };
        TypeName(ns, trim_tick(name))
    }
}
