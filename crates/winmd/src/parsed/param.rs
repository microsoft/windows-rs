use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct Param {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl Param {
    pub fn flags(&self) -> ParamFlags {
        ParamFlags(self.reader.u32(self.row, 0))
    }

    pub fn sequence(&self) -> u32 {
        self.reader.u32(self.row, 1)
    }

    pub fn name(&self) -> &str {
        self.reader.str(self.row, 2)
    }
}

impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Param").field("row", &self.row).finish()
    }
}
