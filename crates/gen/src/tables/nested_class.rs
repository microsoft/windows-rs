use super::*;
macros::table!(NestedClass);

impl NestedClass {
    pub fn nested_type(&self) -> TypeDef {
        let row = Row::new(
            self.reader.u32(self.row, 0) - 1,
            TableIndex::TypeDef,
            self.row.file_index,
        );
        TypeDef {
            reader: self.reader,
            row,
        }
    }

    pub fn enclosing_type(&self) -> TypeDef {
        let row = Row::new(
            self.reader.u32(self.row, 1) - 1,
            TableIndex::TypeDef,
            self.row.file_index,
        );
        TypeDef {
            reader: self.reader,
            row,
        }
    }
}

impl std::fmt::Debug for NestedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("nested_type", &self.nested_type())
            .field("enclosing_type", &self.enclosing_type())
            .finish()
    }
}
