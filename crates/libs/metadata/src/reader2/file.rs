use super::*;
use windows_sys::{Win32::System::Diagnostics::Debug::*, Win32::System::SystemServices::*};

#[derive(Default)]
pub struct File {
    name: String,
    bytes: Vec<u8>,
    strings: usize,
    blobs: usize,
    pub(crate) tables: [Table; TABLE_LEN],
}

#[derive(Default)]
pub struct Table {
    pub offset: usize,
    pub len: usize,
    pub width: usize,
    pub columns: [Column; 6],
}

#[derive(Default)]
pub struct Column {
    pub offset: usize,
    pub width: usize,
}

pub const TABLE_CONSTANT: usize = 0;
pub const TABLE_CUSTOMATTRIBUTE: usize = 1;
pub const TABLE_FIELD: usize = 2;
pub const TABLE_GENERICPARAM: usize = 3;
pub const TABLE_INTERFACEIMPL: usize = 4;
pub const TABLE_MEMBERREF: usize = 5;
pub const TABLE_METHODDEF: usize = 6;
pub const TABLE_PARAM: usize = 7;
pub const TABLE_TYPEDEF: usize = 8;
pub const TABLE_TYPEREF: usize = 9;
pub const TABLE_TYPESPEC: usize = 10;
pub const TABLE_IMPLMAP: usize = 11;
pub const TABLE_MODULEREF: usize = 12;
pub const TABLE_NESTEDCLASS: usize = 13;
pub const TABLE_MODULE: usize = 14;
pub const TABLE_ASSEMBLYREF: usize = 15;
pub const TABLE_CLASSLAYOUT: usize = 16;
pub const TABLE_LEN: usize = 17;

fn error(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}

fn error_invalid_winmd() -> Error {
    error("File is not a valid `winmd` file")
}

