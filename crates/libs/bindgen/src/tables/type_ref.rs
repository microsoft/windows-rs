use super::*;

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeRef({}.{})", self.namespace(), self.name())
    }
}

impl TypeRef {
    pub fn name(&self) -> &'static str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &'static str {
        self.str(2)
    }

    // pub fn type_def(&self) -> TypeDef {
    //     self.reader()
    //         .get(self.namespace(), self.name())
    //         .unwrap()
    //         .type_def()
    // }
}
