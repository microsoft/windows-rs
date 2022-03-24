// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

use super::*;
use std::mem::*;

pub(crate) fn write(filename: &str, tables: Tables) {
    let dos = DosHeader::new();
    let pe = PeHeader::new();
    let mut optional = OptionalHeader::new();
    let mut section = SectionHeader::new();
    let mut clr = ClrHeader::new();
    let metadata = MetadataHeader::new(4);

    let mut strings = Strings::new();
    let mut blobs = Blobs::new();

    let mut tables = tables.into_stream(&mut strings, &mut blobs);
    let mut guids = guid_stream();
    let mut strings = strings.into_stream();
    let mut blobs = blobs.into_stream();

    type TablesHeader = StreamHeader<4>;
    type StringsHeader = StreamHeader<12>;
    type GuidsHeader = StreamHeader<8>;
    type BlobsHeader = StreamHeader<8>;

    let size_of_stream_headers = size_of::<TablesHeader>() + size_of::<StringsHeader>() + size_of::<GuidsHeader>() + size_of::<BlobsHeader>();
    let size_of_image = optional.file_alignment as usize + size_of::<ClrHeader>() + size_of::<MetadataHeader>() + size_of_stream_headers + guids.len() + strings.len() + blobs.len() + tables.len();

    optional.size_of_image = round(size_of_image, optional.section_alignment as _) as _;
    section.virtual_size = size_of_image as u32 - optional.file_alignment;
    section.size_of_raw_data = round(section.virtual_size as _, optional.file_alignment as _) as _;

    optional.data_directory[14] = DataDirectory { virtual_address: SECTION_ALIGNMENT, size: size_of::<ClrHeader>() as _ };
    section.pointer_to_raw_data = optional.file_alignment;
    clr.meta_data = DataDirectory { virtual_address: SECTION_ALIGNMENT + size_of::<ClrHeader>() as u32, size: section.virtual_size - size_of::<ClrHeader>() as u32 };

    let mut buffer = Vec::<u8>::new();
    buffer.write(&dos);
    buffer.write(&pe);
    buffer.write(&optional);
    buffer.write(&section);
    debug_assert!(buffer.len() < optional.file_alignment as _);
    buffer.resize(optional.file_alignment as _, 0);
    buffer.write(&clr);
    let metadata_offset = buffer.len();
    buffer.write(&metadata);

    let stream_offset = buffer.len() - metadata_offset + size_of_stream_headers;
    let tables_header = TablesHeader::new(stream_offset as _, tables.len() as _, b"#~\0\0");
    let strings_header = StringsHeader::new(tables_header.next_offset(), strings.len() as _, b"#Strings\0\0\0\0");
    let guids_header = GuidsHeader::new(strings_header.next_offset(), guids.len() as _, b"#GUID\0\0\0");
    let blobs_header = BlobsHeader::new(guids_header.next_offset(), blobs.len() as _, b"#Blob\0\0\0");

    buffer.write(&tables_header);
    buffer.write(&strings_header);
    buffer.write(&guids_header);
    buffer.write(&blobs_header);

    buffer.append(&mut tables);
    buffer.append(&mut strings);
    buffer.append(&mut guids);
    buffer.append(&mut blobs);

    assert_eq!(clr.meta_data.size as usize, buffer.len() - metadata_offset);
    assert_eq!(size_of_image, buffer.len());

    std::fs::write(filename, buffer).unwrap();
}

const SECTION_ALIGNMENT: u32 = 4096;

#[repr(C)]
#[derive(Default)]
struct DosHeader {
    magic: u16,
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

impl DosHeader {
    fn new() -> Self {
        Self {
            magic: 0x5A4D,                  // MZ
            lfarlc: 64,                     // file address of relocation table
            lfanew: size_of::<Self>() as _, // file address of next header
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Default)]
struct PeHeader {
    signature: u32,
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

impl PeHeader {
    fn new() -> Self {
        Self {
            signature: 0x4550, // PE\0\0
            machine: 0x14C,    // x86
            number_of_sections: 1,
            size_of_optional_header: size_of::<OptionalHeader>() as _,
            characteristics: 0x2102, // DLL
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Default)]
struct OptionalHeader {
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
    data_directory: [DataDirectory; 16],
}

impl OptionalHeader {
    fn new() -> Self {
        Self {
            magic: 0x10B, // PE32
            major_linker_version: 11,
            size_of_initialized_data: 1024,
            image_base: 0x400000,
            section_alignment: SECTION_ALIGNMENT,
            file_alignment: 512,
            major_operating_system_version: 6,
            minor_operating_system_version: 2,
            major_subsystem_version: 6,
            minor_subsystem_version: 2,
            size_of_headers: 512,
            subsystem: 3, // console
            dll_characteristics: 0x540,
            size_of_stack_reserve: 0x100000,
            size_of_heap_reserve: 4096,
            loader_flags: 0x100000,
            number_of_rva_and_sizes: 16,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Default)]
struct DataDirectory {
    virtual_address: u32,
    size: u32,
}

#[repr(C)]
#[derive(Default)]
struct SectionHeader {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    characteristics: u32,
}

impl SectionHeader {
    fn new() -> Self {
        Self { name: *b".text\0\0\0", characteristics: 0x4000_0020, virtual_address: SECTION_ALIGNMENT, ..Default::default() }
    }
}

#[repr(C)]
#[derive(Default)]
struct ClrHeader {
    cb: u32,
    major_runtime_version: u16,
    minor_runtime_version: u16,
    meta_data: DataDirectory,
    flags: u32,
    entry_point_token_or_entry_point_rva: u32,
    resources: DataDirectory,
    strong_name_signature: DataDirectory,
    code_manager_table: DataDirectory,
    vtable_fixups: DataDirectory,
    export_address_table_jumps: DataDirectory,
    managed_native_header: DataDirectory,
}

impl ClrHeader {
    fn new() -> Self {
        Self { cb: size_of::<Self>() as _, major_runtime_version: 2, minor_runtime_version: 5, flags: 0x1, ..Default::default() }
    }
}

#[repr(C)]
#[derive(Default)]
struct MetadataHeader {
    signature: u32,
    major_version: u16,
    minor_version: u16,
    reserved: u32,
    length: u32,
    version: [u8; 20],
    flags: u16,
    streams: u16,
}

impl MetadataHeader {
    fn new(streams: u16) -> Self {
        Self { signature: 0x424A_5342, major_version: 1, minor_version: 1, length: 20, version: *b"WindowsRuntime\0\0\0\0\0\0", streams, ..Default::default() }
    }
}

#[repr(C)]
struct StreamHeader<const LEN: usize> {
    offset: u32,
    size: u32,
    name: [u8; LEN],
}

impl<const LEN: usize> StreamHeader<LEN> {
    fn new(offset: u32, size: u32, name: &[u8; LEN]) -> Self {
        Self { offset, size, name: *name }
    }
    fn next_offset(&self) -> u32 {
        self.offset + self.size
    }
}

fn guid_stream() -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.resize(16, 0); // zero guid
    buffer
}
