impl ::core::default::Default for EVENTLOGRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENTLOGRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Reserved == other.Reserved && self.RecordNumber == other.RecordNumber && self.TimeGenerated == other.TimeGenerated && self.TimeWritten == other.TimeWritten && self.EventID == other.EventID && self.EventType == other.EventType && self.NumStrings == other.NumStrings && self.EventCategory == other.EventCategory && self.ReservedFlags == other.ReservedFlags && self.ClosingRecordNumber == other.ClosingRecordNumber && self.StringOffset == other.StringOffset && self.UserSidLength == other.UserSidLength && self.UserSidOffset == other.UserSidOffset && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for EVENTLOGRECORD {}
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
impl ::core::default::Default for EVENTLOG_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENTLOG_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFull == other.dwFull
    }
}
impl ::core::cmp::Eq for EVENTLOG_FULL_INFORMATION {}
impl ::core::fmt::Debug for EVENTLOG_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTLOG_FULL_INFORMATION").field("dwFull", &self.dwFull).finish()
    }
}
impl ::core::default::Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENTSFORLOGFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.szLogicalLogFile == other.szLogicalLogFile && self.ulNumRecords == other.ulNumRecords && self.pEventLogRecords == other.pEventLogRecords
    }
}
impl ::core::cmp::Eq for EVENTSFORLOGFILE {}
impl ::core::fmt::Debug for EVENTSFORLOGFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTSFORLOGFILE").field("ulSize", &self.ulSize).field("szLogicalLogFile", &self.szLogicalLogFile).field("ulNumRecords", &self.ulNumRecords).field("pEventLogRecords", &self.pEventLogRecords).finish()
    }
}
impl ::core::default::Default for EVT_CHANNEL_CLOCK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_CHANNEL_CLOCK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_CLOCK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_CHANNEL_CONFIG_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_CONFIG_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_CHANNEL_ISOLATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_CHANNEL_ISOLATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_ISOLATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_CHANNEL_REFERENCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_CHANNEL_REFERENCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_REFERENCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_CHANNEL_SID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_CHANNEL_SID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_SID_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_CHANNEL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_EVENT_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_EVENT_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EVENT_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_EVENT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_EVENT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EVENT_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_EXPORTLOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_EXPORTLOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_EXPORTLOG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_FORMAT_MESSAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_FORMAT_MESSAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_FORMAT_MESSAGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_LOGIN_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_LOGIN_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_LOGIN_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_LOG_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_LOG_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_LOG_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_OPEN_LOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_OPEN_LOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_OPEN_LOG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_PUBLISHER_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_PUBLISHER_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_QUERY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_QUERY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_QUERY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_QUERY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_QUERY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_QUERY_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_RENDER_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_RENDER_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RENDER_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_RENDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_RENDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RENDER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_RPC_LOGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVT_RPC_LOGIN {
    fn eq(&self, other: &Self) -> bool {
        self.Server == other.Server && self.User == other.User && self.Domain == other.Domain && self.Password == other.Password && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for EVT_RPC_LOGIN {}
impl ::core::fmt::Debug for EVT_RPC_LOGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVT_RPC_LOGIN").field("Server", &self.Server).field("User", &self.User).field("Domain", &self.Domain).field("Password", &self.Password).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for EVT_RPC_LOGIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_RPC_LOGIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_RPC_LOGIN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_SEEK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_SEEK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SEEK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_SUBSCRIBE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_SUBSCRIBE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SUBSCRIBE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_SUBSCRIBE_NOTIFY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SUBSCRIBE_NOTIFY_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVT_SYSTEM_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_SYSTEM_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_SYSTEM_PROPERTY_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EVT_VARIANT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVT_VARIANT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVT_VARIANT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for READ_EVENT_LOG_READ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for READ_EVENT_LOG_READ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READ_EVENT_LOG_READ_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for REPORT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REPORT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPORT_EVENT_TYPE").field(&self.0).finish()
    }
}
