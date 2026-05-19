use super::*;

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TypeRef({}.{})", self.namespace(), self.name())
    }
}

impl TypeRef {
    pub fn scope(&self) -> ResolutionScope {
        self.decode(0)
    }

    pub fn name(&self) -> &str {
        self.str(1)
    }

    pub fn namespace(&self) -> &str {
        self.str(2)
    }
}
