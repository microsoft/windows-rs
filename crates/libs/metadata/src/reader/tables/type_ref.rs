use super::*;

impl std::fmt::Debug for TypeRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TypeRef({}.{})", self.namespace(), self.name())
    }
}

impl<'a> TypeRef<'a> {
    pub fn scope(&self) -> ResolutionScope<'a> {
        self.decode(0)
    }

    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    pub fn namespace(&self) -> &'a str {
        self.str(2)
    }

    /// Returns the outer namespace and slash-separated enclosing type names.
    pub fn full_name(&self) -> TypeName {
        if self.usize(0) != 0
            && let ResolutionScope::TypeRef(outer) = self.scope()
        {
            let mut name = outer.full_name();
            name.name.push('/');
            name.name.push_str(self.name());
            name
        } else {
            TypeName::named(self.namespace(), self.name())
        }
    }
}
