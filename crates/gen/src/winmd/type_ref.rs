use super::*;
macros::table!(TypeRef);

impl TypeRef {
    pub fn scope(&self) -> ResolutionScope {
        self.reader.decode(self.row, 0)
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 1)
    }

    pub fn namespace(&self) -> &'static str {
        self.reader.str(self.row, 2)
    }

    pub fn full_name(&self) -> (&'static str, &'static str) {
        (self.namespace(), self.name())
    }

    pub fn resolve(&self) -> TypeDef {
        self.reader.resolve_type_def(self.namespace(), self.name())
    }
}
