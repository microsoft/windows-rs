use super::*;

impl std::fmt::Debug for Property<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Property").field(&self.name()).finish()
    }
}

impl<'a> Property<'a> {
    pub fn flags(&self) -> u16 {
        self.usize(0).try_into().unwrap()
    }

    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    pub fn signature(&self, generics: &[Type]) -> Signature {
        self.blob(2).read_method_signature(generics)
    }

    pub fn constant(&self) -> Option<Constant<'a>> {
        self.equal_range(1, HasConstant::Property(*self).encode())
            .next()
    }

    pub fn semantics(&self) -> RowIterator<'a, MethodSemantics<'a>> {
        self.equal_range(2, HasSemantics::Property(*self).encode())
    }
}
