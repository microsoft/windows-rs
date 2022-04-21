#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn CloseTrace(tracehandle: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ControlTraceA(tracehandle: u64, instancename: ::windows_sys::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ControlTraceW(tracehandle: u64, instancename: ::windows_sys::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTraceInstanceId(reghandle: super::super::super::Foundation::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn CveEventWrite(cveid: ::windows_sys::core::PCWSTR, additionaldetails: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows_sys::core::GUID, tracehandle: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EnableTraceEx(providerid: *const ::windows_sys::core::GUID, sourceid: *const ::windows_sys::core::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows_sys::core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventAccessControl(guid: *const ::windows_sys::core::GUID, operation: u32, sid: super::super::super::Foundation::PSID, rights: u32, allowordeny: super::super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn EventAccessQuery(guid: *const ::windows_sys::core::GUID, buffer: super::super::super::Security::PSECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventAccessRemove(guid: *const ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventRegister(providerid: *const ::windows_sys::core::GUID, enablecallback: PENABLECALLBACK, callbackcontext: *const ::core::ffi::c_void, reghandle: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventUnregister(reghandle: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows_sys::core::GUID, relatedactivityid: *const ::windows_sys::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventWriteString(reghandle: u64, level: u8, keyword: u64, string: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows_sys::core::GUID, relatedactivityid: *const ::windows_sys::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushTraceA(tracehandle: u64, instancename: ::windows_sys::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushTraceW(tracehandle: u64, instancename: ::windows_sys::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn GetTraceEnableFlags(tracehandle: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn GetTraceEnableLevel(tracehandle: u64) -> u8;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn GetTraceLoggerHandle(buffer: *const ::core::ffi::c_void) -> u64;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
    pub fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> u64;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
    pub fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> u64;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryTraceA(tracehandle: u64, instancename: ::windows_sys::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryTraceW(tracehandle: u64, instancename: ::windows_sys::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTraceGuidsA(requestaddress: WMIDPREQUEST, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows_sys::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: ::windows_sys::core::PCSTR, mofresourcename: ::windows_sys::core::PCSTR, registrationhandle: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTraceGuidsW(requestaddress: WMIDPREQUEST, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows_sys::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: ::windows_sys::core::PCWSTR, mofresourcename: ::windows_sys::core::PCWSTR, registrationhandle: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn RemoveTraceCallback(pguid: *const ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn SetTraceCallback(pguid: *const ::windows_sys::core::GUID, eventcallback: PEVENT_CALLBACK) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartTraceA(tracehandle: *mut u64, instancename: ::windows_sys::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartTraceW(tracehandle: *mut u64, instancename: ::windows_sys::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StopTraceA(tracehandle: u64, instancename: ::windows_sys::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StopTraceW(tracehandle: u64, instancename: ::windows_sys::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::core::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhCreatePayloadFilter(providerguid: *const ::windows_sys::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: super::super::super::Foundation::BOOLEAN, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows_sys::core::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows_sys::core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhEnumerateProviderFilters(guid: *const ::windows_sys::core::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: ::windows_sys::core::PWSTR, userdataconsumed: *mut u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetEventMapInformation(pevent: *const EVENT_RECORD, pmapname: ::windows_sys::core::PCWSTR, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetManifestEventInformation(providerguid: *const ::windows_sys::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhGetWppProperty(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, propertyname: ::windows_sys::core::PCWSTR, buffersize: *mut u32, buffer: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhLoadManifest(manifest: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhLoadManifestFromBinary(binarypath: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhLoadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhQueryProviderFieldInformation(pguid: *const ::windows_sys::core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhUnloadManifest(manifest: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TdhUnloadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows_sys::core::GUID, messagenumber: u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows_sys::core::GUID, messagenumber: u16, messagearglist: *const i8) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::core::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
    pub fn UnregisterTraceGuids(registrationhandle: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateTraceA(tracehandle: u64, instancename: ::windows_sys::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateTraceW(tracehandle: u64, instancename: ::windows_sys::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct CLASSIC_EVENT_ID {
    pub EventGuid: ::windows_sys::core::GUID,
    pub Type: u8,
    pub Reserved: [u8; 7],
}
impl ::core::marker::Copy for CLASSIC_EVENT_ID {}
impl ::core::clone::Clone for CLASSIC_EVENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_TraceRelogger: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2067822893, data2: 1535, data3: 17604, data4: [144, 88, 244, 64, 199, 31, 23, 212] };
pub const CTraceRelogger: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2067822893, data2: 1535, data3: 17604, data4: [144, 88, 244, 64, 199, 31, 23, 212] };
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type DECODING_SOURCE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceXMLFile: DECODING_SOURCE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceWbem: DECODING_SOURCE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceWPP: DECODING_SOURCE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceTlg: DECODING_SOURCE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceMax: DECODING_SOURCE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DIAG_LOGGER_NAMEA: &str = "DiagLog";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DIAG_LOGGER_NAMEW: &str = "DiagLog";
pub const DefaultTraceSecurityGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 135381423, data2: 31239, data3: 18950, data4: [130, 237, 134, 148, 85, 205, 247, 19] };
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type ENABLECALLBACK_ENABLED_STATE = u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: ENABLECALLBACK_ENABLED_STATE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows_sys::core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
impl ::core::marker::Copy for ENABLE_TRACE_PARAMETERS {}
impl ::core::clone::Clone for ENABLE_TRACE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows_sys::core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
}
impl ::core::marker::Copy for ENABLE_TRACE_PARAMETERS_V1 {}
impl ::core::clone::Clone for ENABLE_TRACE_PARAMETERS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ENABLE_TRACE_PARAMETERS_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_ASCIICHAR_TYPE_VALUE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_ASCIISTRING_TYPE_VALUE: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_BOOLEAN_TYPE_VALUE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_BOOL_TYPE_VALUE: u32 = 108u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: u16,
}
impl ::core::marker::Copy for ETW_BUFFER_CONTEXT {}
impl ::core::clone::Clone for ETW_BUFFER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: u16,
}
impl ::core::marker::Copy for ETW_BUFFER_CONTEXT_0 {}
impl ::core::clone::Clone for ETW_BUFFER_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: u8,
    pub Alignment: u8,
}
impl ::core::marker::Copy for ETW_BUFFER_CONTEXT_0_0 {}
impl ::core::clone::Clone for ETW_BUFFER_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_BYTE_TYPE_VALUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_CHAR_TYPE_VALUE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type ETW_COMPRESSION_RESUMPTION_MODE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwCompressionModeRestart: ETW_COMPRESSION_RESUMPTION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwCompressionModeNoDisable: ETW_COMPRESSION_RESUMPTION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwCompressionModeNoRestart: ETW_COMPRESSION_RESUMPTION_MODE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_COUNTED_STRING_TYPE_VALUE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_DATETIME_TYPE_VALUE: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_DECIMAL_TYPE_VALUE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_DOUBLE_TYPE_VALUE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_GUID_TYPE_VALUE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_HIDDEN_TYPE_VALUE: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_INT16_TYPE_VALUE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_INT32_TYPE_VALUE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_INT64_TYPE_VALUE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_NULL_TYPE_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_OBJECT_TYPE_VALUE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: u32,
    pub OwnerTag: u32,
}
impl ::core::marker::Copy for ETW_PMC_COUNTER_OWNER {}
impl ::core::clone::Clone for ETW_PMC_COUNTER_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    pub ProcessorNumber: u32,
    pub NumberOfCounters: u32,
    pub CounterOwners: [ETW_PMC_COUNTER_OWNER; 1],
}
impl ::core::marker::Copy for ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
impl ::core::clone::Clone for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type ETW_PMC_COUNTER_OWNER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerFree: ETW_PMC_COUNTER_OWNER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerUntagged: ETW_PMC_COUNTER_OWNER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerTagged: ETW_PMC_COUNTER_OWNER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerTaggedWithSource: ETW_PMC_COUNTER_OWNER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_POINTER_TYPE_VALUE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type ETW_PROCESS_HANDLE_INFO_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryPartitionInformation: ETW_PROCESS_HANDLE_INFO_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryPartitionInformationV2: ETW_PROCESS_HANDLE_INFO_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryLastDroppedTimes: ETW_PROCESS_HANDLE_INFO_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryProcessHandleInfoMax: ETW_PROCESS_HANDLE_INFO_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type ETW_PROVIDER_TRAIT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwProviderTraitTypeGroup: ETW_PROVIDER_TRAIT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwProviderTraitDecodeGuid: ETW_PROVIDER_TRAIT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwProviderTraitTypeMax: ETW_PROVIDER_TRAIT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_PTVECTOR_TYPE_VALUE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REDUCED_STRING_TYPE_VALUE: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REFRENCE_TYPE_VALUE: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SBYTE_TYPE_VALUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SID_TYPE_VALUE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SINGLE_TYPE_VALUE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SIZET_TYPE_VALUE: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_STRING_TYPE_VALUE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: ::windows_sys::core::GUID,
    pub ParentId: ::windows_sys::core::GUID,
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
}
impl ::core::marker::Copy for ETW_TRACE_PARTITION_INFORMATION {}
impl ::core::clone::Clone for ETW_TRACE_PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
    pub PartitionId: ::windows_sys::core::PWSTR,
    pub ParentId: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for ETW_TRACE_PARTITION_INFORMATION_V2 {}
impl ::core::clone::Clone for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_UINT16_TYPE_VALUE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_UINT32_TYPE_VALUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_UINT64_TYPE_VALUE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_VARIANT_TYPE_VALUE: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_WMITIME_TYPE_VALUE: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type EVENTSECURITYOPERATION = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecuritySetDACL: EVENTSECURITYOPERATION = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecuritySetSACL: EVENTSECURITYOPERATION = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecurityAddDACL: EVENTSECURITYOPERATION = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecurityAddSACL: EVENTSECURITYOPERATION = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecurityMax: EVENTSECURITYOPERATION = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_GET_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_SET_ID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
impl ::core::marker::Copy for EVENT_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for EVENT_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for EVENT_DATA_DESCRIPTOR_0 {}
impl ::core::clone::Clone for EVENT_DATA_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for EVENT_DATA_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for EVENT_DATA_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
impl ::core::marker::Copy for EVENT_DESCRIPTOR {}
impl ::core::clone::Clone for EVENT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_ENABLE_SILOS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_SID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_TS_ID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    pub Key: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_EVENT_KEY {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_INSTANCE {
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_INSTANCE {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    pub PebsIndex: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_PEBS_INDEX {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    pub Counter: [u64; 1],
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    pub ProcessStartKey: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    pub RelatedActivityId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY32 {
    pub MatchId: u64,
    pub StackKey: u32,
    pub Padding: u32,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_KEY32 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY64 {
    pub MatchId: u64,
    pub StackKey: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_KEY64 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    pub MatchId: u64,
    pub Address: [u32; 1],
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    pub MatchId: u64,
    pub Address: [u64; 1],
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_TS_ID {
    pub SessionId: u32,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_TS_ID {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_TS_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type EVENT_FIELD_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventKeywordInformation: EVENT_FIELD_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventLevelInformation: EVENT_FIELD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventChannelInformation: EVENT_FIELD_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventTaskInformation: EVENT_FIELD_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventInformationMax: EVENT_FIELD_TYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
impl ::core::marker::Copy for EVENT_FILTER_DESCRIPTOR {}
impl ::core::clone::Clone for EVENT_FILTER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub Count: u16,
    pub Events: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_FILTER_EVENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub NameCount: u16,
    pub Names: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_FILTER_EVENT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_FILTER_HEADER {
    pub Id: u16,
    pub Version: u8,
    pub Reserved: [u8; 5],
    pub InstanceId: u64,
    pub Size: u32,
    pub NextOffset: u32,
}
impl ::core::marker::Copy for EVENT_FILTER_HEADER {}
impl ::core::clone::Clone for EVENT_FILTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_FILTER_LEVEL_KW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: u32 = 2147483680u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PACKAGE_ID: u32 = 2147483664u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PAYLOAD: u32 = 2147483904u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PID: u32 = 2147483652u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_SCHEMATIZED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_STACKWALK: u32 = 2147487744u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_STACKWALK_LEVEL_KW: u32 = 2147500032u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_STACKWALK_NAME: u32 = 2147491840u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_TRACEHANDLE: u32 = 2147483650u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: ::windows_sys::core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for EVENT_HEADER {}
impl ::core::clone::Clone for EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
impl ::core::marker::Copy for EVENT_HEADER_0 {}
impl ::core::clone::Clone for EVENT_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl ::core::marker::Copy for EVENT_HEADER_0_0 {}
impl ::core::clone::Clone for EVENT_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
impl ::core::marker::Copy for EVENT_HEADER_EXTENDED_DATA_ITEM {}
impl ::core::clone::Clone for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
impl ::core::clone::Clone for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_CONTAINER_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_MAX: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_QPC_DELTA: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_SID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY32: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY64: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_TS_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_DECODE_GUID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_NO_CPUTIME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_STRING_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_XML: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type EVENT_INFO_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderSetTraits: EVENT_INFO_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MaxEventInfo: EVENT_INFO_CLASS = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
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
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_INSTANCE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_INSTANCE_HEADER_0_0,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_0_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_INSTANCE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_INSTANCE_HEADER_1_0,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_1 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_1_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous1: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_2 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_2_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_2_1 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::super::super::Foundation::HANDLE,
    pub InstanceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_INSTANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_LOGGER_NAME: &str = "EventLog";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_LOGGER_NAMEA: &str = "EventLog";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_LOGGER_NAMEW: &str = "EventLog";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: u32,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
impl ::core::marker::Copy for EVENT_MAP_ENTRY {}
impl ::core::clone::Clone for EVENT_MAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: u32,
    pub InputOffset: u32,
}
impl ::core::marker::Copy for EVENT_MAP_ENTRY_0 {}
impl ::core::clone::Clone for EVENT_MAP_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_MAP_INFO {
    pub NameOffset: u32,
    pub Flag: MAP_FLAGS,
    pub EntryCount: u32,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
impl ::core::marker::Copy for EVENT_MAP_INFO {}
impl ::core::clone::Clone for EVENT_MAP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: u32,
}
impl ::core::marker::Copy for EVENT_MAP_INFO_0 {}
impl ::core::clone::Clone for EVENT_MAP_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_MAX_LEVEL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_MIN_LEVEL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous1: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_1,
    pub structType: EVENT_PROPERTY_INFO_0_2,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_0,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0_0 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0_1 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0_2 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_1 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_2 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_3 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_3_0 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: *mut EVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut ::core::ffi::c_void,
    pub UserContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for EVENT_RECORD {}
impl ::core::clone::Clone for EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows_sys::core::GUID,
    pub MofData: *mut ::core::ffi::c_void,
    pub MofLength: u32,
    pub Anonymous: EVENT_TRACE_0,
}
impl ::core::marker::Copy for EVENT_TRACE {}
impl ::core::clone::Clone for EVENT_TRACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_0 {
    pub ClientContext: u32,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
impl ::core::marker::Copy for EVENT_TRACE_0 {}
impl ::core::clone::Clone for EVENT_TRACE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_ADD_HEADER_MODE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_BUFFERING_MODE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_COMPRESSED_MODE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type EVENT_TRACE_CONTROL = u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_FLUSH: EVENT_TRACE_CONTROL = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_QUERY: EVENT_TRACE_CONTROL = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_STOP: EVENT_TRACE_CONTROL = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_UPDATE: EVENT_TRACE_CONTROL = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_APPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_NEWFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type EVENT_TRACE_FLAG = u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_ALPC: EVENT_TRACE_FLAG = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_CSWITCH: EVENT_TRACE_FLAG = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DBGPRINT: EVENT_TRACE_FLAG = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: EVENT_TRACE_FLAG = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISK_IO: EVENT_TRACE_FLAG = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: EVENT_TRACE_FLAG = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISPATCHER: EVENT_TRACE_FLAG = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DPC: EVENT_TRACE_FLAG = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DRIVER: EVENT_TRACE_FLAG = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_FILE_IO: EVENT_TRACE_FLAG = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: EVENT_TRACE_FLAG = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: EVENT_TRACE_FLAG = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_INTERRUPT: EVENT_TRACE_FLAG = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_JOB: EVENT_TRACE_FLAG = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: EVENT_TRACE_FLAG = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: EVENT_TRACE_FLAG = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: EVENT_TRACE_FLAG = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: EVENT_TRACE_FLAG = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_PROCESS: EVENT_TRACE_FLAG = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: EVENT_TRACE_FLAG = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_PROFILE: EVENT_TRACE_FLAG = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_REGISTRY: EVENT_TRACE_FLAG = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_SPLIT_IO: EVENT_TRACE_FLAG = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_SYSTEMCALL: EVENT_TRACE_FLAG = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_THREAD: EVENT_TRACE_FLAG = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_VAMAP: EVENT_TRACE_FLAG = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: EVENT_TRACE_FLAG = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_FORWARD_WMI: u32 = 1073741824u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
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
impl ::core::marker::Copy for EVENT_TRACE_HEADER {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_0_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_1 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_1_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: ::windows_sys::core::GUID,
    pub GuidPtr: u64,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_2 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous1: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_3 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_3_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_3_1 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: u32 = 134217728u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct EVENT_TRACE_LOGFILEA {
    pub LogFileName: ::windows_sys::core::PSTR,
    pub LoggerName: ::windows_sys::core::PSTR,
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
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: ::windows_sys::core::PWSTR,
    pub LoggerName: ::windows_sys::core::PWSTR,
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
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_MODE_RESERVED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_NONSTOPPABLE_MODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_PRIVATE_IN_PROC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_REAL_TIME_MODE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_RELOG_MODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_SECURE_MODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACCEPT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACKDUP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACKFULL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACKPART: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CHECKPOINT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_CPU: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DPI: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NIC: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PNP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_POWER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONNECT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONNFAIL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_COPY_ARP: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_COPY_TCP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DBGID_RSDS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DC_END: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DC_START: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DEQUEUE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DISCONNECT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_EXTENSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: u32 = 98u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_GUIDMAP: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_INFO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_FLUSH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_READ: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_READ_INIT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_WRITE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_LOAD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_AV: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_COW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_DZF: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_GPF: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_HPF: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_TF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RECEIVE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RECONNECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGCLOSE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGCOMMIT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGCREATE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGDELETE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGFLUSH: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBCREATE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBDELETE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGOPEN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGPREPARE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGROLLBACK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGSETSECURITY: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGSETVALUE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REPLY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RESUME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RETRANSMIT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SECURITY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SEND: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SIDINFO: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SUSPEND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_TERMINATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_WINEVT_SEND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_NOCPUTIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_PAGED_MEMORY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_PROCTIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_WRITE_FLAG_INPRIVATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_WRITE_FLAG_NO_FAULTING: u32 = 1u32;
pub const EventTraceConfigGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 25508453, data2: 16783, data3: 20278, data4: [174, 252, 220, 15, 29, 47, 210, 53] };
pub const EventTraceGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1761466624, data2: 19006, data3: 4561, data4: [132, 244, 0, 0, 248, 4, 100, 227] };
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const GLOBAL_LOGGER_NAME: &str = "GlobalLogger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const GLOBAL_LOGGER_NAMEA: &str = "GlobalLogger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const GLOBAL_LOGGER_NAMEW: &str = "GlobalLogger";
pub type ITraceEvent = *mut ::core::ffi::c_void;
pub type ITraceEventCallback = *mut ::core::ffi::c_void;
pub type ITraceRelogger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const KERNEL_LOGGER_NAME: &str = "NT Kernel Logger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const KERNEL_LOGGER_NAMEA: &str = "NT Kernel Logger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const KERNEL_LOGGER_NAMEW: &str = "NT Kernel Logger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type MAP_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP: MAP_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_MANIFEST_BITMAP: MAP_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP: MAP_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_VALUEMAP: MAP_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_BITMAP: MAP_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_FLAG: MAP_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_NO_MAP: MAP_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type MAP_VALUETYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_ENTRY_VALUETYPE_ULONG: MAP_VALUETYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_ENTRY_VALUETYPE_STRING: MAP_VALUETYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_DATA_DESCRIPTORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTERS_COUNT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_DATA_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_PID_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_MOF_FIELDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_PAYLOAD_PREDICATES: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct MOF_FIELD {
    pub DataPtr: u64,
    pub Length: u32,
    pub DataType: u32,
}
impl ::core::marker::Copy for MOF_FIELD {}
impl ::core::clone::Clone for MOF_FIELD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
impl ::core::marker::Copy for OFFSETINSTANCEDATAANDLENGTH {}
impl ::core::clone::Clone for OFFSETINSTANCEDATAANDLENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: ::windows_sys::core::PWSTR,
    pub CompareOp: u16,
    pub Value: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PAYLOAD_FILTER_PREDICATE {}
impl ::core::clone::Clone for PAYLOAD_FILTER_PREDICATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PAYLOAD_OPERATOR = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_EQ: PAYLOAD_OPERATOR = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_NE: PAYLOAD_OPERATOR = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_LE: PAYLOAD_OPERATOR = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_GT: PAYLOAD_OPERATOR = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_LT: PAYLOAD_OPERATOR = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_GE: PAYLOAD_OPERATOR = 5i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_BETWEEN: PAYLOAD_OPERATOR = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_NOTBETWEEN: PAYLOAD_OPERATOR = 7i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_MODULO: PAYLOAD_OPERATOR = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_CONTAINS: PAYLOAD_OPERATOR = 20i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_DOESNTCONTAIN: PAYLOAD_OPERATOR = 21i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_IS: PAYLOAD_OPERATOR = 30i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_ISNOT: PAYLOAD_OPERATOR = 31i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_INVALID: PAYLOAD_OPERATOR = 32i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PENABLECALLBACK = ::core::option::Option<unsafe extern "system" fn(sourceid: *const ::windows_sys::core::GUID, isenabled: ENABLECALLBACK_ENABLED_STATE, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PEVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pevent: *mut EVENT_TRACE)>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PEVENT_RECORD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(eventrecord: *mut EVENT_RECORD)>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEA) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEW) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PROCESS_TRACE_MODE_EVENT_RECORD: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PROCESS_TRACE_MODE_REAL_TIME: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROFILE_SOURCE_INFO {
    pub NextEntryOffset: u32,
    pub Source: u32,
    pub MinInterval: u32,
    pub MaxInterval: u32,
    pub Reserved: u64,
    pub Description: [u16; 1],
}
impl ::core::marker::Copy for PROFILE_SOURCE_INFO {}
impl ::core::clone::Clone for PROFILE_SOURCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROPERTY_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for PROPERTY_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PROPERTY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyStruct: PROPERTY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamLength: PROPERTY_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamCount: PROPERTY_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamFixedLength: PROPERTY_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamFixedCount: PROPERTY_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyHasTags: PROPERTY_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = 128i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: u32,
    pub Reserved: u32,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
impl ::core::marker::Copy for PROVIDER_ENUMERATION_INFO {}
impl ::core::clone::Clone for PROVIDER_ENUMERATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: u32,
    pub Reserved: u32,
    pub EventDescriptorsArray: [EVENT_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for PROVIDER_EVENT_INFO {}
impl ::core::clone::Clone for PROVIDER_EVENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: u32,
    pub DescriptionOffset: u32,
    pub Value: u64,
}
impl ::core::marker::Copy for PROVIDER_FIELD_INFO {}
impl ::core::clone::Clone for PROVIDER_FIELD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: u32,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
impl ::core::marker::Copy for PROVIDER_FIELD_INFOARRAY {}
impl ::core::clone::Clone for PROVIDER_FIELD_INFOARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_FILTER_INFO {
    pub Id: u8,
    pub Version: u8,
    pub MessageOffset: u32,
    pub Reserved: u32,
    pub PropertyCount: u32,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl ::core::marker::Copy for PROVIDER_FILTER_INFO {}
impl ::core::clone::Clone for PROVIDER_FILTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PrivateLoggerNotificationGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 899001180, data2: 1066, data3: 19598, data4: [185, 66, 45, 5, 155, 254, 177, 177] };
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_ALPC_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_GRAPHICS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_NETWORK: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_OPTICAL: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_PNP: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_SERVICES: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_STORAGE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_SYSTEM: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CPU_KW_CACHE_FLUSH: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CPU_KW_CONFIG: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CPU_KW_SPEC_CONTROL: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_EVENT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_HYPERVISOR_KW_PROFILE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_DPC: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_IPI: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_FAILURE: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_FASTIO: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_INIT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_CC: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_DISK: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_DISK_INIT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_DRIVERS: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_FILE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_FILENAME: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_NETWORK: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_OPTICAL: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_OPTICAL_INIT: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_SPLIT: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_LOCK_KW_SPINLOCK: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_FOOTPRINT: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_HEAP: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_MEMINFO: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: u64 = 32768u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_PFSECTION: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_POOL: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_REFSET: u64 = 8192u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_SESSION: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_VAMAP: u64 = 16384u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_WS: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_POOL_FILTER_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_OBJECT_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_OBJECT_KW_HANDLE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_IDLE_SELECTION: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_DBGPRINT: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_FREEZE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_INSWAP: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_JOB: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_LOADER: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_THREAD: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WAKE_DROP: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROFILE_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_REGISTRY_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_REGISTRY_KW_HIVE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_AFFINITY: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_PRIORITY: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SYSCALL_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_TIMER_KW_GENERAL: u64 = 1u64;
pub const SystemAlpcProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4240030383, data2: 58665, data3: 18816, data4: [146, 233, 206, 209, 166, 170, 223, 223] };
pub const SystemConfigProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4277381302, data2: 12685, data3: 19303, data4: [169, 106, 59, 15, 107, 143, 24, 254] };
pub const SystemCpuProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3334809183, data2: 60136, data3: 18000, data4: [170, 228, 157, 72, 96, 61, 133, 16] };
pub const SystemHypervisorProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3136948010, data2: 37258, data3: 19437, data4: [182, 34, 188, 21, 32, 151, 9, 143] };
pub const SystemInterruptProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3569085975, data2: 46405, data3: 18568, data4: [133, 139, 116, 65, 105, 1, 91, 37] };
pub const SystemIoFilterProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4224750435, data2: 40482, data3: 18017, data4: [184, 191, 231, 163, 75, 83, 91, 140] };
pub const SystemIoProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029456867, data2: 3868, data3: 16898, data4: [184, 23, 23, 76, 0, 112, 220, 121] };
pub const SystemLockProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1914560467, data2: 56012, data3: 19998, data4: [178, 106, 162, 203, 49, 212, 112, 90] };
pub const SystemMemoryProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2190838953, data2: 46797, data3: 18424, data4: [163, 168, 3, 174, 133, 164, 188, 36] };
pub const SystemObjectProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4273828960, data2: 15645, data3: 18411, data4: [175, 73, 201, 238, 177, 225, 70, 242] };
pub const SystemPowerProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3241445450, data2: 13013, data3: 17544, data4: [128, 229, 20, 237, 122, 187, 130, 105] };
pub const SystemProcessProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 354375132, data2: 18045, data3: 18207, data4: [131, 181, 95, 136, 157, 70, 255, 102] };
pub const SystemProfileProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3219850020, data2: 7406, data3: 18799, data4: [164, 9, 42, 194, 180, 138, 99, 34] };
pub const SystemRegistryProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 370502617, data2: 64180, data3: 19706, data4: [162, 50, 137, 209, 9, 144, 88, 227] };
pub const SystemSchedulerProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1503275638, data2: 19857, data3: 18704, data4: [154, 199, 125, 51, 242, 233, 122, 108] };
pub const SystemSyscallProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1128433399, data2: 28443, data3: 17851, data4: [179, 126, 149, 246, 35, 4, 108, 124] };
pub const SystemTimerProviderGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1325798760, data2: 57877, data3: 18847, data4: [171, 46, 237, 160, 174, 137, 10, 91] };
pub const SystemTraceControlGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2659273389, data2: 12804, data3: 4562, data4: [154, 130, 0, 96, 8, 168, 105, 57] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
impl ::core::marker::Copy for TDH_CONTEXT {}
impl ::core::clone::Clone for TDH_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type TDH_CONTEXT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = 5i32;
pub type TDH_HANDLE = isize;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type TEMPLATE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_ACCESS_KERNEL_LOGGER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_ACCESS_REALTIME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_CREATE_INPROC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_CREATE_ONDISK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_CREATE_REALTIME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_GUID_ENABLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_JOIN_GROUP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_LOG_EVENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_REGISTER_GUIDS: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
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
impl ::core::marker::Copy for TRACE_ENABLE_INFO {}
impl ::core::clone::Clone for TRACE_ENABLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: ::windows_sys::core::GUID,
    pub EventGuid: ::windows_sys::core::GUID,
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
impl ::core::marker::Copy for TRACE_EVENT_INFO {}
impl ::core::clone::Clone for TRACE_EVENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_0 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_1 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_2 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_2_0 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for TRACE_GUID_INFO {}
impl ::core::clone::Clone for TRACE_GUID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: ::windows_sys::core::GUID,
    pub GuidType: u32,
    pub LoggerId: u32,
    pub EnableLevel: u32,
    pub EnableFlags: u32,
    pub IsEnable: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACE_GUID_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: *const ::windows_sys::core::GUID,
    pub RegHandle: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACE_GUID_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_LOG_WNODE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_TRACED_GUID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_CRITICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_FATAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_INFORMATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_VERBOSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_WARNING: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
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
    pub LoggerName: ::windows_sys::core::PWSTR,
    pub LogFileName: ::windows_sys::core::PWSTR,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: ::windows_sys::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: ::windows_sys::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: ::windows_sys::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type TRACE_MESSAGE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_COMPONENTID: TRACE_MESSAGE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_GUID: TRACE_MESSAGE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_SEQUENCE: TRACE_MESSAGE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_SYSTEMINFO: TRACE_MESSAGE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_TIMESTAMP: TRACE_MESSAGE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_FLAG_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_POINTER32: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_POINTER64: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: u32,
    pub ProviderCount: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for TRACE_PERIODIC_CAPTURE_STATE_INFO {}
