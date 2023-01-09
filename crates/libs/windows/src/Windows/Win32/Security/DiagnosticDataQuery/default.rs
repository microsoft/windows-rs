impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.moduleName == other.moduleName && self.friendlyModuleName == other.friendlyModuleName && self.eventCount == other.eventCount && self.uploadSizeBytes == other.uploadSizeBytes
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_BINARY_STATS").field("moduleName", &self.moduleName).field("friendlyModuleName", &self.friendlyModuleName).field("eventCount", &self.eventCount).field("uploadSizeBytes", &self.uploadSizeBytes).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION").field("id", &self.id).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION").field("name", &self.name).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.privacyTag == other.privacyTag && self.name == other.name && self.description == other.description
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION").field("privacyTag", &self.privacyTag).field("name", &self.name).field("description", &self.description).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.privacyTag == other.privacyTag && self.eventCount == other.eventCount
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_STATS").field("privacyTag", &self.privacyTag).field("eventCount", &self.eventCount).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.hoursOfHistoryToKeep == other.hoursOfHistoryToKeep && self.maxStoreMegabytes == other.maxStoreMegabytes && self.requestedMaxStoreMegabytes == other.requestedMaxStoreMegabytes
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION").field("hoursOfHistoryToKeep", &self.hoursOfHistoryToKeep).field("maxStoreMegabytes", &self.maxStoreMegabytes).field("requestedMaxStoreMegabytes", &self.requestedMaxStoreMegabytes).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.optInLevel == other.optInLevel && self.transcriptSizeBytes == other.transcriptSizeBytes && self.oldestEventTimestamp == other.oldestEventTimestamp && self.totalEventCountLast24Hours == other.totalEventCountLast24Hours && self.averageDailyEvents == other.averageDailyEvents
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_GENERAL_STATS").field("optInLevel", &self.optInLevel).field("transcriptSizeBytes", &self.transcriptSizeBytes).field("oldestEventTimestamp", &self.oldestEventTimestamp).field("totalEventCountLast24Hours", &self.totalEventCountLast24Hours).field("averageDailyEvents", &self.averageDailyEvents).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.rowId == other.rowId && self.timestamp == other.timestamp && self.eventKeywords == other.eventKeywords && self.fullEventName == other.fullEventName && self.providerGroupGuid == other.providerGroupGuid && self.producerName == other.producerName && self.privacyTags == other.privacyTags && self.privacyTagCount == other.privacyTagCount && self.categoryIds == other.categoryIds && self.categoryIdCount == other.categoryIdCount && self.isCoreData == other.isCoreData && self.extra1 == other.extra1 && self.extra2 == other.extra2 && self.extra3 == other.extra3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_RECORD")
            .field("rowId", &self.rowId)
            .field("timestamp", &self.timestamp)
            .field("eventKeywords", &self.eventKeywords)
            .field("fullEventName", &self.fullEventName)
            .field("providerGroupGuid", &self.providerGroupGuid)
            .field("producerName", &self.producerName)
            .field("privacyTags", &self.privacyTags)
            .field("privacyTagCount", &self.privacyTagCount)
            .field("categoryIds", &self.categoryIds)
            .field("categoryIdCount", &self.categoryIdCount)
            .field("isCoreData", &self.isCoreData)
            .field("extra1", &self.extra1)
            .field("extra2", &self.extra2)
            .field("extra3", &self.extra3)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.producerNames == other.producerNames && self.producerNameCount == other.producerNameCount && self.textToMatch == other.textToMatch && self.categoryIds == other.categoryIds && self.categoryIdCount == other.categoryIdCount && self.privacyTags == other.privacyTags && self.privacyTagCount == other.privacyTagCount && self.coreDataOnly == other.coreDataOnly
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_SEARCH_CRITERIA").field("producerNames", &self.producerNames).field("producerNameCount", &self.producerNameCount).field("textToMatch", &self.textToMatch).field("categoryIds", &self.categoryIds).field("categoryIdCount", &self.categoryIdCount).field("privacyTags", &self.privacyTags).field("privacyTagCount", &self.privacyTagCount).field("coreDataOnly", &self.coreDataOnly).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature && self.bucketId == other.bucketId && self.reportId == other.reportId && self.creationTime == other.creationTime && self.sizeInBytes == other.sizeInBytes && self.cabId == other.cabId && self.reportStatus == other.reportStatus && self.reportIntegratorId == other.reportIntegratorId && self.fileNames == other.fileNames && self.fileCount == other.fileCount && self.friendlyEventName == other.friendlyEventName && self.applicationName == other.applicationName && self.applicationPath == other.applicationPath && self.description == other.description && self.bucketIdString == other.bucketIdString && self.legacyBucketId == other.legacyBucketId && self.reportKey == other.reportKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAGNOSTIC_REPORT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_REPORT_DATA")
            .field("signature", &self.signature)
            .field("bucketId", &self.bucketId)
            .field("reportId", &self.reportId)
            .field("creationTime", &self.creationTime)
            .field("sizeInBytes", &self.sizeInBytes)
            .field("cabId", &self.cabId)
            .field("reportStatus", &self.reportStatus)
            .field("reportIntegratorId", &self.reportIntegratorId)
            .field("fileNames", &self.fileNames)
            .field("fileCount", &self.fileCount)
            .field("friendlyEventName", &self.friendlyEventName)
            .field("applicationName", &self.applicationName)
            .field("applicationPath", &self.applicationPath)
            .field("description", &self.description)
            .field("bucketIdString", &self.bucketIdString)
            .field("legacyBucketId", &self.legacyBucketId)
            .field("reportKey", &self.reportKey)
            .finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_PARAMETER {}
impl ::core::fmt::Debug for DIAGNOSTIC_REPORT_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_REPORT_PARAMETER").field("name", &self.name).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.eventName == other.eventName && self.parameters == other.parameters
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::core::fmt::Debug for DIAGNOSTIC_REPORT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_REPORT_SIGNATURE").field("eventName", &self.eventName).field("parameters", &self.parameters).finish()
    }
}
impl ::core::default::Default for DdqAccessLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DdqAccessLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DdqAccessLevel").field(&self.0).finish()
    }
}
