pub type BYTE = u8;
pub type DWORD = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_COR20_HEADER {
    pub cb: DWORD,
    pub MajorRuntimeVersion: WORD,
    pub MinorRuntimeVersion: WORD,
    pub MetaData: IMAGE_DATA_DIRECTORY,
    pub Flags: DWORD,
    pub Anonymous: IMAGE_COR20_HEADER_0,
    pub Resources: IMAGE_DATA_DIRECTORY,
    pub StrongNameSignature: IMAGE_DATA_DIRECTORY,
    pub CodeManagerTable: IMAGE_DATA_DIRECTORY,
    pub VTableFixups: IMAGE_DATA_DIRECTORY,
    pub ExportAddressTableJumps: IMAGE_DATA_DIRECTORY,
    pub ManagedNativeHeader: IMAGE_DATA_DIRECTORY,
}
impl Default for IMAGE_COR20_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_COR20_HEADER_0 {
    pub EntryPointToken: DWORD,
    pub EntryPointRVA: DWORD,
}
impl Default for IMAGE_COR20_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: DWORD,
    pub Size: DWORD,
}
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: i32 = 14;
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: i32 = 64;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: i32 = 1024;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: i32 = 256;
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: WORD,
    pub e_cblp: WORD,
    pub e_cp: WORD,
    pub e_crlc: WORD,
    pub e_cparhdr: WORD,
    pub e_minalloc: WORD,
    pub e_maxalloc: WORD,
    pub e_ss: WORD,
    pub e_sp: WORD,
    pub e_csum: WORD,
    pub e_ip: WORD,
    pub e_cs: WORD,
    pub e_lfarlc: WORD,
    pub e_ovno: WORD,
    pub e_res: [WORD; 4],
    pub e_oemid: WORD,
    pub e_oeminfo: WORD,
    pub e_res2: [WORD; 10],
    pub e_lfanew: LONG,
}
impl Default for IMAGE_DOS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_DOS_SIGNATURE: i32 = 23117;
pub const IMAGE_FILE_32BIT_MACHINE: i32 = 256;
pub const IMAGE_FILE_DLL: i32 = 8192;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: i32 = 2;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: WORD,
    pub NumberOfSections: WORD,
    pub TimeDateStamp: DWORD,
    pub PointerToSymbolTable: DWORD,
    pub NumberOfSymbols: DWORD,
    pub SizeOfOptionalHeader: WORD,
    pub Characteristics: WORD,
}
pub const IMAGE_FILE_MACHINE_I386: i32 = 332;
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: i32 = 267;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: i32 = 523;
pub const IMAGE_NT_SIGNATURE: i32 = 17744;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: WORD,
    pub MajorLinkerVersion: BYTE,
    pub MinorLinkerVersion: BYTE,
    pub SizeOfCode: DWORD,
    pub SizeOfInitializedData: DWORD,
    pub SizeOfUninitializedData: DWORD,
    pub AddressOfEntryPoint: DWORD,
    pub BaseOfCode: DWORD,
    pub BaseOfData: DWORD,
    pub ImageBase: DWORD,
    pub SectionAlignment: DWORD,
    pub FileAlignment: DWORD,
    pub MajorOperatingSystemVersion: WORD,
    pub MinorOperatingSystemVersion: WORD,
    pub MajorImageVersion: WORD,
    pub MinorImageVersion: WORD,
    pub MajorSubsystemVersion: WORD,
    pub MinorSubsystemVersion: WORD,
    pub Win32VersionValue: DWORD,
    pub SizeOfImage: DWORD,
    pub SizeOfHeaders: DWORD,
    pub CheckSum: DWORD,
    pub Subsystem: WORD,
    pub DllCharacteristics: WORD,
    pub SizeOfStackReserve: DWORD,
    pub SizeOfStackCommit: DWORD,
    pub SizeOfHeapReserve: DWORD,
    pub SizeOfHeapCommit: DWORD,
    pub LoaderFlags: DWORD,
    pub NumberOfRvaAndSizes: DWORD,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl Default for IMAGE_OPTIONAL_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: WORD,
    pub MajorLinkerVersion: BYTE,
    pub MinorLinkerVersion: BYTE,
    pub SizeOfCode: DWORD,
    pub SizeOfInitializedData: DWORD,
    pub SizeOfUninitializedData: DWORD,
    pub AddressOfEntryPoint: DWORD,
    pub BaseOfCode: DWORD,
    pub ImageBase: ULONGLONG,
    pub SectionAlignment: DWORD,
    pub FileAlignment: DWORD,
    pub MajorOperatingSystemVersion: WORD,
    pub MinorOperatingSystemVersion: WORD,
    pub MajorImageVersion: WORD,
    pub MinorImageVersion: WORD,
    pub MajorSubsystemVersion: WORD,
    pub MinorSubsystemVersion: WORD,
    pub Win32VersionValue: DWORD,
    pub SizeOfImage: DWORD,
    pub SizeOfHeaders: DWORD,
    pub CheckSum: DWORD,
    pub Subsystem: WORD,
    pub DllCharacteristics: WORD,
    pub SizeOfStackReserve: ULONGLONG,
    pub SizeOfStackCommit: ULONGLONG,
    pub SizeOfHeapReserve: ULONGLONG,
    pub SizeOfHeapCommit: ULONGLONG,
    pub LoaderFlags: DWORD,
    pub NumberOfRvaAndSizes: DWORD,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl Default for IMAGE_OPTIONAL_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [BYTE; 8],
    pub Misc: IMAGE_SECTION_HEADER_0,
    pub VirtualAddress: DWORD,
    pub SizeOfRawData: DWORD,
    pub PointerToRawData: DWORD,
    pub PointerToRelocations: DWORD,
    pub PointerToLinenumbers: DWORD,
    pub NumberOfRelocations: WORD,
    pub NumberOfLinenumbers: WORD,
    pub Characteristics: DWORD,
}
impl Default for IMAGE_SECTION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union IMAGE_SECTION_HEADER_0 {
    pub PhysicalAddress: DWORD,
    pub VirtualSize: DWORD,
}
impl Default for IMAGE_SECTION_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: i32 = 3;
pub type LONG = i32;
pub type ULONGLONG = u64;
pub type WORD = u16;
