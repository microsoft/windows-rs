use crate::*;

pub struct Row {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

pub struct Reader {
    files: Vec<File>,
    types: std::collections::BTreeMap<String, Vec<Row>>,
}

impl<'a> Reader {
    pub fn from_files<P: IntoIterator<Item = &'a String>>(filenames: P) -> Result<Self, Error> {
        let mut reader = Reader { files: Vec::new(), types: Default::default(), };

        for filename in filenames {
            let file = File::new(filename)?;
            let table = &file.tables[TABLE_TYPE_DEF];
            
            for row in 0..table.row_count {
                if TypeAttributes(table.u32(&file, row, 0)).windows_runtime() {
                    let namespace = table.str(&file, row, 2).to_string();
                    reader.types.entry(namespace).or_insert_with(||Default::default()).push(Row {row: row, table:TABLE_TYPE_DEF as u16, file: reader.files.len() as u16 - 1});
                }
            }
        }

        Ok(reader)
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
}


#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
