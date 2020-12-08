use super::*;
use crate::{TableIndex, TypeReader};

#[derive(Copy, Clone)]
pub struct Field {
    pub reader: &'static TypeReader,
    pub row: Row,
}

impl Field {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 1)
    }

    pub fn sig(&self) -> Blob {
        self.reader.blob(self.row, 2)
    }

    pub fn flags(&self) -> FieldFlags {
        FieldFlags(self.reader.u32(self.row, 0))
    }

    pub fn constants(&self) -> impl Iterator<Item = Constant> + '_ {
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
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").field("row", &self.row).finish()
    }
}
