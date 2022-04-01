use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef<'a>(pub Row<'a>, pub Vec<Type<'a>>);

impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> TypeAttributes {
        todo!()
    }

    pub fn name(&self) -> &str {
        self.0.str(1)
    }

    pub fn namespace(&self) -> &str {
        self.0.str(2)
    }

    // pub fn extends(&self) -> TypeDefOrRef {
    //     todo!()
    // }

    // pub fn field_list(&self) -> impl Iterator<Item = Field> {
    //     todo!()
    // }

    // pub fn method_list(&self) -> impl Iterator<Item = MethodDef> {
    //     todo!()
    // }
}