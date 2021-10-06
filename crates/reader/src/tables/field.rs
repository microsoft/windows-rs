use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field(pub Row);

impl From<Row> for Field {
    fn from(row: Row) -> Self {
        Self(row)
    }
}

impl Field {
    pub fn name(&self) -> &'static str {
        self.0.str(1)
    }

    pub fn is_literal(&self) -> bool {
        self.0.u32(0) & 0b100_0000 != 0
    }

    pub fn constant(&self) -> Option<Constant> {
        self.0
            .file
            .equal_range(
                TableIndex::Constant,
                1,
                HasConstant::Field(self.clone()).encode(),
            )
            .map(Constant)
            .next()
    }

    pub fn parent(&self) -> TypeDef {
        let row = self.0.file.upper_bound_of(
            TableIndex::TypeDef,
            0,
            self.0.file.tables[TableIndex::TypeDef as usize].row_count,
            4,
            self.0.row + 1,
        ) - 1;

        Row::new(row, TableIndex::TypeDef, self.0.file).into()
    }

    pub fn include_dependencies(&self, reader: &mut TypeReader, include: TypeInclude) {
        self.signature().kind.include_definition(reader, include)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0.file.attributes(HasAttribute::Field(self.clone()))
    }

    pub fn signature(&self) -> Signature {
        let mut blob = self.0.blob(2);
        blob.read_unsigned();
        blob.read_modifiers();
        TypeReader::get()
            .signature_from_blob(&mut blob, &[])
            .expect("Field")
    }

    pub fn include_definition(&self, reader: &mut TypeReader, include: TypeInclude) {
        self.signature().include_definition(reader, include)
    }

    pub fn is_blittable(&self) -> bool {
        self.signature().is_blittable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        let r = TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "Rect"));

        let f: Vec<Field> = r.fields().collect();
        assert_eq!(f.len(), 4);

        let s = f[0].signature();
        assert!(s.kind == ElementType::F32);
    }
}
