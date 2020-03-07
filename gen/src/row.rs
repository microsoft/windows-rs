use std::marker::PhantomData;

pub trait Row {
    fn new(row: u32, file: u16) -> Self;
    fn table() -> u16;
}

pub struct RowIterator<T: Row> {
    pub first: u32,
    pub last: u32,
    pub file: u16,
    phantom: PhantomData<T>,
}

impl<T: Row> RowIterator<T> {
    pub fn new(first: u32, last: u32, file: u16) -> RowIterator<T> {
        RowIterator {
            first,
            last,
            file,
            phantom: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first >= self.last
    }
}

impl<T: Row> Iterator for RowIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.first >= self.last {
            None
        } else {
            self.first += 1;
            Some(T::new(self.first - 1, self.file))
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct RowData {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

impl RowData {
    pub fn invalid() -> RowData {
        RowData {
            row: u32::max_value(),
            table: u16::max_value(),
            file: u16::max_value(),
        }
    }

    pub fn new(row: u32, table: u16, file: u16) -> RowData {
        RowData { row, table, file }
    }

    pub fn next(&self) -> RowData {
        RowData::new(self.row + 1, self.table, self.file)
    }
}
