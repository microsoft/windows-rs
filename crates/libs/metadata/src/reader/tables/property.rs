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

    /// Returns the property type, ignoring any index parameters.
    pub fn ty(&self, generics: &[Type]) -> Type {
        // A PropertySig has the same layout as a MethodDefSig (ECMA-335 II.23.2.5).
        self.blob(2).read_method_signature(generics).return_type
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
