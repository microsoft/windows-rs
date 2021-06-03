use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field(pub Row);

impl Field {
    pub fn name(&self) -> &'static str {
        self.0.str(1)
    }

    // TODO: find uses of Field::blob and replace with Field::signature?
    pub fn blob(&self) -> Blob {
        self.0.blob(2)
    }

    pub fn flags(&self) -> FieldFlags {
        FieldFlags(self.0.u32(0))
    }

    pub fn constant(&self) -> Option<Constant> {
        self.0
            .file
            .equal_range(TableIndex::Constant, 1, HasConstant::Field(*self).encode())
            .map(Constant)
            .next()
    }

    pub fn parent(&self) -> TypeDef {
        // TODO: stick this in TypeReader
        let row = self.0.file.upper_bound_of(
            TableIndex::TypeDef,
            0,
            self.0.file.tables[TableIndex::TypeDef as usize].row_count,
            4,
            self.0.row + 1,
        ) - 1;

        Row::new(row, TableIndex::TypeDef, self.0.file).into()
    }

    pub fn dependencies(&self) -> Vec<ElementType> {
        self.signature().kind.definition()
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0
            .file
            .equal_range(
                TableIndex::CustomAttribute,
                0,
                HasAttribute::Field(*self).encode(),
            )
            .map(Attribute)
    }

    pub fn signature(&self) -> Signature {
        let mut blob = self.blob();
        blob.read_unsigned();
        blob.read_modifiers();
        Signature::from_blob(&mut blob, &[]).expect("Field")
    }

    pub fn definition(&self) -> Vec<ElementType> {
        self.signature().definition()
    }

    pub fn is_blittable(&self) -> bool {
        self.signature().is_blittable()
    }

    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.name());
        quote! { #name }
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        let r = TypeReader::get_struct("Windows.Foundation", "Rect");

        let f: Vec<Field> = r.0.fields().collect();
        assert_eq!(f.len(), 4);

        let s = f[0].signature();
        assert_eq!(s.kind, ElementType::F32);
    }
}
