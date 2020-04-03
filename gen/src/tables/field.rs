use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field(pub Row);

impl Field {
    pub fn name(self, reader: &Reader) -> &str {
        reader.str(self.0, 1)
    }

    pub fn sig(self, reader: &Reader) -> Blob {
        reader.blob(self.0, 2)
    }

    pub fn constants(self, reader: &Reader) -> impl Iterator<Item = Constant> {
        reader
            .equal_range(
                self.0.file,
                TABLE_CONSTANT,
                1,
                HasConstant::Field(self).encode(),
            )
            .map(Constant)
    }
}
