#[inline]
pub unsafe fn CloseTrace(tracehandle: PROCESSTRACE_HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn CloseTrace(tracehandle : PROCESSTRACE_HANDLE) -> u32);
    unsafe { CloseTrace(tracehandle) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn ControlTraceA<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ControlTraceA(traceid : CONTROLTRACE_ID, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES, controlcode : u32) -> u32);
    unsafe { ControlTraceA(traceid, instancename.param().abi(), properties as _, controlcode) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn ControlTraceW<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ControlTraceW(traceid : CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES, controlcode : u32) -> u32);
    unsafe { ControlTraceW(traceid, instancename.param().abi(), properties as _, controlcode) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateTraceInstanceId(reghandle: super::winnt::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn CreateTraceInstanceId(reghandle : super::winnt::HANDLE, instinfo : *mut EVENT_INSTANCE_INFO) -> u32);
    unsafe { CreateTraceInstanceId(reghandle, instinfo as _) }
}
#[inline]
pub unsafe fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const windows_core::GUID, traceid: CONTROLTRACE_ID) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnableTrace(enable : u32, enableflag : u32, enablelevel : u32, controlguid : *const windows_core::GUID, traceid : CONTROLTRACE_ID) -> u32);
    unsafe { EnableTrace(enable, enableflag, enablelevel, controlguid, traceid) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn EnableTraceEx(providerid: *const windows_core::GUID, sourceid: Option<*const windows_core::GUID>, traceid: CONTROLTRACE_ID, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: Option<*const super::evntprov::EVENT_FILTER_DESCRIPTOR>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnableTraceEx(providerid : *const windows_core::GUID, sourceid : *const windows_core::GUID, traceid : CONTROLTRACE_ID, isenabled : u32, level : u8, matchanykeyword : u64, matchallkeyword : u64, enableproperty : u32, enablefilterdesc : *const super::evntprov::EVENT_FILTER_DESCRIPTOR) -> u32);
    unsafe { EnableTraceEx(providerid, sourceid.unwrap_or(core::mem::zeroed()) as _, traceid, isenabled, level, matchanykeyword, matchallkeyword, enableproperty, enablefilterdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn EnableTraceEx2(traceid: CONTROLTRACE_ID, providerid: *const windows_core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: Option<*const ENABLE_TRACE_PARAMETERS>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnableTraceEx2(traceid : CONTROLTRACE_ID, providerid : *const windows_core::GUID, controlcode : u32, level : u8, matchanykeyword : u64, matchallkeyword : u64, timeout : u32, enableparameters : *const ENABLE_TRACE_PARAMETERS) -> u32);
    unsafe { EnableTraceEx2(traceid, providerid, controlcode, level, matchanykeyword, matchallkeyword, timeout, enableparameters.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumerateTraceGuids(guidpropertiesarray: &mut [PTRACE_GUID_PROPERTIES], guidcount: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnumerateTraceGuids(guidpropertiesarray : *mut PTRACE_GUID_PROPERTIES, propertyarraycount : u32, guidcount : *mut u32) -> u32);
    unsafe { EnumerateTraceGuids(guidpropertiesarray.as_mut_ptr(), guidpropertiesarray.len().try_into().unwrap(), guidcount as _) }
}
#[inline]
pub unsafe fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: Option<*const core::ffi::c_void>, inbuffersize: u32, outbuffer: Option<*mut core::ffi::c_void>, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnumerateTraceGuidsEx(tracequeryinfoclass : TRACE_QUERY_INFO_CLASS, inbuffer : *const core::ffi::c_void, inbuffersize : u32, outbuffer : *mut core::ffi::c_void, outbuffersize : u32, returnlength : *mut u32) -> u32);
    unsafe { EnumerateTraceGuidsEx(tracequeryinfoclass, inbuffer.unwrap_or(core::mem::zeroed()) as _, inbuffersize, outbuffer.unwrap_or(core::mem::zeroed()) as _, outbuffersize, returnlength as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn FlushTraceA<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn FlushTraceA(traceid : CONTROLTRACE_ID, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { FlushTraceA(traceid, instancename.param().abi(), properties as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn FlushTraceW<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn FlushTraceW(traceid : CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { FlushTraceW(traceid, instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn GetTraceEnableFlags(tracehandle: TRACELOGGER_HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetTraceEnableFlags(tracehandle : TRACELOGGER_HANDLE) -> u32);
    unsafe { GetTraceEnableFlags(tracehandle) }
}
#[inline]
pub unsafe fn GetTraceEnableLevel(tracehandle: TRACELOGGER_HANDLE) -> u8 {
    windows_core::link!("advapi32.dll" "system" fn GetTraceEnableLevel(tracehandle : TRACELOGGER_HANDLE) -> u8);
    unsafe { GetTraceEnableLevel(tracehandle) }
}
#[inline]
pub unsafe fn GetTraceLoggerHandle(buffer: *const core::ffi::c_void) -> TRACELOGGER_HANDLE {
    windows_core::link!("advapi32.dll" "system" fn GetTraceLoggerHandle(buffer : *const core::ffi::c_void) -> TRACELOGGER_HANDLE);
    unsafe { GetTraceLoggerHandle(buffer) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[inline]
pub unsafe fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> PROCESSTRACE_HANDLE {
    windows_core::link!("advapi32.dll" "system" fn OpenTraceA(logfile : *mut EVENT_TRACE_LOGFILEA) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceA(logfile as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[inline]
pub unsafe fn OpenTraceFromBufferStream(options: *const ETW_OPEN_TRACE_OPTIONS, buffercompletioncallback: PETW_BUFFER_COMPLETION_CALLBACK, buffercompletioncontext: Option<*const core::ffi::c_void>) -> PROCESSTRACE_HANDLE {
    windows_core::link!("advapi32.dll" "system" fn OpenTraceFromBufferStream(options : *const ETW_OPEN_TRACE_OPTIONS, buffercompletioncallback : PETW_BUFFER_COMPLETION_CALLBACK, buffercompletioncontext : *const core::ffi::c_void) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceFromBufferStream(options, buffercompletioncallback, buffercompletioncontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[inline]
pub unsafe fn OpenTraceFromFile<P0>(logfilename: P0, options: *const ETW_OPEN_TRACE_OPTIONS, logfileheader: Option<*mut TRACE_LOGFILE_HEADER>) -> PROCESSTRACE_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenTraceFromFile(logfilename : windows_core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, logfileheader : *mut TRACE_LOGFILE_HEADER) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceFromFile(logfilename.param().abi(), options, logfileheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[inline]
pub unsafe fn OpenTraceFromRealTimeLogger<P0>(loggername: P0, options: *const ETW_OPEN_TRACE_OPTIONS, logfileheader: Option<*mut TRACE_LOGFILE_HEADER>) -> PROCESSTRACE_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenTraceFromRealTimeLogger(loggername : windows_core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, logfileheader : *mut TRACE_LOGFILE_HEADER) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceFromRealTimeLogger(loggername.param().abi(), options, logfileheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi", feature = "winnt"))]
#[inline]
pub unsafe fn OpenTraceFromRealTimeLoggerWithAllocationOptions<P0>(loggername: P0, options: *const ETW_OPEN_TRACE_OPTIONS, allocationsize: usize, memorypartitionhandle: Option<super::winnt::HANDLE>, logfileheader: Option<*mut TRACE_LOGFILE_HEADER>) -> PROCESSTRACE_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenTraceFromRealTimeLoggerWithAllocationOptions(loggername : windows_core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, allocationsize : usize, memorypartitionhandle : super::winnt::HANDLE, logfileheader : *mut TRACE_LOGFILE_HEADER) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceFromRealTimeLoggerWithAllocationOptions(loggername.param().abi(), options, allocationsize, memorypartitionhandle.unwrap_or(core::mem::zeroed()) as _, logfileheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[inline]
pub unsafe fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE {
    windows_core::link!("advapi32.dll" "system" fn OpenTraceW(logfile : *mut EVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceW(logfile as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ProcessTrace(handlearray: &[PROCESSTRACE_HANDLE], starttime: Option<*const super::minwindef::FILETIME>, endtime: Option<*const super::minwindef::FILETIME>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn ProcessTrace(handlearray : *const PROCESSTRACE_HANDLE, handlecount : u32, starttime : *const super::minwindef::FILETIME, endtime : *const super::minwindef::FILETIME) -> u32);
    unsafe { ProcessTrace(handlearray.as_ptr(), handlearray.len().try_into().unwrap(), starttime.unwrap_or(core::mem::zeroed()) as _, endtime.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ProcessTraceAddBufferToBufferStream(tracehandle: PROCESSTRACE_HANDLE, buffer: *const ETW_BUFFER_HEADER, buffersize: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn ProcessTraceAddBufferToBufferStream(tracehandle : PROCESSTRACE_HANDLE, buffer : *const ETW_BUFFER_HEADER, buffersize : u32) -> u32);
    unsafe { ProcessTraceAddBufferToBufferStream(tracehandle, buffer, buffersize) }
}
#[inline]
pub unsafe fn ProcessTraceBufferDecrementReference(buffer: *const ETW_BUFFER_HEADER) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn ProcessTraceBufferDecrementReference(buffer : *const ETW_BUFFER_HEADER) -> u32);
    unsafe { ProcessTraceBufferDecrementReference(buffer) }
}
#[inline]
pub unsafe fn ProcessTraceBufferIncrementReference(tracehandle: PROCESSTRACE_HANDLE, buffer: *const ETW_BUFFER_HEADER) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn ProcessTraceBufferIncrementReference(tracehandle : PROCESSTRACE_HANDLE, buffer : *const ETW_BUFFER_HEADER) -> u32);
    unsafe { ProcessTraceBufferIncrementReference(tracehandle, buffer) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn QueryAllTracesA(propertyarray: &mut [PEVENT_TRACE_PROPERTIES], loggercount: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn QueryAllTracesA(propertyarray : *mut PEVENT_TRACE_PROPERTIES, propertyarraycount : u32, loggercount : *mut u32) -> u32);
    unsafe { QueryAllTracesA(propertyarray.as_mut_ptr(), propertyarray.len().try_into().unwrap(), loggercount as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn QueryAllTracesW(propertyarray: &mut [PEVENT_TRACE_PROPERTIES], loggercount: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn QueryAllTracesW(propertyarray : *mut PEVENT_TRACE_PROPERTIES, propertyarraycount : u32, loggercount : *mut u32) -> u32);
    unsafe { QueryAllTracesW(propertyarray.as_mut_ptr(), propertyarray.len().try_into().unwrap(), loggercount as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn QueryTraceA<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn QueryTraceA(traceid : CONTROLTRACE_ID, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { QueryTraceA(traceid, instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn QueryTraceProcessingHandle(processinghandle: PROCESSTRACE_HANDLE, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: Option<*const core::ffi::c_void>, inbuffersize: u32, outbuffer: Option<*mut core::ffi::c_void>, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn QueryTraceProcessingHandle(processinghandle : PROCESSTRACE_HANDLE, informationclass : ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer : *const core::ffi::c_void, inbuffersize : u32, outbuffer : *mut core::ffi::c_void, outbuffersize : u32, returnlength : *mut u32) -> u32);
    unsafe { QueryTraceProcessingHandle(processinghandle, informationclass, inbuffer.unwrap_or(core::mem::zeroed()) as _, inbuffersize, outbuffer.unwrap_or(core::mem::zeroed()) as _, outbuffersize, returnlength as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn QueryTraceW<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn QueryTraceW(traceid : CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { QueryTraceW(traceid, instancename.param().abi(), properties as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn RegisterTraceGuidsA<P5, P6>(requestaddress: WMIDPREQUEST, requestcontext: Option<*const core::ffi::c_void>, controlguid: *const windows_core::GUID, traceguidreg: Option<&[TRACE_GUID_REGISTRATION]>, mofimagepath: P5, mofresourcename: P6, registrationhandle: *mut TRACEGUID_HANDLE) -> u32
where
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterTraceGuidsA(requestaddress : WMIDPREQUEST, requestcontext : *const core::ffi::c_void, controlguid : *const windows_core::GUID, guidcount : u32, traceguidreg : *const TRACE_GUID_REGISTRATION, mofimagepath : windows_core::PCSTR, mofresourcename : windows_core::PCSTR, registrationhandle : *mut TRACEGUID_HANDLE) -> u32);
    unsafe { RegisterTraceGuidsA(requestaddress, requestcontext.unwrap_or(core::mem::zeroed()) as _, controlguid, traceguidreg.map_or(0, |slice| slice.len().try_into().unwrap()), traceguidreg.map_or(core::ptr::null(), |slice| slice.as_ptr()), mofimagepath.param().abi(), mofresourcename.param().abi(), registrationhandle as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn RegisterTraceGuidsW<P5, P6>(requestaddress: WMIDPREQUEST, requestcontext: Option<*const core::ffi::c_void>, controlguid: *const windows_core::GUID, traceguidreg: Option<&[TRACE_GUID_REGISTRATION]>, mofimagepath: P5, mofresourcename: P6, registrationhandle: *mut TRACEGUID_HANDLE) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterTraceGuidsW(requestaddress : WMIDPREQUEST, requestcontext : *const core::ffi::c_void, controlguid : *const windows_core::GUID, guidcount : u32, traceguidreg : *const TRACE_GUID_REGISTRATION, mofimagepath : windows_core::PCWSTR, mofresourcename : windows_core::PCWSTR, registrationhandle : *mut TRACEGUID_HANDLE) -> u32);
    unsafe { RegisterTraceGuidsW(requestaddress, requestcontext.unwrap_or(core::mem::zeroed()) as _, controlguid, traceguidreg.map_or(0, |slice| slice.len().try_into().unwrap()), traceguidreg.map_or(core::ptr::null(), |slice| slice.as_ptr()), mofimagepath.param().abi(), mofresourcename.param().abi(), registrationhandle as _) }
}
#[inline]
pub unsafe fn RemoveTraceCallback(pguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn RemoveTraceCallback(pguid : *const windows_core::GUID) -> u32);
    unsafe { RemoveTraceCallback(pguid) }
}
#[inline]
pub unsafe fn SetTraceCallback(pguid: *const windows_core::GUID, eventcallback: PEVENT_CALLBACK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetTraceCallback(pguid : *const windows_core::GUID, eventcallback : PEVENT_CALLBACK) -> u32);
    unsafe { SetTraceCallback(pguid, eventcallback) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn StartTraceA<P1>(traceid: *mut CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn StartTraceA(traceid : *mut CONTROLTRACE_ID, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { StartTraceA(traceid as _, instancename.param().abi(), properties as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn StartTraceW<P1>(traceid: *mut CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn StartTraceW(traceid : *mut CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { StartTraceW(traceid as _, instancename.param().abi(), properties as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn StopTraceA<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn StopTraceA(traceid : CONTROLTRACE_ID, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { StopTraceA(traceid, instancename.param().abi(), properties as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn StopTraceW<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn StopTraceW(traceid : CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { StopTraceW(traceid, instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn TraceConfigureLastBranchRecord(traceid: CONTROLTRACE_ID, lbrconfiguration: TRACE_LBR_CONFIGURATION, events: &[CLASSIC_EVENT_ID]) -> u32 {
    windows_core::link!("advapi32.dll" "C" fn TraceConfigureLastBranchRecord(traceid : CONTROLTRACE_ID, lbrconfiguration : TRACE_LBR_CONFIGURATION, events : *const CLASSIC_EVENT_ID, eventcount : u32) -> u32);
    unsafe { TraceConfigureLastBranchRecord(traceid, lbrconfiguration, events.as_ptr(), events.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn TraceEvent(tracehandle: TRACELOGGER_HANDLE, eventtrace: *const EVENT_TRACE_HEADER) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn TraceEvent(tracehandle : TRACELOGGER_HANDLE, eventtrace : *const EVENT_TRACE_HEADER) -> u32);
    unsafe { TraceEvent(tracehandle, eventtrace) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TraceEventInstance(tracehandle: TRACELOGGER_HANDLE, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: Option<*const EVENT_INSTANCE_INFO>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn TraceEventInstance(tracehandle : TRACELOGGER_HANDLE, eventtrace : *const EVENT_INSTANCE_HEADER, instinfo : *const EVENT_INSTANCE_INFO, parentinstinfo : *const EVENT_INSTANCE_INFO) -> u32);
    unsafe { TraceEventInstance(tracehandle, eventtrace, instinfo, parentinstinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TraceMessage(loggerhandle: TRACELOGGER_HANDLE, messageflags: u32, messageguid: *const windows_core::GUID, messagenumber: u16) -> u32 {
    windows_core::link!("advapi32.dll" "C" fn TraceMessage(loggerhandle : TRACELOGGER_HANDLE, messageflags : u32, messageguid : *const windows_core::GUID, messagenumber : u16) -> u32);
    unsafe { TraceMessage(loggerhandle, messageflags, messageguid, messagenumber) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn TraceMessageVa(loggerhandle: TRACELOGGER_HANDLE, messageflags: u32, messageguid: *const windows_core::GUID, messagenumber: u16, messagearglist: *const i8) -> u32 {
    windows_core::link!("advapi32.dll" "C" fn TraceMessageVa(loggerhandle : TRACELOGGER_HANDLE, messageflags : u32, messageguid : *const windows_core::GUID, messagenumber : u16, messagearglist : *const i8) -> u32);
    unsafe { TraceMessageVa(loggerhandle, messageflags, messageguid, messagenumber, messagearglist) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn TraceMessageVa(loggerhandle: TRACELOGGER_HANDLE, messageflags: u32, messageguid: *const windows_core::GUID, messagenumber: u16, messagearglist: super::vadefs::va_list) -> u32 {
    windows_core::link!("advapi32.dll" "C" fn TraceMessageVa(loggerhandle : TRACELOGGER_HANDLE, messageflags : u32, messageguid : *const windows_core::GUID, messagenumber : u16, messagearglist : super::vadefs::va_list) -> u32);
    unsafe { TraceMessageVa(loggerhandle, messageflags, messageguid, messagenumber, messagearglist) }
}
#[inline]
pub unsafe fn TraceQueryInformation(traceid: CONTROLTRACE_ID, informationclass: TRACE_INFO_CLASS, traceinformation: *mut core::ffi::c_void, informationlength: u32, returnlength: Option<*mut u32>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn TraceQueryInformation(traceid : CONTROLTRACE_ID, informationclass : TRACE_INFO_CLASS, traceinformation : *mut core::ffi::c_void, informationlength : u32, returnlength : *mut u32) -> u32);
    unsafe { TraceQueryInformation(traceid, informationclass, traceinformation as _, informationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TraceSetInformation(traceid: CONTROLTRACE_ID, informationclass: TRACE_INFO_CLASS, traceinformation: *const core::ffi::c_void, informationlength: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn TraceSetInformation(traceid : CONTROLTRACE_ID, informationclass : TRACE_INFO_CLASS, traceinformation : *const core::ffi::c_void, informationlength : u32) -> u32);
    unsafe { TraceSetInformation(traceid, informationclass, traceinformation, informationlength) }
}
#[inline]
pub unsafe fn UnregisterTraceGuids(registrationhandle: TRACEGUID_HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn UnregisterTraceGuids(registrationhandle : TRACEGUID_HANDLE) -> u32);
    unsafe { UnregisterTraceGuids(registrationhandle) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn UpdateTraceA<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn UpdateTraceA(traceid : CONTROLTRACE_ID, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { UpdateTraceA(traceid, instancename.param().abi(), properties as _) }
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[inline]
pub unsafe fn UpdateTraceW<P1>(traceid: CONTROLTRACE_ID, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn UpdateTraceW(traceid : CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { UpdateTraceW(traceid, instancename.param().abi(), properties as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLASSIC_EVENT_ID {
    pub EventGuid: windows_core::GUID,
    pub Type: u8,
    pub Reserved: [u8; 7],
}
impl Default for CLASSIC_EVENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CONTROLTRACE_ID(pub u64);
pub const DIAG_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("DiagLog");
pub const DIAG_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("DiagLog");
pub const DefaultTraceSecurityGuid: windows_core::GUID = windows_core::GUID::from_u128(0x0811c1af_7a07_4a06_82ed_869455cdf713);
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: windows_core::GUID,
    pub EnableFilterDesc: PEVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: windows_core::GUID,
    pub EnableFilterDesc: PEVENT_FILTER_DESCRIPTOR,
}
pub const ENABLE_TRACE_PARAMETERS_VERSION: u32 = 1;
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: u32 = 2;
pub const ETW_ASCIICHAR_TYPE_VALUE: u32 = 102;
pub const ETW_ASCIISTRING_TYPE_VALUE: u32 = 103;
pub const ETW_BOOLEAN_TYPE_VALUE: u32 = 14;
pub const ETW_BOOL_TYPE_VALUE: u32 = 108;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ETW_BUFFER_CALLBACK_INFORMATION {
    pub TraceHandle: PROCESSTRACE_HANDLE,
    pub LogfileHeader: *const TRACE_LOGFILE_HEADER,
    pub BuffersRead: u32,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for ETW_BUFFER_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: u16,
}
impl Default for ETW_BUFFER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: u16,
}
impl Default for ETW_BUFFER_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: u8,
    pub Alignment: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ETW_BUFFER_HEADER {
    pub Reserved1: [u32; 4],
    pub TimeStamp: i64,
    pub Reserved2: [u32; 4],
    pub ClientContext: ETW_BUFFER_CONTEXT,
    pub Reserved3: u32,
    pub FilledBytes: u32,
    pub Reserved4: [u32; 5],
}
impl Default for ETW_BUFFER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ETW_BYTE_TYPE_VALUE: u32 = 4;
pub const ETW_CHAR_TYPE_VALUE: u32 = 11;
pub type ETW_COMPRESSION_RESUMPTION_MODE = i32;
pub type ETW_CONTEXT_REGISTER_TYPES = u32;
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 109;
pub const ETW_COUNTED_STRING_TYPE_VALUE: u32 = 104;
pub const ETW_DATETIME_TYPE_VALUE: u32 = 119;
pub const ETW_DECIMAL_TYPE_VALUE: u32 = 15;
pub const ETW_DOUBLE_TYPE_VALUE: u32 = 13;
pub const ETW_GUID_TYPE_VALUE: u32 = 101;
pub const ETW_HIDDEN_TYPE_VALUE: u32 = 107;
pub const ETW_INT16_TYPE_VALUE: u32 = 5;
pub const ETW_INT32_TYPE_VALUE: u32 = 7;
pub const ETW_INT64_TYPE_VALUE: u32 = 9;
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: u32 = 112;
pub const ETW_NULL_TYPE_VALUE: u32 = 0;
pub const ETW_OBJECT_TYPE_VALUE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug)]
pub struct ETW_OPEN_TRACE_OPTIONS {
    pub ProcessTraceModes: ETW_PROCESS_TRACE_MODES,
    pub EventCallback: PEVENT_RECORD_CALLBACK,
    pub EventCallbackContext: *mut core::ffi::c_void,
    pub BufferCallback: PETW_BUFFER_CALLBACK,
    pub BufferCallbackContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for ETW_OPEN_TRACE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: u32,
    pub OwnerTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    pub ProcessorNumber: u32,
    pub NumberOfCounters: u32,
    pub CounterOwners: [ETW_PMC_COUNTER_OWNER; 1],
}
impl Default for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ETW_PMC_COUNTER_OWNER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_PMC_SESSION_INFO {
    pub NextEntryOffset: u32,
    pub LoggerId: u16,
    pub Reserved: u16,
    pub ProfileSourceCount: u32,
    pub HookIdCount: u32,
}
pub const ETW_POINTER_TYPE_VALUE: u32 = 105;
pub type ETW_PROCESS_HANDLE_INFO_TYPE = i32;
pub type ETW_PROCESS_TRACE_MODES = i32;
pub const ETW_PROCESS_TRACE_MODE_NONE: ETW_PROCESS_TRACE_MODES = 0;
pub const ETW_PROCESS_TRACE_MODE_RAW_TIMESTAMP: ETW_PROCESS_TRACE_MODES = 1;
pub const ETW_PTVECTOR_TYPE_VALUE: u32 = 117;
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: u32 = 113;
pub const ETW_REDUCED_STRING_TYPE_VALUE: u32 = 114;
pub const ETW_REFRENCE_TYPE_VALUE: u32 = 120;
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 111;
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: u32 = 110;
pub const ETW_SBYTE_TYPE_VALUE: u32 = 3;
pub const ETW_SID_TYPE_VALUE: u32 = 115;
pub const ETW_SINGLE_TYPE_VALUE: u32 = 12;
pub const ETW_SIZET_TYPE_VALUE: u32 = 106;
pub const ETW_STRING_TYPE_VALUE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: windows_core::GUID,
    pub ParentId: windows_core::GUID,
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
    pub PartitionId: windows_core::PWSTR,
    pub ParentId: windows_core::PWSTR,
}
pub const ETW_UINT16_TYPE_VALUE: u32 = 6;
pub const ETW_UINT32_TYPE_VALUE: u32 = 8;
pub const ETW_UINT64_TYPE_VALUE: u32 = 10;
pub const ETW_VARIANT_TYPE_VALUE: u32 = 116;
pub const ETW_WMITIME_TYPE_VALUE: u32 = 118;
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: u32 = 2;
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: u32 = 0;
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER {
    pub Size: u16,
    pub Anonymous: EVENT_INSTANCE_HEADER_0,
    pub Anonymous2: EVENT_INSTANCE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub RegHandle: u64,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub Anonymous3: EVENT_INSTANCE_HEADER_2,
    pub ParentRegHandle: u64,
}
impl Default for EVENT_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_INSTANCE_HEADER_0_0,
}
impl Default for EVENT_INSTANCE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_INSTANCE_HEADER_1_0,
}
impl Default for EVENT_INSTANCE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
impl Default for EVENT_INSTANCE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::winnt::HANDLE,
    pub InstanceId: u32,
}
pub const EVENT_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("EventLog");
pub const EVENT_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("EventLog");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: windows_core::GUID,
    pub MofData: *mut core::ffi::c_void,
    pub MofLength: u32,
    pub Anonymous: EVENT_TRACE_0,
}
impl Default for EVENT_TRACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_0 {
    pub ClientContext: u32,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
impl Default for EVENT_TRACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648;
pub const EVENT_TRACE_ADD_HEADER_MODE: u32 = 4096;
pub const EVENT_TRACE_BUFFERING_MODE: u32 = 1024;
pub const EVENT_TRACE_COMPRESSED_MODE: u32 = 67108864;
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: u32 = 5;
pub const EVENT_TRACE_CONTROL_FLUSH: u32 = 3;
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: u32 = 4;
pub const EVENT_TRACE_CONTROL_QUERY: u32 = 0;
pub const EVENT_TRACE_CONTROL_STOP: u32 = 1;
pub const EVENT_TRACE_CONTROL_UPDATE: u32 = 2;
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: u32 = 512;
pub const EVENT_TRACE_FILE_MODE_APPEND: u32 = 4;
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: u32 = 2;
pub const EVENT_TRACE_FILE_MODE_NEWFILE: u32 = 8;
pub const EVENT_TRACE_FILE_MODE_NONE: u32 = 0;
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: u32 = 32;
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: u32 = 1;
pub const EVENT_TRACE_FLAG_ALPC: u32 = 1048576;
pub const EVENT_TRACE_FLAG_CSWITCH: u32 = 16;
pub const EVENT_TRACE_FLAG_DBGPRINT: u32 = 262144;
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: u32 = 4194304;
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: u32 = 512;
pub const EVENT_TRACE_FLAG_DISK_IO: u32 = 256;
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: u32 = 1024;
pub const EVENT_TRACE_FLAG_DISPATCHER: u32 = 2048;
pub const EVENT_TRACE_FLAG_DPC: u32 = 32;
pub const EVENT_TRACE_FLAG_DRIVER: u32 = 8388608;
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: u32 = 536870912;
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648;
pub const EVENT_TRACE_FLAG_FILE_IO: u32 = 33554432;
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: u32 = 67108864;
pub const EVENT_TRACE_FLAG_FORWARD_WMI: u32 = 1073741824;
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: u32 = 4;
pub const EVENT_TRACE_FLAG_INTERRUPT: u32 = 64;
pub const EVENT_TRACE_FLAG_JOB: u32 = 524288;
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: u32 = 8192;
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: u32 = 4096;
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: u32 = 65536;
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: u32 = 268435456;
pub const EVENT_TRACE_FLAG_PROCESS: u32 = 1;
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: u32 = 8;
pub const EVENT_TRACE_FLAG_PROFILE: u32 = 16777216;
pub const EVENT_TRACE_FLAG_REGISTRY: u32 = 131072;
pub const EVENT_TRACE_FLAG_SPLIT_IO: u32 = 2097152;
pub const EVENT_TRACE_FLAG_SYSTEMCALL: u32 = 128;
pub const EVENT_TRACE_FLAG_THREAD: u32 = 2;
pub const EVENT_TRACE_FLAG_VAMAP: u32 = 32768;
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER {
    pub Size: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0,
    pub Anonymous2: EVENT_TRACE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub Anonymous3: EVENT_TRACE_HEADER_2,
    pub Anonymous4: EVENT_TRACE_HEADER_3,
}
impl Default for EVENT_TRACE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
impl Default for EVENT_TRACE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
impl Default for EVENT_TRACE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: windows_core::GUID,
    pub GuidPtr: u64,
}
impl Default for EVENT_TRACE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: u32 = 134217728;
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_LOGFILEA {
    pub LogFileName: windows_core::PSTR,
    pub LoggerName: windows_core::PSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous: EVENT_TRACE_LOGFILEA_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKA,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEA_1,
    pub IsKernelTrace: u32,
    pub Context: *mut core::ffi::c_void,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for EVENT_TRACE_LOGFILEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for EVENT_TRACE_LOGFILEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for EVENT_TRACE_LOGFILEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: windows_core::PWSTR,
    pub LoggerName: windows_core::PWSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous: EVENT_TRACE_LOGFILEW_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKW,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEW_1,
    pub IsKernelTrace: u32,
    pub Context: *mut core::ffi::c_void,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
impl Default for EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_MODE_RESERVED: u32 = 1048576;
pub const EVENT_TRACE_NONSTOPPABLE_MODE: u32 = 64;
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: u32 = 268435456;
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: u32 = 8388608;
pub const EVENT_TRACE_PRIVATE_IN_PROC: u32 = 131072;
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: u32 = 2048;
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES {
    pub Wnode: super::wmistr::WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: u32,
    pub Anonymous: EVENT_TRACE_PROPERTIES_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::winnt::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(all(feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES_V2 {
    pub Wnode: super::wmistr::WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: u32,
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::winnt::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
    pub Anonymous2: EVENT_TRACE_PROPERTIES_V2_1,
    pub FilterDescCount: u32,
    pub FilterDesc: PEVENT_FILTER_DESCRIPTOR,
    pub Anonymous3: EVENT_TRACE_PROPERTIES_V2_2,
}
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: u32,
}
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: u64,
}
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
impl Default for EVENT_TRACE_PROPERTIES_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
pub const EVENT_TRACE_REAL_TIME_MODE: u32 = 256;
pub const EVENT_TRACE_RELOG_MODE: u32 = 65536;
pub const EVENT_TRACE_SECURE_MODE: u32 = 128;
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: u32 = 4194304;
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: u32 = 33554432;
pub const EVENT_TRACE_TYPE_ACCEPT: u32 = 15;
pub const EVENT_TRACE_TYPE_ACKDUP: u32 = 22;
pub const EVENT_TRACE_TYPE_ACKFULL: u32 = 20;
pub const EVENT_TRACE_TYPE_ACKPART: u32 = 21;
pub const EVENT_TRACE_TYPE_CHECKPOINT: u32 = 8;
pub const EVENT_TRACE_TYPE_CONFIG: u32 = 11;
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: u32 = 37;
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: u32 = 29;
pub const EVENT_TRACE_TYPE_CONFIG_CPU: u32 = 10;
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: u32 = 31;
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: u32 = 33;
pub const EVENT_TRACE_TYPE_CONFIG_DPI: u32 = 28;
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: u32 = 34;
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: u32 = 23;
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: u32 = 21;
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: u32 = 12;
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: u32 = 30;
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: u32 = 32;
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: u32 = 17;
pub const EVENT_TRACE_TYPE_CONFIG_NIC: u32 = 13;
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: u32 = 24;
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: u32 = 18;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: u32 = 11;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK_EX: u32 = 19;
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: u32 = 25;
pub const EVENT_TRACE_TYPE_CONFIG_PNP: u32 = 22;
pub const EVENT_TRACE_TYPE_CONFIG_POWER: u32 = 16;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: u32 = 35;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: u32 = 26;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: u32 = 27;
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: u32 = 15;
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: u32 = 14;
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: u32 = 36;
pub const EVENT_TRACE_TYPE_CONNECT: u32 = 12;
pub const EVENT_TRACE_TYPE_CONNFAIL: u32 = 17;
pub const EVENT_TRACE_TYPE_COPY_ARP: u32 = 19;
pub const EVENT_TRACE_TYPE_COPY_TCP: u32 = 18;
pub const EVENT_TRACE_TYPE_DBGID_RSDS: u32 = 64;
pub const EVENT_TRACE_TYPE_DC_END: u32 = 4;
pub const EVENT_TRACE_TYPE_DC_START: u32 = 3;
pub const EVENT_TRACE_TYPE_DEQUEUE: u32 = 7;
pub const EVENT_TRACE_TYPE_DISCONNECT: u32 = 13;
pub const EVENT_TRACE_TYPE_END: u32 = 2;
pub const EVENT_TRACE_TYPE_EXTENSION: u32 = 5;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: u32 = 99;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: u32 = 101;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: u32 = 97;
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: u32 = 98;
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: u32 = 100;
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: u32 = 96;
pub const EVENT_TRACE_TYPE_GUIDMAP: u32 = 10;
pub const EVENT_TRACE_TYPE_INFO: u32 = 0;
pub const EVENT_TRACE_TYPE_IO_FLUSH: u32 = 14;
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: u32 = 15;
pub const EVENT_TRACE_TYPE_IO_READ: u32 = 10;
pub const EVENT_TRACE_TYPE_IO_READ_INIT: u32 = 12;
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: u32 = 16;
pub const EVENT_TRACE_TYPE_IO_WRITE: u32 = 11;
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: u32 = 13;
pub const EVENT_TRACE_TYPE_LOAD: u32 = 10;
pub const EVENT_TRACE_TYPE_MM_AV: u32 = 15;
pub const EVENT_TRACE_TYPE_MM_COW: u32 = 12;
pub const EVENT_TRACE_TYPE_MM_DZF: u32 = 11;
pub const EVENT_TRACE_TYPE_MM_GPF: u32 = 13;
pub const EVENT_TRACE_TYPE_MM_HPF: u32 = 14;
pub const EVENT_TRACE_TYPE_MM_TF: u32 = 10;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: u32 = 57;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: u32 = 60;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: u32 = 55;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: u32 = 58;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: u32 = 56;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: u32 = 59;
pub const EVENT_TRACE_TYPE_RECEIVE: u32 = 11;
pub const EVENT_TRACE_TYPE_RECONNECT: u32 = 16;
pub const EVENT_TRACE_TYPE_REGCLOSE: u32 = 27;
pub const EVENT_TRACE_TYPE_REGCOMMIT: u32 = 30;
pub const EVENT_TRACE_TYPE_REGCREATE: u32 = 10;
pub const EVENT_TRACE_TYPE_REGDELETE: u32 = 12;
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: u32 = 15;
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: u32 = 17;
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: u32 = 18;
pub const EVENT_TRACE_TYPE_REGFLUSH: u32 = 21;
pub const EVENT_TRACE_TYPE_REGKCBCREATE: u32 = 22;
pub const EVENT_TRACE_TYPE_REGKCBDELETE: u32 = 23;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: u32 = 24;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: u32 = 25;
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: u32 = 33;
pub const EVENT_TRACE_TYPE_REGOPEN: u32 = 11;
pub const EVENT_TRACE_TYPE_REGPREPARE: u32 = 31;
pub const EVENT_TRACE_TYPE_REGQUERY: u32 = 13;
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: u32 = 19;
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: u32 = 29;
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: u32 = 16;
pub const EVENT_TRACE_TYPE_REGROLLBACK: u32 = 32;
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: u32 = 20;
pub const EVENT_TRACE_TYPE_REGSETSECURITY: u32 = 28;
pub const EVENT_TRACE_TYPE_REGSETVALUE: u32 = 14;
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: u32 = 26;
pub const EVENT_TRACE_TYPE_REPLY: u32 = 6;
pub const EVENT_TRACE_TYPE_RESUME: u32 = 7;
pub const EVENT_TRACE_TYPE_RETRANSMIT: u32 = 14;
pub const EVENT_TRACE_TYPE_SECURITY: u32 = 13;
pub const EVENT_TRACE_TYPE_SEND: u32 = 10;
pub const EVENT_TRACE_TYPE_SIDINFO: u32 = 12;
pub const EVENT_TRACE_TYPE_START: u32 = 1;
pub const EVENT_TRACE_TYPE_STOP: u32 = 2;
pub const EVENT_TRACE_TYPE_SUSPEND: u32 = 8;
pub const EVENT_TRACE_TYPE_TERMINATE: u32 = 11;
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: u32 = 240;
pub const EVENT_TRACE_TYPE_WINEVT_SEND: u32 = 9;
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: u32 = 16384;
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: u32 = 8192;
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: u32 = 32768;
pub const EVENT_TRACE_USE_NOCPUTIME: u32 = 2;
pub const EVENT_TRACE_USE_PAGED_MEMORY: u32 = 16777216;
pub const EVENT_TRACE_USE_PROCTIME: u32 = 1;
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
pub const EventTraceConfigGuid: windows_core::GUID = windows_core::GUID::from_u128(0x01853a65_418f_4f36_aefc_dc0f1d2fd235);
pub const EventTraceGuid: windows_core::GUID = windows_core::GUID::from_u128(0x68fdd900_4a3e_11d1_84f4_0000f80464e3);
pub const GLOBAL_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("GlobalLogger");
pub const GLOBAL_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("GlobalLogger");
pub const INVALID_PROCESSTRACE_HANDLE: i32 = -1;
pub const KERNEL_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("NT Kernel Logger");
pub const KERNEL_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("NT Kernel Logger");
pub const LastBranchRecordProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x99134383_5248_43fc_834b_529454e75df3);
pub const MAX_MOF_FIELDS: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOF_FIELD {
    pub DataPtr: u64,
    pub Length: u32,
    pub DataType: u32,
}
pub const MaxTraceSetInfoClass: TRACE_QUERY_INFO_CLASS = 29;
pub type PCLASSIC_EVENT_ID = *mut CLASSIC_EVENT_ID;
#[cfg(feature = "evntprov")]
pub type PENABLE_TRACE_PARAMETERS = *mut ENABLE_TRACE_PARAMETERS;
#[cfg(feature = "evntprov")]
pub type PENABLE_TRACE_PARAMETERS_V1 = *mut ENABLE_TRACE_PARAMETERS_V1;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
pub type PETW_BUFFER_CALLBACK = Option<unsafe extern "system" fn(buffer: *const ETW_BUFFER_HEADER, buffersize: u32, consumerinfo: *const ETW_BUFFER_CALLBACK_INFORMATION, callbackcontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PETW_BUFFER_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(buffer: *const ETW_BUFFER_HEADER, callbackcontext: *const core::ffi::c_void)>;
pub type PETW_BUFFER_CONTEXT = *mut ETW_BUFFER_CONTEXT;
pub type PETW_PMC_COUNTER_OWNER = *mut ETW_PMC_COUNTER_OWNER;
pub type PETW_PMC_COUNTER_OWNERSHIP_STATUS = *mut ETW_PMC_COUNTER_OWNERSHIP_STATUS;
pub type PETW_TRACE_PARTITION_INFORMATION = *mut ETW_TRACE_PARTITION_INFORMATION;
pub type PETW_TRACE_PARTITION_INFORMATION_V2 = *mut ETW_TRACE_PARTITION_INFORMATION_V2;
pub type PEVENT_CALLBACK = Option<unsafe extern "system" fn(pevent: *mut EVENT_TRACE)>;
#[cfg(feature = "evntprov")]
pub type PEVENT_FILTER_DESCRIPTOR = *mut super::evntprov::EVENT_FILTER_DESCRIPTOR;
pub type PEVENT_INSTANCE_HEADER = *mut EVENT_INSTANCE_HEADER;
#[cfg(feature = "winnt")]
pub type PEVENT_INSTANCE_INFO = *mut EVENT_INSTANCE_INFO;
#[cfg(all(feature = "evntcons", feature = "evntprov"))]
pub type PEVENT_RECORD = *mut super::evntcons::EVENT_RECORD;
#[cfg(all(feature = "evntcons", feature = "evntprov"))]
pub type PEVENT_RECORD_CALLBACK = Option<unsafe extern "system" fn(eventrecord: *mut super::evntcons::EVENT_RECORD)>;
pub type PEVENT_TRACE = *mut EVENT_TRACE;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEA) -> u32>;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEW) -> u32>;
pub type PEVENT_TRACE_HEADER = *mut EVENT_TRACE_HEADER;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
pub type PEVENT_TRACE_LOGFILEA = *mut EVENT_TRACE_LOGFILEA;
#[cfg(all(feature = "evntcons", feature = "evntprov", feature = "minwinbase", feature = "timezoneapi"))]
pub type PEVENT_TRACE_LOGFILEW = *mut EVENT_TRACE_LOGFILEW;
#[cfg(all(feature = "winnt", feature = "wmistr"))]
pub type PEVENT_TRACE_PROPERTIES = *mut EVENT_TRACE_PROPERTIES;
#[cfg(all(feature = "evntprov", feature = "winnt", feature = "wmistr"))]
pub type PEVENT_TRACE_PROPERTIES_V2 = *mut EVENT_TRACE_PROPERTIES_V2;
pub type PMOF_FIELD = *mut MOF_FIELD;
pub type PPROFILE_SOURCE_INFO = *mut PROFILE_SOURCE_INFO;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROCESSTRACE_HANDLE(pub u64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROFILE_SOURCE_INFO {
    pub NextEntryOffset: u32,
    pub Source: u32,
    pub MinInterval: u32,
    pub MaxInterval: u32,
    pub Reserved: u64,
    pub Description: [u16; 1],
}
impl Default for PROFILE_SOURCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTRACEHANDLE = *mut u64;
pub type PTRACE_ENABLE_INFO = *mut TRACE_ENABLE_INFO;
pub type PTRACE_GUID_INFO = *mut TRACE_GUID_INFO;
pub type PTRACE_GUID_PROPERTIES = *mut TRACE_GUID_PROPERTIES;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type PTRACE_GUID_REGISTRATION = *mut TRACE_GUID_REGISTRATION;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
pub type PTRACE_LOGFILE_HEADER = *mut TRACE_LOGFILE_HEADER;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
pub type PTRACE_LOGFILE_HEADER32 = *mut TRACE_LOGFILE_HEADER32;
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
pub type PTRACE_LOGFILE_HEADER64 = *mut TRACE_LOGFILE_HEADER64;
pub type PTRACE_PERIODIC_CAPTURE_STATE_INFO = *mut TRACE_PERIODIC_CAPTURE_STATE_INFO;
pub type PTRACE_PROFILE_INTERVAL = *mut TRACE_PROFILE_INTERVAL;
pub type PTRACE_PROVIDER_INSTANCE_INFO = *mut TRACE_PROVIDER_INSTANCE_INFO;
pub type PTRACE_STACK_CACHING_INFO = *mut TRACE_STACK_CACHING_INFO;
pub type PTRACE_VERSION_INFO = *mut TRACE_VERSION_INFO;
pub const PrivateLoggerNotificationGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3595ab5c_042a_4c8e_b942_2d059bfeb1b1);
pub const SYSTEM_ALPC_KW_GENERAL: u32 = 1;
pub const SYSTEM_CONFIG_KW_GRAPHICS: u32 = 2;
pub const SYSTEM_CONFIG_KW_NETWORK: u32 = 8;
pub const SYSTEM_CONFIG_KW_OPTICAL: u32 = 64;
pub const SYSTEM_CONFIG_KW_PNP: u32 = 32;
pub const SYSTEM_CONFIG_KW_SERVICES: u32 = 16;
pub const SYSTEM_CONFIG_KW_STORAGE: u32 = 4;
pub const SYSTEM_CONFIG_KW_SYSTEM: u32 = 1;
pub const SYSTEM_CPU_KW_CACHE_FLUSH: u32 = 2;
pub const SYSTEM_CPU_KW_CONFIG: u32 = 1;
pub const SYSTEM_CPU_KW_DOMAIN_CHANGE: u32 = 8;
pub const SYSTEM_CPU_KW_SPEC_CONTROL: u32 = 4;
pub const SYSTEM_EVENT_TYPE: u32 = 1;
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: u32 = 2;
pub const SYSTEM_HYPERVISOR_KW_PROFILE: u32 = 1;
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: u32 = 4;
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: u32 = 2;
pub const SYSTEM_INTERRUPT_KW_DPC: u32 = 4;
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: u32 = 8;
pub const SYSTEM_INTERRUPT_KW_GENERAL: u32 = 1;
pub const SYSTEM_INTERRUPT_KW_IPI: u32 = 64;
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: u32 = 16;
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: u32 = 32;
pub const SYSTEM_IOFILTER_KW_FAILURE: u32 = 8;
pub const SYSTEM_IOFILTER_KW_FASTIO: u32 = 4;
pub const SYSTEM_IOFILTER_KW_GENERAL: u32 = 1;
pub const SYSTEM_IOFILTER_KW_INIT: u32 = 2;
pub const SYSTEM_IO_KW_CC: u32 = 256;
pub const SYSTEM_IO_KW_DISK: u32 = 1;
pub const SYSTEM_IO_KW_DISK_INIT: u32 = 2;
pub const SYSTEM_IO_KW_DRIVERS: u32 = 128;
pub const SYSTEM_IO_KW_FILE: u32 = 16;
pub const SYSTEM_IO_KW_FILENAME: u32 = 4;
pub const SYSTEM_IO_KW_FILE_INIT: u32 = 1024;
pub const SYSTEM_IO_KW_NETWORK: u32 = 512;
pub const SYSTEM_IO_KW_OPTICAL: u32 = 32;
pub const SYSTEM_IO_KW_OPTICAL_INIT: u32 = 64;
pub const SYSTEM_IO_KW_SPLIT: u32 = 8;
pub const SYSTEM_IO_KW_TIMER: u32 = 2048;
pub const SYSTEM_LOCK_KW_SPINLOCK: u32 = 1;
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: u32 = 2;
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: u32 = 4;
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: u32 = 4;
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: u32 = 512;
pub const SYSTEM_MEMORY_KW_FOOTPRINT: u32 = 2048;
pub const SYSTEM_MEMORY_KW_GENERAL: u32 = 1;
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: u32 = 2;
pub const SYSTEM_MEMORY_KW_HEAP: u32 = 128;
pub const SYSTEM_MEMORY_KW_MEMINFO: u32 = 16;
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: u32 = 64;
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: u32 = 32768;
pub const SYSTEM_MEMORY_KW_PFSECTION: u32 = 32;
pub const SYSTEM_MEMORY_KW_POOL: u32 = 8;
pub const SYSTEM_MEMORY_KW_REFSET: u32 = 8192;
pub const SYSTEM_MEMORY_KW_SESSION: u32 = 4096;
pub const SYSTEM_MEMORY_KW_VAMAP: u32 = 16384;
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: u32 = 1024;
pub const SYSTEM_MEMORY_KW_WS: u32 = 256;
pub const SYSTEM_MEMORY_POOL_FILTER_ID: u32 = 1;
pub const SYSTEM_OBJECT_KW_GENERAL: u32 = 1;
pub const SYSTEM_OBJECT_KW_HANDLE: u32 = 2;
pub const SYSTEM_POWER_KW_GENERAL: u32 = 1;
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: u32 = 2;
pub const SYSTEM_POWER_KW_IDLE_SELECTION: u32 = 8;
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: u32 = 16;
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: u32 = 4;
pub const SYSTEM_PROCESS_KW_DBGPRINT: u32 = 256;
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: u32 = 128;
pub const SYSTEM_PROCESS_KW_FREEZE: u32 = 4;
pub const SYSTEM_PROCESS_KW_GENERAL: u32 = 1;
pub const SYSTEM_PROCESS_KW_INSWAP: u32 = 2;
pub const SYSTEM_PROCESS_KW_JOB: u32 = 512;
pub const SYSTEM_PROCESS_KW_LOADER: u32 = 4096;
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: u32 = 8;
pub const SYSTEM_PROCESS_KW_THREAD: u32 = 2048;
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: u32 = 16;
pub const SYSTEM_PROCESS_KW_WAKE_DROP: u32 = 32;
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: u32 = 64;
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: u32 = 1024;
pub const SYSTEM_PROFILE_KW_GENERAL: u32 = 1;
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: u32 = 2;
pub const SYSTEM_REGISTRY_KW_GENERAL: u32 = 1;
pub const SYSTEM_REGISTRY_KW_HIVE: u32 = 2;
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: u32 = 4;
pub const SYSTEM_SCHEDULER_KW_AFFINITY: u32 = 64;
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: u32 = 16;
pub const SYSTEM_SCHEDULER_KW_AUTOBOOST: u32 = 65536;
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: u32 = 1024;
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: u32 = 512;
pub const SYSTEM_SCHEDULER_KW_CPU_PARTITION: u32 = 8192;
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: u32 = 2;
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: u32 = 256;
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: u32 = 4;
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: u32 = 32;
pub const SYSTEM_SCHEDULER_KW_PRIORITY: u32 = 128;
pub const SYSTEM_SCHEDULER_KW_READY_QUEUE: u32 = 4096;
pub const SYSTEM_SCHEDULER_KW_SCHEDULE_THREAD: u32 = 2048;
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: u32 = 8;
pub const SYSTEM_SCHEDULER_KW_THREAD_FEEDBACK_READ: u32 = 16384;
pub const SYSTEM_SCHEDULER_KW_WORKLOAD_CLASS_UPDATE: u32 = 32768;
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: u32 = 1;
pub const SYSTEM_SYSCALL_KW_GENERAL: u32 = 1;
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: u32 = 2;
pub const SYSTEM_TIMER_KW_GENERAL: u32 = 1;
pub const SystemAlpcProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xfcb9baaf_e529_4980_92e9_ced1a6aadfdf);
pub const SystemConfigProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xfef3a8b6_318d_4b67_a96a_3b0f6b8f18fe);
pub const SystemCpuProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xc6c5265f_eae8_4650_aae4_9d48603d8510);
pub const SystemHypervisorProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xbafa072a_918a_4bed_b622_bc152097098f);
pub const SystemInterruptProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xd4bbee17_b545_4888_858b_744169015b25);
pub const SystemIoFilterProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xfbd09363_9e22_4661_b8bf_e7a34b535b8c);
pub const SystemIoProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3d5c43e3_0f1c_4202_b817_174c0070dc79);
pub const SystemLockProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x721ddfd3_dacc_4e1e_b26a_a2cb31d4705a);
pub const SystemMemoryProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x82958ca9_b6cd_47f8_a3a8_03ae85a4bc24);
pub const SystemObjectProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xfebd7460_3d1d_47eb_af49_c9eeb1e146f2);
pub const SystemPowerProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xc134884a_32d5_4488_80e5_14ed7abb8269);
pub const SystemProcessProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x151f55dc_467d_471f_83b5_5f889d46ff66);
pub const SystemProfileProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0xbfeb0324_1cee_496f_a409_2ac2b48a6322);
pub const SystemRegistryProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x16156bd9_fab4_4cfa_a232_89d1099058e3);
pub const SystemSchedulerProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x599a2a76_4d91_4910_9ac7_7d33f2e97a6c);
pub const SystemSyscallProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x434286f7_6f1b_45bb_b37e_95f623046c7c);
pub const SystemTimerProviderGuid: windows_core::GUID = windows_core::GUID::from_u128(0x4f061568_e215_499f_ab2e_eda0ae890a5b);
pub const SystemTraceControlGuid: windows_core::GUID = windows_core::GUID::from_u128(0x9e814aad_3204_11d2_9a82_006008a86939);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TRACEGUID_HANDLE(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TRACEHANDLE(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TRACELOGGER_HANDLE(pub u64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_CONTEXT_REGISTER_INFO {
    pub RegisterTypes: ETW_CONTEXT_REGISTER_TYPES,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_ENABLE_INFO {
    pub IsEnabled: u32,
    pub Level: u8,
    pub Reserved1: u8,
    pub LoggerId: u16,
    pub EnableProperty: u32,
    pub Reserved2: u32,
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: windows_core::GUID,
    pub GuidType: u32,
    pub LoggerId: u32,
    pub EnableLevel: u32,
    pub EnableFlags: u32,
    pub IsEnable: bool,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: super::guiddef::LPCGUID,
    pub RegHandle: super::winnt::HANDLE,
}
pub const TRACE_HEADER_FLAG_LOG_WNODE: u32 = 262144;
pub const TRACE_HEADER_FLAG_TRACED_GUID: u32 = 131072;
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: u32 = 524288;
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: u32 = 1048576;
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: u32 = 512;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TRACE_INFO_CLASS(pub TRACE_QUERY_INFO_CLASS);
pub type TRACE_LBR_CONFIGURATION = u32;
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
pub const TRACE_LBR_EVENT_OPCODE: u32 = 32;
pub const TRACE_LBR_MAXIMUM_EVENTS: u32 = 4;
pub const TRACE_LEVEL_CRITICAL: u32 = 1;
pub const TRACE_LEVEL_ERROR: u32 = 2;
pub const TRACE_LEVEL_FATAL: u32 = 1;
pub const TRACE_LEVEL_INFORMATION: u32 = 4;
pub const TRACE_LEVEL_NONE: u32 = 0;
pub const TRACE_LEVEL_RESERVED6: u32 = 6;
pub const TRACE_LEVEL_RESERVED7: u32 = 7;
pub const TRACE_LEVEL_RESERVED8: u32 = 8;
pub const TRACE_LEVEL_RESERVED9: u32 = 9;
pub const TRACE_LEVEL_VERBOSE: u32 = 5;
pub const TRACE_LEVEL_WARNING: u32 = 3;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER {
    pub BufferSize: u32,
    pub Anonymous: TRACE_LOGFILE_HEADER_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER_1,
    pub LoggerName: windows_core::PWSTR,
    pub LogFileName: windows_core::PWSTR,
    pub TimeZone: super::timezoneapi::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER32 {
    pub BufferSize: u32,
    pub Anonymous: TRACE_LOGFILE_HEADER32_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER32_1,
    pub LoggerName: u32,
    pub LogFileName: u32,
    pub TimeZone: super::timezoneapi::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER32_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER64 {
    pub BufferSize: u32,
    pub Anonymous: TRACE_LOGFILE_HEADER64_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER64_1,
    pub LoggerName: u64,
    pub LogFileName: u64,
    pub TimeZone: super::timezoneapi::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
impl Default for TRACE_LOGFILE_HEADER64_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "timezoneapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
pub const TRACE_MESSAGE_COMPONENTID: u32 = 4;
pub const TRACE_MESSAGE_FLAG_MASK: u32 = 65535;
pub const TRACE_MESSAGE_GUID: u32 = 2;
pub const TRACE_MESSAGE_MAXIMUM_SIZE: u32 = 65536;
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: u32 = 16;
pub const TRACE_MESSAGE_POINTER32: u32 = 64;
pub const TRACE_MESSAGE_POINTER64: u32 = 128;
pub const TRACE_MESSAGE_SEQUENCE: u32 = 1;
pub const TRACE_MESSAGE_SYSTEMINFO: u32 = 32;
pub const TRACE_MESSAGE_TIMESTAMP: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: u32,
    pub ProviderCount: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: u32,
    pub Interval: u32,
}
pub const TRACE_PROVIDER_FLAG_LEGACY: u32 = 1;
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: u32,
    pub EnableCount: u32,
    pub Pid: u32,
    pub Flags: u32,
}
pub type TRACE_QUERY_INFO_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: bool,
    pub CacheSize: u32,
    pub BucketCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: u32,
    pub Reserved: u32,
}
pub const TraceContextRegisterInfo: TRACE_QUERY_INFO_CLASS = 28;
pub const TraceDisallowListQuery: TRACE_QUERY_INFO_CLASS = 14;
pub const TraceGroupQueryInfo: TRACE_QUERY_INFO_CLASS = 13;
pub const TraceGroupQueryList: TRACE_QUERY_INFO_CLASS = 12;
pub const TraceGuidQueryInfo: TRACE_QUERY_INFO_CLASS = 1;
pub const TraceGuidQueryList: TRACE_QUERY_INFO_CLASS = 0;
pub const TraceGuidQueryProcess: TRACE_QUERY_INFO_CLASS = 2;
pub const TraceInfoReserved15: TRACE_QUERY_INFO_CLASS = 15;
pub const TraceLbrConfigurationInfo: TRACE_QUERY_INFO_CLASS = 20;
pub const TraceLbrEventListInfo: TRACE_QUERY_INFO_CLASS = 21;
pub const TraceMaxLoggersQuery: TRACE_QUERY_INFO_CLASS = 19;
pub const TraceMaxPmcCounterQuery: TRACE_QUERY_INFO_CLASS = 22;
pub const TracePeriodicCaptureStateInfo: TRACE_QUERY_INFO_CLASS = 17;
pub const TracePeriodicCaptureStateListInfo: TRACE_QUERY_INFO_CLASS = 16;
pub const TracePmcCounterListInfo: TRACE_QUERY_INFO_CLASS = 9;
pub const TracePmcCounterOwners: TRACE_QUERY_INFO_CLASS = 25;
pub const TracePmcEventListInfo: TRACE_QUERY_INFO_CLASS = 8;
pub const TracePmcSessionInformation: TRACE_QUERY_INFO_CLASS = 27;
pub const TraceProfileSourceConfigInfo: TRACE_QUERY_INFO_CLASS = 6;
pub const TraceProfileSourceListInfo: TRACE_QUERY_INFO_CLASS = 7;
pub const TraceProviderBinaryTracking: TRACE_QUERY_INFO_CLASS = 18;
pub const TraceSampledProfileIntervalInfo: TRACE_QUERY_INFO_CLASS = 5;
pub const TraceSetDisallowList: TRACE_QUERY_INFO_CLASS = 10;
pub const TraceStackCachingInfo: TRACE_QUERY_INFO_CLASS = 24;
pub const TraceStackTracingInfo: TRACE_QUERY_INFO_CLASS = 3;
pub const TraceStreamCount: TRACE_QUERY_INFO_CLASS = 23;
pub const TraceSystemTraceEnableFlagsInfo: TRACE_QUERY_INFO_CLASS = 4;
pub const TraceUnifiedStackCachingInfo: TRACE_QUERY_INFO_CLASS = 26;
pub const TraceVersionInfo: TRACE_QUERY_INFO_CLASS = 11;
#[cfg(feature = "wmistr")]
pub type WMIDPREQUEST = Option<unsafe extern "system" fn(requestcode: super::wmistr::WMIDPREQUESTCODE, requestcontext: *const core::ffi::c_void, buffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32>;
