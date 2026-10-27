pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64;
pub const PERF_100NSEC_MULTI_TIMER: i32 = 575735040;
pub const PERF_100NSEC_MULTI_TIMER_INV: i32 = 592512256;
pub const PERF_100NSEC_TIMER: i32 = 542180608;
pub const PERF_100NSEC_TIMER_INV: i32 = 558957824;
pub const PERF_AVERAGE_BASE: i32 = 1073939458;
pub const PERF_AVERAGE_BULK: i32 = 1073874176;
pub const PERF_AVERAGE_TIMER: i32 = 805438464;
pub const PERF_COUNTER_100NS_QUEUELEN_TYPE: i32 = 5571840;
pub const PERF_COUNTER_BASE: i32 = 196608;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: super::DWORD,
}
pub const PERF_COUNTER_BULK_COUNT: i32 = 272696576;
pub const PERF_COUNTER_COUNTER: i32 = 272696320;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: super::DWORD,
    pub CounterNameTitleIndex: super::DWORD,
    pub CounterNameTitle: windows_core::PWSTR,
    pub CounterHelpTitleIndex: super::DWORD,
    pub CounterHelpTitle: windows_core::PWSTR,
    pub DefaultScale: super::LONG,
    pub DetailLevel: super::DWORD,
    pub CounterType: super::DWORD,
    pub CounterSize: super::DWORD,
    pub CounterOffset: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: super::DWORD,
    pub CounterNameTitleIndex: super::DWORD,
    pub CounterNameTitle: super::DWORD,
    pub CounterHelpTitleIndex: super::DWORD,
    pub CounterHelpTitle: super::DWORD,
    pub DefaultScale: super::LONG,
    pub DetailLevel: super::DWORD,
    pub CounterType: super::DWORD,
    pub CounterSize: super::DWORD,
    pub CounterOffset: super::DWORD,
}
pub const PERF_COUNTER_DELTA: i32 = 4195328;
pub const PERF_COUNTER_ELAPSED: i32 = 262144;
pub const PERF_COUNTER_FRACTION: i32 = 131072;
pub const PERF_COUNTER_HISTOGRAM: i32 = 393216;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648;
pub const PERF_COUNTER_LARGE_DELTA: i32 = 4195584;
pub const PERF_COUNTER_LARGE_QUEUELEN_TYPE: i32 = 4523264;
pub const PERF_COUNTER_LARGE_RAWCOUNT: i32 = 65792;
pub const PERF_COUNTER_LARGE_RAWCOUNT_HEX: i32 = 256;
pub const PERF_COUNTER_MULTI_BASE: i32 = 1107494144;
pub const PERF_COUNTER_MULTI_TIMER: i32 = 574686464;
pub const PERF_COUNTER_MULTI_TIMER_INV: i32 = 591463680;
pub const PERF_COUNTER_NODATA: i32 = 1073742336;
pub const PERF_COUNTER_OBJ_TIME_QUEUELEN_TYPE: i32 = 6620416;
pub const PERF_COUNTER_PRECISION: i32 = 458752;
pub const PERF_COUNTER_QUEUELEN: i32 = 327680;
pub const PERF_COUNTER_QUEUELEN_TYPE: i32 = 4523008;
pub const PERF_COUNTER_RATE: i32 = 65536;
pub const PERF_COUNTER_RAWCOUNT: i32 = 65536;
pub const PERF_COUNTER_RAWCOUNT_HEX: i32 = 0;
pub const PERF_COUNTER_TEXT: i32 = 2816;
pub const PERF_COUNTER_TIMER: i32 = 541132032;
pub const PERF_COUNTER_TIMER_INV: i32 = 557909248;
pub const PERF_COUNTER_VALUE: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PERF_DATA_BLOCK {
    pub Signature: [super::WCHAR; 4],
    pub LittleEndian: super::DWORD,
    pub Version: super::DWORD,
    pub Revision: super::DWORD,
    pub TotalByteLength: super::DWORD,
    pub HeaderLength: super::DWORD,
    pub NumObjectTypes: super::DWORD,
    pub DefaultObject: super::LONG,
    pub SystemTime: super::SYSTEMTIME,
    pub PerfTime: super::LARGE_INTEGER,
    pub PerfFreq: super::LARGE_INTEGER,
    pub PerfTime100nSec: super::LARGE_INTEGER,
    pub SystemNameLength: super::DWORD,
    pub SystemNameOffset: super::DWORD,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERF_DATA_REVISION: i32 = 1;
pub const PERF_DATA_VERSION: i32 = 1;
pub const PERF_DELTA_BASE: i32 = 8388608;
pub const PERF_DELTA_COUNTER: i32 = 4194304;
pub const PERF_DETAIL_ADVANCED: i32 = 200;
pub const PERF_DETAIL_EXPERT: i32 = 300;
pub const PERF_DETAIL_NOVICE: i32 = 100;
pub const PERF_DETAIL_WIZARD: i32 = 400;
pub const PERF_DISPLAY_NOSHOW: i32 = 1073741824;
pub const PERF_DISPLAY_NO_SUFFIX: i32 = 0;
pub const PERF_DISPLAY_PERCENT: i32 = 536870912;
pub const PERF_DISPLAY_PER_SEC: i32 = 268435456;
pub const PERF_DISPLAY_SECONDS: i32 = 805306368;
pub const PERF_ELAPSED_TIME: i32 = 807666944;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: super::DWORD,
    pub ParentObjectTitleIndex: super::DWORD,
    pub ParentObjectInstance: super::DWORD,
    pub UniqueID: super::LONG,
    pub NameOffset: super::DWORD,
    pub NameLength: super::DWORD,
}
pub const PERF_INVERSE_COUNTER: i32 = 16777216;
pub const PERF_LARGE_RAW_BASE: i32 = 1073939712;
pub const PERF_LARGE_RAW_FRACTION: i32 = 537003264;
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3;
pub const PERF_MULTI_COUNTER: i32 = 33554432;
pub const PERF_NO_INSTANCES: i32 = -1;
pub const PERF_NO_UNIQUE_ID: i32 = -1;
pub const PERF_NUMBER_DECIMAL: i32 = 65536;
pub const PERF_NUMBER_DEC_1000: i32 = 131072;
pub const PERF_NUMBER_HEX: i32 = 0;
pub const PERF_OBJECT_TIMER: i32 = 2097152;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: super::DWORD,
    pub DefinitionLength: super::DWORD,
    pub HeaderLength: super::DWORD,
    pub ObjectNameTitleIndex: super::DWORD,
    pub ObjectNameTitle: windows_core::PWSTR,
    pub ObjectHelpTitleIndex: super::DWORD,
    pub ObjectHelpTitle: windows_core::PWSTR,
    pub DetailLevel: super::DWORD,
    pub NumCounters: super::DWORD,
    pub DefaultCounter: super::LONG,
    pub NumInstances: super::LONG,
    pub CodePage: super::DWORD,
    pub PerfTime: super::LARGE_INTEGER,
    pub PerfFreq: super::LARGE_INTEGER,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: super::DWORD,
    pub DefinitionLength: super::DWORD,
    pub HeaderLength: super::DWORD,
    pub ObjectNameTitleIndex: super::DWORD,
    pub ObjectNameTitle: super::DWORD,
    pub ObjectHelpTitleIndex: super::DWORD,
    pub ObjectHelpTitle: super::DWORD,
    pub DetailLevel: super::DWORD,
    pub NumCounters: super::DWORD,
    pub DefaultCounter: super::LONG,
    pub NumInstances: super::LONG,
    pub CodePage: super::DWORD,
    pub PerfTime: super::LARGE_INTEGER,
    pub PerfFreq: super::LARGE_INTEGER,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERF_OBJ_TIME_TIMER: i32 = 543229184;
