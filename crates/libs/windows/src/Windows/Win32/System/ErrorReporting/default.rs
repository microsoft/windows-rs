impl ::core::default::Default for EFaultRepRetVal {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EFaultRepRetVal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EFaultRepRetVal").field(&self.0).finish()
    }
}
impl ::core::default::Default for REPORT_STORE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REPORT_STORE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPORT_STORE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WER_CONSENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_CONSENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_CONSENT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.dwDumpFlags == other.dwDumpFlags && self.bOnlyThisThread == other.bOnlyThisThread && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags && self.dwOtherThreadFlags == other.dwOtherThreadFlags && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags && self.dwOtherModuleFlags == other.dwOtherModuleFlags && self.wzPreferredModuleList == other.wzPreferredModuleList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.dwDumpFlags == other.dwDumpFlags && self.bOnlyThisThread == other.bOnlyThisThread && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags && self.dwOtherThreadFlags == other.dwOtherThreadFlags && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags && self.dwOtherModuleFlags == other.dwOtherModuleFlags && self.wzPreferredModuleList == other.wzPreferredModuleList && self.dwPreferredModuleResetFlags == other.dwPreferredModuleResetFlags && self.dwOtherModuleResetFlags == other.dwOtherModuleResetFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V2")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwMask == other.dwMask
            && self.dwDumpFlags == other.dwDumpFlags
            && self.bOnlyThisThread == other.bOnlyThisThread
            && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags
            && self.dwOtherThreadFlags == other.dwOtherThreadFlags
            && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags
            && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags
            && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags
            && self.dwOtherModuleFlags == other.dwOtherModuleFlags
            && self.wzPreferredModuleList == other.wzPreferredModuleList
            && self.dwPreferredModuleResetFlags == other.dwPreferredModuleResetFlags
            && self.dwOtherModuleResetFlags == other.dwOtherModuleResetFlags
            && self.pvDumpKey == other.pvDumpKey
            && self.hSnapshot == other.hSnapshot
            && self.dwThreadID == other.dwThreadID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V3")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .field("pvDumpKey", &self.pvDumpKey)
            .field("hSnapshot", &self.hSnapshot)
            .field("dwThreadID", &self.dwThreadID)
            .finish()
    }
}
impl ::core::default::Default for WER_DUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_DUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_DUMP_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for WER_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for WER_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.pExceptionPointers == other.pExceptionPointers && self.bClientPointers == other.bClientPointers
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for WER_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_EXCEPTION_INFORMATION").field("pExceptionPointers", &self.pExceptionPointers).field("bClientPointers", &self.bClientPointers).finish()
    }
}
impl ::core::default::Default for WER_FAULT_REPORTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_FAULT_REPORTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FAULT_REPORTING").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_FAULT_REPORTING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_FAULT_REPORTING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_FAULT_REPORTING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WER_FILE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FILE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WER_FILE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_FILE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_FILE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_FILE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_FILE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WER_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FILE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WER_REGISTER_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_REGISTER_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REGISTER_FILE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION").field("dwSize", &self.dwSize).field("hProcess", &self.hProcess).field("wzConsentKey", &self.wzConsentKey).field("wzFriendlyEventName", &self.wzFriendlyEventName).field("wzApplicationName", &self.wzApplicationName).field("wzApplicationPath", &self.wzApplicationPath).field("wzDescription", &self.wzDescription).field("hwndParent", &self.hwndParent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V3")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup && self.rgbApplicationIdentity == other.rgbApplicationIdentity && self.hSnapshot == other.hSnapshot && self.hDeleteFilesImpersonationToken == other.hDeleteFilesImpersonationToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V4")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup && self.rgbApplicationIdentity == other.rgbApplicationIdentity && self.hSnapshot == other.hSnapshot && self.hDeleteFilesImpersonationToken == other.hDeleteFilesImpersonationToken && self.submitResultMax == other.submitResultMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V5")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .field("submitResultMax", &self.submitResultMax)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_METADATA_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_METADATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V1").field("Signature", &self.Signature).field("BucketId", &self.BucketId).field("ReportId", &self.ReportId).field("CreationTime", &self.CreationTime).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes && self.CabId == other.CabId && self.ReportStatus == other.ReportStatus && self.ReportIntegratorId == other.ReportIntegratorId && self.NumberOfFiles == other.NumberOfFiles && self.SizeOfFileNames == other.SizeOfFileNames && self.FileNames == other.FileNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_METADATA_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_METADATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V2")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_METADATA_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes && self.CabId == other.CabId && self.ReportStatus == other.ReportStatus && self.ReportIntegratorId == other.ReportIntegratorId && self.NumberOfFiles == other.NumberOfFiles && self.SizeOfFileNames == other.SizeOfFileNames && self.FileNames == other.FileNames && self.FriendlyEventName == other.FriendlyEventName && self.ApplicationName == other.ApplicationName && self.ApplicationPath == other.ApplicationPath && self.Description == other.Description && self.BucketIdString == other.BucketIdString && self.LegacyBucketId == other.LegacyBucketId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_METADATA_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_METADATA_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V3")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .field("FriendlyEventName", &self.FriendlyEventName)
            .field("ApplicationName", &self.ApplicationName)
            .field("ApplicationPath", &self.ApplicationPath)
            .field("Description", &self.Description)
            .field("BucketIdString", &self.BucketIdString)
            .field("LegacyBucketId", &self.LegacyBucketId)
            .finish()
    }
}
impl ::core::default::Default for WER_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WER_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WER_REPORT_PARAMETER {}
impl ::core::fmt::Debug for WER_REPORT_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_PARAMETER").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for WER_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WER_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.EventName == other.EventName && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for WER_REPORT_SIGNATURE {}
impl ::core::fmt::Debug for WER_REPORT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_SIGNATURE").field("EventName", &self.EventName).field("Parameters", &self.Parameters).finish()
    }
}
impl ::core::default::Default for WER_REPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_REPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WER_REPORT_UI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_REPORT_UI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REPORT_UI").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WER_SUBMIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_SUBMIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_SUBMIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_SUBMIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WER_SUBMIT_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WER_SUBMIT_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_RESULT").field(&self.0).finish()
    }
}
