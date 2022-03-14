#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupEventLogA<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupEventLogA(heventlog: EventLogHandle, lpbackupfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BackupEventLogA(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupEventLogW<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupEventLogW(heventlog: EventLogHandle, lpbackupfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BackupEventLogW(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearEventLogA<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearEventLogA(heventlog: EventLogHandle, lpbackupfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ClearEventLogA(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearEventLogW<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(heventlog: Param0, lpbackupfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearEventLogW(heventlog: EventLogHandle, lpbackupfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ClearEventLogW(heventlog.into_param().abi(), lpbackupfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
impl ::core::fmt::Debug for EVENTLOGRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTLOGRECORD")
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
unsafe impl ::windows::core::Abi for EVENTLOGRECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENTLOGRECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENTLOGRECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENTLOGRECORD {}
impl ::core::default::Default for EVENTLOGRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
impl ::core::marker::Copy for EVENTLOG_FULL_INFORMATION {}
impl ::core::clone::Clone for EVENTLOG_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENTLOG_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTLOG_FULL_INFORMATION").field("dwFull", &self.dwFull).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENTLOG_FULL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENTLOG_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENTLOG_FULL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENTLOG_FULL_INFORMATION {}
impl ::core::default::Default for EVENTLOG_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
impl ::core::fmt::Debug for EVENTSFORLOGFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTSFORLOGFILE").field("ulSize", &self.ulSize).field("szLogicalLogFile", &self.szLogicalLogFile).field("ulNumRecords", &self.ulNumRecords).field("pEventLogRecords", &self.pEventLogRecords).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENTSFORLOGFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENTSFORLOGFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENTSFORLOGFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENTSFORLOGFILE {}
impl ::core::default::Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_ALL_ACCESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_CHANNEL_CLOCK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(1i32);
impl ::core::marker::Copy for EVT_CHANNEL_CLOCK_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_CLOCK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_CLOCK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_CLOCK_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_CHANNEL_CLOCK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_CLOCK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_CHANNEL_CONFIG_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(21i32);
impl ::core::marker::Copy for EVT_CHANNEL_CONFIG_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_CONFIG_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_CHANNEL_ISOLATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(2i32);
impl ::core::marker::Copy for EVT_CHANNEL_ISOLATION_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_ISOLATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_ISOLATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_ISOLATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_CHANNEL_ISOLATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_ISOLATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_CHANNEL_REFERENCE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = EVT_CHANNEL_REFERENCE_FLAGS(1i32);
impl ::core::marker::Copy for EVT_CHANNEL_REFERENCE_FLAGS {}
impl ::core::clone::Clone for EVT_CHANNEL_REFERENCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_REFERENCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_REFERENCE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_CHANNEL_REFERENCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_REFERENCE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_CHANNEL_SID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(1i32);
impl ::core::marker::Copy for EVT_CHANNEL_SID_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_SID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_SID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_SID_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_CHANNEL_SID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_SID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_CHANNEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(3i32);
impl ::core::marker::Copy for EVT_CHANNEL_TYPE {}
impl ::core::clone::Clone for EVT_CHANNEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_CHANNEL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_EVENT_METADATA_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(9i32);
impl ::core::marker::Copy for EVT_EVENT_METADATA_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_EVENT_METADATA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_EVENT_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_EVENT_METADATA_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_EVENT_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EVENT_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_EVENT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(2i32);
impl ::core::marker::Copy for EVT_EVENT_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_EVENT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_EVENT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_EVENT_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_EVENT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EVENT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_EXPORTLOG_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(8192i32);
impl ::core::marker::Copy for EVT_EXPORTLOG_FLAGS {}
impl ::core::clone::Clone for EVT_EXPORTLOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_EXPORTLOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_EXPORTLOG_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_EXPORTLOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EXPORTLOG_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_FORMAT_MESSAGE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(9i32);
impl ::core::marker::Copy for EVT_FORMAT_MESSAGE_FLAGS {}
impl ::core::clone::Clone for EVT_FORMAT_MESSAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_FORMAT_MESSAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_FORMAT_MESSAGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_FORMAT_MESSAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_FORMAT_MESSAGE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_LOGIN_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLogin: EVT_LOGIN_CLASS = EVT_LOGIN_CLASS(1i32);
impl ::core::marker::Copy for EVT_LOGIN_CLASS {}
impl ::core::clone::Clone for EVT_LOGIN_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_LOGIN_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_LOGIN_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_LOGIN_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_LOGIN_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_LOG_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(7i32);
impl ::core::marker::Copy for EVT_LOG_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_LOG_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_LOG_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_LOG_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_LOG_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_LOG_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_OPEN_LOG_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(2i32);
impl ::core::marker::Copy for EVT_OPEN_LOG_FLAGS {}
impl ::core::clone::Clone for EVT_OPEN_LOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_OPEN_LOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_OPEN_LOG_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_OPEN_LOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_OPEN_LOG_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_PUBLISHER_METADATA_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(21i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(22i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(23i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(24i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(25i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(26i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(27i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(28i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(29i32);
impl ::core::marker::Copy for EVT_PUBLISHER_METADATA_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_PUBLISHER_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_QUERY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(4096i32);
impl ::core::marker::Copy for EVT_QUERY_FLAGS {}
impl ::core::clone::Clone for EVT_QUERY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_QUERY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_QUERY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_QUERY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_QUERY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_QUERY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(2i32);
impl ::core::marker::Copy for EVT_QUERY_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_QUERY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_QUERY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_QUERY_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_QUERY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_QUERY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_READ_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_RENDER_CONTEXT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(2i32);
impl ::core::marker::Copy for EVT_RENDER_CONTEXT_FLAGS {}
impl ::core::clone::Clone for EVT_RENDER_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_RENDER_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_RENDER_CONTEXT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_RENDER_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RENDER_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_RENDER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(2i32);
impl ::core::marker::Copy for EVT_RENDER_FLAGS {}
impl ::core::clone::Clone for EVT_RENDER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_RENDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_RENDER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_RENDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RENDER_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub struct EVT_RPC_LOGIN {
    pub Server: ::windows::core::PWSTR,
    pub User: ::windows::core::PWSTR,
    pub Domain: ::windows::core::PWSTR,
    pub Password: ::windows::core::PWSTR,
    pub Flags: u32,
}
impl ::core::marker::Copy for EVT_RPC_LOGIN {}
impl ::core::clone::Clone for EVT_RPC_LOGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVT_RPC_LOGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVT_RPC_LOGIN").field("Server", &self.Server).field("User", &self.User).field("Domain", &self.Domain).field("Password", &self.Password).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for EVT_RPC_LOGIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVT_RPC_LOGIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVT_RPC_LOGIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVT_RPC_LOGIN {}
impl ::core::default::Default for EVT_RPC_LOGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_RPC_LOGIN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(3i32);
impl ::core::marker::Copy for EVT_RPC_LOGIN_FLAGS {}
impl ::core::clone::Clone for EVT_RPC_LOGIN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_RPC_LOGIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_RPC_LOGIN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_RPC_LOGIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RPC_LOGIN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_SEEK_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSeekStrict: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(65536i32);
impl ::core::marker::Copy for EVT_SEEK_FLAGS {}
impl ::core::clone::Clone for EVT_SEEK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SEEK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_SEEK_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_SEEK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SEEK_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub type EVT_SUBSCRIBE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const ::core::ffi::c_void, event: isize) -> u32>;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_SUBSCRIBE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(65536i32);
impl ::core::marker::Copy for EVT_SUBSCRIBE_FLAGS {}
impl ::core::clone::Clone for EVT_SUBSCRIBE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SUBSCRIBE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_SUBSCRIBE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_SUBSCRIBE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SUBSCRIBE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_SUBSCRIBE_NOTIFY_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(1i32);
impl ::core::marker::Copy for EVT_SUBSCRIBE_NOTIFY_ACTION {}
impl ::core::clone::Clone for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_SUBSCRIBE_NOTIFY_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SUBSCRIBE_NOTIFY_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_SYSTEM_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(18i32);
impl ::core::marker::Copy for EVT_SYSTEM_PROPERTY_ID {}
impl ::core::clone::Clone for EVT_SYSTEM_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_SYSTEM_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_SYSTEM_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_SYSTEM_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SYSTEM_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVT_VARIANT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVT_VARIANT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVT_VARIANT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVT_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
    pub StringVal: ::windows::core::PCWSTR,
    pub AnsiStringVal: ::windows::core::PCSTR,
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
    pub StringArr: *mut ::windows::core::PWSTR,
    pub AnsiStringArr: *mut ::windows::core::PSTR,
    pub SidArr: *mut super::super::Foundation::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: isize,
    pub XmlVal: ::windows::core::PCWSTR,
    pub XmlValArr: *mut ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVT_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVT_VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVT_VARIANT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVT_VARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVT_VARIANT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVT_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVT_VARIANT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(35i32);
impl ::core::marker::Copy for EVT_VARIANT_TYPE {}
impl ::core::clone::Clone for EVT_VARIANT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVT_VARIANT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVT_VARIANT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVT_VARIANT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_VARIANT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVT_WRITE_ACCESS: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EventLogHandle(pub isize);
impl EventLogHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for EventLogHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for EventLogHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for EventLogHandle {}
impl ::core::fmt::Debug for EventLogHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventLogHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for EventLogHandle {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EventSourceHandle(pub isize);
impl EventSourceHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for EventSourceHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for EventSourceHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for EventSourceHandle {}
impl ::core::fmt::Debug for EventSourceHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventSourceHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for EventSourceHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtArchiveExportedLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, logfilepath: Param1, locale: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtArchiveExportedLog(session: isize, logfilepath: ::windows::core::PCWSTR, locale: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtArchiveExportedLog(::core::mem::transmute(session), logfilepath.into_param().abi(), ::core::mem::transmute(locale), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtClearLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, channelpath: Param1, targetfilepath: Param2, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtClearLog(session: isize, channelpath: ::windows::core::PCWSTR, targetfilepath: ::windows::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtClearLog(::core::mem::transmute(session), channelpath.into_param().abi(), targetfilepath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtCreateBookmark<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(bookmarkxml: Param0) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtCreateBookmark(bookmarkxml: ::windows::core::PCWSTR) -> isize;
        }
        ::core::mem::transmute(EvtCreateBookmark(bookmarkxml.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtCreateRenderContext(valuepaths: &[::windows::core::PWSTR], flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtCreateRenderContext(valuepathscount: u32, valuepaths: *const ::windows::core::PWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtCreateRenderContext(valuepaths.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(valuepaths)), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtExportLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, path: Param1, query: Param2, targetfilepath: Param3, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtExportLog(session: isize, path: ::windows::core::PCWSTR, query: ::windows::core::PCWSTR, targetfilepath: ::windows::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtExportLog(::core::mem::transmute(session), path.into_param().abi(), query.into_param().abi(), targetfilepath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtFormatMessage(publishermetadata: isize, event: isize, messageid: u32, values: &[EVT_VARIANT], flags: u32, buffer: &mut [u16], bufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtFormatMessage(publishermetadata: isize, event: isize, messageid: u32, valuecount: u32, values: *const EVT_VARIANT, flags: u32, buffersize: u32, buffer: ::windows::core::PWSTR, bufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtFormatMessage(::core::mem::transmute(publishermetadata), ::core::mem::transmute(event), ::core::mem::transmute(messageid), values.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(values)), ::core::mem::transmute(flags), buffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), ::core::mem::transmute(bufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtGetExtendedStatus(buffer: &mut [u16], bufferused: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtGetExtendedStatus(buffersize: u32, buffer: ::windows::core::PWSTR, bufferused: *mut u32) -> u32;
        }
        ::core::mem::transmute(EvtGetExtendedStatus(buffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), ::core::mem::transmute(bufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNext(resultset: isize, events: &mut [isize], timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNext(resultset: isize, eventssize: u32, events: *mut isize, timeout: u32, flags: u32, returned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtNext(::core::mem::transmute(resultset), events.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(events)), ::core::mem::transmute(timeout), ::core::mem::transmute(flags), ::core::mem::transmute(returned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNextChannelPath(channelenum: isize, channelpathbuffer: &mut [u16], channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNextChannelPath(channelenum: isize, channelpathbuffersize: u32, channelpathbuffer: ::windows::core::PWSTR, channelpathbufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtNextChannelPath(::core::mem::transmute(channelenum), channelpathbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(channelpathbuffer)), ::core::mem::transmute(channelpathbufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtNextPublisherId(publisherenum: isize, publisheridbuffer: &mut [u16], publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtNextPublisherId(publisherenum: isize, publisheridbuffersize: u32, publisheridbuffer: ::windows::core::PWSTR, publisheridbufferused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EvtNextPublisherId(::core::mem::transmute(publisherenum), publisheridbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(publisheridbuffer)), ::core::mem::transmute(publisheridbufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenChannelConfig<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, channelpath: Param1, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenChannelConfig(session: isize, channelpath: ::windows::core::PCWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenChannelConfig(::core::mem::transmute(session), channelpath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, path: Param1, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenLog(session: isize, path: ::windows::core::PCWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenLog(::core::mem::transmute(session), path.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtOpenPublisherMetadata<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, publisherid: Param1, logfilepath: Param2, locale: u32, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtOpenPublisherMetadata(session: isize, publisherid: ::windows::core::PCWSTR, logfilepath: ::windows::core::PCWSTR, locale: u32, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtOpenPublisherMetadata(::core::mem::transmute(session), publisherid.into_param().abi(), logfilepath.into_param().abi(), ::core::mem::transmute(locale), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn EvtQuery<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, path: Param1, query: Param2, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtQuery(session: isize, path: ::windows::core::PCWSTR, query: ::windows::core::PCWSTR, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtQuery(::core::mem::transmute(session), path.into_param().abi(), query.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvtSubscribe<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(session: isize, signalevent: Param1, channelpath: Param2, query: Param3, bookmark: isize, context: *const ::core::ffi::c_void, callback: EVT_SUBSCRIBE_CALLBACK, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvtSubscribe(session: isize, signalevent: super::super::Foundation::HANDLE, channelpath: ::windows::core::PCWSTR, query: ::windows::core::PCWSTR, bookmark: isize, context: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr, flags: u32) -> isize;
        }
        ::core::mem::transmute(EvtSubscribe(::core::mem::transmute(session), signalevent.into_param().abi(), channelpath.into_param().abi(), query.into_param().abi(), ::core::mem::transmute(bookmark), ::core::mem::transmute(context), ::core::mem::transmute(callback), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEventLogInformation<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>>(heventlog: Param0, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEventLogInformation(heventlog: EventLogHandle, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetEventLogInformation(heventlog.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfEventLogRecords<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>>(heventlog: Param0, numberofrecords: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfEventLogRecords(heventlog: EventLogHandle, numberofrecords: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumberOfEventLogRecords(heventlog.into_param().abi(), ::core::mem::transmute(numberofrecords)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOldestEventLogRecord<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>>(heventlog: Param0, oldestrecord: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOldestEventLogRecord(heventlog: EventLogHandle, oldestrecord: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOldestEventLogRecord(heventlog.into_param().abi(), ::core::mem::transmute(oldestrecord)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyChangeEventLog<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(heventlog: Param0, hevent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyChangeEventLog(heventlog: EventLogHandle, hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(NotifyChangeEventLog(heventlog.into_param().abi(), hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenBackupEventLogA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpuncservername: Param0, lpfilename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenBackupEventLogA(lpuncservername: ::windows::core::PCSTR, lpfilename: ::windows::core::PCSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenBackupEventLogA(lpuncservername.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenBackupEventLogW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpuncservername: Param0, lpfilename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenBackupEventLogW(lpuncservername: ::windows::core::PCWSTR, lpfilename: ::windows::core::PCWSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenBackupEventLogW(lpuncservername.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenEventLogA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventLogA(lpuncservername: ::windows::core::PCSTR, lpsourcename: ::windows::core::PCSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenEventLogA(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn OpenEventLogW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventLogHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventLogW(lpuncservername: ::windows::core::PCWSTR, lpsourcename: ::windows::core::PCWSTR) -> EventLogHandle;
        }
        ::core::mem::transmute(OpenEventLogW(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct READ_EVENT_LOG_READ_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_SEEK_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_SEQUENTIAL_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(1u32);
impl ::core::marker::Copy for READ_EVENT_LOG_READ_FLAGS {}
impl ::core::clone::Clone for READ_EVENT_LOG_READ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for READ_EVENT_LOG_READ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for READ_EVENT_LOG_READ_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for READ_EVENT_LOG_READ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READ_EVENT_LOG_READ_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REPORT_EVENT_TYPE(pub u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(0u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_AUDIT_FAILURE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(16u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_AUDIT_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(8u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_ERROR_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(1u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_INFORMATION_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(4u16);
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
pub const EVENTLOG_WARNING_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(2u16);
impl ::core::marker::Copy for REPORT_EVENT_TYPE {}
impl ::core::clone::Clone for REPORT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPORT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REPORT_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for REPORT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPORT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadEventLogA<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>>(heventlog: Param0, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadEventLogA(heventlog: EventLogHandle, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadEventLogA(heventlog.into_param().abi(), ::core::mem::transmute(dwreadflags), ::core::mem::transmute(dwrecordoffset), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytestoread), ::core::mem::transmute(pnbytesread), ::core::mem::transmute(pnminnumberofbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadEventLogW<'a, Param0: ::windows::core::IntoParam<'a, EventLogHandle>>(heventlog: Param0, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadEventLogW(heventlog: EventLogHandle, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadEventLogW(heventlog.into_param().abi(), ::core::mem::transmute(dwreadflags), ::core::mem::transmute(dwrecordoffset), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytestoread), ::core::mem::transmute(pnbytesread), ::core::mem::transmute(pnminnumberofbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn RegisterEventSourceA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventSourceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterEventSourceA(lpuncservername: ::windows::core::PCSTR, lpsourcename: ::windows::core::PCSTR) -> EventSourceHandle;
        }
        ::core::mem::transmute(RegisterEventSourceA(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`*"]
#[inline]
pub unsafe fn RegisterEventSourceW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpuncservername: Param0, lpsourcename: Param1) -> EventSourceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterEventSourceW(lpuncservername: ::windows::core::PCWSTR, lpsourcename: ::windows::core::PCWSTR) -> EventSourceHandle;
        }
        ::core::mem::transmute(RegisterEventSourceW(lpuncservername.into_param().abi(), lpsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportEventA<'a, Param0: ::windows::core::IntoParam<'a, EventSourceHandle>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(heventlog: Param0, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: Param4, dwdatasize: u32, lpstrings: &[::windows::core::PSTR], lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReportEventA(heventlog: EventSourceHandle, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const ::windows::core::PSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReportEventA(heventlog.into_param().abi(), ::core::mem::transmute(wtype), ::core::mem::transmute(wcategory), ::core::mem::transmute(dweventid), lpusersid.into_param().abi(), lpstrings.len() as _, ::core::mem::transmute(dwdatasize), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpstrings)), ::core::mem::transmute(lprawdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventLog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportEventW<'a, Param0: ::windows::core::IntoParam<'a, EventSourceHandle>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(heventlog: Param0, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: Param4, dwdatasize: u32, lpstrings: &[::windows::core::PWSTR], lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReportEventW(heventlog: EventSourceHandle, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: super::super::Foundation::PSID, wnumstrings: u16, dwdatasize: u32, lpstrings: *const ::windows::core::PWSTR, lprawdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReportEventW(heventlog.into_param().abi(), ::core::mem::transmute(wtype), ::core::mem::transmute(wcategory), ::core::mem::transmute(dweventid), lpusersid.into_param().abi(), lpstrings.len() as _, ::core::mem::transmute(dwdatasize), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpstrings)), ::core::mem::transmute(lprawdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