impl ::core::clone::Clone for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: u32,
    pub Interval: u32,
}
impl ::core::marker::Copy for TRACE_PROFILE_INTERVAL {}
impl ::core::clone::Clone for TRACE_PROFILE_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_PROVIDER_FLAG_LEGACY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: ::windows_sys::core::GUID,
    pub SchemaSource: u32,
    pub ProviderNameOffset: u32,
}
impl ::core::marker::Copy for TRACE_PROVIDER_INFO {}
impl ::core::clone::Clone for TRACE_PROVIDER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: u32,
    pub EnableCount: u32,
    pub Pid: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for TRACE_PROVIDER_INSTANCE_INFO {}
impl ::core::clone::Clone for TRACE_PROVIDER_INSTANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type TRACE_QUERY_INFO_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGuidQueryList: TRACE_QUERY_INFO_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGuidQueryInfo: TRACE_QUERY_INFO_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGuidQueryProcess: TRACE_QUERY_INFO_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceStackTracingInfo: TRACE_QUERY_INFO_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceSystemTraceEnableFlagsInfo: TRACE_QUERY_INFO_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceSampledProfileIntervalInfo: TRACE_QUERY_INFO_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceProfileSourceConfigInfo: TRACE_QUERY_INFO_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceProfileSourceListInfo: TRACE_QUERY_INFO_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePmcEventListInfo: TRACE_QUERY_INFO_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePmcCounterListInfo: TRACE_QUERY_INFO_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceSetDisallowList: TRACE_QUERY_INFO_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceVersionInfo: TRACE_QUERY_INFO_CLASS = 11i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGroupQueryList: TRACE_QUERY_INFO_CLASS = 12i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGroupQueryInfo: TRACE_QUERY_INFO_CLASS = 13i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceDisallowListQuery: TRACE_QUERY_INFO_CLASS = 14i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceInfoReserved15: TRACE_QUERY_INFO_CLASS = 15i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePeriodicCaptureStateListInfo: TRACE_QUERY_INFO_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePeriodicCaptureStateInfo: TRACE_QUERY_INFO_CLASS = 17i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceProviderBinaryTracking: TRACE_QUERY_INFO_CLASS = 18i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceMaxLoggersQuery: TRACE_QUERY_INFO_CLASS = 19i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceLbrConfigurationInfo: TRACE_QUERY_INFO_CLASS = 20i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceLbrEventListInfo: TRACE_QUERY_INFO_CLASS = 21i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceMaxPmcCounterQuery: TRACE_QUERY_INFO_CLASS = 22i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceStreamCount: TRACE_QUERY_INFO_CLASS = 23i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceStackCachingInfo: TRACE_QUERY_INFO_CLASS = 24i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePmcCounterOwners: TRACE_QUERY_INFO_CLASS = 25i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceUnifiedStackCachingInfo: TRACE_QUERY_INFO_CLASS = 26i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MaxTraceSetInfoClass: TRACE_QUERY_INFO_CLASS = 27i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub CacheSize: u32,
    pub BucketCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACE_STACK_CACHING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for TRACE_VERSION_INFO {}
