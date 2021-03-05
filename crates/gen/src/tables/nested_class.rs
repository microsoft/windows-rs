use super::*;
macros::table!(NestedClass);

impl NestedClass {
    pub fn nested_class(&self) -> TypeDef {
        let row = Row::new(self.reader.u32(self.row, 0) - 1, TableIndex::TypeDef, self.row.file_index);
        TypeDef {
            reader: self.reader,
            row
        }

    }

    pub fn enclosing_type(&self) -> TypeDef {
        let row = Row::new(self.reader.u32(self.row, 1) - 1, TableIndex::TypeDef, self.row.file_index);
        TypeDef {
            reader: self.reader,
            row
        }
    }
}
