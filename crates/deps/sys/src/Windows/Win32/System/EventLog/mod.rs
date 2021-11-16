#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupEventLogA(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupEventLogW(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearEventLogA(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearEventLogW(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseEventLog(heventlog: EventLogHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterEventSource(heventlog: EventSourceHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtArchiveExportedLog(session: isize, logfilepath: super::super::Foundation::PWSTR, locale: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCancel(object: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtClearLog(session: isize, channelpath: super::super::Foundation::PWSTR, targetfilepath: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtClose(object: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCreateBookmark(bookmarkxml: super::super::Foundation::PWSTR) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCreateRenderContext(valuepathscount: u32, valuepaths: *const super::super::Foundation::PWSTR, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtExportLog(session: isize, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, targetfilepath: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtFormatMessage(publishermetadata: isize, event: isize, messageid: u32, valuecount: u32, values: *const EVT_VARIANT, flags: u32, buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetEventInfo(event: isize, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetEventMetadataProperty(eventmetadata: isize, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: *mut EVT_VARIANT, eventmetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetExtendedStatus(buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetLogInfo(log: isize, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetObjectArrayProperty(objectarray: isize, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetPublisherMetadataProperty(publishermetadata: isize, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: *mut EVT_VARIANT, publishermetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetQueryInfo(queryorsubscription: isize, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNext(resultset: isize, eventssize: u32, events: *mut isize, timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNextChannelPath(channelenum: isize, channelpathbuffersize: u32, channelpathbuffer: super::super::Foundation::PWSTR, channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL;
    pub fn EvtNextEventMetadata(eventmetadataenum: isize, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNextPublisherId(publisherenum: isize, publisheridbuffersize: u32, publisheridbuffer: super::super::Foundation::PWSTR, publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenChannelConfig(session: isize, channelpath: super::super::Foundation::PWSTR, flags: u32) -> isize;
    pub fn EvtOpenChannelEnum(session: isize, flags: u32) -> isize;
    pub fn EvtOpenEventMetadataEnum(publishermetadata: isize, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenLog(session: isize, path: super::super::Foundation::PWSTR, flags: u32) -> isize;
    pub fn EvtOpenPublisherEnum(session: isize, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenPublisherMetadata(session: isize, publisherid: super::super::Foundation::PWSTR, logfilepath: super::super::Foundation::PWSTR, locale: u32, flags: u32) -> isize;
    pub fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *const ::core::ffi::c_void, timeout: u32, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtQuery(session: isize, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtRender(context: isize, fragment: isize, flags: u32, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32, propertycount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSaveChannelConfig(channelconfig: isize, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSeek(resultset: isize, position: i64, bookmark: isize, timeout: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSubscribe(session: isize, signalevent: super::super::Foundation::HANDLE, channelpath: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, bookmark: isize, context: *const ::core::ffi::c_void, callback: EVT_SUBSCRIBE_CALLBACK, flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtUpdateBookmark(bookmark: isize, event: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEventLogInformation(heventlog: super::super::Foundation::HANDLE, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfEventLogRecords(heventlog: super::super::Foundation::HANDLE, numberofrecords: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOldestEventLogRecord(heventlog: super::super::Foundation::HANDLE, oldestrecord: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyChangeEventLog(heventlog: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenBackupEventLogA(lpuncservername: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> EventLogHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenBackupEventLogW(lpuncservername: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> EventLogHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventLogA(lpuncservername: super::super::Foundation::PSTR, lpsourcename: super::super::Foundation::PSTR) -> EventLogHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventLogW(lpuncservername: super::super::Foundation::PWSTR, lpsourcename: super::super::Foundation::PWSTR) -> EventLogHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadEventLogA(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadEventLogW(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterEventSourceA(lpuncservername: super::super::Foundation::PSTR, lpsourcename: super::super::Foundation::PSTR) -> EventSourceHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterEventSourceW(lpuncservername: super::super::Foundation::PWSTR, lpsourcename: super::super::Foundation::PWSTR) -> EventSourceHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportEventA(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportEventW(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PWSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct EVENTLOGRECORD {
    pub Length: u32,
    pub Reserved: u32,
    pub RecordNumber: u32,
    pub TimeGenerated: u32,
    pub TimeWritten: u32,
    pub EventID: u32,
    pub EventType: REPORT_EVENT_TYPE,
    pub NumStrings: u16,
    pub EventCategory: u16,
    pub ReservedFlags: u16,
    pub ClosingRecordNumber: u32,
    pub StringOffset: u32,
    pub UserSidLength: u32,
    pub UserSidOffset: u32,
    pub DataLength: u32,
    pub DataOffset: u32,
}
impl ::core::marker::Copy for EVENTLOGRECORD {}
impl ::core::clone::Clone for EVENTLOGRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
impl ::core::marker::Copy for EVENTLOG_FULL_INFORMATION {}
impl ::core::clone::Clone for EVENTLOG_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EVENTSFORLOGFILE {
    pub ulSize: u32,
    pub szLogicalLogFile: [u16; 256],
    pub ulNumRecords: u32,
    pub pEventLogRecords: [EVENTLOGRECORD; 1],
}
impl ::core::marker::Copy for EVENTSFORLOGFILE {}
impl ::core::clone::Clone for EVENTSFORLOGFILE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EVT_ALL_ACCESS: u32 = 7u32;
pub const EvtChannelClockTypeSystemTime: i32 = 0i32;
pub const EvtChannelClockTypeQPC: i32 = 1i32;
pub const EvtChannelConfigEnabled: i32 = 0i32;
pub const EvtChannelConfigIsolation: i32 = 1i32;
pub const EvtChannelConfigType: i32 = 2i32;
pub const EvtChannelConfigOwningPublisher: i32 = 3i32;
pub const EvtChannelConfigClassicEventlog: i32 = 4i32;
pub const EvtChannelConfigAccess: i32 = 5i32;
pub const EvtChannelLoggingConfigRetention: i32 = 6i32;
pub const EvtChannelLoggingConfigAutoBackup: i32 = 7i32;
pub const EvtChannelLoggingConfigMaxSize: i32 = 8i32;
pub const EvtChannelLoggingConfigLogFilePath: i32 = 9i32;
pub const EvtChannelPublishingConfigLevel: i32 = 10i32;
pub const EvtChannelPublishingConfigKeywords: i32 = 11i32;
pub const EvtChannelPublishingConfigControlGuid: i32 = 12i32;
pub const EvtChannelPublishingConfigBufferSize: i32 = 13i32;
pub const EvtChannelPublishingConfigMinBuffers: i32 = 14i32;
pub const EvtChannelPublishingConfigMaxBuffers: i32 = 15i32;
pub const EvtChannelPublishingConfigLatency: i32 = 16i32;
pub const EvtChannelPublishingConfigClockType: i32 = 17i32;
pub const EvtChannelPublishingConfigSidType: i32 = 18i32;
pub const EvtChannelPublisherList: i32 = 19i32;
pub const EvtChannelPublishingConfigFileMax: i32 = 20i32;
pub const EvtChannelConfigPropertyIdEND: i32 = 21i32;
pub const EvtChannelIsolationTypeApplication: i32 = 0i32;
pub const EvtChannelIsolationTypeSystem: i32 = 1i32;
pub const EvtChannelIsolationTypeCustom: i32 = 2i32;
pub const EvtChannelReferenceImported: i32 = 1i32;
pub const EvtChannelSidTypeNone: i32 = 0i32;
pub const EvtChannelSidTypePublishing: i32 = 1i32;
pub const EvtChannelTypeAdmin: i32 = 0i32;
pub const EvtChannelTypeOperational: i32 = 1i32;
pub const EvtChannelTypeAnalytic: i32 = 2i32;
pub const EvtChannelTypeDebug: i32 = 3i32;
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
pub const EventMetadataEventID: i32 = 0i32;
pub const EventMetadataEventVersion: i32 = 1i32;
pub const EventMetadataEventChannel: i32 = 2i32;
pub const EventMetadataEventLevel: i32 = 3i32;
pub const EventMetadataEventOpcode: i32 = 4i32;
pub const EventMetadataEventTask: i32 = 5i32;
pub const EventMetadataEventKeyword: i32 = 6i32;
pub const EventMetadataEventMessageID: i32 = 7i32;
pub const EventMetadataEventTemplate: i32 = 8i32;
pub const EvtEventMetadataPropertyIdEND: i32 = 9i32;
pub const EvtEventQueryIDs: i32 = 0i32;
pub const EvtEventPath: i32 = 1i32;
pub const EvtEventPropertyIdEND: i32 = 2i32;
pub const EvtExportLogChannelPath: i32 = 1i32;
pub const EvtExportLogFilePath: i32 = 2i32;
pub const EvtExportLogTolerateQueryErrors: i32 = 4096i32;
pub const EvtExportLogOverwrite: i32 = 8192i32;
pub const EvtFormatMessageEvent: i32 = 1i32;
pub const EvtFormatMessageLevel: i32 = 2i32;
pub const EvtFormatMessageTask: i32 = 3i32;
pub const EvtFormatMessageOpcode: i32 = 4i32;
pub const EvtFormatMessageKeyword: i32 = 5i32;
pub const EvtFormatMessageChannel: i32 = 6i32;
pub const EvtFormatMessageProvider: i32 = 7i32;
pub const EvtFormatMessageId: i32 = 8i32;
pub const EvtFormatMessageXml: i32 = 9i32;
pub const EvtRpcLogin: i32 = 1i32;
pub const EvtLogCreationTime: i32 = 0i32;
pub const EvtLogLastAccessTime: i32 = 1i32;
pub const EvtLogLastWriteTime: i32 = 2i32;
pub const EvtLogFileSize: i32 = 3i32;
pub const EvtLogAttributes: i32 = 4i32;
pub const EvtLogNumberOfLogRecords: i32 = 5i32;
pub const EvtLogOldestRecordNumber: i32 = 6i32;
pub const EvtLogFull: i32 = 7i32;
pub const EvtOpenChannelPath: i32 = 1i32;
pub const EvtOpenFilePath: i32 = 2i32;
pub const EvtPublisherMetadataPublisherGuid: i32 = 0i32;
pub const EvtPublisherMetadataResourceFilePath: i32 = 1i32;
pub const EvtPublisherMetadataParameterFilePath: i32 = 2i32;
pub const EvtPublisherMetadataMessageFilePath: i32 = 3i32;
pub const EvtPublisherMetadataHelpLink: i32 = 4i32;
pub const EvtPublisherMetadataPublisherMessageID: i32 = 5i32;
pub const EvtPublisherMetadataChannelReferences: i32 = 6i32;
pub const EvtPublisherMetadataChannelReferencePath: i32 = 7i32;
pub const EvtPublisherMetadataChannelReferenceIndex: i32 = 8i32;
pub const EvtPublisherMetadataChannelReferenceID: i32 = 9i32;
pub const EvtPublisherMetadataChannelReferenceFlags: i32 = 10i32;
pub const EvtPublisherMetadataChannelReferenceMessageID: i32 = 11i32;
pub const EvtPublisherMetadataLevels: i32 = 12i32;
pub const EvtPublisherMetadataLevelName: i32 = 13i32;
pub const EvtPublisherMetadataLevelValue: i32 = 14i32;
pub const EvtPublisherMetadataLevelMessageID: i32 = 15i32;
pub const EvtPublisherMetadataTasks: i32 = 16i32;
pub const EvtPublisherMetadataTaskName: i32 = 17i32;
pub const EvtPublisherMetadataTaskEventGuid: i32 = 18i32;
pub const EvtPublisherMetadataTaskValue: i32 = 19i32;
pub const EvtPublisherMetadataTaskMessageID: i32 = 20i32;
pub const EvtPublisherMetadataOpcodes: i32 = 21i32;
pub const EvtPublisherMetadataOpcodeName: i32 = 22i32;
pub const EvtPublisherMetadataOpcodeValue: i32 = 23i32;
pub const EvtPublisherMetadataOpcodeMessageID: i32 = 24i32;
pub const EvtPublisherMetadataKeywords: i32 = 25i32;
pub const EvtPublisherMetadataKeywordName: i32 = 26i32;
pub const EvtPublisherMetadataKeywordValue: i32 = 27i32;
pub const EvtPublisherMetadataKeywordMessageID: i32 = 28i32;
pub const EvtPublisherMetadataPropertyIdEND: i32 = 29i32;
pub const EvtQueryChannelPath: i32 = 1i32;
pub const EvtQueryFilePath: i32 = 2i32;
pub const EvtQueryForwardDirection: i32 = 256i32;
pub const EvtQueryReverseDirection: i32 = 512i32;
pub const EvtQueryTolerateQueryErrors: i32 = 4096i32;
pub const EvtQueryNames: i32 = 0i32;
pub const EvtQueryStatuses: i32 = 1i32;
pub const EvtQueryPropertyIdEND: i32 = 2i32;
pub const EVT_READ_ACCESS: u32 = 1u32;
pub const EvtRenderContextValues: i32 = 0i32;
pub const EvtRenderContextSystem: i32 = 1i32;
pub const EvtRenderContextUser: i32 = 2i32;
pub const EvtRenderEventValues: i32 = 0i32;
pub const EvtRenderEventXml: i32 = 1i32;
pub const EvtRenderBookmark: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVT_RPC_LOGIN {
    pub Server: super::super::Foundation::PWSTR,
    pub User: super::super::Foundation::PWSTR,
    pub Domain: super::super::Foundation::PWSTR,
    pub Password: super::super::Foundation::PWSTR,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVT_RPC_LOGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVT_RPC_LOGIN {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EvtRpcLoginAuthDefault: i32 = 0i32;
pub const EvtRpcLoginAuthNegotiate: i32 = 1i32;
pub const EvtRpcLoginAuthKerberos: i32 = 2i32;
pub const EvtRpcLoginAuthNTLM: i32 = 3i32;
pub const EvtSeekRelativeToFirst: i32 = 1i32;
pub const EvtSeekRelativeToLast: i32 = 2i32;
pub const EvtSeekRelativeToCurrent: i32 = 3i32;
pub const EvtSeekRelativeToBookmark: i32 = 4i32;
pub const EvtSeekOriginMask: i32 = 7i32;
pub const EvtSeekStrict: i32 = 65536i32;
pub type EVT_SUBSCRIBE_CALLBACK = unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const ::core::ffi::c_void, event: isize) -> u32;
pub const EvtSubscribeToFutureEvents: i32 = 1i32;
pub const EvtSubscribeStartAtOldestRecord: i32 = 2i32;
pub const EvtSubscribeStartAfterBookmark: i32 = 3i32;
pub const EvtSubscribeOriginMask: i32 = 3i32;
pub const EvtSubscribeTolerateQueryErrors: i32 = 4096i32;
pub const EvtSubscribeStrict: i32 = 65536i32;
pub const EvtSubscribeActionError: i32 = 0i32;
pub const EvtSubscribeActionDeliver: i32 = 1i32;
pub const EvtSystemProviderName: i32 = 0i32;
pub const EvtSystemProviderGuid: i32 = 1i32;
pub const EvtSystemEventID: i32 = 2i32;
pub const EvtSystemQualifiers: i32 = 3i32;
pub const EvtSystemLevel: i32 = 4i32;
pub const EvtSystemTask: i32 = 5i32;
pub const EvtSystemOpcode: i32 = 6i32;
pub const EvtSystemKeywords: i32 = 7i32;
pub const EvtSystemTimeCreated: i32 = 8i32;
pub const EvtSystemEventRecordId: i32 = 9i32;
pub const EvtSystemActivityID: i32 = 10i32;
pub const EvtSystemRelatedActivityID: i32 = 11i32;
pub const EvtSystemProcessID: i32 = 12i32;
pub const EvtSystemThreadID: i32 = 13i32;
pub const EvtSystemChannel: i32 = 14i32;
pub const EvtSystemComputer: i32 = 15i32;
pub const EvtSystemUserID: i32 = 16i32;
pub const EvtSystemVersion: i32 = 17i32;
pub const EvtSystemPropertyIdEND: i32 = 18i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVT_VARIANT {
    pub Anonymous: EVT_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVT_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVT_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EVT_VARIANT_0 {
    pub BooleanVal: super::super::Foundation::BOOL,
    pub SByteVal: i8,
    pub Int16Val: i16,
    pub Int32Val: i32,
    pub Int64Val: i64,
    pub ByteVal: u8,
    pub UInt16Val: u16,
    pub UInt32Val: u32,
    pub UInt64Val: u64,
    pub SingleVal: f32,
    pub DoubleVal: f64,
    pub FileTimeVal: u64,
    pub SysTimeVal: *mut super::super::Foundation::SYSTEMTIME,
    pub GuidVal: *mut ::windows_sys::core::GUID,
    pub StringVal: super::super::Foundation::PWSTR,
    pub AnsiStringVal: super::super::Foundation::PSTR,
    pub BinaryVal: *mut u8,
    pub SidVal: super::super::Foundation::PSID,
    pub SizeTVal: usize,
    pub BooleanArr: *mut super::super::Foundation::BOOL,
    pub SByteArr: *mut i8,
    pub Int16Arr: *mut i16,
    pub Int32Arr: *mut i32,
    pub Int64Arr: *mut i64,
    pub ByteArr: *mut u8,
    pub UInt16Arr: *mut u16,
    pub UInt32Arr: *mut u32,
    pub UInt64Arr: *mut u64,
    pub SingleArr: *mut f32,
    pub DoubleArr: *mut f64,
    pub FileTimeArr: *mut super::super::Foundation::FILETIME,
    pub SysTimeArr: *mut super::super::Foundation::SYSTEMTIME,
    pub GuidArr: *mut ::windows_sys::core::GUID,
    pub StringArr: *mut super::super::Foundation::PWSTR,
    pub AnsiStringArr: *mut super::super::Foundation::PSTR,
    pub SidArr: *mut super::super::Foundation::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: isize,
    pub XmlVal: super::super::Foundation::PWSTR,
    pub XmlValArr: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVT_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVT_VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EvtVarTypeNull: i32 = 0i32;
pub const EvtVarTypeString: i32 = 1i32;
pub const EvtVarTypeAnsiString: i32 = 2i32;
pub const EvtVarTypeSByte: i32 = 3i32;
pub const EvtVarTypeByte: i32 = 4i32;
pub const EvtVarTypeInt16: i32 = 5i32;
pub const EvtVarTypeUInt16: i32 = 6i32;
pub const EvtVarTypeInt32: i32 = 7i32;
pub const EvtVarTypeUInt32: i32 = 8i32;
pub const EvtVarTypeInt64: i32 = 9i32;
pub const EvtVarTypeUInt64: i32 = 10i32;
pub const EvtVarTypeSingle: i32 = 11i32;
pub const EvtVarTypeDouble: i32 = 12i32;
pub const EvtVarTypeBoolean: i32 = 13i32;
pub const EvtVarTypeBinary: i32 = 14i32;
pub const EvtVarTypeGuid: i32 = 15i32;
pub const EvtVarTypeSizeT: i32 = 16i32;
pub const EvtVarTypeFileTime: i32 = 17i32;
pub const EvtVarTypeSysTime: i32 = 18i32;
pub const EvtVarTypeSid: i32 = 19i32;
pub const EvtVarTypeHexInt32: i32 = 20i32;
pub const EvtVarTypeHexInt64: i32 = 21i32;
pub const EvtVarTypeEvtHandle: i32 = 32i32;
pub const EvtVarTypeEvtXml: i32 = 35i32;
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EVT_WRITE_ACCESS: u32 = 2u32;
pub type EventLogHandle = isize;
pub type EventSourceHandle = isize;
pub const EVENTLOG_SEEK_READ: u32 = 2u32;
pub const EVENTLOG_SEQUENTIAL_READ: u32 = 1u32;
pub const EVENTLOG_SUCCESS: u16 = 0u16;
pub const EVENTLOG_AUDIT_FAILURE: u16 = 16u16;
pub const EVENTLOG_AUDIT_SUCCESS: u16 = 8u16;
pub const EVENTLOG_ERROR_TYPE: u16 = 1u16;
pub const EVENTLOG_INFORMATION_TYPE: u16 = 4u16;
pub const EVENTLOG_WARNING_TYPE: u16 = 2u16;
