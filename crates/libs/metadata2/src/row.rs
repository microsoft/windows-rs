use super::*;
use std::marker::PhantomData;
use std::num::NonZeroU32;

/// A one-based row identity local to one metadata image.
pub struct RowId<T: Table> {
    value: NonZeroU32,
    marker: PhantomData<fn() -> T>,
}

impl<T: Table> RowId<T> {
    pub(crate) fn new(value: u32) -> Option<Self> {
        Some(Self {
            value: NonZeroU32::new(value)?,
            marker: PhantomData,
        })
    }

    /// Returns the one-based ECMA row number.
    pub const fn number(self) -> u32 {
        self.value.get()
    }
}

impl<T: Table> Clone for RowId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Table> Copy for RowId<T> {}

impl<T: Table> std::fmt::Debug for RowId<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(T::ID.schema().name())
            .field(&self.value)
            .finish()
    }
}

impl<T: Table> PartialEq for RowId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Table> Eq for RowId<T> {}

impl<T: Table> PartialOrd for RowId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Table> Ord for RowId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T: Table> std::hash::Hash for RowId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// A typed one-based list-start index, including the one-past-end sentinel.
pub struct ListIndex<T: Table> {
    value: NonZeroU32,
    marker: PhantomData<fn() -> T>,
}

impl<T: Table> ListIndex<T> {
    pub(crate) fn new(value: u32) -> Option<Self> {
        Some(Self {
            value: NonZeroU32::new(value)?,
            marker: PhantomData,
        })
    }

    /// Returns the encoded one-based list-start index.
    pub const fn number(self) -> u32 {
        self.value.get()
    }
}

impl<T: Table> Clone for ListIndex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Table> Copy for ListIndex<T> {}

impl<T: Table> PartialEq for ListIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Table> Eq for ListIndex<T> {}

impl<T: Table> std::hash::Hash for ListIndex<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: Table> std::fmt::Debug for ListIndex<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(T::ID.schema().name())
            .field(&self.value)
            .finish()
    }
}

/// A table-erased row identity produced by a coded index.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AnyRowId {
    table: TableId,
    value: NonZeroU32,
}

impl AnyRowId {
    pub(crate) fn new(table: TableId, value: u32) -> Option<Self> {
        Some(Self {
            table,
            value: NonZeroU32::new(value)?,
        })
    }

    /// Returns the referenced table.
    pub const fn table(self) -> TableId {
        self.table
    }

    /// Returns the one-based row number.
    pub const fn number(self) -> u32 {
        self.value.get()
    }
}

/// A checked view of one row in an owned metadata image.
pub struct Row<'a, T: Table> {
    image: &'a Image,
    id: RowId<T>,
}

impl<'a, T: Table> Clone for Row<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: Table> Copy for Row<'a, T> {}

impl<'a, T: Table> Row<'a, T> {
    pub(crate) const fn new(image: &'a Image, id: RowId<T>) -> Self {
        Self { image, id }
    }

    /// Returns this row's typed identity.
    pub const fn id(&self) -> RowId<T> {
        self.id
    }

    /// Reads a 16-bit integer column.
    pub fn u16(&self, column: usize) -> Result<u16, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::U16) {
            return Err(Error::invalid(offset, "column is not a 16-bit integer"));
        }
        Ok(value as u16)
    }

    /// Reads a 32-bit integer column.
    pub fn u32(&self, column: usize) -> Result<u32, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::U32) {
            return Err(Error::invalid(offset, "column is not a 32-bit integer"));
        }
        Ok(value)
    }

    /// Reads a string heap column.
    pub fn string(&self, column: usize) -> Result<&'a str, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::String) {
            return Err(Error::invalid(offset, "column is not a string index"));
        }
        self.image.string(StringId::new(value))
    }

    /// Reads a blob heap column.
    pub fn blob(&self, column: usize) -> Result<&'a [u8], Error> {
        self.image.blob(self.blob_id(column)?)
    }

    /// Reads the identifier stored in a blob heap column.
    pub fn blob_id(&self, column: usize) -> Result<BlobId, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::Blob) {
            return Err(Error::invalid(offset, "column is not a blob index"));
        }
        Ok(BlobId::new(value))
    }

    /// Reads a GUID heap column.
    pub fn guid(&self, column: usize) -> Result<Option<[u8; 16]>, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::Guid) {
            return Err(Error::invalid(offset, "column is not a GUID index"));
        }
        self.image.guid(GuidId::new(value))
    }

    /// Reads a direct table-index column.
    pub fn index<U: Table>(&self, column: usize) -> Result<Option<RowId<U>>, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::Table(table) if table == U::ID) {
            return Err(Error::invalid(
                offset,
                "column has a different table target",
            ));
        }
        if value == 0 {
            return Ok(None);
        }
        self.image
            .row(value)
            .map(Some)
            .ok_or_else(|| Error::invalid(offset, "table index is out of bounds"))
    }

    /// Reads a list-start column.
    pub fn list<U: Table>(&self, column: usize) -> Result<ListIndex<U>, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        if !matches!(kind, Column::List(table) if table == U::ID) {
            return Err(Error::invalid(offset, "column has a different list target"));
        }
        let limit = self.image.table(U::ID).rows().checked_add(1);
        if value == 0 || limit.is_none_or(|limit| value > limit) {
            return Err(Error::invalid(offset, "list index is out of bounds"));
        }
        Ok(ListIndex::new(value).unwrap())
    }

    /// Reads a coded-index column.
    pub fn coded(&self, column: usize) -> Result<Option<AnyRowId>, Error> {
        let (value, kind, offset) = self.image.column_data(self.id, column)?;
        let Column::Coded(code) = kind else {
            return Err(Error::invalid(offset, "column is not a coded index"));
        };
        self.image.decode_coded(code, value, offset)
    }
}

/// Iterates the rows of one metadata table.
pub struct Rows<T: Table> {
    next: u32,
    remaining: u32,
    marker: PhantomData<fn() -> T>,
}

impl<T: Table> Rows<T> {
    pub(crate) const fn new(rows: u32) -> Self {
        Self {
            next: 1,
            remaining: rows,
            marker: PhantomData,
        }
    }

    pub(crate) const fn range(start: u32, end: u32) -> Self {
        Self {
            next: start,
            remaining: end - start,
            marker: PhantomData,
        }
    }
}

impl<T: Table> Iterator for Rows<T> {
    type Item = RowId<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let row = RowId::new(self.next).unwrap();
        self.remaining -= 1;
        if self.remaining != 0 {
            self.next += 1;
        }
        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<T: Table> ExactSizeIterator for Rows<T> {}
