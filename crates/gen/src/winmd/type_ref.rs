use super::*;
macros::table!(TypeRef);

impl TypeRef {
    pub fn scope(&self) -> ResolutionScope {
        self.reader.decode(self.row, 0)
    }

    pub fn name(&self) -> (&'static str, &'static str) {
        (self.reader.str(self.row, 2), self.reader.str(self.row, 1))
    }

    pub fn resolve(&self) -> TypeDef {
        self.reader.expect_type_def(self.name())
    }
}
