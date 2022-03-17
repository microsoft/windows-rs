// Bootstrap the winmd generation here...
// https://en.wikipedia.org/wiki/Portable_Executable
// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format
// https://docs.microsoft.com/en-us/archive/msdn-magazine/2002/february/inside-windows-win32-portable-executable-file-format-in-detail
// https://github.com/dotnet/runtime/blob/94c0a7c13d158eb79d27223f474ec8f8db747f11/src/coreclr/dlls/mscorpe/pewriter.cpp

use std::collections::*;
use std::mem::*;
use std::slice::*;

pub fn test() {
    let dos = DosHeader::new();
    let pe = PeHeader::new();
    let mut optional = OptionalHeader::new();
    let mut section = SectionHeader::new();
    let mut clr = ClrHeader::new();
    let metadata = MetadataHeader::new();

    let strings = Strings::default();
    let blobs = Blobs::default();
    let tables = Tables::default();
    // TODO: populate metadata
    let mut strings = strings.buffer();
    let mut blobs = blobs.buffer();
    let mut tables = tables.buffer();

    // TODO: these should all be at fixed offsets so we could just rewrite them in the buffer later on
    // provided we keep track of the offsets.
    optional.size_of_image = 0xCCCC; // TODO: calc size

    optional.data_directory[14] = DataDirectory { virtual_address: 0x1000, size: size_of::<ClrHeader>() as _ };

    section.virtual_size = 0xCCCC; // TODO: calc size
    section.size_of_raw_data = 0xCCCC; // TODO: calc size
    section.pointer_to_raw_data = 0x200;

    clr.meta_data = DataDirectory { virtual_address: 0x1000 + size_of::<ClrHeader>() as u32, size: 0 };

    let mut buffer = Vec::<u8>::new();
    buffer.write(&dos);
    buffer.write(&pe);
    buffer.write(&optional);
    buffer.write(&section);
    debug_assert!(buffer.len() < 0x200);
    buffer.resize(0x200, 0);
    buffer.write(&clr);
    let metadata_offset = buffer.len();
    buffer.write(&metadata);

    let stream_offset = buffer.len() - metadata_offset 
        + size_of::<DataDirectory>() + 12 // #Strings
        + size_of::<DataDirectory>() + 8 // #Blob
        + size_of::<DataDirectory>() + 4; // #~

    // String stream header
    buffer.write(&DataDirectory { virtual_address: stream_offset as u32, size: strings.len() as _ });
    buffer.write(b"#Strings\0\0\0\0");

    // Blob stream header
    buffer.write(&DataDirectory { virtual_address: (stream_offset + strings.len()) as u32, size: blobs.len() as _ });
    buffer.write(b"#Blob\0\0\0");

    // Table stream header
    buffer.write(&DataDirectory { virtual_address: (stream_offset + strings.len() + blobs.len()) as u32, size: tables.len() as _ });
    buffer.write(b"#~\0\0");

    buffer.append(&mut strings);
    buffer.append(&mut blobs);
    buffer.append(&mut tables);

    buffer.resize(0xCF00, 0); // TODO: calc alignment?
    std::fs::write("/git/test.winmd", buffer).unwrap();
}

trait Write {
    fn write<T: Sized>(&mut self, value: &T);
}

impl Write for Vec<u8> {
    fn write<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
        }
    }
}

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
            lfarlc: 0x40,                   // file address of relocation table
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
            major_linker_version: 0xB,
            size_of_initialized_data: 0x400,
            image_base: 0x400000,
            section_alignment: 0x1000,
            file_alignment: 0x200,
            major_operating_system_version: 6,
            minor_operating_system_version: 2,
            major_subsystem_version: 6,
            minor_subsystem_version: 2,
            size_of_headers: 0x200,
            subsystem: 3, // console
            dll_characteristics: 0x540,
            size_of_stack_reserve: 0x100000,
            size_of_heap_reserve: 0x1000,
            loader_flags: 0x100000,
            number_of_rva_and_sizes: 0x10,
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
        Self { name: *b".text\0\0\0", characteristics: 0x4000_0020, virtual_address: 0x1000, ..Default::default() }
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
    fn new() -> Self {
        Self {
            signature: 0x424A_5342,
            major_version: 1,
            minor_version: 1,
            length: 20,
            version: *b"WindowsRuntime\0\0\0\0\0\0",
            streams: 3,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct Strings(BTreeSet<String>);

impl Strings {
    fn buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(0);
        buffer
    }
}

#[derive(Default)]
struct Blobs(BTreeSet<Vec<u8>>);

impl Blobs {
    fn buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(0);
        buffer
    }
}

#[derive(Default)]
struct Tables {
    // TODO: just use fixed 4 byte index sizes
}

impl Tables {
    fn buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = TableStreamHeader::new();
        buffer.write(&header);
        buffer
    }
}

#[repr(C)]
#[derive(Default)]
struct TableStreamHeader {
    reserved1: u32,
    major_version: u8,
    minor_version: u8,
    heap_sizes: u8,
    reserved2: u8,
    valid: u64,
    sorted: u64,
}

impl TableStreamHeader {
    fn new() -> Self {
        Self {
            major_version: 2,
            reserved2: 1,
            heap_sizes: 0b111,
            ..Default::default()
        }
    }
}
