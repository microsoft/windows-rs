use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Row<'a> {
    file: &'a File,
    index: usize,
}

unsafe impl Send for Row<'_> {}
unsafe impl Sync for Row<'_> {}

impl<'a> Row<'a> {
    pub(crate) fn new(file: &'a File, index: usize) -> Self {
        Self { file, index }
    }
}

pub trait AsRow<'a>: Copy {
    const TABLE: usize;
    fn to_row(&self) -> Row<'a>;
    fn from_row(row: Row<'a>) -> Self;

    fn file(&self) -> &'a File {
        self.to_row().file
    }

    fn index(&self) -> usize {
        self.to_row().index
    }

    fn usize(&self, column: usize) -> usize {
        self.file().usize(self.index(), Self::TABLE, column)
    }

    fn str(&self, column: usize) -> &'a str {
        self.file().str(self.index(), Self::TABLE, column)
    }

    fn row<R: AsRow<'a>>(&self, column: usize) -> R {
        R::from_row(Row::new(self.file(), self.usize(column) - 1))
    }

    fn decode<T: Decode<'a>>(&self, column: usize) -> T {
        T::decode(self.file(), self.usize(column))
    }

    fn blob(&self, column: usize) -> Blob<'a> {
        self.file().blob(self.index(), Self::TABLE, column)
    }

    fn list<R: AsRow<'a>>(&self, column: usize) -> RowIterator<'a, R> {
        let file = self.file();
        RowIterator::new(file, file.list(self.index(), Self::TABLE, column, R::TABLE))
    }

    fn equal_range<L: AsRow<'a>>(&self, column: usize, value: usize) -> RowIterator<'a, L> {
        let file = self.file();

        RowIterator::new(file, file.equal_range(L::TABLE, column, value))
    }

    fn parent_row<P: AsRow<'a>>(&'a self, column: usize) -> P {
        let file = self.file();

        P::from_row(Row::new(file, file.parent(self.index(), P::TABLE, column)))
    }
}

pub struct RowIterator<'a, R: AsRow<'a>> {
    file: &'a File,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<'a, R: AsRow<'a>> RowIterator<'a, R> {
    pub(crate) fn new(file: &'a File, rows: std::ops::Range<usize>) -> Self {
        Self {
            file,
            rows,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, R: AsRow<'a>> Iterator for RowIterator<'a, R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows
            .next()
            .map(|row| R::from_row(Row::new(self.file, row)))
    }
}

pub trait HasAttributes<'a> {
    fn attributes(&self) -> RowIterator<'a, Attribute<'a>>;
    fn find_attribute(&self, name: &str) -> Option<Attribute<'a>>;
    fn has_attribute(&self, name: &str) -> bool;
}

impl<'a, R: AsRow<'a> + Into<HasAttribute<'a>>> HasAttributes<'a> for R {
    fn attributes(&self) -> RowIterator<'a, Attribute<'a>> {
        self.equal_range(0, Into::<HasAttribute>::into(*self).encode())
    }

    fn find_attribute(&self, name: &str) -> Option<Attribute<'a>> {
        self.attributes()
            .find(|attribute| attribute.ctor().parent().name() == name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.find_attribute(name).is_some()
    }
}
