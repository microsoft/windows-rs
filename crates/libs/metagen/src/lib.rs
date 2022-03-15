// Bootstrap the winmd generation here...
// https://en.wikipedia.org/wiki/Portable_Executable
// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format
// https://docs.microsoft.com/en-us/archive/msdn-magazine/2002/february/inside-windows-win32-portable-executable-file-format-in-detail
// https://github.com/dotnet/runtime/blob/94c0a7c13d158eb79d27223f474ec8f8db747f11/src/coreclr/dlls/mscorpe/pewriter.cpp

use std::mem::*;
use std::slice::*;

pub fn test() {
    let mut buffer = Vec::<u8>::new();
    let dos = DosHeader::new();
    write(&mut buffer, &dos);
    let pe = PeHeader::new();
    write(&mut buffer, &pe);
    let optional = OptionalHeader::new();
    write(&mut buffer, &optional);
    let sections = SectionHeader::new();
    write(&mut buffer, &sections);
    let clr = ClrHeader::new();
    write(&mut buffer, &clr);

    std::fs::write("/git/test.winmd", buffer).unwrap();
}

fn write<T: Sized>(buffer: &mut Vec<u8>, value: &T) {
    unsafe {
        buffer.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
    }
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

impl PeHeader {
    fn new() -> Self {
        Self {
            signature: 0x4550, // PE\0\0
            machine: 0x14C,    // x86
            number_of_sections: 2,
            size_of_optional_header: size_of::<OptionalHeader>() as _,
            characteristics: 0x2102, // DLL
            ..Default::default()
        }
    }
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

impl SectionHeader {
    fn new() -> Self {
        Self { ..Default::default() }
    }
}

impl ClrHeader {
    fn new() -> Self {
        Self { cb: size_of::<Self>() as _, ..Default::default() }
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
