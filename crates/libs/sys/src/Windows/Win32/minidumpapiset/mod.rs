#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn MiniDumpReadDumpStream(baseofdump : super::PVOID, streamnumber : super::ULONG, dir : *mut PMINIDUMP_DIRECTORY, streampointer : *mut super::PVOID, streamsize : *mut super::ULONG) -> windows_sys::core::BOOL);
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub union CPU_INFORMATION {
    pub X86CpuInfo: CPU_INFORMATION_0,
    pub OtherCpuInfo: CPU_INFORMATION_1,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for CPU_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct CPU_INFORMATION_0 {
    pub VendorId: [super::ULONG32; 3],
    pub VersionInformation: super::ULONG32,
    pub FeatureInformation: super::ULONG32,
    pub AMDExtendedCpuFeatures: super::ULONG32,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for CPU_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct CPU_INFORMATION_1 {
    pub ProcessorFeatures: [super::ULONG64; 2],
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for CPU_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CancelCallback: MINIDUMP_CALLBACK_TYPE = 6;
pub const CommentStreamA: MINIDUMP_STREAM_TYPE = 10;
pub const CommentStreamW: MINIDUMP_STREAM_TYPE = 11;
pub const CompressedMemoryStream: MINIDUMP_STREAM_TYPE = 25;
pub const CompressedMemoryStreamFinishCallback: MINIDUMP_CALLBACK_TYPE = 22;
pub const CompressedMemoryStreamSQL: MINIDUMP_STREAM_TYPE = 28672;
pub const CompressedMemoryStreamStartCallback: MINIDUMP_CALLBACK_TYPE = 21;
pub const ExceptionStream: MINIDUMP_STREAM_TYPE = 6;
pub const FunctionTableStream: MINIDUMP_STREAM_TYPE = 13;
pub const HandleDataStream: MINIDUMP_STREAM_TYPE = 12;
pub const HandleOperationListStream: MINIDUMP_STREAM_TYPE = 18;
pub const IncludeModuleCallback: MINIDUMP_CALLBACK_TYPE = 4;
pub const IncludeThreadCallback: MINIDUMP_CALLBACK_TYPE = 3;
pub const IncludeVmRegionCallback: MINIDUMP_CALLBACK_TYPE = 10;
pub const IoFinishCallback: MINIDUMP_CALLBACK_TYPE = 13;
pub const IoStartCallback: MINIDUMP_CALLBACK_TYPE = 11;
pub const IoWriteAllCallback: MINIDUMP_CALLBACK_TYPE = 12;
pub const IptTraceStream: MINIDUMP_STREAM_TYPE = 23;
pub const IsProcessSnapshotCallback: MINIDUMP_CALLBACK_TYPE = 16;
pub const JavaScriptDataStream: MINIDUMP_STREAM_TYPE = 20;
pub const KernelMinidumpStatusCallback: MINIDUMP_CALLBACK_TYPE = 8;
pub const LastReservedStream: MINIDUMP_STREAM_TYPE = 65535;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0,
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_CALLBACK_OUTPUT_0 {
    pub ModuleWriteFlags: super::ULONG,
    pub ThreadWriteFlags: super::ULONG,
    pub SecondaryFlags: super::ULONG,
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0_0,
    pub Anonymous2: MINIDUMP_CALLBACK_OUTPUT_0_1,
    pub Handle: super::HANDLE,
    pub Anonymous3: MINIDUMP_CALLBACK_OUTPUT_0_2,
    pub Anonymous4: MINIDUMP_CALLBACK_OUTPUT_0_3,
    pub Anonymous5: MINIDUMP_CALLBACK_OUTPUT_0_4,
    pub CompressedMemoryStreamStart: MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK,
    pub Status: windows_sys::core::HRESULT,
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_0 {
    pub MemoryBase: super::ULONG64,
    pub MemorySize: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_1 {
    pub CheckCancel: windows_sys::core::BOOL,
    pub Cancel: windows_sys::core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_2 {
    pub VmRegion: MINIDUMP_MEMORY_INFO,
    pub Continue: windows_sys::core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_3 {
    pub VmQueryStatus: windows_sys::core::HRESULT,
    pub VmQueryResult: MINIDUMP_MEMORY_INFO,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_4 {
    pub VmReadStatus: windows_sys::core::HRESULT,
    pub VmReadBytesCompleted: super::ULONG,
}
pub type MINIDUMP_CALLBACK_TYPE = i32;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK {
    pub CompressedStreamSize: super::ULONG64,
    pub StreamCompressionRate: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK {
    pub MaxParallelism: super::ULONG,
    pub MaxTransferSize: super::ULONG,
    pub NumaNodeInfoArray: super::PNUMA_NODE_RELATIONSHIP,
    pub NumaNodeInfoArraySize: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_DIRECTORY {
    pub StreamType: super::ULONG32,
    pub Location: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_EXCEPTION {
    pub ExceptionCode: super::ULONG32,
    pub ExceptionFlags: super::ULONG32,
    pub ExceptionRecord: super::ULONG64,
    pub ExceptionAddress: super::ULONG64,
    pub NumberParameters: super::ULONG32,
    pub __unusedAlignment: super::ULONG32,
    pub ExceptionInformation: [super::ULONG64; 15],
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_EXCEPTION_INFORMATION {
    pub ThreadId: super::DWORD,
    pub ExceptionPointers: super::PEXCEPTION_POINTERS,
    pub ClientPointers: windows_sys::core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_EXCEPTION_INFORMATION64 {
    pub ThreadId: super::DWORD,
    pub ExceptionRecord: super::ULONG64,
    pub ContextRecord: super::ULONG64,
    pub ClientPointers: windows_sys::core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_EXCEPTION_STREAM {
    pub ThreadId: super::ULONG32,
    pub __alignment: super::ULONG32,
    pub ExceptionRecord: MINIDUMP_EXCEPTION,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    pub MinimumAddress: super::ULONG64,
    pub MaximumAddress: super::ULONG64,
    pub BaseAddress: super::ULONG64,
    pub EntryCount: super::ULONG32,
    pub SizeOfAlignPad: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_FUNCTION_TABLE_STREAM {
    pub SizeOfHeader: super::ULONG32,
    pub SizeOfDescriptor: super::ULONG32,
    pub SizeOfNativeDescriptor: super::ULONG32,
    pub SizeOfFunctionEntry: super::ULONG32,
    pub NumberOfDescriptors: super::ULONG32,
    pub SizeOfAlignPad: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_DATA_STREAM {
    pub SizeOfHeader: super::ULONG32,
    pub SizeOfDescriptor: super::ULONG32,
    pub NumberOfDescriptors: super::ULONG32,
    pub Reserved: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_DESCRIPTOR {
    pub Handle: super::ULONG64,
    pub TypeNameRva: RVA,
    pub ObjectNameRva: RVA,
    pub Attributes: super::ULONG32,
    pub GrantedAccess: super::ULONG32,
    pub HandleCount: super::ULONG32,
    pub PointerCount: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_DESCRIPTOR_2 {
    pub Handle: super::ULONG64,
    pub TypeNameRva: RVA,
    pub ObjectNameRva: RVA,
    pub Attributes: super::ULONG32,
    pub GrantedAccess: super::ULONG32,
    pub HandleCount: super::ULONG32,
    pub PointerCount: super::ULONG32,
    pub ObjectInfoRva: RVA,
    pub Reserved0: super::ULONG32,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type MINIDUMP_HANDLE_DESCRIPTOR_N = MINIDUMP_HANDLE_DESCRIPTOR_2;
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION {
    pub NextInfoRva: RVA,
    pub InfoType: super::ULONG32,
    pub SizeOfInfo: super::ULONG32,
}
pub type MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = i32;
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_OPERATION_LIST {
    pub SizeOfHeader: super::ULONG32,
    pub SizeOfEntry: super::ULONG32,
    pub NumberOfEntries: super::ULONG32,
    pub Reserved: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_HEADER {
    pub Signature: super::ULONG32,
    pub Version: super::ULONG32,
    pub NumberOfStreams: super::ULONG32,
    pub StreamDirectoryRva: RVA,
    pub CheckSum: super::ULONG32,
    pub Anonymous: MINIDUMP_HEADER_0,
    pub Flags: super::ULONG64,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_HEADER_0 {
    pub Reserved: super::ULONG32,
    pub TimeDateStamp: super::ULONG32,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    pub BaseOfImage: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    pub ThreadId: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_IO_CALLBACK {
    pub Handle: super::HANDLE,
    pub Offset: super::ULONG64,
    pub Buffer: super::PVOID,
    pub BufferBytes: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR {
    pub DataSize: super::ULONG32,
    pub Rva: RVA,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR64 {
    pub DataSize: super::ULONG64,
    pub Rva: RVA64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MEMORY64_LIST {
    pub NumberOfMemoryRanges: super::ULONG64,
    pub BaseRva: RVA64,
    pub MemoryRanges: [MINIDUMP_MEMORY_DESCRIPTOR64; 0],
}
#[cfg(feature = "shared")]
impl Default for MINIDUMP_MEMORY64_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR {
    pub StartOfMemoryRange: super::ULONG64,
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR64 {
    pub StartOfMemoryRange: super::ULONG64,
    pub DataSize: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_INFO {
    pub BaseAddress: super::ULONG64,
    pub AllocationBase: super::ULONG64,
    pub AllocationProtect: super::ULONG32,
    pub __alignment1: super::ULONG32,
    pub RegionSize: super::ULONG64,
    pub State: super::ULONG32,
    pub Protect: super::ULONG32,
    pub Type: super::ULONG32,
    pub __alignment2: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_INFO_LIST {
    pub SizeOfHeader: super::ULONG,
    pub SizeOfEntry: super::ULONG,
    pub NumberOfEntries: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MEMORY_LIST {
    pub NumberOfMemoryRanges: super::ULONG32,
    pub MemoryRanges: [MINIDUMP_MEMORY_DESCRIPTOR; 0],
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_MEMORY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_MISC1_PROCESSOR_POWER_INFO: i32 = 4;
pub const MINIDUMP_MISC1_PROCESS_ID: i32 = 1;
pub const MINIDUMP_MISC1_PROCESS_TIMES: i32 = 2;
pub const MINIDUMP_MISC3_PROCESS_EXECUTE_FLAGS: i32 = 32;
pub const MINIDUMP_MISC3_PROCESS_INTEGRITY: i32 = 16;
pub const MINIDUMP_MISC3_PROTECTED_PROCESS: i32 = 128;
pub const MINIDUMP_MISC3_TIMEZONE: i32 = 64;
pub const MINIDUMP_MISC4_BUILDSTRING: i32 = 256;
pub const MINIDUMP_MISC5_PROCESS_COOKIE: i32 = 512;
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MISC_INFO {
    pub SizeOfInfo: super::ULONG32,
    pub Flags1: super::ULONG32,
    pub ProcessId: super::ULONG32,
    pub ProcessCreateTime: super::ULONG32,
    pub ProcessUserTime: super::ULONG32,
    pub ProcessKernelTime: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MISC_INFO_2 {
    pub SizeOfInfo: super::ULONG32,
    pub Flags1: super::ULONG32,
    pub ProcessId: super::ULONG32,
    pub ProcessCreateTime: super::ULONG32,
    pub ProcessUserTime: super::ULONG32,
    pub ProcessKernelTime: super::ULONG32,
    pub ProcessorMaxMhz: super::ULONG32,
    pub ProcessorCurrentMhz: super::ULONG32,
    pub ProcessorMhzLimit: super::ULONG32,
    pub ProcessorMaxIdleState: super::ULONG32,
    pub ProcessorCurrentIdleState: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MISC_INFO_3 {
    pub SizeOfInfo: super::ULONG32,
    pub Flags1: super::ULONG32,
    pub ProcessId: super::ULONG32,
    pub ProcessCreateTime: super::ULONG32,
    pub ProcessUserTime: super::ULONG32,
    pub ProcessKernelTime: super::ULONG32,
    pub ProcessorMaxMhz: super::ULONG32,
    pub ProcessorCurrentMhz: super::ULONG32,
    pub ProcessorMhzLimit: super::ULONG32,
    pub ProcessorMaxIdleState: super::ULONG32,
    pub ProcessorCurrentIdleState: super::ULONG32,
    pub ProcessIntegrityLevel: super::ULONG32,
    pub ProcessExecuteFlags: super::ULONG32,
    pub ProtectedProcess: super::ULONG32,
    pub TimeZoneId: super::ULONG32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MISC_INFO_4 {
    pub SizeOfInfo: super::ULONG32,
    pub Flags1: super::ULONG32,
    pub ProcessId: super::ULONG32,
    pub ProcessCreateTime: super::ULONG32,
    pub ProcessUserTime: super::ULONG32,
    pub ProcessKernelTime: super::ULONG32,
    pub ProcessorMaxMhz: super::ULONG32,
    pub ProcessorCurrentMhz: super::ULONG32,
    pub ProcessorMhzLimit: super::ULONG32,
    pub ProcessorMaxIdleState: super::ULONG32,
    pub ProcessorCurrentIdleState: super::ULONG32,
    pub ProcessIntegrityLevel: super::ULONG32,
    pub ProcessExecuteFlags: super::ULONG32,
    pub ProtectedProcess: super::ULONG32,
    pub TimeZoneId: super::ULONG32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BuildString: [super::WCHAR; 260],
    pub DbgBldStr: [super::WCHAR; 40],
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for MINIDUMP_MISC_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MISC_INFO_5 {
    pub SizeOfInfo: super::ULONG32,
    pub Flags1: super::ULONG32,
    pub ProcessId: super::ULONG32,
    pub ProcessCreateTime: super::ULONG32,
    pub ProcessUserTime: super::ULONG32,
    pub ProcessKernelTime: super::ULONG32,
    pub ProcessorMaxMhz: super::ULONG32,
    pub ProcessorCurrentMhz: super::ULONG32,
    pub ProcessorMhzLimit: super::ULONG32,
    pub ProcessorMaxIdleState: super::ULONG32,
    pub ProcessorCurrentIdleState: super::ULONG32,
    pub ProcessIntegrityLevel: super::ULONG32,
    pub ProcessExecuteFlags: super::ULONG32,
    pub ProtectedProcess: super::ULONG32,
    pub TimeZoneId: super::ULONG32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BuildString: [super::WCHAR; 260],
    pub DbgBldStr: [super::WCHAR; 40],
    pub XStateData: XSTATE_CONFIG_FEATURE_MSC_INFO,
    pub ProcessCookie: super::ULONG32,
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for MINIDUMP_MISC_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type MINIDUMP_MISC_INFO_N = MINIDUMP_MISC_INFO_5;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "verrsrc"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MODULE {
    pub BaseOfImage: super::ULONG64,
    pub SizeOfImage: super::ULONG32,
    pub CheckSum: super::ULONG32,
    pub TimeDateStamp: super::ULONG32,
    pub ModuleNameRva: RVA,
    pub VersionInfo: super::VS_FIXEDFILEINFO,
    pub CvRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    pub MiscRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    pub Reserved0: super::ULONG64,
    pub Reserved1: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "verrsrc", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MODULE_CALLBACK {
    pub FullPath: super::PWCHAR,
    pub BaseOfImage: super::ULONG64,
    pub SizeOfImage: super::ULONG,
    pub CheckSum: super::ULONG,
    pub TimeDateStamp: super::ULONG,
    pub VersionInfo: super::VS_FIXEDFILEINFO,
    pub CvRecord: super::PVOID,
    pub SizeOfCvRecord: super::ULONG,
    pub MiscRecord: super::PVOID,
    pub SizeOfMiscRecord: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "verrsrc"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MODULE_LIST {
    pub NumberOfModules: super::ULONG32,
    pub Modules: [MINIDUMP_MODULE; 0],
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "verrsrc"))]
impl Default for MINIDUMP_MODULE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_PROCESS_VM_COUNTERS: i32 = 1;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_1 {
    pub Revision: super::USHORT,
    pub PageFaultCount: super::ULONG,
    pub PeakWorkingSetSize: super::ULONG64,
    pub WorkingSetSize: super::ULONG64,
    pub QuotaPeakPagedPoolUsage: super::ULONG64,
    pub QuotaPagedPoolUsage: super::ULONG64,
    pub QuotaPeakNonPagedPoolUsage: super::ULONG64,
    pub QuotaNonPagedPoolUsage: super::ULONG64,
    pub PagefileUsage: super::ULONG64,
    pub PeakPagefileUsage: super::ULONG64,
    pub PrivateUsage: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_2 {
    pub Revision: super::USHORT,
    pub Flags: super::USHORT,
    pub PageFaultCount: super::ULONG,
    pub PeakWorkingSetSize: super::ULONG64,
    pub WorkingSetSize: super::ULONG64,
    pub QuotaPeakPagedPoolUsage: super::ULONG64,
    pub QuotaPagedPoolUsage: super::ULONG64,
    pub QuotaPeakNonPagedPoolUsage: super::ULONG64,
    pub QuotaNonPagedPoolUsage: super::ULONG64,
    pub PagefileUsage: super::ULONG64,
    pub PeakPagefileUsage: super::ULONG64,
    pub PeakVirtualSize: super::ULONG64,
    pub VirtualSize: super::ULONG64,
    pub PrivateUsage: super::ULONG64,
    pub PrivateWorkingSetSize: super::ULONG64,
    pub SharedCommitUsage: super::ULONG64,
    pub JobSharedCommitUsage: super::ULONG64,
    pub JobPrivateCommitUsage: super::ULONG64,
    pub JobPeakPrivateCommitUsage: super::ULONG64,
    pub JobPrivateCommitLimit: super::ULONG64,
    pub JobTotalCommitLimit: super::ULONG64,
}
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX: i32 = 4;
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX2: i32 = 8;
pub const MINIDUMP_PROCESS_VM_COUNTERS_JOB: i32 = 16;
#[cfg(feature = "shared")]
pub type MINIDUMP_PROCESS_VM_COUNTERS_N = MINIDUMP_PROCESS_VM_COUNTERS_2;
pub const MINIDUMP_PROCESS_VM_COUNTERS_VIRTUALSIZE: i32 = 2;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    pub Offset: super::ULONG64,
    pub Bytes: super::ULONG,
    pub FailureStatus: windows_sys::core::HRESULT,
}
pub type MINIDUMP_SECONDARY_FLAGS = i32;
pub const MINIDUMP_SIGNATURE: i32 = 1347241037;
pub type MINIDUMP_STREAM_TYPE = i32;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_STRING {
    pub Length: super::ULONG32,
    pub Buffer: [super::WCHAR; 0],
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_SYSMEMINFO1_BASICPERF: i32 = 2;
pub const MINIDUMP_SYSMEMINFO1_FILECACHE_TRANSITIONREPURPOSECOUNT_FLAGS: i32 = 1;
pub const MINIDUMP_SYSMEMINFO1_PERF_CCTOTALDIRTYPAGES_CCDIRTYPAGETHRESHOLD: i32 = 4;
pub const MINIDUMP_SYSMEMINFO1_PERF_MDLPAGESALLOCATED_PFNDATABASECOMMITTEDPAGES: i32 = 16;
pub const MINIDUMP_SYSMEMINFO1_PERF_RESIDENTAVAILABLEPAGES_SHAREDCOMMITPAGES: i32 = 8;
pub const MINIDUMP_SYSMEMINFO1_PERF_SYSTEMPAGETABLECOMMITTEDPAGES_CONTIGUOUSPAGESALLOCATED: i32 = 32;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_BASIC_INFORMATION {
    pub TimerResolution: super::ULONG,
    pub PageSize: super::ULONG,
    pub NumberOfPhysicalPages: super::ULONG,
    pub LowestPhysicalPageNumber: super::ULONG,
    pub HighestPhysicalPageNumber: super::ULONG,
    pub AllocationGranularity: super::ULONG,
    pub MinimumUserModeAddress: super::ULONG64,
    pub MaximumUserModeAddress: super::ULONG64,
    pub ActiveProcessorsAffinityMask: super::ULONG64,
    pub NumberOfProcessors: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    pub AvailablePages: super::ULONG64,
    pub CommittedPages: super::ULONG64,
    pub CommitLimit: super::ULONG64,
    pub PeakCommitment: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    pub CurrentSize: super::ULONG64,
    pub PeakSize: super::ULONG64,
    pub PageFaultCount: super::ULONG,
    pub MinimumWorkingSet: super::ULONG64,
    pub MaximumWorkingSet: super::ULONG64,
    pub CurrentSizeIncludingTransitionInPages: super::ULONG64,
    pub PeakSizeIncludingTransitionInPages: super::ULONG64,
    pub TransitionRePurposeCount: super::ULONG,
    pub Flags: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_SYSTEM_INFO {
    pub ProcessorArchitecture: super::USHORT,
    pub ProcessorLevel: super::USHORT,
    pub ProcessorRevision: super::USHORT,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_0,
    pub MajorVersion: super::ULONG32,
    pub MinorVersion: super::ULONG32,
    pub BuildNumber: super::ULONG32,
    pub PlatformId: super::ULONG32,
    pub CSDVersionRva: RVA,
    pub Anonymous2: MINIDUMP_SYSTEM_INFO_1,
    pub Cpu: CPU_INFORMATION,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_SYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_SYSTEM_INFO_0 {
    pub Reserved0: super::USHORT,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_0_0,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_INFO_0_0 {
    pub NumberOfProcessors: super::UCHAR,
    pub ProductType: super::UCHAR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_SYSTEM_INFO_1 {
    pub Reserved1: super::ULONG32,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_1_0,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_SYSTEM_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_INFO_1_0 {
    pub SuiteMask: super::USHORT,
    pub Reserved2: super::USHORT,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    pub Revision: super::USHORT,
    pub Flags: super::USHORT,
    pub BasicInfo: MINIDUMP_SYSTEM_BASIC_INFORMATION,
    pub FileCacheInfo: MINIDUMP_SYSTEM_FILECACHE_INFORMATION,
    pub BasicPerfInfo: MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION,
    pub PerfInfo: MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_2 {
    pub Revision: super::USHORT,
    pub Flags: super::USHORT,
    pub BasicInfo: MINIDUMP_SYSTEM_BASIC_INFORMATION,
    pub FileCacheInfo: MINIDUMP_SYSTEM_FILECACHE_INFORMATION,
    pub BasicPerfInfo: MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION,
    pub PerfInfo: MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2,
}
#[cfg(feature = "shared")]
pub type MINIDUMP_SYSTEM_MEMORY_INFO_N = MINIDUMP_SYSTEM_MEMORY_INFO_2;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    pub IdleProcessTime: super::ULONG64,
    pub IoReadTransferCount: super::ULONG64,
    pub IoWriteTransferCount: super::ULONG64,
    pub IoOtherTransferCount: super::ULONG64,
    pub IoReadOperationCount: super::ULONG,
    pub IoWriteOperationCount: super::ULONG,
    pub IoOtherOperationCount: super::ULONG,
    pub AvailablePages: super::ULONG,
    pub CommittedPages: super::ULONG,
    pub CommitLimit: super::ULONG,
    pub PeakCommitment: super::ULONG,
    pub PageFaultCount: super::ULONG,
    pub CopyOnWriteCount: super::ULONG,
    pub TransitionCount: super::ULONG,
    pub CacheTransitionCount: super::ULONG,
    pub DemandZeroCount: super::ULONG,
    pub PageReadCount: super::ULONG,
    pub PageReadIoCount: super::ULONG,
    pub CacheReadCount: super::ULONG,
    pub CacheIoCount: super::ULONG,
    pub DirtyPagesWriteCount: super::ULONG,
    pub DirtyWriteIoCount: super::ULONG,
    pub MappedPagesWriteCount: super::ULONG,
    pub MappedWriteIoCount: super::ULONG,
    pub PagedPoolPages: super::ULONG,
    pub NonPagedPoolPages: super::ULONG,
    pub PagedPoolAllocs: super::ULONG,
    pub PagedPoolFrees: super::ULONG,
    pub NonPagedPoolAllocs: super::ULONG,
    pub NonPagedPoolFrees: super::ULONG,
    pub FreeSystemPtes: super::ULONG,
    pub ResidentSystemCodePage: super::ULONG,
    pub TotalSystemDriverPages: super::ULONG,
    pub TotalSystemCodePages: super::ULONG,
    pub NonPagedPoolLookasideHits: super::ULONG,
    pub PagedPoolLookasideHits: super::ULONG,
    pub AvailablePagedPoolPages: super::ULONG,
    pub ResidentSystemCachePage: super::ULONG,
    pub ResidentPagedPoolPage: super::ULONG,
    pub ResidentSystemDriverPage: super::ULONG,
    pub CcFastReadNoWait: super::ULONG,
    pub CcFastReadWait: super::ULONG,
    pub CcFastReadResourceMiss: super::ULONG,
    pub CcFastReadNotPossible: super::ULONG,
    pub CcFastMdlReadNoWait: super::ULONG,
    pub CcFastMdlReadWait: super::ULONG,
    pub CcFastMdlReadResourceMiss: super::ULONG,
    pub CcFastMdlReadNotPossible: super::ULONG,
    pub CcMapDataNoWait: super::ULONG,
    pub CcMapDataWait: super::ULONG,
    pub CcMapDataNoWaitMiss: super::ULONG,
    pub CcMapDataWaitMiss: super::ULONG,
    pub CcPinMappedDataCount: super::ULONG,
    pub CcPinReadNoWait: super::ULONG,
    pub CcPinReadWait: super::ULONG,
    pub CcPinReadNoWaitMiss: super::ULONG,
    pub CcPinReadWaitMiss: super::ULONG,
    pub CcCopyReadNoWait: super::ULONG,
    pub CcCopyReadWait: super::ULONG,
    pub CcCopyReadNoWaitMiss: super::ULONG,
    pub CcCopyReadWaitMiss: super::ULONG,
    pub CcMdlReadNoWait: super::ULONG,
    pub CcMdlReadWait: super::ULONG,
    pub CcMdlReadNoWaitMiss: super::ULONG,
    pub CcMdlReadWaitMiss: super::ULONG,
    pub CcReadAheadIos: super::ULONG,
    pub CcLazyWriteIos: super::ULONG,
    pub CcLazyWritePages: super::ULONG,
    pub CcDataFlushes: super::ULONG,
    pub CcDataPages: super::ULONG,
    pub ContextSwitches: super::ULONG,
    pub FirstLevelTbFills: super::ULONG,
    pub SecondLevelTbFills: super::ULONG,
    pub SystemCalls: super::ULONG,
    pub CcTotalDirtyPages: super::ULONG64,
    pub CcDirtyPageThreshold: super::ULONG64,
    pub ResidentAvailablePages: super::LONG64,
    pub SharedCommittedPages: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2 {
    pub IdleProcessTime: super::ULONG64,
    pub IoReadTransferCount: super::ULONG64,
    pub IoWriteTransferCount: super::ULONG64,
    pub IoOtherTransferCount: super::ULONG64,
    pub IoReadOperationCount: super::ULONG,
    pub IoWriteOperationCount: super::ULONG,
    pub IoOtherOperationCount: super::ULONG,
    pub AvailablePages: super::ULONG,
    pub CommittedPages: super::ULONG,
    pub CommitLimit: super::ULONG,
    pub PeakCommitment: super::ULONG,
    pub PageFaultCount: super::ULONG,
    pub CopyOnWriteCount: super::ULONG,
    pub TransitionCount: super::ULONG,
    pub CacheTransitionCount: super::ULONG,
    pub DemandZeroCount: super::ULONG,
    pub PageReadCount: super::ULONG,
    pub PageReadIoCount: super::ULONG,
    pub CacheReadCount: super::ULONG,
    pub CacheIoCount: super::ULONG,
    pub DirtyPagesWriteCount: super::ULONG,
    pub DirtyWriteIoCount: super::ULONG,
    pub MappedPagesWriteCount: super::ULONG,
    pub MappedWriteIoCount: super::ULONG,
    pub PagedPoolPages: super::ULONG,
    pub NonPagedPoolPages: super::ULONG,
    pub PagedPoolAllocs: super::ULONG,
    pub PagedPoolFrees: super::ULONG,
    pub NonPagedPoolAllocs: super::ULONG,
    pub NonPagedPoolFrees: super::ULONG,
    pub FreeSystemPtes: super::ULONG,
    pub ResidentSystemCodePage: super::ULONG,
    pub TotalSystemDriverPages: super::ULONG,
    pub TotalSystemCodePages: super::ULONG,
    pub NonPagedPoolLookasideHits: super::ULONG,
    pub PagedPoolLookasideHits: super::ULONG,
    pub AvailablePagedPoolPages: super::ULONG,
    pub ResidentSystemCachePage: super::ULONG,
    pub ResidentPagedPoolPage: super::ULONG,
    pub ResidentSystemDriverPage: super::ULONG,
    pub CcFastReadNoWait: super::ULONG,
    pub CcFastReadWait: super::ULONG,
    pub CcFastReadResourceMiss: super::ULONG,
    pub CcFastReadNotPossible: super::ULONG,
    pub CcFastMdlReadNoWait: super::ULONG,
    pub CcFastMdlReadWait: super::ULONG,
    pub CcFastMdlReadResourceMiss: super::ULONG,
    pub CcFastMdlReadNotPossible: super::ULONG,
    pub CcMapDataNoWait: super::ULONG,
    pub CcMapDataWait: super::ULONG,
    pub CcMapDataNoWaitMiss: super::ULONG,
    pub CcMapDataWaitMiss: super::ULONG,
    pub CcPinMappedDataCount: super::ULONG,
    pub CcPinReadNoWait: super::ULONG,
    pub CcPinReadWait: super::ULONG,
    pub CcPinReadNoWaitMiss: super::ULONG,
    pub CcPinReadWaitMiss: super::ULONG,
    pub CcCopyReadNoWait: super::ULONG,
    pub CcCopyReadWait: super::ULONG,
    pub CcCopyReadNoWaitMiss: super::ULONG,
    pub CcCopyReadWaitMiss: super::ULONG,
    pub CcMdlReadNoWait: super::ULONG,
    pub CcMdlReadWait: super::ULONG,
    pub CcMdlReadNoWaitMiss: super::ULONG,
    pub CcMdlReadWaitMiss: super::ULONG,
    pub CcReadAheadIos: super::ULONG,
    pub CcLazyWriteIos: super::ULONG,
    pub CcLazyWritePages: super::ULONG,
    pub CcDataFlushes: super::ULONG,
    pub CcDataPages: super::ULONG,
    pub ContextSwitches: super::ULONG,
    pub FirstLevelTbFills: super::ULONG,
    pub SecondLevelTbFills: super::ULONG,
    pub SystemCalls: super::ULONG,
    pub CcTotalDirtyPages: super::ULONG64,
    pub CcDirtyPageThreshold: super::ULONG64,
    pub ResidentAvailablePages: super::LONG64,
    pub SharedCommittedPages: super::ULONG64,
    pub MdlPagesAllocated: super::ULONGLONG,
    pub PfnDatabaseCommittedPages: super::ULONGLONG,
    pub SystemPageTableCommittedPages: super::ULONGLONG,
    pub ContiguousPagesAllocated: super::ULONGLONG,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD {
    pub ThreadId: super::ULONG32,
    pub SuspendCount: super::ULONG32,
    pub PriorityClass: super::ULONG32,
    pub Priority: super::ULONG32,
    pub Teb: super::ULONG64,
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_EX {
    pub ThreadId: super::ULONG32,
    pub SuspendCount: super::ULONG32,
    pub PriorityClass: super::ULONG32,
    pub Priority: super::ULONG32,
    pub Teb: super::ULONG64,
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
    pub BackingStore: MINIDUMP_MEMORY_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_EX_LIST {
    pub NumberOfThreads: super::ULONG32,
    pub Threads: [MINIDUMP_THREAD_EX; 0],
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_THREAD_EX_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_INFO {
    pub ThreadId: super::ULONG32,
    pub DumpFlags: super::ULONG32,
    pub DumpError: super::ULONG32,
    pub ExitStatus: super::ULONG32,
    pub CreateTime: super::ULONG64,
    pub ExitTime: super::ULONG64,
    pub KernelTime: super::ULONG64,
    pub UserTime: super::ULONG64,
    pub StartAddress: super::ULONG64,
    pub Affinity: super::ULONG64,
}
pub const MINIDUMP_THREAD_INFO_ERROR_THREAD: i32 = 1;
pub const MINIDUMP_THREAD_INFO_EXITED_THREAD: i32 = 4;
pub const MINIDUMP_THREAD_INFO_INVALID_CONTEXT: i32 = 16;
pub const MINIDUMP_THREAD_INFO_INVALID_INFO: i32 = 8;
pub const MINIDUMP_THREAD_INFO_INVALID_TEB: i32 = 32;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_INFO_LIST {
    pub SizeOfHeader: super::ULONG,
    pub SizeOfEntry: super::ULONG,
    pub NumberOfEntries: super::ULONG,
}
pub const MINIDUMP_THREAD_INFO_WRITING_THREAD: i32 = 2;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_LIST {
    pub NumberOfThreads: super::ULONG32,
    pub Threads: [MINIDUMP_THREAD; 0],
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for MINIDUMP_THREAD_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_NAME {
    pub ThreadId: super::ULONG,
    pub RvaOfThreadName: RVA64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_NAME_LIST {
    pub NumberOfThreadNames: super::ULONG,
    pub ThreadNames: [MINIDUMP_THREAD_NAME; 0],
}
#[cfg(feature = "shared")]
impl Default for MINIDUMP_THREAD_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_TOKEN_INFO_HEADER {
    pub TokenSize: super::ULONG,
    pub TokenId: super::ULONG,
    pub TokenHandle: super::ULONG64,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_TOKEN_INFO_LIST {
    pub TokenListSize: super::ULONG,
    pub TokenListEntries: super::ULONG,
    pub ListHeaderSize: super::ULONG,
    pub ElementHeaderSize: super::ULONG,
}
pub type MINIDUMP_TYPE = i32;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_UNLOADED_MODULE {
    pub BaseOfImage: super::ULONG64,
    pub SizeOfImage: super::ULONG32,
    pub CheckSum: super::ULONG32,
    pub TimeDateStamp: super::ULONG32,
    pub ModuleNameRva: RVA,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_UNLOADED_MODULE_LIST {
    pub SizeOfHeader: super::ULONG32,
    pub SizeOfEntry: super::ULONG32,
    pub NumberOfEntries: super::ULONG32,
}
#[repr(C, packed(4))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_USER_RECORD {
    pub Type: super::ULONG32,
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_USER_STREAM {
    pub Type: super::ULONG32,
    pub BufferSize: super::ULONG,
    pub Buffer: super::PVOID,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_USER_STREAM_INFORMATION {
    pub UserStreamCount: super::ULONG,
    pub UserStreamArray: PMINIDUMP_USER_STREAM,
}
pub const MINIDUMP_VERSION: i32 = 42899;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_VM_POST_READ_CALLBACK {
    pub Offset: super::ULONG64,
    pub Buffer: super::PVOID,
    pub Size: super::ULONG,
    pub Completed: super::ULONG,
    pub Status: windows_sys::core::HRESULT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_VM_PRE_READ_CALLBACK {
    pub Offset: super::ULONG64,
    pub Buffer: super::PVOID,
    pub Size: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_VM_QUERY_CALLBACK {
    pub Offset: super::ULONG64,
}
pub type MODULE_WRITE_FLAGS = i32;
pub const Memory64ListStream: MINIDUMP_STREAM_TYPE = 9;
pub const MemoryCallback: MINIDUMP_CALLBACK_TYPE = 5;
pub const MemoryInfoListStream: MINIDUMP_STREAM_TYPE = 16;
pub const MemoryListStream: MINIDUMP_STREAM_TYPE = 5;
pub const MiniDumpFilterMemory: MINIDUMP_TYPE = 8;
pub const MiniDumpFilterModulePaths: MINIDUMP_TYPE = 128;
pub const MiniDumpFilterTriage: MINIDUMP_TYPE = 1048576;
pub const MiniDumpFilterWriteCombinedMemory: MINIDUMP_TYPE = 16777216;
pub const MiniDumpIgnoreInaccessibleMemory: MINIDUMP_TYPE = 131072;
pub const MiniDumpNoIgnoreInaccessibleMemory: MINIDUMP_TYPE = 33554432;
pub const MiniDumpNormal: MINIDUMP_TYPE = 0;
pub const MiniDumpScanInaccessiblePartialPages: MINIDUMP_TYPE = 8388608;
pub const MiniDumpScanMemory: MINIDUMP_TYPE = 16;
pub const MiniDumpValidTypeFlags: MINIDUMP_TYPE = 100663295;
pub const MiniDumpValidTypeFlagsEx: MINIDUMP_TYPE = 134217727;
pub const MiniDumpWithAvxXStateContext: MINIDUMP_TYPE = 2097152;
pub const MiniDumpWithCodeSegs: MINIDUMP_TYPE = 8192;
pub const MiniDumpWithDataSegs: MINIDUMP_TYPE = 1;
pub const MiniDumpWithFullAuxiliaryState: MINIDUMP_TYPE = 32768;
pub const MiniDumpWithFullMemory: MINIDUMP_TYPE = 2;
pub const MiniDumpWithFullMemoryInfo: MINIDUMP_TYPE = 2048;
pub const MiniDumpWithHandleData: MINIDUMP_TYPE = 4;
pub const MiniDumpWithIndirectlyReferencedMemory: MINIDUMP_TYPE = 64;
pub const MiniDumpWithIptTrace: MINIDUMP_TYPE = 4194304;
pub const MiniDumpWithMemoryCompressed: MINIDUMP_TYPE = 67108864;
pub const MiniDumpWithModuleHeaders: MINIDUMP_TYPE = 524288;
pub const MiniDumpWithPrivateReadWriteMemory: MINIDUMP_TYPE = 512;
pub const MiniDumpWithPrivateWriteCopyMemory: MINIDUMP_TYPE = 65536;
pub const MiniDumpWithProcessThreadData: MINIDUMP_TYPE = 256;
pub const MiniDumpWithThreadInfo: MINIDUMP_TYPE = 4096;
pub const MiniDumpWithTokenInformation: MINIDUMP_TYPE = 262144;
pub const MiniDumpWithUnloadedModules: MINIDUMP_TYPE = 32;
pub const MiniDumpWithoutAuxiliaryState: MINIDUMP_TYPE = 16384;
pub const MiniDumpWithoutOptionalData: MINIDUMP_TYPE = 1024;
pub const MiniEventInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 6;
pub const MiniHandleObjectInformationNone: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 0;
pub const MiniHandleObjectInformationTypeMax: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 9;
pub const MiniMutantInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 2;
pub const MiniMutantInformation2: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 3;
pub const MiniProcessInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 4;
pub const MiniProcessInformation2: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 5;
pub const MiniSecondaryValidFlags: MINIDUMP_SECONDARY_FLAGS = 1;
pub const MiniSecondaryWithoutPowerInfo: MINIDUMP_SECONDARY_FLAGS = 1;
pub const MiniSectionInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 7;
pub const MiniSemaphoreInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 8;
pub const MiniThreadInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = 1;
pub const MiscInfoStream: MINIDUMP_STREAM_TYPE = 15;
pub const ModuleCallback: MINIDUMP_CALLBACK_TYPE = 0;
pub const ModuleListStream: MINIDUMP_STREAM_TYPE = 4;
pub const ModuleReferencedByMemory: MODULE_WRITE_FLAGS = 16;
pub const ModuleWriteCodeSegs: MODULE_WRITE_FLAGS = 64;
pub const ModuleWriteCvRecord: MODULE_WRITE_FLAGS = 8;
pub const ModuleWriteDataSeg: MODULE_WRITE_FLAGS = 2;
pub const ModuleWriteMiscRecord: MODULE_WRITE_FLAGS = 4;
pub const ModuleWriteModule: MODULE_WRITE_FLAGS = 1;
pub const ModuleWriteTlsData: MODULE_WRITE_FLAGS = 32;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PCPU_INFORMATION = *mut CPU_INFORMATION;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_CALLBACK_OUTPUT = *mut MINIDUMP_CALLBACK_OUTPUT;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK = *mut MINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK = *mut MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_DIRECTORY = *mut MINIDUMP_DIRECTORY;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_EXCEPTION = *mut MINIDUMP_EXCEPTION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_EXCEPTION_INFORMATION = *mut MINIDUMP_EXCEPTION_INFORMATION;
#[cfg(feature = "shared")]
pub type PMINIDUMP_EXCEPTION_INFORMATION64 = *mut MINIDUMP_EXCEPTION_INFORMATION64;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_EXCEPTION_STREAM = *mut MINIDUMP_EXCEPTION_STREAM;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_FUNCTION_TABLE_DESCRIPTOR = *mut MINIDUMP_FUNCTION_TABLE_DESCRIPTOR;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_FUNCTION_TABLE_STREAM = *mut MINIDUMP_FUNCTION_TABLE_STREAM;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_HANDLE_DATA_STREAM = *mut MINIDUMP_HANDLE_DATA_STREAM;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_HANDLE_DESCRIPTOR = *mut MINIDUMP_HANDLE_DESCRIPTOR;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_HANDLE_DESCRIPTOR_2 = *mut MINIDUMP_HANDLE_DESCRIPTOR_2;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_HANDLE_DESCRIPTOR_N = *mut MINIDUMP_HANDLE_DESCRIPTOR_2;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_HANDLE_OPERATION_LIST = *mut MINIDUMP_HANDLE_OPERATION_LIST;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_HEADER = *mut MINIDUMP_HEADER;
#[cfg(feature = "shared")]
pub type PMINIDUMP_INCLUDE_MODULE_CALLBACK = *mut MINIDUMP_INCLUDE_MODULE_CALLBACK;
#[cfg(feature = "shared")]
pub type PMINIDUMP_INCLUDE_THREAD_CALLBACK = *mut MINIDUMP_INCLUDE_THREAD_CALLBACK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_IO_CALLBACK = *mut MINIDUMP_IO_CALLBACK;
#[cfg(feature = "shared")]
pub type PMINIDUMP_MEMORY64_LIST = *mut MINIDUMP_MEMORY64_LIST;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_MEMORY_DESCRIPTOR = *mut MINIDUMP_MEMORY_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PMINIDUMP_MEMORY_DESCRIPTOR64 = *mut MINIDUMP_MEMORY_DESCRIPTOR64;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_MEMORY_INFO = *mut MINIDUMP_MEMORY_INFO;
#[cfg(feature = "shared")]
pub type PMINIDUMP_MEMORY_INFO_LIST = *mut MINIDUMP_MEMORY_INFO_LIST;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_MEMORY_LIST = *mut MINIDUMP_MEMORY_LIST;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_MISC_INFO = *mut MINIDUMP_MISC_INFO;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_MISC_INFO_2 = *mut MINIDUMP_MISC_INFO_2;
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PMINIDUMP_MISC_INFO_3 = *mut MINIDUMP_MISC_INFO_3;
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PMINIDUMP_MISC_INFO_4 = *mut MINIDUMP_MISC_INFO_4;
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PMINIDUMP_MISC_INFO_5 = *mut MINIDUMP_MISC_INFO_5;
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PMINIDUMP_MISC_INFO_N = *mut MINIDUMP_MISC_INFO_5;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "verrsrc"))]
pub type PMINIDUMP_MODULE = *mut MINIDUMP_MODULE;
#[cfg(all(feature = "shared", feature = "verrsrc", feature = "winnt"))]
pub type PMINIDUMP_MODULE_CALLBACK = *mut MINIDUMP_MODULE_CALLBACK;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "verrsrc"))]
pub type PMINIDUMP_MODULE_LIST = *mut MINIDUMP_MODULE_LIST;
#[cfg(feature = "shared")]
pub type PMINIDUMP_PROCESS_VM_COUNTERS_1 = *mut MINIDUMP_PROCESS_VM_COUNTERS_1;
#[cfg(feature = "shared")]
pub type PMINIDUMP_PROCESS_VM_COUNTERS_2 = *mut MINIDUMP_PROCESS_VM_COUNTERS_2;
#[cfg(feature = "shared")]
pub type PMINIDUMP_PROCESS_VM_COUNTERS_N = *mut MINIDUMP_PROCESS_VM_COUNTERS_2;
#[cfg(feature = "shared")]
pub type PMINIDUMP_READ_MEMORY_FAILURE_CALLBACK = *mut MINIDUMP_READ_MEMORY_FAILURE_CALLBACK;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PMINIDUMP_STRING = *mut MINIDUMP_STRING;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_BASIC_INFORMATION = *mut MINIDUMP_SYSTEM_BASIC_INFORMATION;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION = *mut MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_FILECACHE_INFORMATION = *mut MINIDUMP_SYSTEM_FILECACHE_INFORMATION;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_SYSTEM_INFO = *mut MINIDUMP_SYSTEM_INFO;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_MEMORY_INFO_1 = *mut MINIDUMP_SYSTEM_MEMORY_INFO_1;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_MEMORY_INFO_2 = *mut MINIDUMP_SYSTEM_MEMORY_INFO_2;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_MEMORY_INFO_N = *mut MINIDUMP_SYSTEM_MEMORY_INFO_2;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_PERFORMANCE_INFORMATION = *mut MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PMINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2 = *mut MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_THREAD = *mut MINIDUMP_THREAD;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_THREAD_EX = *mut MINIDUMP_THREAD_EX;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_THREAD_EX_LIST = *mut MINIDUMP_THREAD_EX_LIST;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_THREAD_INFO = *mut MINIDUMP_THREAD_INFO;
#[cfg(feature = "shared")]
pub type PMINIDUMP_THREAD_INFO_LIST = *mut MINIDUMP_THREAD_INFO_LIST;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_THREAD_LIST = *mut MINIDUMP_THREAD_LIST;
#[cfg(feature = "shared")]
pub type PMINIDUMP_THREAD_NAME = *mut MINIDUMP_THREAD_NAME;
#[cfg(feature = "shared")]
pub type PMINIDUMP_THREAD_NAME_LIST = *mut MINIDUMP_THREAD_NAME_LIST;
#[cfg(feature = "shared")]
pub type PMINIDUMP_TOKEN_INFO_HEADER = *mut MINIDUMP_TOKEN_INFO_HEADER;
#[cfg(feature = "shared")]
pub type PMINIDUMP_TOKEN_INFO_LIST = *mut MINIDUMP_TOKEN_INFO_LIST;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMINIDUMP_UNLOADED_MODULE = *mut MINIDUMP_UNLOADED_MODULE;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_UNLOADED_MODULE_LIST = *mut MINIDUMP_UNLOADED_MODULE_LIST;
#[cfg(feature = "basetsd")]
pub type PMINIDUMP_USER_RECORD = *mut MINIDUMP_USER_RECORD;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_USER_STREAM = *mut MINIDUMP_USER_STREAM;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_USER_STREAM_INFORMATION = *mut MINIDUMP_USER_STREAM_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_VM_POST_READ_CALLBACK = *mut MINIDUMP_VM_POST_READ_CALLBACK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMINIDUMP_VM_PRE_READ_CALLBACK = *mut MINIDUMP_VM_PRE_READ_CALLBACK;
#[cfg(feature = "shared")]
pub type PMINIDUMP_VM_QUERY_CALLBACK = *mut MINIDUMP_VM_QUERY_CALLBACK;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PXSTATE_CONFIG_FEATURE_MSC_INFO = *mut XSTATE_CONFIG_FEATURE_MSC_INFO;
pub const ProcessVmCountersStream: MINIDUMP_STREAM_TYPE = 22;
pub type RVA = u32;
pub type RVA64 = u64;
pub const ReadMemoryFailureCallback: MINIDUMP_CALLBACK_TYPE = 14;
pub const RemoveMemoryCallback: MINIDUMP_CALLBACK_TYPE = 9;
pub const ReservedStream0: MINIDUMP_STREAM_TYPE = 1;
pub const ReservedStream1: MINIDUMP_STREAM_TYPE = 2;
pub const SecondaryFlagsCallback: MINIDUMP_CALLBACK_TYPE = 15;
pub const SystemInfoStream: MINIDUMP_STREAM_TYPE = 7;
pub const SystemMemoryInfoStream: MINIDUMP_STREAM_TYPE = 21;
pub type THREAD_WRITE_FLAGS = i32;
pub const ThreadCallback: MINIDUMP_CALLBACK_TYPE = 1;
pub const ThreadExCallback: MINIDUMP_CALLBACK_TYPE = 2;
pub const ThreadExListStream: MINIDUMP_STREAM_TYPE = 8;
pub const ThreadInfoListStream: MINIDUMP_STREAM_TYPE = 17;
pub const ThreadListStream: MINIDUMP_STREAM_TYPE = 3;
pub const ThreadNamesStream: MINIDUMP_STREAM_TYPE = 24;
pub const ThreadWriteBackingStore: THREAD_WRITE_FLAGS = 8;
pub const ThreadWriteContext: THREAD_WRITE_FLAGS = 4;
pub const ThreadWriteInstructionWindow: THREAD_WRITE_FLAGS = 16;
pub const ThreadWriteStack: THREAD_WRITE_FLAGS = 2;
pub const ThreadWriteThread: THREAD_WRITE_FLAGS = 1;
pub const ThreadWriteThreadData: THREAD_WRITE_FLAGS = 32;
pub const ThreadWriteThreadInfo: THREAD_WRITE_FLAGS = 64;
pub const TokenStream: MINIDUMP_STREAM_TYPE = 19;
pub const UnloadedModuleListStream: MINIDUMP_STREAM_TYPE = 14;
pub const UnusedStream: MINIDUMP_STREAM_TYPE = 0;
pub const VmPostReadCallback: MINIDUMP_CALLBACK_TYPE = 20;
pub const VmPreReadCallback: MINIDUMP_CALLBACK_TYPE = 19;
pub const VmQueryCallback: MINIDUMP_CALLBACK_TYPE = 18;
pub const VmStartCallback: MINIDUMP_CALLBACK_TYPE = 17;
pub const WriteKernelMinidumpCallback: MINIDUMP_CALLBACK_TYPE = 7;
#[repr(C, packed(4))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct XSTATE_CONFIG_FEATURE_MSC_INFO {
    pub SizeOfInfo: super::ULONG32,
    pub ContextSize: super::ULONG32,
    pub EnabledFeatures: super::ULONG64,
    pub Features: [super::XSTATE_FEATURE; 64],
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for XSTATE_CONFIG_FEATURE_MSC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ceStreamBucketParameters: MINIDUMP_STREAM_TYPE = 32778;
pub const ceStreamDiagnosisList: MINIDUMP_STREAM_TYPE = 32780;
pub const ceStreamException: MINIDUMP_STREAM_TYPE = 32770;
pub const ceStreamMemoryPhysicalList: MINIDUMP_STREAM_TYPE = 32777;
pub const ceStreamMemoryVirtualList: MINIDUMP_STREAM_TYPE = 32776;
pub const ceStreamModuleList: MINIDUMP_STREAM_TYPE = 32771;
pub const ceStreamNull: MINIDUMP_STREAM_TYPE = 32768;
pub const ceStreamProcessList: MINIDUMP_STREAM_TYPE = 32772;
pub const ceStreamProcessModuleMap: MINIDUMP_STREAM_TYPE = 32779;
pub const ceStreamSystemInfo: MINIDUMP_STREAM_TYPE = 32769;
pub const ceStreamThreadCallStackList: MINIDUMP_STREAM_TYPE = 32775;
pub const ceStreamThreadContextList: MINIDUMP_STREAM_TYPE = 32774;
pub const ceStreamThreadList: MINIDUMP_STREAM_TYPE = 32773;
