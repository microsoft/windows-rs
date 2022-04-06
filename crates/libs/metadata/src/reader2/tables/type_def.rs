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
    pub fn extends(&self) -> TypeDefOrRef {
        self.0.decode(3)
    }
    pub fn fields(&self) -> impl Iterator<Item = Field> {
        self.0.list(TABLE_FIELD, 4).map(Field)
    }
    pub fn methods(&self) -> impl Iterator<Item = MethodDef> {
        self.0.list(TABLE_METHODDEF, 5).map(MethodDef)
    }

    pub fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), self.name())
    }
}
