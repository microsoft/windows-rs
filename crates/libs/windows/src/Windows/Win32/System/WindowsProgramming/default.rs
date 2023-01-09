#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulDataFormatVersion == other.ulDataFormatVersion && self.lpData == other.lpData && self.ulLength == other.ulLength && self.lpSectionGlobalData == other.lpSectionGlobalData && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength && self.lpSectionBase == other.lpSectionBase && self.ulSectionTotalLength == other.ulSectionTotalLength && self.hActCtx == other.hActCtx && self.ulAssemblyRosterIndex == other.ulAssemblyRosterIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_2600")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .finish()
    }
}
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpInformation == other.lpInformation && self.lpSectionBase == other.lpSectionBase && self.ulSectionLength == other.ulSectionLength && self.lpSectionGlobalDataBase == other.lpSectionGlobalDataBase && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength
    }
}
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA").field("lpInformation", &self.lpInformation).field("lpSectionBase", &self.lpSectionBase).field("ulSectionLength", &self.ulSectionLength).field("lpSectionGlobalDataBase", &self.lpSectionGlobalDataBase).field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hActCtx == other.hActCtx && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_BASIC_INFORMATION").field("hActCtx", &self.hActCtx).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CABINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CABINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab && self.pszInf == other.pszInf && self.pszSection == other.pszSection && self.szSrcPath == other.szSrcPath && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CABINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CABINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOA").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CABINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CABINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab && self.pszInf == other.pszInf && self.pszSection == other.pszSection && self.szSrcPath == other.szSrcPath && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CABINFOW {}
impl ::core::fmt::Debug for CABINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOW").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueProcess == other.UniqueProcess && self.UniqueThread == other.UniqueThread
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_ID").field("UniqueProcess", &self.UniqueProcess).field("UniqueThread", &self.UniqueThread).finish()
    }
}
impl ::core::default::Default for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TriggerId == other.TriggerId
    }
}
impl ::core::cmp::Eq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl ::core::fmt::Debug for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG").field("Size", &self.Size).field("TriggerId", &self.TriggerId).finish()
    }
}
impl ::core::default::Default for CameraUIControlCaptureMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlCaptureMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for CameraUIControlLinearSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlLinearSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlLinearSelectionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for CameraUIControlMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for CameraUIControlPhotoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlPhotoFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for CameraUIControlVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlVideoFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for CameraUIControlViewType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlViewType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlViewType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.min == other.min && self.sec == other.sec
    }
}
impl ::core::cmp::Eq for DATETIME {}
impl ::core::fmt::Debug for DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATETIME").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("min", &self.min).field("sec", &self.sec).finish()
    }
}
impl ::core::default::Default for DCICMD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DCICMD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommand == other.dwCommand && self.dwParam1 == other.dwParam1 && self.dwParam2 == other.dwParam2 && self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DCICMD {}
impl ::core::fmt::Debug for DCICMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICMD").field("dwCommand", &self.dwCommand).field("dwParam1", &self.dwParam1).field("dwParam2", &self.dwParam2).field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for DCICREATEINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DCICREATEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.dwCompression == other.dwCompression && self.dwMask == other.dwMask && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwDCICaps == other.dwDCICaps && self.dwBitCount == other.dwBitCount && self.lpSurface == other.lpSurface
    }
}
impl ::core::cmp::Eq for DCICREATEINPUT {}
impl ::core::fmt::Debug for DCICREATEINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICREATEINPUT").field("cmd", &self.cmd).field("dwCompression", &self.dwCompression).field("dwMask", &self.dwMask).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwDCICaps", &self.dwDCICaps).field("dwBitCount", &self.dwBitCount).field("lpSurface", &self.lpSurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DCIENUMINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DCIENUMINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.rSrc == other.rSrc && self.rDst == other.rDst && self.EnumCallback == other.EnumCallback && self.lpContext == other.lpContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DCIENUMINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIENUMINPUT").field("cmd", &self.cmd).field("rSrc", &self.rSrc).field("rDst", &self.rDst).field("EnumCallback", &self.EnumCallback).field("lpContext", &self.lpContext).finish()
    }
}
impl ::core::default::Default for DCIOFFSCREEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DCIOFFSCREEN {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo && self.Draw == other.Draw && self.SetClipList == other.SetClipList && self.SetDestination == other.SetDestination
    }
}
impl ::core::cmp::Eq for DCIOFFSCREEN {}
impl ::core::fmt::Debug for DCIOFFSCREEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOFFSCREEN").field("dciInfo", &self.dciInfo).field("Draw", &self.Draw).field("SetClipList", &self.SetClipList).field("SetDestination", &self.SetDestination).finish()
    }
}
impl ::core::default::Default for DCIOVERLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DCIOVERLAY {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo && self.dwChromakeyValue == other.dwChromakeyValue && self.dwChromakeyMask == other.dwChromakeyMask
    }
}
impl ::core::cmp::Eq for DCIOVERLAY {}
impl ::core::fmt::Debug for DCIOVERLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOVERLAY").field("dciInfo", &self.dciInfo).field("dwChromakeyValue", &self.dwChromakeyValue).field("dwChromakeyMask", &self.dwChromakeyMask).finish()
    }
}
impl ::core::default::Default for DCISURFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DCISURFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwDCICaps == other.dwDCICaps && self.dwCompression == other.dwCompression && self.dwMask == other.dwMask && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.lStride == other.lStride && self.dwBitCount == other.dwBitCount && self.dwOffSurface == other.dwOffSurface && self.wSelSurface == other.wSelSurface && self.wReserved == other.wReserved && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3 && self.BeginAccess == other.BeginAccess && self.EndAccess == other.EndAccess && self.DestroySurface == other.DestroySurface
    }
}
impl ::core::cmp::Eq for DCISURFACEINFO {}
impl ::core::fmt::Debug for DCISURFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCISURFACEINFO")
            .field("dwSize", &self.dwSize)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lStride", &self.lStride)
            .field("dwBitCount", &self.dwBitCount)
            .field("dwOffSurface", &self.dwOffSurface)
            .field("wSelSurface", &self.wSelSurface)
            .field("wReserved", &self.wReserved)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("BeginAccess", &self.BeginAccess)
            .field("EndAccess", &self.EndAccess)
            .field("DestroySurface", &self.DestroySurface)
            .finish()
    }
}
impl ::core::default::Default for DECISION_LOCATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DECISION_LOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DECISION_LOCATION").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FEATURE_CHANGE_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEATURE_CHANGE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_CHANGE_TIME").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEATURE_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEATURE_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_ENABLED_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEATURE_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FEATURE_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.lineNumber == other.lineNumber && self.file == other.file && self.process == other.process && self.module == other.module && self.callerReturnAddressOffset == other.callerReturnAddressOffset && self.callerModule == other.callerModule && self.message == other.message && self.originLineNumber == other.originLineNumber && self.originFile == other.originFile && self.originModule == other.originModule && self.originCallerReturnAddressOffset == other.originCallerReturnAddressOffset && self.originCallerModule == other.originCallerModule && self.originName == other.originName
    }
}
impl ::core::cmp::Eq for FEATURE_ERROR {}
impl ::core::fmt::Debug for FEATURE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FEATURE_ERROR")
            .field("hr", &self.hr)
            .field("lineNumber", &self.lineNumber)
            .field("file", &self.file)
            .field("process", &self.process)
            .field("module", &self.module)
            .field("callerReturnAddressOffset", &self.callerReturnAddressOffset)
            .field("callerModule", &self.callerModule)
            .field("message", &self.message)
            .field("originLineNumber", &self.originLineNumber)
            .field("originFile", &self.originFile)
            .field("originModule", &self.originModule)
            .field("originCallerReturnAddressOffset", &self.originCallerReturnAddressOffset)
            .field("originCallerModule", &self.originCallerModule)
            .field("originName", &self.originName)
            .finish()
    }
}
impl ::core::default::Default for FILE_CASE_SENSITIVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_CASE_SENSITIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_CASE_SENSITIVE_INFO {}
impl ::core::fmt::Debug for FILE_CASE_SENSITIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_CASE_SENSITIVE_INFO").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for FILE_DISPOSITION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO_EX {}
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO_EX").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for FILE_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HW_PROFILE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo && self.szHwProfileGuid == other.szHwProfileGuid && self.szHwProfileName == other.szHwProfileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HW_PROFILE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HW_PROFILE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOA").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
impl ::core::default::Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HW_PROFILE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo && self.szHwProfileGuid == other.szHwProfileGuid && self.szHwProfileName == other.szHwProfileName
    }
}
impl ::core::cmp::Eq for HW_PROFILE_INFOW {}
impl ::core::fmt::Debug for HW_PROFILE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOW").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
impl ::core::cmp::PartialEq for ICameraUIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraUIControl {}
impl ::core::fmt::Debug for ICameraUIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraUIControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICameraUIControlEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraUIControlEventCallback {}
impl ::core::fmt::Debug for ICameraUIControlEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraUIControlEventCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IClipServiceNotificationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClipServiceNotificationHelper {}
impl ::core::fmt::Debug for IClipServiceNotificationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClipServiceNotificationHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContainerActivationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContainerActivationHelper {}
impl ::core::fmt::Debug for IContainerActivationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContainerActivationHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDefaultBrowserSyncSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultBrowserSyncSettings {}
impl ::core::fmt::Debug for IDefaultBrowserSyncSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultBrowserSyncSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDeleteBrowsingHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeleteBrowsingHistory {}
impl ::core::fmt::Debug for IDeleteBrowsingHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeleteBrowsingHistory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEditionUpgradeBroker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEditionUpgradeBroker {}
impl ::core::fmt::Debug for IEditionUpgradeBroker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEditionUpgradeBroker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEditionUpgradeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEditionUpgradeHelper {}
impl ::core::fmt::Debug for IEditionUpgradeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEditionUpgradeHelper").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEPROA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEPROA {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.InstDate == other.InstDate && self.wVersion == other.wVersion && self.szDescription == other.szDescription && self.szName == other.szName && self.szOptions == other.szOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEPROA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROA").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEPROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEPROW {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.InstDate == other.InstDate && self.wVersion == other.wVersion && self.szDescription == other.szDescription && self.szName == other.szName && self.szOptions == other.szOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEPROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROW").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.fnc == other.fnc && self.wParam == other.wParam && self.wCount == other.wCount && self.dchSource == other.dchSource && self.dchDest == other.dchDest && self.lParam1 == other.lParam1 && self.lParam2 == other.lParam2 && self.lParam3 == other.lParam3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRUCT").field("fnc", &self.fnc).field("wParam", &self.wParam).field("wCount", &self.wCount).field("dchSource", &self.dchSource).field("dchDest", &self.dchDest).field("lParam1", &self.lParam1).field("lParam2", &self.lParam2).field("lParam3", &self.lParam3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IWindowsLockModeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsLockModeHelper {}
impl ::core::fmt::Debug for IWindowsLockModeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsLockModeHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JAVA_TRUST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JAVA_TRUST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flag == other.flag && self.fAllActiveXPermissions == other.fAllActiveXPermissions && self.fAllPermissions == other.fAllPermissions && self.dwEncodingType == other.dwEncodingType && self.pbJavaPermissions == other.pbJavaPermissions && self.cbJavaPermissions == other.cbJavaPermissions && self.pbSigner == other.pbSigner && self.cbSigner == other.cbSigner && self.pwszZone == other.pwszZone && self.guidZone == other.guidZone && self.hVerify == other.hVerify
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JAVA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JAVA_TRUST")
            .field("cbSize", &self.cbSize)
            .field("flag", &self.flag)
            .field("fAllActiveXPermissions", &self.fAllActiveXPermissions)
            .field("fAllPermissions", &self.fAllPermissions)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("pbJavaPermissions", &self.pbJavaPermissions)
            .field("cbJavaPermissions", &self.cbJavaPermissions)
            .field("pbSigner", &self.pbSigner)
            .field("cbSigner", &self.cbSigner)
            .field("pwszZone", &self.pwszZone)
            .field("guidZone", &self.guidZone)
            .field("hVerify", &self.hVerify)
            .finish()
    }
}
impl ::core::default::Default for JIT_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JIT_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwProcessorArchitecture == other.dwProcessorArchitecture && self.dwThreadID == other.dwThreadID && self.dwReserved0 == other.dwReserved0 && self.lpExceptionAddress == other.lpExceptionAddress && self.lpExceptionRecord == other.lpExceptionRecord && self.lpContextRecord == other.lpContextRecord
    }
}
impl ::core::cmp::Eq for JIT_DEBUG_INFO {}
impl ::core::fmt::Debug for JIT_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JIT_DEBUG_INFO").field("dwSize", &self.dwSize).field("dwProcessorArchitecture", &self.dwProcessorArchitecture).field("dwThreadID", &self.dwThreadID).field("dwReserved0", &self.dwReserved0).field("lpExceptionAddress", &self.lpExceptionAddress).field("lpExceptionRecord", &self.lpExceptionRecord).field("lpContextRecord", &self.lpContextRecord).finish()
    }
}
impl ::core::default::Default for KEY_SET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KEY_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEY_SET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KEY_VALUE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ValueName == other.ValueName && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset && self.Type == other.Type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEY_VALUE_ENTRY").field("ValueName", &self.ValueName).field("DataLength", &self.DataLength).field("DataOffset", &self.DataOffset).field("Type", &self.Type).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl ::core::default::Default for OBJECT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERUSERSECTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERUSERSECTIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID && self.szDispName == other.szDispName && self.szLocale == other.szLocale && self.szStub == other.szStub && self.szVersion == other.szVersion && self.szCompID == other.szCompID && self.dwIsInstalled == other.dwIsInstalled && self.bRollback == other.bRollback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERUSERSECTIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONA").field("szGUID", &self.szGUID).field("szDispName", &self.szDispName).field("szLocale", &self.szLocale).field("szStub", &self.szStub).field("szVersion", &self.szVersion).field("szCompID", &self.szCompID).field("dwIsInstalled", &self.dwIsInstalled).field("bRollback", &self.bRollback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERUSERSECTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERUSERSECTIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID && self.szDispName == other.szDispName && self.szLocale == other.szLocale && self.szStub == other.szStub && self.szVersion == other.szVersion && self.szCompID == other.szCompID && self.dwIsInstalled == other.dwIsInstalled && self.bRollback == other.bRollback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERUSERSECTIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONW").field("szGUID", &self.szGUID).field("szDispName", &self.szDispName).field("szLocale", &self.szLocale).field("szStub", &self.szStub).field("szVersion", &self.szVersion).field("szCompID", &self.szCompID).field("dwIsInstalled", &self.dwIsInstalled).field("bRollback", &self.bRollback).finish()
    }
}
impl ::core::default::Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes && self.GrantedAccess == other.GrantedAccess && self.HandleCount == other.HandleCount && self.PointerCount == other.PointerCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::fmt::Debug for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_BASIC_INFORMATION").field("Attributes", &self.Attributes).field("GrantedAccess", &self.GrantedAccess).field("HandleCount", &self.HandleCount).field("PointerCount", &self.PointerCount).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TypeName == other.TypeName && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_TYPE_INFORMATION").field("TypeName", &self.TypeName).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STRENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
impl ::core::cmp::Eq for STRENTRYA {}
impl ::core::fmt::Debug for STRENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYA").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
impl ::core::default::Default for STRENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
impl ::core::cmp::Eq for STRENTRYW {}
impl ::core::fmt::Debug for STRENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYW").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
impl ::core::default::Default for STRINGEXSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRINGEXSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uDeterminePos == other.uDeterminePos && self.uDetermineDelimPos == other.uDetermineDelimPos && self.uYomiPos == other.uYomiPos && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::core::cmp::Eq for STRINGEXSTRUCT {}
impl ::core::fmt::Debug for STRINGEXSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRINGEXSTRUCT").field("dwSize", &self.dwSize).field("uDeterminePos", &self.uDeterminePos).field("uDetermineDelimPos", &self.uDetermineDelimPos).field("uYomiPos", &self.uYomiPos).field("uYomiDelimPos", &self.uYomiDelimPos).finish()
    }
}
impl ::core::default::Default for STRTABLEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRTABLEA {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
impl ::core::cmp::Eq for STRTABLEA {}
impl ::core::fmt::Debug for STRTABLEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEA").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
impl ::core::default::Default for STRTABLEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRTABLEW {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
impl ::core::cmp::Eq for STRTABLEW {}
impl ::core::fmt::Debug for STRTABLEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEW").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
impl ::core::default::Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.NumberOfProcessors == other.NumberOfProcessors
    }
}
impl ::core::cmp::Eq for SYSTEM_BASIC_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_BASIC_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("NumberOfProcessors", &self.NumberOfProcessors).finish()
    }
}
impl ::core::default::Default for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.CodeIntegrityOptions == other.CodeIntegrityOptions
    }
}
impl ::core::cmp::Eq for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CODEINTEGRITY_INFORMATION").field("Length", &self.Length).field("CodeIntegrityOptions", &self.CodeIntegrityOptions).finish()
    }
}
impl ::core::default::Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_EXCEPTION_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for SYSTEM_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_INTERRUPT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_INTERRUPT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INTERRUPT_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOOKASIDE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_LOOKASIDE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOOKASIDE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PERFORMANCE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SYSTEM_POLICY_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_POLICY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POLICY_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IdleTime == other.IdleTime && self.KernelTime == other.KernelTime && self.UserTime == other.UserTime && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION").field("IdleTime", &self.IdleTime).field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.NumberOfThreads == other.NumberOfThreads
            && self.Reserved1 == other.Reserved1
            && self.ImageName == other.ImageName
            && self.BasePriority == other.BasePriority
            && self.UniqueProcessId == other.UniqueProcessId
            && self.Reserved2 == other.Reserved2
            && self.HandleCount == other.HandleCount
            && self.SessionId == other.SessionId
            && self.Reserved3 == other.Reserved3
            && self.PeakVirtualSize == other.PeakVirtualSize
            && self.VirtualSize == other.VirtualSize
            && self.Reserved4 == other.Reserved4
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.Reserved5 == other.Reserved5
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.Reserved6 == other.Reserved6
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivatePageCount == other.PrivatePageCount
            && self.Reserved7 == other.Reserved7
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESS_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("Reserved1", &self.Reserved1)
            .field("ImageName", &self.ImageName)
            .field("BasePriority", &self.BasePriority)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved2", &self.Reserved2)
            .field("HandleCount", &self.HandleCount)
            .field("SessionId", &self.SessionId)
            .field("Reserved3", &self.Reserved3)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("Reserved4", &self.Reserved4)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("Reserved5", &self.Reserved5)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("Reserved6", &self.Reserved6)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivatePageCount", &self.PrivatePageCount)
            .field("Reserved7", &self.Reserved7)
            .finish()
    }
}
impl ::core::default::Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RegistryQuotaAllowed == other.RegistryQuotaAllowed && self.RegistryQuotaUsed == other.RegistryQuotaUsed && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_REGISTRY_QUOTA_INFORMATION").field("RegistryQuotaAllowed", &self.RegistryQuotaAllowed).field("RegistryQuotaUsed", &self.RegistryQuotaUsed).field("Reserved1", &self.Reserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.StartAddress == other.StartAddress && self.ClientId == other.ClientId && self.Priority == other.Priority && self.BasePriority == other.BasePriority && self.Reserved3 == other.Reserved3 && self.ThreadState == other.ThreadState && self.WaitReason == other.WaitReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_THREAD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_THREAD_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("StartAddress", &self.StartAddress).field("ClientId", &self.ClientId).field("Priority", &self.Priority).field("BasePriority", &self.BasePriority).field("Reserved3", &self.Reserved3).field("ThreadState", &self.ThreadState).field("WaitReason", &self.WaitReason).finish()
    }
}
impl ::core::default::Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_TIMEOFDAY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_TIMEOFDAY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_TIMEOFDAY_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_QUERY_INFORMATION_EX32_XP").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::core::default::Default for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {}
impl ::core::fmt::Debug for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_QUERY_INFORMATION_EX_W2K").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::core::default::Default for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for TCP_REQUEST_QUERY_INFORMATION_EX_XP {}
impl ::core::fmt::Debug for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_QUERY_INFORMATION_EX_XP").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::core::default::Default for TCP_REQUEST_SET_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_REQUEST_SET_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for TCP_REQUEST_SET_INFORMATION_EX {}
impl ::core::fmt::Debug for TCP_REQUEST_SET_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_SET_INFORMATION_EX").field("ID", &self.ID).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for TDIENTITY_ENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TDIENTITY_ENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDIENTITY_ENTITY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TDIEntityID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TDIEntityID {
    fn eq(&self, other: &Self) -> bool {
        self.tei_entity == other.tei_entity && self.tei_instance == other.tei_instance
    }
}
impl ::core::cmp::Eq for TDIEntityID {}
impl ::core::fmt::Debug for TDIEntityID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIEntityID").field("tei_entity", &self.tei_entity).field("tei_instance", &self.tei_instance).finish()
    }
}
impl ::core::default::Default for TDIObjectID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TDIObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.toi_entity == other.toi_entity && self.toi_class == other.toi_class && self.toi_type == other.toi_type && self.toi_id == other.toi_id
    }
}
impl ::core::cmp::Eq for TDIObjectID {}
impl ::core::fmt::Debug for TDIObjectID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIObjectID").field("toi_entity", &self.toi_entity).field("toi_class", &self.toi_class).field("toi_type", &self.toi_type).field("toi_id", &self.toi_id).finish()
    }
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TDI_TL_IO_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDI_TL_IO_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for THREAD_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for THREAD_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadName == other.ThreadName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for THREAD_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_NAME_INFORMATION").field("ThreadName", &self.ThreadName).finish()
    }
}
impl ::core::default::Default for UNDETERMINESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNDETERMINESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uDefIMESize == other.uDefIMESize && self.uDefIMEPos == other.uDefIMEPos && self.uUndetTextLen == other.uUndetTextLen && self.uUndetTextPos == other.uUndetTextPos && self.uUndetAttrPos == other.uUndetAttrPos && self.uCursorPos == other.uCursorPos && self.uDeltaStart == other.uDeltaStart && self.uDetermineTextLen == other.uDetermineTextLen && self.uDetermineTextPos == other.uDetermineTextPos && self.uDetermineDelimPos == other.uDetermineDelimPos && self.uYomiTextLen == other.uYomiTextLen && self.uYomiTextPos == other.uYomiTextPos && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::core::cmp::Eq for UNDETERMINESTRUCT {}
impl ::core::fmt::Debug for UNDETERMINESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNDETERMINESTRUCT")
            .field("dwSize", &self.dwSize)
            .field("uDefIMESize", &self.uDefIMESize)
            .field("uDefIMEPos", &self.uDefIMEPos)
            .field("uUndetTextLen", &self.uUndetTextLen)
            .field("uUndetTextPos", &self.uUndetTextPos)
            .field("uUndetAttrPos", &self.uUndetAttrPos)
            .field("uCursorPos", &self.uCursorPos)
            .field("uDeltaStart", &self.uDeltaStart)
            .field("uDetermineTextLen", &self.uDetermineTextLen)
            .field("uDetermineTextPos", &self.uDetermineTextPos)
            .field("uDetermineDelimPos", &self.uDetermineDelimPos)
            .field("uYomiTextLen", &self.uYomiTextLen)
            .field("uYomiTextPos", &self.uYomiTextPos)
            .field("uYomiDelimPos", &self.uYomiDelimPos)
            .finish()
    }
}
impl ::core::default::Default for VALUENAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VALUENAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALUENAME").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINSTATIONINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINSTATIONINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSTATIONINFOCLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINSTATIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved2 == other.Reserved2 && self.LogonId == other.LogonId && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for WINSTATIONINFORMATIONW {}
impl ::core::fmt::Debug for WINSTATIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINSTATIONINFORMATIONW").field("Reserved2", &self.Reserved2).field("LogonId", &self.LogonId).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::core::default::Default for WLDP_DEVICE_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLDP_DEVICE_SECURITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.UnlockIdSize == other.UnlockIdSize && self.UnlockId == other.UnlockId && self.ManufacturerIDLength == other.ManufacturerIDLength && self.ManufacturerID == other.ManufacturerID
    }
}
impl ::core::cmp::Eq for WLDP_DEVICE_SECURITY_INFORMATION {}
impl ::core::fmt::Debug for WLDP_DEVICE_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_DEVICE_SECURITY_INFORMATION").field("UnlockIdSize", &self.UnlockIdSize).field("UnlockId", &self.UnlockId).field("ManufacturerIDLength", &self.ManufacturerIDLength).field("ManufacturerID", &self.ManufacturerID).finish()
    }
}
impl ::core::default::Default for WLDP_HOST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_HOST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLDP_HOST_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_HOST_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLDP_HOST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLDP_HOST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwRevision == other.dwRevision && self.dwHostId == other.dwHostId && self.szSource == other.szSource && self.hSource == other.hSource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLDP_HOST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_HOST_INFORMATION").field("dwRevision", &self.dwRevision).field("dwHostId", &self.dwHostId).field("szSource", &self.szSource).field("hSource", &self.hSource).finish()
    }
}
impl ::core::default::Default for WLDP_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_KEY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLDP_POLICY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_POLICY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_POLICY_SETTING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_RESTRICTION").field(&self.0).finish()
    }
}
