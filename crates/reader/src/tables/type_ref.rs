use super::*;

#[derive(Clone)]
pub struct TypeRef(pub Row);

impl TypeRef {
    pub fn scope(&self) -> ResolutionScope {
        self.0.decode(0)
    }

    pub fn name(&self) -> &'static str {
        self.0.str(1)
    }

    pub fn namespace(&self) -> &'static str {
        self.0.str(2)
    }

    pub fn type_name(&self) -> TypeName {
        TypeName::new(self.0.str(2), self.0.str(1))
    }

    pub fn resolve(&self) -> TypeDef {
        TypeReader::get().expect_type_ref(self)
    }
}
