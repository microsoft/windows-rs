#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn CloseTrace(tracehandle : PROCESSTRACE_HANDLE) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn ControlTraceA(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCSTR, properties : PEVENT_TRACE_PROPERTIES, controlcode : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn ControlTraceW(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCWSTR, properties : PEVENT_TRACE_PROPERTIES, controlcode : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateTraceInstanceId(reghandle : super::HANDLE, instinfo : PEVENT_INSTANCE_INFO) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnableTrace(enable : super::ULONG, enableflag : super::ULONG, enablelevel : super::ULONG, controlguid : super::LPCGUID, traceid : CONTROLTRACE_ID) -> super::ULONG);
#[cfg(all(feature = "evntprov", feature = "guiddef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnableTraceEx(providerid : super::LPCGUID, sourceid : super::LPCGUID, traceid : CONTROLTRACE_ID, isenabled : super::ULONG, level : super::UCHAR, matchanykeyword : super::ULONGLONG, matchallkeyword : super::ULONGLONG, enableproperty : super::ULONG, enablefilterdesc : super::PEVENT_FILTER_DESCRIPTOR) -> super::ULONG);
#[cfg(all(feature = "evntprov", feature = "guiddef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnableTraceEx2(traceid : CONTROLTRACE_ID, providerid : super::LPCGUID, controlcode : super::ULONG, level : super::UCHAR, matchanykeyword : super::ULONGLONG, matchallkeyword : super::ULONGLONG, timeout : super::ULONG, enableparameters : PENABLE_TRACE_PARAMETERS) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumerateTraceGuids(guidpropertiesarray : *mut PTRACE_GUID_PROPERTIES, propertyarraycount : super::ULONG, guidcount : super::PULONG) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn EnumerateTraceGuidsEx(tracequeryinfoclass : TRACE_QUERY_INFO_CLASS, inbuffer : super::PVOID, inbuffersize : super::ULONG, outbuffer : super::PVOID, outbuffersize : super::ULONG, returnlength : super::PULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn FlushTraceA(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn FlushTraceW(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCWSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn GetTraceEnableFlags(tracehandle : TRACELOGGER_HANDLE) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn GetTraceEnableLevel(tracehandle : TRACELOGGER_HANDLE) -> super::UCHAR);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetTraceLoggerHandle(buffer : super::PVOID) -> TRACELOGGER_HANDLE);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn OpenTraceA(logfile : PEVENT_TRACE_LOGFILEA) -> PROCESSTRACE_HANDLE);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn OpenTraceFromBufferStream(options : *const ETW_OPEN_TRACE_OPTIONS, buffercompletioncallback : PETW_BUFFER_COMPLETION_CALLBACK, buffercompletioncontext : *const core::ffi::c_void) -> PROCESSTRACE_HANDLE);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn OpenTraceFromFile(logfilename : windows_sys::core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, logfileheader : *mut TRACE_LOGFILE_HEADER) -> PROCESSTRACE_HANDLE);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn OpenTraceFromRealTimeLogger(loggername : windows_sys::core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, logfileheader : *mut TRACE_LOGFILE_HEADER) -> PROCESSTRACE_HANDLE);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn OpenTraceFromRealTimeLoggerWithAllocationOptions(loggername : windows_sys::core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, allocationsize : super::ULONG_PTR, memorypartitionhandle : super::HANDLE, logfileheader : *mut TRACE_LOGFILE_HEADER) -> PROCESSTRACE_HANDLE);
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn OpenTraceW(logfile : PEVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn ProcessTrace(handlearray : *const PROCESSTRACE_HANDLE, handlecount : super::ULONG, starttime : super::LPFILETIME, endtime : super::LPFILETIME) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn ProcessTraceAddBufferToBufferStream(tracehandle : PROCESSTRACE_HANDLE, buffer : *const ETW_BUFFER_HEADER, buffersize : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn ProcessTraceBufferDecrementReference(buffer : *const ETW_BUFFER_HEADER) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn ProcessTraceBufferIncrementReference(tracehandle : PROCESSTRACE_HANDLE, buffer : *const ETW_BUFFER_HEADER) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn QueryAllTracesA(propertyarray : *mut PEVENT_TRACE_PROPERTIES, propertyarraycount : super::ULONG, loggercount : super::PULONG) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn QueryAllTracesW(propertyarray : *mut PEVENT_TRACE_PROPERTIES, propertyarraycount : super::ULONG, loggercount : super::PULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn QueryTraceA(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryTraceProcessingHandle(processinghandle : PROCESSTRACE_HANDLE, informationclass : ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer : super::PVOID, inbuffersize : super::ULONG, outbuffer : super::PVOID, outbuffersize : super::ULONG, returnlength : super::PULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn QueryTraceW(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCWSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn RegisterTraceGuidsA(requestaddress : WMIDPREQUEST, requestcontext : super::PVOID, controlguid : super::LPCGUID, guidcount : super::ULONG, traceguidreg : PTRACE_GUID_REGISTRATION, mofimagepath : windows_sys::core::PCSTR, mofresourcename : windows_sys::core::PCSTR, registrationhandle : *mut TRACEGUID_HANDLE) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn RegisterTraceGuidsW(requestaddress : WMIDPREQUEST, requestcontext : super::PVOID, controlguid : super::LPCGUID, guidcount : super::ULONG, traceguidreg : PTRACE_GUID_REGISTRATION, mofimagepath : windows_sys::core::PCWSTR, mofresourcename : windows_sys::core::PCWSTR, registrationhandle : *mut TRACEGUID_HANDLE) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn RemoveTraceCallback(pguid : super::LPCGUID) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetTraceCallback(pguid : super::LPCGUID, eventcallback : PEVENT_CALLBACK) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn StartTraceA(traceid : *mut CONTROLTRACE_ID, instancename : windows_sys::core::PCSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn StartTraceW(traceid : *mut CONTROLTRACE_ID, instancename : windows_sys::core::PCWSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn StopTraceA(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn StopTraceW(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCWSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-eventing-consumer-l1-1-2.dll" "system" fn TraceConfigureLastBranchRecord(traceid : CONTROLTRACE_ID, lbrconfiguration : TRACE_LBR_CONFIGURATION, events : *const CLASSIC_EVENT_ID, eventcount : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TraceEvent(tracehandle : TRACELOGGER_HANDLE, eventtrace : PEVENT_TRACE_HEADER) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TraceEventInstance(tracehandle : TRACELOGGER_HANDLE, eventtrace : PEVENT_INSTANCE_HEADER, instinfo : PEVENT_INSTANCE_INFO, parentinstinfo : PEVENT_INSTANCE_INFO) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("advapi32.dll" "C" fn TraceMessage(loggerhandle : TRACELOGGER_HANDLE, messageflags : super::ULONG, messageguid : super::LPCGUID, messagenumber : super::USHORT, ...) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn TraceMessageVa(loggerhandle : TRACELOGGER_HANDLE, messageflags : super::ULONG, messageguid : super::LPCGUID, messagenumber : super::USHORT, messagearglist : super::va_list) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TraceQueryInformation(traceid : CONTROLTRACE_ID, informationclass : TRACE_INFO_CLASS, traceinformation : super::PVOID, informationlength : super::ULONG, returnlength : super::PULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TraceSetInformation(traceid : CONTROLTRACE_ID, informationclass : TRACE_INFO_CLASS, traceinformation : super::PVOID, informationlength : super::ULONG) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn UnregisterTraceGuids(registrationhandle : TRACEGUID_HANDLE) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn UpdateTraceA(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
windows_link::link!("advapi32.dll" "system" fn UpdateTraceW(traceid : CONTROLTRACE_ID, instancename : windows_sys::core::PCWSTR, properties : PEVENT_TRACE_PROPERTIES) -> super::ULONG);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CLASSIC_EVENT_ID {
    pub EventGuid: windows_sys::core::GUID,
    pub Type: super::UCHAR,
    pub Reserved: [super::UCHAR; 7],
}
#[cfg(feature = "shared")]
impl Default for CLASSIC_EVENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CONTROLTRACE_ID = u64;
pub const DIAG_LOGGER_NAMEA: windows_sys::core::PCSTR = windows_sys::core::s!("DiagLog");
pub const DIAG_LOGGER_NAMEW: windows_sys::core::PCWSTR = windows_sys::core::w!("DiagLog");
pub const DefaultTraceSecurityGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0811c1af_7a07_4a06_82ed_869455cdf713);
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: super::ULONG,
    pub EnableProperty: super::ULONG,
    pub ControlFlags: super::ULONG,
    pub SourceId: windows_sys::core::GUID,
    pub EnableFilterDesc: super::PEVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: super::ULONG,
    pub EnableProperty: super::ULONG,
    pub ControlFlags: super::ULONG,
    pub SourceId: windows_sys::core::GUID,
    pub EnableFilterDesc: super::PEVENT_FILTER_DESCRIPTOR,
}
pub const ENABLE_TRACE_PARAMETERS_VERSION: i32 = 1;
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: i32 = 2;
pub const ETW_ASCIICHAR_TYPE_VALUE: i32 = 102;
pub const ETW_ASCIISTRING_TYPE_VALUE: i32 = 103;
pub const ETW_BOOLEAN_TYPE_VALUE: i32 = 14;
pub const ETW_BOOL_TYPE_VALUE: i32 = 108;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ETW_BUFFER_CALLBACK_INFORMATION {
    pub TraceHandle: PROCESSTRACE_HANDLE,
    pub LogfileHeader: *const TRACE_LOGFILE_HEADER,
    pub BuffersRead: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for ETW_BUFFER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for ETW_BUFFER_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: super::UCHAR,
    pub Alignment: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ETW_BUFFER_HEADER {
    pub Reserved1: [super::ULONG; 4],
    pub TimeStamp: super::LARGE_INTEGER,
    pub Reserved2: [super::ULONG; 4],
    pub ClientContext: ETW_BUFFER_CONTEXT,
    pub Reserved3: super::ULONG,
    pub FilledBytes: super::ULONG,
    pub Reserved4: [super::ULONG; 5],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ETW_BUFFER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ETW_BYTE_TYPE_VALUE: i32 = 4;
pub const ETW_CHAR_TYPE_VALUE: i32 = 11;
pub type ETW_COMPRESSION_RESUMPTION_MODE = i32;
pub type ETW_CONTEXT_REGISTER_TYPES = i32;
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: i32 = 109;
pub const ETW_COUNTED_STRING_TYPE_VALUE: i32 = 104;
pub const ETW_DATETIME_TYPE_VALUE: i32 = 119;
pub const ETW_DECIMAL_TYPE_VALUE: i32 = 15;
pub const ETW_DOUBLE_TYPE_VALUE: i32 = 13;
pub const ETW_GUID_TYPE_VALUE: i32 = 101;
pub const ETW_HIDDEN_TYPE_VALUE: i32 = 107;
pub const ETW_INT16_TYPE_VALUE: i32 = 5;
pub const ETW_INT32_TYPE_VALUE: i32 = 7;
pub const ETW_INT64_TYPE_VALUE: i32 = 9;
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: i32 = 112;
pub const ETW_NULL_TYPE_VALUE: i32 = 0;
pub const ETW_OBJECT_TYPE_VALUE: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ETW_OPEN_TRACE_OPTIONS {
    pub ProcessTraceModes: ETW_PROCESS_TRACE_MODES,
    pub EventCallback: PEVENT_RECORD_CALLBACK,
    pub EventCallbackContext: *mut core::ffi::c_void,
    pub BufferCallback: PETW_BUFFER_CALLBACK,
    pub BufferCallbackContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: super::ULONG,
    pub OwnerTag: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    pub ProcessorNumber: super::ULONG,
    pub NumberOfCounters: super::ULONG,
    pub CounterOwners: [ETW_PMC_COUNTER_OWNER; 1],
}
#[cfg(feature = "shared")]
impl Default for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ETW_PMC_COUNTER_OWNER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ETW_PMC_SESSION_INFO {
    pub NextEntryOffset: super::ULONG,
    pub LoggerId: super::USHORT,
    pub Reserved: super::USHORT,
    pub ProfileSourceCount: super::ULONG,
    pub HookIdCount: super::ULONG,
}
pub const ETW_POINTER_TYPE_VALUE: i32 = 105;
pub type ETW_PROCESS_HANDLE_INFO_TYPE = i32;
pub type ETW_PROCESS_TRACE_MODES = i32;
pub const ETW_PROCESS_TRACE_MODE_NONE: ETW_PROCESS_TRACE_MODES = 0;
pub const ETW_PROCESS_TRACE_MODE_RAW_TIMESTAMP: ETW_PROCESS_TRACE_MODES = 1;
pub const ETW_PTVECTOR_TYPE_VALUE: i32 = 117;
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: i32 = 113;
pub const ETW_REDUCED_STRING_TYPE_VALUE: i32 = 114;
pub const ETW_REFRENCE_TYPE_VALUE: i32 = 120;
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: i32 = 111;
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: i32 = 110;
pub const ETW_SBYTE_TYPE_VALUE: i32 = 3;
pub const ETW_SID_TYPE_VALUE: i32 = 115;
pub const ETW_SINGLE_TYPE_VALUE: i32 = 12;
pub const ETW_SIZET_TYPE_VALUE: i32 = 106;
pub const ETW_STRING_TYPE_VALUE: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: windows_sys::core::GUID,
    pub ParentId: windows_sys::core::GUID,
    pub QpcOffsetFromRoot: super::LONG64,
    pub PartitionType: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: super::LONG64,
    pub PartitionType: super::ULONG,
    pub PartitionId: windows_sys::core::PWSTR,
    pub ParentId: windows_sys::core::PWSTR,
}
pub const ETW_UINT16_TYPE_VALUE: i32 = 6;
pub const ETW_UINT32_TYPE_VALUE: i32 = 8;
pub const ETW_UINT64_TYPE_VALUE: i32 = 10;
pub const ETW_VARIANT_TYPE_VALUE: i32 = 116;
pub const ETW_WMITIME_TYPE_VALUE: i32 = 118;
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: i32 = 2;
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: i32 = 0;
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER {
    pub Size: super::USHORT,
    pub Anonymous: EVENT_INSTANCE_HEADER_0,
    pub Anonymous2: EVENT_INSTANCE_HEADER_1,
    pub ThreadId: super::ULONG,
    pub ProcessId: super::ULONG,
    pub TimeStamp: super::LARGE_INTEGER,
    pub RegHandle: super::ULONGLONG,
    pub InstanceId: super::ULONG,
    pub ParentInstanceId: super::ULONG,
    pub Anonymous3: EVENT_INSTANCE_HEADER_2,
    pub ParentRegHandle: super::ULONGLONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_0 {
    pub FieldTypeFlags: super::USHORT,
    pub Anonymous: EVENT_INSTANCE_HEADER_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_INSTANCE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_INSTANCE_HEADER_0_0 {
    pub HeaderType: super::UCHAR,
    pub MarkerFlags: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_1 {
    pub Version: super::ULONG,
    pub Class: EVENT_INSTANCE_HEADER_1_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_INSTANCE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: super::UCHAR,
    pub Level: super::UCHAR,
    pub Version: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: super::ULONG64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_INSTANCE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: super::ULONG,
    pub UserTime: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: super::ULONG,
    pub Flags: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::HANDLE,
    pub InstanceId: super::ULONG,
}
pub const EVENT_LOGGER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("EventLog");
pub const EVENT_LOGGER_NAMEA: windows_sys::core::PCSTR = windows_sys::core::s!("EventLog");
pub const EVENT_LOGGER_NAMEW: windows_sys::core::PCWSTR = windows_sys::core::w!("EventLog");
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: super::ULONG,
    pub ParentInstanceId: super::ULONG,
    pub ParentGuid: windows_sys::core::GUID,
    pub MofData: super::PVOID,
    pub MofLength: super::ULONG,
    pub Anonymous: EVENT_TRACE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_0 {
    pub ClientContext: super::ULONG,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648;
pub const EVENT_TRACE_ADD_HEADER_MODE: i32 = 4096;
pub const EVENT_TRACE_BUFFERING_MODE: i32 = 1024;
pub const EVENT_TRACE_COMPRESSED_MODE: i32 = 67108864;
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: i32 = 5;
pub const EVENT_TRACE_CONTROL_FLUSH: i32 = 3;
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: i32 = 4;
pub const EVENT_TRACE_CONTROL_QUERY: i32 = 0;
pub const EVENT_TRACE_CONTROL_STOP: i32 = 1;
pub const EVENT_TRACE_CONTROL_UPDATE: i32 = 2;
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: i32 = 512;
pub const EVENT_TRACE_FILE_MODE_APPEND: i32 = 4;
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: i32 = 2;
pub const EVENT_TRACE_FILE_MODE_NEWFILE: i32 = 8;
pub const EVENT_TRACE_FILE_MODE_NONE: i32 = 0;
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: i32 = 32;
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: i32 = 1;
pub const EVENT_TRACE_FLAG_ALPC: i32 = 1048576;
pub const EVENT_TRACE_FLAG_CSWITCH: i32 = 16;
pub const EVENT_TRACE_FLAG_DBGPRINT: i32 = 262144;
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: i32 = 4194304;
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: i32 = 512;
pub const EVENT_TRACE_FLAG_DISK_IO: i32 = 256;
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: i32 = 1024;
pub const EVENT_TRACE_FLAG_DISPATCHER: i32 = 2048;
pub const EVENT_TRACE_FLAG_DPC: i32 = 32;
pub const EVENT_TRACE_FLAG_DRIVER: i32 = 8388608;
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: i32 = 536870912;
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648;
pub const EVENT_TRACE_FLAG_FILE_IO: i32 = 33554432;
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: i32 = 67108864;
pub const EVENT_TRACE_FLAG_FORWARD_WMI: i32 = 1073741824;
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: i32 = 4;
pub const EVENT_TRACE_FLAG_INTERRUPT: i32 = 64;
pub const EVENT_TRACE_FLAG_JOB: i32 = 524288;
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: i32 = 8192;
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: i32 = 4096;
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: i32 = 65536;
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: i32 = 268435456;
pub const EVENT_TRACE_FLAG_PROCESS: i32 = 1;
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: i32 = 8;
pub const EVENT_TRACE_FLAG_PROFILE: i32 = 16777216;
pub const EVENT_TRACE_FLAG_REGISTRY: i32 = 131072;
pub const EVENT_TRACE_FLAG_SPLIT_IO: i32 = 2097152;
pub const EVENT_TRACE_FLAG_SYSTEMCALL: i32 = 128;
pub const EVENT_TRACE_FLAG_THREAD: i32 = 2;
pub const EVENT_TRACE_FLAG_VAMAP: i32 = 32768;
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: i32 = 16384;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER {
    pub Size: super::USHORT,
    pub Anonymous: EVENT_TRACE_HEADER_0,
    pub Anonymous2: EVENT_TRACE_HEADER_1,
    pub ThreadId: super::ULONG,
    pub ProcessId: super::ULONG,
    pub TimeStamp: super::LARGE_INTEGER,
    pub Anonymous3: EVENT_TRACE_HEADER_2,
    pub Anonymous4: EVENT_TRACE_HEADER_3,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: super::USHORT,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: super::UCHAR,
    pub MarkerFlags: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: super::ULONG,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: super::UCHAR,
    pub Level: super::UCHAR,
    pub Version: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: windows_sys::core::GUID,
    pub GuidPtr: super::ULONGLONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: super::ULONG64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: super::ULONG,
    pub UserTime: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: super::ULONG,
    pub Flags: super::ULONG,
}
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: i32 = 134217728;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type EVENT_TRACE_LOGFILEA = _EVENT_TRACE_LOGFILEA;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type EVENT_TRACE_LOGFILEW = _EVENT_TRACE_LOGFILEW;
pub const EVENT_TRACE_MODE_RESERVED: i32 = 1048576;
pub const EVENT_TRACE_NONSTOPPABLE_MODE: i32 = 64;
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: i32 = 268435456;
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: i32 = 8388608;
pub const EVENT_TRACE_PRIVATE_IN_PROC: i32 = 131072;
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: i32 = 2048;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES {
    pub Wnode: super::WNODE_HEADER,
    pub BufferSize: super::ULONG,
    pub MinimumBuffers: super::ULONG,
    pub MaximumBuffers: super::ULONG,
    pub MaximumFileSize: super::ULONG,
    pub LogFileMode: super::ULONG,
    pub FlushTimer: super::ULONG,
    pub EnableFlags: super::ULONG,
    pub Anonymous: EVENT_TRACE_PROPERTIES_0,
    pub NumberOfBuffers: super::ULONG,
    pub FreeBuffers: super::ULONG,
    pub EventsLost: super::ULONG,
    pub BuffersWritten: super::ULONG,
    pub LogBuffersLost: super::ULONG,
    pub RealTimeBuffersLost: super::ULONG,
    pub LoggerThreadId: super::HANDLE,
    pub LogFileNameOffset: super::ULONG,
    pub LoggerNameOffset: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: super::LONG,
    pub FlushThreshold: super::LONG,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES_V2 {
    pub Wnode: super::WNODE_HEADER,
    pub BufferSize: super::ULONG,
    pub MinimumBuffers: super::ULONG,
    pub MaximumBuffers: super::ULONG,
    pub MaximumFileSize: super::ULONG,
    pub LogFileMode: super::ULONG,
    pub FlushTimer: super::ULONG,
    pub EnableFlags: super::ULONG,
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_0,
    pub NumberOfBuffers: super::ULONG,
    pub FreeBuffers: super::ULONG,
    pub EventsLost: super::ULONG,
    pub BuffersWritten: super::ULONG,
    pub LogBuffersLost: super::ULONG,
    pub RealTimeBuffersLost: super::ULONG,
    pub LoggerThreadId: super::HANDLE,
    pub LogFileNameOffset: super::ULONG,
    pub LoggerNameOffset: super::ULONG,
    pub Anonymous2: EVENT_TRACE_PROPERTIES_V2_1,
    pub FilterDescCount: super::ULONG,
    pub FilterDesc: super::PEVENT_FILTER_DESCRIPTOR,
    pub Anonymous3: EVENT_TRACE_PROPERTIES_V2_2,
}
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: super::LONG,
    pub FlushThreshold: super::LONG,
}
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: super::ULONG,
}
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: super::ULONG64,
}
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
pub const EVENT_TRACE_REAL_TIME_MODE: i32 = 256;
pub const EVENT_TRACE_RELOG_MODE: i32 = 65536;
pub const EVENT_TRACE_SECURE_MODE: i32 = 128;
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: i32 = 4194304;
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: i32 = 33554432;
pub const EVENT_TRACE_TYPE_ACCEPT: i32 = 15;
pub const EVENT_TRACE_TYPE_ACKDUP: i32 = 22;
pub const EVENT_TRACE_TYPE_ACKFULL: i32 = 20;
pub const EVENT_TRACE_TYPE_ACKPART: i32 = 21;
pub const EVENT_TRACE_TYPE_CHECKPOINT: i32 = 8;
pub const EVENT_TRACE_TYPE_CONFIG: i32 = 11;
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: i32 = 37;
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: i32 = 29;
pub const EVENT_TRACE_TYPE_CONFIG_CPU: i32 = 10;
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: i32 = 31;
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: i32 = 33;
pub const EVENT_TRACE_TYPE_CONFIG_DPI: i32 = 28;
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: i32 = 34;
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: i32 = 23;
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: i32 = 21;
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: i32 = 12;
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: i32 = 30;
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: i32 = 32;
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: i32 = 17;
pub const EVENT_TRACE_TYPE_CONFIG_NIC: i32 = 13;
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: i32 = 24;
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: i32 = 18;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: i32 = 11;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK_EX: i32 = 19;
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: i32 = 25;
pub const EVENT_TRACE_TYPE_CONFIG_PNP: i32 = 22;
pub const EVENT_TRACE_TYPE_CONFIG_POWER: i32 = 16;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: i32 = 35;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: i32 = 26;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: i32 = 27;
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: i32 = 15;
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: i32 = 14;
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: i32 = 36;
pub const EVENT_TRACE_TYPE_CONNECT: i32 = 12;
pub const EVENT_TRACE_TYPE_CONNFAIL: i32 = 17;
pub const EVENT_TRACE_TYPE_COPY_ARP: i32 = 19;
pub const EVENT_TRACE_TYPE_COPY_TCP: i32 = 18;
pub const EVENT_TRACE_TYPE_DBGID_RSDS: i32 = 64;
pub const EVENT_TRACE_TYPE_DC_END: i32 = 4;
pub const EVENT_TRACE_TYPE_DC_START: i32 = 3;
pub const EVENT_TRACE_TYPE_DEQUEUE: i32 = 7;
pub const EVENT_TRACE_TYPE_DISCONNECT: i32 = 13;
pub const EVENT_TRACE_TYPE_END: i32 = 2;
pub const EVENT_TRACE_TYPE_EXTENSION: i32 = 5;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: i32 = 99;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: i32 = 101;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: i32 = 97;
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: i32 = 98;
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: i32 = 100;
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: i32 = 96;
pub const EVENT_TRACE_TYPE_GUIDMAP: i32 = 10;
pub const EVENT_TRACE_TYPE_INFO: i32 = 0;
pub const EVENT_TRACE_TYPE_IO_FLUSH: i32 = 14;
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: i32 = 15;
pub const EVENT_TRACE_TYPE_IO_READ: i32 = 10;
pub const EVENT_TRACE_TYPE_IO_READ_INIT: i32 = 12;
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: i32 = 16;
pub const EVENT_TRACE_TYPE_IO_WRITE: i32 = 11;
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: i32 = 13;
pub const EVENT_TRACE_TYPE_LOAD: i32 = 10;
pub const EVENT_TRACE_TYPE_MM_AV: i32 = 15;
pub const EVENT_TRACE_TYPE_MM_COW: i32 = 12;
pub const EVENT_TRACE_TYPE_MM_DZF: i32 = 11;
pub const EVENT_TRACE_TYPE_MM_GPF: i32 = 13;
pub const EVENT_TRACE_TYPE_MM_HPF: i32 = 14;
pub const EVENT_TRACE_TYPE_MM_TF: i32 = 10;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: i32 = 57;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: i32 = 60;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: i32 = 55;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: i32 = 58;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: i32 = 56;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: i32 = 59;
pub const EVENT_TRACE_TYPE_RECEIVE: i32 = 11;
pub const EVENT_TRACE_TYPE_RECONNECT: i32 = 16;
pub const EVENT_TRACE_TYPE_REGCLOSE: i32 = 27;
pub const EVENT_TRACE_TYPE_REGCOMMIT: i32 = 30;
pub const EVENT_TRACE_TYPE_REGCREATE: i32 = 10;
pub const EVENT_TRACE_TYPE_REGDELETE: i32 = 12;
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: i32 = 15;
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: i32 = 17;
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: i32 = 18;
pub const EVENT_TRACE_TYPE_REGFLUSH: i32 = 21;
pub const EVENT_TRACE_TYPE_REGKCBCREATE: i32 = 22;
pub const EVENT_TRACE_TYPE_REGKCBDELETE: i32 = 23;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: i32 = 24;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: i32 = 25;
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: i32 = 33;
pub const EVENT_TRACE_TYPE_REGOPEN: i32 = 11;
pub const EVENT_TRACE_TYPE_REGPREPARE: i32 = 31;
pub const EVENT_TRACE_TYPE_REGQUERY: i32 = 13;
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: i32 = 19;
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: i32 = 29;
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: i32 = 16;
pub const EVENT_TRACE_TYPE_REGROLLBACK: i32 = 32;
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: i32 = 20;
pub const EVENT_TRACE_TYPE_REGSETSECURITY: i32 = 28;
pub const EVENT_TRACE_TYPE_REGSETVALUE: i32 = 14;
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: i32 = 26;
pub const EVENT_TRACE_TYPE_REPLY: i32 = 6;
pub const EVENT_TRACE_TYPE_RESUME: i32 = 7;
pub const EVENT_TRACE_TYPE_RETRANSMIT: i32 = 14;
pub const EVENT_TRACE_TYPE_SECURITY: i32 = 13;
pub const EVENT_TRACE_TYPE_SEND: i32 = 10;
pub const EVENT_TRACE_TYPE_SIDINFO: i32 = 12;
pub const EVENT_TRACE_TYPE_START: i32 = 1;
pub const EVENT_TRACE_TYPE_STOP: i32 = 2;
pub const EVENT_TRACE_TYPE_SUSPEND: i32 = 8;
pub const EVENT_TRACE_TYPE_TERMINATE: i32 = 11;
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: i32 = 240;
pub const EVENT_TRACE_TYPE_WINEVT_SEND: i32 = 9;
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: i32 = 16384;
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: i32 = 8192;
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: i32 = 32768;
pub const EVENT_TRACE_USE_NOCPUTIME: i32 = 2;
pub const EVENT_TRACE_USE_PAGED_MEMORY: i32 = 16777216;
pub const EVENT_TRACE_USE_PROCTIME: i32 = 1;
pub const EtwCompressionModeNoDisable: ETW_COMPRESSION_RESUMPTION_MODE = 1;
pub const EtwCompressionModeNoRestart: ETW_COMPRESSION_RESUMPTION_MODE = 2;
pub const EtwCompressionModeRestart: ETW_COMPRESSION_RESUMPTION_MODE = 0;
pub const EtwContextRegisterTypeControl: ETW_CONTEXT_REGISTER_TYPES = 1;
pub const EtwContextRegisterTypeInteger: ETW_CONTEXT_REGISTER_TYPES = 2;
pub const EtwContextRegisterTypeNone: ETW_CONTEXT_REGISTER_TYPES = 0;
pub const EtwPmcOwnerFree: ETW_PMC_COUNTER_OWNER_TYPE = 0;
pub const EtwPmcOwnerTagged: ETW_PMC_COUNTER_OWNER_TYPE = 2;
pub const EtwPmcOwnerTaggedWithSource: ETW_PMC_COUNTER_OWNER_TYPE = 3;
pub const EtwPmcOwnerUntagged: ETW_PMC_COUNTER_OWNER_TYPE = 1;
pub const EtwQueryLastDroppedTimes: ETW_PROCESS_HANDLE_INFO_TYPE = 3;
pub const EtwQueryLogFileHeader: ETW_PROCESS_HANDLE_INFO_TYPE = 4;
pub const EtwQueryPartitionInformation: ETW_PROCESS_HANDLE_INFO_TYPE = 1;
pub const EtwQueryPartitionInformationV2: ETW_PROCESS_HANDLE_INFO_TYPE = 2;
pub const EtwQueryProcessHandleInfoMax: ETW_PROCESS_HANDLE_INFO_TYPE = 5;
pub const EventTraceConfigGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x01853a65_418f_4f36_aefc_dc0f1d2fd235);
pub const EventTraceGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x68fdd900_4a3e_11d1_84f4_0000f80464e3);
pub const GLOBAL_LOGGER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("GlobalLogger");
pub const GLOBAL_LOGGER_NAMEA: windows_sys::core::PCSTR = windows_sys::core::s!("GlobalLogger");
pub const GLOBAL_LOGGER_NAMEW: windows_sys::core::PCWSTR = windows_sys::core::w!("GlobalLogger");
#[cfg(target_arch = "x86")]
pub const INVALID_PROCESSTRACE_HANDLE: PROCESSTRACE_HANDLE = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INVALID_PROCESSTRACE_HANDLE: PROCESSTRACE_HANDLE = 18446744073709551615;
pub const KERNEL_LOGGER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("NT Kernel Logger");
pub const KERNEL_LOGGER_NAMEA: windows_sys::core::PCSTR = windows_sys::core::s!("NT Kernel Logger");
pub const KERNEL_LOGGER_NAMEW: windows_sys::core::PCWSTR = windows_sys::core::w!("NT Kernel Logger");
pub const LastBranchRecordProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x99134383_5248_43fc_834b_529454e75df3);
pub const MAX_MOF_FIELDS: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MOF_FIELD {
    pub DataPtr: super::ULONG64,
    pub Length: super::ULONG,
    pub DataType: super::ULONG,
}
pub const MaxTraceSetInfoClass: _TRACE_QUERY_INFO_CLASS = 29;
#[cfg(feature = "shared")]
pub type PCLASSIC_EVENT_ID = *mut CLASSIC_EVENT_ID;
#[cfg(all(feature = "evntprov", feature = "shared"))]
pub type PENABLE_TRACE_PARAMETERS = *mut ENABLE_TRACE_PARAMETERS;
#[cfg(all(feature = "evntprov", feature = "shared"))]
pub type PENABLE_TRACE_PARAMETERS_V1 = *mut ENABLE_TRACE_PARAMETERS_V1;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PETW_BUFFER_CALLBACK = Option<unsafe extern "system" fn(buffer: *const ETW_BUFFER_HEADER, buffersize: super::ULONG, consumerinfo: *const ETW_BUFFER_CALLBACK_INFORMATION, callbackcontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PETW_BUFFER_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(buffer: *const ETW_BUFFER_HEADER, callbackcontext: *const core::ffi::c_void)>;
#[cfg(feature = "shared")]
pub type PETW_BUFFER_CONTEXT = *mut ETW_BUFFER_CONTEXT;
#[cfg(feature = "shared")]
pub type PETW_PMC_COUNTER_OWNER = *mut ETW_PMC_COUNTER_OWNER;
#[cfg(feature = "shared")]
pub type PETW_PMC_COUNTER_OWNERSHIP_STATUS = *mut ETW_PMC_COUNTER_OWNERSHIP_STATUS;
#[cfg(feature = "shared")]
pub type PETW_TRACE_PARTITION_INFORMATION = *mut ETW_TRACE_PARTITION_INFORMATION;
#[cfg(feature = "shared")]
pub type PETW_TRACE_PARTITION_INFORMATION_V2 = *mut ETW_TRACE_PARTITION_INFORMATION_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEVENT_CALLBACK = Option<unsafe extern "system" fn(pevent: PEVENT_TRACE)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEVENT_INSTANCE_HEADER = *mut EVENT_INSTANCE_HEADER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEVENT_INSTANCE_INFO = *mut EVENT_INSTANCE_INFO;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "shared", feature = "winnt"))]
pub type PEVENT_RECORD_CALLBACK = Option<unsafe extern "system" fn(eventrecord: super::PEVENT_RECORD)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEVENT_TRACE = *mut EVENT_TRACE;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = Option<unsafe extern "system" fn(logfile: PEVENT_TRACE_LOGFILEA) -> super::ULONG>;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = Option<unsafe extern "system" fn(logfile: PEVENT_TRACE_LOGFILEW) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEVENT_TRACE_HEADER = *mut EVENT_TRACE_HEADER;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PEVENT_TRACE_LOGFILEA = *mut _EVENT_TRACE_LOGFILEA;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PEVENT_TRACE_LOGFILEW = *mut _EVENT_TRACE_LOGFILEW;
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
pub type PEVENT_TRACE_PROPERTIES = *mut EVENT_TRACE_PROPERTIES;
#[cfg(all(feature = "evntprov", feature = "shared", feature = "winnt", feature = "wmistr"))]
pub type PEVENT_TRACE_PROPERTIES_V2 = *mut EVENT_TRACE_PROPERTIES_V2;
#[cfg(feature = "shared")]
pub type PMOF_FIELD = *mut MOF_FIELD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROFILE_SOURCE_INFO = *mut PROFILE_SOURCE_INFO;
pub type PROCESSTRACE_HANDLE = u64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROFILE_SOURCE_INFO {
    pub NextEntryOffset: super::ULONG,
    pub Source: super::ULONG,
    pub MinInterval: super::ULONG,
    pub MaxInterval: super::ULONG,
    pub Reserved: super::ULONG64,
    pub Description: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROFILE_SOURCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTRACEHANDLE = *mut u64;
#[cfg(feature = "shared")]
pub type PTRACE_ENABLE_INFO = *mut TRACE_ENABLE_INFO;
#[cfg(feature = "shared")]
pub type PTRACE_GUID_INFO = *mut TRACE_GUID_INFO;
#[cfg(feature = "shared")]
pub type PTRACE_GUID_PROPERTIES = *mut TRACE_GUID_PROPERTIES;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type PTRACE_GUID_REGISTRATION = *mut TRACE_GUID_REGISTRATION;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PTRACE_LOGFILE_HEADER = *mut TRACE_LOGFILE_HEADER;
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PTRACE_LOGFILE_HEADER32 = *mut TRACE_LOGFILE_HEADER32;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
pub type PTRACE_LOGFILE_HEADER64 = *mut TRACE_LOGFILE_HEADER64;
#[cfg(feature = "shared")]
pub type PTRACE_PERIODIC_CAPTURE_STATE_INFO = *mut TRACE_PERIODIC_CAPTURE_STATE_INFO;
#[cfg(feature = "shared")]
pub type PTRACE_PROFILE_INTERVAL = *mut TRACE_PROFILE_INTERVAL;
#[cfg(feature = "shared")]
pub type PTRACE_PROVIDER_INSTANCE_INFO = *mut TRACE_PROVIDER_INSTANCE_INFO;
#[cfg(feature = "shared")]
pub type PTRACE_STACK_CACHING_INFO = *mut TRACE_STACK_CACHING_INFO;
#[cfg(feature = "shared")]
pub type PTRACE_VERSION_INFO = *mut TRACE_VERSION_INFO;
pub const PrivateLoggerNotificationGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3595ab5c_042a_4c8e_b942_2d059bfeb1b1);
pub const SYSTEM_ALPC_KW_GENERAL: i32 = 1;
pub const SYSTEM_CONFIG_KW_GRAPHICS: i32 = 2;
pub const SYSTEM_CONFIG_KW_NETWORK: i32 = 8;
pub const SYSTEM_CONFIG_KW_OPTICAL: i32 = 64;
pub const SYSTEM_CONFIG_KW_PNP: i32 = 32;
pub const SYSTEM_CONFIG_KW_SERVICES: i32 = 16;
pub const SYSTEM_CONFIG_KW_STORAGE: i32 = 4;
pub const SYSTEM_CONFIG_KW_SYSTEM: i32 = 1;
pub const SYSTEM_CPU_KW_CACHE_FLUSH: i32 = 2;
pub const SYSTEM_CPU_KW_CONFIG: i32 = 1;
pub const SYSTEM_CPU_KW_DOMAIN_CHANGE: i32 = 8;
pub const SYSTEM_CPU_KW_SPEC_CONTROL: i32 = 4;
pub const SYSTEM_EVENT_TYPE: i32 = 1;
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: i32 = 2;
pub const SYSTEM_HYPERVISOR_KW_PROFILE: i32 = 1;
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: i32 = 4;
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: i32 = 2;
pub const SYSTEM_INTERRUPT_KW_DPC: i32 = 4;
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: i32 = 8;
pub const SYSTEM_INTERRUPT_KW_GENERAL: i32 = 1;
pub const SYSTEM_INTERRUPT_KW_IPI: i32 = 64;
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: i32 = 16;
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: i32 = 32;
pub const SYSTEM_IOFILTER_KW_FAILURE: i32 = 8;
pub const SYSTEM_IOFILTER_KW_FASTIO: i32 = 4;
pub const SYSTEM_IOFILTER_KW_GENERAL: i32 = 1;
pub const SYSTEM_IOFILTER_KW_INIT: i32 = 2;
pub const SYSTEM_IO_KW_CC: i32 = 256;
pub const SYSTEM_IO_KW_DISK: i32 = 1;
pub const SYSTEM_IO_KW_DISK_INIT: i32 = 2;
pub const SYSTEM_IO_KW_DRIVERS: i32 = 128;
pub const SYSTEM_IO_KW_FILE: i32 = 16;
pub const SYSTEM_IO_KW_FILENAME: i32 = 4;
pub const SYSTEM_IO_KW_FILE_INIT: i32 = 1024;
pub const SYSTEM_IO_KW_NETWORK: i32 = 512;
pub const SYSTEM_IO_KW_OPTICAL: i32 = 32;
pub const SYSTEM_IO_KW_OPTICAL_INIT: i32 = 64;
pub const SYSTEM_IO_KW_SPLIT: i32 = 8;
pub const SYSTEM_IO_KW_TIMER: i32 = 2048;
pub const SYSTEM_LOCK_KW_SPINLOCK: i32 = 1;
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: i32 = 2;
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: i32 = 4;
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: i32 = 4;
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: i32 = 512;
pub const SYSTEM_MEMORY_KW_FOOTPRINT: i32 = 2048;
pub const SYSTEM_MEMORY_KW_GENERAL: i32 = 1;
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: i32 = 2;
pub const SYSTEM_MEMORY_KW_HEAP: i32 = 128;
pub const SYSTEM_MEMORY_KW_MEMINFO: i32 = 16;
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: i32 = 64;
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: i32 = 32768;
pub const SYSTEM_MEMORY_KW_PFSECTION: i32 = 32;
pub const SYSTEM_MEMORY_KW_POOL: i32 = 8;
pub const SYSTEM_MEMORY_KW_REFSET: i32 = 8192;
pub const SYSTEM_MEMORY_KW_SESSION: i32 = 4096;
pub const SYSTEM_MEMORY_KW_VAMAP: i32 = 16384;
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: i32 = 1024;
pub const SYSTEM_MEMORY_KW_WS: i32 = 256;
pub const SYSTEM_MEMORY_POOL_FILTER_ID: i32 = 1;
pub const SYSTEM_OBJECT_KW_GENERAL: i32 = 1;
pub const SYSTEM_OBJECT_KW_HANDLE: i32 = 2;
pub const SYSTEM_POWER_KW_GENERAL: i32 = 1;
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: i32 = 2;
pub const SYSTEM_POWER_KW_IDLE_SELECTION: i32 = 8;
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: i32 = 16;
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: i32 = 4;
pub const SYSTEM_PROCESS_KW_DBGPRINT: i32 = 256;
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: i32 = 128;
pub const SYSTEM_PROCESS_KW_FREEZE: i32 = 4;
pub const SYSTEM_PROCESS_KW_GENERAL: i32 = 1;
pub const SYSTEM_PROCESS_KW_INSWAP: i32 = 2;
pub const SYSTEM_PROCESS_KW_JOB: i32 = 512;
pub const SYSTEM_PROCESS_KW_LOADER: i32 = 4096;
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: i32 = 8;
pub const SYSTEM_PROCESS_KW_THREAD: i32 = 2048;
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: i32 = 16;
pub const SYSTEM_PROCESS_KW_WAKE_DROP: i32 = 32;
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: i32 = 64;
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: i32 = 1024;
pub const SYSTEM_PROFILE_KW_GENERAL: i32 = 1;
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: i32 = 2;
pub const SYSTEM_REGISTRY_KW_GENERAL: i32 = 1;
pub const SYSTEM_REGISTRY_KW_HIVE: i32 = 2;
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: i32 = 4;
pub const SYSTEM_SCHEDULER_KW_AFFINITY: i32 = 64;
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: i32 = 16;
pub const SYSTEM_SCHEDULER_KW_AUTOBOOST: i32 = 65536;
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: i32 = 1024;
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: i32 = 512;
pub const SYSTEM_SCHEDULER_KW_CPU_PARTITION: i32 = 8192;
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: i32 = 2;
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: i32 = 256;
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: i32 = 4;
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: i32 = 32;
pub const SYSTEM_SCHEDULER_KW_PRIORITY: i32 = 128;
pub const SYSTEM_SCHEDULER_KW_READY_QUEUE: i32 = 4096;
pub const SYSTEM_SCHEDULER_KW_SCHEDULE_THREAD: i32 = 2048;
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: i32 = 8;
pub const SYSTEM_SCHEDULER_KW_THREAD_FEEDBACK_READ: i32 = 16384;
pub const SYSTEM_SCHEDULER_KW_WORKLOAD_CLASS_UPDATE: i32 = 32768;
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: i32 = 1;
pub const SYSTEM_SYSCALL_KW_GENERAL: i32 = 1;
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: i32 = 2;
pub const SYSTEM_TIMER_KW_GENERAL: i32 = 1;
pub const SystemAlpcProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfcb9baaf_e529_4980_92e9_ced1a6aadfdf);
pub const SystemConfigProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfef3a8b6_318d_4b67_a96a_3b0f6b8f18fe);
pub const SystemCpuProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6c5265f_eae8_4650_aae4_9d48603d8510);
pub const SystemHypervisorProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbafa072a_918a_4bed_b622_bc152097098f);
pub const SystemInterruptProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd4bbee17_b545_4888_858b_744169015b25);
pub const SystemIoFilterProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfbd09363_9e22_4661_b8bf_e7a34b535b8c);
pub const SystemIoProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3d5c43e3_0f1c_4202_b817_174c0070dc79);
pub const SystemLockProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x721ddfd3_dacc_4e1e_b26a_a2cb31d4705a);
pub const SystemMemoryProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82958ca9_b6cd_47f8_a3a8_03ae85a4bc24);
pub const SystemObjectProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfebd7460_3d1d_47eb_af49_c9eeb1e146f2);
pub const SystemPowerProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc134884a_32d5_4488_80e5_14ed7abb8269);
pub const SystemProcessProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x151f55dc_467d_471f_83b5_5f889d46ff66);
pub const SystemProfileProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbfeb0324_1cee_496f_a409_2ac2b48a6322);
pub const SystemRegistryProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x16156bd9_fab4_4cfa_a232_89d1099058e3);
pub const SystemSchedulerProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x599a2a76_4d91_4910_9ac7_7d33f2e97a6c);
pub const SystemSyscallProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x434286f7_6f1b_45bb_b37e_95f623046c7c);
pub const SystemTimerProviderGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f061568_e215_499f_ab2e_eda0ae890a5b);
pub const SystemTraceControlGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9e814aad_3204_11d2_9a82_006008a86939);
pub type TRACEGUID_HANDLE = u64;
pub type TRACEHANDLE = u64;
pub type TRACELOGGER_HANDLE = u64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_CONTEXT_REGISTER_INFO {
    pub RegisterTypes: ETW_CONTEXT_REGISTER_TYPES,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_ENABLE_INFO {
    pub IsEnabled: super::ULONG,
    pub Level: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub LoggerId: super::USHORT,
    pub EnableProperty: super::ULONG,
    pub Reserved2: super::ULONG,
    pub MatchAnyKeyword: super::ULONGLONG,
    pub MatchAllKeyword: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: windows_sys::core::GUID,
    pub GuidType: super::ULONG,
    pub LoggerId: super::ULONG,
    pub EnableLevel: super::ULONG,
    pub EnableFlags: super::ULONG,
    pub IsEnable: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: super::LPCGUID,
    pub RegHandle: super::HANDLE,
}
pub const TRACE_HEADER_FLAG_LOG_WNODE: i32 = 262144;
pub const TRACE_HEADER_FLAG_TRACED_GUID: i32 = 131072;
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: i32 = 524288;
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: i32 = 1048576;
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: i32 = 512;
pub type TRACE_INFO_CLASS = _TRACE_QUERY_INFO_CLASS;
pub type TRACE_LBR_CONFIGURATION = i32;
pub const TRACE_LBR_CONFIGURATION_CALLSTACK_ENABLE: TRACE_LBR_CONFIGURATION = 512;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_FAR_BRANCH: TRACE_LBR_CONFIGURATION = 256;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_JCC: TRACE_LBR_CONFIGURATION = 4;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_KERNEL: TRACE_LBR_CONFIGURATION = 1;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_NEAR_IND_CALL: TRACE_LBR_CONFIGURATION = 16;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_NEAR_IND_JMP: TRACE_LBR_CONFIGURATION = 64;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_NEAR_REL_CALL: TRACE_LBR_CONFIGURATION = 8;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_NEAR_REL_JMP: TRACE_LBR_CONFIGURATION = 128;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_NEAR_RET: TRACE_LBR_CONFIGURATION = 32;
pub const TRACE_LBR_CONFIGURATION_EXCLUDE_USER: TRACE_LBR_CONFIGURATION = 2;
pub const TRACE_LBR_CONFIGURATION_NONE: TRACE_LBR_CONFIGURATION = 0;
pub const TRACE_LBR_CONFIGURATION_SAMPLED: TRACE_LBR_CONFIGURATION = 1024;
pub const TRACE_LBR_EVENT_OPCODE: i32 = 32;
pub const TRACE_LBR_MAXIMUM_EVENTS: i32 = 4;
pub const TRACE_LEVEL_CRITICAL: i32 = 1;
pub const TRACE_LEVEL_ERROR: i32 = 2;
pub const TRACE_LEVEL_FATAL: i32 = 1;
pub const TRACE_LEVEL_INFORMATION: i32 = 4;
pub const TRACE_LEVEL_NONE: i32 = 0;
pub const TRACE_LEVEL_RESERVED6: i32 = 6;
pub const TRACE_LEVEL_RESERVED7: i32 = 7;
pub const TRACE_LEVEL_RESERVED8: i32 = 8;
pub const TRACE_LEVEL_RESERVED9: i32 = 9;
pub const TRACE_LEVEL_VERBOSE: i32 = 5;
pub const TRACE_LEVEL_WARNING: i32 = 3;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER {
    pub BufferSize: super::ULONG,
    pub Anonymous: TRACE_LOGFILE_HEADER_0,
    pub ProviderVersion: super::ULONG,
    pub NumberOfProcessors: super::ULONG,
    pub EndTime: super::LARGE_INTEGER,
    pub TimerResolution: super::ULONG,
    pub MaximumFileSize: super::ULONG,
    pub LogFileMode: super::ULONG,
    pub BuffersWritten: super::ULONG,
    pub Anonymous2: TRACE_LOGFILE_HEADER_1,
    pub LoggerName: windows_sys::core::PWSTR,
    pub LogFileName: windows_sys::core::PWSTR,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BootTime: super::LARGE_INTEGER,
    pub PerfFreq: super::LARGE_INTEGER,
    pub StartTime: super::LARGE_INTEGER,
    pub ReservedFlags: super::ULONG,
    pub BuffersLost: super::ULONG,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: super::ULONG,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: super::UCHAR,
    pub MinorVersion: super::UCHAR,
    pub SubVersion: super::UCHAR,
    pub SubMinorVersion: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: windows_sys::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: super::ULONG,
    pub PointerSize: super::ULONG,
    pub EventsLost: super::ULONG,
    pub CpuSpeedInMHz: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER32 {
    pub BufferSize: super::ULONG,
    pub Anonymous: TRACE_LOGFILE_HEADER32_0,
    pub ProviderVersion: super::ULONG,
    pub NumberOfProcessors: super::ULONG,
    pub EndTime: super::LARGE_INTEGER,
    pub TimerResolution: super::ULONG,
    pub MaximumFileSize: super::ULONG,
    pub LogFileMode: super::ULONG,
    pub BuffersWritten: super::ULONG,
    pub Anonymous2: TRACE_LOGFILE_HEADER32_1,
    pub LoggerName: super::ULONG32,
    pub LogFileName: super::ULONG32,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BootTime: super::LARGE_INTEGER,
    pub PerfFreq: super::LARGE_INTEGER,
    pub StartTime: super::LARGE_INTEGER,
    pub ReservedFlags: super::ULONG,
    pub BuffersLost: super::ULONG,
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: super::ULONG,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: super::UCHAR,
    pub MinorVersion: super::UCHAR,
    pub SubVersion: super::UCHAR,
    pub SubMinorVersion: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: windows_sys::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER32_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: super::ULONG,
    pub PointerSize: super::ULONG,
    pub EventsLost: super::ULONG,
    pub CpuSpeedInMHz: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER64 {
    pub BufferSize: super::ULONG,
    pub Anonymous: TRACE_LOGFILE_HEADER64_0,
    pub ProviderVersion: super::ULONG,
    pub NumberOfProcessors: super::ULONG,
    pub EndTime: super::LARGE_INTEGER,
    pub TimerResolution: super::ULONG,
    pub MaximumFileSize: super::ULONG,
    pub LogFileMode: super::ULONG,
    pub BuffersWritten: super::ULONG,
    pub Anonymous2: TRACE_LOGFILE_HEADER64_1,
    pub LoggerName: super::ULONG64,
    pub LogFileName: super::ULONG64,
    pub TimeZone: super::TIME_ZONE_INFORMATION,
    pub BootTime: super::LARGE_INTEGER,
    pub PerfFreq: super::LARGE_INTEGER,
    pub StartTime: super::LARGE_INTEGER,
    pub ReservedFlags: super::ULONG,
    pub BuffersLost: super::ULONG,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: super::ULONG,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: super::UCHAR,
    pub MinorVersion: super::UCHAR,
    pub SubVersion: super::UCHAR,
    pub SubMinorVersion: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: windows_sys::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for TRACE_LOGFILE_HEADER64_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: super::ULONG,
    pub PointerSize: super::ULONG,
    pub EventsLost: super::ULONG,
    pub CpuSpeedInMHz: super::ULONG,
}
pub const TRACE_MESSAGE_COMPONENTID: i32 = 4;
pub const TRACE_MESSAGE_FLAG_MASK: i32 = 65535;
pub const TRACE_MESSAGE_GUID: i32 = 2;
pub const TRACE_MESSAGE_MAXIMUM_SIZE: i32 = 65536;
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: i32 = 16;
pub const TRACE_MESSAGE_POINTER32: i32 = 64;
pub const TRACE_MESSAGE_POINTER64: i32 = 128;
pub const TRACE_MESSAGE_SEQUENCE: i32 = 1;
pub const TRACE_MESSAGE_SYSTEMINFO: i32 = 32;
pub const TRACE_MESSAGE_TIMESTAMP: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: super::ULONG,
    pub ProviderCount: super::USHORT,
    pub Reserved: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: super::ULONG,
    pub Interval: super::ULONG,
}
pub const TRACE_PROVIDER_FLAG_LEGACY: i32 = 1;
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: super::ULONG,
    pub EnableCount: super::ULONG,
    pub Pid: super::ULONG,
    pub Flags: super::ULONG,
}
pub type TRACE_QUERY_INFO_CLASS = _TRACE_QUERY_INFO_CLASS;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: super::BOOLEAN,
    pub CacheSize: super::ULONG,
    pub BucketCount: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: super::UINT,
    pub Reserved: super::UINT,
}
pub const TraceContextRegisterInfo: _TRACE_QUERY_INFO_CLASS = 28;
pub const TraceDisallowListQuery: _TRACE_QUERY_INFO_CLASS = 14;
pub const TraceGroupQueryInfo: _TRACE_QUERY_INFO_CLASS = 13;
pub const TraceGroupQueryList: _TRACE_QUERY_INFO_CLASS = 12;
pub const TraceGuidQueryInfo: _TRACE_QUERY_INFO_CLASS = 1;
pub const TraceGuidQueryList: _TRACE_QUERY_INFO_CLASS = 0;
pub const TraceGuidQueryProcess: _TRACE_QUERY_INFO_CLASS = 2;
pub const TraceInfoReserved15: _TRACE_QUERY_INFO_CLASS = 15;
pub const TraceLbrConfigurationInfo: _TRACE_QUERY_INFO_CLASS = 20;
pub const TraceLbrEventListInfo: _TRACE_QUERY_INFO_CLASS = 21;
pub const TraceMaxLoggersQuery: _TRACE_QUERY_INFO_CLASS = 19;
pub const TraceMaxPmcCounterQuery: _TRACE_QUERY_INFO_CLASS = 22;
pub const TracePeriodicCaptureStateInfo: _TRACE_QUERY_INFO_CLASS = 17;
pub const TracePeriodicCaptureStateListInfo: _TRACE_QUERY_INFO_CLASS = 16;
pub const TracePmcCounterListInfo: _TRACE_QUERY_INFO_CLASS = 9;
pub const TracePmcCounterOwners: _TRACE_QUERY_INFO_CLASS = 25;
pub const TracePmcEventListInfo: _TRACE_QUERY_INFO_CLASS = 8;
pub const TracePmcSessionInformation: _TRACE_QUERY_INFO_CLASS = 27;
pub const TraceProfileSourceConfigInfo: _TRACE_QUERY_INFO_CLASS = 6;
pub const TraceProfileSourceListInfo: _TRACE_QUERY_INFO_CLASS = 7;
pub const TraceProviderBinaryTracking: _TRACE_QUERY_INFO_CLASS = 18;
pub const TraceSampledProfileIntervalInfo: _TRACE_QUERY_INFO_CLASS = 5;
pub const TraceSetDisallowList: _TRACE_QUERY_INFO_CLASS = 10;
pub const TraceStackCachingInfo: _TRACE_QUERY_INFO_CLASS = 24;
pub const TraceStackTracingInfo: _TRACE_QUERY_INFO_CLASS = 3;
pub const TraceStreamCount: _TRACE_QUERY_INFO_CLASS = 23;
pub const TraceSystemTraceEnableFlagsInfo: _TRACE_QUERY_INFO_CLASS = 4;
pub const TraceUnifiedStackCachingInfo: _TRACE_QUERY_INFO_CLASS = 26;
pub const TraceVersionInfo: _TRACE_QUERY_INFO_CLASS = 11;
#[cfg(all(feature = "shared", feature = "winnt", feature = "wmistr"))]
pub type WMIDPREQUEST = Option<unsafe extern "system" fn(requestcode: super::WMIDPREQUESTCODE, requestcontext: super::PVOID, buffersize: *mut super::ULONG, buffer: super::PVOID) -> super::ULONG>;
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct _EVENT_TRACE_LOGFILEA {
    pub LogFileName: windows_sys::core::PSTR,
    pub LoggerName: windows_sys::core::PSTR,
    pub CurrentTime: super::LONGLONG,
    pub BuffersRead: super::ULONG,
    pub Anonymous: _EVENT_TRACE_LOGFILEA_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKA,
    pub BufferSize: super::ULONG,
    pub Filled: super::ULONG,
    pub EventsLost: super::ULONG,
    pub Anonymous2: _EVENT_TRACE_LOGFILEA_1,
    pub IsKernelTrace: super::ULONG,
    pub Context: super::PVOID,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for _EVENT_TRACE_LOGFILEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union _EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: super::ULONG,
    pub ProcessTraceMode: super::ULONG,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for _EVENT_TRACE_LOGFILEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union _EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for _EVENT_TRACE_LOGFILEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct _EVENT_TRACE_LOGFILEW {
    pub LogFileName: windows_sys::core::PWSTR,
    pub LoggerName: windows_sys::core::PWSTR,
    pub CurrentTime: super::LONGLONG,
    pub BuffersRead: super::ULONG,
    pub Anonymous: _EVENT_TRACE_LOGFILEW_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKW,
    pub BufferSize: super::ULONG,
    pub Filled: super::ULONG,
    pub EventsLost: super::ULONG,
    pub Anonymous2: _EVENT_TRACE_LOGFILEW_1,
    pub IsKernelTrace: super::ULONG,
    pub Context: super::PVOID,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for _EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union _EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: super::ULONG,
    pub ProcessTraceMode: super::ULONG,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for _EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union _EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "shared", feature = "timezoneapi", feature = "winnt"))]
impl Default for _EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type _TRACE_QUERY_INFO_CLASS = i32;
pub const _typedef_TRACELOGGER_HANDLE: i32 = 1;
