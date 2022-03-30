use super::*;
use windows_sys::Win32::System::SystemServices::*;

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
    columns: [Column; 6],
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
        let mut file = File::default();
        file.bytes = std::fs::read(&path)?;

        let dos = file.bytes.view_as::<IMAGE_DOS_HEADER>(0);

        assert!(!dos.e_magic != IMAGE_DOS_SIGNATURE as _, "Invalid PE signature: file does not appear to be a winmd file");

        // TODO: share structs with writer!

        // Since the file was read successfully, we just assume it has a valid file name.
        file.name = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(file)
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
    fn new(offset: usize, width: usize) -> Self {
        Self { offset, width }
    }
}

macro_rules! assert_proper_length {
    ($self:expr, $t:ty, $offset:expr, $size:expr) => {
        let enough_room = $offset + $size <= $self.len();
        assert!(enough_room, "Invalid file: not enough bytes at offset {} to represent T", $offset);
    };
}

macro_rules! assert_proper_length_and_alignment {
    ($self:expr, $t:ty, $offset:expr, $size:expr) => {{
        assert_proper_length!($self, $t, $offset, $size);
        let ptr = &$self[$offset] as *const u8 as *const $t;
        let properly_aligned = ptr.align_offset(align_of::<$t>()) == 0;
        assert!(properly_aligned, "Invalid file: offset {} is not properly aligned to T", $offset);
        ptr
    }};
}

trait View {
    fn view_as<T>(&self, offset: usize) -> &T;
    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> &[T];
    fn copy_as<T: Copy>(&self, offset: usize) -> T;
    fn view_as_str(&self, offset: usize) -> &[u8];
}

impl View for [u8] {
    fn view_as<T>(&self, offset: usize) -> &T {
        let ptr = assert_proper_length_and_alignment!(self, T, offset, size_of::<T>());
        unsafe { &*ptr }
    }

    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> &[T] {
        let ptr = assert_proper_length_and_alignment!(self, T, offset, size_of::<T>() * len);
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    fn copy_as<T>(&self, offset: usize) -> T {
        assert_proper_length!(self, T, offset, size_of::<T>());
        unsafe {
            let mut data = MaybeUninit::zeroed().assume_init();
            copy_nonoverlapping(self[offset..].as_ptr(), &mut data as *mut T as *mut u8, size_of::<T>());
            data
        }
    }

    fn view_as_str(&self, offset: usize) -> &[u8] {
        let buffer = &self[offset..];
        let index = buffer.iter().position(|c| *c == b'\0').expect("Invalid file");
        &self[offset..offset + index]
    }
}
