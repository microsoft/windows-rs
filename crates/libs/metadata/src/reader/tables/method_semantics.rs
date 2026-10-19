use super::*;

impl std::fmt::Debug for MethodSemantics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MethodSemantics")
            .field("semantics", &self.semantics().0)
            .field("method", &self.method().name())
            .finish()
    }
}

impl<'a> MethodSemantics<'a> {
    pub fn semantics(&self) -> MethodSemanticsAttributes {
        MethodSemanticsAttributes(self.usize(0).try_into().unwrap())
    }

    pub fn method(&self) -> MethodDef<'a> {
        self.row(1)
    }
}

pub(crate) fn find_semantics<'a>(
    mut semantics: RowIterator<'a, MethodSemantics<'a>>,
    flag: MethodSemanticsAttributes,
) -> Option<MethodDef<'a>> {
    semantics
        .find(|row| row.semantics().contains(flag))
        .map(|row| row.method())
}
