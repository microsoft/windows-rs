use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef<'a>(Row<'a>, Vec<Type<'a>>);

impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> TypeAttributes {
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn namespace(&self) -> &str {
        todo!()
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