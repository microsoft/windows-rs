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
        self.0.file.equal_range(TableIndex::Constant, 1, HasConstant::Field(self.clone()).encode()).map(Constant).next()
    }

    pub fn include_dependencies(&self, enclosing: Option<&TypeDef>, include: TypeInclude) {
        self.signature(enclosing).kind.include_definition(include)
    }

    pub fn include_definition(&self, enclosing: Option<&TypeDef>, include: TypeInclude) {
        self.signature(enclosing).kind.include_definition(include)
    }

    pub fn features(&self, enclosing: Option<&TypeDef>, features: &mut BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
        self.signature(enclosing).kind.features(features, keys);
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0.file.attributes(HasAttribute::Field(self.clone()))
    }

    pub fn signature(&self, enclosing: Option<&TypeDef>) -> Signature {
        let mut blob = self.0.blob(2);
        blob.read_unsigned();
        blob.read_modifiers();
        TypeReader::get().signature_from_blob(&mut blob, enclosing, &[]).expect("Field")
    }

    pub fn is_blittable(&self, enclosing: Option<&TypeDef>) -> bool {
        self.signature(enclosing).is_blittable()
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

        let s = f[0].signature(None);
        assert!(s.kind == ElementType::F32);
    }
}
