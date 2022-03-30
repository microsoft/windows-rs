#[derive(Default)]
pub struct File {
    name: String,
    bytes: Vec<u8>,
    strings: usize,
    blobs: usize,
    tables: [Table; TABLE_LEN],
}

#[derive(Default)]
struct Table {
    offset: usize,
    len: usize,
    width: usize,
    columns: [Column; 6]
}

#[derive(Default)]
struct Column {
    offset: usize,
    width: usize,
}

const TABLE_CONSTANT: usize = 0;
const TABLE_CUSTOMATTRIBUTE: usize = 1;
const TABLE_FIELD: usize = 2;
const TABLE_GENERICPARAM: usize = 3;
const TABLE_INTERFACEIMPL: usize = 4;
const TABLE_MEMBERREF: usize = 5;
const TABLE_METHODDEF: usize = 6;
const TABLE_PARAM: usize = 7;
const TABLE_TYPEDEF: usize = 8;
const TABLE_TYPEREF: usize = 9;
const TABLE_TYPESPEC: usize = 10;
const TABLE_IMPLMAP: usize = 11;
const TABLE_MODULEREF: usize = 12;
const TABLE_NESTEDCLASS: usize = 13;
const TABLE_MODULE: usize = 14;
const TABLE_ASSEMBLYREF: usize = 15;
const TABLE_CLASSLAYOUT: usize = 16;
const TABLE_LEN: usize = 17;

impl File {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let path = std::path::Path::new(path);
        let bytes = std::fs::read(&path)?;

        // TODO: share structs with writer!

        // TODO: now that we just have a large batch code generator, should we just load up all 
        // type information in memory and avoid hitting the files repeatedly? Could be faster...

        // Since the file was read successfully, we just assume it has a valid file name.
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        Ok(File { bytes, name, ..Default::default() })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Table {
    fn index_width(&self) -> usize {
        if self.len < (1 << 16) {
            2
        } else {
            4
        }
    }

    fn set_columns(&mut self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) {
        self.width = a + b + c + d + e + f;
        self.columns[0] = Column::new(0, a);
        if b != 0 {
            self.columns[1] = Column::new(a, b);
        }
        if c != 0 {
            self.columns[2] = Column::new(a + b, c);
        }
        if d != 0 {
            self.columns[3] = Column::new(a + b + c, d);
        }
        if e != 0 {
            self.columns[4] = Column::new(a + b + c + d, e);
        }
        if f != 0 {
            self.columns[5] = Column::new(a + b + c + d + e, f);
        }
    }

    fn set_data(&mut self, offset: &mut usize) {
        if self.len != 0 {
            let next = *offset + self.len * self.width;
            self.offset = *offset;
            *offset = next;
        }
    }
}

impl Column {
    fn new(    offset: usize,        width: usize) -> Self {
        Self{ offset, width }
    }
}