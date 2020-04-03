use crate::*;

use std::cmp::Ordering;

pub struct Reader {
    pub files: Vec<WinmdFile>,
    pub types: BTreeMap<String, BTreeMap<String, TypeDef>>,
}

impl Reader {
    pub fn from_os() -> Self {
        let mut path = std::path::PathBuf::new();
        path.push(std::env::var("windir").unwrap());
        path.push(super::SYSTEM32);
        path.push("winmetadata");
        Self::from_dir(path)
    }

    pub fn from_dir<P: AsRef<std::path::Path>>(directory: P) -> Self {
        let files: Vec<String> = std::fs::read_dir(directory)
            .unwrap()
            .filter_map(|value| {
                value
                    .ok()
                    .map(|value| value.path().to_str().unwrap().to_string())
            })
            .collect();
        Self::from_files(&files)
    }

    pub fn from_files<'a, P: IntoIterator<Item = &'a String>>(filenames: P) -> Self {
        let mut reader = Reader {
            files: Default::default(),
            types: Default::default(),
        };

        reader.load(filenames);

        reader
    }

    fn load<'a, P: IntoIterator<Item = &'a String>>(&mut self, filenames: P) {
        debug_assert!(self.files.is_empty());

        for filename in filenames {
            self.files.push(WinmdFile::new(filename));
            let table = &self.files[self.files.len() - 1].type_def_table();

            for row in 0..table.row_count {
                let def = TypeDef(Row::new(
                    row,
                    TABLE_TYPEDEF as u16,
                    self.files.len() as u16 - 1,
                ));

                if def.ignore(self) {
                    continue;
                }

                let (namespace, name) = def.name(self);
                let namespace = namespace.to_string();
                let name = name.to_string();

                self.types
                    .entry(namespace)
                    .or_default()
                    .entry(name)
                    .or_insert(def);
            }
        }
    }

    pub fn namespaces(&self) -> Keys<String, BTreeMap<String, TypeDef>> {
        self.types.keys()
    }

    pub fn namespace_types(&self, namespace: &str) -> Values<String, TypeDef> {
        self.types[namespace].values()
    }

    pub fn resolve(&self, name: (&str, &str)) -> TypeDef {
        if let Some(types) = self.types.get(name.0) {
            if let Some(def) = types.get(name.1) {
                return *def;
            }
        }

        panic!("Could not find type `{}.{}`", name.0, name.1);
    }

    pub fn type_info(&self, def: TypeDef) -> Type {
        Type::from_type_def(self, def)
    }

    pub fn u32(&self, row: Row, column: u32) -> u32 {
        let file = &self.files[row.file as usize];
        let table = &file.tables[row.table as usize];
        let offset = table.data + row.row * table.row_size + table.columns[column as usize].0;
        match table.columns[column as usize].1 {
            1 => *(file.bytes.view_as::<u8>(offset)) as u32,
            2 => *(file.bytes.view_as::<u16>(offset)) as u32,
            4 => *(file.bytes.view_as::<u32>(offset)) as u32,
            _ => *(file.bytes.view_as::<u64>(offset)) as u32,
        }
    }

    pub fn str(&self, row: Row, column: u32) -> &str {
        let file = &self.files[row.file as usize];
        let offset = (file.strings + self.u32(row, column)) as usize;
        let last = file.bytes[offset..]
            .iter()
            .position(|c| *c == b'\0')
            .unwrap();
        std::str::from_utf8(&file.bytes[offset..offset + last]).unwrap()
    }

    pub fn decode<T: Decode>(&self, row: Row, column: u32) -> T {
        T::decode(self.u32(row, column), row.file)
    }

    pub fn equal_range(
        &self,
        file: u16,
        table: usize,
        column: u32,
        value: u32,
    ) -> impl Iterator<Item = Row> {
        let (first, last) = self.equal_range_of(
            table as u16,
            file,
            0,
            self.files[file as usize].tables[table].row_count,
            column,
            value,
        );

        (first..last).map(move |row| Row::new(row, table as u16, file))
    }

    pub fn list(&self, row: Row, table: u16, column: u32) -> impl Iterator<Item = Row> {
        let file = &self.files[row.file as usize];
        let first = self.u32(row, column) - 1;

        let last = if row.row + 1 < file.tables[row.table as usize].row_count {
            self.u32(row.next(), column) - 1
        } else {
            file.tables[table as usize].row_count
        };

        (first..last).map(move |value| Row::new(value, table as u16, row.file))
    }

    pub fn blob(&self, row: Row, column: u32) -> Blob {
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
        Blob::new(self, row.file, offset + blob_size_bytes)
    }

    fn lower_bound_of(
        &self,
        table: u16,
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

    pub fn upper_bound(&self, file: u16, table: u16, column: u32, value: u32) -> Row {
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
        table: u16,
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
        table: u16,
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
