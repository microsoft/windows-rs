use super::*;

impl std::fmt::Debug for MethodSemantics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("MethodSemantics")
            .field(&self.semantics())
            .finish()
    }
}

impl<'a> MethodSemantics<'a> {
    pub fn semantics(&self) -> u16 {
        self.usize(0).try_into().unwrap()
    }

    pub fn method(&self) -> MethodDef<'a> {
        self.row(1)
    }

    pub fn association(&self) -> HasSemantics<'a> {
        self.decode(2)
    }
}
