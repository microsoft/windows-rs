use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

    // TODO: consider removing and making cache hits explicit
    pub fn resolve(&self) -> &TypeDef {
        TypeReader::get().resolve_type_ref(self)
    }
}

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.namespace(), self.name())
    }
}
