#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn CloseTrace(tracehandle: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ControlTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ControlTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTraceInstanceId(reghandle: super::super::super::Foundation::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CveEventWrite(cveid: super::super::super::Foundation::PWSTR, additionaldetails: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::runtime::GUID, tracehandle: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnableTraceEx(providerid: *const ::windows::runtime::GUID, sourceid: *const ::windows::runtime::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::runtime::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventAccessControl(guid: *const ::windows::runtime::GUID, operation: u32, sid: super::super::super::Foundation::PSID, rights: u32, allowordeny: super::super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EventAccessQuery(guid: *const ::windows::runtime::GUID, buffer: *mut super::super::super::Security::SECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventAccessRemove(guid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventRegister(providerid: *const ::windows::runtime::GUID, enablecallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void, reghandle: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventUnregister(reghandle: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::runtime::GUID, relatedactivityid: *const ::windows::runtime::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventWriteString(reghandle: u64, level: u8, keyword: u64, string: super::super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::runtime::GUID, relatedactivityid: *const ::windows::runtime::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn GetTraceEnableFlags(tracehandle: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn GetTraceEnableLevel(tracehandle: u64) -> u8;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn GetTraceLoggerHandle(buffer: *const ::core::ffi::c_void) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
    pub fn OpenTraceA(logfile: *mut ::core::mem::ManuallyDrop<EVENT_TRACE_LOGFILEA>) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
    pub fn OpenTraceW(logfile: *mut ::core::mem::ManuallyDrop<EVENT_TRACE_LOGFILEW>) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTraceGuidsA(requestaddress: ::windows::runtime::RawPtr, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::runtime::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: super::super::super::Foundation::PSTR, mofresourcename: super::super::super::Foundation::PSTR, registrationhandle: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTraceGuidsW(requestaddress: ::windows::runtime::RawPtr, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::runtime::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: super::super::super::Foundation::PWSTR, mofresourcename: super::super::super::Foundation::PWSTR, registrationhandle: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn RemoveTraceCallback(pguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn SetTraceCallback(pguid: *const ::windows::runtime::GUID, eventcallback: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartTraceA(tracehandle: *mut u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartTraceW(tracehandle: *mut u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StopTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StopTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::core::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhCreatePayloadFilter(providerguid: *const ::windows::runtime::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: super::super::super::Foundation::BOOLEAN, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::runtime::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::runtime::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProviderFilters(guid: *const ::windows::runtime::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: super::super::super::Foundation::PWSTR, userdataconsumed: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhGetEventMapInformation(pevent: *const EVENT_RECORD, pmapname: super::super::super::Foundation::PWSTR, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetManifestEventInformation(providerguid: *const ::windows::runtime::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhGetWppProperty(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, propertyname: super::super::super::Foundation::PWSTR, buffersize: *mut u32, buffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhLoadManifest(manifest: super::super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhLoadManifestFromBinary(binarypath: super::super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhLoadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhQueryProviderFieldInformation(pguid: *const ::windows::runtime::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhUnloadManifest(manifest: super::super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhUnloadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::runtime::GUID, messagenumber: u16) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::runtime::GUID, messagenumber: u16, messagearglist: *const i8) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::core::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn UnregisterTraceGuids(registrationhandle: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
}
