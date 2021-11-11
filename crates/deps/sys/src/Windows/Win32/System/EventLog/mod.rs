#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupEventLogA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupEventLogW();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearEventLogA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearEventLogW();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseEventLog();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterEventSource();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtArchiveExportedLog();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCancel();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtClearLog();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtClose();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCreateBookmark();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtCreateRenderContext();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtExportLog();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtFormatMessage();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetChannelConfigProperty();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetEventInfo();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetEventMetadataProperty();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetExtendedStatus();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetLogInfo();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetObjectArrayProperty();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetObjectArraySize();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetPublisherMetadataProperty();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtGetQueryInfo();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNext();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNextChannelPath();
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtNextEventMetadata();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtNextPublisherId();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenChannelConfig();
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenChannelEnum();
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenEventMetadataEnum();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenLog();
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenPublisherEnum();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtOpenPublisherMetadata();
    #[doc = "*Required features: `Win32_System_EventLog`*"]
    pub fn EvtOpenSession();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtQuery();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtRender();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSaveChannelConfig();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSeek();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSetChannelConfigProperty();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtSubscribe();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvtUpdateBookmark();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEventLogInformation();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfEventLogRecords();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOldestEventLogRecord();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyChangeEventLog();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenBackupEventLogA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenBackupEventLogW();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventLogA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventLogW();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadEventLogA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadEventLogW();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterEventSourceA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterEventSourceW();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportEventA();
    #[doc = "*Required features: `Win32_System_EventLog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportEventW();
}
