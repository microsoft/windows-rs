use super::*;

impl std::fmt::Debug for EventMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("EventMap").field(&self.0).finish()
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
