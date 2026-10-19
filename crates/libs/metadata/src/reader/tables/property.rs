use super::*;

impl std::fmt::Debug for Property<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Property").field(&self.name()).finish()
    }
}

impl<'a> Property<'a> {
    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    pub fn ty(&self, generics: &[Type]) -> Type {
        let mut blob = self.blob(2);
        let prolog = blob.read_u8();
        debug_assert_eq!(prolog & 0x08, 0x08);
        let params = blob.read_compressed();
        debug_assert_eq!(params, 0);

        blob.read_type_signature(generics)
    }

    pub fn semantics(&self) -> RowIterator<'a, MethodSemantics<'a>> {
        self.equal_range(2, HasSemantics::Property(*self).encode())
    }

    pub fn getter(&self) -> Option<MethodDef<'a>> {
        find_semantics(self.semantics(), MethodSemanticsAttributes::Getter)
    }

    pub fn setter(&self) -> Option<MethodDef<'a>> {
        find_semantics(self.semantics(), MethodSemanticsAttributes::Setter)
    }
}

impl std::fmt::Debug for PropertyMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("PropertyMap").field(&self.parent()).finish()
    }
}

impl<'a> PropertyMap<'a> {
    pub fn parent(&self) -> TypeDef<'a> {
        self.row(0)
    }

    pub fn properties(&self) -> RowIterator<'a, Property<'a>> {
        self.list(1)
    }
}
