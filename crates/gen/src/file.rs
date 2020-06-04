#[derive(Default)]
pub struct TableData {
    pub data: u32,
    pub row_count: u32,
    pub row_size: u32,
    pub columns: [(u32, u32); 6],
}

#[derive(Default)]
pub struct WinmdFile {
    pub bytes: Vec<u8>,
    pub strings: u32,
    pub blobs: u32,
    pub guids: u32,
    pub tables: [TableData; 11],
}

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub enum TableIndex {
    Constant = 0,
    CustomAttribute,
    Field,
    GenericParam,
    InterfaceImpl,
    MemberRef,
    MethodDef,
    Param,
    TypeDef,
    TypeRef,
    TypeSpec,
}

impl TableData {
    fn index_size(&self) -> u32 {
        if self.row_count < (1 << 16) {
            2
        } else {
            4
        }
    }

    fn set_columns(&mut self, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) {
        self.row_size = a + b + c + d + e + f;
        self.columns[0] = (0, a);
        if b != 0 {
            self.columns[1] = ((a), b);
        }
        if c != 0 {
            self.columns[2] = ((a + b), c);
        }
        if d != 0 {
            self.columns[3] = ((a + b + c), d);
        }
        if e != 0 {
            self.columns[4] = ((a + b + c + d), e);
        }
        if f != 0 {
            self.columns[5] = ((a + b + c + d + e), f);
        }
    }

    fn set_data(&mut self, data: &mut u32) {
        if self.row_count != 0 {
            let next = *data + self.row_count * self.row_size;
            self.data = *data;
            *data = next;
        }
    }
}

