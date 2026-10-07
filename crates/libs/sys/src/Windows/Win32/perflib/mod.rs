#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfAddCounters(hquery : super::HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : super::HANDLE) -> u32);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfCreateInstance(providerhandle : super::HANDLE, countersetguid : super::LPCGUID, name : windows_sys::core::PCWSTR, id : u32) -> PPERF_COUNTERSET_INSTANCE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfDecrementULongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfDecrementULongLongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfDeleteCounters(hquery : super::HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfDeleteInstance(provider : super::HANDLE, instanceblock : PPERF_COUNTERSET_INSTANCE) -> u32);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn PerfEnumerateCounterSet(szmachine : windows_sys::core::PCWSTR, pcountersetids : super::LPGUID, ccountersetids : u32, pccountersetidsactual : super::LPDWORD) -> u32);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn PerfEnumerateCounterSetInstances(szmachine : windows_sys::core::PCWSTR, pcountersetid : super::LPCGUID, pinstances : PPERF_INSTANCE_HEADER, cbinstances : u32, pcbinstancesactual : super::LPDWORD) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfIncrementULongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfIncrementULongLongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_sys::core::PCWSTR, phquery : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryCounterData(hquery : super::HANDLE, pcounterblock : PPERF_DATA_HEADER, cbcounterblock : u32, pcbcounterblockactual : super::LPDWORD) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : super::HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : u32, pcbcountersactual : super::LPDWORD) -> u32);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryCounterSetRegistrationInfo(szmachine : windows_sys::core::PCWSTR, pcountersetid : super::LPCGUID, requestcode : PerfRegInfoType, requestlangid : u32, pbreginfo : super::LPBYTE, cbreginfo : u32, pcbreginfoactual : super::LPDWORD) -> u32);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryInstance(providerhandle : super::HANDLE, countersetguid : super::LPCGUID, name : windows_sys::core::PCWSTR, id : u32) -> PPERF_COUNTERSET_INSTANCE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfSetCounterRefValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, address : *const core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfSetCounterSetInfo(providerhandle : super::HANDLE, template : PPERF_COUNTERSET_INFO, templatesize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfSetULongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfSetULongLongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfStartProvider(providerguid : super::LPGUID, controlcallback : PERFLIBREQUEST, phprovider : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfStartProviderEx(providerguid : super::LPGUID, providercontext : PPERF_PROVIDER_CONTEXT, provider : super::PHANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PerfStopProvider(providerhandle : super::HANDLE) -> u32);
pub type PERFLIBREQUEST = Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut core::ffi::c_void, buffersize: u32) -> u32>;
pub const PERF_ADD_COUNTER: i32 = 1;
pub const PERF_AGGREGATE_AVG: i32 = 2;
pub const PERF_AGGREGATE_INSTANCE: windows_sys::core::PCWSTR = windows_sys::core::w!("_Total");
pub const PERF_AGGREGATE_MAX: i32 = 4;
pub const PERF_AGGREGATE_MIN: i32 = 3;
pub const PERF_AGGREGATE_TOTAL: i32 = 1;
pub const PERF_AGGREGATE_UNDEFINED: i32 = 0;
pub const PERF_ATTRIB_BY_REFERENCE: i32 = 1;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: i32 = 16;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: i32 = 8;
pub const PERF_ATTRIB_NO_DISPLAYABLE: i32 = 2;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: i32 = 4;
pub const PERF_COLLECT_END: i32 = 6;
pub const PERF_COLLECT_START: i32 = 5;
pub const PERF_COUNTERSET: PerfCounterDataType = 6;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: i32 = 4;
pub const PERF_COUNTERSET_FLAG_HISTORY: i32 = 8;
pub const PERF_COUNTERSET_FLAG_INSTANCE: i32 = 16;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub ProviderGuid: windows_sys::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
pub const PERF_COUNTERSET_INSTANCE_AGGREGATE: i32 = 22;
pub const PERF_COUNTERSET_MULTI_AGGREGATE: i32 = 6;
pub const PERF_COUNTERSET_MULTI_INSTANCES: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: i32 = 4;
pub const PERF_COUNTERSET_SINGLE_AGGREGATE_HISTORY: i32 = 12;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy, Default)]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::SYSTEMTIME,
}
pub const PERF_ENUM_INSTANCES: i32 = 3;
pub const PERF_ERROR_RETURN: PerfCounterDataType = 0;
pub const PERF_FILTER: i32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
pub const PERF_MAX_INSTANCE_NAME: i32 = 1024;
pub type PERF_MEM_ALLOC = Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PERF_MEM_FREE = Option<unsafe extern "system" fn(pbuffer: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void)>;
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = 2;
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut core::ffi::c_void,
}
pub const PERF_PROVIDER_DRIVER: i32 = 2;
pub const PERF_PROVIDER_KERNEL_MODE: i32 = 1;
pub const PERF_PROVIDER_USER_MODE: i32 = 0;
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = 9;
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = 4;
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = 3;
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = 1;
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = 10;
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = 6;
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = 5;
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = 2;
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = 8;
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = 7;
pub const PERF_REMOVE_COUNTER: i32 = 2;
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295;
pub const PERF_WILDCARD_INSTANCE: windows_sys::core::PCWSTR = windows_sys::core::w!("*");
pub type PPERF_COUNTERSET_INFO = *mut PERF_COUNTERSET_INFO;
pub type PPERF_COUNTERSET_INSTANCE = *mut PERF_COUNTERSET_INSTANCE;
pub type PPERF_COUNTERSET_REG_INFO = *mut PERF_COUNTERSET_REG_INFO;
pub type PPERF_COUNTER_DATA = *mut PERF_COUNTER_DATA;
pub type PPERF_COUNTER_HEADER = *mut PERF_COUNTER_HEADER;
pub type PPERF_COUNTER_IDENTIFIER = *mut PERF_COUNTER_IDENTIFIER;
pub type PPERF_COUNTER_IDENTITY = *mut PERF_COUNTER_IDENTITY;
pub type PPERF_COUNTER_INFO = *mut PERF_COUNTER_INFO;
pub type PPERF_COUNTER_REG_INFO = *mut PERF_COUNTER_REG_INFO;
#[cfg(feature = "minwinbase")]
pub type PPERF_DATA_HEADER = *mut PERF_DATA_HEADER;
pub type PPERF_INSTANCE_HEADER = *mut PERF_INSTANCE_HEADER;
pub type PPERF_MULTI_COUNTERS = *mut PERF_MULTI_COUNTERS;
pub type PPERF_MULTI_INSTANCES = *mut PERF_MULTI_INSTANCES;
pub type PPERF_PROVIDER_CONTEXT = *mut PERF_PROVIDER_CONTEXT;
pub type PPERF_STRING_BUFFER_HEADER = *mut PERF_STRING_BUFFER_HEADER;
pub type PPERF_STRING_COUNTER_HEADER = *mut PERF_STRING_COUNTER_HEADER;
pub type PerfCounterDataType = i32;
pub type PerfRegInfoType = i32;
