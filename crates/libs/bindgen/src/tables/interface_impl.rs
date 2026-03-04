use super::*;

pub trait InterfaceImplExt {
    fn ty(&self, generics: &[Type], reader: &Reader) -> Type;
}

impl InterfaceImplExt for InterfaceImpl {
    fn ty(&self, generics: &[Type], reader: &Reader) -> Type {
        Type::from_ref(self.decode(1), None, generics, reader)
    }
}
