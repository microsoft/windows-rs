use super::*;

impl std::fmt::Debug for InterfaceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("InterfaceImpl").field(&self.0).finish()
    }
}

impl InterfaceImpl {
    pub fn class(&self) -> TypeDef {
        self.row_at(0)
    }

    pub fn interface(&self, generics: &[Type]) -> Type {
        self.decode::<TypeDefOrRef>(1).ty(generics)
    }
}
