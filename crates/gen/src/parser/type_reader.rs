use super::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A reader of type information from Windows Metadata
pub struct TypeReader {
    // TODO: fields should be private
    /// The parsed Windows metadata files the [`TypeReader`] has access to
    pub(crate) files: Vec<File>,
    /// Types known to this [`TypeReader`]
    ///
    /// This is a mapping between namespace names and the types inside
    /// that namespace. The keys are the namespace and the values is a mapping
    /// of type names to type definitions
    types: BTreeMap<String, BTreeMap<String, TypeRow>>,

    nested: BTreeMap<Row, BTreeMap<String, Row>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum TypeRow {
    TypeDef(Row),
    Function(Row),
    Constant(Row),
}

impl TypeReader {
    pub fn get() -> &'static Self {
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
        let mut files: Vec<File> = files.into_iter().map(|file| File::new(file)).collect();

        if !files.iter().any(|file| file.name.starts_with("Windows.")) {
            files.push(File::from_bytes(
                "Windows.Win32.winmd".to_string(),
                include_bytes!("../../default/Windows.Win32.winmd").to_vec(),
            ));
            files.push(File::from_bytes(
                "Windows.WinRT.winmd".to_string(),
                include_bytes!("../../default/Windows.WinRT.winmd").to_vec(),
            ));
        }

        let reader = Self {
            files,
            types: BTreeMap::default(),
            nested: BTreeMap::default(),
        };

        let mut types = BTreeMap::<String, BTreeMap<String, TypeRow>>::default();
        let mut nested = BTreeMap::<Row, BTreeMap<String, Row>>::new();

        for (index, file) in reader.files.iter().enumerate() {
            let index = index as u16;
            let row_count = file.type_def_table().row_count;

            for row in 0..row_count {
                let def = Row::new(row, TableIndex::TypeDef, index);
                let namespace = reader.str(def, 2);
                let name = trim_tick(reader.str(def, 1));

                if namespace.is_empty() {
                    continue;
                }

                types
                    .entry(namespace.to_string())
                    .or_default()
                    .entry(name.to_string())
                    .or_insert_with(|| TypeRow::TypeDef(def));

                let flags = TypeFlags(reader.u32(def, 0));

                if flags.interface() || flags.windows_runtime() {
                    continue;
                }

                let extends = reader.u32(def, 3);

                if extends == 0 {
                    continue;
                }

                let extends = Row::new((extends >> 2) - 1, TableIndex::TypeRef, index);

                if (reader.str(extends, 2), reader.str(extends, 1)) != ("System", "Object") {
                    continue;
                }

                for field in reader.list(def, TableIndex::Field, 4) {
                    let name = reader.str(field, 1);

                    types
                        .entry(namespace.to_string())
                        .or_default()
                        .entry(name.to_string())
                        .or_insert_with(|| TypeRow::Constant(field));
                }

                for method in reader.list(def, TableIndex::MethodDef, 5) {
                    let name = reader.str(method, 3);

                    types
                        .entry(namespace.to_string())
                        .or_default()
                        .entry(name.to_string())
                        .or_insert_with(|| TypeRow::Function(method));
                }
            }

            let row_count = file.nested_class_table().row_count;

            for row in 0..row_count {
                let row = Row::new(row, TableIndex::NestedClass, index);
                let enclosed = Row::new(reader.u32(row, 0) - 1, TableIndex::TypeDef, index);
                let enclosing = Row::new(reader.u32(row, 1) - 1, TableIndex::TypeDef, index);
                let name = reader.str(enclosed, 1);

                nested
                    .entry(enclosing)
                    .or_default()
                    .insert(name.to_string(), enclosed);
            }
        }

        let exclude = &[
            ("Windows.Foundation", "HResult"),
            ("Windows.Win32.Com", "IUnknown"),
            ("Windows.Win32.Direct2D", "D2D_MATRIX_3X2_F"),
            ("Windows.Win32.SystemServices", "LARGE_INTEGER"),
            ("Windows.Win32.SystemServices", "ULARGE_INTEGER"),
            // TODO: remove once this is fixed: https://github.com/microsoft/win32metadata/issues/30
            ("Windows.Win32", "CFunctionDiscoveryNotificationWrapper"),
        ];

        for (namespace, name) in exclude {
            if let Some(value) = types.get_mut(*namespace) {
                value.remove(*name);
            }
        }

