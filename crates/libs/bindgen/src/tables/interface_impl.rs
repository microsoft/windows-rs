use super::*;

pub trait InterfaceImplExt {
    fn ty(&self, generics: &[Type]) -> Type;
}

impl InterfaceImplExt for InterfaceImpl {
    fn ty(&self, generics: &[Type]) -> Type {
        Type::from_ref(self.decode(1), None, generics)
    }
}
