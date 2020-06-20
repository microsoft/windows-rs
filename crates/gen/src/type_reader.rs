use crate::blob::Blob;
use crate::codes::Decode;
use crate::file::{TableIndex, View, WinmdFile};
use crate::row::Row;
use crate::tables::TypeDef;

use std::cmp::Ordering;
use std::collections::BTreeMap;

/// A reader of type information from Windows Metadata
pub struct TypeReader {
    /// The parsed Windows metadata files the [`TypeReader`] has access to
    pub files: Vec<WinmdFile>,
    /// Types known to this [`TypeReader`]
    ///
    /// This is a mapping between namespace names and the types inside
    /// that namespace
    pub types: BTreeMap<String, BTreeMap<String, TypeDef>>,
}

impl TypeReader {
    pub fn from_os() -> Self {
        Self::new(crate::load_winmd::from_os())
    }

    /// Create a new [`TypeReader`] from a [`WinmdFile`]s
    pub fn new(files: Vec<WinmdFile>) -> Self {
        let mut reader = Self {
            files: Vec::default(),
            types: BTreeMap::default(),
        };
        for (file_index, file) in files.into_iter().enumerate() {
            let row_count = file.type_def_table().row_count;
            reader.files.push(file);

            for row in 0..row_count {
                let def = TypeDef(Row::new(row, TableIndex::TypeDef, file_index as u16));

                if def.ignore(&reader) {
                    continue;
                }

                let (namespace, name) = def.name(&reader);
                let namespace = namespace.to_string();
                let name = name.to_string();

                reader
                    .types
                    .entry(namespace)
                    .or_default()
                    .entry(name)
                    .or_insert(def);
            }
        }
        reader
    }

    /// Get all the namespace names that the [`TypeReader`] knows about
    pub fn namespaces(&self) -> impl Iterator<Item = &String> {
        self.types.keys()
    }

    /// Get all type definitions ([`TypeDef`]s) for a given namespace
    ///
    /// # Panics
    ///
    /// Panics if the namespace does not exist
    pub fn namespace_types(&self, namespace: &str) -> impl Iterator<Item = (&str, &TypeDef)> {
        self.types[namespace].iter().map(|(n, t)| (n.as_str(), t))
    }

    /// Resolve a type definition given its namespace and type name
    ///
    /// # Panics
    ///
    /// Panics if no type definition for the given namespace and type name can be found
    pub fn resolve_type_def(&self, (namespace, type_name): (&str, &str)) -> TypeDef {
        if let Some(types) = self.types.get(namespace) {
            if let Some(def) = types.get(type_name) {
                return *def;
            }
        }

        panic!("Could not find type `{}.{}`", namespace, type_name);
    }

    /// Read a [`u32`] value from a specific [`Row`] and column
    pub fn u32(&self, row: Row, column: u32) -> u32 {
        let file = &self.files[row.file_index as usize];
        let table = &file.tables[row.table_index as usize];
        let offset = table.data + row.index * table.row_size + table.columns[column as usize].0;
        match table.columns[column as usize].1 {
            1 => file.bytes.copy_as::<u8>(offset) as u32,
            2 => file.bytes.copy_as::<u16>(offset) as u32,
            4 => file.bytes.copy_as::<u32>(offset) as u32,
            _ => file.bytes.copy_as::<u64>(offset) as u32,
        }
    }

    /// Read a [`&str`] value from a specific [`Row`] and column
    pub fn str(&self, row: Row, column: u32) -> &str {
        let file = &self.files[row.file_index as usize];
        let offset = (file.strings + self.u32(row, column)) as usize;
        let last = file.bytes[offset..]
            .iter()
            .position(|c| *c == b'\0')
            .unwrap();
        std::str::from_utf8(&file.bytes[offset..offset + last]).unwrap()
    }

    /// Read a `T: Decode` value from a specific [`Row`] and column
    pub fn decode<T: Decode>(&self, row: Row, column: u32) -> T {
        T::decode(self.u32(row, column), row.file_index)
    }

    pub fn list(&self, row: Row, table: TableIndex, column: u32) -> impl Iterator<Item = Row> {
        let file = &self.files[row.file_index as usize];
        let first = self.u32(row, column) - 1;

        let last = if row.index + 1 < file.tables[row.table_index as usize].row_count {
            self.u32(row.next(), column) - 1
        } else {
            file.tables[table as usize].row_count
        };

        (first..last).map(move |value| Row::new(value, table, row.file_index))
    }

    pub fn blob(&self, row: Row, column: u32) -> Blob {
        let file = &self.files[row.file_index as usize];
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
        Blob::new(self, row.file_index, offset + blob_size_bytes)
    }

    pub fn equal_range(
        &self,
        file: u16,
        table: TableIndex,
        column: u32,
        value: u32,
    ) -> impl Iterator<Item = Row> {
        let (first, last) = self.equal_range_of(
            table,
            file,
            0,
            self.files[file as usize].tables[table as usize].row_count,
            column,
            value,
        );

        (first..last).map(move |row| Row::new(row, table, file))
    }

    fn lower_bound_of(
        &self,
        table: TableIndex,
        file: u16,
        mut first: u32,
        last: u32,
        column: u32,
        value: u32,
    ) -> u32 {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if self.u32(Row::new(middle, table, file), column) < value {
                first = middle + 1;
                count -= count2 + 1;
            } else {
                count = count2;
            }
        }
        first
    }

    pub fn upper_bound(&self, file: u16, table: TableIndex, column: u32, value: u32) -> Row {
        Row::new(
            self.upper_bound_of(
                table,
                file,
                0,
                self.files[file as usize].tables[table as usize].row_count,
                column,
                value,
            ),
            table,
            file,
        )
    }

    fn upper_bound_of(
        &self,
        table: TableIndex,
        file: u16,
        mut first: u32,
        last: u32,
        column: u32,
        value: u32,
    ) -> u32 {
        let mut count = last - first;

        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if value < self.u32(Row::new(middle, table, file), column) {
                count = count2
            } else {
                first = middle + 1;
                count -= count2 + 1;
            }
        }

        first
    }

    fn equal_range_of(
        &self,
        table: TableIndex,
        file: u16,
        mut first: u32,
        mut last: u32,
        column: u32,
        value: u32,
    ) -> (u32, u32) {
        let mut count = last - first;
        loop {
            if count == 0 {
                last = first;
                break;
            }
            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = self.u32(Row::new(middle, table, file), column);
            match middle_value.cmp(&value) {
                Ordering::Less => {
                    first = middle + 1;
                    count -= count2 + 1;
                }
                Ordering::Greater => count = count2,
                Ordering::Equal => {
                    let first2 = self.lower_bound_of(table, file, first, middle, column, value);
                    first += count;
                    last = self.upper_bound_of(table, file, middle + 1, first, column, value);
                    first = first2;
                    break;
                }
            }
        }
        (first, last)
    }
}