        Self {
            files: reader.files,
            types,
            nested,
        }
    }

    pub fn find_lowercase_namespace(&'static self, lowercase: &str) -> Option<&'static str> {
        self.types
            .keys()
            .find(|namespace| namespace.to_lowercase() == lowercase)
            .map(|namespace| namespace.as_str())
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
    ) -> impl Iterator<Item = ElementType> + '_ {
        self.types[namespace]
            .values()
            .filter_map(move |row| self.to_element_type(row))
    }

    // TODO: how to make this return an iterator?
    pub fn nested_types(&'static self, enclosing: &tables::TypeDef) -> Vec<tables::TypeDef> {
        self.nested
            .get(&enclosing.row)
            .iter()
            .flat_map(|t| t.values())
            .map(move |row| tables::TypeDef {
                reader: self,
                row: *row,
            })
            .collect()
    }

    pub fn resolve_type(&'static self, namespace: &str, name: &str) -> ElementType {
        if let Some(types) = self.types.get(namespace) {
            if let Some(row) = types.get(trim_tick(name)) {
                return self.to_element_type(row).unwrap();
            }
        }

        panic!("Could not find type `{}.{}`", namespace, name);
    }

    // TODO: this only returns Option<T> instead of just T because the TypeReader's cache still has constracts and attributes
    // that need to be excluded but are hard to do at that layer.
    fn to_element_type(&'static self, row: &TypeRow) -> Option<ElementType> {
        match row {
            TypeRow::TypeDef(row) => {
                let def = tables::TypeDef {
                    reader: self,
                    row: *row,
                };

                ElementType::from_type_def(def, Vec::new())
            }
            TypeRow::Function(row) => {
                Some(ElementType::Function(types::Function(tables::MethodDef {
                    reader: self,
                    row: *row,
                })))
            }
            TypeRow::Constant(row) => Some(ElementType::Constant(types::Constant(tables::Field {
                reader: self,
                row: *row,
            }))),
        }
    }

    pub fn resolve_type_def(&'static self, namespace: &str, name: &str) -> tables::TypeDef {
        if let Some(types) = self.types.get(namespace) {
            if let Some(TypeRow::TypeDef(row)) = types.get(trim_tick(name)) {
                return tables::TypeDef {
                    reader: self,
                    row: *row,
                };
            }
        }

        panic!("Could not find type def `{}.{}`", namespace, name);
    }

    pub fn resolve_type_ref(&'static self, type_ref: &tables::TypeRef) -> tables::TypeDef {
        if let ResolutionScope::TypeRef(scope) = type_ref.scope() {
            let row = if let Some(scope) = self.nested.get(&scope.resolve().row) {
                scope[type_ref.name()]
            } else {
                // TODO: workaround for https://github.com/microsoft/win32metadata/issues/127
                self.resolve_type_def("Windows.Win32.WindowsAccessibility", "IUIAutomation6").row
            };

            tables::TypeDef { reader: self, row }
        } else {
            self.resolve_type_def(type_ref.namespace(), type_ref.name())
        }
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
        let (blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            _ => unexpected!(),
        };
        let mut blob_size = blob_size as usize;
        for byte in &file.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + (*byte as usize);
        }
        Blob {
            reader: self,
            file_index: row.file_index,
            offset: offset + blob_size_bytes,
            size: blob_size,
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

    #[cfg(test)]
    pub fn get_class(namespace: &str, name: &str) -> types::Class {
        if let ElementType::Class(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[cfg(test)]
    pub fn get_struct(namespace: &str, name: &str) -> types::Struct {
        if let ElementType::Struct(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[cfg(test)]
    pub fn get_enum(namespace: &str, name: &str) -> types::Enum {
        if let ElementType::Enum(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[cfg(test)]
    pub fn get_interface(namespace: &str, name: &str) -> types::Interface {
        if let ElementType::Interface(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }
}

fn winmd_paths() -> Vec<std::path::PathBuf> {
    let mut windows_path = workspace_windows_dir();
    windows_path.push("winmd");

    let mut paths = vec![];
    push_winmd_paths(windows_path, &mut paths);
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

fn trim_tick(name: &str) -> &str {
    match name.as_bytes().get(name.len() - 2) {
        Some(c) if *c == b'`' => &name[..name.len() - 2],
        _ => name,
    }
}
