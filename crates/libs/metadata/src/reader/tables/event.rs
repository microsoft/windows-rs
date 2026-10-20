use super::*;

impl std::fmt::Debug for Event<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Event").field(&self.name()).finish()
    }
}

impl<'a> Event<'a> {
    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    /// Returns the event's handler delegate type.
    pub fn ty(&self, generics: &[Type]) -> Type {
        self.decode::<TypeDefOrRef>(2).ty(generics)
    }

    pub fn semantics(&self) -> RowIterator<'a, MethodSemantics<'a>> {
        self.equal_range(2, HasSemantics::Event(*self).encode())
    }

    pub fn adder(&self) -> Option<MethodDef<'a>> {
        find_semantics(self.semantics(), MethodSemanticsAttributes::AddOn)
    }

    pub fn remover(&self) -> Option<MethodDef<'a>> {
        find_semantics(self.semantics(), MethodSemanticsAttributes::RemoveOn)
    }
}

impl std::fmt::Debug for EventMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("EventMap").field(&self.parent()).finish()
    }
}

impl<'a> EventMap<'a> {
    pub fn parent(&self) -> TypeDef<'a> {
        self.row(0)
    }

    pub fn events(&self) -> RowIterator<'a, Event<'a>> {
        self.list(1)
    }
}