impl File {
    pub fn new(path: &str) -> Result<Self> {
        let path = std::path::Path::new(path);
        let mut result = File::default();
        result.bytes = std::fs::read(&path)?;

        let dos = result.bytes.view_as::<IMAGE_DOS_HEADER>(0);

        if dos.e_magic != IMAGE_DOS_SIGNATURE as _ || result.bytes.copy_as::<u32>(dos.e_lfanew as _) != IMAGE_NT_SIGNATURE {
            return Err(error_invalid_winmd());
        }

        let file_offset = dos.e_lfanew as usize + size_of::<u32>();
        let file = result.bytes.view_as::<IMAGE_FILE_HEADER>(file_offset);

        let optional_offset = file_offset + size_of::<IMAGE_FILE_HEADER>();

        let (com_virtual_address, sections) = match result.bytes.copy_as::<u16>(optional_offset) {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
                let optional = result.bytes.view_as::<IMAGE_OPTIONAL_HEADER32>(optional_offset);
                (optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].VirtualAddress, result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(optional_offset + size_of::<IMAGE_OPTIONAL_HEADER32>(), file.NumberOfSections as usize))
            }
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                let optional = result.bytes.view_as::<IMAGE_OPTIONAL_HEADER64>(optional_offset);
                (optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].VirtualAddress, result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(optional_offset + size_of::<IMAGE_OPTIONAL_HEADER64>(), file.NumberOfSections as usize))
            }
            _ => return Err(error_invalid_winmd()),
        };

        let clr = result.bytes.view_as::<IMAGE_COR20_HEADER>(offset_from_rva(section_from_rva(sections, com_virtual_address)?, com_virtual_address) as _);

        if clr.cb != size_of::<IMAGE_COR20_HEADER>() as _ {
            return Err(error_invalid_winmd());
        }

        let metadata_offset = offset_from_rva(section_from_rva(sections, clr.MetaData.VirtualAddress)?, clr.MetaData.VirtualAddress);
        let metadata = result.bytes.view_as::<METADATA_HEADER>(metadata_offset as _);

        if metadata.signature != METADATA_SIGNATURE {
            return Err(error_invalid_winmd());
        }

        // The METADATA_HEADER struct is not a fixed size so have to offset a little more carefully.
        // TODO: check that this still applies... non-winrt winmd?
        let mut view = metadata_offset + metadata.length as usize + 20;
        let mut tables_data: (usize, usize) = (0, 0);

        for _ in 0..result.bytes.copy_as::<u16>(metadata_offset + metadata.length as usize + 18) {
            let stream_offset = result.bytes.copy_as::<u32>(view) as usize;
            let stream_len = result.bytes.copy_as::<u32>(view + 4) as usize;
            let stream_name = result.bytes.view_as_str(view + 8);
            match stream_name {
                b"#Strings" => result.strings = metadata_offset + stream_offset,
                b"#Blob" => result.blobs = metadata_offset + stream_offset,
                b"#~" => tables_data = (metadata_offset + stream_offset, stream_len),
                b"#GUID" => {}
                b"#US" => {}
                _ => unimplemented!(),
            }
            let mut padding = 4 - stream_name.len() % 4;
            if padding == 0 {
                padding = 4;
            }
            view += 8 + stream_name.len() + padding;
        }

        let heap_sizes = result.bytes.copy_as::<u8>(tables_data.0 + 6);
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if (heap_sizes >> 1 & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if (heap_sizes >> 2 & 1) == 1 { 4 } else { 2 };
        let valid_bits = result.bytes.copy_as::<u64>(tables_data.0 + 8);
        view = tables_data.0 + 24;

        // These tables are unused by the reader, but needed temporarily to calculate sizes and offsets for subsequent tables.
        let unused_empty = Table::default();
        let mut unused_assembly = Table::default();
        let mut unused_assembly_os = Table::default();
        let mut unused_assembly_processor = Table::default();
        let mut unused_assembly_ref_os = Table::default();
        let mut unused_assembly_ref_processor = Table::default();
        let mut unused_decl_security = Table::default();
        let mut unused_event = Table::default();
        let mut unused_event_map = Table::default();
        let mut unused_exported_type = Table::default();
        let mut unused_field_layout = Table::default();
        let mut unused_field_marshal = Table::default();
        let mut unused_field_rva = Table::default();
        let mut unused_file = Table::default();
        let mut unused_generic_param_constraint = Table::default();
        let mut unused_manifest_resource = Table::default();
        let mut unused_method_impl = Table::default();
        let mut unused_method_semantics = Table::default();
        let mut unused_method_spec = Table::default();
        let mut unused_property = Table::default();
        let mut unused_property_map = Table::default();
        let mut unused_standalone_sig = Table::default();

        for i in 0..64 {
            if (valid_bits >> i & 1) == 0 {
                continue;
            }

            let len = result.bytes.copy_as::<u32>(view) as _;
            view += 4;

            match i {
                0x00 => result.tables[TABLE_MODULE].len = len,
                0x01 => result.tables[TABLE_TYPEREF].len = len,
                0x02 => result.tables[TABLE_TYPEDEF].len = len,
                0x04 => result.tables[TABLE_FIELD].len = len,
                0x06 => result.tables[TABLE_METHODDEF].len = len,
                0x08 => result.tables[TABLE_PARAM].len = len,
                0x09 => result.tables[TABLE_INTERFACEIMPL].len = len,
                0x0a => result.tables[TABLE_MEMBERREF].len = len,
                0x0b => result.tables[TABLE_CONSTANT].len = len,
                0x0c => result.tables[TABLE_CUSTOMATTRIBUTE].len = len,
                0x0d => unused_field_marshal.len = len,
                0x0e => unused_decl_security.len = len,
                0x0f => result.tables[TABLE_CLASSLAYOUT].len = len,
                0x10 => unused_field_layout.len = len,
                0x11 => unused_standalone_sig.len = len,
                0x12 => unused_event_map.len = len,
                0x14 => unused_event.len = len,
                0x15 => unused_property_map.len = len,
                0x17 => unused_property.len = len,
                0x18 => unused_method_semantics.len = len,
                0x19 => unused_method_impl.len = len,
                0x1a => result.tables[TABLE_MODULEREF].len = len,
                0x1b => result.tables[TABLE_TYPESPEC].len = len,
                0x1c => result.tables[TABLE_IMPLMAP].len = len,
                0x1d => unused_field_rva.len = len,
                0x20 => unused_assembly.len = len,
                0x21 => unused_assembly_processor.len = len,
                0x22 => unused_assembly_os.len = len,
                0x23 => result.tables[TABLE_ASSEMBLYREF].len = len,
                0x24 => unused_assembly_ref_processor.len = len,
                0x25 => unused_assembly_ref_os.len = len,
                0x26 => unused_file.len = len,
                0x27 => unused_exported_type.len = len,
                0x28 => unused_manifest_resource.len = len,
                0x29 => result.tables[TABLE_NESTEDCLASS].len = len,
                0x2a => result.tables[TABLE_GENERICPARAM].len = len,
                0x2b => unused_method_spec.len = len,
                0x2c => unused_generic_param_constraint.len = len,
                _ => unreachable!(),
            };
        }

        let tables = &result.tables;
        let type_def_or_ref = composite_index_size(&[tables[TABLE_TYPEDEF].len, tables[TABLE_TYPEREF].len, tables[TABLE_TYPESPEC].len]);
        let has_constant = composite_index_size(&[tables[TABLE_FIELD].len, tables[TABLE_PARAM].len, unused_property.len]);
        let has_field_marshal = composite_index_size(&[tables[TABLE_FIELD].len, tables[TABLE_PARAM].len]);
        let has_decl_security = composite_index_size(&[tables[TABLE_TYPEDEF].len, tables[TABLE_METHODDEF].len, unused_assembly.len]);
        let member_ref_parent = composite_index_size(&[tables[TABLE_TYPEDEF].len, tables[TABLE_TYPEREF].len, tables[TABLE_MODULEREF].len, tables[TABLE_METHODDEF].len, tables[TABLE_TYPESPEC].len]);
        let has_semantics = composite_index_size(&[unused_event.len, unused_property.len]);
        let method_def_or_ref = composite_index_size(&[tables[TABLE_METHODDEF].len, tables[TABLE_MEMBERREF].len]);
        let member_forwarded = composite_index_size(&[tables[TABLE_FIELD].len, tables[TABLE_METHODDEF].len]);
        let implementation = composite_index_size(&[unused_file.len, tables[TABLE_ASSEMBLYREF].len, unused_exported_type.len]);
        let custom_attribute_type = composite_index_size(&[tables[TABLE_METHODDEF].len, tables[TABLE_MEMBERREF].len, unused_empty.len, unused_empty.len, unused_empty.len]);
        let resolution_scope = composite_index_size(&[tables[TABLE_MODULE].len, tables[TABLE_MODULEREF].len, tables[TABLE_ASSEMBLYREF].len, tables[TABLE_TYPEREF].len]);
        let type_or_method_def = composite_index_size(&[tables[TABLE_TYPEDEF].len, tables[TABLE_METHODDEF].len]);

        let has_custom_attribute = composite_index_size(&[
            tables[TABLE_METHODDEF].len,
            tables[TABLE_FIELD].len,
            tables[TABLE_TYPEREF].len,
            tables[TABLE_TYPEDEF].len,
            tables[TABLE_PARAM].len,
            tables[TABLE_INTERFACEIMPL].len,
            tables[TABLE_MEMBERREF].len,
            tables[TABLE_MODULE].len,
            unused_property.len,
            unused_event.len,
            unused_standalone_sig.len,
            tables[TABLE_MODULEREF].len,
            tables[TABLE_TYPESPEC].len,
            unused_assembly.len,
            tables[TABLE_ASSEMBLYREF].len,
            unused_file.len,
            unused_exported_type.len,
            unused_manifest_resource.len,
            tables[TABLE_GENERICPARAM].len,
            unused_generic_param_constraint.len,
            unused_method_spec.len,
        ]);

        unused_assembly.set_columns(4, 8, 4, blob_index_size, string_index_size, string_index_size);
        unused_assembly_os.set_columns(4, 4, 4, 0, 0, 0);
        unused_assembly_processor.set_columns(4, 0, 0, 0, 0, 0);
        result.tables[TABLE_ASSEMBLYREF].set_columns(8, 4, blob_index_size, string_index_size, string_index_size, blob_index_size);
        unused_assembly_ref_os.set_columns(4, 4, 4, result.tables[TABLE_ASSEMBLYREF].index_width(), 0, 0);
        unused_assembly_ref_processor.set_columns(4, result.tables[TABLE_ASSEMBLYREF].index_width(), 0, 0, 0, 0);
        result.tables[TABLE_CLASSLAYOUT].set_columns(2, 4, result.tables[TABLE_TYPEDEF].index_width(), 0, 0, 0);
        result.tables[TABLE_CONSTANT].set_columns(2, has_constant, blob_index_size, 0, 0, 0);
        result.tables[TABLE_CUSTOMATTRIBUTE].set_columns(has_custom_attribute, custom_attribute_type, blob_index_size, 0, 0, 0);
        unused_decl_security.set_columns(2, has_decl_security, blob_index_size, 0, 0, 0);
        unused_event_map.set_columns(result.tables[TABLE_TYPEDEF].index_width(), unused_event.index_width(), 0, 0, 0, 0);
        unused_event.set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        unused_exported_type.set_columns(4, 4, string_index_size, string_index_size, implementation, 0);
        result.tables[TABLE_FIELD].set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_field_layout.set_columns(4, result.tables[TABLE_FIELD].index_width(), 0, 0, 0, 0);
        unused_field_marshal.set_columns(has_field_marshal, blob_index_size, 0, 0, 0, 0);
        unused_field_rva.set_columns(4, result.tables[TABLE_FIELD].index_width(), 0, 0, 0, 0);
        unused_file.set_columns(4, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[TABLE_GENERICPARAM].set_columns(2, 2, type_or_method_def, string_index_size, 0, 0);
        unused_generic_param_constraint.set_columns(result.tables[TABLE_GENERICPARAM].index_width(), type_def_or_ref, 0, 0, 0, 0);
        result.tables[TABLE_IMPLMAP].set_columns(2, member_forwarded, string_index_size, result.tables[TABLE_MODULEREF].index_width(), 0, 0);
        result.tables[TABLE_INTERFACEIMPL].set_columns(result.tables[TABLE_TYPEDEF].index_width(), type_def_or_ref, 0, 0, 0, 0);
        unused_manifest_resource.set_columns(4, 4, string_index_size, implementation, 0, 0);
        result.tables[TABLE_MEMBERREF].set_columns(member_ref_parent, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[TABLE_METHODDEF].set_columns(4, 2, 2, string_index_size, blob_index_size, result.tables[TABLE_PARAM].index_width());
        unused_method_impl.set_columns(result.tables[TABLE_TYPEDEF].index_width(), method_def_or_ref, method_def_or_ref, 0, 0, 0);
        unused_method_semantics.set_columns(2, result.tables[TABLE_METHODDEF].index_width(), has_semantics, 0, 0, 0);
        unused_method_spec.set_columns(method_def_or_ref, blob_index_size, 0, 0, 0, 0);
        result.tables[TABLE_MODULE].set_columns(2, string_index_size, guid_index_size, guid_index_size, guid_index_size, 0);
        result.tables[TABLE_MODULEREF].set_columns(string_index_size, 0, 0, 0, 0, 0);
        result.tables[TABLE_NESTEDCLASS].set_columns(result.tables[TABLE_TYPEDEF].index_width(), result.tables[TABLE_TYPEDEF].index_width(), 0, 0, 0, 0);
        result.tables[TABLE_PARAM].set_columns(2, 2, string_index_size, 0, 0, 0);
        unused_property.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_property_map.set_columns(result.tables[TABLE_TYPEDEF].index_width(), unused_property.index_width(), 0, 0, 0, 0);
        unused_standalone_sig.set_columns(blob_index_size, 0, 0, 0, 0, 0);
        result.tables[TABLE_TYPEDEF].set_columns(4, string_index_size, string_index_size, type_def_or_ref, result.tables[TABLE_FIELD].index_width(), result.tables[TABLE_METHODDEF].index_width());
        result.tables[TABLE_TYPEREF].set_columns(resolution_scope, string_index_size, string_index_size, 0, 0, 0);
        result.tables[TABLE_TYPESPEC].set_columns(blob_index_size, 0, 0, 0, 0, 0);

        result.tables[TABLE_MODULE].set_data(&mut view);
        result.tables[TABLE_TYPEREF].set_data(&mut view);
        result.tables[TABLE_TYPEDEF].set_data(&mut view);
        result.tables[TABLE_FIELD].set_data(&mut view);
        result.tables[TABLE_METHODDEF].set_data(&mut view);
        result.tables[TABLE_PARAM].set_data(&mut view);
        result.tables[TABLE_INTERFACEIMPL].set_data(&mut view);
        result.tables[TABLE_MEMBERREF].set_data(&mut view);
        result.tables[TABLE_CONSTANT].set_data(&mut view);
        result.tables[TABLE_CUSTOMATTRIBUTE].set_data(&mut view);
        unused_field_marshal.set_data(&mut view);
        unused_decl_security.set_data(&mut view);
        result.tables[TABLE_CLASSLAYOUT].set_data(&mut view);
        unused_field_layout.set_data(&mut view);
        unused_standalone_sig.set_data(&mut view);
        unused_event_map.set_data(&mut view);
        unused_event.set_data(&mut view);
        unused_property_map.set_data(&mut view);
        unused_property.set_data(&mut view);
        unused_method_semantics.set_data(&mut view);
        unused_method_impl.set_data(&mut view);
        result.tables[TABLE_MODULEREF].set_data(&mut view);
        result.tables[TABLE_TYPESPEC].set_data(&mut view);
        result.tables[TABLE_IMPLMAP].set_data(&mut view);
        unused_field_rva.set_data(&mut view);
        unused_assembly.set_data(&mut view);
        unused_assembly_processor.set_data(&mut view);
        unused_assembly_os.set_data(&mut view);
        result.tables[TABLE_ASSEMBLYREF].set_data(&mut view);
        unused_assembly_ref_processor.set_data(&mut view);
        unused_assembly_ref_os.set_data(&mut view);
        unused_file.set_data(&mut view);
        unused_exported_type.set_data(&mut view);
        unused_manifest_resource.set_data(&mut view);
        result.tables[TABLE_NESTEDCLASS].set_data(&mut view);
        result.tables[TABLE_GENERICPARAM].set_data(&mut view);

        // Since the file was read successfully, we just assume it has a valid file name.
        result.name = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(result)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn usize(&self, row: usize, table: usize, column: usize) -> usize {
        let table = &self.tables[table];
        let column = &table.columns[column];
        let offset = table.offset + row * table.width + column.offset;
        match column.width {
            1 => self.bytes.copy_as::<u8>(offset) as _,
            2 => self.bytes.copy_as::<u16>(offset) as _,
            4 => self.bytes.copy_as::<u32>(offset) as _,
            _ => self.bytes.copy_as::<u64>(offset) as _,
        }
    }

    pub fn str(&self, row: usize, table: usize, column: usize) -> &str {
        let offset = self.strings + self.usize(row, table, column);

        unsafe {
            let len = strlen(self.bytes.as_ptr().add(offset));
            std::str::from_utf8_unchecked(&self.bytes[offset..offset + len])
        }
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

fn section_from_rva(sections: &[IMAGE_SECTION_HEADER], rva: u32) -> Result<&IMAGE_SECTION_HEADER> {
    sections.iter().find(|&s| rva >= s.VirtualAddress && rva < s.VirtualAddress + unsafe { s.Misc.VirtualSize }).ok_or_else(error_invalid_winmd)
}

fn offset_from_rva(section: &IMAGE_SECTION_HEADER, rva: u32) -> usize {
    (rva - section.VirtualAddress + section.PointerToRawData) as usize
}
