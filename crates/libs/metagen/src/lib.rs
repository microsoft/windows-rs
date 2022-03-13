// Bootstrap the winmd generation here...
// https://en.wikipedia.org/wiki/Portable_Executable
// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format
// https://docs.microsoft.com/en-us/archive/msdn-magazine/2002/february/inside-windows-win32-portable-executable-file-format-in-detail
// https://github.com/dotnet/runtime/blob/94c0a7c13d158eb79d27223f474ec8f8db747f11/src/coreclr/dlls/mscorpe/pewriter.cpp

use std::mem::*;
use std::slice::*;

pub fn test() {
    let mut buffer = Vec::<u8>::new();
    let dos = ImageDosHeader::new();
    write(&mut buffer, &dos);
    let pe = ImageNtHeader::new();
    write(&mut buffer, &pe);
    let sections = ImageSectionHeader::new();
    write(&mut buffer, &sections);
    let cor = ImageCorHeader::new();
    write(&mut buffer, &cor);

    std::fs::write("/git/test.winmd", buffer).unwrap();

    assert!(size_of::<ImageDosHeader>() == 0x40);
    assert!(size_of::<ImageOptionalHeader>() == 0xE0);
}

fn write<T:Sized>(buffer: &mut Vec::<u8>, value: &T) {
    unsafe {
    buffer.extend_from_slice(from_raw_parts(value as *const _ as _,  size_of::<T>()));
    }
}

impl ImageDosHeader {
    fn new() -> Self {
        Self {
            magic: 0x5A4D, // MZ
            lfarlc: 0x40, // file address of relocation table
            lfanew: size_of::<Self>() as _, // file address of next header
            ..Default::default()
        }
    }
}

impl ImageNtHeader {
    fn new() -> Self {
        let mut new = Self {
            signature: 0x4550, // PE\0\0
            file_header: ImageFileHeader::new(),
            optional_header: ImageOptionalHeader::new(),
        };
        new.optional_header.data_directory[14].virtual_address = 0;
        new
    }
}

impl ImageFileHeader {
    fn new() -> Self {
        Self {
            machine: 0x14C, // x86
            number_of_sections: 2,
            size_of_optional_header: size_of::<ImageOptionalHeader>() as _,
            characteristics: 0x2102, // DLL
            ..Default::default()
        }
    }
}

impl ImageOptionalHeader {
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
            size_of_image: 0, // TODO: needs filling in
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

impl ImageSectionHeader {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl ImageCorHeader {
    fn new() -> Self {
        Self {
            cb : size_of::<Self>() as _,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Default)]
struct ImageDosHeader {
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

#[repr(C)]
#[derive(Default)]
struct ImageDataDirectory {
    virtual_address: u32,
    size: u32,
}

#[repr(C)]
#[derive(Default)]
struct ImageNtHeader {
    signature: u32,
    file_header: ImageFileHeader,
    optional_header: ImageOptionalHeader,
}

#[repr(C)]
#[derive(Default)]
struct ImageFileHeader {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

#[repr(C)]
#[derive(Default)]
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

#[repr(C)]
#[derive(Default)]
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

#[repr(C)]
#[derive(Default)]
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
