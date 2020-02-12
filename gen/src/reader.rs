use crate::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub trait Code {
    fn decode(code: u32, file: u16) -> Self;
    fn encode(&self) -> u32;
}

pub trait Row {
    fn new(row: u32, file: u16) -> Self;
    fn table() -> u16;
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct RowData {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

impl RowData {
    pub fn invalid() -> RowData {
        RowData { row: u32::max_value(), table: u16::max_value(), file: u16::max_value() }
    }

    pub fn new(row: u32, table: u16, file: u16) -> RowData {
        RowData { row, table, file }
    }

    fn next(&self) -> RowData {
        RowData::new(self.row + 1, self.table, self.file)
    }
}

pub struct RowIterator<T: Row> {
    pub first: u32,
    pub last: u32,
    pub file: u16,
    phantom: PhantomData<T>,
}

impl<T: Row> RowIterator<T> {
    pub fn new(first: u32, last: u32, file: u16) -> RowIterator<T> {
        RowIterator { first, last, file, phantom: PhantomData }
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

pub struct Reader {
    files: Vec<File>,
    types: BTreeMap<String, BTreeMap<String, TypeDef>>,
}

impl<'a> Reader {
    pub fn from_files<P: IntoIterator<Item = &'a String>>(filenames: P) -> Result<Self, Error> {
        let mut r = Reader { files: Vec::new(), types: Default::default() };

        for filename in filenames {
            r.files.push(File::new(filename)?);
            let table = &r.files[r.files.len() - 1].tables[TABLE_TYPEDEF];

            for row in 0..table.row_count {
                let t = TypeDef::new(row, r.files.len() as u16 - 1);
                if t.flags(&r).windows_runtime() {
                    let name = t.name(&r).to_string();
                    let namespace = t.namespace(&r).to_string();
                    r.types.entry(namespace).or_insert_with(|| Default::default()).entry(name).or_insert(t);
                }
            }
        }

        Ok(r)
    }

    pub fn from_dir<P: AsRef<std::path::Path>>(directory: P) -> Result<Self, Error> {
        let files: Vec<String> = std::fs::read_dir(directory)?.filter_map(|value| value.ok().map(|value| value.path().to_str().unwrap().to_string())).collect();
        Self::from_files(&files)
    }

    pub fn from_os() -> Result<Self, Error> {
        let mut path = std::path::PathBuf::new();
        path.push(std::env::var("windir").expect("'windir' environment variable not found"));
        path.push(SYSTEM32);
        path.push("winmetadata");
        Self::from_dir(path)
    }

    // TODO: panic with "'full name' not found"
    pub fn resolve(&self, full_name: &str) -> TypeDef {
        let (namespace, name) = split_type_name(full_name);
        *self.types.get(namespace).unwrap().get(name).unwrap()
    }

    pub fn namespaces(&self) -> &BTreeMap<String, BTreeMap<String, TypeDef>> {
        &self.types
    }

    pub fn namespace_types(&self, namespace: &str) -> std::collections::btree_map::Values<String, TypeDef> {
        self.types[namespace].values()
    }

    pub fn decode<T: Code>(&self, row: &RowData, column: u32) -> T {
        T::decode(self.u32(row, column), row.file)
    }

    pub fn u32(&self, row: &RowData, column: u32) -> u32 {
        let file = &self.files[row.file as usize];
        let table = &file.tables[row.table as usize];
        let offset = table.data + row.row * table.row_size + table.columns[column as usize].0;
        match table.columns[column as usize].1 {
            1 => *(file.bytes.view_as::<u8>(offset).unwrap()) as u32,
            2 => *(file.bytes.view_as::<u16>(offset).unwrap()) as u32,
            4 => *(file.bytes.view_as::<u32>(offset).unwrap()) as u32,
            _ => *(file.bytes.view_as::<u64>(offset).unwrap()) as u32,
        }
    }

    pub fn str(&self, row: &RowData, column: u32) -> &str {
        let file = &self.files[row.file as usize];
        let offset = (file.strings + self.u32(row, column)) as usize;
        let last = file.bytes[offset..].iter().position(|c| *c == b'\0').unwrap();
        std::str::from_utf8(&file.bytes[offset..offset + last]).unwrap()
    }

    pub fn blob(&self, row: &RowData, column: u32) -> &[u8] {
        let file = &self.files[row.file as usize];
        let offset = (file.blobs + self.u32(row, column)) as usize;
        let initial_byte = file.bytes[offset];
        let (mut blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            _ => panic!(),
        };
        for byte in &file.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + byte;
        }
        &file.bytes[offset + blob_size_bytes..offset + blob_size_bytes + blob_size as usize]
    }

    pub fn list<T: Row>(&self, row: &RowData, column: u32) -> RowIterator<T> {
        let file = &self.files[row.file as usize];
        let first = self.u32(row, column) - 1;

        let last = if row.row + 1 < file.tables[row.table as usize].row_count { self.u32(&row.next(), column) - 1 } else { file.tables[T::table() as usize].row_count };

        RowIterator::new(first, last, row.file)
    }

    fn lower_bound_of(&self, table: u16, file: u16, mut first: u32, last: u32, column: u32, value: u32) -> u32 {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if self.u32(&RowData::new(middle, table, file), column) < value {
                first = middle + 1;
                count -= count2 + 1;
            } else {
                count = count2;
            }
        }
        first
    }

    fn upper_bound_of(&self, table: u16, file: u16, mut first: u32, last: u32, column: u32, value: u32) -> u32 {
        let mut count = last - first;

        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if value < self.u32(&RowData::new(middle, table, file), column) {
                count = count2
            } else {
                first = middle + 1;
                count -= count2 + 1;
            }
        }

        first
    }

    fn equal_range_of(&self, table: u16, file: u16, mut first: u32, mut last: u32, column: u32, value: u32) -> (u32, u32) {
        let mut count = last - first;
        loop {
            if count <= 0 {
                last = first;
                break;
            }
            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = self.u32(&RowData::new(middle, table, file), column);
            if middle_value < value {
                first = middle + 1;
                count -= count2 + 1;
            } else if value < middle_value {
                count = count2;
            } else {
                let first2 = self.lower_bound_of(table, file, first, middle, column, value);
                first += count;
                last = self.upper_bound_of(table, file, middle + 1, first, column, value);
                first = first2;
                break;
            }
        }
        (first, last)
    }

    pub fn equal_range<T: Row>(&self, file: u16, column: u32, value: u32) -> RowIterator<T> {
        let table = T::table();
        let (first, last) = self.equal_range_of(table, file, 0, self.files[file as usize].tables[table as usize].row_count, column, value);
        RowIterator::new(first, last, file)
    }

    pub fn upper_bound<T: Row>(&self, file: u16, column: u32, value: u32) -> T {
        let table = T::table();
        T::new(self.upper_bound_of(table, file, 0, self.files[file as usize].tables[table as usize].row_count, column, value), file)
    }
}

pub fn split_type_name(name: &str) -> (&str, &str) {
    let index = name.rfind('.').unwrap();
    (&name[0..index], &name[index + 1..])
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
