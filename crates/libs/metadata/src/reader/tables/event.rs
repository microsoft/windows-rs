use super::*;

impl std::fmt::Debug for Event<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Event").field(&self.name()).finish()
    }
}

impl<'a> Event<'a> {
    pub fn flags(&self) -> u16 {
        self.usize(0).try_into().unwrap()
    }

    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    pub fn ty(&self, generics: &[Type]) -> Type {
        self.decode::<TypeDefOrRef>(2).ty(generics)
    }

    pub fn semantics(&self) -> RowIterator<'a, MethodSemantics<'a>> {
        self.equal_range(2, HasSemantics::Event(*self).encode())
    }
}