impl WinmdFile {
    pub fn new<P: AsRef<std::path::Path>>(filename: P) -> Self {
        let bytes = std::fs::read(filename.as_ref())
            .unwrap_or_else(|e| panic!("Could not read file {:?}: {:?}", filename.as_ref(), e));
        let mut file = Self {
            bytes,
            ..Default::default()
        };
        let dos = file.bytes.view_as::<ImageDosHeader>(0);

        if dos.signature != IMAGE_DOS_SIGNATURE {
            panic!(
                "Invalid file: file does not appear to be a winmd file - '{:?}'",
                filename.as_ref()
            );
        }

        let pe = file.bytes.view_as::<ImageNtHeader>(dos.lfanew as u32);

        let (com_virtual_address, sections) = match pe.optional_header.magic {
            MAGIC_PE32 => (
                pe.optional_header.data_directory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize]
                    .virtual_address,
                file.bytes.view_as_slice_of::<ImageSectionHeader>(
                    dos.lfanew as u32 + sizeof::<ImageNtHeader>(),
                    pe.file_header.number_of_sections as u32,
                ),
            ),
            MAGIC_PE32PLUS => (
                file.bytes
                    .view_as::<ImageNtHeaderPlus>(dos.lfanew as u32)
                    .optional_header
                    .data_directory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize]
                    .virtual_address,
                file.bytes.view_as_slice_of::<ImageSectionHeader>(
                    dos.lfanew as u32 + sizeof::<ImageNtHeaderPlus>(),
                    pe.file_header.number_of_sections as u32,
                ),
            ),
            _ => panic!("Invalid file: invalid magic"),
        };

        let cli = file.bytes.view_as::<ImageCorHeader>(offset_from_rva(
            section_from_rva(sections, com_virtual_address),
            com_virtual_address,
        ));

        if cli.cb != sizeof::<ImageCorHeader>() {
            panic!("Invalid file: invalid ImageCorHeader");
        }

        let cli_offset = offset_from_rva(
            section_from_rva(sections, cli.meta_data.virtual_address),
            cli.meta_data.virtual_address,
        );

        if file.bytes.copy_as::<u32>(cli_offset) != STORAGE_MAGIC_SIG {
            panic!("Invalid file: invalid STORAGE_MAGIC_SIG");
        }

        let version_length = file.bytes.copy_as::<u32>(cli_offset + 12);
        let mut view = cli_offset + version_length + 20;
        let mut tables_data: (u32, u32) = (0, 0);

        for _ in 0..file.bytes.copy_as::<u16>(cli_offset + version_length + 18) {
            let stream_offset = file.bytes.copy_as::<u32>(view);
            let stream_size = file.bytes.copy_as::<u32>(view + 4);
            let stream_name = file.bytes.view_as_str(view + 8);
            match stream_name {
                b"#Strings" => file.strings = cli_offset + stream_offset,
                b"#Blob" => file.blobs = cli_offset + stream_offset,
                b"#GUID" => file.guids = cli_offset + stream_offset,
                b"#~" => tables_data = (cli_offset + stream_offset, stream_size),
                b"#US" => {}
                _ => panic!("Invalid file: invalid stream name"),
            }
            let mut padding = 4 - stream_name.len() % 4;
            if padding == 0 {
                padding = 4;
            }
            view += (8 + stream_name.len() + padding) as u32;
        }

        let heap_sizes = file.bytes.copy_as::<u8>(tables_data.0 + 6);
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if (heap_sizes >> 1 & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if (heap_sizes >> 2 & 1) == 1 { 4 } else { 2 };
        let valid_bits = file.bytes.copy_as::<u64>(tables_data.0 + 8);
        view = tables_data.0 + 24;

        // These tables are unused by WinRT, but needed temporarily to calculate sizes and offsets for subsequent tables.
        let unused_empty = TableData::default();
        let mut unused_assembly = TableData::default();
        let mut unused_assembly_os = TableData::default();
        let mut unused_assembly_processor = TableData::default();
        let mut unused_assembly_ref = TableData::default();
        let mut unused_assembly_ref_os = TableData::default();
        let mut unused_assembly_ref_processor = TableData::default();
        let mut unused_class_layout = TableData::default();
        let mut unused_decl_security = TableData::default();
        let mut unused_event = TableData::default();
        let mut unused_event_map = TableData::default();
        let mut unused_exported_type = TableData::default();
        let mut unused_field_layout = TableData::default();
        let mut unused_field_marshal = TableData::default();
        let mut unused_field_rva = TableData::default();
        let mut unused_file = TableData::default();
        let mut unused_generic_param_constraint = TableData::default();
        let mut unused_impl_map = TableData::default();
        let mut unused_manifest_resource = TableData::default();
        let mut unused_method_impl = TableData::default();
        let mut unused_method_semantics = TableData::default();
        let mut unused_method_spec = TableData::default();
        let mut unused_module = TableData::default();
        let mut unused_module_ref = TableData::default();
        let mut unused_nested_class = TableData::default();
        let mut unused_property = TableData::default();
        let mut unused_property_map = TableData::default();
        let mut unused_standalone_sig = TableData::default();

        for i in 0..64 {
            if (valid_bits >> i & 1) == 0 {
                continue;
            }

            let row_count = file.bytes.copy_as::<u32>(view);
            view += 4;

            match i {
                0x00 => unused_module.row_count = row_count,
                0x01 => file.tables[TableIndex::TypeRef as usize].row_count = row_count,
                0x02 => file.tables[TableIndex::TypeDef as usize].row_count = row_count,
                0x04 => file.tables[TableIndex::Field as usize].row_count = row_count,
                0x06 => file.tables[TableIndex::MethodDef as usize].row_count = row_count,
                0x08 => file.tables[TableIndex::Param as usize].row_count = row_count,
                0x09 => file.tables[TableIndex::InterfaceImpl as usize].row_count = row_count,
                0x0a => file.tables[TableIndex::MemberRef as usize].row_count = row_count,
                0x0b => file.tables[TableIndex::Constant as usize].row_count = row_count,
                0x0c => file.tables[TableIndex::CustomAttribute as usize].row_count = row_count,
                0x0d => unused_field_marshal.row_count = row_count,
                0x0e => unused_decl_security.row_count = row_count,
                0x0f => unused_class_layout.row_count = row_count,
                0x10 => unused_field_layout.row_count = row_count,
                0x11 => unused_standalone_sig.row_count = row_count,
                0x12 => unused_event_map.row_count = row_count,
                0x14 => unused_event.row_count = row_count,
                0x15 => unused_property_map.row_count = row_count,
                0x17 => unused_property.row_count = row_count,
                0x18 => unused_method_semantics.row_count = row_count,
                0x19 => unused_method_impl.row_count = row_count,
                0x1a => unused_module_ref.row_count = row_count,
                0x1b => file.tables[TableIndex::TypeSpec as usize].row_count = row_count,
                0x1c => unused_impl_map.row_count = row_count,
                0x1d => unused_field_rva.row_count = row_count,
                0x20 => unused_assembly.row_count = row_count,
                0x21 => unused_assembly_processor.row_count = row_count,
                0x22 => unused_assembly_os.row_count = row_count,
                0x23 => unused_assembly_ref.row_count = row_count,
                0x24 => unused_assembly_ref_processor.row_count = row_count,
                0x25 => unused_assembly_ref_os.row_count = row_count,
                0x26 => unused_file.row_count = row_count,
                0x27 => unused_exported_type.row_count = row_count,
                0x28 => unused_manifest_resource.row_count = row_count,
                0x29 => unused_nested_class.row_count = row_count,
                0x2a => file.tables[TableIndex::GenericParam as usize].row_count = row_count,
                0x2b => unused_method_spec.row_count = row_count,
                0x2c => unused_generic_param_constraint.row_count = row_count,
                _ => unreachable!(),
            };
        }

        let type_def_or_ref = composite_index_size(&[
            &file.tables[TableIndex::TypeDef as usize],
            &file.tables[TableIndex::TypeRef as usize],
            &file.tables[TableIndex::TypeSpec as usize],
        ]);

        let has_constant = composite_index_size(&[
            &file.tables[TableIndex::Field as usize],
            &file.tables[TableIndex::Param as usize],
            &unused_property,
        ]);

        let has_custom_attribute = composite_index_size(&[
            &file.tables[TableIndex::MethodDef as usize],
            &file.tables[TableIndex::Field as usize],
            &file.tables[TableIndex::TypeRef as usize],
            &file.tables[TableIndex::TypeDef as usize],
            &file.tables[TableIndex::Param as usize],
            &file.tables[TableIndex::InterfaceImpl as usize],
            &file.tables[TableIndex::MemberRef as usize],
            &unused_module,
            &unused_property,
            &unused_event,
            &unused_standalone_sig,
            &unused_module_ref,
            &file.tables[TableIndex::TypeSpec as usize],
            &unused_assembly,
            &unused_assembly_ref,
            &unused_file,
            &unused_exported_type,
            &unused_manifest_resource,
            &file.tables[TableIndex::GenericParam as usize],
            &unused_generic_param_constraint,
            &unused_method_spec,
        ]);

        let has_field_marshal = composite_index_size(&[
            &file.tables[TableIndex::Field as usize],
            &file.tables[TableIndex::Param as usize],
        ]);

        let has_decl_security = composite_index_size(&[
            &file.tables[TableIndex::TypeDef as usize],
            &file.tables[TableIndex::MethodDef as usize],
            &unused_assembly,
        ]);

        let member_ref_parent = composite_index_size(&[
            &file.tables[TableIndex::TypeDef as usize],
            &file.tables[TableIndex::TypeRef as usize],
            &unused_module_ref,
            &file.tables[TableIndex::MethodDef as usize],
            &file.tables[TableIndex::TypeSpec as usize],
        ]);

        let has_semantics = composite_index_size(&[&unused_event, &unused_property]);

        let method_def_or_ref = composite_index_size(&[
            &file.tables[TableIndex::MethodDef as usize],
            &file.tables[TableIndex::MemberRef as usize],
        ]);

        let member_forwarded = composite_index_size(&[
            &file.tables[TableIndex::Field as usize],
            &file.tables[TableIndex::MethodDef as usize],
        ]);

        let implementation =
            composite_index_size(&[&unused_file, &unused_assembly_ref, &unused_exported_type]);

        let custom_attribute_type = composite_index_size(&[
            &file.tables[TableIndex::MethodDef as usize],
            &file.tables[TableIndex::MemberRef as usize],
            &unused_empty,
            &unused_empty,
            &unused_empty,
        ]);

        let resolution_scope = composite_index_size(&[
            &unused_module,
            &unused_module_ref,
            &unused_assembly_ref,
            &file.tables[TableIndex::TypeRef as usize],
        ]);

        let type_or_method_def = composite_index_size(&[
            &file.tables[TableIndex::TypeDef as usize],
            &file.tables[TableIndex::MethodDef as usize],
        ]);

        unused_assembly.set_columns(
            4,
            8,
            4,
            blob_index_size,
            string_index_size,
            string_index_size,
        );
        unused_assembly_os.set_columns(4, 4, 4, 0, 0, 0);
        unused_assembly_processor.set_columns(4, 0, 0, 0, 0, 0);
        unused_assembly_ref.set_columns(
            8,
            4,
            blob_index_size,
            string_index_size,
            string_index_size,
            blob_index_size,
        );
        unused_assembly_ref_os.set_columns(4, 4, 4, unused_assembly_ref.index_size(), 0, 0);
        unused_assembly_ref_processor.set_columns(4, unused_assembly_ref.index_size(), 0, 0, 0, 0);
        unused_class_layout.set_columns(
            2,
            4,
            file.tables[TableIndex::TypeDef as usize].index_size(),
            0,
            0,
            0,
        );
        file.tables[TableIndex::Constant as usize].set_columns(
            2,
            has_constant,
            blob_index_size,
            0,
            0,
            0,
        );
        file.tables[TableIndex::CustomAttribute as usize].set_columns(
            has_custom_attribute,
            custom_attribute_type,
            blob_index_size,
            0,
            0,
            0,
        );
        unused_decl_security.set_columns(2, has_decl_security, blob_index_size, 0, 0, 0);
        unused_event_map.set_columns(
            file.tables[TableIndex::TypeDef as usize].index_size(),
            unused_event.index_size(),
            0,
            0,
            0,
            0,
        );
        unused_event.set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        unused_exported_type.set_columns(
            4,
            4,
            string_index_size,
            string_index_size,
            implementation,
            0,
        );
        file.tables[TableIndex::Field as usize].set_columns(
            2,
            string_index_size,
            blob_index_size,
            0,
            0,
            0,
        );
        unused_field_layout.set_columns(
            4,
            file.tables[TableIndex::Field as usize].index_size(),
            0,
            0,
            0,
            0,
        );
        unused_field_marshal.set_columns(has_field_marshal, blob_index_size, 0, 0, 0, 0);
        unused_field_rva.set_columns(
            4,
            file.tables[TableIndex::Field as usize].index_size(),
            0,
            0,
            0,
            0,
        );
        unused_file.set_columns(4, string_index_size, blob_index_size, 0, 0, 0);
        file.tables[TableIndex::GenericParam as usize].set_columns(
            2,
            2,
            type_or_method_def,
            string_index_size,
            0,
            0,
        );
        unused_generic_param_constraint.set_columns(
            file.tables[TableIndex::GenericParam as usize].index_size(),
            type_def_or_ref,
            0,
            0,
            0,
            0,
        );
        unused_impl_map.set_columns(
            2,
            member_forwarded,
            string_index_size,
            unused_module_ref.index_size(),
            0,
            0,
        );
        file.tables[TableIndex::InterfaceImpl as usize].set_columns(
            file.tables[TableIndex::TypeDef as usize].index_size(),
            type_def_or_ref,
            0,
            0,
            0,
            0,
        );
        unused_manifest_resource.set_columns(4, 4, string_index_size, implementation, 0, 0);
        file.tables[TableIndex::MemberRef as usize].set_columns(
            member_ref_parent,
            string_index_size,
            blob_index_size,
            0,
            0,
            0,
        );
        file.tables[TableIndex::MethodDef as usize].set_columns(
            4,
            2,
            2,
            string_index_size,
            blob_index_size,
            file.tables[TableIndex::Param as usize].index_size(),
        );
        unused_method_impl.set_columns(
            file.tables[TableIndex::TypeDef as usize].index_size(),
            method_def_or_ref,
            method_def_or_ref,
            0,
            0,
            0,
        );
        unused_method_semantics.set_columns(
            2,
            file.tables[TableIndex::MethodDef as usize].index_size(),
            has_semantics,
            0,
            0,
            0,
        );
        unused_method_spec.set_columns(method_def_or_ref, blob_index_size, 0, 0, 0, 0);
        unused_module.set_columns(
            2,
            string_index_size,
            guid_index_size,
            guid_index_size,
            guid_index_size,
            0,
        );
        unused_module_ref.set_columns(string_index_size, 0, 0, 0, 0, 0);
        unused_nested_class.set_columns(
            file.tables[TableIndex::TypeDef as usize].index_size(),
            file.tables[TableIndex::TypeDef as usize].index_size(),
            0,
            0,
            0,
            0,
        );
        file.tables[TableIndex::Param as usize].set_columns(2, 2, string_index_size, 0, 0, 0);
        unused_property.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_property_map.set_columns(
            file.tables[TableIndex::TypeDef as usize].index_size(),
            unused_property.index_size(),
            0,
            0,
            0,
            0,
        );
        unused_standalone_sig.set_columns(blob_index_size, 0, 0, 0, 0, 0);
        file.tables[TableIndex::TypeDef as usize].set_columns(
            4,
            string_index_size,
            string_index_size,
            type_def_or_ref,
            file.tables[TableIndex::Field as usize].index_size(),
            file.tables[TableIndex::MethodDef as usize].index_size(),
        );
        file.tables[TableIndex::TypeRef as usize].set_columns(
            resolution_scope,
            string_index_size,
            string_index_size,
            0,
            0,
            0,
        );
        file.tables[TableIndex::TypeSpec as usize].set_columns(blob_index_size, 0, 0, 0, 0, 0);

        unused_module.set_data(&mut view);
        file.tables[TableIndex::TypeRef as usize].set_data(&mut view);
        file.tables[TableIndex::TypeDef as usize].set_data(&mut view);
        file.tables[TableIndex::Field as usize].set_data(&mut view);
        file.tables[TableIndex::MethodDef as usize].set_data(&mut view);
        file.tables[TableIndex::Param as usize].set_data(&mut view);
        file.tables[TableIndex::InterfaceImpl as usize].set_data(&mut view);
        file.tables[TableIndex::MemberRef as usize].set_data(&mut view);
        file.tables[TableIndex::Constant as usize].set_data(&mut view);
        file.tables[TableIndex::CustomAttribute as usize].set_data(&mut view);
        unused_field_marshal.set_data(&mut view);
        unused_decl_security.set_data(&mut view);
        unused_class_layout.set_data(&mut view);
        unused_field_layout.set_data(&mut view);
        unused_standalone_sig.set_data(&mut view);
        unused_event_map.set_data(&mut view);
        unused_event.set_data(&mut view);
        unused_property_map.set_data(&mut view);
        unused_property.set_data(&mut view);
        unused_method_semantics.set_data(&mut view);
        unused_method_impl.set_data(&mut view);
        unused_module_ref.set_data(&mut view);
        file.tables[TableIndex::TypeSpec as usize].set_data(&mut view);
        unused_impl_map.set_data(&mut view);
        unused_field_rva.set_data(&mut view);
        unused_assembly.set_data(&mut view);
        unused_assembly_processor.set_data(&mut view);
        unused_assembly_os.set_data(&mut view);
        unused_assembly_ref.set_data(&mut view);
        unused_assembly_ref_processor.set_data(&mut view);
        unused_assembly_ref_os.set_data(&mut view);
        unused_file.set_data(&mut view);
        unused_exported_type.set_data(&mut view);
        unused_manifest_resource.set_data(&mut view);
        unused_nested_class.set_data(&mut view);
        file.tables[TableIndex::GenericParam as usize].set_data(&mut view);

        file
    }

    pub fn type_def_table(&self) -> &TableData {
        &self.tables[TableIndex::TypeDef as usize]
    }
}

