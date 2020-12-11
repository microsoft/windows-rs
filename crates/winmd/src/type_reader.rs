use crate::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A reader of type information from Windows Metadata
pub struct TypeReader {
    /// The parsed Windows metadata files the [`TypeReader`] has access to
    pub files: Vec<File>,
    /// Types known to this [`TypeReader`]
    ///
    /// This is a mapping between namespace names and the types inside
    /// that namespace. The keys are the namespace and the values is a mapping
    /// of type names to type definitions
    pub types: BTreeMap<String, BTreeMap<String, Row>>,
    // TODO: store Row objects and turn them into TypeDef on request.
    // When turning into TypeDef they add the &'static TypeReader
}

impl TypeReader {
    pub fn from_build() -> &'static Self {
        use std::{mem::MaybeUninit, sync::Once};
        static ONCE: Once = Once::new();
        static mut VALUE: MaybeUninit<TypeReader> = MaybeUninit::uninit();

        ONCE.call_once(|| {
            // This is safe because `Once` provides thread-safe one-time initialization
            unsafe { VALUE = MaybeUninit::new(Self::from_iter(winmd_paths())) }
        });

        // This is safe because `call_once` has already been called.
        unsafe { &*VALUE.as_ptr() }
    }

    /// Insert WinRT metadata at the given paths
    ///
    /// # Panics
    ///
    /// This function panics if the if the files where the windows metadata are stored cannot be read.
    fn from_iter<I: IntoIterator<Item = PathBuf>>(files: I) -> Self {
        let mut reader = Self {
            files: Vec::default(),
            types: BTreeMap::default(),
        };
        for (file_index, file) in files.into_iter().enumerate() {
            let file = File::new(file);
            reader.insert_file_at_index(file, file_index);
        }

        reader.remove_excluded_type(("Windows.Foundation", "HResult"));
        reader.remove_excluded_type(("Windows.Win32", "IUnknown"));

        // TODO: remove once this is fixed: https://github.com/microsoft/win32metadata/issues/30
        reader.remove_excluded_type(("Windows.Win32", "CFunctionDiscoveryNotificationWrapper"));

        reader
    }

    fn remove_excluded_type(&mut self, (namespace, type_name): (&str, &str)) {
        if let Some(value) = self.types.get_mut(namespace) {
            value.remove(type_name);
        }
    }

    fn insert_file_at_index(&mut self, file: File, file_index: usize) {
        let row_count = file.type_def_table().row_count;
        self.files.push(file);

        for row in 0..row_count {
            let row = Row::new(row, TableIndex::TypeDef, file_index as u16);
            let (namespace, name) = (self.str(row, 2), self.str(row, 1));
            let namespace = namespace.to_string();
            let name = name.to_string();

            self.types
                .entry(namespace)
                .or_default()
                .entry(name)
                .or_insert(row);
        }
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
    pub fn namespace_types(
        &'static self,
        namespace: &str,
    ) -> impl Iterator<Item = (&str, TypeDef)> {
        self.types[namespace].iter().map(move |(n, row)| {
            (
                n.as_str(),
                TypeDef {
                    reader: self,
                    row: *row,
                },
            )
        })
    }

    /// Resolve a type definition given its namespace and type name
    ///
    /// # Panics
    ///
    /// Panics if no type definition for the given namespace and type name can be found
    pub fn resolve_type_def(&'static self, (namespace, type_name): (&str, &str)) -> TypeDef {
        if let Some(types) = self.types.get(namespace) {
            if let Some(def) = types.get(type_name) {
                return TypeDef {
                    reader: self,
                    row: *def,
                };
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
    pub(crate) fn decode<T: Decode>(&'static self, row: Row, column: u32) -> T {
        T::decode(self, self.u32(row, column), row.file_index)
    }

    pub(crate) fn list(
        &self,
        row: Row,
        table: TableIndex,
        column: u32,
    ) -> impl Iterator<Item = Row> {
        let file = &self.files[row.file_index as usize];
        let first = self.u32(row, column) - 1;

        let last = if row.index + 1 < file.tables[row.table_index as usize].row_count {
            self.u32(row.next(), column) - 1
        } else {
            file.tables[table as usize].row_count
        };

        (first..last).map(move |value| Row::new(value, table, row.file_index))
    }

    /// Read a blob for a given row and column
    pub fn blob(&'static self, row: Row, column: u32) -> Blob {
        let file = &self.files[row.file_index as usize];
        let offset = (file.blobs + self.u32(row, column)) as usize;
        let initial_byte = file.bytes[offset];
        let (mut blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            _ => panic!("Invalid blob size"),
        };
        for byte in &file.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + byte;
        }
        Blob {
            reader: self,
            file_index: row.file_index,
            offset: offset + blob_size_bytes,
        }
    }

    pub(crate) fn equal_range(
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

    pub(crate) fn upper_bound(&self, file: u16, table: TableIndex, column: u32, value: u32) -> Row {
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

fn winmd_paths() -> Vec<std::path::PathBuf> {
    let mut windows_path = workspace_windows_dir();
    windows_path.push("winmd");

    let mut paths = vec![];
    push_winmd_paths(windows_path, &mut paths);

    // If at this point the paths vector is still empty then go and grab the winmd files from WinMetadata
    // to make it easy for developers to get started without having to figure out where to get metadata.

    if paths.is_empty() {
        if let Ok(dir) = std::env::var("windir") {
            let mut dir = std::path::PathBuf::from(dir);
            dir.push(SYSTEM32);
            dir.push("winmetadata");

            push_winmd_paths(dir, &mut paths);
        }
    }

    paths
}

fn push_winmd_paths(dir: std::path::PathBuf, paths: &mut Vec<std::path::PathBuf>) {
    if let Ok(files) = std::fs::read_dir(dir) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if let Some("winmd") = path.extension().and_then(|extension| extension.to_str())
                    {
                        paths.push(file.path());
                    }
                }
            }
        }
    }
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
