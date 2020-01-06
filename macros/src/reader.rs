use crate::*;

pub struct Row {
    pub index: u32,
    pub file: u32,
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

            for row in 0..file.type_def.row_count {
                if TypeAttributes(file.type_def.u32(&file, row, 0)).windows_runtime() {
                    let type_name = file.type_def.str(&file, row, 1);
                    let type_namespace = file.type_def.str(&file, row, 2);

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