fn section_from_rva(sections: &[ImageSectionHeader], rva: u32) -> &ImageSectionHeader {
    sections
        .iter()
        .find(|&s| {
            rva >= s.virtual_address && rva < s.virtual_address + s.physical_address_or_virtual_size
        })
        .expect("Invalid file")
}

fn offset_from_rva(section: &ImageSectionHeader, rva: u32) -> u32 {
    rva - section.virtual_address + section.pointer_to_raw_data
}

fn sizeof<T>() -> u32 {
    std::mem::size_of::<T>() as u32
}

fn composite_index_size(tables: &[&TableData]) -> u32 {
    fn small(row_count: u32, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        loop {
            value >>= 1;
            if value == 0 {
                break;
            }
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables
        .iter()
        .all(|table| small(table.row_count, bits_needed))
    {
        2
    } else {
        4
    }
}

pub(crate) trait View {
    fn view_as<T: Pod>(&self, cli_offset: u32) -> &T;
    fn view_as_slice_of<T: Pod>(&self, cli_offset: u32, len: u32) -> &[T];
    fn copy_as<T: Copy + CopyPod>(&self, cli_offset: u32) -> T;
    fn view_as_str(&self, cli_offset: u32) -> &[u8];
}

macro_rules! assert_proper_length {
    ($self:expr, $t:ty, $cli_offset:expr, $size:expr) => {
        let enough_room = $cli_offset + $size <= $self.len() as u32;
        assert!(
            enough_room,
            "Invalid file: not enough bytes at offset {} to represent T",
            $cli_offset
        );
    };
}

macro_rules! assert_proper_length_and_alignment {
    ($self:expr, $t:ty, $cli_offset:expr, $size:expr) => {{
        assert_proper_length!($self, $t, $cli_offset, $size);

        let ptr = &$self[$cli_offset as usize] as *const u8 as *const $t;

        let properly_aligned = ptr.align_offset(std::mem::align_of::<$t>()) == 0;

        assert!(
            properly_aligned,
            "Invalid file: offset {} is not properly aligned to T",
            $cli_offset
        );
        ptr
    }};
}

impl View for [u8] {
    fn view_as<T: Pod>(&self, cli_offset: u32) -> &T {
        let ptr = assert_proper_length_and_alignment!(self, T, cli_offset, sizeof::<T>());

        unsafe { &*ptr }
    }

    fn view_as_slice_of<T: Pod>(&self, cli_offset: u32, len: u32) -> &[T] {
        let ptr = assert_proper_length_and_alignment!(self, T, cli_offset, sizeof::<T>() * len);

        unsafe { std::slice::from_raw_parts(ptr, len as usize) }
    }

    fn copy_as<T: CopyPod>(&self, cli_offset: u32) -> T {
        assert_proper_length!(self, T, cli_offset, sizeof::<T>());

        unsafe {
            let mut data = std::mem::MaybeUninit::zeroed().assume_init();

            std::ptr::copy_nonoverlapping(
                self[cli_offset as usize..].as_ptr(),
                &mut data as *mut T as *mut u8,
                std::mem::size_of::<T>(),
            );

            data
        }
    }

    fn view_as_str(&self, cli_offset: u32) -> &[u8] {
        let buffer = &self[cli_offset as usize..];
        let index = buffer
            .iter()
            .position(|c| *c == b'\0')
            .expect("Invalid file");
        &self[cli_offset as usize..cli_offset as usize + index]
    }
}

const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
const MAGIC_PE32: u16 = 0x10B;
const MAGIC_PE32PLUS: u16 = 0x20B;
const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: u32 = 14;
const STORAGE_MAGIC_SIG: u32 = 0x424A_5342;

#[repr(C)]
struct ImageDosHeader {
    signature: u16,
    cblp: u16,
    cp: u16,
    crlc: u16,
    cparhdr: u16,
    minalloc: u16,
    maxalloc: u16,
    ss: u16,
    sp: u16,
    csum: u16,
    ip: u16,
    cs: u16,
    lfarlc: u16,
    ovno: u16,
    res: [u16; 4],
    oemid: u16,
    oeminfo: u16,
    res2: [u16; 10],
    lfanew: i32,
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageDosHeader {}

#[repr(C)]
struct ImageFileHeader {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageFileHeader {}

#[repr(C)]
struct ImageDataDirectory {
    virtual_address: u32,
    size: u32,
}

#[repr(C)]
struct ImageOptionalHeader {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: u32,
    image_base: u32,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u32,
    size_of_stack_commit: u32,
    size_of_heap_reserve: u32,
    size_of_heap_commit: u32,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: [ImageDataDirectory; 16],
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageOptionalHeader {}

#[repr(C)]
struct ImageNtHeader {
    signature: u32,
    file_header: ImageFileHeader,
    optional_header: ImageOptionalHeader,
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageNtHeader {}

#[repr(C)]
struct ImageOptionalHeaderPlus {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: [ImageDataDirectory; 16],
}

#[repr(C)]
struct ImageNtHeaderPlus {
    signature: u32,
    file_header: ImageFileHeader,
    optional_header: ImageOptionalHeaderPlus,
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageNtHeaderPlus {}

#[repr(C)]
struct ImageSectionHeader {
    name: [u8; 8],
    physical_address_or_virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    characteristics: u32,
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageSectionHeader {}

#[repr(C)]
struct ImageCorHeader {
    cb: u32,
    major_runtime_version: u16,
    minor_runtime_version: u16,
    meta_data: ImageDataDirectory,
    flags: u32,
    entry_point_token_or_entry_point_rva: u32,
    resources: ImageDataDirectory,
    strong_name_signature: ImageDataDirectory,
    code_manager_table: ImageDataDirectory,
    vtable_fixups: ImageDataDirectory,
    export_address_table_jumps: ImageDataDirectory,
    managed_native_header: ImageDataDirectory,
}
// Safety: this is safe because the type is #[repr(C)]
// and only contains data that is itself `Pod`
unsafe impl Pod for ImageCorHeader {}

/// A "Plain Ol' Data" structure that represents any type that
/// can be viewed from any probably aligned and sized buffer of
/// bytes in a well defined way. This means that the representation
/// of the type in memory must be stable and it must not contain
/// any invariant constraints.
///
/// ## Examples
///
/// A `u32` is a `Pod` because any [u8; 4] can be viewed as a `u32` safely.
/// A `bool` is _not_ a `Pod` because it must either be a `0` or `1` in memory
pub(crate) unsafe trait Pod {}

/// A Pod type that is also safe to copy.
///
/// In addition to the same safety properties as Pod types, this type must be able to
/// be zeroed, many that it is valid to represent this type in memory as all 0s.
pub(crate) unsafe trait CopyPod: Copy {}

unsafe impl CopyPod for u8 {}
unsafe impl CopyPod for u16 {}
unsafe impl CopyPod for u32 {}
unsafe impl CopyPod for u64 {}
unsafe impl CopyPod for i8 {}
unsafe impl CopyPod for i16 {}
unsafe impl CopyPod for i32 {}
unsafe impl CopyPod for i64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    struct Foo {
        bar: u16,
        baz: u8,
    }
    unsafe impl Pod for Foo {}

    #[test]
    fn view_bytes_as_type() {
        let bytes = [1u8, 3, 48, 90];

        let foo = bytes.view_as::<Foo>(0);
        assert_eq!(foo.bar, 0x0301);
        assert_eq!(foo.baz, 48)
    }

    #[test]
    #[should_panic]
    fn panic_on_unaligned_bytes() {
        let bytes = [1u8, 3, 48, 90, 90];

        let _ = bytes.view_as::<Foo>(0);
        let _ = bytes.view_as::<Foo>(1);
    }

    #[test]
    #[should_panic]
    fn panic_on_too_few_bytes() {
        let bytes = [1u8, 3];

        let _ = bytes.view_as::<Foo>(0);
    }

    #[test]
    fn copy_bytes_as_type() {
        let bytes = [1u8, 3];

        let foo = bytes.copy_as::<u16>(0);
        assert_eq!(foo, 0x0301);
    }
}
