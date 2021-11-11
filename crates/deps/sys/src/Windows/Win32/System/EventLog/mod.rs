#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupEventLogA(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupEventLogW(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearEventLogA(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearEventLogW(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseEventLog(heventlog: EventLogHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterEventSource(heventlog: EventSourceHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtArchiveExportedLog(session: isize, logfilepath: super::super::Foundation::PWSTR, locale: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCancel(object: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtClearLog(session: isize, channelpath: super::super::Foundation::PWSTR, targetfilepath: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtClose(object: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCreateBookmark(bookmarkxml: super::super::Foundation::PWSTR) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCreateRenderContext(valuepathscount: u32, valuepaths: *const super::super::Foundation::PWSTR, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtExportLog(session: isize, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, targetfilepath: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtFormatMessage(publishermetadata: isize, event: isize, messageid: u32, valuecount: u32, values: *const EVT_VARIANT, flags: u32, buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetEventInfo(event: isize, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetEventMetadataProperty(eventmetadata: isize, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: *mut EVT_VARIANT, eventmetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetExtendedStatus(buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetLogInfo(log: isize, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetObjectArrayProperty(objectarray: isize, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetPublisherMetadataProperty(publishermetadata: isize, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: *mut EVT_VARIANT, publishermetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetQueryInfo(queryorsubscription: isize, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNext(resultset: isize, eventssize: u32, events: *mut isize, timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNextChannelPath(channelenum: isize, channelpathbuffersize: u32, channelpathbuffer: super::super::Foundation::PWSTR, channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtNextEventMetadata(eventmetadataenum: isize, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNextPublisherId(publisherenum: isize, publisheridbuffersize: u32, publisheridbuffer: super::super::Foundation::PWSTR, publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenChannelConfig(session: isize, channelpath: super::super::Foundation::PWSTR, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenChannelEnum(session: isize, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenEventMetadataEnum(publishermetadata: isize, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenLog(session: isize, path: super::super::Foundation::PWSTR, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenPublisherEnum(session: isize, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenPublisherMetadata(session: isize, publisherid: super::super::Foundation::PWSTR, logfilepath: super::super::Foundation::PWSTR, locale: u32, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *const ::core::ffi::c_void, timeout: u32, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtQuery(session: isize, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtRender(context: isize, fragment: isize, flags: u32, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32, propertycount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSaveChannelConfig(channelconfig: isize, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSeek(resultset: isize, position: i64, bookmark: isize, timeout: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSubscribe(session: isize, signalevent: super::super::Foundation::HANDLE, channelpath: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, bookmark: isize, context: *const ::core::ffi::c_void, callback: ::windows::runtime::RawPtr, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtUpdateBookmark(bookmark: isize, event: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEventLogInformation(heventlog: super::super::Foundation::HANDLE, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfEventLogRecords(heventlog: super::super::Foundation::HANDLE, numberofrecords: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOldestEventLogRecord(heventlog: super::super::Foundation::HANDLE, oldestrecord: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyChangeEventLog(heventlog: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenBackupEventLogA(lpuncservername: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> EventLogHandle;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenBackupEventLogW(lpuncservername: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> EventLogHandle;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventLogA(lpuncservername: super::super::Foundation::PSTR, lpsourcename: super::super::Foundation::PSTR) -> EventLogHandle;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventLogW(lpuncservername: super::super::Foundation::PWSTR, lpsourcename: super::super::Foundation::PWSTR) -> EventLogHandle;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadEventLogA(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadEventLogW(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterEventSourceA(lpuncservername: super::super::Foundation::PSTR, lpsourcename: super::super::Foundation::PSTR) -> EventSourceHandle;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterEventSourceW(lpuncservername: super::super::Foundation::PWSTR, lpsourcename: super::super::Foundation::PWSTR) -> EventSourceHandle;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportEventA(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportEventW(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PWSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
