pub const EVENTLOG_AUDIT_FAILURE: REPORT_EVENT_TYPE = 16u16;
pub const EVENTLOG_AUDIT_SUCCESS: REPORT_EVENT_TYPE = 8u16;
pub const EVENTLOG_ERROR_TYPE: REPORT_EVENT_TYPE = 1u16;
pub const EVENTLOG_INFORMATION_TYPE: REPORT_EVENT_TYPE = 4u16;
pub const EVENTLOG_SEEK_READ: READ_EVENT_LOG_READ_FLAGS = 2u32;
pub const EVENTLOG_SEQUENTIAL_READ: READ_EVENT_LOG_READ_FLAGS = 1u32;
pub const EVENTLOG_SUCCESS: REPORT_EVENT_TYPE = 0u16;
pub const EVENTLOG_WARNING_TYPE: REPORT_EVENT_TYPE = 2u16;
pub const EVT_ALL_ACCESS: u32 = 7u32;
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
pub const EVT_READ_ACCESS: u32 = 1u32;
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EVT_WRITE_ACCESS: u32 = 2u32;
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = 2i32;
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = 0i32;
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = 6i32;
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = 3i32;
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = 7i32;
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = 4i32;
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = 5i32;
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = 8i32;
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = 1i32;
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = 1i32;
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = 0i32;
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = 5i32;
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = 4i32;
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = 0i32;
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = 1i32;
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = 3i32;
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = 21i32;
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 2i32;
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = 0i32;
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = 2i32;
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = 1i32;
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = 7i32;
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = 9i32;
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = 8i32;
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = 6i32;
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = 19i32;
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = 13i32;
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 17i32;
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = 12i32;
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = 20i32;
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = 11i32;
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = 16i32;
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = 10i32;
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = 15i32;
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = 14i32;
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = 18i32;
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = 1u32;
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = 0i32;
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = 1i32;
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = 0i32;
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = 2i32;
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = 3i32;
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = 1i32;
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = 9i32;
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = 1i32;
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = 2i32;
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = 0i32;
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = 1u32;
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = 2u32;
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = 8192u32;
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = 4096u32;
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = 6u32;
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = 1u32;
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = 8u32;
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = 5u32;
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = 2u32;
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = 4u32;
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = 7u32;
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = 3u32;
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = 9u32;
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = 4i32;
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = 0i32;
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = 3i32;
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = 7i32;
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = 1i32;
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = 2i32;
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = 5i32;
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = 6i32;
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = 1u32;
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = 2u32;
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = 10i32;
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 9i32;
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = 8i32;
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 11i32;
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 7i32;
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = 6i32;
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = 4i32;
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 28i32;
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 26i32;
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 27i32;
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = 25i32;
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 15i32;
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 13i32;
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 14i32;
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = 12i32;
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 3i32;
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 24i32;
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 22i32;
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 23i32;
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = 21i32;
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 2i32;
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = 29i32;
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = 0i32;
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 5i32;
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = 1i32;
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = 18i32;
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = 20i32;
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = 17i32;
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = 19i32;
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = 16i32;
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = 1u32;
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = 2u32;
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = 256u32;
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = 0i32;
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = 2i32;
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = 512u32;
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = 1i32;
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = 4096u32;
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = 2u32;
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = 1u32;
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = 2u32;
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = 0u32;
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = 0u32;
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = 1u32;
pub const EvtRpcLogin: EVT_LOGIN_CLASS = 1i32;
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = 0u32;
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = 2u32;
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = 3u32;
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = 1u32;
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = 7u32;
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = 4u32;
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = 3u32;
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = 1u32;
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = 2u32;
pub const EvtSeekStrict: EVT_SEEK_FLAGS = 65536u32;
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = 1i32;
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = 0i32;
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = 3u32;
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = 3u32;
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = 2u32;
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = 65536u32;
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = 1u32;
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = 4096u32;
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = 10i32;
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = 14i32;
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = 15i32;
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = 2i32;
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = 9i32;
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = 7i32;
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = 4i32;
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = 6i32;
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = 12i32;
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = 18i32;
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = 1i32;
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = 0i32;
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = 3i32;
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = 11i32;
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = 5i32;
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = 13i32;
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = 8i32;
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = 16i32;
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = 17i32;
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = 2i32;
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = 14i32;
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = 13i32;
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = 4i32;
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = 12i32;
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = 32i32;
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = 35i32;
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = 17i32;
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = 15i32;
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = 20i32;
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = 21i32;
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = 5i32;
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = 7i32;
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = 9i32;
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = 0i32;
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = 3i32;
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = 19i32;
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = 11i32;
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = 16i32;
pub const EvtVarTypeString: EVT_VARIANT_TYPE = 1i32;
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = 18i32;
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = 6i32;
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = 8i32;
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = 10i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_CHANNEL_CLOCK_TYPE(pub i32);
impl windows_core::TypeKind for EVT_CHANNEL_CLOCK_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_CHANNEL_CONFIG_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_CHANNEL_ISOLATION_TYPE(pub i32);
impl windows_core::TypeKind for EVT_CHANNEL_ISOLATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_CHANNEL_REFERENCE_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_CHANNEL_REFERENCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_CHANNEL_SID_TYPE(pub i32);
impl windows_core::TypeKind for EVT_CHANNEL_SID_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_CHANNEL_TYPE(pub i32);
impl windows_core::TypeKind for EVT_CHANNEL_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_EVENT_METADATA_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_EVENT_METADATA_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_EVENT_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_EVENT_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_EXPORTLOG_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_EXPORTLOG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_FORMAT_MESSAGE_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_FORMAT_MESSAGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_LOGIN_CLASS(pub i32);
impl windows_core::TypeKind for EVT_LOGIN_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_LOG_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_LOG_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_OPEN_LOG_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_OPEN_LOG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_PUBLISHER_METADATA_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_QUERY_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_QUERY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_QUERY_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_QUERY_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_RENDER_CONTEXT_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_RENDER_CONTEXT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_RENDER_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_RENDER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_RPC_LOGIN_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_RPC_LOGIN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_SEEK_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_SEEK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_SUBSCRIBE_FLAGS(pub u32);
impl windows_core::TypeKind for EVT_SUBSCRIBE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_SUBSCRIBE_NOTIFY_ACTION(pub i32);
impl windows_core::TypeKind for EVT_SUBSCRIBE_NOTIFY_ACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_SYSTEM_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for EVT_SYSTEM_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EVT_VARIANT_TYPE(pub i32);
impl windows_core::TypeKind for EVT_VARIANT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct READ_EVENT_LOG_READ_FLAGS(pub u32);
impl windows_core::TypeKind for READ_EVENT_LOG_READ_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REPORT_EVENT_TYPE(pub u16);
impl windows_core::TypeKind for REPORT_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for EVENTLOGRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EVENTLOGRECORD {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
impl Default for EVENTLOG_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EVENTLOG_FULL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENTSFORLOGFILE {
    pub ulSize: u32,
    pub szLogicalLogFile: [u16; 256],
    pub ulNumRecords: u32,
    pub pEventLogRecords: [EVENTLOGRECORD; 1],
}
impl Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EVENTSFORLOGFILE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVT_RPC_LOGIN {
    pub Server: windows_core::PWSTR,
    pub User: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub Password: windows_core::PWSTR,
    pub Flags: u32,
}
impl Default for EVT_RPC_LOGIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EVT_RPC_LOGIN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVT_VARIANT {
    pub Anonymous: EVT_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Security")]
impl Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for EVT_VARIANT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub GuidVal: *mut windows_core::GUID,
    pub StringVal: windows_core::PCWSTR,
    pub AnsiStringVal: windows_core::PCSTR,
    pub BinaryVal: *mut u8,
    pub SidVal: super::super::Security::PSID,
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
    pub GuidArr: *mut windows_core::GUID,
    pub StringArr: *mut windows_core::PWSTR,
    pub AnsiStringArr: *mut windows_core::PSTR,
    pub SidArr: *mut super::super::Security::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: EVT_HANDLE,
    pub XmlVal: windows_core::PCWSTR,
    pub XmlValArr: *const windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Security")]
impl Default for EVT_VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for EVT_VARIANT_0 {
    type TypeKind = windows_core::CloneType;
}
pub type EVT_SUBSCRIBE_CALLBACK = Option<unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const core::ffi::c_void, event: EVT_HANDLE) -> u32>;
