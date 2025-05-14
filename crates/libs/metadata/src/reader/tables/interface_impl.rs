use super::*;

impl std::fmt::Debug for InterfaceImpl<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InterfaceImpl").field(&self.0).finish()
    }
}

impl InterfaceImpl<'_> {
    pub fn class(&self) -> TypeDef {
        self.row(0)
    }

    pub fn interface(&self, generics: &[Type]) -> Type {
        self.decode::<TypeDefOrRef>(1).ty(generics)
    }
}
