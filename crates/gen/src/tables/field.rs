use super::*;
macros::table!(Field);

impl Field {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 1)
    }

    // TODO: find uses of Field::blob and replace with Field::signature?
    pub fn blob(&self) -> Blob {
        self.reader.blob(self.row, 2)
    }

    pub fn flags(&self) -> FieldFlags {
        FieldFlags(self.reader.u32(self.row, 0))
    }

    pub fn constant(&self) -> Option<Constant> {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::Constant,
                1,
                HasConstant::Field(*self).encode(),
            )
            .map(move |row| Constant {
                reader: self.reader,
                row,
            })
            .next()
    }

    pub fn signature(&self) -> Signature {
        let mut blob = self.blob();
        blob.read_unsigned();
        blob.read_modifiers();
        Signature::from_blob(&mut blob, &[]).expect("Field")
    }

    pub fn definition(&self) -> Option<TypeDef> {
        self.signature().definition()
    }

    pub fn is_blittable(&self) -> bool {
        self.signature().is_blittable()
    }

    pub fn gen_name(&self) -> Ident {
        to_ident(&to_snake(self.name()))
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
