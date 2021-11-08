use super::*;

#[derive(Clone, Debug)]
pub struct Row {
    pub row: u32,
    pub table: TableIndex,
    pub file: &'static File,
}

impl Row {
    pub fn new(row: u32, table: TableIndex, file: &'static File) -> Self {
        Self { row, table, file }
    }

    pub fn u32(&self, column: u32) -> u32 {
        self.file.u32(self.row, self.table, column)
    }

    pub fn str(&self, column: u32) -> &'static str {
        self.file.str(self.row, self.table, column)
    }

    pub fn blob(&self, column: u32) -> Blob {
        self.file.blob(self.row, self.table, column)
    }

    pub fn decode<T: Decode>(&self, column: u32) -> T {
        self.file.decode(self.row, self.table, column)
    }

    pub fn list(&self, column: u32, table: TableIndex) -> impl Iterator<Item = Self> {
        self.file.list(self, table, column)
    }
}

impl core::hash::Hash for Row {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.table.hash(state);
        self.file.bytes.as_ptr().hash(state);
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        (self.row, self.table, self.file.bytes.as_ptr() as usize) == (other.row, other.table, other.file.bytes.as_ptr() as usize)
    }
}

impl Eq for Row {}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.row, self.table, self.file.bytes.as_ptr() as usize).cmp(&(other.row, other.table, other.file.bytes.as_ptr() as usize))
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
