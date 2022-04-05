use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeSpec<'a>(pub Row<'a>);

impl<'a> TypeSpec<'a> {
    pub fn signature(&self) -> Blob {
        self.0.blob(0)
    }
}

// impl<'a> ToTypeName for TypeRef<'a> {
//     fn type_name(&self) -> TypeName {
//         TypeName::new(self.namespace(), self.name())
//     }
// }
