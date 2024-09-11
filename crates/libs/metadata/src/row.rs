use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Row {
    file: &'static File,
    index: usize,
}

impl Row {
    pub(crate) fn new(file: &'static File, index: usize) -> Self {
        Self { file, index }
    }
}

pub trait AsRow: Copy {
    const TABLE: usize;
    fn to_row(&self) -> Row;
    fn from_row(row: Row) -> Self;

    fn file(&self) -> &'static File {
        self.to_row().file
    }

    fn reader(&self) -> &'static Reader {
        // Safety: At this point the File is already pointing to a valid Reader.
        unsafe { &*self.file().reader }
    }

    fn index(&self) -> usize {
        self.to_row().index
    }

    fn usize(&self, column: usize) -> usize {
        self.file().usize(self.index(), Self::TABLE, column)
    }

    fn str(&self, column: usize) -> &'static str {
        self.file().str(self.index(), Self::TABLE, column)
    }

    fn row(&self, column: usize) -> Row {
        Row::new(self.file(), self.usize(column) - 1)
    }

    fn decode<T: Decode>(&self, column: usize) -> T {
        T::decode(self.file(), self.usize(column))
    }

    fn blob(&self, column: usize) -> Blob {
        self.file().blob(self.index(), Self::TABLE, column)
    }

    fn list<R: AsRow>(&self, column: usize) -> RowIterator<R> {
        self.file().list(self.index(), Self::TABLE, column)
    }
}

pub struct RowIterator<R: AsRow> {
    file: &'static File,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<R: AsRow> RowIterator<R> {
    pub(crate) fn new(file: &'static File, rows: std::ops::Range<usize>) -> Self {
        Self {
            file,
            rows,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<R: AsRow> Iterator for RowIterator<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows
            .next()
            .map(|row| R::from_row(Row::new(self.file, row)))
    }
}

pub trait HasAttributes {
    fn attributes(&self) -> RowIterator<Attribute>;
    fn find_attribute(&self, name: &str) -> Option<Attribute>;
    fn has_attribute(&self, name: &str) -> bool;
}

impl<R: AsRow + Into<HasAttribute>> HasAttributes for R {
    fn attributes(&self) -> RowIterator<Attribute> {
        self.file()
            .equal_range(0, Into::<HasAttribute>::into(*self).encode())
    }

    fn find_attribute(&self, name: &str) -> Option<Attribute> {
        self.attributes().find(|attribute| attribute.name() == name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.find_attribute(name).is_some()
    }
}
