windows_link::link!("dbghelp.dll" "system" fn MiniDumpReadDumpStream(baseofdump : *const core::ffi::c_void, streamnumber : u32, dir : *mut PMINIDUMP_DIRECTORY, streampointer : *mut *mut core::ffi::c_void, streamsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn MiniDumpWriteDump(hprocess : super::HANDLE, processid : u32, hfile : super::HANDLE, dumptype : MINIDUMP_TYPE, exceptionparam : *const MINIDUMP_EXCEPTION_INFORMATION, userstreamparam : *const MINIDUMP_USER_STREAM_INFORMATION, callbackparam : *const MINIDUMP_CALLBACK_INFORMATION) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy)]
pub union CPU_INFORMATION {
    pub X86CpuInfo: CPU_INFORMATION_0,
    pub OtherCpuInfo: CPU_INFORMATION_1,
}
impl Default for CPU_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPU_INFORMATION_0 {
    pub VendorId: [u32; 3],
    pub VersionInformation: u32,
    pub FeatureInformation: u32,
    pub AMDExtendedCpuFeatures: u32,
}
impl Default for CPU_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CPU_INFORMATION_1 {
    pub ProcessorFeatures: [u64; 2],
}
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
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_CALLBACK_INFORMATION {
    pub CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    pub CallbackParam: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_CALLBACK_INFORMATION {
    pub CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    pub CallbackParam: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_CALLBACK_INPUT {
    pub ProcessId: u32,
    pub ProcessHandle: super::HANDLE,
    pub CallbackType: u32,
    pub Anonymous: MINIDUMP_CALLBACK_INPUT_0,
}
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_CALLBACK_INPUT_0 {
    pub Status: windows_sys::core::HRESULT,
    pub Thread: MINIDUMP_THREAD_CALLBACK,
    pub ThreadEx: MINIDUMP_THREAD_EX_CALLBACK,
    pub Module: MINIDUMP_MODULE_CALLBACK,
    pub IncludeThread: MINIDUMP_INCLUDE_THREAD_CALLBACK,
    pub IncludeModule: MINIDUMP_INCLUDE_MODULE_CALLBACK,
    pub Io: MINIDUMP_IO_CALLBACK,
    pub ReadMemoryFailure: MINIDUMP_READ_MEMORY_FAILURE_CALLBACK,
    pub SecondaryFlags: u32,
    pub VmQuery: MINIDUMP_VM_QUERY_CALLBACK,
    pub VmPreRead: MINIDUMP_VM_PRE_READ_CALLBACK,
    pub VmPostRead: MINIDUMP_VM_POST_READ_CALLBACK,
    pub CompressedMemoryStreamFinish: MINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK,
}
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_CALLBACK_OUTPUT_0 {
    pub ModuleWriteFlags: u32,
    pub ThreadWriteFlags: u32,
    pub SecondaryFlags: u32,
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0_0,
    pub Anonymous2: MINIDUMP_CALLBACK_OUTPUT_0_1,
    pub Handle: super::HANDLE,
    pub Anonymous3: MINIDUMP_CALLBACK_OUTPUT_0_2,
    pub Anonymous4: MINIDUMP_CALLBACK_OUTPUT_0_3,
    pub Anonymous5: MINIDUMP_CALLBACK_OUTPUT_0_4,
    pub CompressedMemoryStreamStart: MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK,
    pub Status: windows_sys::core::HRESULT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_0 {
    pub MemoryBase: u64,
    pub MemorySize: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_1 {
    pub CheckCancel: windows_sys::core::BOOL,
    pub Cancel: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_2 {
    pub VmRegion: MINIDUMP_MEMORY_INFO,
    pub Continue: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_3 {
    pub VmQueryStatus: windows_sys::core::HRESULT,
    pub VmQueryResult: MINIDUMP_MEMORY_INFO,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_4 {
    pub VmReadStatus: windows_sys::core::HRESULT,
    pub VmReadBytesCompleted: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MINIDUMP_CALLBACK_OUTPUT_0 {
    pub ModuleWriteFlags: u32,
    pub ThreadWriteFlags: u32,
    pub SecondaryFlags: u32,
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0_0,
    pub Anonymous2: MINIDUMP_CALLBACK_OUTPUT_0_1,
    pub Handle: super::HANDLE,
    pub Anonymous3: MINIDUMP_CALLBACK_OUTPUT_0_2,
    pub Anonymous4: MINIDUMP_CALLBACK_OUTPUT_0_3,
    pub Anonymous5: MINIDUMP_CALLBACK_OUTPUT_0_4,
    pub CompressedMemoryStreamStart: MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK,
    pub Status: windows_sys::core::HRESULT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_CALLBACK_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_0 {
    pub MemoryBase: u64,
    pub MemorySize: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_1 {
    pub CheckCancel: windows_sys::core::BOOL,
    pub Cancel: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_2 {
    pub VmRegion: MINIDUMP_MEMORY_INFO,
    pub Continue: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_3 {
    pub VmQueryStatus: windows_sys::core::HRESULT,
    pub VmQueryResult: MINIDUMP_MEMORY_INFO,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_4 {
    pub VmReadStatus: windows_sys::core::HRESULT,
    pub VmReadBytesCompleted: u32,
}
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
pub type MINIDUMP_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(callbackparam: *mut core::ffi::c_void, callbackinput: *const MINIDUMP_CALLBACK_INPUT, callbackoutput: *mut MINIDUMP_CALLBACK_OUTPUT) -> windows_sys::core::BOOL>;
pub type MINIDUMP_CALLBACK_TYPE = i32;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK {
    pub CompressedStreamSize: u64,
    pub StreamCompressionRate: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK {
    pub MaxParallelism: u32,
    pub MaxTransferSize: u32,
    pub NumaNodeInfoArray: super::PNUMA_NODE_RELATIONSHIP,
    pub NumaNodeInfoArraySize: u32,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK {
    pub MaxParallelism: u32,
    pub MaxTransferSize: u32,
    pub NumaNodeInfoArray: super::PNUMA_NODE_RELATIONSHIP,
    pub NumaNodeInfoArraySize: u32,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_DIRECTORY {
    pub StreamType: u32,
    pub Location: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_EXCEPTION {
    pub ExceptionCode: u32,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u64,
    pub ExceptionAddress: u64,
    pub NumberParameters: u32,
    pub __unusedAlignment: u32,
    pub ExceptionInformation: [u64; 15],
}
impl Default for MINIDUMP_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_EXCEPTION_INFORMATION {
    pub ThreadId: u32,
    pub ExceptionPointers: super::PEXCEPTION_POINTERS,
    pub ClientPointers: windows_sys::core::BOOL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_EXCEPTION_INFORMATION {
    pub ThreadId: u32,
    pub ExceptionPointers: super::PEXCEPTION_POINTERS,
    pub ClientPointers: windows_sys::core::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_EXCEPTION_INFORMATION64 {
    pub ThreadId: u32,
    pub ExceptionRecord: u64,
    pub ContextRecord: u64,
    pub ClientPointers: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_EXCEPTION_STREAM {
    pub ThreadId: u32,
    pub __alignment: u32,
    pub ExceptionRecord: MINIDUMP_EXCEPTION,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    pub MinimumAddress: u64,
    pub MaximumAddress: u64,
    pub BaseAddress: u64,
    pub EntryCount: u32,
    pub SizeOfAlignPad: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_FUNCTION_TABLE_STREAM {
    pub SizeOfHeader: u32,
    pub SizeOfDescriptor: u32,
    pub SizeOfNativeDescriptor: u32,
    pub SizeOfFunctionEntry: u32,
    pub NumberOfDescriptors: u32,
    pub SizeOfAlignPad: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_DATA_STREAM {
    pub SizeOfHeader: u32,
    pub SizeOfDescriptor: u32,
    pub NumberOfDescriptors: u32,
    pub Reserved: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_DESCRIPTOR {
    pub Handle: u64,
    pub TypeNameRva: RVA,
    pub ObjectNameRva: RVA,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_DESCRIPTOR_2 {
    pub Handle: u64,
    pub TypeNameRva: RVA,
    pub ObjectNameRva: RVA,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub ObjectInfoRva: RVA,
    pub Reserved0: u32,
}
pub type MINIDUMP_HANDLE_DESCRIPTOR_N = MINIDUMP_HANDLE_DESCRIPTOR_2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION {
    pub NextInfoRva: RVA,
    pub InfoType: u32,
    pub SizeOfInfo: u32,
}
pub type MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_HANDLE_OPERATION_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u32,
    pub Reserved: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_HEADER {
    pub Signature: u32,
    pub Version: u32,
    pub NumberOfStreams: u32,
    pub StreamDirectoryRva: RVA,
    pub CheckSum: u32,
    pub Anonymous: MINIDUMP_HEADER_0,
    pub Flags: u64,
}
impl Default for MINIDUMP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MINIDUMP_HEADER_0 {
    pub Reserved: u32,
    pub TimeDateStamp: u32,
}
impl Default for MINIDUMP_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    pub BaseOfImage: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    pub ThreadId: u32,
}
#[repr(C, packed(4))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_IO_CALLBACK {
    pub Handle: super::HANDLE,
    pub Offset: u64,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferBytes: u32,
}
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_IO_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR {
    pub DataSize: u32,
    pub Rva: RVA,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR64 {
    pub DataSize: u64,
    pub Rva: RVA64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MEMORY64_LIST {
    pub NumberOfMemoryRanges: u64,
    pub BaseRva: RVA64,
    pub MemoryRanges: [MINIDUMP_MEMORY_DESCRIPTOR64; 0],
}
impl Default for MINIDUMP_MEMORY64_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR {
    pub StartOfMemoryRange: u64,
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR64 {
    pub StartOfMemoryRange: u64,
    pub DataSize: u64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_INFO {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: u32,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
    pub __alignment2: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MEMORY_INFO_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MEMORY_LIST {
    pub NumberOfMemoryRanges: u32,
    pub MemoryRanges: [MINIDUMP_MEMORY_DESCRIPTOR; 0],
}
impl Default for MINIDUMP_MEMORY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_MISC1_PROCESSOR_POWER_INFO: u32 = 4;
pub const MINIDUMP_MISC1_PROCESS_ID: u32 = 1;
pub const MINIDUMP_MISC1_PROCESS_TIMES: u32 = 2;
pub const MINIDUMP_MISC3_PROCESS_EXECUTE_FLAGS: u32 = 32;
pub const MINIDUMP_MISC3_PROCESS_INTEGRITY: u32 = 16;
pub const MINIDUMP_MISC3_PROTECTED_PROCESS: u32 = 128;
pub const MINIDUMP_MISC3_TIMEZONE: u32 = 64;
pub const MINIDUMP_MISC4_BUILDSTRING: u32 = 256;
pub const MINIDUMP_MISC5_PROCESS_COOKIE: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MISC_INFO {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MISC_INFO_2 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MISC_INFO_3 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
    pub ProcessIntegrityLevel: u32,
    pub ProcessExecuteFlags: u32,
    pub ProtectedProcess: u32,
    pub TimeZoneId: u32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MISC_INFO_4 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
    pub ProcessIntegrityLevel: u32,
    pub ProcessExecuteFlags: u32,
    pub ProtectedProcess: u32,
    pub TimeZoneId: u32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BuildString: [u16; 260],
    pub DbgBldStr: [u16; 40],
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for MINIDUMP_MISC_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MISC_INFO_5 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
    pub ProcessIntegrityLevel: u32,
    pub ProcessExecuteFlags: u32,
    pub ProtectedProcess: u32,
    pub TimeZoneId: u32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BuildString: [u16; 260],
    pub DbgBldStr: [u16; 40],
    pub XStateData: XSTATE_CONFIG_FEATURE_MSC_INFO,
    pub ProcessCookie: u32,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi", feature = "winnt"))]
impl Default for MINIDUMP_MISC_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi", feature = "winnt"))]
pub type MINIDUMP_MISC_INFO_N = MINIDUMP_MISC_INFO_5;
#[repr(C, packed(4))]
#[cfg(feature = "verrsrc")]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_MODULE {
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub ModuleNameRva: RVA,
    pub VersionInfo: super::VS_FIXEDFILEINFO,
    pub CvRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    pub MiscRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    pub Reserved0: u64,
    pub Reserved1: u64,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MODULE_CALLBACK {
    pub FullPath: super::PWCHAR,
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub VersionInfo: super::VS_FIXEDFILEINFO,
    pub CvRecord: *mut core::ffi::c_void,
    pub SizeOfCvRecord: u32,
    pub MiscRecord: *mut core::ffi::c_void,
    pub SizeOfMiscRecord: u32,
}
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
impl Default for MINIDUMP_MODULE_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "verrsrc")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_MODULE_LIST {
    pub NumberOfModules: u32,
    pub Modules: [MINIDUMP_MODULE; 0],
}
#[cfg(feature = "verrsrc")]
impl Default for MINIDUMP_MODULE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_PROCESS_VM_COUNTERS: u32 = 1;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_1 {
    pub Revision: u16,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: u64,
    pub WorkingSetSize: u64,
    pub QuotaPeakPagedPoolUsage: u64,
    pub QuotaPagedPoolUsage: u64,
    pub QuotaPeakNonPagedPoolUsage: u64,
    pub QuotaNonPagedPoolUsage: u64,
    pub PagefileUsage: u64,
    pub PeakPagefileUsage: u64,
    pub PrivateUsage: u64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_2 {
    pub Revision: u16,
    pub Flags: u16,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: u64,
    pub WorkingSetSize: u64,
    pub QuotaPeakPagedPoolUsage: u64,
    pub QuotaPagedPoolUsage: u64,
    pub QuotaPeakNonPagedPoolUsage: u64,
    pub QuotaNonPagedPoolUsage: u64,
    pub PagefileUsage: u64,
    pub PeakPagefileUsage: u64,
    pub PeakVirtualSize: u64,
    pub VirtualSize: u64,
    pub PrivateUsage: u64,
    pub PrivateWorkingSetSize: u64,
    pub SharedCommitUsage: u64,
    pub JobSharedCommitUsage: u64,
    pub JobPrivateCommitUsage: u64,
    pub JobPeakPrivateCommitUsage: u64,
    pub JobPrivateCommitLimit: u64,
    pub JobTotalCommitLimit: u64,
}
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX: u32 = 4;
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX2: u32 = 8;
pub const MINIDUMP_PROCESS_VM_COUNTERS_JOB: u32 = 16;
pub type MINIDUMP_PROCESS_VM_COUNTERS_N = MINIDUMP_PROCESS_VM_COUNTERS_2;
pub const MINIDUMP_PROCESS_VM_COUNTERS_VIRTUALSIZE: u32 = 2;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    pub Offset: u64,
    pub Bytes: u32,
    pub FailureStatus: windows_sys::core::HRESULT,
}
pub type MINIDUMP_SECONDARY_FLAGS = i32;
pub const MINIDUMP_SIGNATURE: u32 = 1347241037;
pub type MINIDUMP_STREAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINIDUMP_STRING {
    pub Length: u32,
    pub Buffer: [u16; 0],
}
impl Default for MINIDUMP_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_SYSMEMINFO1_BASICPERF: u32 = 2;
pub const MINIDUMP_SYSMEMINFO1_FILECACHE_TRANSITIONREPURPOSECOUNT_FLAGS: u32 = 1;
pub const MINIDUMP_SYSMEMINFO1_PERF_CCTOTALDIRTYPAGES_CCDIRTYPAGETHRESHOLD: u32 = 4;
pub const MINIDUMP_SYSMEMINFO1_PERF_MDLPAGESALLOCATED_PFNDATABASECOMMITTEDPAGES: u32 = 16;
pub const MINIDUMP_SYSMEMINFO1_PERF_RESIDENTAVAILABLEPAGES_SHAREDCOMMITPAGES: u32 = 8;
pub const MINIDUMP_SYSMEMINFO1_PERF_SYSTEMPAGETABLECOMMITTEDPAGES_CONTIGUOUSPAGESALLOCATED: u32 = 32;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_BASIC_INFORMATION {
    pub TimerResolution: u32,
    pub PageSize: u32,
    pub NumberOfPhysicalPages: u32,
    pub LowestPhysicalPageNumber: u32,
    pub HighestPhysicalPageNumber: u32,
    pub AllocationGranularity: u32,
    pub MinimumUserModeAddress: u64,
    pub MaximumUserModeAddress: u64,
    pub ActiveProcessorsAffinityMask: u64,
    pub NumberOfProcessors: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    pub AvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    pub CurrentSize: u64,
    pub PeakSize: u64,
    pub PageFaultCount: u32,
    pub MinimumWorkingSet: u64,
    pub MaximumWorkingSet: u64,
    pub CurrentSizeIncludingTransitionInPages: u64,
    pub PeakSizeIncludingTransitionInPages: u64,
    pub TransitionRePurposeCount: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINIDUMP_SYSTEM_INFO {
    pub ProcessorArchitecture: u16,
    pub ProcessorLevel: u16,
    pub ProcessorRevision: u16,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_0,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BuildNumber: u32,
    pub PlatformId: u32,
    pub CSDVersionRva: RVA,
    pub Anonymous2: MINIDUMP_SYSTEM_INFO_1,
    pub Cpu: CPU_INFORMATION,
}
impl Default for MINIDUMP_SYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MINIDUMP_SYSTEM_INFO_0 {
    pub Reserved0: u16,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_0_0,
}
impl Default for MINIDUMP_SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_INFO_0_0 {
    pub NumberOfProcessors: u8,
    pub ProductType: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MINIDUMP_SYSTEM_INFO_1 {
    pub Reserved1: u32,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_1_0,
}
impl Default for MINIDUMP_SYSTEM_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_INFO_1_0 {
    pub SuiteMask: u16,
    pub Reserved2: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    pub Revision: u16,
    pub Flags: u16,
    pub BasicInfo: MINIDUMP_SYSTEM_BASIC_INFORMATION,
    pub FileCacheInfo: MINIDUMP_SYSTEM_FILECACHE_INFORMATION,
    pub BasicPerfInfo: MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION,
    pub PerfInfo: MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_2 {
    pub Revision: u16,
    pub Flags: u16,
    pub BasicInfo: MINIDUMP_SYSTEM_BASIC_INFORMATION,
    pub FileCacheInfo: MINIDUMP_SYSTEM_FILECACHE_INFORMATION,
    pub BasicPerfInfo: MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION,
    pub PerfInfo: MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2,
}
pub type MINIDUMP_SYSTEM_MEMORY_INFO_N = MINIDUMP_SYSTEM_MEMORY_INFO_2;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    pub IdleProcessTime: u64,
    pub IoReadTransferCount: u64,
    pub IoWriteTransferCount: u64,
    pub IoOtherTransferCount: u64,
    pub IoReadOperationCount: u32,
    pub IoWriteOperationCount: u32,
    pub IoOtherOperationCount: u32,
    pub AvailablePages: u32,
    pub CommittedPages: u32,
    pub CommitLimit: u32,
    pub PeakCommitment: u32,
    pub PageFaultCount: u32,
    pub CopyOnWriteCount: u32,
    pub TransitionCount: u32,
    pub CacheTransitionCount: u32,
    pub DemandZeroCount: u32,
    pub PageReadCount: u32,
    pub PageReadIoCount: u32,
    pub CacheReadCount: u32,
    pub CacheIoCount: u32,
    pub DirtyPagesWriteCount: u32,
    pub DirtyWriteIoCount: u32,
    pub MappedPagesWriteCount: u32,
    pub MappedWriteIoCount: u32,
    pub PagedPoolPages: u32,
    pub NonPagedPoolPages: u32,
    pub PagedPoolAllocs: u32,
    pub PagedPoolFrees: u32,
    pub NonPagedPoolAllocs: u32,
    pub NonPagedPoolFrees: u32,
    pub FreeSystemPtes: u32,
    pub ResidentSystemCodePage: u32,
    pub TotalSystemDriverPages: u32,
    pub TotalSystemCodePages: u32,
    pub NonPagedPoolLookasideHits: u32,
    pub PagedPoolLookasideHits: u32,
    pub AvailablePagedPoolPages: u32,
    pub ResidentSystemCachePage: u32,
    pub ResidentPagedPoolPage: u32,
    pub ResidentSystemDriverPage: u32,
    pub CcFastReadNoWait: u32,
    pub CcFastReadWait: u32,
    pub CcFastReadResourceMiss: u32,
    pub CcFastReadNotPossible: u32,
    pub CcFastMdlReadNoWait: u32,
    pub CcFastMdlReadWait: u32,
    pub CcFastMdlReadResourceMiss: u32,
    pub CcFastMdlReadNotPossible: u32,
    pub CcMapDataNoWait: u32,
    pub CcMapDataWait: u32,
    pub CcMapDataNoWaitMiss: u32,
    pub CcMapDataWaitMiss: u32,
    pub CcPinMappedDataCount: u32,
    pub CcPinReadNoWait: u32,
    pub CcPinReadWait: u32,
    pub CcPinReadNoWaitMiss: u32,
    pub CcPinReadWaitMiss: u32,
    pub CcCopyReadNoWait: u32,
    pub CcCopyReadWait: u32,
    pub CcCopyReadNoWaitMiss: u32,
    pub CcCopyReadWaitMiss: u32,
    pub CcMdlReadNoWait: u32,
    pub CcMdlReadWait: u32,
    pub CcMdlReadNoWaitMiss: u32,
    pub CcMdlReadWaitMiss: u32,
    pub CcReadAheadIos: u32,
    pub CcLazyWriteIos: u32,
    pub CcLazyWritePages: u32,
    pub CcDataFlushes: u32,
    pub CcDataPages: u32,
    pub ContextSwitches: u32,
    pub FirstLevelTbFills: u32,
    pub SecondLevelTbFills: u32,
    pub SystemCalls: u32,
    pub CcTotalDirtyPages: u64,
    pub CcDirtyPageThreshold: u64,
    pub ResidentAvailablePages: i64,
    pub SharedCommittedPages: u64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2 {
    pub IdleProcessTime: u64,
    pub IoReadTransferCount: u64,
    pub IoWriteTransferCount: u64,
    pub IoOtherTransferCount: u64,
    pub IoReadOperationCount: u32,
    pub IoWriteOperationCount: u32,
    pub IoOtherOperationCount: u32,
    pub AvailablePages: u32,
    pub CommittedPages: u32,
    pub CommitLimit: u32,
    pub PeakCommitment: u32,
    pub PageFaultCount: u32,
    pub CopyOnWriteCount: u32,
    pub TransitionCount: u32,
    pub CacheTransitionCount: u32,
    pub DemandZeroCount: u32,
    pub PageReadCount: u32,
    pub PageReadIoCount: u32,
    pub CacheReadCount: u32,
    pub CacheIoCount: u32,
    pub DirtyPagesWriteCount: u32,
    pub DirtyWriteIoCount: u32,
    pub MappedPagesWriteCount: u32,
    pub MappedWriteIoCount: u32,
    pub PagedPoolPages: u32,
    pub NonPagedPoolPages: u32,
    pub PagedPoolAllocs: u32,
    pub PagedPoolFrees: u32,
    pub NonPagedPoolAllocs: u32,
    pub NonPagedPoolFrees: u32,
    pub FreeSystemPtes: u32,
    pub ResidentSystemCodePage: u32,
    pub TotalSystemDriverPages: u32,
    pub TotalSystemCodePages: u32,
    pub NonPagedPoolLookasideHits: u32,
    pub PagedPoolLookasideHits: u32,
    pub AvailablePagedPoolPages: u32,
    pub ResidentSystemCachePage: u32,
    pub ResidentPagedPoolPage: u32,
    pub ResidentSystemDriverPage: u32,
    pub CcFastReadNoWait: u32,
    pub CcFastReadWait: u32,
    pub CcFastReadResourceMiss: u32,
    pub CcFastReadNotPossible: u32,
    pub CcFastMdlReadNoWait: u32,
    pub CcFastMdlReadWait: u32,
    pub CcFastMdlReadResourceMiss: u32,
    pub CcFastMdlReadNotPossible: u32,
    pub CcMapDataNoWait: u32,
    pub CcMapDataWait: u32,
    pub CcMapDataNoWaitMiss: u32,
    pub CcMapDataWaitMiss: u32,
    pub CcPinMappedDataCount: u32,
    pub CcPinReadNoWait: u32,
    pub CcPinReadWait: u32,
    pub CcPinReadNoWaitMiss: u32,
    pub CcPinReadWaitMiss: u32,
    pub CcCopyReadNoWait: u32,
    pub CcCopyReadWait: u32,
    pub CcCopyReadNoWaitMiss: u32,
    pub CcCopyReadWaitMiss: u32,
    pub CcMdlReadNoWait: u32,
    pub CcMdlReadWait: u32,
    pub CcMdlReadNoWaitMiss: u32,
    pub CcMdlReadWaitMiss: u32,
    pub CcReadAheadIos: u32,
    pub CcLazyWriteIos: u32,
    pub CcLazyWritePages: u32,
    pub CcDataFlushes: u32,
    pub CcDataPages: u32,
    pub ContextSwitches: u32,
    pub FirstLevelTbFills: u32,
    pub SecondLevelTbFills: u32,
    pub SystemCalls: u32,
    pub CcTotalDirtyPages: u64,
    pub CcDirtyPageThreshold: u64,
    pub ResidentAvailablePages: i64,
    pub SharedCommittedPages: u64,
    pub MdlPagesAllocated: u64,
    pub PfnDatabaseCommittedPages: u64,
    pub SystemPageTableCommittedPages: u64,
    pub ContiguousPagesAllocated: u64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD {
    pub ThreadId: u32,
    pub SuspendCount: u32,
    pub PriorityClass: u32,
    pub Priority: u32,
    pub Teb: u64,
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::HANDLE,
    pub Context: super::CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::HANDLE,
    pub Context: super::CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::HANDLE,
    pub Pad: u32,
    pub Context: super::CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_EX {
    pub ThreadId: u32,
    pub SuspendCount: u32,
    pub PriorityClass: u32,
    pub Priority: u32,
    pub Teb: u64,
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
    pub BackingStore: MINIDUMP_MEMORY_DESCRIPTOR,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_EX_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::HANDLE,
    pub Context: super::CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
    pub BackingStoreBase: u64,
    pub BackingStoreEnd: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_EX_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::HANDLE,
    pub Context: super::CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
    pub BackingStoreBase: u64,
    pub BackingStoreEnd: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_EX_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::HANDLE,
    pub Pad: u32,
    pub Context: super::CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
    pub BackingStoreBase: u64,
    pub BackingStoreEnd: u64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_EX_LIST {
    pub NumberOfThreads: u32,
    pub Threads: [MINIDUMP_THREAD_EX; 0],
}
impl Default for MINIDUMP_THREAD_EX_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_INFO {
    pub ThreadId: u32,
    pub DumpFlags: u32,
    pub DumpError: u32,
    pub ExitStatus: u32,
    pub CreateTime: u64,
    pub ExitTime: u64,
    pub KernelTime: u64,
    pub UserTime: u64,
    pub StartAddress: u64,
    pub Affinity: u64,
}
pub const MINIDUMP_THREAD_INFO_ERROR_THREAD: u32 = 1;
pub const MINIDUMP_THREAD_INFO_EXITED_THREAD: u32 = 4;
pub const MINIDUMP_THREAD_INFO_INVALID_CONTEXT: u32 = 16;
pub const MINIDUMP_THREAD_INFO_INVALID_INFO: u32 = 8;
pub const MINIDUMP_THREAD_INFO_INVALID_TEB: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_INFO_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u32,
}
pub const MINIDUMP_THREAD_INFO_WRITING_THREAD: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_LIST {
    pub NumberOfThreads: u32,
    pub Threads: [MINIDUMP_THREAD; 0],
}
impl Default for MINIDUMP_THREAD_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_THREAD_NAME {
    pub ThreadId: u32,
    pub RvaOfThreadName: RVA64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINIDUMP_THREAD_NAME_LIST {
    pub NumberOfThreadNames: u32,
    pub ThreadNames: [MINIDUMP_THREAD_NAME; 0],
}
impl Default for MINIDUMP_THREAD_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_TOKEN_INFO_HEADER {
    pub TokenSize: u32,
    pub TokenId: u32,
    pub TokenHandle: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_TOKEN_INFO_LIST {
    pub TokenListSize: u32,
    pub TokenListEntries: u32,
    pub ListHeaderSize: u32,
    pub ElementHeaderSize: u32,
}
pub type MINIDUMP_TYPE = i32;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_UNLOADED_MODULE {
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub ModuleNameRva: RVA,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_UNLOADED_MODULE_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_USER_RECORD {
    pub Type: u32,
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_USER_STREAM {
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for MINIDUMP_USER_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_USER_STREAM {
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MINIDUMP_USER_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct MINIDUMP_USER_STREAM_INFORMATION {
    pub UserStreamCount: u32,
    pub UserStreamArray: PMINIDUMP_USER_STREAM,
}
#[cfg(target_arch = "x86")]
impl Default for MINIDUMP_USER_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_USER_STREAM_INFORMATION {
    pub UserStreamCount: u32,
    pub UserStreamArray: PMINIDUMP_USER_STREAM,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MINIDUMP_USER_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MINIDUMP_VERSION: u32 = 42899;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_VM_POST_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut core::ffi::c_void,
    pub Size: u32,
    pub Completed: u32,
    pub Status: windows_sys::core::HRESULT,
}
impl Default for MINIDUMP_VM_POST_READ_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MINIDUMP_VM_PRE_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut core::ffi::c_void,
    pub Size: u32,
}
impl Default for MINIDUMP_VM_PRE_READ_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct MINIDUMP_VM_QUERY_CALLBACK {
    pub Offset: u64,
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
pub type PCPU_INFORMATION = *mut CPU_INFORMATION;
#[cfg(all(feature = "basetsd", feature = "verrsrc", feature = "winnt"))]
pub type PMINIDUMP_CALLBACK_INFORMATION = *mut MINIDUMP_CALLBACK_INFORMATION;
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
pub type PMINIDUMP_CALLBACK_INPUT = *mut MINIDUMP_CALLBACK_INPUT;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PMINIDUMP_CALLBACK_OUTPUT = *mut MINIDUMP_CALLBACK_OUTPUT;
pub type PMINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK = *mut MINIDUMP_COMPRESSED_MEMORY_STREAM_FINISH_CALLBACK;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PMINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK = *mut MINIDUMP_COMPRESSED_MEMORY_STREAM_START_CALLBACK;
pub type PMINIDUMP_DIRECTORY = *mut MINIDUMP_DIRECTORY;
pub type PMINIDUMP_EXCEPTION = *mut MINIDUMP_EXCEPTION;
#[cfg(feature = "winnt")]
pub type PMINIDUMP_EXCEPTION_INFORMATION = *mut MINIDUMP_EXCEPTION_INFORMATION;
pub type PMINIDUMP_EXCEPTION_INFORMATION64 = *mut MINIDUMP_EXCEPTION_INFORMATION64;
pub type PMINIDUMP_EXCEPTION_STREAM = *mut MINIDUMP_EXCEPTION_STREAM;
pub type PMINIDUMP_FUNCTION_TABLE_DESCRIPTOR = *mut MINIDUMP_FUNCTION_TABLE_DESCRIPTOR;
pub type PMINIDUMP_FUNCTION_TABLE_STREAM = *mut MINIDUMP_FUNCTION_TABLE_STREAM;
pub type PMINIDUMP_HANDLE_DATA_STREAM = *mut MINIDUMP_HANDLE_DATA_STREAM;
pub type PMINIDUMP_HANDLE_DESCRIPTOR = *mut MINIDUMP_HANDLE_DESCRIPTOR;
pub type PMINIDUMP_HANDLE_DESCRIPTOR_2 = *mut MINIDUMP_HANDLE_DESCRIPTOR_2;
pub type PMINIDUMP_HANDLE_DESCRIPTOR_N = *mut MINIDUMP_HANDLE_DESCRIPTOR_N;
pub type PMINIDUMP_HANDLE_OPERATION_LIST = *mut MINIDUMP_HANDLE_OPERATION_LIST;
pub type PMINIDUMP_HEADER = *mut MINIDUMP_HEADER;
pub type PMINIDUMP_INCLUDE_MODULE_CALLBACK = *mut MINIDUMP_INCLUDE_MODULE_CALLBACK;
pub type PMINIDUMP_INCLUDE_THREAD_CALLBACK = *mut MINIDUMP_INCLUDE_THREAD_CALLBACK;
#[cfg(feature = "winnt")]
pub type PMINIDUMP_IO_CALLBACK = *mut MINIDUMP_IO_CALLBACK;
pub type PMINIDUMP_MEMORY64_LIST = *mut MINIDUMP_MEMORY64_LIST;
pub type PMINIDUMP_MEMORY_DESCRIPTOR = *mut MINIDUMP_MEMORY_DESCRIPTOR;
pub type PMINIDUMP_MEMORY_DESCRIPTOR64 = *mut MINIDUMP_MEMORY_DESCRIPTOR64;
pub type PMINIDUMP_MEMORY_INFO = *mut MINIDUMP_MEMORY_INFO;
pub type PMINIDUMP_MEMORY_INFO_LIST = *mut MINIDUMP_MEMORY_INFO_LIST;
pub type PMINIDUMP_MEMORY_LIST = *mut MINIDUMP_MEMORY_LIST;
pub type PMINIDUMP_MISC_INFO = *mut MINIDUMP_MISC_INFO;
pub type PMINIDUMP_MISC_INFO_2 = *mut MINIDUMP_MISC_INFO_2;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
pub type PMINIDUMP_MISC_INFO_3 = *mut MINIDUMP_MISC_INFO_3;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
pub type PMINIDUMP_MISC_INFO_4 = *mut MINIDUMP_MISC_INFO_4;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi", feature = "winnt"))]
pub type PMINIDUMP_MISC_INFO_5 = *mut MINIDUMP_MISC_INFO_5;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi", feature = "winnt"))]
pub type PMINIDUMP_MISC_INFO_N = *mut MINIDUMP_MISC_INFO_N;
#[cfg(feature = "verrsrc")]
pub type PMINIDUMP_MODULE = *mut MINIDUMP_MODULE;
#[cfg(all(feature = "verrsrc", feature = "winnt"))]
pub type PMINIDUMP_MODULE_CALLBACK = *mut MINIDUMP_MODULE_CALLBACK;
#[cfg(feature = "verrsrc")]
pub type PMINIDUMP_MODULE_LIST = *mut MINIDUMP_MODULE_LIST;
pub type PMINIDUMP_PROCESS_VM_COUNTERS_1 = *mut MINIDUMP_PROCESS_VM_COUNTERS_1;
pub type PMINIDUMP_PROCESS_VM_COUNTERS_2 = *mut MINIDUMP_PROCESS_VM_COUNTERS_2;
pub type PMINIDUMP_PROCESS_VM_COUNTERS_N = *mut MINIDUMP_PROCESS_VM_COUNTERS_N;
pub type PMINIDUMP_READ_MEMORY_FAILURE_CALLBACK = *mut MINIDUMP_READ_MEMORY_FAILURE_CALLBACK;
pub type PMINIDUMP_STRING = *mut MINIDUMP_STRING;
pub type PMINIDUMP_SYSTEM_BASIC_INFORMATION = *mut MINIDUMP_SYSTEM_BASIC_INFORMATION;
pub type PMINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION = *mut MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION;
pub type PMINIDUMP_SYSTEM_FILECACHE_INFORMATION = *mut MINIDUMP_SYSTEM_FILECACHE_INFORMATION;
pub type PMINIDUMP_SYSTEM_INFO = *mut MINIDUMP_SYSTEM_INFO;
pub type PMINIDUMP_SYSTEM_MEMORY_INFO_1 = *mut MINIDUMP_SYSTEM_MEMORY_INFO_1;
pub type PMINIDUMP_SYSTEM_MEMORY_INFO_2 = *mut MINIDUMP_SYSTEM_MEMORY_INFO_2;
pub type PMINIDUMP_SYSTEM_MEMORY_INFO_N = *mut MINIDUMP_SYSTEM_MEMORY_INFO_N;
pub type PMINIDUMP_SYSTEM_PERFORMANCE_INFORMATION = *mut MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION;
pub type PMINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2 = *mut MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2;
pub type PMINIDUMP_THREAD = *mut MINIDUMP_THREAD;
#[cfg(feature = "winnt")]
pub type PMINIDUMP_THREAD_CALLBACK = *mut MINIDUMP_THREAD_CALLBACK;
pub type PMINIDUMP_THREAD_EX = *mut MINIDUMP_THREAD_EX;
#[cfg(feature = "winnt")]
pub type PMINIDUMP_THREAD_EX_CALLBACK = *mut MINIDUMP_THREAD_EX_CALLBACK;
pub type PMINIDUMP_THREAD_EX_LIST = *mut MINIDUMP_THREAD_EX_LIST;
pub type PMINIDUMP_THREAD_INFO = *mut MINIDUMP_THREAD_INFO;
pub type PMINIDUMP_THREAD_INFO_LIST = *mut MINIDUMP_THREAD_INFO_LIST;
pub type PMINIDUMP_THREAD_LIST = *mut MINIDUMP_THREAD_LIST;
pub type PMINIDUMP_THREAD_NAME = *mut MINIDUMP_THREAD_NAME;
pub type PMINIDUMP_THREAD_NAME_LIST = *mut MINIDUMP_THREAD_NAME_LIST;
pub type PMINIDUMP_TOKEN_INFO_HEADER = *mut MINIDUMP_TOKEN_INFO_HEADER;
pub type PMINIDUMP_TOKEN_INFO_LIST = *mut MINIDUMP_TOKEN_INFO_LIST;
pub type PMINIDUMP_UNLOADED_MODULE = *mut MINIDUMP_UNLOADED_MODULE;
pub type PMINIDUMP_UNLOADED_MODULE_LIST = *mut MINIDUMP_UNLOADED_MODULE_LIST;
pub type PMINIDUMP_USER_RECORD = *mut MINIDUMP_USER_RECORD;
pub type PMINIDUMP_USER_STREAM = *mut MINIDUMP_USER_STREAM;
pub type PMINIDUMP_USER_STREAM_INFORMATION = *mut MINIDUMP_USER_STREAM_INFORMATION;
pub type PMINIDUMP_VM_POST_READ_CALLBACK = *mut MINIDUMP_VM_POST_READ_CALLBACK;
pub type PMINIDUMP_VM_PRE_READ_CALLBACK = *mut MINIDUMP_VM_PRE_READ_CALLBACK;
pub type PMINIDUMP_VM_QUERY_CALLBACK = *mut MINIDUMP_VM_QUERY_CALLBACK;
#[cfg(feature = "winnt")]
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct XSTATE_CONFIG_FEATURE_MSC_INFO {
    pub SizeOfInfo: u32,
    pub ContextSize: u32,
    pub EnabledFeatures: u64,
    pub Features: [super::XSTATE_FEATURE; 64],
}
#[cfg(feature = "winnt")]
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
