use super::*;
use windows_sys::{Win32::System::Diagnostics::Debug::*, Win32::System::SystemServices::*};

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

fn error(message: &str) -> Result<File> {
    Err(Error::new(ErrorKind::Other, message))
}

fn error_invalid_winmd() -> Result<File> {
    error("File is not a valid `winmd` file")
}

impl File {
    pub fn new(path: &str) -> Result<Self> {
        let path = std::path::Path::new(path);
        let mut result = File::default();
        result.bytes = std::fs::read(&path)?;

        let dos = result.bytes.view_as::<IMAGE_DOS_HEADER>(0);

        if dos.e_magic != IMAGE_DOS_SIGNATURE as _ || 
        result.bytes.copy_as::<u32>(dos.e_lfanew as _) != IMAGE_NT_SIGNATURE {
            return error_invalid_winmd();
        }

        let file_offset = dos.e_lfanew as usize + size_of::<u32>();
        let file = result.bytes.view_as::<IMAGE_FILE_HEADER>(file_offset);

        let optional_offset = file_offset + size_of::<IMAGE_FILE_HEADER>();

        let (com_virtual_address, sections) = match result.bytes.copy_as::<u16>(optional_offset) {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
                let optional = result.bytes.view_as::< IMAGE_OPTIONAL_HEADER32 >(optional_offset);   
                (optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].VirtualAddress, result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(optional_offset +  size_of::<IMAGE_OPTIONAL_HEADER32>(), file.NumberOfSections as usize))
            }
            IMAGE_NT_OPTIONAL_HDR64_MAGIC =>{
                let optional = result.bytes.view_as::< IMAGE_OPTIONAL_HEADER64>(optional_offset);   
                 (optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].VirtualAddress, result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(optional_offset + size_of::<IMAGE_OPTIONAL_HEADER64>(), file.NumberOfSections as usize))
            }
            _ => return error_invalid_winmd(),
        };

        let cli = result.bytes.view_as::<IMAGE_COR20_HEADER>(offset_from_rva(section_from_rva(sections, com_virtual_address), com_virtual_address));

        // if cli.cb != size_of::<ImageCorHeader>() {
        //     unimplemented!();
        // }

        // let cli_offset = offset_from_rva(section_from_rva(sections, cli.meta_data.virtual_address), cli.meta_data.virtual_address);

        // if result.bytes.copy_as::<u32>(cli_offset) != STORAGE_MAGIC_SIG {
        //     unimplemented!();
        // }

        // Since the file was read successfully, we just assume it has a valid file name.
        result.name = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(result)
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
