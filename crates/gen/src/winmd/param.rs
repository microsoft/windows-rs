use crate::flags::ParamFlags;
use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Param(pub Row);

impl Param {
    pub fn flags(self, reader: &TypeReader) -> ParamFlags {
        ParamFlags(reader.u32(self.0, 0))
    }

    pub fn sequence(self, reader: &TypeReader) -> u32 {
        reader.u32(self.0, 1)
    }

    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 2)
    }
}
