use crate::flags::ParamFlags;
use crate::reader::Reader;
use crate::row::Row;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Param(pub Row);

impl Param {
    pub fn flags(self, reader: &Reader) -> ParamFlags {
        ParamFlags(reader.u32(self.0, 0))
    }

    pub fn sequence(self, reader: &Reader) -> u32 {
        reader.u32(self.0, 1)
    }

    pub fn name(self, reader: &Reader) -> &str {
        reader.str(self.0, 2)
    }
}
