pub const ADDRESS_TYPE_INDEX_NOT_FOUND: u32 = 11;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BUSDATA {
    pub BusDataType: u32,
    pub BusNumber: u32,
    pub SlotNumber: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
impl Default for BUSDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CANNOT_ALLOCATE_MEMORY: u32 = 9;
pub const CROSS_PLATFORM_MAXIMUM_PROCESSORS: u32 = 2048;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CURRENT_KD_SECONDARY_VERSION: u32 = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const CURRENT_KD_SECONDARY_VERSION: u32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBGKD_DEBUG_DATA_HEADER32 {
    pub List: super::winnt::LIST_ENTRY32,
    pub OwnerTag: u32,
    pub Size: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBGKD_DEBUG_DATA_HEADER64 {
    pub List: super::winnt::LIST_ENTRY64,
    pub OwnerTag: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBGKD_GET_VERSION32 {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub ProtocolVersion: u16,
    pub Flags: u16,
    pub KernBase: u32,
    pub PsLoadedModuleList: u32,
    pub MachineType: u16,
    pub ThCallbackStack: u16,
    pub NextCallback: u16,
    pub FramePointer: u16,
    pub KiCallUserMode: u32,
    pub KeUserCallbackDispatcher: u32,
    pub BreakpointWithStatus: u32,
    pub DebuggerDataList: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBGKD_GET_VERSION64 {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub ProtocolVersion: u8,
    pub KdSecondaryVersion: u8,
    pub Flags: u16,
    pub MachineType: u16,
    pub MaxPacketType: u8,
    pub MaxStateChange: u8,
    pub MaxManipulate: u8,
    pub Simulation: u8,
    pub Unused: [u16; 1],
    pub KernBase: u64,
    pub PsLoadedModuleList: u64,
    pub DebuggerDataList: u64,
}
impl Default for DBGKD_GET_VERSION64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DBGKD_MAJOR_BIG: DBGKD_MAJOR_TYPES = 2;
pub const DBGKD_MAJOR_CE: DBGKD_MAJOR_TYPES = 10;
pub const DBGKD_MAJOR_COUNT: DBGKD_MAJOR_TYPES = 11;
pub const DBGKD_MAJOR_EFI: DBGKD_MAJOR_TYPES = 5;
pub const DBGKD_MAJOR_EXDI: DBGKD_MAJOR_TYPES = 3;
pub const DBGKD_MAJOR_HYPERVISOR: DBGKD_MAJOR_TYPES = 8;
pub const DBGKD_MAJOR_MIDORI: DBGKD_MAJOR_TYPES = 9;
pub const DBGKD_MAJOR_NT: DBGKD_MAJOR_TYPES = 0;
pub const DBGKD_MAJOR_NTBD: DBGKD_MAJOR_TYPES = 4;
pub const DBGKD_MAJOR_SINGULARITY: DBGKD_MAJOR_TYPES = 7;
pub const DBGKD_MAJOR_TNT: DBGKD_MAJOR_TYPES = 6;
pub type DBGKD_MAJOR_TYPES = i32;
pub const DBGKD_MAJOR_XBOX: DBGKD_MAJOR_TYPES = 1;
pub const DBGKD_SIMULATION_EXDI: i32 = 1;
pub const DBGKD_SIMULATION_NONE: i32 = 0;
pub const DBGKD_VERS_FLAG_DATA: u32 = 2;
pub const DBGKD_VERS_FLAG_HAL_IN_NTOS: u32 = 64;
pub const DBGKD_VERS_FLAG_HSS: u32 = 16;
pub const DBGKD_VERS_FLAG_MP: u32 = 1;
pub const DBGKD_VERS_FLAG_NOMM: u32 = 8;
pub const DBGKD_VERS_FLAG_PARTITIONS: u32 = 32;
pub const DBGKD_VERS_FLAG_PTR64: u32 = 4;
pub const DBG_DUMP_ADDRESS_AT_END: u32 = 131072;
pub const DBG_DUMP_ADDRESS_OF_FIELD: u32 = 65536;
pub const DBG_DUMP_ARRAY: u32 = 32768;
pub const DBG_DUMP_BLOCK_RECURSE: u32 = 2097152;
pub const DBG_DUMP_CALL_FOR_EACH: u32 = 8;
pub const DBG_DUMP_COMPACT_OUT: u32 = 8192;
pub const DBG_DUMP_COPY_TYPE_DATA: u32 = 262144;
pub const DBG_DUMP_FIELD_ARRAY: u32 = 16;
pub const DBG_DUMP_FIELD_CALL_BEFORE_PRINT: u32 = 1;
pub const DBG_DUMP_FIELD_COPY_FIELD_DATA: u32 = 32;
pub const DBG_DUMP_FIELD_DEFAULT_STRING: u32 = 65536;
pub const DBG_DUMP_FIELD_FULL_NAME: u32 = 8;
pub const DBG_DUMP_FIELD_GUID_STRING: u32 = 524288;
pub const DBG_DUMP_FIELD_MULTI_STRING: u32 = 262144;
pub const DBG_DUMP_FIELD_NO_CALLBACK_REQ: u32 = 2;
pub const DBG_DUMP_FIELD_NO_PRINT: u32 = 16384;
pub const DBG_DUMP_FIELD_RECUR_ON_THIS: u32 = 4;
pub const DBG_DUMP_FIELD_RETURN_ADDRESS: u32 = 4096;
pub const DBG_DUMP_FIELD_SIZE_IN_BITS: u32 = 8192;
pub const DBG_DUMP_FIELD_UTF32_STRING: u32 = 1048576;
pub const DBG_DUMP_FIELD_WCHAR_STRING: u32 = 131072;
pub const DBG_DUMP_FUNCTION_FORMAT: u32 = 1048576;
pub const DBG_DUMP_GET_SIZE_ONLY: u32 = 128;
pub const DBG_DUMP_LIST: u32 = 32;
pub const DBG_DUMP_MATCH_SIZE: u32 = 4194304;
pub const DBG_DUMP_NO_INDENT: u32 = 1;
pub const DBG_DUMP_NO_OFFSET: u32 = 2;
pub const DBG_DUMP_NO_PRINT: u32 = 64;
pub const DBG_DUMP_READ_PHYSICAL: u32 = 524288;
pub const DBG_DUMP_VERBOSE: u32 = 4;
pub const DBG_RETURN_SUBTYPES: u32 = 0;
pub const DBG_RETURN_TYPE: u32 = 0;
pub const DBG_RETURN_TYPE_VALUES: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEBUG_TYPED_DATA {
    pub ModBase: u64,
    pub Offset: u64,
    pub EngineHandle: u64,
    pub Data: u64,
    pub Size: u32,
    pub Flags: u32,
    pub TypeId: u32,
    pub BaseTypeId: u32,
    pub Tag: u32,
    pub Register: u32,
    pub Internal: [u64; 9],
}
impl Default for DEBUG_TYPED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_TYPED_DATA_IS_IN_MEMORY: u32 = 1;
pub const DEBUG_TYPED_DATA_PHYSICAL_CACHED: u32 = 4;
pub const DEBUG_TYPED_DATA_PHYSICAL_DEFAULT: u32 = 2;
pub const DEBUG_TYPED_DATA_PHYSICAL_MEMORY: u32 = 14;
pub const DEBUG_TYPED_DATA_PHYSICAL_UNCACHED: u32 = 6;
pub const DEBUG_TYPED_DATA_PHYSICAL_WRITE_COMBINED: u32 = 8;
pub const DbgkdBlockSize: POOL_HEADER_FIELD_NAME = 2;
pub const DbgkdPoolIndex: POOL_HEADER_FIELD_NAME = 1;
pub const DbgkdPoolType: POOL_HEADER_FIELD_NAME = 3;
pub const DbgkdPreviousSize: POOL_HEADER_FIELD_NAME = 0;
pub const DbgkdUlong1: POOL_HEADER_FIELD_NAME = 4;
pub const EXIT_ON_CONTROLC: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTSTACKTRACE {
    pub FramePointer: u32,
    pub ProgramCounter: u32,
    pub ReturnAddress: u32,
    pub Args: [u32; 4],
}
impl Default for EXTSTACKTRACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTSTACKTRACE32 {
    pub FramePointer: u32,
    pub ProgramCounter: u32,
    pub ReturnAddress: u32,
    pub Args: [u32; 4],
}
impl Default for EXTSTACKTRACE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTSTACKTRACE64 {
    pub FramePointer: u64,
    pub ProgramCounter: u64,
    pub ReturnAddress: u64,
    pub Args: [u64; 4],
}
impl Default for EXTSTACKTRACE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXT_API_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Revision: u16,
    pub Reserved: u16,
}
pub const EXT_API_VERSION_NUMBER: u32 = 5;
pub const EXT_API_VERSION_NUMBER32: u32 = 5;
pub const EXT_API_VERSION_NUMBER64: u32 = 6;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXT_FIND_FILE {
    pub FileName: windows_core::PCWSTR,
    pub IndexedSize: u64,
    pub ImageTimeDateStamp: u32,
    pub ImageCheckSum: u32,
    pub ExtraInfo: *mut core::ffi::c_void,
    pub ExtraInfoSize: u32,
    pub Flags: u32,
    pub FileMapping: *mut core::ffi::c_void,
    pub FileMappingSize: u64,
    pub FileHandle: super::winnt::HANDLE,
    pub FoundFileName: windows_core::PWSTR,
    pub FoundFileNameChars: u32,
}
#[cfg(feature = "winnt")]
impl Default for EXT_FIND_FILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXT_FIND_FILE_ALLOW_GIVEN_PATH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXT_MATCH_PATTERN_A {
    pub Str: windows_core::PCSTR,
    pub Pattern: windows_core::PCSTR,
    pub CaseSensitive: u32,
}
pub const EXT_TDF_PHYSICAL_CACHED: u32 = 4;
pub const EXT_TDF_PHYSICAL_DEFAULT: u32 = 2;
pub const EXT_TDF_PHYSICAL_MEMORY: u32 = 14;
pub const EXT_TDF_PHYSICAL_UNCACHED: u32 = 6;
pub const EXT_TDF_PHYSICAL_WRITE_COMBINED: u32 = 8;
pub type EXT_TDOP = i32;
pub const EXT_TDOP_COPY: EXT_TDOP = 0;
pub const EXT_TDOP_COUNT: EXT_TDOP = 19;
pub const EXT_TDOP_EVALUATE: EXT_TDOP = 5;
pub const EXT_TDOP_GET_ARRAY_ELEMENT: EXT_TDOP = 12;
pub const EXT_TDOP_GET_DEREFERENCE: EXT_TDOP = 13;
pub const EXT_TDOP_GET_FIELD: EXT_TDOP = 4;
pub const EXT_TDOP_GET_FIELD_OFFSET: EXT_TDOP = 11;
pub const EXT_TDOP_GET_POINTER_TO: EXT_TDOP = 16;
pub const EXT_TDOP_GET_TYPE_NAME: EXT_TDOP = 6;
pub const EXT_TDOP_GET_TYPE_SIZE: EXT_TDOP = 14;
pub const EXT_TDOP_HAS_FIELD: EXT_TDOP = 10;
pub const EXT_TDOP_OUTPUT_FULL_VALUE: EXT_TDOP = 9;
pub const EXT_TDOP_OUTPUT_SIMPLE_VALUE: EXT_TDOP = 8;
pub const EXT_TDOP_OUTPUT_TYPE_DEFINITION: EXT_TDOP = 15;
pub const EXT_TDOP_OUTPUT_TYPE_NAME: EXT_TDOP = 7;
pub const EXT_TDOP_RELEASE: EXT_TDOP = 1;
pub const EXT_TDOP_SET_FROM_EXPR: EXT_TDOP = 2;
pub const EXT_TDOP_SET_FROM_TYPE_ID_AND_U64: EXT_TDOP = 17;
pub const EXT_TDOP_SET_FROM_U64_EXPR: EXT_TDOP = 3;
pub const EXT_TDOP_SET_PTR_FROM_TYPE_ID_AND_U64: EXT_TDOP = 18;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXT_TYPED_DATA {
    pub Operation: EXT_TDOP,
    pub Flags: u32,
    pub InData: DEBUG_TYPED_DATA,
    pub OutData: DEBUG_TYPED_DATA,
    pub InStrIndex: u32,
    pub In32: u32,
    pub Out32: u32,
    pub In64: u64,
    pub Out64: u64,
    pub StrBufferIndex: u32,
    pub StrBufferChars: u32,
    pub StrCharsNeeded: u32,
    pub DataBufferIndex: u32,
    pub DataBufferBytes: u32,
    pub DataBytesNeeded: u32,
    pub Status: windows_core::HRESULT,
    pub Reserved: [u64; 8],
}
impl Default for EXT_TYPED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIELDS_DID_NOT_MATCH: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct FIELD_INFO {
    pub fName: super::minwindef::PUCHAR,
    pub printName: super::minwindef::PUCHAR,
    pub size: u32,
    pub fOptions: u32,
    pub address: u64,
    pub Anonymous: FIELD_INFO_0,
    pub TypeId: u32,
    pub FieldOffset: u32,
    pub BufferSize: u32,
    pub BitField: FIELD_INFO_1,
    pub _bitfield: u32,
}
#[cfg(feature = "minwindef")]
impl Default for FIELD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union FIELD_INFO_0 {
    pub fieldCallBack: *mut core::ffi::c_void,
    pub pBuffer: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for FIELD_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FIELD_INFO_1 {
    pub Position: u16,
    pub Size: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_CONTEXT_EX {
    pub Status: u32,
    pub ContextSize: u32,
    pub pContext: *mut core::ffi::c_void,
}
impl Default for GET_CONTEXT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CURRENT_PROCESS_ADDRESS {
    pub Processor: u32,
    pub CurrentThread: u64,
    pub Address: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CURRENT_THREAD_ADDRESS {
    pub Processor: u32,
    pub Address: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_EXPRESSION_EX {
    pub Expression: windows_core::PCSTR,
    pub Remainder: windows_core::PCSTR,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_INPUT_LINE {
    pub Prompt: windows_core::PCSTR,
    pub Buffer: windows_core::PSTR,
    pub BufferSize: u32,
    pub InputSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_PEB_ADDRESS {
    pub CurrentThread: u64,
    pub Address: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_SET_SYMPATH {
    pub Args: windows_core::PCSTR,
    pub Result: windows_core::PSTR,
    pub Length: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_TEB_ADDRESS {
    pub Address: u64,
}
pub const IG_DISASSEMBLE_BUFFER: u32 = 44;
pub const IG_DUMP_SYMBOL_INFO: u32 = 22;
pub const IG_FIND_FILE: u32 = 40;
pub const IG_GET_ANY_MODULE_IN_RANGE: u32 = 45;
pub const IG_GET_BUS_DATA: u32 = 20;
pub const IG_GET_CACHE_SIZE: u32 = 32;
pub const IG_GET_CLR_DATA_INTERFACE: u32 = 38;
pub const IG_GET_CONTEXT_EX: u32 = 48;
pub const IG_GET_CURRENT_PROCESS: u32 = 26;
pub const IG_GET_CURRENT_PROCESS_HANDLE: u32 = 28;
pub const IG_GET_CURRENT_THREAD: u32 = 25;
pub const IG_GET_DEBUGGER_DATA: u32 = 14;
pub const IG_GET_EXCEPTION_RECORD: u32 = 18;
pub const IG_GET_EXPRESSION_EX: u32 = 30;
pub const IG_GET_INPUT_LINE: u32 = 29;
pub const IG_GET_KERNEL_VERSION: u32 = 15;
pub const IG_GET_PEB_ADDRESS: u32 = 129;
pub const IG_GET_SET_SYMPATH: u32 = 17;
pub const IG_GET_TEB_ADDRESS: u32 = 128;
pub const IG_GET_THREAD_OS_INFO: u32 = 37;
pub const IG_GET_TYPE_SIZE: u32 = 27;
pub const IG_IS_PTR64: u32 = 19;
pub const IG_KD_CONTEXT: u32 = 1;
pub const IG_KSTACK_HELP: u32 = 10;
pub const IG_LOWMEM_CHECK: u32 = 23;
pub const IG_MATCH_PATTERN_A: u32 = 39;
pub const IG_OBSOLETE_PLACEHOLDER_36: u32 = 36;
pub const IG_PHYSICAL_TO_VIRTUAL: u32 = 47;
pub const IG_POINTER_SEARCH_PHYSICAL: u32 = 35;
pub const IG_QUERY_TARGET_INTERFACE: u32 = 42;
pub const IG_READ_CONTROL_SPACE: u32 = 2;
pub const IG_READ_IO_SPACE: u32 = 4;
pub const IG_READ_IO_SPACE_EX: u32 = 8;
pub const IG_READ_MSR: u32 = 12;
pub const IG_READ_PHYSICAL: u32 = 6;
pub const IG_READ_PHYSICAL_WITH_FLAGS: u32 = 33;
pub const IG_RELOAD_SYMBOLS: u32 = 16;
pub const IG_SEARCH_MEMORY: u32 = 24;
pub const IG_SET_BUS_DATA: u32 = 21;
pub const IG_SET_THREAD: u32 = 11;
pub const IG_TRANSLATE_VIRTUAL_TO_PHYSICAL: u32 = 31;
pub const IG_TYPED_DATA: u32 = 43;
pub const IG_TYPED_DATA_OBSOLETE: u32 = 41;
pub const IG_VIRTUAL_TO_PHYSICAL: u32 = 46;
pub const IG_WRITE_CONTROL_SPACE: u32 = 3;
pub const IG_WRITE_IO_SPACE: u32 = 5;
pub const IG_WRITE_IO_SPACE_EX: u32 = 9;
pub const IG_WRITE_MSR: u32 = 13;
pub const IG_WRITE_PHYSICAL: u32 = 7;
pub const IG_WRITE_PHYSICAL_WITH_FLAGS: u32 = 34;
pub const INCORRECT_VERSION_INFO: u32 = 7;
pub const INSUFFICIENT_SPACE_TO_COPY: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE {
    pub Address: u32,
    pub Length: u32,
    pub Data: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE32 {
    pub Address: u32,
    pub Length: u32,
    pub Data: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE64 {
    pub Address: u64,
    pub Length: u32,
    pub Data: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE_EX {
    pub Address: u32,
    pub Length: u32,
    pub Data: u32,
    pub InterfaceType: u32,
    pub BusNumber: u32,
    pub AddressSpace: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE_EX32 {
    pub Address: u32,
    pub Length: u32,
    pub Data: u32,
    pub InterfaceType: u32,
    pub BusNumber: u32,
    pub AddressSpace: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE_EX64 {
    pub Address: u64,
    pub Length: u32,
    pub Data: u32,
    pub InterfaceType: u32,
    pub BusNumber: u32,
    pub AddressSpace: u32,
}
pub const KDBG_TAG: u32 = 1195525195;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KDDEBUGGER_DATA32 {
    pub Header: DBGKD_DEBUG_DATA_HEADER32,
    pub KernBase: u32,
    pub BreakpointWithStatus: u32,
    pub SavedContext: u32,
    pub ThCallbackStack: u16,
    pub NextCallback: u16,
    pub FramePointer: u16,
    pub _bitfield: u16,
    pub KiCallUserMode: u32,
    pub KeUserCallbackDispatcher: u32,
    pub PsLoadedModuleList: u32,
    pub PsActiveProcessHead: u32,
    pub PspCidTable: u32,
    pub ExpSystemResourcesList: u32,
    pub ExpPagedPoolDescriptor: u32,
    pub ExpNumberOfPagedPools: u32,
    pub KeTimeIncrement: u32,
    pub KeBugCheckCallbackListHead: u32,
    pub KiBugcheckData: u32,
    pub IopErrorLogListHead: u32,
    pub ObpRootDirectoryObject: u32,
    pub ObpTypeObjectType: u32,
    pub MmSystemCacheStart: u32,
    pub MmSystemCacheEnd: u32,
    pub MmSystemCacheWs: u32,
    pub MmPfnDatabase: u32,
    pub MmSystemPtesStart: u32,
    pub MmSystemPtesEnd: u32,
    pub MmSubsectionBase: u32,
    pub MmNumberOfPagingFiles: u32,
    pub MmLowestPhysicalPage: u32,
    pub MmHighestPhysicalPage: u32,
    pub MmNumberOfPhysicalPages: u32,
    pub MmMaximumNonPagedPoolInBytes: u32,
    pub MmNonPagedSystemStart: u32,
    pub MmNonPagedPoolStart: u32,
    pub MmNonPagedPoolEnd: u32,
    pub MmPagedPoolStart: u32,
    pub MmPagedPoolEnd: u32,
    pub MmPagedPoolInformation: u32,
    pub MmPageSize: u32,
    pub MmSizeOfPagedPoolInBytes: u32,
    pub MmTotalCommitLimit: u32,
    pub MmTotalCommittedPages: u32,
    pub MmSharedCommit: u32,
    pub MmDriverCommit: u32,
    pub MmProcessCommit: u32,
    pub MmPagedPoolCommit: u32,
    pub MmExtendedCommit: u32,
    pub MmZeroedPageListHead: u32,
    pub MmFreePageListHead: u32,
    pub MmStandbyPageListHead: u32,
    pub MmModifiedPageListHead: u32,
    pub MmModifiedNoWritePageListHead: u32,
    pub MmAvailablePages: u32,
    pub MmResidentAvailablePages: u32,
    pub PoolTrackTable: u32,
    pub NonPagedPoolDescriptor: u32,
    pub MmHighestUserAddress: u32,
    pub MmSystemRangeStart: u32,
    pub MmUserProbeAddress: u32,
    pub KdPrintCircularBuffer: u32,
    pub KdPrintCircularBufferEnd: u32,
    pub KdPrintWritePointer: u32,
    pub KdPrintRolloverCount: u32,
    pub MmLoadedUserImageList: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KDDEBUGGER_DATA64 {
    pub Header: DBGKD_DEBUG_DATA_HEADER64,
    pub KernBase: u64,
    pub BreakpointWithStatus: u64,
    pub SavedContext: u64,
    pub ThCallbackStack: u16,
    pub NextCallback: u16,
    pub FramePointer: u16,
    pub _bitfield: u16,
    pub KiCallUserMode: u64,
    pub KeUserCallbackDispatcher: u64,
    pub PsLoadedModuleList: u64,
    pub PsActiveProcessHead: u64,
    pub PspCidTable: u64,
    pub ExpSystemResourcesList: u64,
    pub ExpPagedPoolDescriptor: u64,
    pub ExpNumberOfPagedPools: u64,
    pub KeTimeIncrement: u64,
    pub KeBugCheckCallbackListHead: u64,
    pub KiBugcheckData: u64,
    pub IopErrorLogListHead: u64,
    pub ObpRootDirectoryObject: u64,
    pub ObpTypeObjectType: u64,
    pub MmSystemCacheStart: u64,
    pub MmSystemCacheEnd: u64,
    pub MmSystemCacheWs: u64,
    pub MmPfnDatabase: u64,
    pub MmSystemPtesStart: u64,
    pub MmSystemPtesEnd: u64,
    pub MmSubsectionBase: u64,
    pub MmNumberOfPagingFiles: u64,
    pub MmLowestPhysicalPage: u64,
    pub MmHighestPhysicalPage: u64,
    pub MmNumberOfPhysicalPages: u64,
    pub MmMaximumNonPagedPoolInBytes: u64,
    pub MmNonPagedSystemStart: u64,
    pub MmNonPagedPoolStart: u64,
    pub MmNonPagedPoolEnd: u64,
    pub MmPagedPoolStart: u64,
    pub MmPagedPoolEnd: u64,
    pub MmPagedPoolInformation: u64,
    pub MmPageSize: u64,
    pub MmSizeOfPagedPoolInBytes: u64,
    pub MmTotalCommitLimit: u64,
    pub MmTotalCommittedPages: u64,
    pub MmSharedCommit: u64,
    pub MmDriverCommit: u64,
    pub MmProcessCommit: u64,
    pub MmPagedPoolCommit: u64,
    pub MmExtendedCommit: u64,
    pub MmZeroedPageListHead: u64,
    pub MmFreePageListHead: u64,
    pub MmStandbyPageListHead: u64,
    pub MmModifiedPageListHead: u64,
    pub MmModifiedNoWritePageListHead: u64,
    pub MmAvailablePages: u64,
    pub MmResidentAvailablePages: u64,
    pub PoolTrackTable: u64,
    pub NonPagedPoolDescriptor: u64,
    pub MmHighestUserAddress: u64,
    pub MmSystemRangeStart: u64,
    pub MmUserProbeAddress: u64,
    pub KdPrintCircularBuffer: u64,
    pub KdPrintCircularBufferEnd: u64,
    pub KdPrintWritePointer: u64,
    pub KdPrintRolloverCount: u64,
    pub MmLoadedUserImageList: u64,
    pub NtBuildLab: u64,
    pub KiNormalSystemCall: u64,
    pub KiProcessorBlock: u64,
    pub MmUnloadedDrivers: u64,
    pub MmLastUnloadedDriver: u64,
    pub MmTriageActionTaken: u64,
    pub MmSpecialPoolTag: u64,
    pub KernelVerifier: u64,
    pub MmVerifierData: u64,
    pub MmAllocatedNonPagedPool: u64,
    pub MmPeakCommitment: u64,
    pub MmTotalCommitLimitMaximum: u64,
    pub CmNtCSDVersion: u64,
    pub MmPhysicalMemoryBlock: u64,
    pub MmSessionBase: u64,
    pub MmSessionSize: u64,
    pub MmSystemParentTablePage: u64,
    pub MmVirtualTranslationBase: u64,
    pub OffsetKThreadNextProcessor: u16,
    pub OffsetKThreadTeb: u16,
    pub OffsetKThreadKernelStack: u16,
    pub OffsetKThreadInitialStack: u16,
    pub OffsetKThreadApcProcess: u16,
    pub OffsetKThreadState: u16,
    pub OffsetKThreadBStore: u16,
    pub OffsetKThreadBStoreLimit: u16,
    pub SizeEProcess: u16,
    pub OffsetEprocessPeb: u16,
    pub OffsetEprocessParentCID: u16,
    pub OffsetEprocessDirectoryTableBase: u16,
    pub SizePrcb: u16,
    pub OffsetPrcbDpcRoutine: u16,
    pub OffsetPrcbCurrentThread: u16,
    pub OffsetPrcbMhz: u16,
    pub OffsetPrcbCpuType: u16,
    pub OffsetPrcbVendorString: u16,
    pub OffsetPrcbProcStateContext: u16,
    pub OffsetPrcbNumber: u16,
    pub SizeEThread: u16,
    pub L1tfHighPhysicalBitIndex: u8,
    pub L1tfSwizzleBitIndex: u8,
    pub Padding0: u32,
    pub KdPrintCircularBufferPtr: u64,
    pub KdPrintBufferSize: u64,
    pub KeLoaderBlock: u64,
    pub SizePcr: u16,
    pub OffsetPcrSelfPcr: u16,
    pub OffsetPcrCurrentPrcb: u16,
    pub OffsetPcrContainedPrcb: u16,
    pub OffsetPcrInitialBStore: u16,
    pub OffsetPcrBStoreLimit: u16,
    pub OffsetPcrInitialStack: u16,
    pub OffsetPcrStackLimit: u16,
    pub OffsetPrcbPcrPage: u16,
    pub OffsetPrcbProcStateSpecialReg: u16,
    pub GdtR0Code: u16,
    pub GdtR0Data: u16,
    pub GdtR0Pcr: u16,
    pub GdtR3Code: u16,
    pub GdtR3Data: u16,
    pub GdtR3Teb: u16,
    pub GdtLdt: u16,
    pub GdtTss: u16,
    pub Gdt64R3CmCode: u16,
    pub Gdt64R3CmTeb: u16,
    pub IopNumTriageDumpDataBlocks: u64,
    pub IopTriageDumpDataBlocks: u64,
    pub VfCrashDataBlock: u64,
    pub MmBadPagesDetected: u64,
    pub MmZeroedPageSingleBitErrorsDetected: u64,
    pub EtwpDebuggerData: u64,
    pub OffsetPrcbContext: u16,
    pub OffsetPrcbMaxBreakpoints: u16,
    pub OffsetPrcbMaxWatchpoints: u16,
    pub OffsetKThreadStackLimit: u32,
    pub OffsetKThreadStackBase: u32,
    pub OffsetKThreadQueueListEntry: u32,
    pub OffsetEThreadIrpList: u32,
    pub OffsetPrcbIdleThread: u16,
    pub OffsetPrcbNormalDpcState: u16,
    pub OffsetPrcbDpcStack: u16,
    pub OffsetPrcbIsrStack: u16,
    pub SizeKDPC_STACK_FRAME: u16,
    pub OffsetKPriQueueThreadListHead: u16,
    pub OffsetKThreadWaitReason: u16,
    pub Padding1: u16,
    pub PteBase: u64,
    pub RetpolineStubFunctionTable: u64,
    pub RetpolineStubFunctionTableSize: u32,
    pub RetpolineStubOffset: u32,
    pub RetpolineStubSize: u32,
    pub OffsetEProcessMmHotPatchContext: u16,
    pub OffsetKThreadShadowStackLimit: u32,
    pub OffsetKThreadShadowStackBase: u32,
    pub ShadowStackEnabled: u64,
    pub PointerAuthMask: u64,
    pub OffsetPrcbExceptionStack: u16,
    pub PointerIgnoreBits: u64,
}
pub const KD_SECONDARY_VERSION_AMD64_CONTEXT: u32 = 2;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_1: u32 = 0;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_2: u32 = 1;
pub const KD_SECONDARY_VERSION_DEFAULT: u32 = 0;
pub type LPEXT_API_VERSION = *mut EXT_API_VERSION;
pub const MEMORY_READ_ERROR: u32 = 1;
pub const NULL_FIELD_NAME: u32 = 6;
pub const NULL_SYM_DUMP_PARAM: u32 = 5;
pub type PBUSDATA = *mut BUSDATA;
#[cfg(feature = "winnt")]
pub type PDBGKD_DEBUG_DATA_HEADER32 = *mut DBGKD_DEBUG_DATA_HEADER32;
#[cfg(feature = "winnt")]
pub type PDBGKD_DEBUG_DATA_HEADER64 = *mut DBGKD_DEBUG_DATA_HEADER64;
pub type PDBGKD_GET_VERSION32 = *mut DBGKD_GET_VERSION32;
pub type PDBGKD_GET_VERSION64 = *mut DBGKD_GET_VERSION64;
pub type PDEBUG_TYPED_DATA = *mut DEBUG_TYPED_DATA;
pub type PEXTSTACKTRACE = *mut EXTSTACKTRACE;
pub type PEXTSTACKTRACE32 = *mut EXTSTACKTRACE32;
pub type PEXTSTACKTRACE64 = *mut EXTSTACKTRACE64;
#[cfg(feature = "winnt")]
pub type PEXT_FIND_FILE = *mut EXT_FIND_FILE;
pub type PEXT_MATCH_PATTERN_A = *mut EXT_MATCH_PATTERN_A;
pub type PEXT_TYPED_DATA = *mut EXT_TYPED_DATA;
#[cfg(feature = "minwindef")]
pub type PFIELD_INFO = *mut FIELD_INFO;
pub type PGET_CONTEXT_EX = *mut GET_CONTEXT_EX;
pub type PGET_CURRENT_PROCESS_ADDRESS = *mut GET_CURRENT_PROCESS_ADDRESS;
pub type PGET_CURRENT_THREAD_ADDRESS = *mut GET_CURRENT_THREAD_ADDRESS;
pub type PGET_EXPRESSION_EX = *mut GET_EXPRESSION_EX;
pub type PGET_INPUT_LINE = *mut GET_INPUT_LINE;
pub type PGET_PEB_ADDRESS = *mut GET_PEB_ADDRESS;
pub type PGET_SET_SYMPATH = *mut GET_SET_SYMPATH;
pub type PGET_TEB_ADDRESS = *mut GET_TEB_ADDRESS;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL {
    pub Address: u64,
    pub BufLen: u32,
    pub Buf: [u8; 1],
}
impl Default for PHYSICAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PHYSICAL_TO_VIRTUAL {
    pub Status: u32,
    pub Size: u32,
    pub PdeAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_WITH_FLAGS {
    pub Address: u64,
    pub BufLen: u32,
    pub Flags: u32,
    pub Buf: [u8; 1],
}
impl Default for PHYSICAL_WITH_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PHYS_FLAG_CACHED: u32 = 1;
pub const PHYS_FLAG_DEFAULT: u32 = 0;
pub const PHYS_FLAG_UNCACHED: u32 = 2;
pub const PHYS_FLAG_WRITE_COMBINED: u32 = 3;
pub type PIOSPACE = *mut IOSPACE;
pub type PIOSPACE32 = *mut IOSPACE32;
pub type PIOSPACE64 = *mut IOSPACE64;
pub type PIOSPACE_EX = *mut IOSPACE_EX;
pub type PIOSPACE_EX32 = *mut IOSPACE_EX32;
pub type PIOSPACE_EX64 = *mut IOSPACE_EX64;
#[cfg(feature = "winnt")]
pub type PKDDEBUGGER_DATA32 = *mut KDDEBUGGER_DATA32;
#[cfg(feature = "winnt")]
pub type PKDDEBUGGER_DATA64 = *mut KDDEBUGGER_DATA64;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_SEARCH_PHYSICAL {
    pub Offset: u64,
    pub Length: u64,
    pub PointerMin: u64,
    pub PointerMax: u64,
    pub Flags: u32,
    pub MatchOffsets: super::basetsd::PULONG64,
    pub MatchOffsetsSize: u32,
    pub MatchOffsetsCount: u32,
}
pub type POOL_HEADER_FIELD_NAME = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union POOL_HEADER_SIZE_64 {
    pub Anonymous: POOL_HEADER_SIZE_64_0,
    pub Ulong1: u32,
}
impl Default for POOL_HEADER_SIZE_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POOL_HEADER_SIZE_64_0 {
    pub UnsafePrevSize: u8,
    pub Unused1: u8,
    pub UnsafeSize: u8,
    pub UnsafePoolType: u8,
}
pub type PPHYSICAL = *mut PHYSICAL;
pub type PPHYSICAL_TO_VIRTUAL = *mut PHYSICAL_TO_VIRTUAL;
pub type PPHYSICAL_WITH_FLAGS = *mut PHYSICAL_WITH_FLAGS;
#[cfg(feature = "basetsd")]
pub type PPOINTER_SEARCH_PHYSICAL = *mut POINTER_SEARCH_PHYSICAL;
pub type PPOOL_HEADER_SIZE_64 = *mut POOL_HEADER_SIZE_64;
pub type PPROCESSORINFO = *mut PROCESSORINFO;
pub type PREADCONTROLSPACE = *mut READCONTROLSPACE;
pub type PREADCONTROLSPACE32 = *mut READCONTROLSPACE32;
pub type PREADCONTROLSPACE64 = *mut READCONTROLSPACE64;
pub type PREAD_WRITE_MSR = *mut READ_WRITE_MSR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSORINFO {
    pub Processor: u16,
    pub NumberProcessors: u16,
}
pub type PSEARCHMEMORY = *mut SEARCHMEMORY;
#[cfg(feature = "minwindef")]
pub type PSYM_DUMP_FIELD_CALLBACK = Option<unsafe extern "system" fn(pfield: *mut FIELD_INFO, usercontext: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "minwindef")]
pub type PSYM_DUMP_PARAM = *mut SYM_DUMP_PARAM;
pub type PTRANSLATE_VIRTUAL_TO_PHYSICAL = *mut TRANSLATE_VIRTUAL_TO_PHYSICAL;
pub const PTR_SEARCH_NO_SYMBOL_CHECK: u32 = 2147483648;
pub const PTR_SEARCH_PHYS_ALL_HITS: u32 = 1;
pub const PTR_SEARCH_PHYS_PTE: u32 = 2;
pub const PTR_SEARCH_PHYS_RANGE_CHECK_ONLY: u32 = 4;
pub const PTR_SEARCH_PHYS_SIZE_MASK: u32 = 120;
pub const PTR_SEARCH_PHYS_SIZE_SHIFT: u32 = 3;
pub type PVIRTUAL_TO_PHYSICAL = *mut VIRTUAL_TO_PHYSICAL;
pub type PWDBGEXTS_CLR_DATA_INTERFACE = *mut WDBGEXTS_CLR_DATA_INTERFACE;
pub type PWDBGEXTS_DISASSEMBLE_BUFFER = *mut WDBGEXTS_DISASSEMBLE_BUFFER;
pub type PWDBGEXTS_MODULE_IN_RANGE = *mut WDBGEXTS_MODULE_IN_RANGE;
pub type PWDBGEXTS_QUERY_INTERFACE = *mut WDBGEXTS_QUERY_INTERFACE;
pub type PWDBGEXTS_THREAD_OS_INFO = *mut WDBGEXTS_THREAD_OS_INFO;
pub type PWINDBG_CHECK_CONTROL_C = Option<unsafe extern "system" fn() -> u32>;
pub type PWINDBG_CHECK_VERSION = Option<unsafe extern "system" fn() -> u32>;
pub type PWINDBG_DISASM = Option<unsafe extern "system" fn(lpoffset: *mut usize, lpbuffer: windows_core::PSTR, fshoweffectiveaddress: u32) -> u32>;
pub type PWINDBG_DISASM32 = Option<unsafe extern "system" fn(lpoffset: *mut u32, lpbuffer: windows_core::PSTR, fshoweffectiveaddress: u32) -> u32>;
pub type PWINDBG_DISASM64 = Option<unsafe extern "system" fn(lpoffset: *mut u64, lpbuffer: windows_core::PSTR, fshoweffectiveaddress: u32) -> u32>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_APIS = *mut WINDBG_EXTENSION_APIS;
pub type PWINDBG_EXTENSION_API_VERSION = Option<unsafe extern "system" fn() -> LPEXT_API_VERSION>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_DLL_INIT = Option<unsafe extern "system" fn(lpextensionapis: *const WINDBG_EXTENSION_APIS, majorversion: u16, minorversion: u16)>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_DLL_INIT32 = Option<unsafe extern "system" fn(lpextensionapis: *const WINDBG_EXTENSION_APIS32, majorversion: u16, minorversion: u16)>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_DLL_INIT64 = Option<unsafe extern "system" fn(lpextensionapis: *const WINDBG_EXTENSION_APIS64, majorversion: u16, minorversion: u16)>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_ROUTINE = Option<unsafe extern "system" fn(hcurrentprocess: super::winnt::HANDLE, hcurrentthread: super::winnt::HANDLE, dwcurrentpc: u32, dwprocessor: u32, lpargumentstring: windows_core::PCSTR)>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_ROUTINE32 = Option<unsafe extern "system" fn(hcurrentprocess: super::winnt::HANDLE, hcurrentthread: super::winnt::HANDLE, dwcurrentpc: u32, dwprocessor: u32, lpargumentstring: windows_core::PCSTR)>;
#[cfg(feature = "winnt")]
pub type PWINDBG_EXTENSION_ROUTINE64 = Option<unsafe extern "system" fn(hcurrentprocess: super::winnt::HANDLE, hcurrentthread: super::winnt::HANDLE, dwcurrentpc: u64, dwprocessor: u32, lpargumentstring: windows_core::PCSTR)>;
pub type PWINDBG_GET_EXPRESSION = Option<unsafe extern "system" fn(lpexpression: windows_core::PCSTR) -> usize>;
pub type PWINDBG_GET_EXPRESSION32 = Option<unsafe extern "system" fn(lpexpression: windows_core::PCSTR) -> u32>;
pub type PWINDBG_GET_EXPRESSION64 = Option<unsafe extern "system" fn(lpexpression: windows_core::PCSTR) -> u64>;
pub type PWINDBG_GET_SYMBOL = Option<unsafe extern "system" fn(offset: *const core::ffi::c_void, pchbuffer: *mut i8, pdisplacement: *mut usize)>;
pub type PWINDBG_GET_SYMBOL32 = Option<unsafe extern "system" fn(offset: u32, pchbuffer: *mut i8, pdisplacement: *mut u32)>;
pub type PWINDBG_GET_SYMBOL64 = Option<unsafe extern "system" fn(offset: u64, pchbuffer: *mut i8, pdisplacement: *mut u64)>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
pub type PWINDBG_GET_THREAD_CONTEXT_ROUTINE = Option<unsafe extern "system" fn(processor: u32, lpcontext: *mut super::winnt::CONTEXT, cbsizeofcontext: u32) -> u32>;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
pub type PWINDBG_GET_THREAD_CONTEXT_ROUTINE = Option<unsafe extern "system" fn(processor: u32, lpcontext: *mut super::winnt::ARM64_NT_CONTEXT, cbsizeofcontext: u32) -> u32>;
pub type PWINDBG_IOCTL_ROUTINE = Option<unsafe extern "system" fn(ioctltype: u16, lpvdata: *mut core::ffi::c_void, cbsize: u32) -> u32>;
pub type PWINDBG_OLDKD_EXTENSION_APIS = *mut WINDBG_OLDKD_EXTENSION_APIS;
pub type PWINDBG_OLDKD_EXTENSION_ROUTINE = Option<unsafe extern "system" fn(dwcurrentpc: u32, lpextensionapis: *const WINDBG_OLDKD_EXTENSION_APIS, lpargumentstring: windows_core::PCSTR)>;
pub type PWINDBG_OLDKD_READ_PHYSICAL_MEMORY = Option<unsafe extern "system" fn(address: u64, buffer: *mut core::ffi::c_void, count: u32, bytesread: *mut u32) -> u32>;
pub type PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY = Option<unsafe extern "system" fn(address: u64, buffer: *const core::ffi::c_void, length: u32, byteswritten: *mut u32) -> u32>;
pub type PWINDBG_OLD_EXTENSION_APIS = *mut WINDBG_OLD_EXTENSION_APIS;
#[cfg(feature = "winnt")]
pub type PWINDBG_OLD_EXTENSION_ROUTINE = Option<unsafe extern "system" fn(dwcurrentpc: u32, lpextensionapis: *const WINDBG_EXTENSION_APIS, lpargumentstring: windows_core::PCSTR)>;
pub type PWINDBG_OUTPUT_ROUTINE = *mut u8;
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(offset: usize, lpbuffer: *mut core::ffi::c_void, cb: u32, lpcbbytesread: *mut u32) -> u32>;
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE32 = Option<unsafe extern "system" fn(offset: u32, lpbuffer: *mut core::ffi::c_void, cb: u32, lpcbbytesread: *mut u32) -> u32>;
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(offset: u64, lpbuffer: *mut core::ffi::c_void, cb: u32, lpcbbytesread: *mut u32) -> u32>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
pub type PWINDBG_SET_THREAD_CONTEXT_ROUTINE = Option<unsafe extern "system" fn(processor: u32, lpcontext: *const super::winnt::CONTEXT, cbsizeofcontext: u32) -> u32>;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
pub type PWINDBG_SET_THREAD_CONTEXT_ROUTINE = Option<unsafe extern "system" fn(processor: u32, lpcontext: *const super::winnt::ARM64_NT_CONTEXT, cbsizeofcontext: u32) -> u32>;
pub type PWINDBG_STACKTRACE_ROUTINE = Option<unsafe extern "system" fn(framepointer: u32, stackpointer: u32, programcounter: u32, stackframes: *mut EXTSTACKTRACE, frames: u32) -> u32>;
pub type PWINDBG_STACKTRACE_ROUTINE32 = Option<unsafe extern "system" fn(framepointer: u32, stackpointer: u32, programcounter: u32, stackframes: *mut EXTSTACKTRACE32, frames: u32) -> u32>;
pub type PWINDBG_STACKTRACE_ROUTINE64 = Option<unsafe extern "system" fn(framepointer: u64, stackpointer: u64, programcounter: u64, stackframes: *mut EXTSTACKTRACE64, frames: u32) -> u32>;
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(offset: usize, lpbuffer: *const core::ffi::c_void, cb: u32, lpcbbyteswritten: *mut u32) -> u32>;
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32 = Option<unsafe extern "system" fn(offset: u32, lpbuffer: *const core::ffi::c_void, cb: u32, lpcbbyteswritten: *mut u32) -> u32>;
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(offset: u64, lpbuffer: *const core::ffi::c_void, cb: u32, lpcbbyteswritten: *mut u32) -> u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READCONTROLSPACE {
    pub Processor: u16,
    pub Address: u32,
    pub BufLen: u32,
    pub Buf: [u8; 1],
}
impl Default for READCONTROLSPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READCONTROLSPACE32 {
    pub Processor: u16,
    pub Address: u32,
    pub BufLen: u32,
    pub Buf: [u8; 1],
}
impl Default for READCONTROLSPACE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READCONTROLSPACE64 {
    pub Processor: u16,
    pub Address: u64,
    pub BufLen: u32,
    pub Buf: [u8; 1],
}
impl Default for READCONTROLSPACE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_WRITE_MSR {
    pub Msr: u32,
    pub Value: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEARCHMEMORY {
    pub SearchAddress: u64,
    pub SearchLength: u64,
    pub FoundAddress: u64,
    pub PatternLength: u32,
    pub Pattern: *mut core::ffi::c_void,
}
impl Default for SEARCHMEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYMBOL_TYPE_INDEX_NOT_FOUND: u32 = 2;
pub const SYMBOL_TYPE_INFO_NOT_FOUND: u32 = 3;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct SYM_DUMP_PARAM {
    pub size: u32,
    pub sName: super::minwindef::PUCHAR,
    pub Options: u32,
    pub addr: u64,
    pub listLink: PFIELD_INFO,
    pub Anonymous: SYM_DUMP_PARAM_0,
    pub CallbackRoutine: PSYM_DUMP_FIELD_CALLBACK,
    pub nFields: u32,
    pub Fields: PFIELD_INFO,
    pub ModBase: u64,
    pub TypeId: u32,
    pub TypeSize: u32,
    pub BufferSize: u32,
    pub _bitfield: u32,
}
#[cfg(feature = "minwindef")]
impl Default for SYM_DUMP_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union SYM_DUMP_PARAM_0 {
    pub Context: *mut core::ffi::c_void,
    pub pBuffer: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for SYM_DUMP_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSLATE_VIRTUAL_TO_PHYSICAL {
    pub Virtual: u64,
    pub Physical: u64,
}
pub const UNAVAILABLE_ERROR: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_TO_PHYSICAL {
    pub Status: u32,
    pub Size: u32,
    pub PdeAddress: u64,
    pub Virtual: u64,
    pub Physical: u64,
}
pub const WDBGEXTS_ADDRESS_DEFAULT: u32 = 0;
pub const WDBGEXTS_ADDRESS_RESERVED0: u32 = 2147483648;
pub const WDBGEXTS_ADDRESS_SEG16: u32 = 1;
pub const WDBGEXTS_ADDRESS_SEG32: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WDBGEXTS_CLR_DATA_INTERFACE {
    pub Iid: *const windows_core::GUID,
    pub Iface: *mut core::ffi::c_void,
}
impl Default for WDBGEXTS_CLR_DATA_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WDBGEXTS_DISASSEMBLE_BUFFER {
    pub InOffset: u64,
    pub OutOffset: u64,
    pub AddrFlags: u32,
    pub FormatFlags: u32,
    pub DataBufferBytes: u32,
    pub DisasmBufferChars: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub DisasmBuffer: windows_core::PWSTR,
    pub Reserved0: [u64; 3],
}
impl Default for WDBGEXTS_DISASSEMBLE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WDBGEXTS_MAXSIZE_T: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDBGEXTS_MODULE_IN_RANGE {
    pub Start: u64,
    pub End: u64,
    pub FoundModBase: u64,
    pub FoundModSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WDBGEXTS_QUERY_INTERFACE {
    pub Iid: *const windows_core::GUID,
    pub Iface: *mut core::ffi::c_void,
}
impl Default for WDBGEXTS_QUERY_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDBGEXTS_THREAD_OS_INFO {
    pub ThreadId: u32,
    pub ExitStatus: u32,
    pub PriorityClass: u32,
    pub Priority: u32,
    pub CreateTime: u64,
    pub ExitTime: u64,
    pub KernelTime: u64,
    pub UserTime: u64,
    pub StartOffset: u64,
    pub Affinity: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINDBG_EXTENSION_APIS {
    pub nSize: u32,
    pub lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    pub lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION,
    pub lpGetSymbolRoutine: PWINDBG_GET_SYMBOL,
    pub lpDisasmRoutine: PWINDBG_DISASM,
    pub lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    pub lpReadProcessMemoryRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE,
    pub lpWriteProcessMemoryRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE,
    pub lpGetThreadContextRoutine: PWINDBG_GET_THREAD_CONTEXT_ROUTINE,
    pub lpSetThreadContextRoutine: PWINDBG_SET_THREAD_CONTEXT_ROUTINE,
    pub lpIoctlRoutine: PWINDBG_IOCTL_ROUTINE,
    pub lpStackTraceRoutine: PWINDBG_STACKTRACE_ROUTINE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINDBG_EXTENSION_APIS32 {
    pub nSize: u32,
    pub lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    pub lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION32,
    pub lpGetSymbolRoutine: PWINDBG_GET_SYMBOL32,
    pub lpDisasmRoutine: PWINDBG_DISASM32,
    pub lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    pub lpReadProcessMemoryRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE32,
    pub lpWriteProcessMemoryRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32,
    pub lpGetThreadContextRoutine: PWINDBG_GET_THREAD_CONTEXT_ROUTINE,
    pub lpSetThreadContextRoutine: PWINDBG_SET_THREAD_CONTEXT_ROUTINE,
    pub lpIoctlRoutine: PWINDBG_IOCTL_ROUTINE,
    pub lpStackTraceRoutine: PWINDBG_STACKTRACE_ROUTINE32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINDBG_EXTENSION_APIS64 {
    pub nSize: u32,
    pub lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    pub lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION64,
    pub lpGetSymbolRoutine: PWINDBG_GET_SYMBOL64,
    pub lpDisasmRoutine: PWINDBG_DISASM64,
    pub lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    pub lpReadProcessMemoryRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE64,
    pub lpWriteProcessMemoryRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64,
    pub lpGetThreadContextRoutine: PWINDBG_GET_THREAD_CONTEXT_ROUTINE,
    pub lpSetThreadContextRoutine: PWINDBG_SET_THREAD_CONTEXT_ROUTINE,
    pub lpIoctlRoutine: PWINDBG_IOCTL_ROUTINE,
    pub lpStackTraceRoutine: PWINDBG_STACKTRACE_ROUTINE64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINDBG_OLDKD_EXTENSION_APIS {
    pub nSize: u32,
    pub lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    pub lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION32,
    pub lpGetSymbolRoutine: PWINDBG_GET_SYMBOL32,
    pub lpDisasmRoutine: PWINDBG_DISASM32,
    pub lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
    pub lpReadVirtualMemRoutine: PWINDBG_READ_PROCESS_MEMORY_ROUTINE32,
    pub lpWriteVirtualMemRoutine: PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32,
    pub lpReadPhysicalMemRoutine: PWINDBG_OLDKD_READ_PHYSICAL_MEMORY,
    pub lpWritePhysicalMemRoutine: PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINDBG_OLD_EXTENSION_APIS {
    pub nSize: u32,
    pub lpOutputRoutine: PWINDBG_OUTPUT_ROUTINE,
    pub lpGetExpressionRoutine: PWINDBG_GET_EXPRESSION,
    pub lpGetSymbolRoutine: PWINDBG_GET_SYMBOL,
    pub lpDisasmRoutine: PWINDBG_DISASM,
    pub lpCheckControlCRoutine: PWINDBG_CHECK_CONTROL_C,
}
