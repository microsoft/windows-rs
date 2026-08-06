use super::*;

impl std::fmt::Debug for PropertyMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("PropertyMap").field(&self.0).finish()
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
