use super::*;
use crate::TypeReader;

#[derive(Copy, Clone)]
pub struct ImplMap {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl ImplMap {
    pub fn flags(&self) -> u32 {
        self.reader.u32(self.row, 0)
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 2)
    }

    pub fn scope(&self) -> &'static str {
        // scope(3) is index into ModuleRef table - return its name(0) - from string heap
        panic!();
    }
}
