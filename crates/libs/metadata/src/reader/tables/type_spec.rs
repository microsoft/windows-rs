use super::*;

impl std::fmt::Debug for TypeSpec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("TypeSpec").field(&self.0).finish()
    }
}

impl TypeSpec<'_> {
    pub fn ty(&self, generics: &[Type]) -> Type {
        self.blob(0).read_type_code(generics)
    }
}
