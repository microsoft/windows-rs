#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupEventLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupEventLogA(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BackupEventLogA(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupEventLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupEventLogW(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BackupEventLogW(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearEventLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearEventLogA(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ClearEventLogA(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearEventLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearEventLogW(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ClearEventLogW(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseEventLog<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>>(heventlog: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseEventLog(heventlog: EventLogHandle) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseEventLog(heventlog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeregisterEventSource<'a, Param0: ::windows::core::IntoParam<'a, EventSourceHandle>>(heventlog: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeregisterEventSource(heventlog: EventSourceHandle) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeregisterEventSource(heventlog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl EVENTLOGRECORD {}
impl ::core::default::Default for EVENTLOGRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENTLOGRECORD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENTLOGRECORD")
            .field("Length", &self.Length)
            .field("Reserved", &self.Reserved)
            .field("RecordNumber", &self.RecordNumber)
            .field("TimeGenerated", &self.TimeGenerated)
            .field("TimeWritten", &self.TimeWritten)
            .field("EventID", &self.EventID)
            .field("EventType", &self.EventType)
            .field("NumStrings", &self.NumStrings)
            .field("EventCategory", &self.EventCategory)
            .field("ReservedFlags", &self.ReservedFlags)
            .field("ClosingRecordNumber", &self.ClosingRecordNumber)
            .field("StringOffset", &self.StringOffset)
            .field("UserSidLength", &self.UserSidLength)
            .field("UserSidOffset", &self.UserSidOffset)
            .field("DataLength", &self.DataLength)
            .field("DataOffset", &self.DataOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EVENTLOGRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Reserved == other.Reserved && self.RecordNumber == other.RecordNumber && self.TimeGenerated == other.TimeGenerated && self.TimeWritten == other.TimeWritten && self.EventID == other.EventID && self.EventType == other.EventType && self.NumStrings == other.NumStrings && self.EventCategory == other.EventCategory && self.ReservedFlags == other.ReservedFlags && self.ClosingRecordNumber == other.ClosingRecordNumber && self.StringOffset == other.StringOffset && self.UserSidLength == other.UserSidLength && self.UserSidOffset == other.UserSidOffset && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for EVENTLOGRECORD {}
unsafe impl ::windows::core::Abi for EVENTLOGRECORD {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
impl EVENTLOG_FULL_INFORMATION {}
impl ::core::default::Default for EVENTLOG_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENTLOG_FULL_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENTLOG_FULL_INFORMATION").field("dwFull", &self.dwFull).finish()
    }
}
impl ::core::cmp::PartialEq for EVENTLOG_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFull == other.dwFull
    }
}
impl ::core::cmp::Eq for EVENTLOG_FULL_INFORMATION {}
unsafe impl ::windows::core::Abi for EVENTLOG_FULL_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENTSFORLOGFILE {
    pub ulSize: u32,
    pub szLogicalLogFile: [u16; 256],
    pub ulNumRecords: u32,
    pub pEventLogRecords: [EVENTLOGRECORD; 1],
}
impl EVENTSFORLOGFILE {}
impl ::core::default::Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENTSFORLOGFILE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENTSFORLOGFILE").field("ulSize", &self.ulSize).field("szLogicalLogFile", &self.szLogicalLogFile).field("ulNumRecords", &self.ulNumRecords).field("pEventLogRecords", &self.pEventLogRecords).finish()
    }
}
impl ::core::cmp::PartialEq for EVENTSFORLOGFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.szLogicalLogFile == other.szLogicalLogFile && self.ulNumRecords == other.ulNumRecords && self.pEventLogRecords == other.pEventLogRecords
    }
}
impl ::core::cmp::Eq for EVENTSFORLOGFILE {}
unsafe impl ::windows::core::Abi for EVENTSFORLOGFILE {
    type Abi = Self;
}
pub const EVT_ALL_ACCESS: u32 = 7u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_CHANNEL_CLOCK_TYPE(pub i32);
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(0i32);
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(1i32);
impl ::core::convert::From<i32> for EVT_CHANNEL_CLOCK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_CLOCK_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_CHANNEL_CONFIG_PROPERTY_ID(pub i32);
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(0i32);
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(1i32);
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(2i32);
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(3i32);
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(4i32);
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(5i32);
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(6i32);
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(7i32);
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(8i32);
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(9i32);
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(10i32);
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(11i32);
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(12i32);
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(13i32);
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(14i32);
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(15i32);
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(16i32);
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(17i32);
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(18i32);
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(19i32);
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(20i32);
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(21i32);
impl ::core::convert::From<i32> for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_CHANNEL_ISOLATION_TYPE(pub i32);
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(0i32);
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(1i32);
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(2i32);
impl ::core::convert::From<i32> for EVT_CHANNEL_ISOLATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_ISOLATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_CHANNEL_REFERENCE_FLAGS(pub i32);
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = EVT_CHANNEL_REFERENCE_FLAGS(1i32);
impl ::core::convert::From<i32> for EVT_CHANNEL_REFERENCE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_REFERENCE_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_CHANNEL_SID_TYPE(pub i32);
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(0i32);
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(1i32);
impl ::core::convert::From<i32> for EVT_CHANNEL_SID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_SID_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_CHANNEL_TYPE(pub i32);
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(0i32);
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(1i32);
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(2i32);
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(3i32);
impl ::core::convert::From<i32> for EVT_CHANNEL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_TYPE {
    type Abi = Self;
}
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_EVENT_METADATA_PROPERTY_ID(pub i32);
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(0i32);
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(1i32);
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(2i32);
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(3i32);
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(4i32);
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(5i32);
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(6i32);
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(7i32);
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(8i32);
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(9i32);
impl ::core::convert::From<i32> for EVT_EVENT_METADATA_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_EVENT_METADATA_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_EVENT_PROPERTY_ID(pub i32);
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(0i32);
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(1i32);
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(2i32);
impl ::core::convert::From<i32> for EVT_EVENT_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_EVENT_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_EXPORTLOG_FLAGS(pub i32);
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(1i32);
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(2i32);
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(4096i32);
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(8192i32);
impl ::core::convert::From<i32> for EVT_EXPORTLOG_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_EXPORTLOG_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_FORMAT_MESSAGE_FLAGS(pub i32);
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(1i32);
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(2i32);
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(3i32);
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(4i32);
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(5i32);
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(6i32);
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(7i32);
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(8i32);
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(9i32);
impl ::core::convert::From<i32> for EVT_FORMAT_MESSAGE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_FORMAT_MESSAGE_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_LOGIN_CLASS(pub i32);
pub const EvtRpcLogin: EVT_LOGIN_CLASS = EVT_LOGIN_CLASS(1i32);
impl ::core::convert::From<i32> for EVT_LOGIN_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_LOGIN_CLASS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_LOG_PROPERTY_ID(pub i32);
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(0i32);
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(1i32);
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(2i32);
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(3i32);
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(4i32);
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(5i32);
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(6i32);
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(7i32);
impl ::core::convert::From<i32> for EVT_LOG_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_LOG_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_OPEN_LOG_FLAGS(pub i32);
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(1i32);
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(2i32);
impl ::core::convert::From<i32> for EVT_OPEN_LOG_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_OPEN_LOG_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_PUBLISHER_METADATA_PROPERTY_ID(pub i32);
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(0i32);
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(1i32);
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(2i32);
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(3i32);
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(4i32);
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(5i32);
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(6i32);
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(7i32);
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(8i32);
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(9i32);
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(10i32);
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(11i32);
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(12i32);
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(13i32);
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(14i32);
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(15i32);
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(16i32);
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(17i32);
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(18i32);
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(19i32);
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(20i32);
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(21i32);
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(22i32);
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(23i32);
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(24i32);
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(25i32);
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(26i32);
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(27i32);
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(28i32);
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(29i32);
impl ::core::convert::From<i32> for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_QUERY_FLAGS(pub i32);
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(1i32);
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(2i32);
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(256i32);
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(512i32);
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(4096i32);
impl ::core::convert::From<i32> for EVT_QUERY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_QUERY_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_QUERY_PROPERTY_ID(pub i32);
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(0i32);
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(1i32);
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(2i32);
impl ::core::convert::From<i32> for EVT_QUERY_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_QUERY_PROPERTY_ID {
    type Abi = Self;
}
pub const EVT_READ_ACCESS: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_RENDER_CONTEXT_FLAGS(pub i32);
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(0i32);
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(1i32);
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(2i32);
impl ::core::convert::From<i32> for EVT_RENDER_CONTEXT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_RENDER_CONTEXT_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_RENDER_FLAGS(pub i32);
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(0i32);
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(1i32);
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(2i32);
impl ::core::convert::From<i32> for EVT_RENDER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_RENDER_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl EVT_RPC_LOGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_RPC_LOGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVT_RPC_LOGIN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVT_RPC_LOGIN").field("Server", &self.Server).field("User", &self.User).field("Domain", &self.Domain).field("Password", &self.Password).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVT_RPC_LOGIN {
    fn eq(&self, other: &Self) -> bool {
        self.Server == other.Server && self.User == other.User && self.Domain == other.Domain && self.Password == other.Password && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVT_RPC_LOGIN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVT_RPC_LOGIN {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_RPC_LOGIN_FLAGS(pub i32);
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(0i32);
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(1i32);
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(2i32);
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(3i32);
impl ::core::convert::From<i32> for EVT_RPC_LOGIN_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_RPC_LOGIN_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_SEEK_FLAGS(pub i32);
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(1i32);
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(2i32);
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(3i32);
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(4i32);
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(7i32);
pub const EvtSeekStrict: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(65536i32);
impl ::core::convert::From<i32> for EVT_SEEK_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_SEEK_FLAGS {
    type Abi = Self;
}
pub type EVT_SUBSCRIBE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const ::core::ffi::c_void, event: isize) -> u32>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_SUBSCRIBE_FLAGS(pub i32);
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(1i32);
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(2i32);
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3i32);
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3i32);
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(4096i32);
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(65536i32);
impl ::core::convert::From<i32> for EVT_SUBSCRIBE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_SUBSCRIBE_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_SUBSCRIBE_NOTIFY_ACTION(pub i32);
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(0i32);
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(1i32);
impl ::core::convert::From<i32> for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_SUBSCRIBE_NOTIFY_ACTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_SYSTEM_PROPERTY_ID(pub i32);
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(0i32);
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(1i32);
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(2i32);
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(3i32);
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(4i32);
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(5i32);
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(6i32);
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(7i32);
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(8i32);
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(9i32);
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(10i32);
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(11i32);
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(12i32);
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(13i32);
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(14i32);
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(15i32);
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(16i32);
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(17i32);
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(18i32);
impl ::core::convert::From<i32> for EVT_SYSTEM_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_SYSTEM_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVT_VARIANT {
    pub Anonymous: EVT_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVT_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVT_VARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVT_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVT_VARIANT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub GuidVal: *mut ::windows::core::GUID,
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
    pub GuidArr: *mut ::windows::core::GUID,
    pub StringArr: *mut super::super::Foundation::PWSTR,
    pub AnsiStringArr: *mut super::super::Foundation::PSTR,
    pub SidArr: *mut super::super::Foundation::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: isize,
    pub XmlVal: super::super::Foundation::PWSTR,
    pub XmlValArr: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl EVT_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVT_VARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVT_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVT_VARIANT_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVT_VARIANT_TYPE(pub i32);
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(0i32);
pub const EvtVarTypeString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(1i32);
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(2i32);
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(3i32);
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(4i32);
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(5i32);
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(6i32);
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(7i32);
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(8i32);
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(9i32);
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(10i32);
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(11i32);
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(12i32);
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(13i32);
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(14i32);
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(15i32);
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(16i32);
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(17i32);
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(18i32);
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(19i32);
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(20i32);
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(21i32);
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(32i32);
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(35i32);
impl ::core::convert::From<i32> for EVT_VARIANT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVT_VARIANT_TYPE {
    type Abi = Self;
}
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EVT_WRITE_ACCESS: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct EventLogHandle(pub isize);
impl ::core::default::Default for EventLogHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for EventLogHandle {}
unsafe impl ::windows::core::Abi for EventLogHandle {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct EventSourceHandle(pub isize);
impl ::core::default::Default for EventSourceHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for EventSourceHandle {}
unsafe impl ::windows::core::Abi for EventSourceHandle {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtArchiveExportedLog<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, logfilepath: Param1, locale: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtArchiveExportedLog(session: isize, logfilepath: super::super::Foundation::PWSTR, locale: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtArchiveExportedLog(::core::mem::transmute(session), logfilepath.into_param().abi(), ::core::mem::transmute(locale), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtCancel(object: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtCancel(object: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtCancel(::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtClearLog<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, channelpath: Param1, targetfilepath: Param2, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtClearLog(session: isize, channelpath: super::super::Foundation::PWSTR, targetfilepath: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtClearLog(::core::mem::transmute(session), channelpath.into_param().abi(), targetfilepath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtClose(object: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtClose(object: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtClose(::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtCreateBookmark<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(bookmarkxml: Param0) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtCreateBookmark(bookmarkxml: super::super::Foundation::PWSTR) -> isize;
        }
        ::core::mem::transmute(EvtCreateBookmark(bookmarkxml.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtCreateRenderContext(valuepathscount: u32, valuepaths: *const super::super::Foundation::PWSTR, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtCreateRenderContext(valuepathscount: u32, valuepaths: *const super::super::Foundation::PWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtCreateRenderContext(::core::mem::transmute(valuepathscount), ::core::mem::transmute(valuepaths), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtExportLog<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, path: Param1, query: Param2, targetfilepath: Param3, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtExportLog(session: isize, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, targetfilepath: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtExportLog(::core::mem::transmute(session), path.into_param().abi(), query.into_param().abi(), targetfilepath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtFormatMessage(publishermetadata: isize, event: isize, messageid: u32, valuecount: u32, values: *const EVT_VARIANT, flags: u32, buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtFormatMessage(publishermetadata: isize, event: isize, messageid: u32, valuecount: u32, values: *const EVT_VARIANT, flags: u32, buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtFormatMessage(::core::mem::transmute(publishermetadata), ::core::mem::transmute(event), ::core::mem::transmute(messageid), ::core::mem::transmute(valuecount), ::core::mem::transmute(values), ::core::mem::transmute(flags), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetChannelConfigProperty(::core::mem::transmute(channelconfig), ::core::mem::transmute(propertyid), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetEventInfo(event: isize, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetEventInfo(event: isize, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetEventInfo(::core::mem::transmute(event), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetEventMetadataProperty(eventmetadata: isize, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: *mut EVT_VARIANT, eventmetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetEventMetadataProperty(eventmetadata: isize, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: *mut EVT_VARIANT, eventmetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetEventMetadataProperty(::core::mem::transmute(eventmetadata), ::core::mem::transmute(propertyid), ::core::mem::transmute(flags), ::core::mem::transmute(eventmetadatapropertybuffersize), ::core::mem::transmute(eventmetadatapropertybuffer), ::core::mem::transmute(eventmetadatapropertybufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetExtendedStatus(buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetExtendedStatus(buffersize: u32, buffer: super::super::Foundation::PWSTR, bufferused: *mut u32) -> u32;
        }
        ::core::mem::transmute(EvtGetExtendedStatus(::core::mem::transmute(buffersize), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetLogInfo(log: isize, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetLogInfo(log: isize, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetLogInfo(::core::mem::transmute(log), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetObjectArrayProperty(objectarray: isize, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetObjectArrayProperty(objectarray: isize, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetObjectArrayProperty(::core::mem::transmute(objectarray), ::core::mem::transmute(propertyid), ::core::mem::transmute(arrayindex), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetObjectArraySize(::core::mem::transmute(objectarray), ::core::mem::transmute(objectarraysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetPublisherMetadataProperty(publishermetadata: isize, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: *mut EVT_VARIANT, publishermetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetPublisherMetadataProperty(publishermetadata: isize, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: *mut EVT_VARIANT, publishermetadatapropertybufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetPublisherMetadataProperty(::core::mem::transmute(publishermetadata), ::core::mem::transmute(propertyid), ::core::mem::transmute(flags), ::core::mem::transmute(publishermetadatapropertybuffersize), ::core::mem::transmute(publishermetadatapropertybuffer), ::core::mem::transmute(publishermetadatapropertybufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtGetQueryInfo(queryorsubscription: isize, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetQueryInfo(queryorsubscription: isize, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EVT_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtGetQueryInfo(::core::mem::transmute(queryorsubscription), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNext(resultset: isize, eventssize: u32, events: *mut isize, timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNext(resultset: isize, eventssize: u32, events: *mut isize, timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtNext(::core::mem::transmute(resultset), ::core::mem::transmute(eventssize), ::core::mem::transmute(events), ::core::mem::transmute(timeout), ::core::mem::transmute(flags), ::core::mem::transmute(returned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNextChannelPath(channelenum: isize, channelpathbuffersize: u32, channelpathbuffer: super::super::Foundation::PWSTR, channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNextChannelPath(channelenum: isize, channelpathbuffersize: u32, channelpathbuffer: super::super::Foundation::PWSTR, channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtNextChannelPath(::core::mem::transmute(channelenum), ::core::mem::transmute(channelpathbuffersize), ::core::mem::transmute(channelpathbuffer), ::core::mem::transmute(channelpathbufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvtNextEventMetadata(eventmetadataenum: isize, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNextEventMetadata(eventmetadataenum: isize, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtNextEventMetadata(::core::mem::transmute(eventmetadataenum), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNextPublisherId(publisherenum: isize, publisheridbuffersize: u32, publisheridbuffer: super::super::Foundation::PWSTR, publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNextPublisherId(publisherenum: isize, publisheridbuffersize: u32, publisheridbuffer: super::super::Foundation::PWSTR, publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtNextPublisherId(::core::mem::transmute(publisherenum), ::core::mem::transmute(publisheridbuffersize), ::core::mem::transmute(publisheridbuffer), ::core::mem::transmute(publisheridbufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtOpenChannelConfig<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, channelpath: Param1, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenChannelConfig(session: isize, channelpath: super::super::Foundation::PWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenChannelConfig(::core::mem::transmute(session), channelpath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvtOpenChannelEnum(session: isize, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenChannelEnum(session: isize, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenChannelEnum(::core::mem::transmute(session), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvtOpenEventMetadataEnum(publishermetadata: isize, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenEventMetadataEnum(publishermetadata: isize, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenEventMetadataEnum(::core::mem::transmute(publishermetadata), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtOpenLog<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, path: Param1, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenLog(session: isize, path: super::super::Foundation::PWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenLog(::core::mem::transmute(session), path.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvtOpenPublisherEnum(session: isize, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenPublisherEnum(session: isize, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenPublisherEnum(::core::mem::transmute(session), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtOpenPublisherMetadata<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, publisherid: Param1, logfilepath: Param2, locale: u32, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenPublisherMetadata(session: isize, publisherid: super::super::Foundation::PWSTR, logfilepath: super::super::Foundation::PWSTR, locale: u32, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenPublisherMetadata(::core::mem::transmute(session), publisherid.into_param().abi(), logfilepath.into_param().abi(), ::core::mem::transmute(locale), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *const ::core::ffi::c_void, timeout: u32, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *const ::core::ffi::c_void, timeout: u32, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenSession(::core::mem::transmute(loginclass), ::core::mem::transmute(login), ::core::mem::transmute(timeout), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtQuery<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, path: Param1, query: Param2, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtQuery(session: isize, path: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtQuery(::core::mem::transmute(session), path.into_param().abi(), query.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtRender(context: isize, fragment: isize, flags: u32, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32, propertycount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtRender(context: isize, fragment: isize, flags: u32, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32, propertycount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtRender(::core::mem::transmute(context), ::core::mem::transmute(fragment), ::core::mem::transmute(flags), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferused), ::core::mem::transmute(propertycount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSaveChannelConfig(channelconfig: isize, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtSaveChannelConfig(channelconfig: isize, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtSaveChannelConfig(::core::mem::transmute(channelconfig), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSeek(resultset: isize, position: i64, bookmark: isize, timeout: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtSeek(resultset: isize, position: i64, bookmark: isize, timeout: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtSeek(::core::mem::transmute(resultset), ::core::mem::transmute(position), ::core::mem::transmute(bookmark), ::core::mem::transmute(timeout), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtSetChannelConfigProperty(channelconfig: isize, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtSetChannelConfigProperty(::core::mem::transmute(channelconfig), ::core::mem::transmute(propertyid), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSubscribe<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(session: isize, signalevent: Param1, channelpath: Param2, query: Param3, bookmark: isize, context: *const ::core::ffi::c_void, callback: EVT_SUBSCRIBE_CALLBACK, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtSubscribe(session: isize, signalevent: super::super::Foundation::HANDLE, channelpath: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, bookmark: isize, context: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtSubscribe(::core::mem::transmute(session), signalevent.into_param().abi(), channelpath.into_param().abi(), query.into_param().abi(), ::core::mem::transmute(bookmark), ::core::mem::transmute(context), ::core::mem::transmute(callback), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtUpdateBookmark(bookmark: isize, event: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtUpdateBookmark(bookmark: isize, event: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtUpdateBookmark(::core::mem::transmute(bookmark), ::core::mem::transmute(event)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEventLogInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEventLogInformation(heventlog: super::super::Foundation::HANDLE, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetEventLogInformation(heventlog.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfEventLogRecords<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, numberofrecords: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfEventLogRecords(heventlog: super::super::Foundation::HANDLE, numberofrecords: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumberOfEventLogRecords(heventlog.into_param().abi(), ::core::mem::transmute(numberofrecords)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOldestEventLogRecord<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, oldestrecord: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOldestEventLogRecord(heventlog: super::super::Foundation::HANDLE, oldestrecord: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOldestEventLogRecord(heventlog.into_param().abi(), ::core::mem::transmute(oldestrecord)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyChangeEventLog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, hevent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyChangeEventLog(heventlog: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(NotifyChangeEventLog(heventlog.into_param().abi(), hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenBackupEventLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpuncservername: Param0, lpfilename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenBackupEventLogA(lpuncservername: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenBackupEventLogA(lpuncservername.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenBackupEventLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpuncservername: Param0, lpfilename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenBackupEventLogW(lpuncservername: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenBackupEventLogW(lpuncservername.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEventLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventLogA(lpuncservername: super::super::Foundation::PSTR, lpsourcename: super::super::Foundation::PSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenEventLogA(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEventLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventLogW(lpuncservername: super::super::Foundation::PWSTR, lpsourcename: super::super::Foundation::PWSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenEventLogW(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct READ_EVENT_LOG_READ_FLAGS(pub u32);
pub const EVENTLOG_SEEK_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(2u32);
pub const EVENTLOG_SEQUENTIAL_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(1u32);
impl ::core::convert::From<u32> for READ_EVENT_LOG_READ_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for READ_EVENT_LOG_READ_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for READ_EVENT_LOG_READ_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for READ_EVENT_LOG_READ_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for READ_EVENT_LOG_READ_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for READ_EVENT_LOG_READ_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for READ_EVENT_LOG_READ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REPORT_EVENT_TYPE(pub u16);
pub const EVENTLOG_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(0u16);
pub const EVENTLOG_AUDIT_FAILURE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(16u16);
pub const EVENTLOG_AUDIT_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(8u16);
pub const EVENTLOG_ERROR_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(1u16);
pub const EVENTLOG_INFORMATION_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(4u16);
pub const EVENTLOG_WARNING_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(2u16);
impl ::core::convert::From<u16> for REPORT_EVENT_TYPE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REPORT_EVENT_TYPE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadEventLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadEventLogA(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadEventLogA(heventlog.into_param().abi(), ::core::mem::transmute(dwreadflags), ::core::mem::transmute(dwrecordoffset), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytestoread), ::core::mem::transmute(pnbytesread), ::core::mem::transmute(pnminnumberofbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadEventLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadEventLogW(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadEventLogW(heventlog.into_param().abi(), ::core::mem::transmute(dwreadflags), ::core::mem::transmute(dwrecordoffset), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytestoread), ::core::mem::transmute(pnbytesread), ::core::mem::transmute(pnminnumberofbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterEventSourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventSourceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterEventSourceA(lpuncservername: super::super::Foundation::PSTR, lpsourcename: super::super::Foundation::PSTR) -> EventSourceHandle;
        }
        ::core::mem::transmute(RegisterEventSourceA(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterEventSourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventSourceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterEventSourceW(lpuncservername: super::super::Foundation::PWSTR, lpsourcename: super::super::Foundation::PWSTR) -> EventSourceHandle;
        }
        ::core::mem::transmute(RegisterEventSourceW(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportEventA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(heventlog: Param0, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: Param4, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReportEventA(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReportEventA(heventlog.into_param().abi(), ::core::mem::transmute(wtype), ::core::mem::transmute(wcategory), ::core::mem::transmute(dweventid), lpusersid.into_param().abi(), ::core::mem::transmute(wnumstrings), ::core::mem::transmute(dwdatasize), ::core::mem::transmute(lpstrings), ::core::mem::transmute(lprawdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportEventW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(heventlog: Param0, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: Param4, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PWSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReportEventW(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const super::super::Foundation::PWSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReportEventW(heventlog.into_param().abi(), ::core::mem::transmute(wtype), ::core::mem::transmute(wcategory), ::core::mem::transmute(dweventid), lpusersid.into_param().abi(), ::core::mem::transmute(wnumstrings), ::core::mem::transmute(dwdatasize), ::core::mem::transmute(lpstrings), ::core::mem::transmute(lprawdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