impl ::core::clone::Clone for TRACE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type WMIDPREQUEST = ::core::option::Option<unsafe extern "system" fn(requestcode: WMIDPREQUESTCODE, requestcontext: *const ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type WMIDPREQUESTCODE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_SET_SINGLE_INSTANCE: WMIDPREQUESTCODE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_SET_SINGLE_ITEM: WMIDPREQUESTCODE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = 5i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = 7i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_REGINFO: WMIDPREQUESTCODE = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = 9i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = 10i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_EXECUTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_NOTIFICATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_READ_DESCRIPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_SET: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct WMIREGGUIDW {
    pub Guid: ::windows_sys::core::GUID,
    pub Flags: u32,
    pub InstanceCount: u32,
    pub Anonymous: WMIREGGUIDW_0,
}
impl ::core::marker::Copy for WMIREGGUIDW {}
impl ::core::clone::Clone for WMIREGGUIDW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union WMIREGGUIDW_0 {
    pub InstanceNameList: u32,
    pub BaseNameOffset: u32,
    pub Pdo: usize,
    pub InstanceInfo: usize,
}
impl ::core::marker::Copy for WMIREGGUIDW_0 {}
impl ::core::clone::Clone for WMIREGGUIDW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct WMIREGINFOW {
    pub BufferSize: u32,
    pub NextWmiRegInfo: u32,
    pub RegistryPath: u32,
    pub MofResourceName: u32,
    pub GuidCount: u32,
    pub WmiRegGuid: [WMIREGGUIDW; 1],
}
impl ::core::marker::Copy for WMIREGINFOW {}
impl ::core::clone::Clone for WMIREGINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_EVENT_ONLY_GUID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_EXPENSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_INSTANCE_BASENAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_INSTANCE_LIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_INSTANCE_PDO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_REMOVE_GUID: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_RESERVED1: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_RESERVED2: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_TRACED_GUID: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GLOBAL_LOGGER_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_EVENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_TRACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_TRACECONTROL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_ALL_DATA {
    pub WnodeHeader: WNODE_HEADER,
    pub DataBlockOffset: u32,
    pub InstanceCount: u32,
    pub OffsetInstanceNameOffsets: u32,
    pub Anonymous: WNODE_ALL_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_ALL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_ALL_DATA_0 {
    pub FixedInstanceSize: u32,
    pub OffsetInstanceDataAndLength: [OFFSETINSTANCEDATAANDLENGTH; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_ALL_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_ALL_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_EVENT_ITEM {
    pub WnodeHeader: WNODE_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_EVENT_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_EVENT_REFERENCE {
    pub WnodeHeader: WNODE_HEADER,
    pub TargetGuid: ::windows_sys::core::GUID,
    pub TargetDataBlockSize: u32,
    pub Anonymous: WNODE_EVENT_REFERENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_EVENT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_EVENT_REFERENCE_0 {
    pub TargetInstanceIndex: u32,
    pub TargetInstanceName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_EVENT_REFERENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_EVENT_REFERENCE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_ALL_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_ANSI_INSTANCENAMES: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_EVENT_ITEM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_EVENT_REFERENCE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_INSTANCES_SAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_INTERNAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_LOG_WNODE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_METHOD_ITEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_NO_HEADER: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_PERSIST_EVENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SEND_DATA_BLOCK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SINGLE_INSTANCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SINGLE_ITEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_TOO_SMALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_TRACED_GUID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_USE_GUID_PTR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_USE_MOF_PTR: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_USE_TIMESTAMP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_VERSIONED_PROPERTIES: u32 = 8388608u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous1: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: ::windows_sys::core::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::super::super::Foundation::HANDLE,
    pub TimeStamp: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_METHOD_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub MethodId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_METHOD_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_SINGLE_INSTANCE {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_SINGLE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_SINGLE_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub ItemId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataItem: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_SINGLE_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_TOO_SMALL {
    pub WnodeHeader: WNODE_HEADER,
    pub SizeNeeded: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_TOO_SMALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type _TDH_IN_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_NULL: _TDH_IN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UNICODESTRING: _TDH_IN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_ANSISTRING: _TDH_IN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT8: _TDH_IN_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT8: _TDH_IN_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT16: _TDH_IN_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT16: _TDH_IN_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT32: _TDH_IN_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT32: _TDH_IN_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT64: _TDH_IN_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT64: _TDH_IN_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_FLOAT: _TDH_IN_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_DOUBLE: _TDH_IN_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_BOOLEAN: _TDH_IN_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_BINARY: _TDH_IN_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_GUID: _TDH_IN_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_POINTER: _TDH_IN_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_FILETIME: _TDH_IN_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_SYSTEMTIME: _TDH_IN_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_SID: _TDH_IN_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_HEXINT32: _TDH_IN_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_HEXINT64: _TDH_IN_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_MANIFEST_COUNTEDSTRING: _TDH_IN_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_MANIFEST_COUNTEDANSISTRING: _TDH_IN_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_RESERVED24: _TDH_IN_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_MANIFEST_COUNTEDBINARY: _TDH_IN_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = 300i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = 301i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = 302i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = 303i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_NONNULLTERMINATEDSTRING: _TDH_IN_TYPE = 304i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_NONNULLTERMINATEDANSISTRING: _TDH_IN_TYPE = 305i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UNICODECHAR: _TDH_IN_TYPE = 306i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_ANSICHAR: _TDH_IN_TYPE = 307i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_SIZET: _TDH_IN_TYPE = 308i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_HEXDUMP: _TDH_IN_TYPE = 309i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_WBEMSID: _TDH_IN_TYPE = 310i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type _TDH_OUT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_NULL: _TDH_OUT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_STRING: _TDH_OUT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_DATETIME: _TDH_OUT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_BYTE: _TDH_OUT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDBYTE: _TDH_OUT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_SHORT: _TDH_OUT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDSHORT: _TDH_OUT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_INT: _TDH_OUT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDINT: _TDH_OUT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_LONG: _TDH_OUT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDLONG: _TDH_OUT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_FLOAT: _TDH_OUT_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_DOUBLE: _TDH_OUT_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_BOOLEAN: _TDH_OUT_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_GUID: _TDH_OUT_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXBINARY: _TDH_OUT_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT8: _TDH_OUT_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT16: _TDH_OUT_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT32: _TDH_OUT_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT64: _TDH_OUT_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_PID: _TDH_OUT_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_TID: _TDH_OUT_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_PORT: _TDH_OUT_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_IPV4: _TDH_OUT_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_IPV6: _TDH_OUT_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_SOCKETADDRESS: _TDH_OUT_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_CIMDATETIME: _TDH_OUT_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_ETWTIME: _TDH_OUT_TYPE = 27i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_XML: _TDH_OUT_TYPE = 28i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_ERRORCODE: _TDH_OUT_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_WIN32ERROR: _TDH_OUT_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_NTSTATUS: _TDH_OUT_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HRESULT: _TDH_OUT_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME: _TDH_OUT_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_JSON: _TDH_OUT_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UTF8: _TDH_OUT_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_PKCS7_WITH_TYPE_INFO: _TDH_OUT_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_CODE_POINTER: _TDH_OUT_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_DATETIME_UTC: _TDH_OUT_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_REDUCEDSTRING: _TDH_OUT_TYPE = 300i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_NOPRINT: _TDH_OUT_TYPE = 301i32;
