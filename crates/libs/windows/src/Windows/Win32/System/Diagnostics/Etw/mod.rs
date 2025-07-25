#[inline]
pub unsafe fn CloseTrace(tracehandle: PROCESSTRACE_HANDLE) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn CloseTrace(tracehandle : PROCESSTRACE_HANDLE) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { CloseTrace(core::mem::transmute(tracehandle)) }
}
#[inline]
pub unsafe fn ControlTraceA<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn ControlTraceA(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES, controlcode : EVENT_TRACE_CONTROL) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { ControlTraceA(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _, controlcode) }
}
#[inline]
pub unsafe fn ControlTraceW<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn ControlTraceW(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES, controlcode : EVENT_TRACE_CONTROL) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { ControlTraceW(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _, controlcode) }
}
#[inline]
pub unsafe fn CreateTraceInstanceId(reghandle: super::super::super::Foundation::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn CreateTraceInstanceId(reghandle : super::super::super::Foundation:: HANDLE, instinfo : *mut EVENT_INSTANCE_INFO) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { CreateTraceInstanceId(reghandle, instinfo as _) }
}
#[inline]
pub unsafe fn CveEventWrite<P0, P1>(cveid: P0, additionaldetails: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CveEventWrite(cveid : windows_core::PCWSTR, additionaldetails : windows_core::PCWSTR) -> i32);
    unsafe { CveEventWrite(cveid.param().abi(), additionaldetails.param().abi()) }
}
#[inline]
pub unsafe fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const windows_core::GUID, tracehandle: CONTROLTRACE_HANDLE) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn EnableTrace(enable : u32, enableflag : u32, enablelevel : u32, controlguid : *const windows_core::GUID, tracehandle : CONTROLTRACE_HANDLE) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { EnableTrace(enable, enableflag, enablelevel, controlguid, core::mem::transmute(tracehandle)) }
}
#[inline]
pub unsafe fn EnableTraceEx(providerid: *const windows_core::GUID, sourceid: Option<*const windows_core::GUID>, tracehandle: CONTROLTRACE_HANDLE, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: Option<*const EVENT_FILTER_DESCRIPTOR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn EnableTraceEx(providerid : *const windows_core::GUID, sourceid : *const windows_core::GUID, tracehandle : CONTROLTRACE_HANDLE, isenabled : u32, level : u8, matchanykeyword : u64, matchallkeyword : u64, enableproperty : u32, enablefilterdesc : *const EVENT_FILTER_DESCRIPTOR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { EnableTraceEx(providerid, sourceid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(tracehandle), isenabled, level, matchanykeyword, matchallkeyword, enableproperty, enablefilterdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnableTraceEx2(tracehandle: CONTROLTRACE_HANDLE, providerid: *const windows_core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: Option<*const ENABLE_TRACE_PARAMETERS>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn EnableTraceEx2(tracehandle : CONTROLTRACE_HANDLE, providerid : *const windows_core::GUID, controlcode : u32, level : u8, matchanykeyword : u64, matchallkeyword : u64, timeout : u32, enableparameters : *const ENABLE_TRACE_PARAMETERS) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { EnableTraceEx2(core::mem::transmute(tracehandle), providerid, controlcode, level, matchanykeyword, matchallkeyword, timeout, enableparameters.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumerateTraceGuids(guidpropertiesarray: &mut [*mut TRACE_GUID_PROPERTIES], guidcount: *mut u32) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn EnumerateTraceGuids(guidpropertiesarray : *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount : u32, guidcount : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { EnumerateTraceGuids(core::mem::transmute(guidpropertiesarray.as_ptr()), guidpropertiesarray.len().try_into().unwrap(), guidcount as _) }
}
#[inline]
pub unsafe fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: Option<*const core::ffi::c_void>, inbuffersize: u32, outbuffer: Option<*mut core::ffi::c_void>, outbuffersize: u32, returnlength: *mut u32) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn EnumerateTraceGuidsEx(tracequeryinfoclass : TRACE_QUERY_INFO_CLASS, inbuffer : *const core::ffi::c_void, inbuffersize : u32, outbuffer : *mut core::ffi::c_void, outbuffersize : u32, returnlength : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { EnumerateTraceGuidsEx(tracequeryinfoclass, inbuffer.unwrap_or(core::mem::zeroed()) as _, inbuffersize, outbuffer.unwrap_or(core::mem::zeroed()) as _, outbuffersize, returnlength as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EventAccessControl(guid: *const windows_core::GUID, operation: u32, sid: super::super::super::Security::PSID, rights: u32, allowordeny: bool) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventAccessControl(guid : *const windows_core::GUID, operation : u32, sid : super::super::super::Security:: PSID, rights : u32, allowordeny : bool) -> u32);
    unsafe { EventAccessControl(guid, operation, sid, rights, allowordeny) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EventAccessQuery(guid: *const windows_core::GUID, buffer: Option<super::super::super::Security::PSECURITY_DESCRIPTOR>, buffersize: *mut u32) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventAccessQuery(guid : *const windows_core::GUID, buffer : super::super::super::Security:: PSECURITY_DESCRIPTOR, buffersize : *mut u32) -> u32);
    unsafe { EventAccessQuery(guid, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn EventAccessRemove(guid: *const windows_core::GUID) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventAccessRemove(guid : *const windows_core::GUID) -> u32);
    unsafe { EventAccessRemove(guid) }
}
#[inline]
pub unsafe fn EventActivityIdControl(controlcode: u32, activityid: *mut windows_core::GUID) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventActivityIdControl(controlcode : u32, activityid : *mut windows_core::GUID) -> u32);
    unsafe { EventActivityIdControl(controlcode, activityid as _) }
}
#[inline]
pub unsafe fn EventEnabled(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn EventEnabled(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR) -> bool);
    unsafe { EventEnabled(reghandle, eventdescriptor) }
}
#[inline]
pub unsafe fn EventProviderEnabled(reghandle: REGHANDLE, level: u8, keyword: u64) -> bool {
    windows_link::link!("advapi32.dll" "system" fn EventProviderEnabled(reghandle : REGHANDLE, level : u8, keyword : u64) -> bool);
    unsafe { EventProviderEnabled(reghandle, level, keyword) }
}
#[inline]
pub unsafe fn EventRegister(providerid: *const windows_core::GUID, enablecallback: PENABLECALLBACK, callbackcontext: Option<*const core::ffi::c_void>, reghandle: *mut REGHANDLE) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventRegister(providerid : *const windows_core::GUID, enablecallback : PENABLECALLBACK, callbackcontext : *const core::ffi::c_void, reghandle : *mut REGHANDLE) -> u32);
    unsafe { EventRegister(providerid, enablecallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _, reghandle as _) }
}
#[inline]
pub unsafe fn EventSetInformation(reghandle: REGHANDLE, informationclass: EVENT_INFO_CLASS, eventinformation: *const core::ffi::c_void, informationlength: u32) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventSetInformation(reghandle : REGHANDLE, informationclass : EVENT_INFO_CLASS, eventinformation : *const core::ffi::c_void, informationlength : u32) -> u32);
    unsafe { EventSetInformation(reghandle, informationclass, eventinformation, informationlength) }
}
#[inline]
pub unsafe fn EventUnregister(reghandle: REGHANDLE) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventUnregister(reghandle : REGHANDLE) -> u32);
    unsafe { EventUnregister(reghandle) }
}
#[inline]
pub unsafe fn EventWrite(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR, userdata: Option<&[EVENT_DATA_DESCRIPTOR]>) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventWrite(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
    unsafe { EventWrite(reghandle, eventdescriptor, userdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(userdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn EventWriteEx(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: Option<*const windows_core::GUID>, relatedactivityid: Option<*const windows_core::GUID>, userdata: Option<&[EVENT_DATA_DESCRIPTOR]>) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventWriteEx(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, filter : u64, flags : u32, activityid : *const windows_core::GUID, relatedactivityid : *const windows_core::GUID, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
    unsafe { EventWriteEx(reghandle, eventdescriptor, filter, flags, activityid.unwrap_or(core::mem::zeroed()) as _, relatedactivityid.unwrap_or(core::mem::zeroed()) as _, userdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(userdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn EventWriteString<P3>(reghandle: REGHANDLE, level: u8, keyword: u64, string: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn EventWriteString(reghandle : REGHANDLE, level : u8, keyword : u64, string : windows_core::PCWSTR) -> u32);
    unsafe { EventWriteString(reghandle, level, keyword, string.param().abi()) }
}
#[inline]
pub unsafe fn EventWriteTransfer(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: Option<*const windows_core::GUID>, relatedactivityid: Option<*const windows_core::GUID>, userdata: Option<&[EVENT_DATA_DESCRIPTOR]>) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn EventWriteTransfer(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, activityid : *const windows_core::GUID, relatedactivityid : *const windows_core::GUID, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
    unsafe { EventWriteTransfer(reghandle, eventdescriptor, activityid.unwrap_or(core::mem::zeroed()) as _, relatedactivityid.unwrap_or(core::mem::zeroed()) as _, userdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(userdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn FlushTraceA<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn FlushTraceA(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { FlushTraceA(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn FlushTraceW<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn FlushTraceW(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { FlushTraceW(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn GetTraceEnableFlags(tracehandle: u64) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn GetTraceEnableFlags(tracehandle : u64) -> u32);
    unsafe { GetTraceEnableFlags(tracehandle) }
}
#[inline]
pub unsafe fn GetTraceEnableLevel(tracehandle: u64) -> u8 {
    windows_link::link!("advapi32.dll" "system" fn GetTraceEnableLevel(tracehandle : u64) -> u8);
    unsafe { GetTraceEnableLevel(tracehandle) }
}
#[inline]
pub unsafe fn GetTraceLoggerHandle(buffer: *const core::ffi::c_void) -> u64 {
    windows_link::link!("advapi32.dll" "system" fn GetTraceLoggerHandle(buffer : *const core::ffi::c_void) -> u64);
    unsafe { GetTraceLoggerHandle(buffer) }
}
#[cfg(feature = "Win32_System_Time")]
#[inline]
pub unsafe fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> PROCESSTRACE_HANDLE {
    windows_link::link!("advapi32.dll" "system" fn OpenTraceA(logfile : *mut EVENT_TRACE_LOGFILEA) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceA(logfile as _) }
}
#[cfg(feature = "Win32_System_Time")]
#[inline]
pub unsafe fn OpenTraceFromBufferStream(options: *const ETW_OPEN_TRACE_OPTIONS, buffercompletioncallback: PETW_BUFFER_COMPLETION_CALLBACK, buffercompletioncontext: Option<*const core::ffi::c_void>) -> u64 {
    windows_link::link!("advapi32.dll" "system" fn OpenTraceFromBufferStream(options : *const ETW_OPEN_TRACE_OPTIONS, buffercompletioncallback : PETW_BUFFER_COMPLETION_CALLBACK, buffercompletioncontext : *const core::ffi::c_void) -> u64);
    unsafe { OpenTraceFromBufferStream(options, buffercompletioncallback, buffercompletioncontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Time")]
#[inline]
pub unsafe fn OpenTraceFromFile<P0>(logfilename: P0, options: *const ETW_OPEN_TRACE_OPTIONS, logfileheader: Option<*mut TRACE_LOGFILE_HEADER>) -> u64
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn OpenTraceFromFile(logfilename : windows_core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, logfileheader : *mut TRACE_LOGFILE_HEADER) -> u64);
    unsafe { OpenTraceFromFile(logfilename.param().abi(), options, logfileheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Time")]
#[inline]
pub unsafe fn OpenTraceFromRealTimeLogger<P0>(loggername: P0, options: *const ETW_OPEN_TRACE_OPTIONS, logfileheader: Option<*mut TRACE_LOGFILE_HEADER>) -> u64
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn OpenTraceFromRealTimeLogger(loggername : windows_core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, logfileheader : *mut TRACE_LOGFILE_HEADER) -> u64);
    unsafe { OpenTraceFromRealTimeLogger(loggername.param().abi(), options, logfileheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Time")]
#[inline]
pub unsafe fn OpenTraceFromRealTimeLoggerWithAllocationOptions<P0>(loggername: P0, options: *const ETW_OPEN_TRACE_OPTIONS, allocationsize: usize, memorypartitionhandle: Option<super::super::super::Foundation::HANDLE>, logfileheader: Option<*mut TRACE_LOGFILE_HEADER>) -> u64
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn OpenTraceFromRealTimeLoggerWithAllocationOptions(loggername : windows_core::PCWSTR, options : *const ETW_OPEN_TRACE_OPTIONS, allocationsize : usize, memorypartitionhandle : super::super::super::Foundation:: HANDLE, logfileheader : *mut TRACE_LOGFILE_HEADER) -> u64);
    unsafe { OpenTraceFromRealTimeLoggerWithAllocationOptions(loggername.param().abi(), options, allocationsize, memorypartitionhandle.unwrap_or(core::mem::zeroed()) as _, logfileheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Time")]
#[inline]
pub unsafe fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE {
    windows_link::link!("advapi32.dll" "system" fn OpenTraceW(logfile : *mut EVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceW(logfile as _) }
}
#[inline]
pub unsafe fn ProcessTrace(handlearray: &[PROCESSTRACE_HANDLE], starttime: Option<*const super::super::super::Foundation::FILETIME>, endtime: Option<*const super::super::super::Foundation::FILETIME>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn ProcessTrace(handlearray : *const PROCESSTRACE_HANDLE, handlecount : u32, starttime : *const super::super::super::Foundation:: FILETIME, endtime : *const super::super::super::Foundation:: FILETIME) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { ProcessTrace(core::mem::transmute(handlearray.as_ptr()), handlearray.len().try_into().unwrap(), starttime.unwrap_or(core::mem::zeroed()) as _, endtime.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ProcessTraceAddBufferToBufferStream(tracehandle: u64, buffer: *const ETW_BUFFER_HEADER, buffersize: u32) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn ProcessTraceAddBufferToBufferStream(tracehandle : u64, buffer : *const ETW_BUFFER_HEADER, buffersize : u32) -> u32);
    unsafe { ProcessTraceAddBufferToBufferStream(tracehandle, buffer, buffersize) }
}
#[inline]
pub unsafe fn ProcessTraceBufferDecrementReference(buffer: *const ETW_BUFFER_HEADER) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn ProcessTraceBufferDecrementReference(buffer : *const ETW_BUFFER_HEADER) -> u32);
    unsafe { ProcessTraceBufferDecrementReference(buffer) }
}
#[inline]
pub unsafe fn ProcessTraceBufferIncrementReference(tracehandle: u64, buffer: *const ETW_BUFFER_HEADER) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn ProcessTraceBufferIncrementReference(tracehandle : u64, buffer : *const ETW_BUFFER_HEADER) -> u32);
    unsafe { ProcessTraceBufferIncrementReference(tracehandle, buffer) }
}
#[inline]
pub unsafe fn QueryAllTracesA(propertyarray: &mut [*mut EVENT_TRACE_PROPERTIES], loggercount: *mut u32) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn QueryAllTracesA(propertyarray : *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount : u32, loggercount : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { QueryAllTracesA(core::mem::transmute(propertyarray.as_ptr()), propertyarray.len().try_into().unwrap(), loggercount as _) }
}
#[inline]
pub unsafe fn QueryAllTracesW(propertyarray: &mut [*mut EVENT_TRACE_PROPERTIES], loggercount: *mut u32) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn QueryAllTracesW(propertyarray : *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount : u32, loggercount : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { QueryAllTracesW(core::mem::transmute(propertyarray.as_ptr()), propertyarray.len().try_into().unwrap(), loggercount as _) }
}
#[inline]
pub unsafe fn QueryTraceA<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn QueryTraceA(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { QueryTraceA(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn QueryTraceProcessingHandle(processinghandle: PROCESSTRACE_HANDLE, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: Option<*const core::ffi::c_void>, inbuffersize: u32, outbuffer: Option<*mut core::ffi::c_void>, outbuffersize: u32, returnlength: *mut u32) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn QueryTraceProcessingHandle(processinghandle : PROCESSTRACE_HANDLE, informationclass : ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer : *const core::ffi::c_void, inbuffersize : u32, outbuffer : *mut core::ffi::c_void, outbuffersize : u32, returnlength : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { QueryTraceProcessingHandle(core::mem::transmute(processinghandle), informationclass, inbuffer.unwrap_or(core::mem::zeroed()) as _, inbuffersize, outbuffer.unwrap_or(core::mem::zeroed()) as _, outbuffersize, returnlength as _) }
}
#[inline]
pub unsafe fn QueryTraceW<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn QueryTraceW(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { QueryTraceW(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn RegisterTraceGuidsA<P5, P6>(requestaddress: WMIDPREQUEST, requestcontext: Option<*const core::ffi::c_void>, controlguid: *const windows_core::GUID, traceguidreg: Option<&[TRACE_GUID_REGISTRATION]>, mofimagepath: P5, mofresourcename: P6, registrationhandle: *mut u64) -> u32
where
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn RegisterTraceGuidsA(requestaddress : WMIDPREQUEST, requestcontext : *const core::ffi::c_void, controlguid : *const windows_core::GUID, guidcount : u32, traceguidreg : *const TRACE_GUID_REGISTRATION, mofimagepath : windows_core::PCSTR, mofresourcename : windows_core::PCSTR, registrationhandle : *mut u64) -> u32);
    unsafe { RegisterTraceGuidsA(requestaddress, requestcontext.unwrap_or(core::mem::zeroed()) as _, controlguid, traceguidreg.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(traceguidreg.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), mofimagepath.param().abi(), mofresourcename.param().abi(), registrationhandle as _) }
}
#[inline]
pub unsafe fn RegisterTraceGuidsW<P5, P6>(requestaddress: WMIDPREQUEST, requestcontext: Option<*const core::ffi::c_void>, controlguid: *const windows_core::GUID, traceguidreg: Option<&[TRACE_GUID_REGISTRATION]>, mofimagepath: P5, mofresourcename: P6, registrationhandle: *mut u64) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn RegisterTraceGuidsW(requestaddress : WMIDPREQUEST, requestcontext : *const core::ffi::c_void, controlguid : *const windows_core::GUID, guidcount : u32, traceguidreg : *const TRACE_GUID_REGISTRATION, mofimagepath : windows_core::PCWSTR, mofresourcename : windows_core::PCWSTR, registrationhandle : *mut u64) -> u32);
    unsafe { RegisterTraceGuidsW(requestaddress, requestcontext.unwrap_or(core::mem::zeroed()) as _, controlguid, traceguidreg.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(traceguidreg.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), mofimagepath.param().abi(), mofresourcename.param().abi(), registrationhandle as _) }
}
#[inline]
pub unsafe fn RemoveTraceCallback(pguid: *const windows_core::GUID) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn RemoveTraceCallback(pguid : *const windows_core::GUID) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { RemoveTraceCallback(pguid) }
}
#[inline]
pub unsafe fn SetTraceCallback(pguid: *const windows_core::GUID, eventcallback: PEVENT_CALLBACK) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn SetTraceCallback(pguid : *const windows_core::GUID, eventcallback : PEVENT_CALLBACK) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { SetTraceCallback(pguid, eventcallback) }
}
#[inline]
pub unsafe fn StartTraceA<P1>(tracehandle: *mut CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn StartTraceA(tracehandle : *mut CONTROLTRACE_HANDLE, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { StartTraceA(tracehandle as _, instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn StartTraceW<P1>(tracehandle: *mut CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn StartTraceW(tracehandle : *mut CONTROLTRACE_HANDLE, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { StartTraceW(tracehandle as _, instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn StopTraceA<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn StopTraceA(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { StopTraceA(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn StopTraceW<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn StopTraceW(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { StopTraceW(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const core::ffi::c_void, eventmatchallflags: Option<*const bool>, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhAggregatePayloadFilters(payloadfiltercount : u32, payloadfilterptrs : *const *const core::ffi::c_void, eventmatchallflags : *const bool, eventfilterdescriptor : *mut EVENT_FILTER_DESCRIPTOR) -> u32);
    unsafe { TdhAggregatePayloadFilters(payloadfiltercount, payloadfilterptrs, eventmatchallflags.unwrap_or(core::mem::zeroed()) as _, eventfilterdescriptor as _) }
}
#[inline]
pub unsafe fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor : *mut EVENT_FILTER_DESCRIPTOR) -> u32);
    unsafe { TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor as _) }
}
#[inline]
pub unsafe fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhCloseDecodingHandle(handle : TDH_HANDLE) -> u32);
    unsafe { TdhCloseDecodingHandle(handle) }
}
#[inline]
pub unsafe fn TdhCreatePayloadFilter(providerguid: *const windows_core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: bool, payloadpredicates: &[PAYLOAD_FILTER_PREDICATE], payloadfilter: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhCreatePayloadFilter(providerguid : *const windows_core::GUID, eventdescriptor : *const EVENT_DESCRIPTOR, eventmatchany : bool, payloadpredicatecount : u32, payloadpredicates : *const PAYLOAD_FILTER_PREDICATE, payloadfilter : *mut *mut core::ffi::c_void) -> u32);
    unsafe { TdhCreatePayloadFilter(providerguid, eventdescriptor, eventmatchany, payloadpredicates.len().try_into().unwrap(), core::mem::transmute(payloadpredicates.as_ptr()), payloadfilter as _) }
}
#[inline]
pub unsafe fn TdhDeletePayloadFilter(payloadfilter: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhDeletePayloadFilter(payloadfilter : *mut *mut core::ffi::c_void) -> u32);
    unsafe { TdhDeletePayloadFilter(payloadfilter as _) }
}
#[inline]
pub unsafe fn TdhEnumerateManifestProviderEvents(providerguid: *const windows_core::GUID, buffer: Option<*mut PROVIDER_EVENT_INFO>, buffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhEnumerateManifestProviderEvents(providerguid : *const windows_core::GUID, buffer : *mut PROVIDER_EVENT_INFO, buffersize : *mut u32) -> u32);
    unsafe { TdhEnumerateManifestProviderEvents(providerguid, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProviderFieldInformation(pguid: *const windows_core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: Option<*mut PROVIDER_FIELD_INFOARRAY>, pbuffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhEnumerateProviderFieldInformation(pguid : *const windows_core::GUID, eventfieldtype : EVENT_FIELD_TYPE, pbuffer : *mut PROVIDER_FIELD_INFOARRAY, pbuffersize : *mut u32) -> u32);
    unsafe { TdhEnumerateProviderFieldInformation(pguid, eventfieldtype, pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProviderFilters(guid: *const windows_core::GUID, tdhcontext: Option<&[TDH_CONTEXT]>, filtercount: *mut u32, buffer: Option<*mut *mut PROVIDER_FILTER_INFO>, buffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhEnumerateProviderFilters(guid : *const windows_core::GUID, tdhcontextcount : u32, tdhcontext : *const TDH_CONTEXT, filtercount : *mut u32, buffer : *mut *mut PROVIDER_FILTER_INFO, buffersize : *mut u32) -> u32);
    unsafe { TdhEnumerateProviderFilters(guid, tdhcontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(tdhcontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), filtercount as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProviders(pbuffer: Option<*mut PROVIDER_ENUMERATION_INFO>, pbuffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhEnumerateProviders(pbuffer : *mut PROVIDER_ENUMERATION_INFO, pbuffersize : *mut u32) -> u32);
    unsafe { TdhEnumerateProviders(pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: Option<*mut PROVIDER_ENUMERATION_INFO>, buffersize: u32, bufferrequired: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhEnumerateProvidersForDecodingSource(filter : DECODING_SOURCE, buffer : *mut PROVIDER_ENUMERATION_INFO, buffersize : u32, bufferrequired : *mut u32) -> u32);
    unsafe { TdhEnumerateProvidersForDecodingSource(filter, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, bufferrequired as _) }
}
#[inline]
pub unsafe fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: Option<*const EVENT_MAP_INFO>, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdata: &[u8], buffersize: *mut u32, buffer: Option<windows_core::PWSTR>, userdataconsumed: *mut u16) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhFormatProperty(eventinfo : *const TRACE_EVENT_INFO, mapinfo : *const EVENT_MAP_INFO, pointersize : u32, propertyintype : u16, propertyouttype : u16, propertylength : u16, userdatalength : u16, userdata : *const u8, buffersize : *mut u32, buffer : windows_core::PWSTR, userdataconsumed : *mut u16) -> u32);
    unsafe { TdhFormatProperty(eventinfo, mapinfo.unwrap_or(core::mem::zeroed()) as _, pointersize, propertyintype, propertyouttype, propertylength, userdata.len().try_into().unwrap(), core::mem::transmute(userdata.as_ptr()), buffersize as _, buffer.unwrap_or(core::mem::zeroed()) as _, userdataconsumed as _) }
}
#[inline]
pub unsafe fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhGetDecodingParameter(handle : TDH_HANDLE, tdhcontext : *mut TDH_CONTEXT) -> u32);
    unsafe { TdhGetDecodingParameter(handle, tdhcontext as _) }
}
#[inline]
pub unsafe fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontext: Option<&[TDH_CONTEXT]>, buffer: Option<*mut TRACE_EVENT_INFO>, buffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhGetEventInformation(event : *const EVENT_RECORD, tdhcontextcount : u32, tdhcontext : *const TDH_CONTEXT, buffer : *mut TRACE_EVENT_INFO, buffersize : *mut u32) -> u32);
    unsafe { TdhGetEventInformation(event, tdhcontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(tdhcontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn TdhGetEventMapInformation<P1>(pevent: *const EVENT_RECORD, pmapname: P1, pbuffer: Option<*mut EVENT_MAP_INFO>, pbuffersize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tdh.dll" "system" fn TdhGetEventMapInformation(pevent : *const EVENT_RECORD, pmapname : windows_core::PCWSTR, pbuffer : *mut EVENT_MAP_INFO, pbuffersize : *mut u32) -> u32);
    unsafe { TdhGetEventMapInformation(pevent, pmapname.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn TdhGetManifestEventInformation(providerguid: *const windows_core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: Option<*mut TRACE_EVENT_INFO>, buffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhGetManifestEventInformation(providerguid : *const windows_core::GUID, eventdescriptor : *const EVENT_DESCRIPTOR, buffer : *mut TRACE_EVENT_INFO, buffersize : *mut u32) -> u32);
    unsafe { TdhGetManifestEventInformation(providerguid, eventdescriptor, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn TdhGetProperty(pevent: *const EVENT_RECORD, ptdhcontext: Option<&[TDH_CONTEXT]>, ppropertydata: &[PROPERTY_DATA_DESCRIPTOR], pbuffer: &mut [u8]) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhGetProperty(pevent : *const EVENT_RECORD, tdhcontextcount : u32, ptdhcontext : *const TDH_CONTEXT, propertydatacount : u32, ppropertydata : *const PROPERTY_DATA_DESCRIPTOR, buffersize : u32, pbuffer : *mut u8) -> u32);
    unsafe { TdhGetProperty(pevent, ptdhcontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ptdhcontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppropertydata.len().try_into().unwrap(), core::mem::transmute(ppropertydata.as_ptr()), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr())) }
}
#[inline]
pub unsafe fn TdhGetPropertySize(pevent: *const EVENT_RECORD, ptdhcontext: Option<&[TDH_CONTEXT]>, ppropertydata: &[PROPERTY_DATA_DESCRIPTOR], ppropertysize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhGetPropertySize(pevent : *const EVENT_RECORD, tdhcontextcount : u32, ptdhcontext : *const TDH_CONTEXT, propertydatacount : u32, ppropertydata : *const PROPERTY_DATA_DESCRIPTOR, ppropertysize : *mut u32) -> u32);
    unsafe { TdhGetPropertySize(pevent, ptdhcontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ptdhcontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppropertydata.len().try_into().unwrap(), core::mem::transmute(ppropertydata.as_ptr()), ppropertysize as _) }
}
#[inline]
pub unsafe fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhGetWppMessage(handle : TDH_HANDLE, eventrecord : *const EVENT_RECORD, buffersize : *mut u32, buffer : *mut u8) -> u32);
    unsafe { TdhGetWppMessage(handle, eventrecord, buffersize as _, buffer as _) }
}
#[inline]
pub unsafe fn TdhGetWppProperty<P2>(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, propertyname: P2, buffersize: *mut u32, buffer: *mut u8) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tdh.dll" "system" fn TdhGetWppProperty(handle : TDH_HANDLE, eventrecord : *const EVENT_RECORD, propertyname : windows_core::PCWSTR, buffersize : *mut u32, buffer : *mut u8) -> u32);
    unsafe { TdhGetWppProperty(handle, eventrecord, propertyname.param().abi(), buffersize as _, buffer as _) }
}
#[inline]
pub unsafe fn TdhLoadManifest<P0>(manifest: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tdh.dll" "system" fn TdhLoadManifest(manifest : windows_core::PCWSTR) -> u32);
    unsafe { TdhLoadManifest(manifest.param().abi()) }
}
#[inline]
pub unsafe fn TdhLoadManifestFromBinary<P0>(binarypath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tdh.dll" "system" fn TdhLoadManifestFromBinary(binarypath : windows_core::PCWSTR) -> u32);
    unsafe { TdhLoadManifestFromBinary(binarypath.param().abi()) }
}
#[inline]
pub unsafe fn TdhLoadManifestFromMemory(pdata: *const core::ffi::c_void, cbdata: u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhLoadManifestFromMemory(pdata : *const core::ffi::c_void, cbdata : u32) -> u32);
    unsafe { TdhLoadManifestFromMemory(pdata, cbdata) }
}
#[inline]
pub unsafe fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhOpenDecodingHandle(handle : *mut TDH_HANDLE) -> u32);
    unsafe { TdhOpenDecodingHandle(handle as _) }
}
#[inline]
pub unsafe fn TdhQueryProviderFieldInformation(pguid: *const windows_core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: Option<*mut PROVIDER_FIELD_INFOARRAY>, pbuffersize: *mut u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhQueryProviderFieldInformation(pguid : *const windows_core::GUID, eventfieldvalue : u64, eventfieldtype : EVENT_FIELD_TYPE, pbuffer : *mut PROVIDER_FIELD_INFOARRAY, pbuffersize : *mut u32) -> u32);
    unsafe { TdhQueryProviderFieldInformation(pguid, eventfieldvalue, eventfieldtype, pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhSetDecodingParameter(handle : TDH_HANDLE, tdhcontext : *const TDH_CONTEXT) -> u32);
    unsafe { TdhSetDecodingParameter(handle, tdhcontext) }
}
#[inline]
pub unsafe fn TdhUnloadManifest<P0>(manifest: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tdh.dll" "system" fn TdhUnloadManifest(manifest : windows_core::PCWSTR) -> u32);
    unsafe { TdhUnloadManifest(manifest.param().abi()) }
}
#[inline]
pub unsafe fn TdhUnloadManifestFromMemory(pdata: *const core::ffi::c_void, cbdata: u32) -> u32 {
    windows_link::link!("tdh.dll" "system" fn TdhUnloadManifestFromMemory(pdata : *const core::ffi::c_void, cbdata : u32) -> u32);
    unsafe { TdhUnloadManifestFromMemory(pdata, cbdata) }
}
#[inline]
pub unsafe fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn TraceEvent(tracehandle : u64, eventtrace : *const EVENT_TRACE_HEADER) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { TraceEvent(tracehandle, eventtrace) }
}
#[inline]
pub unsafe fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: Option<*const EVENT_INSTANCE_INFO>) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn TraceEventInstance(tracehandle : u64, eventtrace : *const EVENT_INSTANCE_HEADER, instinfo : *const EVENT_INSTANCE_INFO, parentinstinfo : *const EVENT_INSTANCE_INFO) -> u32);
    unsafe { TraceEventInstance(tracehandle, eventtrace, instinfo, parentinstinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const windows_core::GUID, messagenumber: u16) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "C" fn TraceMessage(loggerhandle : u64, messageflags : TRACE_MESSAGE_FLAGS, messageguid : *const windows_core::GUID, messagenumber : u16) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { TraceMessage(loggerhandle, messageflags, messageguid, messagenumber) }
}
#[inline]
pub unsafe fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const windows_core::GUID, messagenumber: u16, messagearglist: *const i8) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn TraceMessageVa(loggerhandle : u64, messageflags : TRACE_MESSAGE_FLAGS, messageguid : *const windows_core::GUID, messagenumber : u16, messagearglist : *const i8) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { TraceMessageVa(loggerhandle, messageflags, messageguid, messagenumber, messagearglist) }
}
#[inline]
pub unsafe fn TraceQueryInformation(sessionhandle: CONTROLTRACE_HANDLE, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut core::ffi::c_void, informationlength: u32, returnlength: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn TraceQueryInformation(sessionhandle : CONTROLTRACE_HANDLE, informationclass : TRACE_QUERY_INFO_CLASS, traceinformation : *mut core::ffi::c_void, informationlength : u32, returnlength : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { TraceQueryInformation(core::mem::transmute(sessionhandle), informationclass, traceinformation as _, informationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TraceSetInformation(sessionhandle: CONTROLTRACE_HANDLE, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const core::ffi::c_void, informationlength: u32) -> super::super::super::Foundation::WIN32_ERROR {
    windows_link::link!("advapi32.dll" "system" fn TraceSetInformation(sessionhandle : CONTROLTRACE_HANDLE, informationclass : TRACE_QUERY_INFO_CLASS, traceinformation : *const core::ffi::c_void, informationlength : u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { TraceSetInformation(core::mem::transmute(sessionhandle), informationclass, traceinformation, informationlength) }
}
#[inline]
pub unsafe fn UnregisterTraceGuids(registrationhandle: u64) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn UnregisterTraceGuids(registrationhandle : u64) -> u32);
    unsafe { UnregisterTraceGuids(registrationhandle) }
}
#[inline]
pub unsafe fn UpdateTraceA<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn UpdateTraceA(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { UpdateTraceA(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn UpdateTraceW<P1>(tracehandle: CONTROLTRACE_HANDLE, instancename: P1, properties: *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn UpdateTraceW(tracehandle : CONTROLTRACE_HANDLE, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { UpdateTraceW(core::mem::transmute(tracehandle), instancename.param().abi(), properties as _) }
}
pub const ALPCGuid: windows_core::GUID = windows_core::GUID::from_u128(0x45d8cccd_539f_4b72_a8b7_5c683142609a);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const CLSID_TraceRelogger: windows_core::GUID = windows_core::GUID::from_u128(0x7b40792d_05ff_44c4_9058_f440c71f17d4);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONTROLTRACE_HANDLE {
    pub Value: u64,
}
pub const CTraceRelogger: windows_core::GUID = windows_core::GUID::from_u128(0x7b40792d_05ff_44c4_9058_f440c71f17d4);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECODING_SOURCE(pub i32);
pub const DIAG_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("DiagLog");
pub const DIAG_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("DiagLog");
pub const DecodingSourceMax: DECODING_SOURCE = DECODING_SOURCE(4i32);
pub const DecodingSourceTlg: DECODING_SOURCE = DECODING_SOURCE(3i32);
pub const DecodingSourceWPP: DECODING_SOURCE = DECODING_SOURCE(2i32);
pub const DecodingSourceWbem: DECODING_SOURCE = DECODING_SOURCE(1i32);
pub const DecodingSourceXMLFile: DECODING_SOURCE = DECODING_SOURCE(0i32);
pub const DefaultTraceSecurityGuid: windows_core::GUID = windows_core::GUID::from_u128(0x0811c1af_7a07_4a06_82ed_869455cdf713);
pub const DiskIoGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3d6fa8d4_fe05_11d0_9dda_00c04fd7ba7c);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENABLECALLBACK_ENABLED_STATE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: windows_core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
impl Default for ENABLE_TRACE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: windows_core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
}
impl Default for ENABLE_TRACE_PARAMETERS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENABLE_TRACE_PARAMETERS_VERSION: u32 = 1u32;
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: u32 = 2u32;
pub const ETW_ASCIICHAR_TYPE_VALUE: u32 = 102u32;
pub const ETW_ASCIISTRING_TYPE_VALUE: u32 = 103u32;
pub const ETW_BOOLEAN_TYPE_VALUE: u32 = 14u32;
pub const ETW_BOOL_TYPE_VALUE: u32 = 108u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ETW_BUFFER_CALLBACK_INFORMATION {
    pub TraceHandle: u64,
    pub LogfileHeader: *const TRACE_LOGFILE_HEADER,
    pub BuffersRead: u32,
}
#[cfg(feature = "Win32_System_Time")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const ETW_BYTE_TYPE_VALUE: u32 = 4u32;
pub const ETW_CHAR_TYPE_VALUE: u32 = 11u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_COMPRESSION_RESUMPTION_MODE(pub i32);
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 109u32;
pub const ETW_COUNTED_STRING_TYPE_VALUE: u32 = 104u32;
pub const ETW_DATETIME_TYPE_VALUE: u32 = 119u32;
pub const ETW_DECIMAL_TYPE_VALUE: u32 = 15u32;
pub const ETW_DOUBLE_TYPE_VALUE: u32 = 13u32;
pub const ETW_GUID_TYPE_VALUE: u32 = 101u32;
pub const ETW_HIDDEN_TYPE_VALUE: u32 = 107u32;
pub const ETW_INT16_TYPE_VALUE: u32 = 5u32;
pub const ETW_INT32_TYPE_VALUE: u32 = 7u32;
pub const ETW_INT64_TYPE_VALUE: u32 = 9u32;
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: u32 = 112u32;
pub const ETW_NULL_TYPE_VALUE: u32 = 0u32;
pub const ETW_OBJECT_TYPE_VALUE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug)]
pub struct ETW_OPEN_TRACE_OPTIONS {
    pub ProcessTraceModes: ETW_PROCESS_TRACE_MODES,
    pub EventCallback: PEVENT_RECORD_CALLBACK,
    pub EventCallbackContext: *mut core::ffi::c_void,
    pub BufferCallback: PETW_BUFFER_CALLBACK,
    pub BufferCallbackContext: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for ETW_OPEN_TRACE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: u32,
    pub OwnerTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_PMC_COUNTER_OWNER_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ETW_PMC_SESSION_INFO {
    pub NextEntryOffset: u32,
    pub LoggerId: u16,
    pub Reserved: u16,
    pub ProfileSourceCount: u32,
    pub HookIdCount: u32,
}
pub const ETW_POINTER_TYPE_VALUE: u32 = 105u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_PROCESS_HANDLE_INFO_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_PROCESS_TRACE_MODES(pub i32);
pub const ETW_PROCESS_TRACE_MODE_NONE: ETW_PROCESS_TRACE_MODES = ETW_PROCESS_TRACE_MODES(0i32);
pub const ETW_PROCESS_TRACE_MODE_RAW_TIMESTAMP: ETW_PROCESS_TRACE_MODES = ETW_PROCESS_TRACE_MODES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_PROVIDER_TRAIT_TYPE(pub i32);
pub const ETW_PTVECTOR_TYPE_VALUE: u32 = 117u32;
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: u32 = 113u32;
pub const ETW_REDUCED_STRING_TYPE_VALUE: u32 = 114u32;
pub const ETW_REFRENCE_TYPE_VALUE: u32 = 120u32;
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 111u32;
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: u32 = 110u32;
pub const ETW_SBYTE_TYPE_VALUE: u32 = 3u32;
pub const ETW_SID_TYPE_VALUE: u32 = 115u32;
pub const ETW_SINGLE_TYPE_VALUE: u32 = 12u32;
pub const ETW_SIZET_TYPE_VALUE: u32 = 106u32;
pub const ETW_STRING_TYPE_VALUE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: windows_core::GUID,
    pub ParentId: windows_core::GUID,
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
    pub PartitionId: windows_core::PWSTR,
    pub ParentId: windows_core::PWSTR,
}
pub const ETW_UINT16_TYPE_VALUE: u32 = 6u32;
pub const ETW_UINT32_TYPE_VALUE: u32 = 8u32;
pub const ETW_UINT64_TYPE_VALUE: u32 = 10u32;
pub const ETW_VARIANT_TYPE_VALUE: u32 = 116u32;
pub const ETW_WMITIME_TYPE_VALUE: u32 = 118u32;
pub const EVENTMAP_ENTRY_VALUETYPE_STRING: MAP_VALUETYPE = MAP_VALUETYPE(1i32);
pub const EVENTMAP_ENTRY_VALUETYPE_ULONG: MAP_VALUETYPE = MAP_VALUETYPE(0i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_BITMAP: MAP_FLAGS = MAP_FLAGS(2i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP: MAP_FLAGS = MAP_FLAGS(4i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP: MAP_FLAGS = MAP_FLAGS(1i32);
pub const EVENTMAP_INFO_FLAG_WBEM_BITMAP: MAP_FLAGS = MAP_FLAGS(16i32);
pub const EVENTMAP_INFO_FLAG_WBEM_FLAG: MAP_FLAGS = MAP_FLAGS(32i32);
pub const EVENTMAP_INFO_FLAG_WBEM_NO_MAP: MAP_FLAGS = MAP_FLAGS(64i32);
pub const EVENTMAP_INFO_FLAG_WBEM_VALUEMAP: MAP_FLAGS = MAP_FLAGS(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENTSECURITYOPERATION(pub i32);
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: u32 = 3u32;
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: u32 = 5u32;
pub const EVENT_ACTIVITY_CTRL_GET_ID: u32 = 1u32;
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: u32 = 4u32;
pub const EVENT_ACTIVITY_CTRL_SET_ID: u32 = 2u32;
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(2u32);
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(0u32);
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(1u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
impl Default for EVENT_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
impl Default for EVENT_DATA_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: u32 = 1u32;
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: u32 = 0u32;
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: u32 = 2u32;
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: u32 = 64u32;
pub const EVENT_ENABLE_PROPERTY_ENABLE_SILOS: u32 = 1024u32;
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: u32 = 256u32;
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: u32 = 512u32;
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: u32 = 16u32;
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: u32 = 128u32;
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: u32 = 32u32;
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: u32 = 8u32;
pub const EVENT_ENABLE_PROPERTY_SID: u32 = 1u32;
pub const EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING: u32 = 2048u32;
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: u32 = 4u32;
pub const EVENT_ENABLE_PROPERTY_TS_ID: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    pub Key: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_INSTANCE {
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    pub PebsIndex: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    pub Counter: [u64; 1],
}
impl Default for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    pub ProcessStartKey: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    pub RelatedActivityId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY32 {
    pub MatchId: u64,
    pub StackKey: u32,
    pub Padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY64 {
    pub MatchId: u64,
    pub StackKey: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    pub MatchId: u64,
    pub Address: [u32; 1],
}
impl Default for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    pub MatchId: u64,
    pub Address: [u64; 1],
}
impl Default for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_TS_ID {
    pub SessionId: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_FIELD_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: bool,
    pub Reserved: u8,
    pub Count: u16,
    pub Events: [u16; 1],
}
impl Default for EVENT_FILTER_EVENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: bool,
    pub NameCount: u16,
    pub Names: [u8; 1],
}
impl Default for EVENT_FILTER_EVENT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_FILTER_HEADER {
    pub Id: u16,
    pub Version: u8,
    pub Reserved: [u8; 5],
    pub InstanceId: u64,
    pub Size: u32,
    pub NextOffset: u32,
}
impl Default for EVENT_FILTER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: bool,
}
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416u32;
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160u32;
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672u32;
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656u32;
pub const EVENT_FILTER_TYPE_NONE: u32 = 0u32;
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: u32 = 2147483680u32;
pub const EVENT_FILTER_TYPE_PACKAGE_ID: u32 = 2147483664u32;
pub const EVENT_FILTER_TYPE_PAYLOAD: u32 = 2147483904u32;
pub const EVENT_FILTER_TYPE_PID: u32 = 2147483652u32;
pub const EVENT_FILTER_TYPE_SCHEMATIZED: u32 = 2147483648u32;
pub const EVENT_FILTER_TYPE_STACKWALK: u32 = 2147487744u32;
pub const EVENT_FILTER_TYPE_STACKWALK_LEVEL_KW: u32 = 2147500032u32;
pub const EVENT_FILTER_TYPE_STACKWALK_NAME: u32 = 2147491840u32;
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: u32 = 2147483649u32;
pub const EVENT_FILTER_TYPE_TRACEHANDLE: u32 = 2147483650u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: windows_core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: windows_core::GUID,
}
impl Default for EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
impl Default for EVENT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
pub const EVENT_HEADER_EXT_TYPE_CONTAINER_ID: u32 = 16u32;
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: u32 = 14u32;
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: u32 = 10u32;
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: u32 = 11u32;
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: u32 = 4u32;
pub const EVENT_HEADER_EXT_TYPE_MAX: u32 = 19u32;
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: u32 = 7u32;
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: u32 = 8u32;
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: u32 = 13u32;
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: u32 = 12u32;
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: u32 = 9u32;
pub const EVENT_HEADER_EXT_TYPE_QPC_DELTA: u32 = 15u32;
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: u32 = 1u32;
pub const EVENT_HEADER_EXT_TYPE_SID: u32 = 2u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY32: u32 = 17u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY64: u32 = 18u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: u32 = 5u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: u32 = 6u32;
pub const EVENT_HEADER_EXT_TYPE_TS_ID: u32 = 3u32;
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: u32 = 32u32;
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: u32 = 64u32;
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: u32 = 256u32;
pub const EVENT_HEADER_FLAG_DECODE_GUID: u32 = 128u32;
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: u32 = 1u32;
pub const EVENT_HEADER_FLAG_NO_CPUTIME: u32 = 16u32;
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: u32 = 2u32;
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: u32 = 512u32;
pub const EVENT_HEADER_FLAG_STRING_ONLY: u32 = 4u32;
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: u32 = 8u32;
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: u32 = 2u32;
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: u32 = 4u32;
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: u32 = 8u32;
pub const EVENT_HEADER_PROPERTY_XML: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_INFO_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER {
    pub Size: u16,
    pub Anonymous1: EVENT_INSTANCE_HEADER_0,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous1: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
impl Default for EVENT_INSTANCE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::super::super::Foundation::HANDLE,
    pub InstanceId: u32,
}
pub const EVENT_LOGGER_NAME: windows_core::PCWSTR = windows_core::w!("EventLog");
pub const EVENT_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("EventLog");
pub const EVENT_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("EventLog");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: u32,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
impl Default for EVENT_MAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: u32,
    pub InputOffset: u32,
}
impl Default for EVENT_MAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_MAP_INFO {
    pub NameOffset: u32,
    pub Flag: MAP_FLAGS,
    pub EntryCount: u32,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
impl Default for EVENT_MAP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: u32,
}
impl Default for EVENT_MAP_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_MAX_LEVEL: u32 = 255u32;
pub const EVENT_MIN_LEVEL: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous1: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_0,
    pub structType: EVENT_PROPERTY_INFO_0_1,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_2,
}
impl Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: *mut EVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut core::ffi::c_void,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648u32;
pub const EVENT_TRACE_ADD_HEADER_MODE: u32 = 4096u32;
pub const EVENT_TRACE_BUFFERING_MODE: u32 = 1024u32;
pub const EVENT_TRACE_COMPRESSED_MODE: u32 = 67108864u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_CONTROL(pub u32);
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: u32 = 5u32;
pub const EVENT_TRACE_CONTROL_FLUSH: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(3u32);
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: u32 = 4u32;
pub const EVENT_TRACE_CONTROL_QUERY: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(0u32);
pub const EVENT_TRACE_CONTROL_STOP: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(1u32);
pub const EVENT_TRACE_CONTROL_UPDATE: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(2u32);
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: u32 = 512u32;
pub const EVENT_TRACE_FILE_MODE_APPEND: u32 = 4u32;
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: u32 = 2u32;
pub const EVENT_TRACE_FILE_MODE_NEWFILE: u32 = 8u32;
pub const EVENT_TRACE_FILE_MODE_NONE: u32 = 0u32;
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: u32 = 32u32;
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_FLAG(pub u32);
impl EVENT_TRACE_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EVENT_TRACE_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EVENT_TRACE_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EVENT_TRACE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const EVENT_TRACE_FLAG_ALPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1048576u32);
pub const EVENT_TRACE_FLAG_CSWITCH: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16u32);
pub const EVENT_TRACE_FLAG_DBGPRINT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(262144u32);
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: u32 = 4194304u32;
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(512u32);
pub const EVENT_TRACE_FLAG_DISK_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(256u32);
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1024u32);
pub const EVENT_TRACE_FLAG_DISPATCHER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2048u32);
pub const EVENT_TRACE_FLAG_DPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32u32);
pub const EVENT_TRACE_FLAG_DRIVER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8388608u32);
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: u32 = 536870912u32;
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648u32;
pub const EVENT_TRACE_FLAG_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(33554432u32);
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(67108864u32);
pub const EVENT_TRACE_FLAG_FORWARD_WMI: u32 = 1073741824u32;
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4u32);
pub const EVENT_TRACE_FLAG_INTERRUPT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(64u32);
pub const EVENT_TRACE_FLAG_JOB: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(524288u32);
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8192u32);
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4096u32);
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(65536u32);
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(268435456u32);
pub const EVENT_TRACE_FLAG_PROCESS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1u32);
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8u32);
pub const EVENT_TRACE_FLAG_PROFILE: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16777216u32);
pub const EVENT_TRACE_FLAG_REGISTRY: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(131072u32);
pub const EVENT_TRACE_FLAG_SPLIT_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2097152u32);
pub const EVENT_TRACE_FLAG_SYSTEMCALL: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(128u32);
pub const EVENT_TRACE_FLAG_THREAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2u32);
pub const EVENT_TRACE_FLAG_VAMAP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32768u32);
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16384u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER {
    pub Size: u16,
    pub Anonymous1: EVENT_TRACE_HEADER_0,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub Anonymous1: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: u32 = 134217728u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_LOGFILEA {
    pub LogFileName: windows_core::PSTR,
    pub LoggerName: windows_core::PSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEA_0,
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
#[cfg(feature = "Win32_System_Time")]
impl Default for EVENT_TRACE_LOGFILEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for EVENT_TRACE_LOGFILEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for EVENT_TRACE_LOGFILEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: windows_core::PWSTR,
    pub LoggerName: windows_core::PWSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEW_0,
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
#[cfg(feature = "Win32_System_Time")]
impl Default for EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_MODE_RESERVED: u32 = 1048576u32;
pub const EVENT_TRACE_NONSTOPPABLE_MODE: u32 = 64u32;
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: u32 = 268435456u32;
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: u32 = 8388608u32;
pub const EVENT_TRACE_PRIVATE_IN_PROC: u32 = 131072u32;
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: u32 = 2048u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: EVENT_TRACE_FLAG,
    pub Anonymous: EVENT_TRACE_PROPERTIES_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::super::super::Foundation::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
}
impl Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
impl Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES_V2 {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: EVENT_TRACE_FLAG,
    pub Anonymous1: EVENT_TRACE_PROPERTIES_V2_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::super::super::Foundation::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
    pub Anonymous2: EVENT_TRACE_PROPERTIES_V2_1,
    pub FilterDescCount: u32,
    pub FilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub Anonymous3: EVENT_TRACE_PROPERTIES_V2_2,
}
impl Default for EVENT_TRACE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
impl Default for EVENT_TRACE_PROPERTIES_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: u32,
}
impl Default for EVENT_TRACE_PROPERTIES_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: u64,
}
impl Default for EVENT_TRACE_PROPERTIES_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
pub const EVENT_TRACE_REAL_TIME_MODE: u32 = 256u32;
pub const EVENT_TRACE_RELOG_MODE: u32 = 65536u32;
pub const EVENT_TRACE_SECURE_MODE: u32 = 128u32;
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: u32 = 4194304u32;
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: u32 = 33554432u32;
pub const EVENT_TRACE_TYPE_ACCEPT: u32 = 15u32;
pub const EVENT_TRACE_TYPE_ACKDUP: u32 = 22u32;
pub const EVENT_TRACE_TYPE_ACKFULL: u32 = 20u32;
pub const EVENT_TRACE_TYPE_ACKPART: u32 = 21u32;
pub const EVENT_TRACE_TYPE_CHECKPOINT: u32 = 8u32;
pub const EVENT_TRACE_TYPE_CONFIG: u32 = 11u32;
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: u32 = 37u32;
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: u32 = 29u32;
pub const EVENT_TRACE_TYPE_CONFIG_CPU: u32 = 10u32;
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: u32 = 31u32;
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: u32 = 33u32;
pub const EVENT_TRACE_TYPE_CONFIG_DPI: u32 = 28u32;
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: u32 = 34u32;
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: u32 = 23u32;
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: u32 = 21u32;
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: u32 = 12u32;
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: u32 = 30u32;
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: u32 = 32u32;
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: u32 = 17u32;
pub const EVENT_TRACE_TYPE_CONFIG_NIC: u32 = 13u32;
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: u32 = 24u32;
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: u32 = 18u32;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: u32 = 11u32;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK_EX: u32 = 19u32;
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: u32 = 25u32;
pub const EVENT_TRACE_TYPE_CONFIG_PNP: u32 = 22u32;
pub const EVENT_TRACE_TYPE_CONFIG_POWER: u32 = 16u32;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: u32 = 35u32;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: u32 = 26u32;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: u32 = 27u32;
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: u32 = 15u32;
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: u32 = 14u32;
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: u32 = 36u32;
pub const EVENT_TRACE_TYPE_CONNECT: u32 = 12u32;
pub const EVENT_TRACE_TYPE_CONNFAIL: u32 = 17u32;
pub const EVENT_TRACE_TYPE_COPY_ARP: u32 = 19u32;
pub const EVENT_TRACE_TYPE_COPY_TCP: u32 = 18u32;
pub const EVENT_TRACE_TYPE_DBGID_RSDS: u32 = 64u32;
pub const EVENT_TRACE_TYPE_DC_END: u32 = 4u32;
pub const EVENT_TRACE_TYPE_DC_START: u32 = 3u32;
pub const EVENT_TRACE_TYPE_DEQUEUE: u32 = 7u32;
pub const EVENT_TRACE_TYPE_DISCONNECT: u32 = 13u32;
pub const EVENT_TRACE_TYPE_END: u32 = 2u32;
pub const EVENT_TRACE_TYPE_EXTENSION: u32 = 5u32;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: u32 = 99u32;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: u32 = 101u32;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: u32 = 97u32;
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: u32 = 98u32;
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: u32 = 100u32;
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: u32 = 96u32;
pub const EVENT_TRACE_TYPE_GUIDMAP: u32 = 10u32;
pub const EVENT_TRACE_TYPE_INFO: u32 = 0u32;
pub const EVENT_TRACE_TYPE_IO_FLUSH: u32 = 14u32;
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: u32 = 15u32;
pub const EVENT_TRACE_TYPE_IO_READ: u32 = 10u32;
pub const EVENT_TRACE_TYPE_IO_READ_INIT: u32 = 12u32;
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: u32 = 16u32;
pub const EVENT_TRACE_TYPE_IO_WRITE: u32 = 11u32;
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: u32 = 13u32;
pub const EVENT_TRACE_TYPE_LOAD: u32 = 10u32;
pub const EVENT_TRACE_TYPE_MM_AV: u32 = 15u32;
pub const EVENT_TRACE_TYPE_MM_COW: u32 = 12u32;
pub const EVENT_TRACE_TYPE_MM_DZF: u32 = 11u32;
pub const EVENT_TRACE_TYPE_MM_GPF: u32 = 13u32;
pub const EVENT_TRACE_TYPE_MM_HPF: u32 = 14u32;
pub const EVENT_TRACE_TYPE_MM_TF: u32 = 10u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: u32 = 57u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: u32 = 60u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: u32 = 55u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: u32 = 58u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: u32 = 56u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: u32 = 59u32;
pub const EVENT_TRACE_TYPE_RECEIVE: u32 = 11u32;
pub const EVENT_TRACE_TYPE_RECONNECT: u32 = 16u32;
pub const EVENT_TRACE_TYPE_REGCLOSE: u32 = 27u32;
pub const EVENT_TRACE_TYPE_REGCOMMIT: u32 = 30u32;
pub const EVENT_TRACE_TYPE_REGCREATE: u32 = 10u32;
pub const EVENT_TRACE_TYPE_REGDELETE: u32 = 12u32;
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: u32 = 15u32;
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: u32 = 17u32;
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: u32 = 18u32;
pub const EVENT_TRACE_TYPE_REGFLUSH: u32 = 21u32;
pub const EVENT_TRACE_TYPE_REGKCBCREATE: u32 = 22u32;
pub const EVENT_TRACE_TYPE_REGKCBDELETE: u32 = 23u32;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: u32 = 24u32;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: u32 = 25u32;
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: u32 = 33u32;
pub const EVENT_TRACE_TYPE_REGOPEN: u32 = 11u32;
pub const EVENT_TRACE_TYPE_REGPREPARE: u32 = 31u32;
pub const EVENT_TRACE_TYPE_REGQUERY: u32 = 13u32;
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: u32 = 19u32;
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: u32 = 29u32;
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: u32 = 16u32;
pub const EVENT_TRACE_TYPE_REGROLLBACK: u32 = 32u32;
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: u32 = 20u32;
pub const EVENT_TRACE_TYPE_REGSETSECURITY: u32 = 28u32;
pub const EVENT_TRACE_TYPE_REGSETVALUE: u32 = 14u32;
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: u32 = 26u32;
pub const EVENT_TRACE_TYPE_REPLY: u32 = 6u32;
pub const EVENT_TRACE_TYPE_RESUME: u32 = 7u32;
pub const EVENT_TRACE_TYPE_RETRANSMIT: u32 = 14u32;
pub const EVENT_TRACE_TYPE_SECURITY: u32 = 13u32;
pub const EVENT_TRACE_TYPE_SEND: u32 = 10u32;
pub const EVENT_TRACE_TYPE_SIDINFO: u32 = 12u32;
pub const EVENT_TRACE_TYPE_START: u32 = 1u32;
pub const EVENT_TRACE_TYPE_STOP: u32 = 2u32;
pub const EVENT_TRACE_TYPE_SUSPEND: u32 = 8u32;
pub const EVENT_TRACE_TYPE_TERMINATE: u32 = 11u32;
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: u32 = 240u32;
pub const EVENT_TRACE_TYPE_WINEVT_SEND: u32 = 9u32;
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: u32 = 16384u32;
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: u32 = 8192u32;
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: u32 = 32768u32;
pub const EVENT_TRACE_USE_NOCPUTIME: u32 = 2u32;
pub const EVENT_TRACE_USE_PAGED_MEMORY: u32 = 16777216u32;
pub const EVENT_TRACE_USE_PROCTIME: u32 = 1u32;
pub const EVENT_WRITE_FLAG_INPRIVATE: u32 = 2u32;
pub const EVENT_WRITE_FLAG_NO_FAULTING: u32 = 1u32;
pub const EtwCompressionModeNoDisable: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(1i32);
pub const EtwCompressionModeNoRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(2i32);
pub const EtwCompressionModeRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(0i32);
pub const EtwPmcOwnerFree: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(0i32);
pub const EtwPmcOwnerTagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(2i32);
pub const EtwPmcOwnerTaggedWithSource: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(3i32);
pub const EtwPmcOwnerUntagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(1i32);
pub const EtwProviderTraitDecodeGuid: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(2i32);
pub const EtwProviderTraitTypeGroup: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(1i32);
pub const EtwProviderTraitTypeMax: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(3i32);
pub const EtwQueryLastDroppedTimes: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(3i32);
pub const EtwQueryLogFileHeader: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(4i32);
pub const EtwQueryPartitionInformation: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(1i32);
pub const EtwQueryPartitionInformationV2: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(2i32);
pub const EtwQueryProcessHandleInfoMax: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(5i32);
pub const EventChannelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(2i32);
pub const EventInformationMax: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(5i32);
pub const EventKeywordInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(0i32);
pub const EventLevelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(1i32);
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(4i32);
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(0i32);
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = EVENT_INFO_CLASS(1i32);
pub const EventProviderSetTraits: EVENT_INFO_CLASS = EVENT_INFO_CLASS(2i32);
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = EVENT_INFO_CLASS(3i32);
pub const EventSecurityAddDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(2i32);
pub const EventSecurityAddSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(3i32);
pub const EventSecurityMax: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(4i32);
pub const EventSecuritySetDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(0i32);
pub const EventSecuritySetSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(1i32);
pub const EventTaskInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(3i32);
pub const EventTraceConfigGuid: windows_core::GUID = windows_core::GUID::from_u128(0x01853a65_418f_4f36_aefc_dc0f1d2fd235);
pub const EventTraceGuid: windows_core::GUID = windows_core::GUID::from_u128(0x68fdd900_4a3e_11d1_84f4_0000f80464e3);
pub const FileIoGuid: windows_core::GUID = windows_core::GUID::from_u128(0x90cbdc39_4a3e_11d1_84f4_0000f80464e3);
pub const GLOBAL_LOGGER_NAME: windows_core::PCWSTR = windows_core::w!("GlobalLogger");
pub const GLOBAL_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("GlobalLogger");
pub const GLOBAL_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("GlobalLogger");
windows_core::imp::define_interface!(ITraceEvent, ITraceEvent_Vtbl, 0x8cc97f40_9028_4ff3_9b62_7d1f79ca7bcb);
windows_core::imp::interface_hierarchy!(ITraceEvent, windows_core::IUnknown);
impl ITraceEvent {
    pub unsafe fn Clone(&self) -> windows_core::Result<ITraceEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetUserContext(&self) -> windows_core::Result<*mut core::ffi::c_void> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEventRecord(&self) -> windows_core::Result<*mut EVENT_RECORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventRecord)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPayload(&self, payload: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPayload)(windows_core::Interface::as_raw(self), core::mem::transmute(payload.as_ptr()), payload.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventDescriptor)(windows_core::Interface::as_raw(self), eventdescriptor).ok() }
    }
    pub unsafe fn SetProcessId(&self, processid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProcessId)(windows_core::Interface::as_raw(self), processid).ok() }
    }
    pub unsafe fn SetProcessorIndex(&self, processorindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProcessorIndex)(windows_core::Interface::as_raw(self), processorindex).ok() }
    }
    pub unsafe fn SetThreadId(&self, threadid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetThreadId)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
    pub unsafe fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetThreadTimes)(windows_core::Interface::as_raw(self), kerneltime, usertime).ok() }
    }
    pub unsafe fn SetActivityId(&self, activityid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActivityId)(windows_core::Interface::as_raw(self), activityid).ok() }
    }
    pub unsafe fn SetTimeStamp(&self, timestamp: *const i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeStamp)(windows_core::Interface::as_raw(self), timestamp).ok() }
    }
    pub unsafe fn SetProviderId(&self, providerid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProviderId)(windows_core::Interface::as_raw(self), providerid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut EVENT_RECORD) -> windows_core::HRESULT,
    pub SetPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub SetEventDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *const EVENT_DESCRIPTOR) -> windows_core::HRESULT,
    pub SetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetProcessorIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetThreadTimes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetTimeStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *const i64) -> windows_core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITraceEvent_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<ITraceEvent>;
    fn GetUserContext(&self) -> windows_core::Result<*mut core::ffi::c_void>;
    fn GetEventRecord(&self) -> windows_core::Result<*mut EVENT_RECORD>;
    fn SetPayload(&self, payload: *const u8, payloadsize: u32) -> windows_core::Result<()>;
    fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> windows_core::Result<()>;
    fn SetProcessId(&self, processid: u32) -> windows_core::Result<()>;
    fn SetProcessorIndex(&self, processorindex: u32) -> windows_core::Result<()>;
    fn SetThreadId(&self, threadid: u32) -> windows_core::Result<()>;
    fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> windows_core::Result<()>;
    fn SetActivityId(&self, activityid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetTimeStamp(&self, timestamp: *const i64) -> windows_core::Result<()>;
    fn SetProviderId(&self, providerid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITraceEvent_Vtbl {
    pub const fn new<Identity: ITraceEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceEvent_Impl::Clone(this) {
                    Ok(ok__) => {
                        newevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserContext<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usercontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceEvent_Impl::GetUserContext(this) {
                    Ok(ok__) => {
                        usercontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventRecord<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceEvent_Impl::GetEventRecord(this) {
                    Ok(ok__) => {
                        eventrecord.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPayload<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, payload: *const u8, payloadsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetPayload(this, core::mem::transmute_copy(&payload), core::mem::transmute_copy(&payloadsize)).into()
            }
        }
        unsafe extern "system" fn SetEventDescriptor<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetEventDescriptor(this, core::mem::transmute_copy(&eventdescriptor)).into()
            }
        }
        unsafe extern "system" fn SetProcessId<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetProcessId(this, core::mem::transmute_copy(&processid)).into()
            }
        }
        unsafe extern "system" fn SetProcessorIndex<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetProcessorIndex(this, core::mem::transmute_copy(&processorindex)).into()
            }
        }
        unsafe extern "system" fn SetThreadId<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetThreadId(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        unsafe extern "system" fn SetThreadTimes<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kerneltime: u32, usertime: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetThreadTimes(this, core::mem::transmute_copy(&kerneltime), core::mem::transmute_copy(&usertime)).into()
            }
        }
        unsafe extern "system" fn SetActivityId<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetActivityId(this, core::mem::transmute_copy(&activityid)).into()
            }
        }
        unsafe extern "system" fn SetTimeStamp<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *const i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetTimeStamp(this, core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn SetProviderId<Identity: ITraceEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEvent_Impl::SetProviderId(this, core::mem::transmute_copy(&providerid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            GetUserContext: GetUserContext::<Identity, OFFSET>,
            GetEventRecord: GetEventRecord::<Identity, OFFSET>,
            SetPayload: SetPayload::<Identity, OFFSET>,
            SetEventDescriptor: SetEventDescriptor::<Identity, OFFSET>,
            SetProcessId: SetProcessId::<Identity, OFFSET>,
            SetProcessorIndex: SetProcessorIndex::<Identity, OFFSET>,
            SetThreadId: SetThreadId::<Identity, OFFSET>,
            SetThreadTimes: SetThreadTimes::<Identity, OFFSET>,
            SetActivityId: SetActivityId::<Identity, OFFSET>,
            SetTimeStamp: SetTimeStamp::<Identity, OFFSET>,
            SetProviderId: SetProviderId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceEvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITraceEvent {}
windows_core::imp::define_interface!(ITraceEventCallback, ITraceEventCallback_Vtbl, 0x3ed25501_593f_43e9_8f38_3ab46f5a4a52);
windows_core::imp::interface_hierarchy!(ITraceEventCallback, windows_core::IUnknown);
impl ITraceEventCallback {
    pub unsafe fn OnBeginProcessTrace<P0, P1>(&self, headerevent: P0, relogger: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceEvent>,
        P1: windows_core::Param<ITraceRelogger>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeginProcessTrace)(windows_core::Interface::as_raw(self), headerevent.param().abi(), relogger.param().abi()).ok() }
    }
    pub unsafe fn OnFinalizeProcessTrace<P0>(&self, relogger: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceRelogger>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnFinalizeProcessTrace)(windows_core::Interface::as_raw(self), relogger.param().abi()).ok() }
    }
    pub unsafe fn OnEvent<P0, P1>(&self, event: P0, relogger: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceEvent>,
        P1: windows_core::Param<ITraceRelogger>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), event.param().abi(), relogger.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBeginProcessTrace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnFinalizeProcessTrace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITraceEventCallback_Impl: windows_core::IUnknownImpl {
    fn OnBeginProcessTrace(&self, headerevent: windows_core::Ref<'_, ITraceEvent>, relogger: windows_core::Ref<'_, ITraceRelogger>) -> windows_core::Result<()>;
    fn OnFinalizeProcessTrace(&self, relogger: windows_core::Ref<'_, ITraceRelogger>) -> windows_core::Result<()>;
    fn OnEvent(&self, event: windows_core::Ref<'_, ITraceEvent>, relogger: windows_core::Ref<'_, ITraceRelogger>) -> windows_core::Result<()>;
}
impl ITraceEventCallback_Vtbl {
    pub const fn new<Identity: ITraceEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBeginProcessTrace<Identity: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, headerevent: *mut core::ffi::c_void, relogger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEventCallback_Impl::OnBeginProcessTrace(this, core::mem::transmute_copy(&headerevent), core::mem::transmute_copy(&relogger)).into()
            }
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Identity: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relogger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEventCallback_Impl::OnFinalizeProcessTrace(this, core::mem::transmute_copy(&relogger)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, relogger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceEventCallback_Impl::OnEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&relogger)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginProcessTrace: OnBeginProcessTrace::<Identity, OFFSET>,
            OnFinalizeProcessTrace: OnFinalizeProcessTrace::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceEventCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITraceEventCallback {}
windows_core::imp::define_interface!(ITraceRelogger, ITraceRelogger_Vtbl, 0xf754ad43_3bcc_4286_8009_9c5da214e84e);
windows_core::imp::interface_hierarchy!(ITraceRelogger, windows_core::IUnknown);
impl ITraceRelogger {
    pub unsafe fn AddLogfileTraceStream(&self, logfilename: &windows_core::BSTR, usercontext: *const core::ffi::c_void) -> windows_core::Result<RELOGSTREAM_HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddLogfileTraceStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(logfilename), usercontext, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddRealtimeTraceStream(&self, loggername: &windows_core::BSTR, usercontext: *const core::ffi::c_void) -> windows_core::Result<RELOGSTREAM_HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddRealtimeTraceStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(loggername), usercontext, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterCallback<P0>(&self, callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceEventCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterCallback)(windows_core::Interface::as_raw(self), callback.param().abi()).ok() }
    }
    pub unsafe fn Inject<P0>(&self, event: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).Inject)(windows_core::Interface::as_raw(self), event.param().abi()).ok() }
    }
    pub unsafe fn CreateEventInstance(&self, tracehandle: RELOGSTREAM_HANDLE, flags: u32) -> windows_core::Result<ITraceEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEventInstance)(windows_core::Interface::as_raw(self), core::mem::transmute(tracehandle), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProcessTrace(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessTrace)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetOutputFilename(&self, logfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFilename)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(logfilename)).ok() }
    }
    pub unsafe fn SetCompressionMode(&self, compressionmode: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompressionMode)(windows_core::Interface::as_raw(self), compressionmode).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceRelogger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddLogfileTraceStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, *mut RELOGSTREAM_HANDLE) -> windows_core::HRESULT,
    pub AddRealtimeTraceStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, *mut RELOGSTREAM_HANDLE) -> windows_core::HRESULT,
    pub RegisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Inject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEventInstance: unsafe extern "system" fn(*mut core::ffi::c_void, RELOGSTREAM_HANDLE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessTrace: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputFilename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompressionMode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITraceRelogger_Impl: windows_core::IUnknownImpl {
    fn AddLogfileTraceStream(&self, logfilename: &windows_core::BSTR, usercontext: *const core::ffi::c_void) -> windows_core::Result<RELOGSTREAM_HANDLE>;
    fn AddRealtimeTraceStream(&self, loggername: &windows_core::BSTR, usercontext: *const core::ffi::c_void) -> windows_core::Result<RELOGSTREAM_HANDLE>;
    fn RegisterCallback(&self, callback: windows_core::Ref<'_, ITraceEventCallback>) -> windows_core::Result<()>;
    fn Inject(&self, event: windows_core::Ref<'_, ITraceEvent>) -> windows_core::Result<()>;
    fn CreateEventInstance(&self, tracehandle: &RELOGSTREAM_HANDLE, flags: u32) -> windows_core::Result<ITraceEvent>;
    fn ProcessTrace(&self) -> windows_core::Result<()>;
    fn SetOutputFilename(&self, logfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCompressionMode(&self, compressionmode: bool) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl ITraceRelogger_Vtbl {
    pub const fn new<Identity: ITraceRelogger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddLogfileTraceStream<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilename: *mut core::ffi::c_void, usercontext: *const core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceRelogger_Impl::AddLogfileTraceStream(this, core::mem::transmute(&logfilename), core::mem::transmute_copy(&usercontext)) {
                    Ok(ok__) => {
                        tracehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggername: *mut core::ffi::c_void, usercontext: *const core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceRelogger_Impl::AddRealtimeTraceStream(this, core::mem::transmute(&loggername), core::mem::transmute_copy(&usercontext)) {
                    Ok(ok__) => {
                        tracehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterCallback<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceRelogger_Impl::RegisterCallback(this, core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn Inject<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceRelogger_Impl::Inject(this, core::mem::transmute_copy(&event)).into()
            }
        }
        unsafe extern "system" fn CreateEventInstance<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tracehandle: RELOGSTREAM_HANDLE, flags: u32, event: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceRelogger_Impl::CreateEventInstance(this, core::mem::transmute(&tracehandle), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        event.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessTrace<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceRelogger_Impl::ProcessTrace(this).into()
            }
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceRelogger_Impl::SetOutputFilename(this, core::mem::transmute(&logfilename)).into()
            }
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compressionmode: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceRelogger_Impl::SetCompressionMode(this, core::mem::transmute_copy(&compressionmode)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceRelogger_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddLogfileTraceStream: AddLogfileTraceStream::<Identity, OFFSET>,
            AddRealtimeTraceStream: AddRealtimeTraceStream::<Identity, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, OFFSET>,
            Inject: Inject::<Identity, OFFSET>,
            CreateEventInstance: CreateEventInstance::<Identity, OFFSET>,
            ProcessTrace: ProcessTrace::<Identity, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceRelogger as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITraceRelogger {}
pub const ImageLoadGuid: windows_core::GUID = windows_core::GUID::from_u128(0x2cb15d1d_5fc1_11d2_abe1_00a0c911f518);
pub const KERNEL_LOGGER_NAME: windows_core::PCWSTR = windows_core::w!("NT Kernel Logger");
pub const KERNEL_LOGGER_NAMEA: windows_core::PCSTR = windows_core::s!("NT Kernel Logger");
pub const KERNEL_LOGGER_NAMEW: windows_core::PCWSTR = windows_core::w!("NT Kernel Logger");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAP_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAP_VALUETYPE(pub i32);
pub const MAX_EVENT_DATA_DESCRIPTORS: u32 = 128u32;
pub const MAX_EVENT_FILTERS_COUNT: u32 = 13u32;
pub const MAX_EVENT_FILTER_DATA_SIZE: u32 = 1024u32;
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: u32 = 64u32;
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: u32 = 4096u32;
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: u32 = 4096u32;
pub const MAX_EVENT_FILTER_PID_COUNT: u32 = 8u32;
pub const MAX_MOF_FIELDS: u32 = 16u32;
pub const MAX_PAYLOAD_PREDICATES: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOF_FIELD {
    pub DataPtr: u64,
    pub Length: u32,
    pub DataType: u32,
}
pub const MaxEventInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(4i32);
pub const MaxTraceSetInfoClass: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(28i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
pub const PAYLOADFIELD_BETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(6i32);
pub const PAYLOADFIELD_CONTAINS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(20i32);
pub const PAYLOADFIELD_DOESNTCONTAIN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(21i32);
pub const PAYLOADFIELD_EQ: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(0i32);
pub const PAYLOADFIELD_GE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(5i32);
pub const PAYLOADFIELD_GT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(3i32);
pub const PAYLOADFIELD_INVALID: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(32i32);
pub const PAYLOADFIELD_IS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(30i32);
pub const PAYLOADFIELD_ISNOT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(31i32);
pub const PAYLOADFIELD_LE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(2i32);
pub const PAYLOADFIELD_LT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(4i32);
pub const PAYLOADFIELD_MODULO: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(8i32);
pub const PAYLOADFIELD_NE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(1i32);
pub const PAYLOADFIELD_NOTBETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: windows_core::PWSTR,
    pub CompareOp: u16,
    pub Value: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAYLOAD_OPERATOR(pub i32);
pub type PENABLECALLBACK = Option<unsafe extern "system" fn(sourceid: *const windows_core::GUID, isenabled: ENABLECALLBACK_ENABLED_STATE, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut core::ffi::c_void)>;
#[cfg(feature = "Win32_System_Time")]
pub type PETW_BUFFER_CALLBACK = Option<unsafe extern "system" fn(buffer: *const ETW_BUFFER_HEADER, buffersize: u32, consumerinfo: *const ETW_BUFFER_CALLBACK_INFORMATION, callbackcontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PETW_BUFFER_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(buffer: *const ETW_BUFFER_HEADER, callbackcontext: *const core::ffi::c_void)>;
pub type PEVENT_CALLBACK = Option<unsafe extern "system" fn(pevent: *mut EVENT_TRACE)>;
pub type PEVENT_RECORD_CALLBACK = Option<unsafe extern "system" fn(eventrecord: *mut EVENT_RECORD)>;
#[cfg(feature = "Win32_System_Time")]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEA) -> u32>;
#[cfg(feature = "Win32_System_Time")]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEW) -> u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSTRACE_HANDLE {
    pub Value: u64,
}
pub const PROCESS_TRACE_MODE_EVENT_RECORD: u32 = 268435456u32;
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: u32 = 4096u32;
pub const PROCESS_TRACE_MODE_REAL_TIME: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTY_FLAGS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: u32,
    pub Reserved: u32,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
impl Default for PROVIDER_ENUMERATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: u32,
    pub Reserved: u32,
    pub EventDescriptorsArray: [EVENT_DESCRIPTOR; 1],
}
impl Default for PROVIDER_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: u32,
    pub DescriptionOffset: u32,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: u32,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
impl Default for PROVIDER_FIELD_INFOARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROVIDER_FILTER_INFO {
    pub Id: u8,
    pub Version: u8,
    pub MessageOffset: u32,
    pub Reserved: u32,
    pub PropertyCount: u32,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl Default for PROVIDER_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PageFaultGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3d6fa8d3_fe05_11d0_9dda_00c04fd7ba7c);
pub const PerfInfoGuid: windows_core::GUID = windows_core::GUID::from_u128(0xce1dbfb4_137e_4da6_87b0_3f59aa102cbc);
pub const PrivateLoggerNotificationGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3595ab5c_042a_4c8e_b942_2d059bfeb1b1);
pub const ProcessGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3d6fa8d0_fe05_11d0_9dda_00c04fd7ba7c);
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = PROPERTY_FLAGS(128i32);
pub const PropertyHasTags: PROPERTY_FLAGS = PROPERTY_FLAGS(64i32);
pub const PropertyParamCount: PROPERTY_FLAGS = PROPERTY_FLAGS(4i32);
pub const PropertyParamFixedCount: PROPERTY_FLAGS = PROPERTY_FLAGS(32i32);
pub const PropertyParamFixedLength: PROPERTY_FLAGS = PROPERTY_FLAGS(16i32);
pub const PropertyParamLength: PROPERTY_FLAGS = PROPERTY_FLAGS(2i32);
pub const PropertyStruct: PROPERTY_FLAGS = PROPERTY_FLAGS(1i32);
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = PROPERTY_FLAGS(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct REGHANDLE(pub i64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RELOGSTREAM_HANDLE {
    pub Value: u64,
}
pub const RegistryGuid: windows_core::GUID = windows_core::GUID::from_u128(0xae53722e_c863_11d2_8659_00c04fa321a1);
pub const SYSTEM_ALPC_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_CONFIG_KW_GRAPHICS: u64 = 2u64;
pub const SYSTEM_CONFIG_KW_NETWORK: u64 = 8u64;
pub const SYSTEM_CONFIG_KW_OPTICAL: u64 = 64u64;
pub const SYSTEM_CONFIG_KW_PNP: u64 = 32u64;
pub const SYSTEM_CONFIG_KW_SERVICES: u64 = 16u64;
pub const SYSTEM_CONFIG_KW_STORAGE: u64 = 4u64;
pub const SYSTEM_CONFIG_KW_SYSTEM: u64 = 1u64;
pub const SYSTEM_CPU_KW_CACHE_FLUSH: u64 = 2u64;
pub const SYSTEM_CPU_KW_CONFIG: u64 = 1u64;
pub const SYSTEM_CPU_KW_SPEC_CONTROL: u64 = 4u64;
pub const SYSTEM_EVENT_TYPE: u32 = 1u32;
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: u64 = 2u64;
pub const SYSTEM_HYPERVISOR_KW_PROFILE: u64 = 1u64;
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: u64 = 4u64;
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: u64 = 2u64;
pub const SYSTEM_INTERRUPT_KW_DPC: u64 = 4u64;
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: u64 = 8u64;
pub const SYSTEM_INTERRUPT_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_INTERRUPT_KW_IPI: u64 = 64u64;
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: u64 = 16u64;
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: u64 = 32u64;
pub const SYSTEM_IOFILTER_KW_FAILURE: u64 = 8u64;
pub const SYSTEM_IOFILTER_KW_FASTIO: u64 = 4u64;
pub const SYSTEM_IOFILTER_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_IOFILTER_KW_INIT: u64 = 2u64;
pub const SYSTEM_IO_KW_CC: u64 = 256u64;
pub const SYSTEM_IO_KW_DISK: u64 = 1u64;
pub const SYSTEM_IO_KW_DISK_INIT: u64 = 2u64;
pub const SYSTEM_IO_KW_DRIVERS: u64 = 128u64;
pub const SYSTEM_IO_KW_FILE: u64 = 16u64;
pub const SYSTEM_IO_KW_FILENAME: u64 = 4u64;
pub const SYSTEM_IO_KW_NETWORK: u64 = 512u64;
pub const SYSTEM_IO_KW_OPTICAL: u64 = 32u64;
pub const SYSTEM_IO_KW_OPTICAL_INIT: u64 = 64u64;
pub const SYSTEM_IO_KW_SPLIT: u64 = 8u64;
pub const SYSTEM_LOCK_KW_SPINLOCK: u64 = 1u64;
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: u64 = 2u64;
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: u64 = 4u64;
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: u64 = 4u64;
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: u64 = 512u64;
pub const SYSTEM_MEMORY_KW_FOOTPRINT: u64 = 2048u64;
pub const SYSTEM_MEMORY_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: u64 = 2u64;
pub const SYSTEM_MEMORY_KW_HEAP: u64 = 128u64;
pub const SYSTEM_MEMORY_KW_MEMINFO: u64 = 16u64;
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: u64 = 64u64;
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: u64 = 32768u64;
pub const SYSTEM_MEMORY_KW_PFSECTION: u64 = 32u64;
pub const SYSTEM_MEMORY_KW_POOL: u64 = 8u64;
pub const SYSTEM_MEMORY_KW_REFSET: u64 = 8192u64;
pub const SYSTEM_MEMORY_KW_SESSION: u64 = 4096u64;
pub const SYSTEM_MEMORY_KW_VAMAP: u64 = 16384u64;
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: u64 = 1024u64;
pub const SYSTEM_MEMORY_KW_WS: u64 = 256u64;
pub const SYSTEM_MEMORY_POOL_FILTER_ID: u32 = 1u32;
pub const SYSTEM_OBJECT_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_OBJECT_KW_HANDLE: u64 = 2u64;
pub const SYSTEM_POWER_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: u64 = 2u64;
pub const SYSTEM_POWER_KW_IDLE_SELECTION: u64 = 8u64;
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: u64 = 16u64;
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: u64 = 4u64;
pub const SYSTEM_PROCESS_KW_DBGPRINT: u64 = 256u64;
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: u64 = 128u64;
pub const SYSTEM_PROCESS_KW_FREEZE: u64 = 4u64;
pub const SYSTEM_PROCESS_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_PROCESS_KW_INSWAP: u64 = 2u64;
pub const SYSTEM_PROCESS_KW_JOB: u64 = 512u64;
pub const SYSTEM_PROCESS_KW_LOADER: u64 = 4096u64;
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: u64 = 8u64;
pub const SYSTEM_PROCESS_KW_THREAD: u64 = 2048u64;
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: u64 = 16u64;
pub const SYSTEM_PROCESS_KW_WAKE_DROP: u64 = 32u64;
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: u64 = 64u64;
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: u64 = 1024u64;
pub const SYSTEM_PROFILE_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: u64 = 2u64;
pub const SYSTEM_REGISTRY_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_REGISTRY_KW_HIVE: u64 = 2u64;
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: u64 = 4u64;
pub const SYSTEM_SCHEDULER_KW_AFFINITY: u64 = 64u64;
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: u64 = 16u64;
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: u64 = 1024u64;
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: u64 = 512u64;
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: u64 = 2u64;
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: u64 = 256u64;
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: u64 = 4u64;
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: u64 = 32u64;
pub const SYSTEM_SCHEDULER_KW_PRIORITY: u64 = 128u64;
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: u64 = 8u64;
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: u64 = 1u64;
pub const SYSTEM_SYSCALL_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: u64 = 2u64;
pub const SYSTEM_TIMER_KW_GENERAL: u64 = 1u64;
pub const SplitIoGuid: windows_core::GUID = windows_core::GUID::from_u128(0xd837ca92_12b9_44a5_ad6a_3a65b3578aa8);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(5i32);
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(4i32);
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TDH_CONTEXT_TYPE(pub i32);
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(2i32);
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(0i32);
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TDH_HANDLE(pub isize);
impl TDH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for TDH_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("tdh.dll" "system" fn TdhCloseDecodingHandle(handle : isize) -> u32);
            unsafe {
                TdhCloseDecodingHandle(self.0);
            }
        }
    }
}
pub const TDH_INTYPE_ANSICHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(307i32);
pub const TDH_INTYPE_ANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(2i32);
pub const TDH_INTYPE_BINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(14i32);
pub const TDH_INTYPE_BOOLEAN: _TDH_IN_TYPE = _TDH_IN_TYPE(13i32);
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(301i32);
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(300i32);
pub const TDH_INTYPE_DOUBLE: _TDH_IN_TYPE = _TDH_IN_TYPE(12i32);
pub const TDH_INTYPE_FILETIME: _TDH_IN_TYPE = _TDH_IN_TYPE(17i32);
pub const TDH_INTYPE_FLOAT: _TDH_IN_TYPE = _TDH_IN_TYPE(11i32);
pub const TDH_INTYPE_GUID: _TDH_IN_TYPE = _TDH_IN_TYPE(15i32);
pub const TDH_INTYPE_HEXDUMP: _TDH_IN_TYPE = _TDH_IN_TYPE(309i32);
pub const TDH_INTYPE_HEXINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(20i32);
pub const TDH_INTYPE_HEXINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(21i32);
pub const TDH_INTYPE_INT16: _TDH_IN_TYPE = _TDH_IN_TYPE(5i32);
pub const TDH_INTYPE_INT32: _TDH_IN_TYPE = _TDH_IN_TYPE(7i32);
pub const TDH_INTYPE_INT64: _TDH_IN_TYPE = _TDH_IN_TYPE(9i32);
pub const TDH_INTYPE_INT8: _TDH_IN_TYPE = _TDH_IN_TYPE(3i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(23i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDBINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(25i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(22i32);
pub const TDH_INTYPE_NONNULLTERMINATEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(305i32);
pub const TDH_INTYPE_NONNULLTERMINATEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(304i32);
pub const TDH_INTYPE_NULL: _TDH_IN_TYPE = _TDH_IN_TYPE(0i32);
pub const TDH_INTYPE_POINTER: _TDH_IN_TYPE = _TDH_IN_TYPE(16i32);
pub const TDH_INTYPE_RESERVED24: _TDH_IN_TYPE = _TDH_IN_TYPE(24i32);
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(303i32);
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(302i32);
pub const TDH_INTYPE_SID: _TDH_IN_TYPE = _TDH_IN_TYPE(19i32);
pub const TDH_INTYPE_SIZET: _TDH_IN_TYPE = _TDH_IN_TYPE(308i32);
pub const TDH_INTYPE_SYSTEMTIME: _TDH_IN_TYPE = _TDH_IN_TYPE(18i32);
pub const TDH_INTYPE_UINT16: _TDH_IN_TYPE = _TDH_IN_TYPE(6i32);
pub const TDH_INTYPE_UINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(8i32);
pub const TDH_INTYPE_UINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(10i32);
pub const TDH_INTYPE_UINT8: _TDH_IN_TYPE = _TDH_IN_TYPE(4i32);
pub const TDH_INTYPE_UNICODECHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(306i32);
pub const TDH_INTYPE_UNICODESTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(1i32);
pub const TDH_INTYPE_WBEMSID: _TDH_IN_TYPE = _TDH_IN_TYPE(310i32);
pub const TDH_OUTTYPE_BOOLEAN: _TDH_OUT_TYPE = _TDH_OUT_TYPE(13i32);
pub const TDH_OUTTYPE_BYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(3i32);
pub const TDH_OUTTYPE_CIMDATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(26i32);
pub const TDH_OUTTYPE_CODE_POINTER: _TDH_OUT_TYPE = _TDH_OUT_TYPE(37i32);
pub const TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(33i32);
pub const TDH_OUTTYPE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(2i32);
pub const TDH_OUTTYPE_DATETIME_UTC: _TDH_OUT_TYPE = _TDH_OUT_TYPE(38i32);
pub const TDH_OUTTYPE_DOUBLE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(12i32);
pub const TDH_OUTTYPE_ERRORCODE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(29i32);
pub const TDH_OUTTYPE_ETWTIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(27i32);
pub const TDH_OUTTYPE_FLOAT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(11i32);
pub const TDH_OUTTYPE_GUID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(14i32);
pub const TDH_OUTTYPE_HEXBINARY: _TDH_OUT_TYPE = _TDH_OUT_TYPE(15i32);
pub const TDH_OUTTYPE_HEXINT16: _TDH_OUT_TYPE = _TDH_OUT_TYPE(17i32);
pub const TDH_OUTTYPE_HEXINT32: _TDH_OUT_TYPE = _TDH_OUT_TYPE(18i32);
pub const TDH_OUTTYPE_HEXINT64: _TDH_OUT_TYPE = _TDH_OUT_TYPE(19i32);
pub const TDH_OUTTYPE_HEXINT8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(16i32);
pub const TDH_OUTTYPE_HRESULT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(32i32);
pub const TDH_OUTTYPE_INT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(7i32);
pub const TDH_OUTTYPE_IPV4: _TDH_OUT_TYPE = _TDH_OUT_TYPE(23i32);
pub const TDH_OUTTYPE_IPV6: _TDH_OUT_TYPE = _TDH_OUT_TYPE(24i32);
pub const TDH_OUTTYPE_JSON: _TDH_OUT_TYPE = _TDH_OUT_TYPE(34i32);
pub const TDH_OUTTYPE_LONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(9i32);
pub const TDH_OUTTYPE_NOPRINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(301i32);
pub const TDH_OUTTYPE_NTSTATUS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(31i32);
pub const TDH_OUTTYPE_NULL: _TDH_OUT_TYPE = _TDH_OUT_TYPE(0i32);
pub const TDH_OUTTYPE_PID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(20i32);
pub const TDH_OUTTYPE_PKCS7_WITH_TYPE_INFO: _TDH_OUT_TYPE = _TDH_OUT_TYPE(36i32);
pub const TDH_OUTTYPE_PORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(22i32);
pub const TDH_OUTTYPE_REDUCEDSTRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(300i32);
pub const TDH_OUTTYPE_SHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(5i32);
pub const TDH_OUTTYPE_SOCKETADDRESS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(25i32);
pub const TDH_OUTTYPE_STRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(1i32);
pub const TDH_OUTTYPE_TID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(21i32);
pub const TDH_OUTTYPE_UNSIGNEDBYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(4i32);
pub const TDH_OUTTYPE_UNSIGNEDINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(8i32);
pub const TDH_OUTTYPE_UNSIGNEDLONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(10i32);
pub const TDH_OUTTYPE_UNSIGNEDSHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(6i32);
pub const TDH_OUTTYPE_UTF8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(35i32);
pub const TDH_OUTTYPE_WIN32ERROR: _TDH_OUT_TYPE = _TDH_OUT_TYPE(30i32);
pub const TDH_OUTTYPE_XML: _TDH_OUT_TYPE = _TDH_OUT_TYPE(28i32);
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = TEMPLATE_FLAGS(4i32);
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEMPLATE_FLAGS(pub i32);
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(2i32);
pub const TRACELOG_ACCESS_KERNEL_LOGGER: u32 = 256u32;
pub const TRACELOG_ACCESS_REALTIME: u32 = 1024u32;
pub const TRACELOG_CREATE_INPROC: u32 = 512u32;
pub const TRACELOG_CREATE_ONDISK: u32 = 64u32;
pub const TRACELOG_CREATE_REALTIME: u32 = 32u32;
pub const TRACELOG_GUID_ENABLE: u32 = 128u32;
pub const TRACELOG_JOIN_GROUP: u32 = 4096u32;
pub const TRACELOG_LOG_EVENT: u32 = 512u32;
pub const TRACELOG_REGISTER_GUIDS: u32 = 2048u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy)]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: windows_core::GUID,
    pub EventGuid: windows_core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub DecodingSource: DECODING_SOURCE,
    pub ProviderNameOffset: u32,
    pub LevelNameOffset: u32,
    pub ChannelNameOffset: u32,
    pub KeywordsNameOffset: u32,
    pub TaskNameOffset: u32,
    pub OpcodeNameOffset: u32,
    pub EventMessageOffset: u32,
    pub ProviderMessageOffset: u32,
    pub BinaryXMLOffset: u32,
    pub BinaryXMLSize: u32,
    pub Anonymous1: TRACE_EVENT_INFO_0,
    pub Anonymous2: TRACE_EVENT_INFO_1,
    pub PropertyCount: u32,
    pub TopLevelPropertyCount: u32,
    pub Anonymous3: TRACE_EVENT_INFO_2,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
impl Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
impl Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
impl Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: windows_core::GUID,
    pub GuidType: u32,
    pub LoggerId: u32,
    pub EnableLevel: u32,
    pub EnableFlags: u32,
    pub IsEnable: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: *const windows_core::GUID,
    pub RegHandle: super::super::super::Foundation::HANDLE,
}
impl Default for TRACE_GUID_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TRACE_HEADER_FLAG_LOG_WNODE: u32 = 262144u32;
pub const TRACE_HEADER_FLAG_TRACED_GUID: u32 = 131072u32;
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: u32 = 524288u32;
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: u32 = 1048576u32;
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: u32 = 512u32;
pub const TRACE_LEVEL_CRITICAL: u32 = 1u32;
pub const TRACE_LEVEL_ERROR: u32 = 2u32;
pub const TRACE_LEVEL_FATAL: u32 = 1u32;
pub const TRACE_LEVEL_INFORMATION: u32 = 4u32;
pub const TRACE_LEVEL_NONE: u32 = 0u32;
pub const TRACE_LEVEL_RESERVED6: u32 = 6u32;
pub const TRACE_LEVEL_RESERVED7: u32 = 7u32;
pub const TRACE_LEVEL_RESERVED8: u32 = 8u32;
pub const TRACE_LEVEL_RESERVED9: u32 = 9u32;
pub const TRACE_LEVEL_VERBOSE: u32 = 5u32;
pub const TRACE_LEVEL_WARNING: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER_0,
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
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER32 {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER32_0,
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
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER32_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER64 {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER64_0,
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
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
#[cfg(feature = "Win32_System_Time")]
impl Default for TRACE_LOGFILE_HEADER64_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Time")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
pub const TRACE_MESSAGE_COMPONENTID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_MESSAGE_FLAGS(pub u32);
impl TRACE_MESSAGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TRACE_MESSAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TRACE_MESSAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TRACE_MESSAGE_FLAG_MASK: u32 = 65535u32;
pub const TRACE_MESSAGE_GUID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(2u32);
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: u32 = 16u32;
pub const TRACE_MESSAGE_POINTER32: u32 = 64u32;
pub const TRACE_MESSAGE_POINTER64: u32 = 128u32;
pub const TRACE_MESSAGE_SEQUENCE: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(1u32);
pub const TRACE_MESSAGE_SYSTEMINFO: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(32u32);
pub const TRACE_MESSAGE_TIMESTAMP: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(8u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: u32,
    pub ProviderCount: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: u32,
    pub Interval: u32,
}
pub const TRACE_PROVIDER_FLAG_LEGACY: u32 = 1u32;
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: windows_core::GUID,
    pub SchemaSource: u32,
    pub ProviderNameOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: u32,
    pub EnableCount: u32,
    pub Pid: u32,
    pub Flags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_QUERY_INFO_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: bool,
    pub CacheSize: u32,
    pub BucketCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: u32,
    pub Reserved: u32,
}
pub const TcpIpGuid: windows_core::GUID = windows_core::GUID::from_u128(0x9a280ac0_c8e0_11d1_84e2_00c04fb998a2);
pub const ThreadGuid: windows_core::GUID = windows_core::GUID::from_u128(0x3d6fa8d1_fe05_11d0_9dda_00c04fd7ba7c);
pub const TraceDisallowListQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(14i32);
pub const TraceGroupQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(13i32);
pub const TraceGroupQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(12i32);
pub const TraceGuidQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(1i32);
pub const TraceGuidQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(0i32);
pub const TraceGuidQueryProcess: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(2i32);
pub const TraceInfoReserved15: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(15i32);
pub const TraceLbrConfigurationInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(20i32);
pub const TraceLbrEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(21i32);
pub const TraceMaxLoggersQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(19i32);
pub const TraceMaxPmcCounterQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(22i32);
pub const TracePeriodicCaptureStateInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(17i32);
pub const TracePeriodicCaptureStateListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(16i32);
pub const TracePmcCounterListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(9i32);
pub const TracePmcCounterOwners: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(25i32);
pub const TracePmcEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(8i32);
pub const TracePmcSessionInformation: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(27i32);
pub const TraceProfileSourceConfigInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(6i32);
pub const TraceProfileSourceListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(7i32);
pub const TraceProviderBinaryTracking: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(18i32);
pub const TraceSampledProfileIntervalInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(5i32);
pub const TraceSetDisallowList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(10i32);
pub const TraceStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(24i32);
pub const TraceStackTracingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(3i32);
pub const TraceStreamCount: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(23i32);
pub const TraceSystemTraceEnableFlagsInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(4i32);
pub const TraceUnifiedStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(26i32);
pub const TraceVersionInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(11i32);
pub const UdpIpGuid: windows_core::GUID = windows_core::GUID::from_u128(0xbf3a50c5_a9c9_4988_a005_2df0b7c80f80);
pub type WMIDPREQUEST = Option<unsafe extern "system" fn(requestcode: WMIDPREQUESTCODE, requestcontext: *const core::ffi::c_void, buffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMIDPREQUESTCODE(pub i32);
pub const WMIGUID_EXECUTE: u32 = 16u32;
pub const WMIGUID_NOTIFICATION: u32 = 4u32;
pub const WMIGUID_QUERY: u32 = 1u32;
pub const WMIGUID_READ_DESCRIPTION: u32 = 8u32;
pub const WMIGUID_SET: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMIREGGUIDW {
    pub Guid: windows_core::GUID,
    pub Flags: u32,
    pub InstanceCount: u32,
    pub Anonymous: WMIREGGUIDW_0,
}
impl Default for WMIREGGUIDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WMIREGGUIDW_0 {
    pub InstanceNameList: u32,
    pub BaseNameOffset: u32,
    pub Pdo: usize,
    pub InstanceInfo: usize,
}
impl Default for WMIREGGUIDW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMIREGINFOW {
    pub BufferSize: u32,
    pub NextWmiRegInfo: u32,
    pub RegistryPath: u32,
    pub MofResourceName: u32,
    pub GuidCount: u32,
    pub WmiRegGuid: [WMIREGGUIDW; 1],
}
impl Default for WMIREGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMIREG_FLAG_EVENT_ONLY_GUID: u32 = 64u32;
pub const WMIREG_FLAG_EXPENSIVE: u32 = 1u32;
pub const WMIREG_FLAG_INSTANCE_BASENAME: u32 = 8u32;
pub const WMIREG_FLAG_INSTANCE_LIST: u32 = 4u32;
pub const WMIREG_FLAG_INSTANCE_PDO: u32 = 32u32;
pub const WMIREG_FLAG_REMOVE_GUID: u32 = 65536u32;
pub const WMIREG_FLAG_RESERVED1: u32 = 131072u32;
pub const WMIREG_FLAG_RESERVED2: u32 = 262144u32;
pub const WMIREG_FLAG_TRACED_GUID: u32 = 524288u32;
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: u32 = 4096u32;
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(10i32);
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(7i32);
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(5i32);
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(6i32);
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(4i32);
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = WMIDPREQUESTCODE(9i32);
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = WMIDPREQUESTCODE(0i32);
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(1i32);
pub const WMI_GLOBAL_LOGGER_ID: u32 = 1u32;
pub const WMI_GUIDTYPE_DATA: u32 = 2u32;
pub const WMI_GUIDTYPE_EVENT: u32 = 3u32;
pub const WMI_GUIDTYPE_TRACE: u32 = 1u32;
pub const WMI_GUIDTYPE_TRACECONTROL: u32 = 0u32;
pub const WMI_REGINFO: WMIDPREQUESTCODE = WMIDPREQUESTCODE(8i32);
pub const WMI_SET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(2i32);
pub const WMI_SET_SINGLE_ITEM: WMIDPREQUESTCODE = WMIDPREQUESTCODE(3i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_ALL_DATA {
    pub WnodeHeader: WNODE_HEADER,
    pub DataBlockOffset: u32,
    pub InstanceCount: u32,
    pub OffsetInstanceNameOffsets: u32,
    pub Anonymous: WNODE_ALL_DATA_0,
}
impl Default for WNODE_ALL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WNODE_ALL_DATA_0 {
    pub FixedInstanceSize: u32,
    pub OffsetInstanceDataAndLength: [OFFSETINSTANCEDATAANDLENGTH; 1],
}
impl Default for WNODE_ALL_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_EVENT_ITEM {
    pub WnodeHeader: WNODE_HEADER,
}
impl Default for WNODE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_EVENT_REFERENCE {
    pub WnodeHeader: WNODE_HEADER,
    pub TargetGuid: windows_core::GUID,
    pub TargetDataBlockSize: u32,
    pub Anonymous: WNODE_EVENT_REFERENCE_0,
}
impl Default for WNODE_EVENT_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WNODE_EVENT_REFERENCE_0 {
    pub TargetInstanceIndex: u32,
    pub TargetInstanceName: [u16; 1],
}
impl Default for WNODE_EVENT_REFERENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WNODE_FLAG_ALL_DATA: u32 = 1u32;
pub const WNODE_FLAG_ANSI_INSTANCENAMES: u32 = 16384u32;
pub const WNODE_FLAG_EVENT_ITEM: u32 = 8u32;
pub const WNODE_FLAG_EVENT_REFERENCE: u32 = 8192u32;
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: u32 = 16u32;
pub const WNODE_FLAG_INSTANCES_SAME: u32 = 64u32;
pub const WNODE_FLAG_INTERNAL: u32 = 256u32;
pub const WNODE_FLAG_LOG_WNODE: u32 = 262144u32;
pub const WNODE_FLAG_METHOD_ITEM: u32 = 32768u32;
pub const WNODE_FLAG_NO_HEADER: u32 = 2097152u32;
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: u32 = 65536u32;
pub const WNODE_FLAG_PERSIST_EVENT: u32 = 1024u32;
pub const WNODE_FLAG_SEND_DATA_BLOCK: u32 = 4194304u32;
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080u32;
pub const WNODE_FLAG_SINGLE_INSTANCE: u32 = 2u32;
pub const WNODE_FLAG_SINGLE_ITEM: u32 = 4u32;
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: u32 = 128u32;
pub const WNODE_FLAG_TOO_SMALL: u32 = 32u32;
pub const WNODE_FLAG_TRACED_GUID: u32 = 131072u32;
pub const WNODE_FLAG_USE_GUID_PTR: u32 = 524288u32;
pub const WNODE_FLAG_USE_MOF_PTR: u32 = 1048576u32;
pub const WNODE_FLAG_USE_TIMESTAMP: u32 = 512u32;
pub const WNODE_FLAG_VERSIONED_PROPERTIES: u32 = 8388608u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous1: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: windows_core::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
impl Default for WNODE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
impl Default for WNODE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::super::super::Foundation::HANDLE,
    pub TimeStamp: i64,
}
impl Default for WNODE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_METHOD_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub MethodId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
impl Default for WNODE_METHOD_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_SINGLE_INSTANCE {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
impl Default for WNODE_SINGLE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_SINGLE_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub ItemId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataItem: u32,
    pub VariableData: [u8; 1],
}
impl Default for WNODE_SINGLE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_TOO_SMALL {
    pub WnodeHeader: WNODE_HEADER,
    pub SizeNeeded: u32,
}
impl Default for WNODE_TOO_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _TDH_IN_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _TDH_OUT_TYPE(pub i32);
