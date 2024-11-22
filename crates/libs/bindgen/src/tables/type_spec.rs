use super::*;

impl std::fmt::Debug for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypeSpec").field(&self.0).finish()
    }
}

// impl TypeSpec {
//     pub fn ty(&self, generics: &[Type]) -> Type {
//             self.blob(0).winrt_type_from_spec(generics)
//     }
// }
