pub const ADDRESS_TYPE_INDEX_NOT_FOUND: i32 = 11;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BUSDATA {
    pub BusDataType: super::ULONG,
    pub BusNumber: super::ULONG,
    pub SlotNumber: super::ULONG,
    pub Buffer: super::PVOID,
    pub Offset: super::ULONG,
    pub Length: super::ULONG,
}
pub const CANNOT_ALLOCATE_MEMORY: i32 = 9;
pub const CROSS_PLATFORM_MAXIMUM_PROCESSORS: i32 = 2048;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CURRENT_KD_SECONDARY_VERSION: i32 = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const CURRENT_KD_SECONDARY_VERSION: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBGKD_DEBUG_DATA_HEADER32 {
    pub List: super::LIST_ENTRY32,
    pub OwnerTag: super::ULONG,
    pub Size: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBGKD_DEBUG_DATA_HEADER64 {
    pub List: super::LIST_ENTRY64,
    pub OwnerTag: super::ULONG,
    pub Size: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBGKD_GET_VERSION32 {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
    pub ProtocolVersion: super::USHORT,
    pub Flags: super::USHORT,
    pub KernBase: super::ULONG,
    pub PsLoadedModuleList: super::ULONG,
    pub MachineType: super::USHORT,
    pub ThCallbackStack: super::USHORT,
    pub NextCallback: super::USHORT,
    pub FramePointer: super::USHORT,
    pub KiCallUserMode: super::ULONG,
    pub KeUserCallbackDispatcher: super::ULONG,
    pub BreakpointWithStatus: super::ULONG,
    pub DebuggerDataList: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBGKD_GET_VERSION64 {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
    pub ProtocolVersion: super::UCHAR,
    pub KdSecondaryVersion: super::UCHAR,
    pub Flags: super::USHORT,
    pub MachineType: super::USHORT,
    pub MaxPacketType: super::UCHAR,
    pub MaxStateChange: super::UCHAR,
    pub MaxManipulate: super::UCHAR,
    pub Simulation: super::UCHAR,
    pub Unused: [super::USHORT; 1],
    pub KernBase: super::ULONG64,
    pub PsLoadedModuleList: super::ULONG64,
    pub DebuggerDataList: super::ULONG64,
}
#[cfg(feature = "shared")]
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
pub const DBGKD_VERS_FLAG_DATA: i32 = 2;
pub const DBGKD_VERS_FLAG_HAL_IN_NTOS: i32 = 64;
pub const DBGKD_VERS_FLAG_HSS: i32 = 16;
pub const DBGKD_VERS_FLAG_MP: i32 = 1;
pub const DBGKD_VERS_FLAG_NOMM: i32 = 8;
pub const DBGKD_VERS_FLAG_PARTITIONS: i32 = 32;
pub const DBGKD_VERS_FLAG_PTR64: i32 = 4;
pub const DBG_DUMP_ADDRESS_AT_END: i32 = 131072;
pub const DBG_DUMP_ADDRESS_OF_FIELD: i32 = 65536;
pub const DBG_DUMP_ARRAY: i32 = 32768;
pub const DBG_DUMP_BLOCK_RECURSE: i32 = 2097152;
pub const DBG_DUMP_CALL_FOR_EACH: i32 = 8;
pub const DBG_DUMP_COMPACT_OUT: i32 = 8192;
pub const DBG_DUMP_COPY_TYPE_DATA: i32 = 262144;
pub const DBG_DUMP_FIELD_ARRAY: i32 = 16;
pub const DBG_DUMP_FIELD_CALL_BEFORE_PRINT: i32 = 1;
pub const DBG_DUMP_FIELD_COPY_FIELD_DATA: i32 = 32;
pub const DBG_DUMP_FIELD_DEFAULT_STRING: i32 = 65536;
pub const DBG_DUMP_FIELD_FULL_NAME: i32 = 8;
pub const DBG_DUMP_FIELD_GUID_STRING: i32 = 524288;
pub const DBG_DUMP_FIELD_MULTI_STRING: i32 = 262144;
pub const DBG_DUMP_FIELD_NO_CALLBACK_REQ: i32 = 2;
pub const DBG_DUMP_FIELD_NO_PRINT: i32 = 16384;
pub const DBG_DUMP_FIELD_RECUR_ON_THIS: i32 = 4;
pub const DBG_DUMP_FIELD_RETURN_ADDRESS: i32 = 4096;
pub const DBG_DUMP_FIELD_SIZE_IN_BITS: i32 = 8192;
pub const DBG_DUMP_FIELD_UTF32_STRING: i32 = 1048576;
pub const DBG_DUMP_FIELD_WCHAR_STRING: i32 = 131072;
pub const DBG_DUMP_FUNCTION_FORMAT: i32 = 1048576;
pub const DBG_DUMP_GET_SIZE_ONLY: i32 = 128;
pub const DBG_DUMP_LIST: i32 = 32;
pub const DBG_DUMP_MATCH_SIZE: i32 = 4194304;
pub const DBG_DUMP_NO_INDENT: i32 = 1;
pub const DBG_DUMP_NO_OFFSET: i32 = 2;
pub const DBG_DUMP_NO_PRINT: i32 = 64;
pub const DBG_DUMP_READ_PHYSICAL: i32 = 524288;
pub const DBG_DUMP_VERBOSE: i32 = 4;
pub const DBG_RETURN_SUBTYPES: i32 = 0;
pub const DBG_RETURN_TYPE: i32 = 0;
pub const DBG_RETURN_TYPE_VALUES: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEBUG_TYPED_DATA {
    pub ModBase: super::ULONG64,
    pub Offset: super::ULONG64,
    pub EngineHandle: super::ULONG64,
    pub Data: super::ULONG64,
    pub Size: super::ULONG,
    pub Flags: super::ULONG,
    pub TypeId: super::ULONG,
    pub BaseTypeId: super::ULONG,
    pub Tag: super::ULONG,
    pub Register: super::ULONG,
    pub Internal: [super::ULONG64; 9],
}
#[cfg(feature = "shared")]
impl Default for DEBUG_TYPED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_TYPED_DATA_IS_IN_MEMORY: i32 = 1;
pub const DEBUG_TYPED_DATA_PHYSICAL_CACHED: i32 = 4;
pub const DEBUG_TYPED_DATA_PHYSICAL_DEFAULT: i32 = 2;
pub const DEBUG_TYPED_DATA_PHYSICAL_MEMORY: i32 = 14;
pub const DEBUG_TYPED_DATA_PHYSICAL_UNCACHED: i32 = 6;
pub const DEBUG_TYPED_DATA_PHYSICAL_WRITE_COMBINED: i32 = 8;
pub const DbgkdBlockSize: POOL_HEADER_FIELD_NAME = 2;
pub const DbgkdPoolIndex: POOL_HEADER_FIELD_NAME = 1;
pub const DbgkdPoolType: POOL_HEADER_FIELD_NAME = 3;
pub const DbgkdPreviousSize: POOL_HEADER_FIELD_NAME = 0;
pub const DbgkdUlong1: POOL_HEADER_FIELD_NAME = 4;
pub const EXIT_ON_CONTROLC: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTSTACKTRACE {
    pub FramePointer: super::ULONG,
    pub ProgramCounter: super::ULONG,
    pub ReturnAddress: super::ULONG,
    pub Args: [super::ULONG; 4],
}
#[cfg(feature = "shared")]
impl Default for EXTSTACKTRACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTSTACKTRACE32 {
    pub FramePointer: super::ULONG,
    pub ProgramCounter: super::ULONG,
    pub ReturnAddress: super::ULONG,
    pub Args: [super::ULONG; 4],
}
#[cfg(feature = "shared")]
impl Default for EXTSTACKTRACE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXTSTACKTRACE64 {
    pub FramePointer: super::ULONG64,
    pub ProgramCounter: super::ULONG64,
    pub ReturnAddress: super::ULONG64,
    pub Args: [super::ULONG64; 4],
}
#[cfg(feature = "shared")]
impl Default for EXTSTACKTRACE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXT_API_VERSION {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
    pub Revision: super::USHORT,
    pub Reserved: super::USHORT,
}
pub const EXT_API_VERSION_NUMBER: i32 = 5;
pub const EXT_API_VERSION_NUMBER32: i32 = 5;
pub const EXT_API_VERSION_NUMBER64: i32 = 6;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXT_FIND_FILE {
    pub FileName: windows_core::PCWSTR,
    pub IndexedSize: super::ULONG64,
    pub ImageTimeDateStamp: super::ULONG,
    pub ImageCheckSum: super::ULONG,
    pub ExtraInfo: super::PVOID,
    pub ExtraInfoSize: super::ULONG,
    pub Flags: super::ULONG,
    pub FileMapping: super::PVOID,
    pub FileMappingSize: super::ULONG64,
    pub FileHandle: super::HANDLE,
    pub FoundFileName: windows_core::PWSTR,
    pub FoundFileNameChars: super::ULONG,
}
pub const EXT_FIND_FILE_ALLOW_GIVEN_PATH: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXT_MATCH_PATTERN_A {
    pub Str: windows_core::PCSTR,
    pub Pattern: windows_core::PCSTR,
    pub CaseSensitive: super::ULONG,
}
pub const EXT_TDF_PHYSICAL_CACHED: i32 = 4;
pub const EXT_TDF_PHYSICAL_DEFAULT: i32 = 2;
pub const EXT_TDF_PHYSICAL_MEMORY: i32 = 14;
pub const EXT_TDF_PHYSICAL_UNCACHED: i32 = 6;
pub const EXT_TDF_PHYSICAL_WRITE_COMBINED: i32 = 8;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXT_TYPED_DATA {
    pub Operation: EXT_TDOP,
    pub Flags: super::ULONG,
    pub InData: DEBUG_TYPED_DATA,
    pub OutData: DEBUG_TYPED_DATA,
    pub InStrIndex: super::ULONG,
    pub In32: super::ULONG,
    pub Out32: super::ULONG,
    pub In64: super::ULONG64,
    pub Out64: super::ULONG64,
    pub StrBufferIndex: super::ULONG,
    pub StrBufferChars: super::ULONG,
    pub StrCharsNeeded: super::ULONG,
    pub DataBufferIndex: super::ULONG,
    pub DataBufferBytes: super::ULONG,
    pub DataBytesNeeded: super::ULONG,
    pub Status: windows_core::HRESULT,
    pub Reserved: [super::ULONG64; 8],
}
#[cfg(feature = "shared")]
impl Default for EXT_TYPED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIELDS_DID_NOT_MATCH: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FIELD_INFO {
    pub fName: super::PUCHAR,
    pub printName: super::PUCHAR,
    pub size: super::ULONG,
    pub fOptions: super::ULONG,
    pub address: super::ULONG64,
    pub Anonymous: FIELD_INFO_0,
    pub TypeId: super::ULONG,
    pub FieldOffset: super::ULONG,
    pub BufferSize: super::ULONG,
    pub BitField: FIELD_INFO__BitField,
    pub _bitfield: u32,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl FIELD_INFO {
    pub fn fPointer(&self) -> super::ULONG {
        (self._bitfield) & 3
    }
    pub fn set_fPointer(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn fArray(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_fArray(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn fStruct(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_fStruct(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn fConstant(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_fConstant(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn fStatic(&self) -> super::ULONG {
        (self._bitfield >> 5) & 1
    }
    pub fn set_fStatic(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for FIELD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FIELD_INFO_0 {
    pub fieldCallBack: super::PVOID,
    pub pBuffer: super::PVOID,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for FIELD_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FIELD_INFO__BitField {
    pub Position: super::USHORT,
    pub Size: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CONTEXT_EX {
    pub Status: super::ULONG,
    pub ContextSize: super::ULONG,
    pub pContext: super::PVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CURRENT_PROCESS_ADDRESS {
    pub Processor: super::ULONG,
    pub CurrentThread: super::ULONG64,
    pub Address: super::ULONG64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CURRENT_THREAD_ADDRESS {
    pub Processor: super::ULONG,
    pub Address: super::ULONG64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_EXPRESSION_EX {
    pub Expression: windows_core::PCSTR,
    pub Remainder: windows_core::PCSTR,
    pub Value: super::ULONG64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_INPUT_LINE {
    pub Prompt: windows_core::PCSTR,
    pub Buffer: windows_core::PSTR,
    pub BufferSize: super::ULONG,
    pub InputSize: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_PEB_ADDRESS {
    pub CurrentThread: super::ULONG64,
    pub Address: super::ULONGLONG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_SET_SYMPATH {
    pub Args: windows_core::PCSTR,
    pub Result: windows_core::PSTR,
    pub Length: i32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_TEB_ADDRESS {
    pub Address: super::ULONGLONG,
}
pub const IG_DISASSEMBLE_BUFFER: i32 = 44;
pub const IG_DUMP_SYMBOL_INFO: i32 = 22;
pub const IG_FIND_FILE: i32 = 40;
pub const IG_GET_ANY_MODULE_IN_RANGE: i32 = 45;
pub const IG_GET_BUS_DATA: i32 = 20;
pub const IG_GET_CACHE_SIZE: i32 = 32;
pub const IG_GET_CLR_DATA_INTERFACE: i32 = 38;
pub const IG_GET_CONTEXT_EX: i32 = 48;
pub const IG_GET_CURRENT_PROCESS: i32 = 26;
pub const IG_GET_CURRENT_PROCESS_HANDLE: i32 = 28;
pub const IG_GET_CURRENT_THREAD: i32 = 25;
pub const IG_GET_DEBUGGER_DATA: i32 = 14;
pub const IG_GET_EXCEPTION_RECORD: i32 = 18;
pub const IG_GET_EXPRESSION_EX: i32 = 30;
pub const IG_GET_INPUT_LINE: i32 = 29;
pub const IG_GET_KERNEL_VERSION: i32 = 15;
pub const IG_GET_PEB_ADDRESS: i32 = 129;
pub const IG_GET_SET_SYMPATH: i32 = 17;
pub const IG_GET_TEB_ADDRESS: i32 = 128;
pub const IG_GET_THREAD_OS_INFO: i32 = 37;
pub const IG_GET_TYPE_SIZE: i32 = 27;
pub const IG_IS_PTR64: i32 = 19;
pub const IG_KD_CONTEXT: i32 = 1;
pub const IG_KSTACK_HELP: i32 = 10;
pub const IG_LOWMEM_CHECK: i32 = 23;
pub const IG_MATCH_PATTERN_A: i32 = 39;
pub const IG_OBSOLETE_PLACEHOLDER_36: i32 = 36;
pub const IG_PHYSICAL_TO_VIRTUAL: i32 = 47;
pub const IG_POINTER_SEARCH_PHYSICAL: i32 = 35;
pub const IG_QUERY_TARGET_INTERFACE: i32 = 42;
pub const IG_READ_CONTROL_SPACE: i32 = 2;
pub const IG_READ_IO_SPACE: i32 = 4;
pub const IG_READ_IO_SPACE_EX: i32 = 8;
pub const IG_READ_MSR: i32 = 12;
pub const IG_READ_PHYSICAL: i32 = 6;
pub const IG_READ_PHYSICAL_WITH_FLAGS: i32 = 33;
pub const IG_RELOAD_SYMBOLS: i32 = 16;
pub const IG_SEARCH_MEMORY: i32 = 24;
pub const IG_SET_BUS_DATA: i32 = 21;
pub const IG_SET_THREAD: i32 = 11;
pub const IG_TRANSLATE_VIRTUAL_TO_PHYSICAL: i32 = 31;
pub const IG_TYPED_DATA: i32 = 43;
pub const IG_TYPED_DATA_OBSOLETE: i32 = 41;
pub const IG_VIRTUAL_TO_PHYSICAL: i32 = 46;
pub const IG_WRITE_CONTROL_SPACE: i32 = 3;
pub const IG_WRITE_IO_SPACE: i32 = 5;
pub const IG_WRITE_IO_SPACE_EX: i32 = 9;
pub const IG_WRITE_MSR: i32 = 13;
pub const IG_WRITE_PHYSICAL: i32 = 7;
pub const IG_WRITE_PHYSICAL_WITH_FLAGS: i32 = 34;
pub const INCORRECT_VERSION_INFO: i32 = 7;
pub const INSUFFICIENT_SPACE_TO_COPY: i32 = 10;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE {
    pub Address: super::ULONG,
    pub Length: super::ULONG,
    pub Data: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE32 {
    pub Address: super::ULONG,
    pub Length: super::ULONG,
    pub Data: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE64 {
    pub Address: super::ULONG64,
    pub Length: super::ULONG,
    pub Data: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE_EX {
    pub Address: super::ULONG,
    pub Length: super::ULONG,
    pub Data: super::ULONG,
    pub InterfaceType: super::ULONG,
    pub BusNumber: super::ULONG,
    pub AddressSpace: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE_EX32 {
    pub Address: super::ULONG,
    pub Length: super::ULONG,
    pub Data: super::ULONG,
    pub InterfaceType: super::ULONG,
    pub BusNumber: super::ULONG,
    pub AddressSpace: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOSPACE_EX64 {
    pub Address: super::ULONG64,
    pub Length: super::ULONG,
    pub Data: super::ULONG,
    pub InterfaceType: super::ULONG,
    pub BusNumber: super::ULONG,
    pub AddressSpace: super::ULONG,
}
pub const KDBG_TAG: i32 = 1195525195;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KDDEBUGGER_DATA32 {
    pub Header: DBGKD_DEBUG_DATA_HEADER32,
    pub KernBase: super::ULONG,
    pub BreakpointWithStatus: super::ULONG,
    pub SavedContext: super::ULONG,
    pub ThCallbackStack: super::USHORT,
    pub NextCallback: super::USHORT,
    pub FramePointer: super::USHORT,
    pub _bitfield: u16,
    pub KiCallUserMode: super::ULONG,
    pub KeUserCallbackDispatcher: super::ULONG,
    pub PsLoadedModuleList: super::ULONG,
    pub PsActiveProcessHead: super::ULONG,
    pub PspCidTable: super::ULONG,
    pub ExpSystemResourcesList: super::ULONG,
    pub ExpPagedPoolDescriptor: super::ULONG,
    pub ExpNumberOfPagedPools: super::ULONG,
    pub KeTimeIncrement: super::ULONG,
    pub KeBugCheckCallbackListHead: super::ULONG,
    pub KiBugcheckData: super::ULONG,
    pub IopErrorLogListHead: super::ULONG,
    pub ObpRootDirectoryObject: super::ULONG,
    pub ObpTypeObjectType: super::ULONG,
    pub MmSystemCacheStart: super::ULONG,
    pub MmSystemCacheEnd: super::ULONG,
    pub MmSystemCacheWs: super::ULONG,
    pub MmPfnDatabase: super::ULONG,
    pub MmSystemPtesStart: super::ULONG,
    pub MmSystemPtesEnd: super::ULONG,
    pub MmSubsectionBase: super::ULONG,
    pub MmNumberOfPagingFiles: super::ULONG,
    pub MmLowestPhysicalPage: super::ULONG,
    pub MmHighestPhysicalPage: super::ULONG,
    pub MmNumberOfPhysicalPages: super::ULONG,
    pub MmMaximumNonPagedPoolInBytes: super::ULONG,
    pub MmNonPagedSystemStart: super::ULONG,
    pub MmNonPagedPoolStart: super::ULONG,
    pub MmNonPagedPoolEnd: super::ULONG,
    pub MmPagedPoolStart: super::ULONG,
    pub MmPagedPoolEnd: super::ULONG,
    pub MmPagedPoolInformation: super::ULONG,
    pub MmPageSize: super::ULONG,
    pub MmSizeOfPagedPoolInBytes: super::ULONG,
    pub MmTotalCommitLimit: super::ULONG,
    pub MmTotalCommittedPages: super::ULONG,
    pub MmSharedCommit: super::ULONG,
    pub MmDriverCommit: super::ULONG,
    pub MmProcessCommit: super::ULONG,
    pub MmPagedPoolCommit: super::ULONG,
    pub MmExtendedCommit: super::ULONG,
    pub MmZeroedPageListHead: super::ULONG,
    pub MmFreePageListHead: super::ULONG,
    pub MmStandbyPageListHead: super::ULONG,
    pub MmModifiedPageListHead: super::ULONG,
    pub MmModifiedNoWritePageListHead: super::ULONG,
    pub MmAvailablePages: super::ULONG,
    pub MmResidentAvailablePages: super::ULONG,
    pub PoolTrackTable: super::ULONG,
    pub NonPagedPoolDescriptor: super::ULONG,
    pub MmHighestUserAddress: super::ULONG,
    pub MmSystemRangeStart: super::ULONG,
    pub MmUserProbeAddress: super::ULONG,
    pub KdPrintCircularBuffer: super::ULONG,
    pub KdPrintCircularBufferEnd: super::ULONG,
    pub KdPrintWritePointer: super::ULONG,
    pub KdPrintRolloverCount: super::ULONG,
    pub MmLoadedUserImageList: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl KDDEBUGGER_DATA32 {
    pub fn PaeEnabled(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_PaeEnabled(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KDDEBUGGER_DATA64 {
    pub Header: DBGKD_DEBUG_DATA_HEADER64,
    pub KernBase: super::ULONG64,
    pub BreakpointWithStatus: super::ULONG64,
    pub SavedContext: super::ULONG64,
    pub ThCallbackStack: super::USHORT,
    pub NextCallback: super::USHORT,
    pub FramePointer: super::USHORT,
    pub _bitfield: u16,
    pub KiCallUserMode: super::ULONG64,
    pub KeUserCallbackDispatcher: super::ULONG64,
    pub PsLoadedModuleList: super::ULONG64,
    pub PsActiveProcessHead: super::ULONG64,
    pub PspCidTable: super::ULONG64,
    pub ExpSystemResourcesList: super::ULONG64,
    pub ExpPagedPoolDescriptor: super::ULONG64,
    pub ExpNumberOfPagedPools: super::ULONG64,
    pub KeTimeIncrement: super::ULONG64,
    pub KeBugCheckCallbackListHead: super::ULONG64,
    pub KiBugcheckData: super::ULONG64,
    pub IopErrorLogListHead: super::ULONG64,
    pub ObpRootDirectoryObject: super::ULONG64,
    pub ObpTypeObjectType: super::ULONG64,
    pub MmSystemCacheStart: super::ULONG64,
    pub MmSystemCacheEnd: super::ULONG64,
    pub MmSystemCacheWs: super::ULONG64,
    pub MmPfnDatabase: super::ULONG64,
    pub MmSystemPtesStart: super::ULONG64,
    pub MmSystemPtesEnd: super::ULONG64,
    pub MmSubsectionBase: super::ULONG64,
    pub MmNumberOfPagingFiles: super::ULONG64,
    pub MmLowestPhysicalPage: super::ULONG64,
    pub MmHighestPhysicalPage: super::ULONG64,
    pub MmNumberOfPhysicalPages: super::ULONG64,
    pub MmMaximumNonPagedPoolInBytes: super::ULONG64,
    pub MmNonPagedSystemStart: super::ULONG64,
    pub MmNonPagedPoolStart: super::ULONG64,
    pub MmNonPagedPoolEnd: super::ULONG64,
    pub MmPagedPoolStart: super::ULONG64,
    pub MmPagedPoolEnd: super::ULONG64,
    pub MmPagedPoolInformation: super::ULONG64,
    pub MmPageSize: super::ULONG64,
    pub MmSizeOfPagedPoolInBytes: super::ULONG64,
    pub MmTotalCommitLimit: super::ULONG64,
    pub MmTotalCommittedPages: super::ULONG64,
    pub MmSharedCommit: super::ULONG64,
    pub MmDriverCommit: super::ULONG64,
    pub MmProcessCommit: super::ULONG64,
    pub MmPagedPoolCommit: super::ULONG64,
    pub MmExtendedCommit: super::ULONG64,
    pub MmZeroedPageListHead: super::ULONG64,
    pub MmFreePageListHead: super::ULONG64,
    pub MmStandbyPageListHead: super::ULONG64,
    pub MmModifiedPageListHead: super::ULONG64,
    pub MmModifiedNoWritePageListHead: super::ULONG64,
    pub MmAvailablePages: super::ULONG64,
    pub MmResidentAvailablePages: super::ULONG64,
    pub PoolTrackTable: super::ULONG64,
    pub NonPagedPoolDescriptor: super::ULONG64,
    pub MmHighestUserAddress: super::ULONG64,
    pub MmSystemRangeStart: super::ULONG64,
    pub MmUserProbeAddress: super::ULONG64,
    pub KdPrintCircularBuffer: super::ULONG64,
    pub KdPrintCircularBufferEnd: super::ULONG64,
    pub KdPrintWritePointer: super::ULONG64,
    pub KdPrintRolloverCount: super::ULONG64,
    pub MmLoadedUserImageList: super::ULONG64,
    pub NtBuildLab: super::ULONG64,
    pub KiNormalSystemCall: super::ULONG64,
    pub KiProcessorBlock: super::ULONG64,
    pub MmUnloadedDrivers: super::ULONG64,
    pub MmLastUnloadedDriver: super::ULONG64,
    pub MmTriageActionTaken: super::ULONG64,
    pub MmSpecialPoolTag: super::ULONG64,
    pub KernelVerifier: super::ULONG64,
    pub MmVerifierData: super::ULONG64,
    pub MmAllocatedNonPagedPool: super::ULONG64,
    pub MmPeakCommitment: super::ULONG64,
    pub MmTotalCommitLimitMaximum: super::ULONG64,
    pub CmNtCSDVersion: super::ULONG64,
    pub MmPhysicalMemoryBlock: super::ULONG64,
    pub MmSessionBase: super::ULONG64,
    pub MmSessionSize: super::ULONG64,
    pub MmSystemParentTablePage: super::ULONG64,
    pub MmVirtualTranslationBase: super::ULONG64,
    pub OffsetKThreadNextProcessor: super::USHORT,
    pub OffsetKThreadTeb: super::USHORT,
    pub OffsetKThreadKernelStack: super::USHORT,
    pub OffsetKThreadInitialStack: super::USHORT,
    pub OffsetKThreadApcProcess: super::USHORT,
    pub OffsetKThreadState: super::USHORT,
    pub OffsetKThreadBStore: super::USHORT,
    pub OffsetKThreadBStoreLimit: super::USHORT,
    pub SizeEProcess: super::USHORT,
    pub OffsetEprocessPeb: super::USHORT,
    pub OffsetEprocessParentCID: super::USHORT,
    pub OffsetEprocessDirectoryTableBase: super::USHORT,
    pub SizePrcb: super::USHORT,
    pub OffsetPrcbDpcRoutine: super::USHORT,
    pub OffsetPrcbCurrentThread: super::USHORT,
    pub OffsetPrcbMhz: super::USHORT,
    pub OffsetPrcbCpuType: super::USHORT,
    pub OffsetPrcbVendorString: super::USHORT,
    pub OffsetPrcbProcStateContext: super::USHORT,
    pub OffsetPrcbNumber: super::USHORT,
    pub SizeEThread: super::USHORT,
    pub L1tfHighPhysicalBitIndex: super::UCHAR,
    pub L1tfSwizzleBitIndex: super::UCHAR,
    pub Padding0: super::ULONG,
    pub KdPrintCircularBufferPtr: super::ULONG64,
    pub KdPrintBufferSize: super::ULONG64,
    pub KeLoaderBlock: super::ULONG64,
    pub SizePcr: super::USHORT,
    pub OffsetPcrSelfPcr: super::USHORT,
    pub OffsetPcrCurrentPrcb: super::USHORT,
    pub OffsetPcrContainedPrcb: super::USHORT,
    pub OffsetPcrInitialBStore: super::USHORT,
    pub OffsetPcrBStoreLimit: super::USHORT,
    pub OffsetPcrInitialStack: super::USHORT,
    pub OffsetPcrStackLimit: super::USHORT,
    pub OffsetPrcbPcrPage: super::USHORT,
    pub OffsetPrcbProcStateSpecialReg: super::USHORT,
    pub GdtR0Code: super::USHORT,
    pub GdtR0Data: super::USHORT,
    pub GdtR0Pcr: super::USHORT,
    pub GdtR3Code: super::USHORT,
    pub GdtR3Data: super::USHORT,
    pub GdtR3Teb: super::USHORT,
    pub GdtLdt: super::USHORT,
    pub GdtTss: super::USHORT,
    pub Gdt64R3CmCode: super::USHORT,
    pub Gdt64R3CmTeb: super::USHORT,
    pub IopNumTriageDumpDataBlocks: super::ULONG64,
    pub IopTriageDumpDataBlocks: super::ULONG64,
    pub VfCrashDataBlock: super::ULONG64,
    pub MmBadPagesDetected: super::ULONG64,
    pub MmZeroedPageSingleBitErrorsDetected: super::ULONG64,
    pub EtwpDebuggerData: super::ULONG64,
    pub OffsetPrcbContext: super::USHORT,
    pub OffsetPrcbMaxBreakpoints: super::USHORT,
    pub OffsetPrcbMaxWatchpoints: super::USHORT,
    pub OffsetKThreadStackLimit: super::ULONG,
    pub OffsetKThreadStackBase: super::ULONG,
    pub OffsetKThreadQueueListEntry: super::ULONG,
    pub OffsetEThreadIrpList: super::ULONG,
    pub OffsetPrcbIdleThread: super::USHORT,
    pub OffsetPrcbNormalDpcState: super::USHORT,
    pub OffsetPrcbDpcStack: super::USHORT,
    pub OffsetPrcbIsrStack: super::USHORT,
    pub SizeKDPC_STACK_FRAME: super::USHORT,
    pub OffsetKPriQueueThreadListHead: super::USHORT,
    pub OffsetKThreadWaitReason: super::USHORT,
    pub Padding1: super::USHORT,
    pub PteBase: super::ULONG64,
    pub RetpolineStubFunctionTable: super::ULONG64,
    pub RetpolineStubFunctionTableSize: super::ULONG,
    pub RetpolineStubOffset: super::ULONG,
    pub RetpolineStubSize: super::ULONG,
    pub OffsetEProcessMmHotPatchContext: super::USHORT,
    pub OffsetKThreadShadowStackLimit: super::ULONG,
    pub OffsetKThreadShadowStackBase: super::ULONG,
    pub ShadowStackEnabled: super::ULONG64,
    pub PointerAuthMask: super::ULONG64,
    pub OffsetPrcbExceptionStack: super::USHORT,
    pub PointerIgnoreBits: super::ULONG64,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl KDDEBUGGER_DATA64 {
    pub fn PaeEnabled(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_PaeEnabled(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn KiBugCheckRecoveryActive(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_KiBugCheckRecoveryActive(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn PagingLevels(&self) -> super::USHORT {
        (self._bitfield >> 2) & 15
    }
    pub fn set_PagingLevels(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(15 << 2)) | ((value & 15) << 2);
    }
}
pub const KD_SECONDARY_VERSION_AMD64_CONTEXT: i32 = 2;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_1: i32 = 0;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_2: i32 = 1;
pub const KD_SECONDARY_VERSION_DEFAULT: i32 = 0;
#[cfg(feature = "shared")]
pub type LPEXT_API_VERSION = *mut EXT_API_VERSION;
pub const MEMORY_READ_ERROR: i32 = 1;
pub const NULL_FIELD_NAME: i32 = 6;
pub const NULL_SYM_DUMP_PARAM: i32 = 5;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBUSDATA = *mut BUSDATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDBGKD_DEBUG_DATA_HEADER32 = *mut DBGKD_DEBUG_DATA_HEADER32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDBGKD_DEBUG_DATA_HEADER64 = *mut DBGKD_DEBUG_DATA_HEADER64;
#[cfg(feature = "shared")]
pub type PDBGKD_GET_VERSION32 = *mut DBGKD_GET_VERSION32;
#[cfg(feature = "shared")]
pub type PDBGKD_GET_VERSION64 = *mut DBGKD_GET_VERSION64;
#[cfg(feature = "shared")]
pub type PDEBUG_TYPED_DATA = *mut DEBUG_TYPED_DATA;
#[cfg(feature = "shared")]
pub type PEXTSTACKTRACE = *mut EXTSTACKTRACE;
#[cfg(feature = "shared")]
pub type PEXTSTACKTRACE32 = *mut EXTSTACKTRACE32;
#[cfg(feature = "shared")]
pub type PEXTSTACKTRACE64 = *mut EXTSTACKTRACE64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEXT_FIND_FILE = *mut EXT_FIND_FILE;
#[cfg(feature = "shared")]
pub type PEXT_MATCH_PATTERN_A = *mut EXT_MATCH_PATTERN_A;
#[cfg(feature = "shared")]
pub type PEXT_TYPED_DATA = *mut EXT_TYPED_DATA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFIELD_INFO = *mut FIELD_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_CONTEXT_EX = *mut GET_CONTEXT_EX;
#[cfg(feature = "shared")]
pub type PGET_CURRENT_PROCESS_ADDRESS = *mut GET_CURRENT_PROCESS_ADDRESS;
#[cfg(feature = "shared")]
pub type PGET_CURRENT_THREAD_ADDRESS = *mut GET_CURRENT_THREAD_ADDRESS;
#[cfg(feature = "shared")]
pub type PGET_EXPRESSION_EX = *mut GET_EXPRESSION_EX;
#[cfg(feature = "shared")]
pub type PGET_INPUT_LINE = *mut GET_INPUT_LINE;
#[cfg(feature = "shared")]
pub type PGET_PEB_ADDRESS = *mut GET_PEB_ADDRESS;
pub type PGET_SET_SYMPATH = *mut GET_SET_SYMPATH;
#[cfg(feature = "shared")]
pub type PGET_TEB_ADDRESS = *mut GET_TEB_ADDRESS;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL {
    pub Address: super::ULONGLONG,
    pub BufLen: super::ULONG,
    pub Buf: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for PHYSICAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PHYSICAL_TO_VIRTUAL {
    pub Status: super::ULONG,
    pub Size: super::ULONG,
    pub PdeAddress: super::ULONG64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_WITH_FLAGS {
    pub Address: super::ULONGLONG,
    pub BufLen: super::ULONG,
    pub Flags: super::ULONG,
    pub Buf: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for PHYSICAL_WITH_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PHYS_FLAG_CACHED: i32 = 1;
pub const PHYS_FLAG_DEFAULT: i32 = 0;
pub const PHYS_FLAG_UNCACHED: i32 = 2;
pub const PHYS_FLAG_WRITE_COMBINED: i32 = 3;
#[cfg(feature = "shared")]
pub type PIOSPACE = *mut IOSPACE;
#[cfg(feature = "shared")]
pub type PIOSPACE32 = *mut IOSPACE32;
#[cfg(feature = "shared")]
pub type PIOSPACE64 = *mut IOSPACE64;
#[cfg(feature = "shared")]
pub type PIOSPACE_EX = *mut IOSPACE_EX;
#[cfg(feature = "shared")]
pub type PIOSPACE_EX32 = *mut IOSPACE_EX32;
#[cfg(feature = "shared")]
pub type PIOSPACE_EX64 = *mut IOSPACE_EX64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKDDEBUGGER_DATA32 = *mut KDDEBUGGER_DATA32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKDDEBUGGER_DATA64 = *mut KDDEBUGGER_DATA64;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_SEARCH_PHYSICAL {
    pub Offset: super::ULONG64,
    pub Length: super::ULONG64,
    pub PointerMin: super::ULONG64,
    pub PointerMax: super::ULONG64,
    pub Flags: super::ULONG,
    pub MatchOffsets: super::PULONG64,
    pub MatchOffsetsSize: super::ULONG,
    pub MatchOffsetsCount: super::ULONG,
}
pub type POOL_HEADER_FIELD_NAME = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union POOL_HEADER_SIZE_64 {
    pub Anonymous: POOL_HEADER_SIZE_64_0,
    pub Ulong1: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for POOL_HEADER_SIZE_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POOL_HEADER_SIZE_64_0 {
    pub UnsafePrevSize: super::UCHAR,
    pub Unused1: super::UCHAR,
    pub UnsafeSize: super::UCHAR,
    pub UnsafePoolType: super::UCHAR,
}
#[cfg(feature = "shared")]
pub type PPHYSICAL = *mut PHYSICAL;
#[cfg(feature = "shared")]
pub type PPHYSICAL_TO_VIRTUAL = *mut PHYSICAL_TO_VIRTUAL;
#[cfg(feature = "shared")]
pub type PPHYSICAL_WITH_FLAGS = *mut PHYSICAL_WITH_FLAGS;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PPOINTER_SEARCH_PHYSICAL = *mut POINTER_SEARCH_PHYSICAL;
#[cfg(feature = "shared")]
pub type PPOOL_HEADER_SIZE_64 = *mut POOL_HEADER_SIZE_64;
#[cfg(feature = "shared")]
pub type PPROCESSORINFO = *mut PROCESSORINFO;
#[cfg(feature = "shared")]
pub type PREADCONTROLSPACE = *mut READCONTROLSPACE;
#[cfg(feature = "shared")]
pub type PREADCONTROLSPACE32 = *mut READCONTROLSPACE32;
#[cfg(feature = "shared")]
pub type PREADCONTROLSPACE64 = *mut READCONTROLSPACE64;
#[cfg(feature = "shared")]
pub type PREAD_WRITE_MSR = *mut READ_WRITE_MSR;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSORINFO {
    pub Processor: super::USHORT,
    pub NumberProcessors: super::USHORT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSEARCHMEMORY = *mut SEARCHMEMORY;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSYM_DUMP_FIELD_CALLBACK = Option<unsafe extern "system" fn(pfield: *mut FIELD_INFO, usercontext: super::PVOID) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSYM_DUMP_PARAM = *mut SYM_DUMP_PARAM;
#[cfg(feature = "shared")]
pub type PTRANSLATE_VIRTUAL_TO_PHYSICAL = *mut TRANSLATE_VIRTUAL_TO_PHYSICAL;
pub const PTR_SEARCH_NO_SYMBOL_CHECK: u32 = 2147483648;
pub const PTR_SEARCH_PHYS_ALL_HITS: i32 = 1;
pub const PTR_SEARCH_PHYS_PTE: i32 = 2;
pub const PTR_SEARCH_PHYS_RANGE_CHECK_ONLY: i32 = 4;
pub const PTR_SEARCH_PHYS_SIZE_MASK: i32 = 120;
pub const PTR_SEARCH_PHYS_SIZE_SHIFT: i32 = 3;
#[cfg(feature = "shared")]
pub type PVIRTUAL_TO_PHYSICAL = *mut VIRTUAL_TO_PHYSICAL;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type PWDBGEXTS_CLR_DATA_INTERFACE = *mut WDBGEXTS_CLR_DATA_INTERFACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWDBGEXTS_DISASSEMBLE_BUFFER = *mut WDBGEXTS_DISASSEMBLE_BUFFER;
#[cfg(feature = "shared")]
pub type PWDBGEXTS_MODULE_IN_RANGE = *mut WDBGEXTS_MODULE_IN_RANGE;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type PWDBGEXTS_QUERY_INTERFACE = *mut WDBGEXTS_QUERY_INTERFACE;
#[cfg(feature = "shared")]
pub type PWDBGEXTS_THREAD_OS_INFO = *mut WDBGEXTS_THREAD_OS_INFO;
#[cfg(feature = "shared")]
pub type PWINDBG_CHECK_CONTROL_C = Option<unsafe extern "system" fn() -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_CHECK_VERSION = Option<unsafe extern "system" fn() -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_DISASM = Option<unsafe extern "system" fn(lpoffset: *mut super::ULONG_PTR, lpbuffer: windows_core::PCSTR, fshoweffectiveaddress: super::ULONG) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_DISASM32 = Option<unsafe extern "system" fn(lpoffset: *mut super::ULONG, lpbuffer: windows_core::PCSTR, fshoweffectiveaddress: super::ULONG) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_DISASM64 = Option<unsafe extern "system" fn(lpoffset: *mut super::ULONG64, lpbuffer: windows_core::PCSTR, fshoweffectiveaddress: super::ULONG) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_EXTENSION_API_VERSION = Option<unsafe extern "system" fn() -> LPEXT_API_VERSION>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_EXTENSION_ROUTINE = Option<unsafe extern "system" fn(hcurrentprocess: super::HANDLE, hcurrentthread: super::HANDLE, dwcurrentpc: super::ULONG, dwprocessor: super::ULONG, lpargumentstring: windows_core::PCSTR)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_EXTENSION_ROUTINE32 = Option<unsafe extern "system" fn(hcurrentprocess: super::HANDLE, hcurrentthread: super::HANDLE, dwcurrentpc: super::ULONG, dwprocessor: super::ULONG, lpargumentstring: windows_core::PCSTR)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_EXTENSION_ROUTINE64 = Option<unsafe extern "system" fn(hcurrentprocess: super::HANDLE, hcurrentthread: super::HANDLE, dwcurrentpc: super::ULONG64, dwprocessor: super::ULONG, lpargumentstring: windows_core::PCSTR)>;
#[cfg(feature = "shared")]
pub type PWINDBG_GET_EXPRESSION = Option<unsafe extern "system" fn(lpexpression: windows_core::PCSTR) -> super::ULONG_PTR>;
#[cfg(feature = "shared")]
pub type PWINDBG_GET_EXPRESSION32 = Option<unsafe extern "system" fn(lpexpression: windows_core::PCSTR) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_GET_EXPRESSION64 = Option<unsafe extern "system" fn(lpexpression: windows_core::PCSTR) -> super::ULONG64>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_GET_SYMBOL = Option<unsafe extern "system" fn(offset: super::PVOID, pchbuffer: super::PCHAR, pdisplacement: *mut super::ULONG_PTR)>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINDBG_GET_SYMBOL32 = Option<unsafe extern "system" fn(offset: super::ULONG, pchbuffer: super::PCHAR, pdisplacement: super::PULONG)>;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PWINDBG_GET_SYMBOL64 = Option<unsafe extern "system" fn(offset: super::ULONG64, pchbuffer: super::PCHAR, pdisplacement: super::PULONG64)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_GET_THREAD_CONTEXT_ROUTINE = Option<unsafe extern "system" fn(processor: super::ULONG, lpcontext: super::PCONTEXT, cbsizeofcontext: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_IOCTL_ROUTINE = Option<unsafe extern "system" fn(ioctltype: super::USHORT, lpvdata: super::PVOID, cbsize: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINDBG_OLDKD_READ_PHYSICAL_MEMORY = Option<unsafe extern "system" fn(address: super::ULONGLONG, buffer: super::PVOID, count: super::ULONG, bytesread: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY = Option<unsafe extern "system" fn(address: super::ULONGLONG, buffer: super::PVOID, length: super::ULONG, byteswritten: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(offset: super::ULONG_PTR, lpbuffer: super::PVOID, cb: super::ULONG, lpcbbytesread: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE32 = Option<unsafe extern "system" fn(offset: super::ULONG, lpbuffer: super::PVOID, cb: super::ULONG, lpcbbytesread: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(offset: super::ULONG64, lpbuffer: super::PVOID, cb: super::ULONG, lpcbbytesread: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWINDBG_SET_THREAD_CONTEXT_ROUTINE = Option<unsafe extern "system" fn(processor: super::ULONG, lpcontext: super::PCONTEXT, cbsizeofcontext: super::ULONG) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_STACKTRACE_ROUTINE = Option<unsafe extern "system" fn(framepointer: super::ULONG, stackpointer: super::ULONG, programcounter: super::ULONG, stackframes: PEXTSTACKTRACE, frames: super::ULONG) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_STACKTRACE_ROUTINE32 = Option<unsafe extern "system" fn(framepointer: super::ULONG, stackpointer: super::ULONG, programcounter: super::ULONG, stackframes: PEXTSTACKTRACE32, frames: super::ULONG) -> super::ULONG>;
#[cfg(feature = "shared")]
pub type PWINDBG_STACKTRACE_ROUTINE64 = Option<unsafe extern "system" fn(framepointer: super::ULONG64, stackpointer: super::ULONG64, programcounter: super::ULONG64, stackframes: PEXTSTACKTRACE64, frames: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(offset: super::ULONG_PTR, lpbuffer: super::LPCVOID, cb: super::ULONG, lpcbbyteswritten: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32 = Option<unsafe extern "system" fn(offset: super::ULONG, lpbuffer: super::LPCVOID, cb: super::ULONG, lpcbbyteswritten: super::PULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(offset: super::ULONG64, lpbuffer: super::LPCVOID, cb: super::ULONG, lpcbbyteswritten: super::PULONG) -> super::ULONG>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READCONTROLSPACE {
    pub Processor: super::USHORT,
    pub Address: super::ULONG,
    pub BufLen: super::ULONG,
    pub Buf: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for READCONTROLSPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READCONTROLSPACE32 {
    pub Processor: super::USHORT,
    pub Address: super::ULONG,
    pub BufLen: super::ULONG,
    pub Buf: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for READCONTROLSPACE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READCONTROLSPACE64 {
    pub Processor: super::USHORT,
    pub Address: super::ULONG64,
    pub BufLen: super::ULONG,
    pub Buf: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for READCONTROLSPACE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_WRITE_MSR {
    pub Msr: super::ULONG,
    pub Value: super::LONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEARCHMEMORY {
    pub SearchAddress: super::ULONG64,
    pub SearchLength: super::ULONG64,
    pub FoundAddress: super::ULONG64,
    pub PatternLength: super::ULONG,
    pub Pattern: super::PVOID,
}
pub const SYMBOL_TYPE_INDEX_NOT_FOUND: i32 = 2;
pub const SYMBOL_TYPE_INFO_NOT_FOUND: i32 = 3;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYM_DUMP_PARAM {
    pub size: super::ULONG,
    pub sName: super::PUCHAR,
    pub Options: super::ULONG,
    pub addr: super::ULONG64,
    pub listLink: PFIELD_INFO,
    pub Anonymous: SYM_DUMP_PARAM_0,
    pub CallbackRoutine: PSYM_DUMP_FIELD_CALLBACK,
    pub nFields: super::ULONG,
    pub Fields: PFIELD_INFO,
    pub ModBase: super::ULONG64,
    pub TypeId: super::ULONG,
    pub TypeSize: super::ULONG,
    pub BufferSize: super::ULONG,
    pub _bitfield: u32,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl SYM_DUMP_PARAM {
    pub fn fPointer(&self) -> super::ULONG {
        (self._bitfield) & 3
    }
    pub fn set_fPointer(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn fArray(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_fArray(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn fStruct(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_fStruct(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn fConstant(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_fConstant(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(134217727 << 5)) | ((value & 134217727) << 5);
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for SYM_DUMP_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union SYM_DUMP_PARAM_0 {
    pub Context: super::PVOID,
    pub pBuffer: super::PVOID,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for SYM_DUMP_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSLATE_VIRTUAL_TO_PHYSICAL {
    pub Virtual: super::ULONG64,
    pub Physical: super::ULONG64,
}
pub const UNAVAILABLE_ERROR: i32 = 12;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_TO_PHYSICAL {
    pub Status: super::ULONG,
    pub Size: super::ULONG,
    pub PdeAddress: super::ULONG64,
    pub Virtual: super::ULONG64,
    pub Physical: super::ULONG64,
}
pub const WDBGEXTS_ADDRESS_DEFAULT: i32 = 0;
pub const WDBGEXTS_ADDRESS_RESERVED0: u32 = 2147483648;
pub const WDBGEXTS_ADDRESS_SEG16: i32 = 1;
pub const WDBGEXTS_ADDRESS_SEG32: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDBGEXTS_CLR_DATA_INTERFACE {
    pub Iid: *const super::IID,
    pub Iface: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WDBGEXTS_DISASSEMBLE_BUFFER {
    pub InOffset: super::ULONG64,
    pub OutOffset: super::ULONG64,
    pub AddrFlags: super::ULONG,
    pub FormatFlags: super::ULONG,
    pub DataBufferBytes: super::ULONG,
    pub DisasmBufferChars: super::ULONG,
    pub DataBuffer: super::PVOID,
    pub DisasmBuffer: windows_core::PWSTR,
    pub Reserved0: [super::ULONG64; 3],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WDBGEXTS_DISASSEMBLE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const WDBGEXTS_MAXSIZE_T: super::SIZE_T = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const WDBGEXTS_MAXSIZE_T: super::SIZE_T = 18446744073709551615u64 as usize;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDBGEXTS_MODULE_IN_RANGE {
    pub Start: super::ULONG64,
    pub End: super::ULONG64,
    pub FoundModBase: super::ULONG64,
    pub FoundModSize: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDBGEXTS_QUERY_INTERFACE {
    pub Iid: *const super::IID,
    pub Iface: super::PVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDBGEXTS_THREAD_OS_INFO {
    pub ThreadId: super::ULONG,
    pub ExitStatus: super::ULONG,
    pub PriorityClass: super::ULONG,
    pub Priority: super::ULONG,
    pub CreateTime: super::ULONG64,
    pub ExitTime: super::ULONG64,
    pub KernelTime: super::ULONG64,
    pub UserTime: super::ULONG64,
    pub StartOffset: super::ULONG64,
    pub Affinity: super::ULONG64,
}
