#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfAddCounters(hquery : super::HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : super::DWORD) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : super::HANDLE) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfCreateInstance(providerhandle : super::HANDLE, countersetguid : super::LPCGUID, name : windows_sys::core::PCWSTR, id : super::ULONG) -> PPERF_COUNTERSET_INSTANCE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfDecrementULongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, value : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfDecrementULongLongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, value : super::ULONGLONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfDeleteCounters(hquery : super::HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : super::DWORD) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfDeleteInstance(provider : super::HANDLE, instanceblock : PPERF_COUNTERSET_INSTANCE) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn PerfEnumerateCounterSet(szmachine : windows_sys::core::PCWSTR, pcountersetids : super::LPGUID, ccountersetids : super::DWORD, pccountersetidsactual : super::LPDWORD) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn PerfEnumerateCounterSetInstances(szmachine : windows_sys::core::PCWSTR, pcountersetid : super::LPCGUID, pinstances : PPERF_INSTANCE_HEADER, cbinstances : super::DWORD, pcbinstancesactual : super::LPDWORD) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfIncrementULongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, value : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfIncrementULongLongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, value : super::ULONGLONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_sys::core::PCWSTR, phquery : *mut super::HANDLE) -> super::ULONG);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryCounterData(hquery : super::HANDLE, pcounterblock : PPERF_DATA_HEADER, cbcounterblock : super::DWORD, pcbcounterblockactual : super::LPDWORD) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : super::HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : super::DWORD, pcbcountersactual : super::LPDWORD) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryCounterSetRegistrationInfo(szmachine : windows_sys::core::PCWSTR, pcountersetid : super::LPCGUID, requestcode : PerfRegInfoType, requestlangid : super::DWORD, pbreginfo : super::LPBYTE, cbreginfo : super::DWORD, pcbreginfoactual : super::LPDWORD) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfQueryInstance(providerhandle : super::HANDLE, countersetguid : super::LPCGUID, name : windows_sys::core::PCWSTR, id : super::ULONG) -> PPERF_COUNTERSET_INSTANCE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfSetCounterRefValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, address : super::PVOID) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfSetCounterSetInfo(providerhandle : super::HANDLE, template : PPERF_COUNTERSET_INFO, templatesize : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfSetULongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, value : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfSetULongLongCounterValue(provider : super::HANDLE, instance : PPERF_COUNTERSET_INSTANCE, counterid : super::ULONG, value : super::ULONGLONG) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfStartProvider(providerguid : super::LPGUID, controlcallback : PERFLIBREQUEST, provider : super::PHANDLE) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfStartProviderEx(providerguid : super::LPGUID, providercontext : PPERF_PROVIDER_CONTEXT, provider : super::PHANDLE) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PerfStopProvider(providerhandle : super::HANDLE) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PERFLIBREQUEST = Option<unsafe extern "system" fn(requestcode: super::ULONG, buffer: super::PVOID, buffersize: super::ULONG) -> super::ULONG>;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub ProviderGuid: windows_sys::core::GUID,
    pub NumCounters: super::ULONG,
    pub InstanceType: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub dwSize: super::ULONG,
    pub InstanceId: super::ULONG,
    pub InstanceNameOffset: super::ULONG,
    pub InstanceNameSize: super::ULONG,
}
pub const PERF_COUNTERSET_INSTANCE_AGGREGATE: i32 = 22;
pub const PERF_COUNTERSET_MULTI_AGGREGATE: i32 = 6;
pub const PERF_COUNTERSET_MULTI_INSTANCES: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub CounterSetType: super::ULONG,
    pub DetailLevel: super::ULONG,
    pub NumCounters: super::ULONG,
    pub InstanceType: super::ULONG,
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: i32 = 4;
pub const PERF_COUNTERSET_SINGLE_AGGREGATE_HISTORY: i32 = 12;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: super::ULONG,
    pub dwSize: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: super::ULONG,
    pub dwType: PerfCounterDataType,
    pub dwSize: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub Status: super::ULONG,
    pub Size: super::ULONG,
    pub CounterId: super::ULONG,
    pub InstanceId: super::ULONG,
    pub Index: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub BufferSize: super::ULONG,
    pub CounterId: super::ULONG,
    pub InstanceId: super::ULONG,
    pub MachineOffset: super::ULONG,
    pub NameOffset: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_INFO {
    pub CounterId: super::ULONG,
    pub Type: super::ULONG,
    pub Attrib: super::ULONGLONG,
    pub Size: super::ULONG,
    pub DetailLevel: super::ULONG,
    pub Scale: super::LONG,
    pub Offset: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: super::ULONG,
    pub Type: super::ULONG,
    pub Attrib: super::ULONGLONG,
    pub DetailLevel: super::ULONG,
    pub DefaultScale: super::LONG,
    pub BaseCounterId: super::ULONG,
    pub PerfTimeId: super::ULONG,
    pub PerfFreqId: super::ULONG,
    pub MultiId: super::ULONG,
    pub AggregateFunc: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: super::ULONG,
    pub dwNumCounters: super::ULONG,
    pub PerfTimeStamp: super::LONGLONG,
    pub PerfTime100NSec: super::LONGLONG,
    pub PerfFreq: super::LONGLONG,
    pub SystemTime: super::SYSTEMTIME,
}
pub const PERF_ENUM_INSTANCES: i32 = 3;
pub const PERF_ERROR_RETURN: PerfCounterDataType = 0;
pub const PERF_FILTER: i32 = 9;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_INSTANCE_HEADER {
    pub Size: super::ULONG,
    pub InstanceId: super::ULONG,
}
pub const PERF_MAX_INSTANCE_NAME: i32 = 1024;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PERF_MEM_ALLOC = Option<unsafe extern "system" fn(allocsize: super::SIZE_T, pcontext: super::LPVOID) -> super::LPVOID>;
#[cfg(feature = "minwindef")]
pub type PERF_MEM_FREE = Option<unsafe extern "system" fn(pbuffer: super::LPVOID, pcontext: super::LPVOID)>;
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = 2;
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: super::ULONG,
    pub dwCounters: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: super::ULONG,
    pub dwInstances: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: super::DWORD,
    pub Reserved: super::DWORD,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: super::LPVOID,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: super::DWORD,
    pub dwCounters: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: super::DWORD,
    pub dwOffset: super::DWORD,
}
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295;
pub const PERF_WILDCARD_INSTANCE: windows_sys::core::PCWSTR = windows_sys::core::w!("*");
#[cfg(feature = "shared")]
pub type PPERF_COUNTERSET_INFO = *mut PERF_COUNTERSET_INFO;
#[cfg(feature = "shared")]
pub type PPERF_COUNTERSET_INSTANCE = *mut PERF_COUNTERSET_INSTANCE;
#[cfg(feature = "shared")]
pub type PPERF_COUNTERSET_REG_INFO = *mut PERF_COUNTERSET_REG_INFO;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_DATA = *mut PERF_COUNTER_DATA;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_HEADER = *mut PERF_COUNTER_HEADER;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_IDENTIFIER = *mut PERF_COUNTER_IDENTIFIER;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_IDENTITY = *mut PERF_COUNTER_IDENTITY;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_INFO = *mut PERF_COUNTER_INFO;
#[cfg(feature = "shared")]
pub type PPERF_COUNTER_REG_INFO = *mut PERF_COUNTER_REG_INFO;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type PPERF_DATA_HEADER = *mut PERF_DATA_HEADER;
#[cfg(feature = "shared")]
pub type PPERF_INSTANCE_HEADER = *mut PERF_INSTANCE_HEADER;
#[cfg(feature = "shared")]
pub type PPERF_MULTI_COUNTERS = *mut PERF_MULTI_COUNTERS;
#[cfg(feature = "shared")]
pub type PPERF_MULTI_INSTANCES = *mut PERF_MULTI_INSTANCES;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PPERF_PROVIDER_CONTEXT = *mut PERF_PROVIDER_CONTEXT;
#[cfg(feature = "shared")]
pub type PPERF_STRING_BUFFER_HEADER = *mut PERF_STRING_BUFFER_HEADER;
#[cfg(feature = "shared")]
pub type PPERF_STRING_COUNTER_HEADER = *mut PERF_STRING_COUNTER_HEADER;
pub type PerfCounterDataType = i32;
pub type PerfRegInfoType = i32;
