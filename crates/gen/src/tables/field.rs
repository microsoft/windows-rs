use super::*;
macros::table!(Field);

#[derive(Default)]
pub struct FieldFlags(pub u32);

impl FieldFlags {
    pub fn literal(&self) -> bool {
        self.0 & 0b100_0000 != 0
    }

    pub fn is_static(&self) -> bool {
        self.0 & 0b1_0000 != 0
    }
}

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

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.signature().dependencies()
    }

    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.name());
        quote! { #name }
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        let name = self.name();

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/90
        if name == "NaN" || name == "POSITIVE_INFINITY" || name == "NEGATIVE_INFINITY" {
            return quote! {};
        }

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/88
        if self.constant().is_none() {
            return quote! {};
        }

        let name = to_ident(name);
        let value = self.constant().expect("Field").value().gen();

        quote! {
            pub const #name: #value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get(namespace: &str, name:&str) -> Struct {
        if let ElementType::Struct(value) = TypeReader::get()
            .resolve_type(namespace, name) { value.clone() } else { unexpected!(); }
    }

    #[test]
    fn test_generic() {
        let r = get("Windows.Foundation", "Rect");

        let f: Vec<Field> = r.0.fields().collect();
        assert_eq!(f.len(), 4);

        let s = f[0].signature();
        assert_eq!(s.kind, ElementType::F32);
    }
}
