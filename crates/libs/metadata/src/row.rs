use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Row {
    pub file: &'static File,
    pub index: usize,
}

impl Row {
    pub fn new(file: &'static File, index: usize) -> Self {
        Self { file, index }
    }

    fn next(&self) -> Self {
        Self { file: self.file, index: self.index + 1 }
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

    fn next(&self) -> Self {
        Self::from_row(self.to_row().next())
    }

    fn usize(&self, column: usize) -> usize {
        self.file().usize(self.index(), Self::TABLE, column)
    }

    fn str(&self, column: usize) -> &'static str {
        let file = self.file();
        let offset = file.strings + self.usize(column);
        let bytes = &file.bytes[offset..];
        let nul_pos = bytes.iter().position(|&c| c == 0).expect("expected null-terminated C-string");
        std::str::from_utf8(&bytes[..nul_pos]).expect("expected valid utf-8 C-string")
    }

    fn row(&self, column: usize) -> Row {
        Row::new(self.file(), self.usize(column) - 1)
    }

    fn decode<T: Decode>(&self, column: usize) -> T {
        T::decode(self.file(), self.usize(column))
    }

    fn blob(&self, column: usize) -> Blob {
        let file = self.file();
        let offset = file.blobs + self.usize(column);
        let initial_byte = file.bytes[offset];

        let (blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            rest => unimplemented!("{rest:?}"),
        };

        let mut blob_size = blob_size as usize;

        for byte in &file.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + (*byte as usize);
        }

        let offset = offset + blob_size_bytes;
        Blob::new(file, &file.bytes[offset..offset + blob_size])
    }

    fn list<R: AsRow>(&self, column: usize) -> RowIterator<R> {
        let file = self.file();
        let first = self.usize(column) - 1;
        let next = self.next();
        let last = if next.index() < file.tables[Self::TABLE].len { next.usize(column) - 1 } else { file.tables[R::TABLE].len };
        RowIterator::new(file, first..last)
    }

    fn equal_range<L: AsRow>(&self, column: usize, value: usize) -> RowIterator<L> {
        let file = self.file();
        let mut first = 0;
        let mut last = file.tables[L::TABLE].len;
        let mut count = last;

        loop {
            if count == 0 {
                last = first;
                break;
            }

            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = file.usize(middle, L::TABLE, column);

            match middle_value.cmp(&value) {
                Ordering::Less => {
                    first = middle + 1;
                    count -= count2 + 1;
                }
                Ordering::Greater => count = count2,
                Ordering::Equal => {
                    let first2 = file.lower_bound_of(L::TABLE, first, middle, column, value);
                    first += count;
                    last = file.upper_bound_of(L::TABLE, middle + 1, first, column, value);
                    first = first2;
                    break;
                }
            }
        }

        RowIterator::new(file, first..last)
    }
}

pub struct RowIterator<R: AsRow> {
    file: &'static File,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<R: AsRow> RowIterator<R> {
    pub fn new(file: &'static File, rows: std::ops::Range<usize>) -> Self {
        Self { file, rows, phantom: std::marker::PhantomData }
    }
}

impl<R: AsRow> Iterator for RowIterator<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| R::from_row(Row::new(self.file, row)))
    }
}

pub trait HasAttributes {
    fn attributes(&self) -> RowIterator<Attribute>;
    fn find_attribute(&self, name: &str) -> Option<Attribute>;
    fn has_attribute(&self, name: &str) -> bool;
}

impl<R: AsRow + Into<HasAttribute>> HasAttributes for R {
    fn attributes(&self) -> RowIterator<Attribute> {
        self.equal_range(0, Into::<HasAttribute>::into(*self).encode())
    }

    fn find_attribute(&self, name: &str) -> Option<Attribute> {
        self.attributes().find(|attribute| attribute.name() == name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.find_attribute(name).is_some()
    }
}
