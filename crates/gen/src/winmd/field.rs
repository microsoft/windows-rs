use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Field(pub winmd::Row);

impl Field {
    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 1)
    }

    pub fn sig(self, reader: &TypeReader) -> winmd::Blob {
        reader.blob(self.0, 2)
    }

    pub fn flags(self, reader: &TypeReader) -> winmd::FieldFlags {
        winmd::FieldFlags(reader.u32(self.0, 0))
    }

    pub fn constants(self, reader: &TypeReader) -> impl Iterator<Item = winmd::Constant> {
        reader
            .equal_range(
                self.0.file_index,
                winmd::TableIndex::Constant,
                1,
                winmd::HasConstant::Field(self).encode(),
            )
            .map(winmd::Constant)
    }
}
