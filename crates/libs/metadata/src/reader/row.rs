use super::*;

/// An owned, lifetime-free handle to a single row in a winmd table.
///
/// `Row` holds a clone of the `Arc`-backed `TypeIndex`, so it can be
/// stored, hashed, compared, and sent across threads without any lifetime
/// annotations.  Cloning a `Row` is cheap: it just increments the `Arc`
/// reference count.
#[derive(Clone)]
pub struct Row {
    pub index: TypeIndex,
    pub file: usize,
    pub pos: usize,
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("file", &self.file)
            .field("pos", &self.pos)
            .finish()
    }
}

impl std::hash::Hash for Row {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.pos.hash(state);
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        (self.file, self.pos) == (other.file, other.pos)
    }
}

impl Eq for Row {}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.file, self.pos).cmp(&(other.file, other.pos))
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Row {
    pub(crate) fn new(index: TypeIndex, file: usize, pos: usize) -> Self {
        Self { index, file, pos }
    }
}

/// Trait implemented by all winmd table row wrappers.
///
/// Methods returning `&str` or `Blob` borrow from `&self` (which keeps the
/// backing `Arc<TypeIndexData>` alive), so no explicit lifetime is needed.
pub trait AsRow: Clone {
    const TABLE: usize;
    fn row_ref(&self) -> &Row;
    fn from_row(row: Row) -> Self;

    fn pos(&self) -> usize {
        self.row_ref().pos
    }

    fn file(&self) -> &File {
        let r = self.row_ref();
        r.index.files(r.file)
    }

    fn usize(&self, column: usize) -> usize {
        self.file().usize(self.pos(), Self::TABLE, column)
    }

    fn str(&self, column: usize) -> &str {
        self.file().str(self.pos(), Self::TABLE, column)
    }

    /// Return another row referenced by a column value (1-based index → 0-based pos).
    fn row_at<R: AsRow>(&self, column: usize) -> R {
        let r = self.row_ref();
        R::from_row(Row::new(r.index.clone(), r.file, self.usize(column) - 1))
    }

    fn decode<T: Decode>(&self, column: usize) -> T {
        let r = self.row_ref();
        T::decode(r.index.clone(), r.file, self.usize(column))
    }

    fn blob(&self, column: usize) -> Blob<'_> {
        let r = self.row_ref();
        Blob::new(
            &r.index,
            r.file,
            self.file().blob(self.pos(), Self::TABLE, column),
        )
    }

    fn list<R: AsRow>(&self, column: usize) -> RowIterator<R> {
        let r = self.row_ref();
        RowIterator::new(
            r.index.clone(),
            r.file,
            self.file().list(self.pos(), Self::TABLE, column, R::TABLE),
        )
    }

    fn equal_range<L: AsRow>(&self, column: usize, value: usize) -> RowIterator<L> {
        let r = self.row_ref();
        RowIterator::new(
            r.index.clone(),
            r.file,
            self.file().equal_range(L::TABLE, column, value),
        )
    }

    fn parent_row<P: AsRow>(&self, column: usize) -> P {
        let r = self.row_ref();
        P::from_row(Row::new(
            r.index.clone(),
            r.file,
            self.file().parent(self.pos(), P::TABLE, column),
        ))
    }
}

pub struct RowIterator<R: AsRow> {
    index: TypeIndex,
    file: usize,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<R: AsRow> RowIterator<R> {
    pub(crate) fn new(index: TypeIndex, file: usize, rows: std::ops::Range<usize>) -> Self {
        Self {
            index,
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
            .map(|pos| R::from_row(Row::new(self.index.clone(), self.file, pos)))
    }
}

impl<R: AsRow> ExactSizeIterator for RowIterator<R> {
    fn len(&self) -> usize {
        self.rows.len()
    }
}

pub trait HasAttributes {
    fn attributes(&self) -> RowIterator<Attribute>;
    fn find_attribute(&self, name: &str) -> Option<Attribute>;
    fn has_attribute(&self, name: &str) -> bool;
    fn arches(&self) -> i32;
}

impl<R: AsRow + Into<HasAttribute>> HasAttributes for R {
    fn attributes(&self) -> RowIterator<Attribute> {
        self.equal_range(0, Into::<HasAttribute>::into(self.clone()).encode())
    }

    fn find_attribute(&self, name: &str) -> Option<Attribute> {
        self.attributes()
            .find(|attribute| attribute.ctor().parent().name() == name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.find_attribute(name).is_some()
    }

    fn arches(&self) -> i32 {
        let mut arches = 0;

        if let Some(attribute) = self.find_attribute("SupportedArchitectureAttribute") {
            arches = match attribute.value().first() {
                Some((_, Value::I32(v))) => *v,
                Some((_, Value::EnumValue(_, inner))) => {
                    if let Value::I32(v) = inner.as_ref() {
                        *v
                    } else {
                        0
                    }
                }
                _ => 0,
            };
        }

        arches
    }
}
