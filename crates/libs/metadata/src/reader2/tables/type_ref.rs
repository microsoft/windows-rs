use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeRef<'a>(pub Row<'a>);

impl<'a> TypeRef<'a> {
    pub fn name(&self) -> &str {
        self.0.str(1)
    }

    pub fn namespace(&self) -> &str {
        self.0.str(2)
    }
}

impl<'a> ToTypeName for TypeRef<'a> {
    fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), self.name())
    }
}
