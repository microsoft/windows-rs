pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: u32 = 64;
pub const PERF_100NSEC_MULTI_TIMER: u32 = 575735040;
pub const PERF_100NSEC_MULTI_TIMER_INV: u32 = 592512256;
pub const PERF_100NSEC_TIMER: u32 = 542180608;
pub const PERF_100NSEC_TIMER_INV: u32 = 558957824;
pub const PERF_AVERAGE_BASE: u32 = 1073939458;
pub const PERF_AVERAGE_BULK: u32 = 1073874176;
pub const PERF_AVERAGE_TIMER: u32 = 805438464;
pub const PERF_COUNTER_100NS_QUEUELEN_TYPE: u32 = 5571840;
pub const PERF_COUNTER_BASE: u32 = 196608;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
pub const PERF_COUNTER_BULK_COUNT: u32 = 272696576;
pub const PERF_COUNTER_COUNTER: u32 = 272696320;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: windows_core::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: windows_core::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
pub const PERF_COUNTER_DELTA: u32 = 4195328;
pub const PERF_COUNTER_ELAPSED: u32 = 262144;
pub const PERF_COUNTER_FRACTION: u32 = 131072;
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648;
pub const PERF_COUNTER_LARGE_DELTA: u32 = 4195584;
pub const PERF_COUNTER_LARGE_QUEUELEN_TYPE: u32 = 4523264;
pub const PERF_COUNTER_LARGE_RAWCOUNT: u32 = 65792;
pub const PERF_COUNTER_LARGE_RAWCOUNT_HEX: u32 = 256;
pub const PERF_COUNTER_MULTI_BASE: u32 = 1107494144;
pub const PERF_COUNTER_MULTI_TIMER: u32 = 574686464;
pub const PERF_COUNTER_MULTI_TIMER_INV: u32 = 591463680;
pub const PERF_COUNTER_NODATA: u32 = 1073742336;
pub const PERF_COUNTER_OBJ_TIME_QUEUELEN_TYPE: u32 = 6620416;
pub const PERF_COUNTER_PRECISION: u32 = 458752;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680;
pub const PERF_COUNTER_QUEUELEN_TYPE: u32 = 4523008;
pub const PERF_COUNTER_RATE: u32 = 65536;
pub const PERF_COUNTER_RAWCOUNT: u32 = 65536;
pub const PERF_COUNTER_RAWCOUNT_HEX: u32 = 0;
pub const PERF_COUNTER_TEXT: u32 = 2816;
pub const PERF_COUNTER_TIMER: u32 = 541132032;
pub const PERF_COUNTER_TIMER_INV: u32 = 557909248;
pub const PERF_COUNTER_VALUE: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_minwinbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::minwinbase::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERF_DATA_REVISION: u32 = 1;
pub const PERF_DATA_VERSION: u32 = 1;
pub const PERF_DELTA_BASE: u32 = 8388608;
pub const PERF_DELTA_COUNTER: u32 = 4194304;
pub const PERF_DETAIL_ADVANCED: u32 = 200;
pub const PERF_DETAIL_EXPERT: u32 = 300;
pub const PERF_DETAIL_NOVICE: u32 = 100;
pub const PERF_DETAIL_WIZARD: u32 = 400;
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368;
pub const PERF_ELAPSED_TIME: u32 = 807666944;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
pub const PERF_INVERSE_COUNTER: u32 = 16777216;
pub const PERF_LARGE_RAW_BASE: u32 = 1073939712;
pub const PERF_LARGE_RAW_FRACTION: u32 = 537003264;
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3;
pub const PERF_MULTI_COUNTER: u32 = 33554432;
pub const PERF_NO_INSTANCES: i32 = -1;
pub const PERF_NO_UNIQUE_ID: i32 = -1;
pub const PERF_NUMBER_DECIMAL: u32 = 65536;
pub const PERF_NUMBER_DEC_1000: u32 = 131072;
pub const PERF_NUMBER_HEX: u32 = 0;
pub const PERF_OBJECT_TIMER: u32 = 2097152;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: windows_core::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: windows_core::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
pub const PERF_OBJ_TIME_TIMER: u32 = 543229184;
pub const PERF_PRECISION_100NS_TIMER: u32 = 542573824;
pub const PERF_PRECISION_OBJECT_TIMER: u32 = 543622400;
pub const PERF_PRECISION_SYSTEM_TIMER: u32 = 541525248;
pub const PERF_PRECISION_TIMESTAMP: u32 = 1073939712;
pub const PERF_RAW_BASE: u32 = 1073939459;
pub const PERF_RAW_FRACTION: u32 = 537003008;
pub const PERF_SAMPLE_BASE: u32 = 1073939457;
pub const PERF_SAMPLE_COUNTER: u32 = 4260864;
pub const PERF_SAMPLE_FRACTION: u32 = 549585920;
pub const PERF_SIZE_DWORD: u32 = 0;
pub const PERF_SIZE_LARGE: u32 = 256;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768;
pub const PERF_SIZE_ZERO: u32 = 512;
pub const PERF_TEXT_ASCII: u32 = 65536;
pub const PERF_TEXT_UNICODE: u32 = 0;
pub const PERF_TIMER_100NS: u32 = 1048576;
pub const PERF_TIMER_TICK: u32 = 0;
pub const PERF_TYPE_COUNTER: u32 = 1024;
pub const PERF_TYPE_NUMBER: u32 = 0;
pub const PERF_TYPE_TEXT: u32 = 2048;
pub const PERF_TYPE_ZERO: u32 = 3072;
pub type PM_CLOSE_PROC = Option<unsafe extern "system" fn() -> u32>;
pub type PM_COLLECT_PROC = Option<unsafe extern "system" fn(pvaluename: windows_core::PCWSTR, ppdata: *mut *mut core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
pub type PM_OPEN_PROC = Option<unsafe extern "system" fn(pcontext: windows_core::PCWSTR) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERF_COUNTER_BLOCK(pub *mut PERF_COUNTER_BLOCK);
impl PPERF_COUNTER_BLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERF_COUNTER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERF_COUNTER_DEFINITION(pub *mut PERF_COUNTER_DEFINITION);
impl PPERF_COUNTER_DEFINITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwinbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERF_DATA_BLOCK(pub *mut PERF_DATA_BLOCK);
#[cfg(feature = "Win32_minwinbase")]
impl PPERF_DATA_BLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for PPERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERF_INSTANCE_DEFINITION(pub *mut PERF_INSTANCE_DEFINITION);
impl PPERF_INSTANCE_DEFINITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERF_INSTANCE_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERF_OBJECT_TYPE(pub *mut PERF_OBJECT_TYPE);
impl PPERF_OBJECT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINPERF_LOG_DEBUG: u32 = 2;
pub const WINPERF_LOG_NONE: u32 = 0;
pub const WINPERF_LOG_USER: u32 = 1;
pub const WINPERF_LOG_VERBOSE: u32 = 3;
