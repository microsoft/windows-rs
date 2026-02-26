use super::*;

pub struct File {
    pub(crate) reader: *const Reader,
    inner: windows_metadata::reader::File,
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::write!(f, "{:?}", self as *const _)
    }
}

impl std::hash::Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self as *const Self).hash(state);
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for File {}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        (self as *const Self).cmp(&(other as *const _))
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

unsafe impl Sync for File {}

impl File {
    pub fn new(bytes: Vec<u8>) -> Option<Self> {
        windows_metadata::reader::File::new(bytes).map(|inner| Self {
            reader: std::ptr::null(),
            inner,
        })
    }

    pub(crate) fn usize(&self, row: usize, table: usize, column: usize) -> usize {
        self.inner.usize(row, table, column)
    }

    pub(crate) fn str(&'static self, row: usize, table: usize, column: usize) -> &'static str {
        // Safety: `self` has a `'static` lifetime, which is enforced by `Reader::new()` in
        // `reader.rs` where all `File` instances are heap-allocated via `Box::new` and stored as
        // raw pointers in the `Reader`. The `File` objects live as long as the `Reader`. Since
        // the caller provides `&'static self`, the returned string slice pointing into the file's
        // byte buffer is also valid for `'static`.
        unsafe { std::mem::transmute(self.inner.str(row, table, column)) }
    }

    pub(crate) fn blob(&'static self, row: usize, table: usize, column: usize) -> Blob {
        // Safety: Same invariant as `str()` - `self` has a `'static` lifetime established by
        // `Reader::new()`, so the returned byte slice pointing into the file's buffer is also
        // valid for `'static`.
        let slice: &'static [u8] =
            unsafe { std::mem::transmute(self.inner.blob(row, table, column)) };
        Blob::new(self, slice)
    }

    pub(crate) fn list<R: AsRow>(
        &'static self,
        row: usize,
        table: usize,
        column: usize,
    ) -> RowIterator<R> {
        let range = self.inner.list(row, table, column, R::TABLE);
        RowIterator::new(self, range)
    }

    pub(crate) fn equal_range<L: AsRow>(
        &'static self,
        column: usize,
        value: usize,
    ) -> RowIterator<L> {
        let range = self.inner.equal_range(L::TABLE, column, value);
        RowIterator::new(self, range)
    }

    pub(crate) fn parent<P: AsRow, C: AsRow>(&'static self, column: usize, child: C) -> P {
        P::from_row(Row::new(
            self,
            self.inner.parent(child.index(), P::TABLE, column),
        ))
    }

    pub(crate) fn table<R: AsRow>(&'static self) -> RowIterator<R> {
        RowIterator::new(self, 0..self.inner.table_len(R::TABLE))
    }

    pub(crate) fn reader(&self) -> &'static Reader {
        // Safety: At this point the File is already pointing to a valid Reader.
        unsafe { &*self.reader }
    }
}
