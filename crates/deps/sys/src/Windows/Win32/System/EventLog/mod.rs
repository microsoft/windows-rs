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
    pub fn EvtSubscribe(session: isize, signalevent: super::super::Foundation::HANDLE, channelpath: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, bookmark: isize, context: *const ::core::ffi::c_void, callback: ::core::option::Option<EVT_SUBSCRIBE_CALLBACK>, flags: u32) -> isize;
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
pub type EVT_CHANNEL_CLOCK_TYPE = i32;
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = 0i32;
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = 1i32;
pub type EVT_CHANNEL_CONFIG_PROPERTY_ID = i32;
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = 0i32;
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = 1i32;
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 2i32;
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = 3i32;
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = 4i32;
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = 5i32;
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = 6i32;
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = 7i32;
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = 8i32;
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = 9i32;
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = 10i32;
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = 11i32;
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = 12i32;
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = 13i32;
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = 14i32;
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = 15i32;
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = 16i32;
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 17i32;
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 18i32;
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = 19i32;
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = 20i32;
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = 21i32;
pub type EVT_CHANNEL_ISOLATION_TYPE = i32;
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = 0i32;
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = 1i32;
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = 2i32;
pub type EVT_CHANNEL_REFERENCE_FLAGS = i32;
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = 1i32;
pub type EVT_CHANNEL_SID_TYPE = i32;
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = 0i32;
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = 1i32;
pub type EVT_CHANNEL_TYPE = i32;
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = 0i32;
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = 1i32;
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = 2i32;
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = 3i32;
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
pub type EVT_EVENT_METADATA_PROPERTY_ID = i32;
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = 0i32;
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = 1i32;
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = 2i32;
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = 3i32;
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = 4i32;
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = 5i32;
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = 6i32;
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = 7i32;
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = 8i32;
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = 9i32;
pub type EVT_EVENT_PROPERTY_ID = i32;
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = 0i32;
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = 1i32;
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = 2i32;
pub type EVT_EXPORTLOG_FLAGS = i32;
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = 1i32;
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = 2i32;
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = 4096i32;
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = 8192i32;
pub type EVT_FORMAT_MESSAGE_FLAGS = i32;
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = 1i32;
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = 2i32;
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = 3i32;
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = 4i32;
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = 5i32;
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = 6i32;
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = 7i32;
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = 8i32;
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = 9i32;
pub type EVT_LOGIN_CLASS = i32;
pub const EvtRpcLogin: EVT_LOGIN_CLASS = 1i32;
pub type EVT_LOG_PROPERTY_ID = i32;
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = 0i32;
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = 1i32;
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = 2i32;
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = 3i32;
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = 4i32;
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = 5i32;
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = 6i32;
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = 7i32;
pub type EVT_OPEN_LOG_FLAGS = i32;
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = 1i32;
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = 2i32;
pub type EVT_PUBLISHER_METADATA_PROPERTY_ID = i32;
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = 0i32;
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 1i32;
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 2i32;
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 3i32;
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = 4i32;
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 5i32;
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = 6i32;
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 7i32;
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = 8i32;
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 9i32;
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = 10i32;
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 11i32;
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = 12i32;
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 13i32;
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 14i32;
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 15i32;
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = 16i32;
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 17i32;
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = 18i32;
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 19i32;
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 20i32;
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = 21i32;
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 22i32;
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 23i32;
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 24i32;
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = 25i32;
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 26i32;
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 27i32;
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 28i32;
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = 29i32;
pub type EVT_QUERY_FLAGS = i32;
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = 1i32;
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = 2i32;
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = 256i32;
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = 512i32;
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = 4096i32;
pub type EVT_QUERY_PROPERTY_ID = i32;
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = 0i32;
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = 1i32;
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = 2i32;
pub const EVT_READ_ACCESS: u32 = 1u32;
pub type EVT_RENDER_CONTEXT_FLAGS = i32;
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = 0i32;
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = 1i32;
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = 2i32;
pub type EVT_RENDER_FLAGS = i32;
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = 0i32;
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = 1i32;
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = 2i32;
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
pub type EVT_RPC_LOGIN_FLAGS = i32;
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = 0i32;
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = 1i32;
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = 2i32;
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = 3i32;
pub type EVT_SEEK_FLAGS = i32;
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = 1i32;
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = 2i32;
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = 3i32;
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = 4i32;
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = 7i32;
pub const EvtSeekStrict: EVT_SEEK_FLAGS = 65536i32;
pub type EVT_SUBSCRIBE_CALLBACK = unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const ::core::ffi::c_void, event: isize) -> u32;
pub type EVT_SUBSCRIBE_FLAGS = i32;
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = 1i32;
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = 2i32;
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = 3i32;
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = 3i32;
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = 4096i32;
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = 65536i32;
pub type EVT_SUBSCRIBE_NOTIFY_ACTION = i32;
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = 0i32;
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = 1i32;
pub type EVT_SYSTEM_PROPERTY_ID = i32;
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = 0i32;
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = 1i32;
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = 2i32;
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = 3i32;
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = 4i32;
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = 5i32;
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = 6i32;
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = 7i32;
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = 8i32;
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = 9i32;
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = 10i32;
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = 11i32;
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = 12i32;
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = 13i32;
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = 14i32;
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = 15i32;
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = 16i32;
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = 17i32;
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = 18i32;
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
pub type EVT_VARIANT_TYPE = i32;
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = 0i32;
pub const EvtVarTypeString: EVT_VARIANT_TYPE = 1i32;
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = 2i32;
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = 3i32;
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = 4i32;
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = 5i32;
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = 6i32;
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = 7i32;
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = 8i32;
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = 9i32;
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = 10i32;
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = 11i32;
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = 12i32;
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = 13i32;
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = 14i32;
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = 15i32;
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = 16i32;
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = 17i32;
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = 18i32;
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = 19i32;
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = 20i32;
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = 21i32;
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = 32i32;
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = 35i32;
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EVT_WRITE_ACCESS: u32 = 2u32;
pub type EventLogHandle = isize;
pub type EventSourceHandle = isize;
pub type READ_EVENT_LOG_READ_FLAGS = u32;
pub const EVENTLOG_SEEK_READ: READ_EVENT_LOG_READ_FLAGS = 2u32;
pub const EVENTLOG_SEQUENTIAL_READ: READ_EVENT_LOG_READ_FLAGS = 1u32;
pub type REPORT_EVENT_TYPE = u16;
pub const EVENTLOG_SUCCESS: REPORT_EVENT_TYPE = 0u16;
pub const EVENTLOG_AUDIT_FAILURE: REPORT_EVENT_TYPE = 16u16;
pub const EVENTLOG_AUDIT_SUCCESS: REPORT_EVENT_TYPE = 8u16;
pub const EVENTLOG_ERROR_TYPE: REPORT_EVENT_TYPE = 1u16;
pub const EVENTLOG_INFORMATION_TYPE: REPORT_EVENT_TYPE = 4u16;
pub const EVENTLOG_WARNING_TYPE: REPORT_EVENT_TYPE = 2u16;
