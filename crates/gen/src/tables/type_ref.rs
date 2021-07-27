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

    pub fn full_name(&self) -> (&'static str, &'static str) {
        (self.namespace(), self.name())
    }

    pub fn resolve(&self) -> TypeDef {
        TypeReader::get().resolve_type_ref(self)
    }
}
