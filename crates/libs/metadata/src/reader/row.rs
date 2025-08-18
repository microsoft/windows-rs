use super::*;

#[derive(Copy, Clone)]
pub struct Row<'a> {
    pub index: &'a TypeIndex,
    pub file: usize,
    pub pos: usize,
}

impl std::fmt::Debug for Row<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("file", &self.file)
            .field("pos", &self.pos)
            .finish()
    }
}

impl std::hash::Hash for Row<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.pos.hash(state);
    }
}

impl PartialEq for Row<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.file, self.pos) == (other.file, other.pos)
    }
}

impl Eq for Row<'_> {}

impl Ord for Row<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.file, self.pos).cmp(&(other.file, other.pos))
    }
}

impl PartialOrd for Row<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

unsafe impl Send for Row<'_> {}
unsafe impl Sync for Row<'_> {}

impl<'a> Row<'a> {
    pub(crate) fn new(index: &'a TypeIndex, file: usize, pos: usize) -> Self {
        Self { index, file, pos }
    }
}

pub trait AsRow<'a>: Copy {
    const TABLE: usize;
    fn to_row(&self) -> Row<'a>;
    fn from_row(row: Row<'a>) -> Self;

    fn index(&self) -> &'a TypeIndex {
        let row = self.to_row();
        row.index
    }

    fn file(&self) -> &'a File {
        let row = self.to_row();
        row.index.files(row.file)
    }

    fn pos(&self) -> usize {
        self.to_row().pos
    }

    fn usize(&self, column: usize) -> usize {
        self.file().usize(self.pos(), Self::TABLE, column)
    }

    fn str(&self, column: usize) -> &'a str {
        self.file().str(self.pos(), Self::TABLE, column)
    }

    fn row<R: AsRow<'a>>(&self, column: usize) -> R {
        let row = self.to_row();
        R::from_row(Row::new(row.index, row.file, self.usize(column) - 1))
    }

    fn decode<T: Decode<'a>>(&self, column: usize) -> T {
        let row = self.to_row();
        T::decode(row.index, row.file, self.usize(column))
    }

    fn blob(&self, column: usize) -> Blob<'a> {
        let row = self.to_row();
        Blob::new(
            row.index,
            row.file,
            self.file().blob(self.pos(), Self::TABLE, column),
        )
    }

    fn list<R: AsRow<'a>>(&self, column: usize) -> RowIterator<'a, R> {
        let row = self.to_row();
        RowIterator::new(
            row.index,
            row.file,
            self.file().list(self.pos(), Self::TABLE, column, R::TABLE),
        )
    }

    fn equal_range<L: AsRow<'a>>(&self, column: usize, value: usize) -> RowIterator<'a, L> {
        let row = self.to_row();

        RowIterator::new(
            row.index,
            row.file,
            self.file().equal_range(L::TABLE, column, value),
        )
    }

    fn parent_row<P: AsRow<'a>>(&self, column: usize) -> P {
        let row = self.to_row();

        P::from_row(Row::new(
            row.index,
            row.file,
            self.file().parent(self.pos(), P::TABLE, column),
        ))
    }
}

pub struct RowIterator<'a, R: AsRow<'a>> {
    index: &'a TypeIndex,
    file: usize,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<'a, R: AsRow<'a>> RowIterator<'a, R> {
    pub(crate) fn new(index: &'a TypeIndex, file: usize, rows: std::ops::Range<usize>) -> Self {
        Self {
            index,
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
            .map(|row| R::from_row(Row::new(self.index, self.file, row)))
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
