use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef<'a>(pub Row<'a>, pub Vec<Type<'a>>);

impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> TypeAttributes {
        TypeAttributes(self.0.usize(0))
    }
    pub fn name(&self) -> &str {
        self.0.str(1)
    }
    pub fn namespace(&self) -> &str {
        self.0.str(2)
    }
    pub fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), self.name())
    }
    pub fn extends(&self) -> TypeDefOrRef {
        self.0.decode(3)
    }
    pub fn fields(&self) -> impl Iterator<Item = Field> {
        self.0.list(TABLE_FIELD, 4).map(Field)
    }
    pub fn methods(&self) -> impl Iterator<Item = MethodDef> {
        self.0.list(TABLE_METHODDEF, 5).map(MethodDef)
    }
    pub fn attributes(&self) -> impl Iterator<Item = CustomAttribute> {
        self.0.attributes(HasCustomAttribute::TypeDef(self.clone()))
    }
    pub fn generic_params(&self) -> impl Iterator<Item = GenericParam> {
        self.0.equal_range(TABLE_GENERICPARAM, 2, TypeOrMethodDef::TypeDef(self.clone()).encode()).map(GenericParam)
    }
    pub fn interface_impls(&self) -> impl Iterator<Item = InterfaceImpl> {
        self.0.equal_range(TABLE_INTERFACEIMPL, 0, (self.0.key.row + 1) as _).map(InterfaceImpl)
    }
}