pub const PERF_PRECISION_100NS_TIMER: i32 = 542573824;
pub const PERF_PRECISION_OBJECT_TIMER: i32 = 543622400;
pub const PERF_PRECISION_SYSTEM_TIMER: i32 = 541525248;
pub const PERF_PRECISION_TIMESTAMP: i32 = 1073939712;
pub const PERF_RAW_BASE: i32 = 1073939459;
pub const PERF_RAW_FRACTION: i32 = 537003008;
pub const PERF_SAMPLE_BASE: i32 = 1073939457;
pub const PERF_SAMPLE_COUNTER: i32 = 4260864;
pub const PERF_SAMPLE_FRACTION: i32 = 549585920;
pub const PERF_SIZE_DWORD: i32 = 0;
pub const PERF_SIZE_LARGE: i32 = 256;
pub const PERF_SIZE_VARIABLE_LEN: i32 = 768;
pub const PERF_SIZE_ZERO: i32 = 512;
pub const PERF_TEXT_ASCII: i32 = 65536;
pub const PERF_TEXT_UNICODE: i32 = 0;
pub const PERF_TIMER_100NS: i32 = 1048576;
pub const PERF_TIMER_TICK: i32 = 0;
pub const PERF_TYPE_COUNTER: i32 = 1024;
pub const PERF_TYPE_NUMBER: i32 = 0;
pub const PERF_TYPE_TEXT: i32 = 2048;
pub const PERF_TYPE_ZERO: i32 = 3072;
#[cfg(feature = "shared")]
pub type PM_CLOSE_PROC = Option<unsafe extern "system" fn() -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PM_COLLECT_PROC = Option<unsafe extern "system" fn(pvaluename: windows_core::PCWSTR, ppdata: *mut *mut core::ffi::c_void, pcbtotalbytes: *mut super::DWORD, pnumobjecttypes: *mut super::DWORD) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PM_OPEN_PROC = Option<unsafe extern "system" fn(pcontext: windows_core::PCWSTR) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_BLOCK = *mut PERF_COUNTER_BLOCK;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_DEFINITION = *mut PERF_COUNTER_DEFINITION;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PPERF_DATA_BLOCK = *mut PERF_DATA_BLOCK;
#[cfg(feature = "shared")]
pub type PPERF_INSTANCE_DEFINITION = *mut PERF_INSTANCE_DEFINITION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPERF_OBJECT_TYPE = *mut PERF_OBJECT_TYPE;
pub const WINPERF_LOG_DEBUG: i32 = 2;
pub const WINPERF_LOG_NONE: i32 = 0;
pub const WINPERF_LOG_USER: i32 = 1;
pub const WINPERF_LOG_VERBOSE: i32 = 3;
