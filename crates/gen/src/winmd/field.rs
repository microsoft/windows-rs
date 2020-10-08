use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Field(pub Row);

impl Field {
    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 1)
    }

    pub fn sig(self, reader: &TypeReader) -> Blob {
        reader.blob(self.0, 2)
    }

    pub fn flags(self, reader: &TypeReader) -> FieldFlags {
        FieldFlags(reader.u32(self.0, 0))
    }

    pub fn constants(self, reader: &TypeReader) -> impl Iterator<Item = Constant> {
        reader
            .equal_range(
                self.0.file_index,
                TableIndex::Constant,
                1,
                HasConstant::Field(self).encode(),
            )
            .map(Constant)
    }
}
