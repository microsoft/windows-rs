pub const AppDomainHelper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: super::super::Foundation::BOOL,
    pub HasAutomaticLifetimeRecycling: super::super::Foundation::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationProcessRecycleInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationProcessRecycleInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ApplicationProcessRecycleInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationProcessRecycleInfo")
            .field("IsRecyclable", &self.IsRecyclable)
            .field("IsRecycled", &self.IsRecycled)
            .field("TimeRecycled", &self.TimeRecycled)
            .field("TimeToTerminate", &self.TimeToTerminate)
            .field("RecycleReasonCode", &self.RecycleReasonCode)
            .field("IsPendingRecycle", &self.IsPendingRecycle)
            .field("HasAutomaticLifetimeRecycling", &self.HasAutomaticLifetimeRecycling)
            .field("TimeForAutomaticRecycling", &self.TimeForAutomaticRecycling)
            .field("MemoryLimitInKB", &self.MemoryLimitInKB)
            .field("MemoryUsageInKBLastCheck", &self.MemoryUsageInKBLastCheck)
            .field("ActivationLimit", &self.ActivationLimit)
            .field("NumActivationsLastReported", &self.NumActivationsLastReported)
            .field("CallLimit", &self.CallLimit)
            .field("NumCallsLastReported", &self.NumCallsLastReported)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ApplicationProcessRecycleInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ApplicationProcessRecycleInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationProcessRecycleInfo>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ApplicationProcessRecycleInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ApplicationProcessRecycleInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: u32,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
    pub AvgCallsPerSecond: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ::core::marker::Copy for ApplicationProcessStatistics {}
impl ::core::clone::Clone for ApplicationProcessStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ApplicationProcessStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationProcessStatistics").field("NumCallsOutstanding", &self.NumCallsOutstanding).field("NumTrackedComponents", &self.NumTrackedComponents).field("NumComponentInstances", &self.NumComponentInstances).field("AvgCallsPerSecond", &self.AvgCallsPerSecond).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
unsafe impl ::windows::core::Abi for ApplicationProcessStatistics {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationProcessStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationProcessStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for ApplicationProcessStatistics {}
impl ::core::default::Default for ApplicationProcessStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: ::windows::core::GUID,
    pub ApplicationIdPrimaryApplication: ::windows::core::GUID,
    pub ApplicationInstanceId: ::windows::core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: ::windows::core::PWSTR,
    pub IsService: super::super::Foundation::BOOL,
    pub IsPaused: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationProcessSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationProcessSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ApplicationProcessSummary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationProcessSummary")
            .field("PartitionIdPrimaryApplication", &self.PartitionIdPrimaryApplication)
            .field("ApplicationIdPrimaryApplication", &self.ApplicationIdPrimaryApplication)
            .field("ApplicationInstanceId", &self.ApplicationInstanceId)
            .field("ProcessId", &self.ProcessId)
            .field("Type", &self.Type)
            .field("ProcessExeName", &self.ProcessExeName)
            .field("IsService", &self.IsService)
            .field("IsPaused", &self.IsPaused)
            .field("IsRecycled", &self.IsRecycled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ApplicationProcessSummary {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ApplicationProcessSummary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationProcessSummary>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ApplicationProcessSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ApplicationProcessSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: ::windows::core::GUID,
    pub PartitionId: ::windows::core::GUID,
    pub ApplicationId: ::windows::core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: ::windows::core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
impl ::core::marker::Copy for ApplicationSummary {}
impl ::core::clone::Clone for ApplicationSummary {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ApplicationSummary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationSummary").field("ApplicationInstanceId", &self.ApplicationInstanceId).field("PartitionId", &self.PartitionId).field("ApplicationId", &self.ApplicationId).field("Type", &self.Type).field("ApplicationName", &self.ApplicationName).field("NumTrackedComponents", &self.NumTrackedComponents).field("NumComponentInstances", &self.NumComponentInstances).finish()
    }
}
unsafe impl ::windows::core::Abi for ApplicationSummary {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationSummary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationSummary>()) == 0 }
    }
}
impl ::core::cmp::Eq for ApplicationSummary {}
impl ::core::default::Default for ApplicationSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoSvcs_Error_Constants(pub u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803138u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803139u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803140u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803141u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803142u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803143u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803148u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803149u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803150u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803151u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599296u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599297u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599298u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599299u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599300u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrOutParam: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599301u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599302u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599303u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599304u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599376u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599377u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599378u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599379u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599380u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599381u32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599382u32);
impl ::core::marker::Copy for AutoSvcs_Error_Constants {}
impl ::core::clone::Clone for AutoSvcs_Error_Constants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoSvcs_Error_Constants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoSvcs_Error_Constants {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoSvcs_Error_Constants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoSvcs_Error_Constants").field(&self.0).finish()
    }
}
pub const ByotServerEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CAppData {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: CAppStatistics,
}
impl ::core::marker::Copy for CAppData {}
impl ::core::clone::Clone for CAppData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAppData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAppData").field("m_idApp", &self.m_idApp).field("m_szAppGuid", &self.m_szAppGuid).field("m_dwAppProcessId", &self.m_dwAppProcessId).field("m_AppStatistics", &self.m_AppStatistics).finish()
    }
}
unsafe impl ::windows::core::Abi for CAppData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAppData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAppData>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAppData {}
impl ::core::default::Default for CAppData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CAppStatistics {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
impl ::core::marker::Copy for CAppStatistics {}
impl ::core::clone::Clone for CAppStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAppStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAppStatistics").field("m_cTotalCalls", &self.m_cTotalCalls).field("m_cTotalInstances", &self.m_cTotalInstances).field("m_cTotalClasses", &self.m_cTotalClasses).field("m_cCallsPerSecond", &self.m_cCallsPerSecond).finish()
    }
}
unsafe impl ::windows::core::Abi for CAppStatistics {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAppStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAppStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAppStatistics {}
impl ::core::default::Default for CAppStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CCLSIDData {
    pub m_clsid: ::windows::core::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl ::core::marker::Copy for CCLSIDData {}
impl ::core::clone::Clone for CCLSIDData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CCLSIDData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCLSIDData").field("m_clsid", &self.m_clsid).field("m_cReferences", &self.m_cReferences).field("m_cBound", &self.m_cBound).field("m_cPooled", &self.m_cPooled).field("m_cInCall", &self.m_cInCall).field("m_dwRespTime", &self.m_dwRespTime).field("m_cCallsCompleted", &self.m_cCallsCompleted).field("m_cCallsFailed", &self.m_cCallsFailed).finish()
    }
}
unsafe impl ::windows::core::Abi for CCLSIDData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCLSIDData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCLSIDData>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCLSIDData {}
impl ::core::default::Default for CCLSIDData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CCLSIDData2 {
    pub m_clsid: ::windows::core::GUID,
    pub m_appid: ::windows::core::GUID,
    pub m_partid: ::windows::core::GUID,
    pub m_pwszAppName: ::windows::core::PWSTR,
    pub m_pwszCtxName: ::windows::core::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl ::core::marker::Copy for CCLSIDData2 {}
impl ::core::clone::Clone for CCLSIDData2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CCLSIDData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCLSIDData2")
            .field("m_clsid", &self.m_clsid)
            .field("m_appid", &self.m_appid)
            .field("m_partid", &self.m_partid)
            .field("m_pwszAppName", &self.m_pwszAppName)
            .field("m_pwszCtxName", &self.m_pwszCtxName)
            .field("m_eAppType", &self.m_eAppType)
            .field("m_cReferences", &self.m_cReferences)
            .field("m_cBound", &self.m_cBound)
            .field("m_cPooled", &self.m_cPooled)
            .field("m_cInCall", &self.m_cInCall)
            .field("m_dwRespTime", &self.m_dwRespTime)
            .field("m_cCallsCompleted", &self.m_cCallsCompleted)
            .field("m_cCallsFailed", &self.m_cCallsFailed)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CCLSIDData2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCLSIDData2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCLSIDData2>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCLSIDData2 {}
impl ::core::default::Default for CCLSIDData2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminAccessChecksLevelOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(1i32);
impl ::core::marker::Copy for COMAdminAccessChecksLevelOptions {}
impl ::core::clone::Clone for COMAdminAccessChecksLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminAccessChecksLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminAccessChecksLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminAccessChecksLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminAccessChecksLevelOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminActivationOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminActivationInproc: COMAdminActivationOptions = COMAdminActivationOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminActivationLocal: COMAdminActivationOptions = COMAdminActivationOptions(1i32);
impl ::core::marker::Copy for COMAdminActivationOptions {}
impl ::core::clone::Clone for COMAdminActivationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminActivationOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminActivationOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminActivationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminActivationOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminApplicationExportOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(16i32);
impl ::core::marker::Copy for COMAdminApplicationExportOptions {}
impl ::core::clone::Clone for COMAdminApplicationExportOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminApplicationExportOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminApplicationExportOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminApplicationExportOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminApplicationExportOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminApplicationInstallOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(2i32);
impl ::core::marker::Copy for COMAdminApplicationInstallOptions {}
impl ::core::clone::Clone for COMAdminApplicationInstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminApplicationInstallOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminApplicationInstallOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminApplicationInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminApplicationInstallOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminAuthenticationCapabilitiesOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(32i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(64i32);
impl ::core::marker::Copy for COMAdminAuthenticationCapabilitiesOptions {}
impl ::core::clone::Clone for COMAdminAuthenticationCapabilitiesOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminAuthenticationCapabilitiesOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminAuthenticationCapabilitiesOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminAuthenticationCapabilitiesOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminAuthenticationCapabilitiesOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminAuthenticationLevelOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(5i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(6i32);
impl ::core::marker::Copy for COMAdminAuthenticationLevelOptions {}
impl ::core::clone::Clone for COMAdminAuthenticationLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminAuthenticationLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminAuthenticationLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminAuthenticationLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminAuthenticationLevelOptions").field(&self.0).finish()
    }
}
pub const COMAdminCatalog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminComponentFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = COMAdminComponentFlags(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = COMAdminComponentFlags(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = COMAdminComponentFlags(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = COMAdminComponentFlags(8i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = COMAdminComponentFlags(16i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = COMAdminComponentFlags(32i32);
impl ::core::marker::Copy for COMAdminComponentFlags {}
impl ::core::clone::Clone for COMAdminComponentFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminComponentFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminComponentFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminComponentFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminComponentFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminComponentType(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdmin32BitComponent: COMAdminComponentType = COMAdminComponentType(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdmin64BitComponent: COMAdminComponentType = COMAdminComponentType(2i32);
impl ::core::marker::Copy for COMAdminComponentType {}
impl ::core::clone::Clone for COMAdminComponentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminComponentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminComponentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminComponentType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminErrorCodes(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = COMAdminErrorCodes(-2146368511i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368510i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146368509i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368508i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368505i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368504i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = COMAdminErrorCodes(-2146368503i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadPath: COMAdminErrorCodes = COMAdminErrorCodes(-2146368502i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368501i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRoleExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368500i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = COMAdminErrorCodes(-2146368499i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoUser: COMAdminErrorCodes = COMAdminErrorCodes(-2146368497i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = COMAdminErrorCodes(-2146368496i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368495i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368494i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = COMAdminErrorCodes(-2146368493i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368492i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368488i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = COMAdminErrorCodes(-2146368487i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = COMAdminErrorCodes(-2146368486i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = COMAdminErrorCodes(-2146368485i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368483i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368482i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368481i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368477i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368476i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368475i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = COMAdminErrorCodes(-2146368474i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368473i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368472i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368471i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368470i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368469i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrSession: COMAdminErrorCodes = COMAdminErrorCodes(-2146368468i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = COMAdminErrorCodes(-2146368467i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146368466i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368464i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368461i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = COMAdminErrorCodes(-2146368460i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368459i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368458i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368457i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368456i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrComponentExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368455i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = COMAdminErrorCodes(-2146368453i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = COMAdminErrorCodes(-2146368452i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = COMAdminErrorCodes(-2146368450i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368449i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368442i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368441i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = COMAdminErrorCodes(-2146368440i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = COMAdminErrorCodes(-2146368439i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = COMAdminErrorCodes(-2146367998i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146367480i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146367479i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = COMAdminErrorCodes(-2146368438i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368437i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368436i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = COMAdminErrorCodes(-2146368435i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146367478i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = COMAdminErrorCodes(-2146368434i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = COMAdminErrorCodes(-2146368433i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146368432i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = COMAdminErrorCodes(-2146368425i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = COMAdminErrorCodes(-2146368423i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368421i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = COMAdminErrorCodes(-2146368398i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = COMAdminErrorCodes(-2146368397i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = COMAdminErrorCodes(-2146368396i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146368395i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368384i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368383i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368382i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368381i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368380i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368379i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = COMAdminErrorCodes(-2146368378i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367473i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367471i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367470i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367469i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = COMAdminErrorCodes(-2146367477i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146367463i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368431i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = COMAdminErrorCodes(-2146367460i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146367459i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = COMAdminErrorCodes(-2146367458i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = COMAdminErrorCodes(-2146367456i32);
impl ::core::marker::Copy for COMAdminErrorCodes {}
impl ::core::clone::Clone for COMAdminErrorCodes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminErrorCodes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminErrorCodes {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminErrorCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminErrorCodes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminFileFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = COMAdminFileFlags(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagCOM: COMAdminFileFlags = COMAdminFileFlags(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = COMAdminFileFlags(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = COMAdminFileFlags(8i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = COMAdminFileFlags(16i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = COMAdminFileFlags(32i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = COMAdminFileFlags(64i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = COMAdminFileFlags(128i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = COMAdminFileFlags(256i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = COMAdminFileFlags(512i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = COMAdminFileFlags(1024i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = COMAdminFileFlags(2048i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = COMAdminFileFlags(4096i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = COMAdminFileFlags(8192i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = COMAdminFileFlags(16384i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = COMAdminFileFlags(32768i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = COMAdminFileFlags(65536i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = COMAdminFileFlags(131072i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagError: COMAdminFileFlags = COMAdminFileFlags(262144i32);
impl ::core::marker::Copy for COMAdminFileFlags {}
impl ::core::clone::Clone for COMAdminFileFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminFileFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminFileFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminFileFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminFileFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminImpersonationLevelOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(4i32);
impl ::core::marker::Copy for COMAdminImpersonationLevelOptions {}
impl ::core::clone::Clone for COMAdminImpersonationLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminImpersonationLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminImpersonationLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminImpersonationLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminImpersonationLevelOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminInUse(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminNotInUse: COMAdminInUse = COMAdminInUse(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByCatalog: COMAdminInUse = COMAdminInUse(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = COMAdminInUse(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = COMAdminInUse(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = COMAdminInUse(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = COMAdminInUse(5i32);
impl ::core::marker::Copy for COMAdminInUse {}
impl ::core::clone::Clone for COMAdminInUse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminInUse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminInUse {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminInUse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminInUse").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminOS(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSNotInitialized: COMAdminOS = COMAdminOS(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows3_1: COMAdminOS = COMAdminOS(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows9x: COMAdminOS = COMAdminOS(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000: COMAdminOS = COMAdminOS(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = COMAdminOS(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000Unknown: COMAdminOS = COMAdminOS(5i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSUnknown: COMAdminOS = COMAdminOS(6i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = COMAdminOS(11i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = COMAdminOS(12i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = COMAdminOS(13i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = COMAdminOS(14i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = COMAdminOS(15i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = COMAdminOS(16i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = COMAdminOS(17i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = COMAdminOS(18i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = COMAdminOS(19i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = COMAdminOS(20i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = COMAdminOS(21i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = COMAdminOS(22i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7Personal: COMAdminOS = COMAdminOS(23i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7Professional: COMAdminOS = COMAdminOS(24i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7StandardServer: COMAdminOS = COMAdminOS(25i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = COMAdminOS(26i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = COMAdminOS(27i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7WebServer: COMAdminOS = COMAdminOS(28i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8Personal: COMAdminOS = COMAdminOS(29i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8Professional: COMAdminOS = COMAdminOS(30i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8StandardServer: COMAdminOS = COMAdminOS(31i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = COMAdminOS(32i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = COMAdminOS(33i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8WebServer: COMAdminOS = COMAdminOS(34i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = COMAdminOS(35i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = COMAdminOS(36i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = COMAdminOS(37i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = COMAdminOS(38i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = COMAdminOS(39i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = COMAdminOS(40i32);
impl ::core::marker::Copy for COMAdminOS {}
impl ::core::clone::Clone for COMAdminOS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminOS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminOS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminOS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminQCMessageAuthenticateOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(2i32);
impl ::core::marker::Copy for COMAdminQCMessageAuthenticateOptions {}
impl ::core::clone::Clone for COMAdminQCMessageAuthenticateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminQCMessageAuthenticateOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminQCMessageAuthenticateOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminQCMessageAuthenticateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminQCMessageAuthenticateOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminServiceOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = COMAdminServiceOptions(1i32);
impl ::core::marker::Copy for COMAdminServiceOptions {}
impl ::core::clone::Clone for COMAdminServiceOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminServiceOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminServiceOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminServiceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminServiceOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminServiceStatusOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(5i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(6i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(7i32);
impl ::core::marker::Copy for COMAdminServiceStatusOptions {}
impl ::core::clone::Clone for COMAdminServiceStatusOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminServiceStatusOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminServiceStatusOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminServiceStatusOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminServiceStatusOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminSynchronizationOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(4i32);
impl ::core::marker::Copy for COMAdminSynchronizationOptions {}
impl ::core::clone::Clone for COMAdminSynchronizationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminSynchronizationOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminSynchronizationOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminSynchronizationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminSynchronizationOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminThreadingModels(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = COMAdminThreadingModels(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = COMAdminThreadingModels(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = COMAdminThreadingModels(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = COMAdminThreadingModels(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = COMAdminThreadingModels(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = COMAdminThreadingModels(5i32);
impl ::core::marker::Copy for COMAdminThreadingModels {}
impl ::core::clone::Clone for COMAdminThreadingModels {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminThreadingModels {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminThreadingModels {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminThreadingModels {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminThreadingModels").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminTransactionOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = COMAdminTransactionOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionNone: COMAdminTransactionOptions = COMAdminTransactionOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = COMAdminTransactionOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = COMAdminTransactionOptions(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = COMAdminTransactionOptions(4i32);
impl ::core::marker::Copy for COMAdminTransactionOptions {}
impl ::core::clone::Clone for COMAdminTransactionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminTransactionOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminTransactionOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminTransactionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminTransactionOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMAdminTxIsolationLevelOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(3i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(4i32);
impl ::core::marker::Copy for COMAdminTxIsolationLevelOptions {}
impl ::core::clone::Clone for COMAdminTxIsolationLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminTxIsolationLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMAdminTxIsolationLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminTxIsolationLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminTxIsolationLevelOptions").field(&self.0).finish()
    }
}
pub const COMEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPLUS_APPTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = COMPLUS_APPTYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = COMPLUS_APPTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = COMPLUS_APPTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_SWC: COMPLUS_APPTYPE = COMPLUS_APPTYPE(2i32);
impl ::core::marker::Copy for COMPLUS_APPTYPE {}
impl ::core::clone::Clone for COMPLUS_APPTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPLUS_APPTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMPLUS_APPTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMPLUS_APPTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPLUS_APPTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: ::windows::core::GUID,
    pub sMachineName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for COMSVCSEVENTINFO {}
impl ::core::clone::Clone for COMSVCSEVENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMSVCSEVENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMSVCSEVENTINFO").field("cbSize", &self.cbSize).field("dwPid", &self.dwPid).field("lTime", &self.lTime).field("lMicroTime", &self.lMicroTime).field("perfCount", &self.perfCount).field("guidApp", &self.guidApp).field("sMachineName", &self.sMachineName).finish()
    }
}
unsafe impl ::windows::core::Abi for COMSVCSEVENTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMSVCSEVENTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMSVCSEVENTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMSVCSEVENTINFO {}
impl ::core::default::Default for COMSVCSEVENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CRMClerk: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRMFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = CRMFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = CRMFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = CRMFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = CRMFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = CRMFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = CRMFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = CRMFLAGS(64i32);
impl ::core::marker::Copy for CRMFLAGS {}
impl ::core::clone::Clone for CRMFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRMFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRMFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRMREGFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = CRMREGFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = CRMREGFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = CRMREGFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = CRMREGFLAGS(7i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = CRMREGFLAGS(16i32);
impl ::core::marker::Copy for CRMREGFLAGS {}
impl ::core::clone::Clone for CRMREGFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRMREGFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRMREGFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRMREGFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRMREGFLAGS").field(&self.0).finish()
    }
}
pub const CRMRecoveryClerk: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_Binding(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoBinding: CSC_Binding = CSC_Binding(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_BindToPoolThread: CSC_Binding = CSC_Binding(1i32);
impl ::core::marker::Copy for CSC_Binding {}
impl ::core::clone::Clone for CSC_Binding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_Binding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_Binding {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_Binding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_Binding").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_COMTIIntrinsicsConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(1i32);
impl ::core::marker::Copy for CSC_COMTIIntrinsicsConfig {}
impl ::core::clone::Clone for CSC_COMTIIntrinsicsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_COMTIIntrinsicsConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_COMTIIntrinsicsConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_COMTIIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_COMTIIntrinsicsConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_IISIntrinsicsConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(1i32);
impl ::core::marker::Copy for CSC_IISIntrinsicsConfig {}
impl ::core::clone::Clone for CSC_IISIntrinsicsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_IISIntrinsicsConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_IISIntrinsicsConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_IISIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_IISIntrinsicsConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_InheritanceConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_Inherit: CSC_InheritanceConfig = CSC_InheritanceConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_Ignore: CSC_InheritanceConfig = CSC_InheritanceConfig(1i32);
impl ::core::marker::Copy for CSC_InheritanceConfig {}
impl ::core::clone::Clone for CSC_InheritanceConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_InheritanceConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_InheritanceConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_InheritanceConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_InheritanceConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_PartitionConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoPartition: CSC_PartitionConfig = CSC_PartitionConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritPartition: CSC_PartitionConfig = CSC_PartitionConfig(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewPartition: CSC_PartitionConfig = CSC_PartitionConfig(2i32);
impl ::core::marker::Copy for CSC_PartitionConfig {}
impl ::core::clone::Clone for CSC_PartitionConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_PartitionConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_PartitionConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_PartitionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_PartitionConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_SxsConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoSxs: CSC_SxsConfig = CSC_SxsConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritSxs: CSC_SxsConfig = CSC_SxsConfig(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSxs: CSC_SxsConfig = CSC_SxsConfig(2i32);
impl ::core::marker::Copy for CSC_SxsConfig {}
impl ::core::clone::Clone for CSC_SxsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_SxsConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_SxsConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_SxsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_SxsConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_SynchronizationConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = CSC_SynchronizationConfig(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = CSC_SynchronizationConfig(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(3i32);
impl ::core::marker::Copy for CSC_SynchronizationConfig {}
impl ::core::clone::Clone for CSC_SynchronizationConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_SynchronizationConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_SynchronizationConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_SynchronizationConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_SynchronizationConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_ThreadPool(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_ThreadPoolNone: CSC_ThreadPool = CSC_ThreadPool(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = CSC_ThreadPool(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_STAThreadPool: CSC_ThreadPool = CSC_ThreadPool(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_MTAThreadPool: CSC_ThreadPool = CSC_ThreadPool(3i32);
impl ::core::marker::Copy for CSC_ThreadPool {}
impl ::core::clone::Clone for CSC_ThreadPool {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_ThreadPool {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_ThreadPool {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_ThreadPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_ThreadPool").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_TrackerConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_DontUseTracker: CSC_TrackerConfig = CSC_TrackerConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_UseTracker: CSC_TrackerConfig = CSC_TrackerConfig(1i32);
impl ::core::marker::Copy for CSC_TrackerConfig {}
impl ::core::clone::Clone for CSC_TrackerConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_TrackerConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_TrackerConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_TrackerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_TrackerConfig").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CSC_TransactionConfig(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoTransaction: CSC_TransactionConfig = CSC_TransactionConfig(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = CSC_TransactionConfig(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = CSC_TransactionConfig(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewTransaction: CSC_TransactionConfig = CSC_TransactionConfig(3i32);
impl ::core::marker::Copy for CSC_TransactionConfig {}
impl ::core::clone::Clone for CSC_TransactionConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_TransactionConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSC_TransactionConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_TransactionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_TransactionConfig").field(&self.0).finish()
    }
}
pub const CServiceConfig: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn CoCreateActivity<'a, P0>(piunknown: P0, riid: *const ::windows::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoCreateActivity(piunknown: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    CoCreateActivity(piunknown.into().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppobj)).ok()
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn CoEnterServiceDomain<'a, P0>(pconfigobject: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoEnterServiceDomain(pconfigobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    CoEnterServiceDomain(pconfigobject.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    CoGetDefaultContext(apttype, ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn CoLeaveServiceDomain<'a, P0>(punkstatus: P0)
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoLeaveServiceDomain(punkstatus: *mut ::core::ffi::c_void);
    }
    CoLeaveServiceDomain(punkstatus.into().abi())
}
pub const CoMTSLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: super::super::Foundation::BOOL,
    pub TerminateOnHang: super::super::Foundation::BOOL,
    pub AvgCallThresholdInMs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ComponentHangMonitorInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ComponentHangMonitorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ComponentHangMonitorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComponentHangMonitorInfo").field("IsMonitored", &self.IsMonitored).field("TerminateOnHang", &self.TerminateOnHang).field("AvgCallThresholdInMs", &self.AvgCallThresholdInMs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ComponentHangMonitorInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ComponentHangMonitorInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComponentHangMonitorInfo>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ComponentHangMonitorInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ComponentHangMonitorInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ComponentStatistics {
    pub NumInstances: u32,
    pub NumBoundReferences: u32,
    pub NumPooledObjects: u32,
    pub NumObjectsInCall: u32,
    pub AvgResponseTimeInMs: u32,
    pub NumCallsCompletedRecent: u32,
    pub NumCallsFailedRecent: u32,
    pub NumCallsCompletedTotal: u32,
    pub NumCallsFailedTotal: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ::core::marker::Copy for ComponentStatistics {}
impl ::core::clone::Clone for ComponentStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComponentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComponentStatistics")
            .field("NumInstances", &self.NumInstances)
            .field("NumBoundReferences", &self.NumBoundReferences)
            .field("NumPooledObjects", &self.NumPooledObjects)
            .field("NumObjectsInCall", &self.NumObjectsInCall)
            .field("AvgResponseTimeInMs", &self.AvgResponseTimeInMs)
            .field("NumCallsCompletedRecent", &self.NumCallsCompletedRecent)
            .field("NumCallsFailedRecent", &self.NumCallsFailedRecent)
            .field("NumCallsCompletedTotal", &self.NumCallsCompletedTotal)
            .field("NumCallsFailedTotal", &self.NumCallsFailedTotal)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ComponentStatistics {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComponentStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComponentStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComponentStatistics {}
impl ::core::default::Default for ComponentStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ComponentSummary {
    pub ApplicationInstanceId: ::windows::core::GUID,
    pub PartitionId: ::windows::core::GUID,
    pub ApplicationId: ::windows::core::GUID,
    pub Clsid: ::windows::core::GUID,
    pub ClassName: ::windows::core::PWSTR,
    pub ApplicationName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ComponentSummary {}
impl ::core::clone::Clone for ComponentSummary {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComponentSummary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComponentSummary").field("ApplicationInstanceId", &self.ApplicationInstanceId).field("PartitionId", &self.PartitionId).field("ApplicationId", &self.ApplicationId).field("Clsid", &self.Clsid).field("ClassName", &self.ClassName).field("ApplicationName", &self.ApplicationName).finish()
    }
}
unsafe impl ::windows::core::Abi for ComponentSummary {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComponentSummary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComponentSummary>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComponentSummary {}
impl ::core::default::Default for ComponentSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ContextInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo {
    pub unsafe fn IsInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransactionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetActivityId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetActivityId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContextId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetContextId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo> for ::windows::core::IUnknown {
    fn from(value: ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ContextInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo> for ::windows::core::IUnknown {
    fn from(value: &ContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo> for super::Com::IDispatch {
    fn from(value: ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ContextInfo> for &'a super::Com::IDispatch {
    fn from(value: &'a ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo> for super::Com::IDispatch {
    fn from(value: &ContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ContextInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ContextInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ContextInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ContextInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ContextInfo {
    type Vtable = ContextInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19a5a02c_0ac8_11d2_b286_00c04f8ef934);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTransactionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstractivityid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetActivityId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContextId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrctxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContextId: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ContextInfo2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo2 {
    pub unsafe fn IsInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransactionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetActivityId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetActivityId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContextId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetContextId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartitionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPartitionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetApplicationId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetApplicationId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetApplicationInstanceId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetApplicationInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for ::windows::core::IUnknown {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ContextInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for ::windows::core::IUnknown {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for super::Com::IDispatch {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ContextInfo2> for &'a super::Com::IDispatch {
    fn from(value: &'a ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for super::Com::IDispatch {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for ContextInfo {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ContextInfo2> for &'a ContextInfo {
    fn from(value: &'a ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for ContextInfo {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ContextInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ContextInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ContextInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ContextInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextInfo2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ContextInfo2 {
    type Vtable = ContextInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99d6e75_2375_11d4_8331_00c04f605588);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo2_Vtbl {
    pub base__: ContextInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationInstanceId: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for CrmLogRecordRead {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for CrmLogRecordRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CrmLogRecordRead").field("dwCrmFlags", &self.dwCrmFlags).field("dwSequenceNumber", &self.dwSequenceNumber).field("blobUserData", &self.blobUserData).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for CrmLogRecordRead {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for CrmLogRecordRead {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CrmLogRecordRead>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for CrmLogRecordRead {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CrmTransactionState(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Active: CrmTransactionState = CrmTransactionState(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Committed: CrmTransactionState = CrmTransactionState(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Aborted: CrmTransactionState = CrmTransactionState(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Indoubt: CrmTransactionState = CrmTransactionState(3i32);
impl ::core::marker::Copy for CrmTransactionState {}
impl ::core::clone::Clone for CrmTransactionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CrmTransactionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CrmTransactionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CrmTransactionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrmTransactionState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DUMPTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_FULL: DUMPTYPE = DUMPTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_MINI: DUMPTYPE = DUMPTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_NONE: DUMPTYPE = DUMPTYPE(2i32);
impl ::core::marker::Copy for DUMPTYPE {}
impl ::core::clone::Clone for DUMPTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DUMPTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DUMPTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DUMPTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUMPTYPE").field(&self.0).finish()
    }
}
pub const DispenserManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub const EventServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GUID_STRING_SIZE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GetAppTrackerDataFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = GetAppTrackerDataFlags(2i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = GetAppTrackerDataFlags(4i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(8i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(16i32);
impl ::core::marker::Copy for GetAppTrackerDataFlags {}
impl ::core::clone::Clone for GetAppTrackerDataFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GetAppTrackerDataFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GetAppTrackerDataFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for GetAppTrackerDataFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetAppTrackerDataFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn GetDispenserManager() -> ::windows::core::Result<IDispenserManager> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDispenserManager(param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetDispenserManager(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDispenserManager>(result__)
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn GetManagedExtensions(dwexts: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetManagedExtensions(dwexts: *mut u32) -> ::windows::core::HRESULT;
    }
    GetManagedExtensions(::core::mem::transmute(dwexts)).ok()
}
pub const GetSecurityCallContextAppObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: super::super::Foundation::BOOL,
    pub fTerminateOnHang: super::super::Foundation::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HANG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HANG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HANG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HANG_INFO").field("fAppHangMonitorEnabled", &self.fAppHangMonitorEnabled).field("fTerminateOnHang", &self.fTerminateOnHang).field("DumpType", &self.DumpType).field("dwHangTimeout", &self.dwHangTimeout).field("dwDumpCount", &self.dwDumpCount).field("dwInfoMsgCount", &self.dwInfoMsgCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HANG_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HANG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HANG_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HANG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HANG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAppDomainHelper(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAppDomainHelper {
    pub unsafe fn Initialize<'a, P0>(&self, punkad: P0, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), punkad.into().abi(), __midl__iappdomainhelper0000, ::core::mem::transmute(ppool)).ok()
    }
    pub unsafe fn DoCallback<'a, P0>(&self, punkad: P0, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).DoCallback)(::windows::core::Interface::as_raw(self), punkad.into().abi(), __midl__iappdomainhelper0001, ::core::mem::transmute(ppool)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAppDomainHelper> for ::windows::core::IUnknown {
    fn from(value: IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IAppDomainHelper> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAppDomainHelper> for ::windows::core::IUnknown {
    fn from(value: &IAppDomainHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAppDomainHelper> for super::Com::IDispatch {
    fn from(value: IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IAppDomainHelper> for &'a super::Com::IDispatch {
    fn from(value: &'a IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAppDomainHelper> for super::Com::IDispatch {
    fn from(value: &IAppDomainHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAppDomainHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAppDomainHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAppDomainHelper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAppDomainHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppDomainHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAppDomainHelper {
    type Vtable = IAppDomainHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7b67079_8255_42c6_9ec0_6994a3548780);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAppDomainHelper_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DoCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAssemblyLocator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyLocator {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetModules<'a, P0, P1, P2>(&self, applicationdir: P0, applicationname: P1, assemblyname: P2) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetModules)(::windows::core::Interface::as_raw(self), applicationdir.into().abi(), applicationname.into().abi(), assemblyname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAssemblyLocator> for ::windows::core::IUnknown {
    fn from(value: IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IAssemblyLocator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAssemblyLocator> for ::windows::core::IUnknown {
    fn from(value: &IAssemblyLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAssemblyLocator> for super::Com::IDispatch {
    fn from(value: IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IAssemblyLocator> for &'a super::Com::IDispatch {
    fn from(value: &'a IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAssemblyLocator> for super::Com::IDispatch {
    fn from(value: &IAssemblyLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAssemblyLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAssemblyLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAssemblyLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAssemblyLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAssemblyLocator {
    type Vtable = IAssemblyLocator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x391ffbb9_a8ee_432a_abc8_baa238dab90f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetModules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetModules: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IAsyncErrorNotify(::windows::core::IUnknown);
impl IAsyncErrorNotify {
    pub unsafe fn OnError(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnError)(::windows::core::Interface::as_raw(self), hr).ok()
    }
}
impl ::core::convert::From<IAsyncErrorNotify> for ::windows::core::IUnknown {
    fn from(value: IAsyncErrorNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAsyncErrorNotify> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAsyncErrorNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncErrorNotify> for ::windows::core::IUnknown {
    fn from(value: &IAsyncErrorNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAsyncErrorNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncErrorNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncErrorNotify {}
impl ::core::fmt::Debug for IAsyncErrorNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncErrorNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAsyncErrorNotify {
    type Vtable = IAsyncErrorNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe6777fb_a674_4177_8f32_6d707e113484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncErrorNotify_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICOMAdminCatalog(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCollection<'a, P0>(&self, bstrcollname: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCollection)(::windows::core::Interface::as_raw(self), bstrcollname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Connect<'a, P0>(&self, bstrcatalogservername: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), bstrcatalogservername.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MajorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MinorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCollectionByQuery<'a, P0>(&self, bstrcollname: P0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCollectionByQuery)(::windows::core::Interface::as_raw(self), bstrcollname.into().abi(), ::core::mem::transmute(ppsavarquery), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportComponent<'a, P0, P1>(&self, bstrapplidorname: P0, bstrclsidorprogid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ImportComponent)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrclsidorprogid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallComponent<'a, P0, P1, P2, P3>(&self, bstrapplidorname: P0, bstrdll: P1, bstrtlb: P2, bstrpsdll: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallComponent)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrdll.into().abi(), bstrtlb.into().abi(), bstrpsdll.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShutdownApplication<'a, P0>(&self, bstrapplidorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ShutdownApplication)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportApplication<'a, P0, P1>(&self, bstrapplidorname: P0, bstrapplicationfile: P1, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ExportApplication)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrapplicationfile.into().abi(), loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallApplication<'a, P0, P1, P2, P3, P4>(&self, bstrapplicationfile: P0, bstrdestinationdirectory: P1, loptions: COMAdminApplicationInstallOptions, bstruserid: P2, bstrpassword: P3, bstrrsn: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallApplication)(::windows::core::Interface::as_raw(self), bstrapplicationfile.into().abi(), bstrdestinationdirectory.into().abi(), loptions, bstruserid.into().abi(), bstrpassword.into().abi(), bstrrsn.into().abi()).ok()
    }
    pub unsafe fn StopRouter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopRouter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshRouter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshRouter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartRouter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartRouter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reserved1)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reserved2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstallMultipleComponents<'a, P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallMultipleComponents)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMultipleComponentsInfo<'a, P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GetMultipleComponentsInfo)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarclassnames), ::core::mem::transmute(ppsavarfileflags), ::core::mem::transmute(ppsavarcomponentflags)).ok()
    }
    pub unsafe fn RefreshComponents(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshComponents)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupREGDB<'a, P0>(&self, bstrbackupfilepath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).BackupREGDB)(::windows::core::Interface::as_raw(self), bstrbackupfilepath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreREGDB<'a, P0>(&self, bstrbackupfilepath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).RestoreREGDB)(::windows::core::Interface::as_raw(self), bstrbackupfilepath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryApplicationFile<'a, P0>(&self, bstrapplicationfile: P0, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).QueryApplicationFile)(::windows::core::Interface::as_raw(self), bstrapplicationfile.into().abi(), ::core::mem::transmute(pbstrapplicationname), ::core::mem::transmute(pbstrapplicationdescription), ::core::mem::transmute(pbhasusers), ::core::mem::transmute(pbisproxy), ::core::mem::transmute(ppsavarfilenames)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartApplication<'a, P0>(&self, bstrapplidorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).StartApplication)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi()).ok()
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ServiceCheck)(::windows::core::Interface::as_raw(self), lservice, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstallMultipleEventClasses<'a, P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallMultipleEventClasses)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallEventClass<'a, P0, P1, P2, P3>(&self, bstrapplidorname: P0, bstrdll: P1, bstrtlb: P2, bstrpsdll: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallEventClass)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrdll.into().abi(), bstrtlb.into().abi(), bstrpsdll.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEventClassesForIID<'a, P0>(&self, bstriid: P0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GetEventClassesForIID)(::windows::core::Interface::as_raw(self), bstriid.into().abi(), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarprogids), ::core::mem::transmute(ppsavardescriptions)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog> for ::windows::core::IUnknown {
    fn from(value: ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICOMAdminCatalog> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog> for ::windows::core::IUnknown {
    fn from(value: &ICOMAdminCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog> for super::Com::IDispatch {
    fn from(value: ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICOMAdminCatalog> for &'a super::Com::IDispatch {
    fn from(value: &'a ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog> for super::Com::IDispatch {
    fn from(value: &ICOMAdminCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICOMAdminCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICOMAdminCatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICOMAdminCatalog {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICOMAdminCatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICOMAdminCatalog").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICOMAdminCatalog {
    type Vtable = ICOMAdminCatalog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd662187_dfc2_11d1_a2cf_00805fc79235);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollectionByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollectionByQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShutdownApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShutdownApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallApplication: usize,
    pub StopRouter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RefreshRouter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartRouter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reserved2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InstallMultipleComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InstallMultipleComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMultipleComponentsInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMultipleComponentsInfo: usize,
    pub RefreshComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupREGDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupREGDB: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RestoreREGDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestoreREGDB: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryApplicationFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryApplicationFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartApplication: usize,
    pub ServiceCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InstallMultipleEventClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InstallMultipleEventClasses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallEventClass: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetEventClassesForIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetEventClassesForIID: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICOMAdminCatalog2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCollection<'a, P0>(&self, bstrcollname: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCollection)(::windows::core::Interface::as_raw(self), bstrcollname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Connect<'a, P0>(&self, bstrcatalogservername: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Connect)(::windows::core::Interface::as_raw(self), bstrcatalogservername.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MajorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MinorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetCollectionByQuery<'a, P0>(&self, bstrcollname: P0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCollectionByQuery)(::windows::core::Interface::as_raw(self), bstrcollname.into().abi(), ::core::mem::transmute(ppsavarquery), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportComponent<'a, P0, P1>(&self, bstrapplidorname: P0, bstrclsidorprogid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.ImportComponent)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrclsidorprogid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallComponent<'a, P0, P1, P2, P3>(&self, bstrapplidorname: P0, bstrdll: P1, bstrtlb: P2, bstrpsdll: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.InstallComponent)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrdll.into().abi(), bstrtlb.into().abi(), bstrpsdll.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShutdownApplication<'a, P0>(&self, bstrapplidorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.ShutdownApplication)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportApplication<'a, P0, P1>(&self, bstrapplidorname: P0, bstrapplicationfile: P1, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.ExportApplication)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrapplicationfile.into().abi(), loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallApplication<'a, P0, P1, P2, P3, P4>(&self, bstrapplicationfile: P0, bstrdestinationdirectory: P1, loptions: COMAdminApplicationInstallOptions, bstruserid: P2, bstrpassword: P3, bstrrsn: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.InstallApplication)(::windows::core::Interface::as_raw(self), bstrapplicationfile.into().abi(), bstrdestinationdirectory.into().abi(), loptions, bstruserid.into().abi(), bstrpassword.into().abi(), bstrrsn.into().abi()).ok()
    }
    pub unsafe fn StopRouter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StopRouter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshRouter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RefreshRouter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartRouter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartRouter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reserved1)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reserved2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstallMultipleComponents<'a, P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.InstallMultipleComponents)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMultipleComponentsInfo<'a, P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMultipleComponentsInfo)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarclassnames), ::core::mem::transmute(ppsavarfileflags), ::core::mem::transmute(ppsavarcomponentflags)).ok()
    }
    pub unsafe fn RefreshComponents(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RefreshComponents)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupREGDB<'a, P0>(&self, bstrbackupfilepath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.BackupREGDB)(::windows::core::Interface::as_raw(self), bstrbackupfilepath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreREGDB<'a, P0>(&self, bstrbackupfilepath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.RestoreREGDB)(::windows::core::Interface::as_raw(self), bstrbackupfilepath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryApplicationFile<'a, P0>(&self, bstrapplicationfile: P0, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.QueryApplicationFile)(::windows::core::Interface::as_raw(self), bstrapplicationfile.into().abi(), ::core::mem::transmute(pbstrapplicationname), ::core::mem::transmute(pbstrapplicationdescription), ::core::mem::transmute(pbhasusers), ::core::mem::transmute(pbisproxy), ::core::mem::transmute(ppsavarfilenames)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartApplication<'a, P0>(&self, bstrapplidorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.StartApplication)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi()).ok()
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ServiceCheck)(::windows::core::Interface::as_raw(self), lservice, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstallMultipleEventClasses<'a, P0>(&self, bstrapplidorname: P0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.InstallMultipleEventClasses)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallEventClass<'a, P0, P1, P2, P3>(&self, bstrapplidorname: P0, bstrdll: P1, bstrtlb: P2, bstrpsdll: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.InstallEventClass)(::windows::core::Interface::as_raw(self), bstrapplidorname.into().abi(), bstrdll.into().abi(), bstrtlb.into().abi(), bstrpsdll.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEventClassesForIID<'a, P0>(&self, bstriid: P0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.GetEventClassesForIID)(::windows::core::Interface::as_raw(self), bstriid.into().abi(), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarprogids), ::core::mem::transmute(ppsavardescriptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCollectionByQuery2<'a, P0>(&self, bstrcollectionname: P0, pvarquerystrings: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCollectionByQuery2)(::windows::core::Interface::as_raw(self), bstrcollectionname.into().abi(), ::core::mem::transmute(pvarquerystrings), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetApplicationInstanceIDFromProcessID)(::windows::core::Interface::as_raw(self), lprocessid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownApplicationInstances)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PauseApplicationInstances)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeApplicationInstances)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RecycleApplicationInstances)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid), lreasoncode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AreApplicationInstancesPaused)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpApplicationInstance<'a, P0, P1>(&self, bstrapplicationinstanceid: P0, bstrdirectory: P1, lmaximages: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DumpApplicationInstance)(::windows::core::Interface::as_raw(self), bstrapplicationinstanceid.into().abi(), bstrdirectory.into().abi(), lmaximages, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn IsApplicationInstanceDumpSupported(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsApplicationInstanceDumpSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateServiceForApplication<'a, P0, P1, P2, P3, P4, P5, P6>(&self, bstrapplicationidorname: P0, bstrservicename: P1, bstrstarttype: P2, bstrerrorcontrol: P3, bstrdependencies: P4, bstrrunas: P5, bstrpassword: P6, bdesktopok: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P6: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).CreateServiceForApplication)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi(), bstrservicename.into().abi(), bstrstarttype.into().abi(), bstrerrorcontrol.into().abi(), bstrdependencies.into().abi(), bstrrunas.into().abi(), bstrpassword.into().abi(), bdesktopok).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteServiceForApplication<'a, P0>(&self, bstrapplicationidorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).DeleteServiceForApplication)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartitionID<'a, P0>(&self, bstrapplicationidorname: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPartitionID)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartitionName<'a, P0>(&self, bstrapplicationidorname: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPartitionName)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCurrentPartition<'a, P0>(&self, bstrpartitionidorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetCurrentPartition)(::windows::core::Interface::as_raw(self), bstrpartitionidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentPartitionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPartitionID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentPartitionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPartitionName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GlobalPartitionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GlobalPartitionID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn FlushPartitionCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushPartitionCache)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyApplications<'a, P0, P1>(&self, bstrsourcepartitionidorname: P0, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).CopyApplications)(::windows::core::Interface::as_raw(self), bstrsourcepartitionidorname.into().abi(), ::core::mem::transmute(pvarapplicationid), bstrdestinationpartitionidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyComponents<'a, P0, P1>(&self, bstrsourceapplicationidorname: P0, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).CopyComponents)(::windows::core::Interface::as_raw(self), bstrsourceapplicationidorname.into().abi(), ::core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveComponents<'a, P0, P1>(&self, bstrsourceapplicationidorname: P0, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).MoveComponents)(::windows::core::Interface::as_raw(self), bstrsourceapplicationidorname.into().abi(), ::core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AliasComponent<'a, P0, P1, P2, P3, P4>(&self, bstrsrcapplicationidorname: P0, bstrclsidorprogid: P1, bstrdestapplicationidorname: P2, bstrnewprogid: P3, bstrnewclsid: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AliasComponent)(::windows::core::Interface::as_raw(self), bstrsrcapplicationidorname.into().abi(), bstrclsidorprogid.into().abi(), bstrdestapplicationidorname.into().abi(), bstrnewprogid.into().abi(), bstrnewclsid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSafeToDelete<'a, P0>(&self, bstrdllname: P0) -> ::windows::core::Result<COMAdminInUse>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSafeToDelete)(::windows::core::Interface::as_raw(self), bstrdllname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<COMAdminInUse>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportUnconfiguredComponents<'a, P0>(&self, bstrapplicationidorname: P0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ImportUnconfiguredComponents)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PromoteUnconfiguredComponents<'a, P0>(&self, bstrapplicationidorname: P0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).PromoteUnconfiguredComponents)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportComponents<'a, P0>(&self, bstrapplicationidorname: P0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ImportComponents)(::windows::core::Interface::as_raw(self), bstrapplicationidorname.into().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    pub unsafe fn Is64BitCatalogServer(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Is64BitCatalogServer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportPartition<'a, P0, P1>(&self, bstrpartitionidorname: P0, bstrpartitionfilename: P1, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ExportPartition)(::windows::core::Interface::as_raw(self), bstrpartitionidorname.into().abi(), bstrpartitionfilename.into().abi(), loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallPartition<'a, P0, P1, P2, P3, P4>(&self, bstrfilename: P0, bstrdestdirectory: P1, loptions: COMAdminApplicationInstallOptions, bstruserid: P2, bstrpassword: P3, bstrrsn: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InstallPartition)(::windows::core::Interface::as_raw(self), bstrfilename.into().abi(), bstrdestdirectory.into().abi(), loptions, bstruserid.into().abi(), bstrpassword.into().abi(), bstrrsn.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryApplicationFile2<'a, P0>(&self, bstrapplicationfile: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryApplicationFile2)(::windows::core::Interface::as_raw(self), bstrapplicationfile.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentVersionCount<'a, P0>(&self, bstrclsidorprogid: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetComponentVersionCount)(::windows::core::Interface::as_raw(self), bstrclsidorprogid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for ::windows::core::IUnknown {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICOMAdminCatalog2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for ::windows::core::IUnknown {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for super::Com::IDispatch {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICOMAdminCatalog2> for &'a super::Com::IDispatch {
    fn from(value: &'a ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for super::Com::IDispatch {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for ICOMAdminCatalog {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICOMAdminCatalog2> for &'a ICOMAdminCatalog {
    fn from(value: &'a ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for ICOMAdminCatalog {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICOMAdminCatalog2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICOMAdminCatalog2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICOMAdminCatalog2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICOMAdminCatalog2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICOMAdminCatalog2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICOMAdminCatalog2 {
    type Vtable = ICOMAdminCatalog2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790c6e0b_9194_4cc9_9426_a48a63185696);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog2_Vtbl {
    pub base__: ICOMAdminCatalog_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCollectionByQuery2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCollectionByQuery2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationInstanceIDFromProcessID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationInstanceIDFromProcessID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ShutdownApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ShutdownApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PauseApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PauseApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ResumeApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ResumeApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RecycleApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RecycleApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AreApplicationInstancesPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AreApplicationInstancesPaused: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DumpApplicationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DumpApplicationInstance: usize,
    pub IsApplicationInstanceDumpSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateServiceForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bdesktopok: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateServiceForApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteServiceForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteServiceForApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCurrentPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCurrentPartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentPartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentPartitionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentPartitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentPartitionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GlobalPartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GlobalPartitionID: usize,
    pub FlushPartitionCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyApplications: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MoveComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MoveComponents: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AliasComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AliasComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSafeToDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSafeToDelete: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportUnconfiguredComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PromoteUnconfiguredComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PromoteUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportComponents: usize,
    pub Is64BitCatalogServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbis64bit: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportPartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPartition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryApplicationFile2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryApplicationFile2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentVersionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentVersionCount: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICOMLBArguments(::windows::core::IUnknown);
impl ICOMLBArguments {
    pub unsafe fn GetCLSID(&self, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclsid)).ok()
    }
    pub unsafe fn SetCLSID(&self, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCLSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclsid)).ok()
    }
    pub unsafe fn GetMachineName(&self, szservername: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMachineName)(::windows::core::Interface::as_raw(self), szservername.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(szservername))).ok()
    }
    pub unsafe fn SetMachineName(&self, szservername: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMachineName)(::windows::core::Interface::as_raw(self), szservername.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(szservername))).ok()
    }
}
impl ::core::convert::From<ICOMLBArguments> for ::windows::core::IUnknown {
    fn from(value: ICOMLBArguments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICOMLBArguments> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICOMLBArguments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICOMLBArguments> for ::windows::core::IUnknown {
    fn from(value: &ICOMLBArguments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICOMLBArguments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICOMLBArguments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICOMLBArguments {}
impl ::core::fmt::Debug for ICOMLBArguments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICOMLBArguments").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICOMLBArguments {
    type Vtable = ICOMLBArguments_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a0f150f_8ee5_4b94_b40e_aef2f9e42ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICOMLBArguments_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICatalogCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), lindex).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn Populate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Populate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveChanges(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SaveChanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCollection<'a, P0, P1>(&self, bstrcollname: P0, varobjectkey: P1) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCollection)(::windows::core::Interface::as_raw(self), bstrcollname.into().abi(), varobjectkey.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn AddEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn RemoveEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RemoveEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUtilInterface(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUtilInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn DataStoreMajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DataStoreMajorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DataStoreMinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DataStoreMinorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PopulateByKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psakeys)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PopulateByQuery<'a, P0>(&self, bstrquerystring: P0, lquerytype: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).PopulateByQuery)(::windows::core::Interface::as_raw(self), bstrquerystring.into().abi(), lquerytype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogCollection> for ::windows::core::IUnknown {
    fn from(value: ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICatalogCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogCollection> for ::windows::core::IUnknown {
    fn from(value: &ICatalogCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogCollection> for super::Com::IDispatch {
    fn from(value: ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICatalogCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogCollection> for super::Com::IDispatch {
    fn from(value: &ICatalogCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICatalogCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICatalogCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICatalogCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICatalogCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICatalogCollection {
    type Vtable = ICatalogCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22872_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Populate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Name: usize,
    pub AddEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT,
    pub RemoveEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUtilInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUtilInterface: usize,
    pub DataStoreMajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT,
    pub DataStoreMinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PopulateByKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PopulateByKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PopulateByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PopulateByQuery: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICatalogObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Value<'a, P0>(&self, bstrpropname: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Value)(::windows::core::Interface::as_raw(self), bstrpropname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_Value<'a, P0, P1>(&self, bstrpropname: P0, val: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).put_Value)(::windows::core::Interface::as_raw(self), bstrpropname.into().abi(), val.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Key(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Key)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPropertyReadOnly<'a, P0>(&self, bstrpropname: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsPropertyReadOnly)(::windows::core::Interface::as_raw(self), bstrpropname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Valid(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Valid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPropertyWriteOnly<'a, P0>(&self, bstrpropname: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsPropertyWriteOnly)(::windows::core::Interface::as_raw(self), bstrpropname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogObject> for ::windows::core::IUnknown {
    fn from(value: ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICatalogObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogObject> for ::windows::core::IUnknown {
    fn from(value: &ICatalogObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogObject> for super::Com::IDispatch {
    fn from(value: ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICatalogObject> for &'a super::Com::IDispatch {
    fn from(value: &'a ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogObject> for super::Com::IDispatch {
    fn from(value: &ICatalogObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICatalogObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICatalogObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICatalogObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICatalogObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICatalogObject {
    type Vtable = ICatalogObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb22871_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Key: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPropertyReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPropertyReadOnly: usize,
    pub Valid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbretval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPropertyWriteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPropertyWriteOnly: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICheckSxsConfig(::windows::core::IUnknown);
impl ICheckSxsConfig {
    pub unsafe fn IsSameSxsConfig<'a, P0, P1, P2>(&self, wszsxsname: P0, wszsxsdirectory: P1, wszsxsappname: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsSameSxsConfig)(::windows::core::Interface::as_raw(self), wszsxsname.into(), wszsxsdirectory.into(), wszsxsappname.into()).ok()
    }
}
impl ::core::convert::From<ICheckSxsConfig> for ::windows::core::IUnknown {
    fn from(value: ICheckSxsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICheckSxsConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICheckSxsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICheckSxsConfig> for ::windows::core::IUnknown {
    fn from(value: &ICheckSxsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICheckSxsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICheckSxsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICheckSxsConfig {}
impl ::core::fmt::Debug for ICheckSxsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICheckSxsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICheckSxsConfig {
    type Vtable = ICheckSxsConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ff5a96f_11fc_47d1_baa6_25dd347e7242);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICheckSxsConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub IsSameSxsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszsxsname: ::windows::core::PCWSTR, wszsxsdirectory: ::windows::core::PCWSTR, wszsxsappname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComActivityEvents(::windows::core::IUnknown);
impl IComActivityEvents {
    pub unsafe fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityCreate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity)).ok()
    }
    pub unsafe fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityDestroy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity)).ok()
    }
    pub unsafe fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityEnter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidentered), dwthread).ok()
    }
    pub unsafe fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidentered), dwthread, dwtimeout).ok()
    }
    pub unsafe fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityReenter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), dwthread, dwcalldepth).ok()
    }
    pub unsafe fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityLeave)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidleft)).ok()
    }
    pub unsafe fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityLeaveSame)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), dwcalldepth).ok()
    }
}
impl ::core::convert::From<IComActivityEvents> for ::windows::core::IUnknown {
    fn from(value: IComActivityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComActivityEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComActivityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComActivityEvents> for ::windows::core::IUnknown {
    fn from(value: &IComActivityEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComActivityEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComActivityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComActivityEvents {}
impl ::core::fmt::Debug for IComActivityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComActivityEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComActivityEvents {
    type Vtable = IComActivityEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130b0_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComActivityEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnActivityCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnActivityDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnActivityEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::HRESULT,
    pub OnActivityTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub OnActivityReenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::HRESULT,
    pub OnActivityLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnActivityLeaveSame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComApp2Events(::windows::core::IUnknown);
impl IComApp2Events {
    pub unsafe fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppActivation2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp), ::core::mem::transmute(guidprocess)).ok()
    }
    pub unsafe fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppShutdown2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppForceShutdown2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnAppPaused2<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnAppPaused2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp), bpaused.into()).ok()
    }
    pub unsafe fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppRecycle2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp), ::core::mem::transmute(guidprocess), lreason).ok()
    }
}
impl ::core::convert::From<IComApp2Events> for ::windows::core::IUnknown {
    fn from(value: IComApp2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComApp2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComApp2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComApp2Events> for ::windows::core::IUnknown {
    fn from(value: &IComApp2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComApp2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComApp2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComApp2Events {}
impl ::core::fmt::Debug for IComApp2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComApp2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComApp2Events {
    type Vtable = IComApp2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1290bc1a_b219_418d_b078_5934ded08242);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComApp2Events_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnAppActivation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnAppShutdown2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnAppForceShutdown2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAppPaused2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAppPaused2: usize,
    pub OnAppRecycle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComAppEvents(::windows::core::IUnknown);
impl IComAppEvents {
    pub unsafe fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppActivation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppShutdown)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAppForceShutdown)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
}
impl ::core::convert::From<IComAppEvents> for ::windows::core::IUnknown {
    fn from(value: IComAppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComAppEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComAppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComAppEvents> for ::windows::core::IUnknown {
    fn from(value: &IComAppEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComAppEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComAppEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComAppEvents {}
impl ::core::fmt::Debug for IComAppEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComAppEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComAppEvents {
    type Vtable = IComAppEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130a6_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComAppEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnAppActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnAppForceShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComCRMEvents(::windows::core::IUnknown);
impl IComCRMEvents {
    pub unsafe fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMRecoveryStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMRecoveryDone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMCheckpoint)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidapp)).ok()
    }
    pub unsafe fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: &[u16; 64], szdescription: &[u16; 64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMBegin)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidtx), ::core::mem::transmute(::windows::core::as_ptr_or_null(szprogidcompensator)), ::core::mem::transmute(::windows::core::as_ptr_or_null(szdescription))).ok()
    }
    pub unsafe fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMPrepare)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMCommit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMAbort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMIndoubt)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMDone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMRelease)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMAnalyze)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid), dwcrmrecordtype, dwrecordsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCRMWrite<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: P0, dwrecordsize: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnCRMWrite)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid), fvariants.into(), dwrecordsize).ok()
    }
    pub unsafe fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMForget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    pub unsafe fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCRMForce)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCRMDeliver<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: P0, dwrecordsize: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnCRMDeliver)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidclerkclsid), fvariants.into(), dwrecordsize).ok()
    }
}
impl ::core::convert::From<IComCRMEvents> for ::windows::core::IUnknown {
    fn from(value: IComCRMEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComCRMEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComCRMEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComCRMEvents> for ::windows::core::IUnknown {
    fn from(value: &IComCRMEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComCRMEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComCRMEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComCRMEvents {}
impl ::core::fmt::Debug for IComCRMEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComCRMEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComCRMEvents {
    type Vtable = IComCRMEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130b5_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComCRMEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnCRMRecoveryStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMRecoveryDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMCheckpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: ::windows::core::PCWSTR, szdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnCRMPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMIndoubt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMAnalyze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnCRMWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnCRMWrite: usize,
    pub OnCRMForget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnCRMForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnCRMDeliver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnCRMDeliver: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComExceptionEvents(::windows::core::IUnknown);
impl IComExceptionEvents {
    pub unsafe fn OnExceptionUser<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnExceptionUser)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), code, address, pszstacktrace.into()).ok()
    }
}
impl ::core::convert::From<IComExceptionEvents> for ::windows::core::IUnknown {
    fn from(value: IComExceptionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComExceptionEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComExceptionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComExceptionEvents> for ::windows::core::IUnknown {
    fn from(value: &IComExceptionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComExceptionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComExceptionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComExceptionEvents {}
impl ::core::fmt::Debug for IComExceptionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComExceptionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComExceptionEvents {
    type Vtable = IComExceptionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130b3_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComExceptionEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnExceptionUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComIdentityEvents(::windows::core::IUnknown);
impl IComIdentityEvents {
    pub unsafe fn OnIISRequestInfo<'a, P0, P1, P2>(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: P0, pszserverip: P1, pszurl: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnIISRequestInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objid, pszclientip.into(), pszserverip.into(), pszurl.into()).ok()
    }
}
impl ::core::convert::From<IComIdentityEvents> for ::windows::core::IUnknown {
    fn from(value: IComIdentityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComIdentityEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComIdentityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComIdentityEvents> for ::windows::core::IUnknown {
    fn from(value: &IComIdentityEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComIdentityEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComIdentityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComIdentityEvents {}
impl ::core::fmt::Debug for IComIdentityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComIdentityEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComIdentityEvents {
    type Vtable = IComIdentityEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130b1_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComIdentityEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnIISRequestInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: ::windows::core::PCWSTR, pszserverip: ::windows::core::PCWSTR, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComInstance2Events(::windows::core::IUnknown);
impl IComInstance2Events {
    pub unsafe fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectCreate2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(clsid), ::core::mem::transmute(tsid), ctxtid, objectid, ::core::mem::transmute(guidpartition)).ok()
    }
    pub unsafe fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectDestroy2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid).ok()
    }
}
impl ::core::convert::From<IComInstance2Events> for ::windows::core::IUnknown {
    fn from(value: IComInstance2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComInstance2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComInstance2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComInstance2Events> for ::windows::core::IUnknown {
    fn from(value: &IComInstance2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComInstance2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComInstance2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComInstance2Events {}
impl ::core::fmt::Debug for IComInstance2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComInstance2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComInstance2Events {
    type Vtable = IComInstance2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e3bf07_b506_4ad5_a50c_d2ca5b9c158e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstance2Events_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjectCreate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnObjectDestroy2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComInstanceEvents(::windows::core::IUnknown);
impl IComInstanceEvents {
    pub unsafe fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectCreate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(clsid), ::core::mem::transmute(tsid), ctxtid, objectid).ok()
    }
    pub unsafe fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectDestroy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid).ok()
    }
}
impl ::core::convert::From<IComInstanceEvents> for ::windows::core::IUnknown {
    fn from(value: IComInstanceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComInstanceEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComInstanceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComInstanceEvents> for ::windows::core::IUnknown {
    fn from(value: &IComInstanceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComInstanceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComInstanceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComInstanceEvents {}
impl ::core::fmt::Debug for IComInstanceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComInstanceEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComInstanceEvents {
    type Vtable = IComInstanceEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130a7_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstanceEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjectCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT,
    pub OnObjectDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComLTxEvents(::windows::core::IUnknown);
impl IComLTxEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnLtxTransactionStart<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: P0, nisolationlevel: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnLtxTransactionStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidltx), ::core::mem::transmute(tsid), froot.into(), nisolationlevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnLtxTransactionPrepare<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnLtxTransactionPrepare)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidltx), fvote.into()).ok()
    }
    pub unsafe fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLtxTransactionAbort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidltx)).ok()
    }
    pub unsafe fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLtxTransactionCommit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidltx)).ok()
    }
    pub unsafe fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLtxTransactionPromote)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidltx), ::core::mem::transmute(txnid)).ok()
    }
}
impl ::core::convert::From<IComLTxEvents> for ::windows::core::IUnknown {
    fn from(value: IComLTxEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComLTxEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComLTxEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComLTxEvents> for ::windows::core::IUnknown {
    fn from(value: &IComLTxEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComLTxEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComLTxEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComLTxEvents {}
impl ::core::fmt::Debug for IComLTxEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComLTxEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComLTxEvents {
    type Vtable = IComLTxEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x605cf82c_578e_4298_975d_82babcd9e053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComLTxEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnLtxTransactionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnLtxTransactionStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnLtxTransactionPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnLtxTransactionPrepare: usize,
    pub OnLtxTransactionAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnLtxTransactionCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnLtxTransactionPromote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComMethod2Events(::windows::core::IUnknown);
impl IComMethod2Events {
    pub unsafe fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMethodCall2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), oid, ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), dwthread, imeth).ok()
    }
    pub unsafe fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMethodReturn2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), oid, ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), dwthread, imeth, hresult).ok()
    }
    pub unsafe fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMethodException2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), oid, ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), dwthread, imeth).ok()
    }
}
impl ::core::convert::From<IComMethod2Events> for ::windows::core::IUnknown {
    fn from(value: IComMethod2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComMethod2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComMethod2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComMethod2Events> for ::windows::core::IUnknown {
    fn from(value: &IComMethod2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComMethod2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComMethod2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComMethod2Events {}
impl ::core::fmt::Debug for IComMethod2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComMethod2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComMethod2Events {
    type Vtable = IComMethod2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb388aaa_567d_4024_af8e_6e93ee748573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethod2Events_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnMethodCall2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT,
    pub OnMethodReturn2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnMethodException2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComMethodEvents(::windows::core::IUnknown);
impl IComMethodEvents {
    pub unsafe fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMethodCall)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), oid, ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), imeth).ok()
    }
    pub unsafe fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMethodReturn)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), oid, ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), imeth, hresult).ok()
    }
    pub unsafe fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMethodException)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), oid, ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), imeth).ok()
    }
}
impl ::core::convert::From<IComMethodEvents> for ::windows::core::IUnknown {
    fn from(value: IComMethodEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComMethodEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComMethodEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComMethodEvents> for ::windows::core::IUnknown {
    fn from(value: &IComMethodEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComMethodEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComMethodEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComMethodEvents {}
impl ::core::fmt::Debug for IComMethodEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComMethodEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComMethodEvents {
    type Vtable = IComMethodEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130a9_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethodEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnMethodCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT,
    pub OnMethodReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnMethodException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComMtaThreadPoolKnobs(::windows::core::IUnknown);
impl IComMtaThreadPoolKnobs {
    pub unsafe fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MTASetMaxThreadCount)(::windows::core::Interface::as_raw(self), dwmaxthreads).ok()
    }
    pub unsafe fn MTAGetMaxThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MTAGetMaxThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn MTASetThrottleValue(&self, dwthrottle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MTASetThrottleValue)(::windows::core::Interface::as_raw(self), dwthrottle).ok()
    }
    pub unsafe fn MTAGetThrottleValue(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MTAGetThrottleValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IComMtaThreadPoolKnobs> for ::windows::core::IUnknown {
    fn from(value: IComMtaThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComMtaThreadPoolKnobs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComMtaThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComMtaThreadPoolKnobs> for ::windows::core::IUnknown {
    fn from(value: &IComMtaThreadPoolKnobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComMtaThreadPoolKnobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComMtaThreadPoolKnobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComMtaThreadPoolKnobs {}
impl ::core::fmt::Debug for IComMtaThreadPoolKnobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComMtaThreadPoolKnobs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComMtaThreadPoolKnobs {
    type Vtable = IComMtaThreadPoolKnobs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9a76d2e_76a5_43eb_a0c4_49bec8e48480);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMtaThreadPoolKnobs_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub MTASetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows::core::HRESULT,
    pub MTAGetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows::core::HRESULT,
    pub MTASetThrottleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows::core::HRESULT,
    pub MTAGetThrottleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComObjectConstruction2Events(::windows::core::IUnknown);
impl IComObjectConstruction2Events {
    pub unsafe fn OnObjectConstruct2<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: P0, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnObjectConstruct2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), sconstructstring.into(), oid, ::core::mem::transmute(guidpartition)).ok()
    }
}
impl ::core::convert::From<IComObjectConstruction2Events> for ::windows::core::IUnknown {
    fn from(value: IComObjectConstruction2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComObjectConstruction2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComObjectConstruction2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectConstruction2Events> for ::windows::core::IUnknown {
    fn from(value: &IComObjectConstruction2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComObjectConstruction2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectConstruction2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectConstruction2Events {}
impl ::core::fmt::Debug for IComObjectConstruction2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectConstruction2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComObjectConstruction2Events {
    type Vtable = IComObjectConstruction2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b5a7827_8df2_45c0_8f6f_57ea1f856a9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstruction2Events_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjectConstruct2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: ::windows::core::PCWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComObjectConstructionEvents(::windows::core::IUnknown);
impl IComObjectConstructionEvents {
    pub unsafe fn OnObjectConstruct<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: P0, oid: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnObjectConstruct)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), sconstructstring.into(), oid).ok()
    }
}
impl ::core::convert::From<IComObjectConstructionEvents> for ::windows::core::IUnknown {
    fn from(value: IComObjectConstructionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComObjectConstructionEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComObjectConstructionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectConstructionEvents> for ::windows::core::IUnknown {
    fn from(value: &IComObjectConstructionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComObjectConstructionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectConstructionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectConstructionEvents {}
impl ::core::fmt::Debug for IComObjectConstructionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectConstructionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComObjectConstructionEvents {
    type Vtable = IComObjectConstructionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130af_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstructionEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjectConstruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: ::windows::core::PCWSTR, oid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComObjectEvents(::windows::core::IUnknown);
impl IComObjectEvents {
    pub unsafe fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectActivate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid, objectid).ok()
    }
    pub unsafe fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectDeactivate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid, objectid).ok()
    }
    pub unsafe fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisableCommit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid).ok()
    }
    pub unsafe fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEnableCommit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid).ok()
    }
    pub unsafe fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetComplete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid).ok()
    }
    pub unsafe fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetAbort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ctxtid).ok()
    }
}
impl ::core::convert::From<IComObjectEvents> for ::windows::core::IUnknown {
    fn from(value: IComObjectEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComObjectEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComObjectEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectEvents> for ::windows::core::IUnknown {
    fn from(value: &IComObjectEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComObjectEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectEvents {}
impl ::core::fmt::Debug for IComObjectEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComObjectEvents {
    type Vtable = IComObjectEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130aa_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjectActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT,
    pub OnObjectDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT,
    pub OnDisableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT,
    pub OnEnableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT,
    pub OnSetComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT,
    pub OnSetAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComObjectPool2Events(::windows::core::IUnknown);
impl IComObjectPool2Events {
    pub unsafe fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolPutObject2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), nreason, dwavailable, oid).ok()
    }
    pub unsafe fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolGetObject2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), dwavailable, oid, ::core::mem::transmute(guidpartition)).ok()
    }
    pub unsafe fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolRecycleToTx2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), objid).ok()
    }
    pub unsafe fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolGetFromTx2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), objid, ::core::mem::transmute(guidpartition)).ok()
    }
}
impl ::core::convert::From<IComObjectPool2Events> for ::windows::core::IUnknown {
    fn from(value: IComObjectPool2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComObjectPool2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComObjectPool2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectPool2Events> for ::windows::core::IUnknown {
    fn from(value: &IComObjectPool2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComObjectPool2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectPool2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectPool2Events {}
impl ::core::fmt::Debug for IComObjectPool2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectPool2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComObjectPool2Events {
    type Vtable = IComObjectPool2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65bf6534_85ea_4f64_8cf4_3d974b2ab1cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPool2Events_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjPoolPutObject2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolGetObject2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnObjPoolRecycleToTx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolGetFromTx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComObjectPoolEvents(::windows::core::IUnknown);
impl IComObjectPoolEvents {
    pub unsafe fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolPutObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), nreason, dwavailable, oid).ok()
    }
    pub unsafe fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolGetObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), dwavailable, oid).ok()
    }
    pub unsafe fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolRecycleToTx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), objid).ok()
    }
    pub unsafe fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolGetFromTx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), objid).ok()
    }
}
impl ::core::convert::From<IComObjectPoolEvents> for ::windows::core::IUnknown {
    fn from(value: IComObjectPoolEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComObjectPoolEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComObjectPoolEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectPoolEvents> for ::windows::core::IUnknown {
    fn from(value: &IComObjectPoolEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComObjectPoolEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectPoolEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectPoolEvents {}
impl ::core::fmt::Debug for IComObjectPoolEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectPoolEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComObjectPoolEvents {
    type Vtable = IComObjectPoolEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130ad_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjPoolPutObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolGetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolRecycleToTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolGetFromTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComObjectPoolEvents2(::windows::core::IUnknown);
impl IComObjectPoolEvents2 {
    pub unsafe fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolCreateObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), dwobjscreated, oid).ok()
    }
    pub unsafe fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolDestroyObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), dwobjscreated, oid).ok()
    }
    pub unsafe fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolCreateDecision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), dwthreadswaiting, dwavail, dwcreated, dwmin, dwmax).ok()
    }
    pub unsafe fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidactivity), dwtimeout).ok()
    }
    pub unsafe fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjPoolCreatePool)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), dwmin, dwmax, dwtimeout).ok()
    }
}
impl ::core::convert::From<IComObjectPoolEvents2> for ::windows::core::IUnknown {
    fn from(value: IComObjectPoolEvents2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComObjectPoolEvents2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComObjectPoolEvents2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectPoolEvents2> for ::windows::core::IUnknown {
    fn from(value: &IComObjectPoolEvents2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComObjectPoolEvents2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectPoolEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectPoolEvents2 {}
impl ::core::fmt::Debug for IComObjectPoolEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectPoolEvents2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComObjectPoolEvents2 {
    type Vtable = IComObjectPoolEvents2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130ae_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnObjPoolCreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolDestroyObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT,
    pub OnObjPoolCreateDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::HRESULT,
    pub OnObjPoolTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub OnObjPoolCreatePool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComQCEvents(::windows::core::IUnknown);
impl IComQCEvents {
    pub unsafe fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &[u16; 60], guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objid, ::core::mem::transmute(::windows::core::as_ptr_or_null(szqueue)), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), msmqhr).ok()
    }
    pub unsafe fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &[u16; 60], queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCQueueOpen)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(::windows::core::as_ptr_or_null(szqueue)), queueid, hr).ok()
    }
    pub unsafe fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCReceive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), queueid, ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), hr).ok()
    }
    pub unsafe fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCReceiveFail)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), queueid, msmqhr).ok()
    }
    pub unsafe fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCMoveToReTryQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), retryindex).ok()
    }
    pub unsafe fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCMoveToDeadQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid)).ok()
    }
    pub unsafe fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQCPlayback)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objid, ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), hr).ok()
    }
}
impl ::core::convert::From<IComQCEvents> for ::windows::core::IUnknown {
    fn from(value: IComQCEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComQCEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComQCEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComQCEvents> for ::windows::core::IUnknown {
    fn from(value: &IComQCEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComQCEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComQCEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComQCEvents {}
impl ::core::fmt::Debug for IComQCEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComQCEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComQCEvents {
    type Vtable = IComQCEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130b2_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComQCEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnQCRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: ::windows::core::PCWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnQCQueueOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: ::windows::core::PCWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnQCReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnQCReceiveFail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnQCMoveToReTryQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::HRESULT,
    pub OnQCMoveToDeadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnQCPlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComResourceEvents(::windows::core::IUnknown);
impl IComResourceEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnResourceCreate<'a, P0, P1>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64, enlisted: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnResourceCreate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objectid, psztype.into(), resid, enlisted.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnResourceAllocate<'a, P0, P1>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64, enlisted: P1, numrated: u32, rating: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnResourceAllocate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objectid, psztype.into(), resid, enlisted.into(), numrated, rating).ok()
    }
    pub unsafe fn OnResourceRecycle<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnResourceRecycle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objectid, psztype.into(), resid).ok()
    }
    pub unsafe fn OnResourceDestroy<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: P0, resid: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnResourceDestroy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objectid, hr, psztype.into(), resid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnResourceTrack<'a, P0, P1>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P0, resid: u64, enlisted: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnResourceTrack)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), objectid, psztype.into(), resid, enlisted.into()).ok()
    }
}
impl ::core::convert::From<IComResourceEvents> for ::windows::core::IUnknown {
    fn from(value: IComResourceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComResourceEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComResourceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComResourceEvents> for ::windows::core::IUnknown {
    fn from(value: &IComResourceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComResourceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComResourceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComResourceEvents {}
impl ::core::fmt::Debug for IComResourceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComResourceEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComResourceEvents {
    type Vtable = IComResourceEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130ab_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComResourceEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnResourceCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnResourceCreate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnResourceAllocate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnResourceAllocate: usize,
    pub OnResourceRecycle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64) -> ::windows::core::HRESULT,
    pub OnResourceDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: ::windows::core::PCWSTR, resid: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnResourceTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnResourceTrack: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComSecurityEvents(::windows::core::IUnknown);
impl IComSecurityEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnAuthenticate<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnAuthenticate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), objectid, ::core::mem::transmute(guidiid), imeth, psidoriginaluser.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psidoriginaluser)), psidcurrentuser.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psidcurrentuser)), bcurrentuserinpersonatinginproc.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnAuthenticateFail<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnAuthenticateFail)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), objectid, ::core::mem::transmute(guidiid), imeth, psidoriginaluser.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psidoriginaluser)), psidcurrentuser.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psidcurrentuser)), bcurrentuserinpersonatinginproc.into()).ok()
    }
}
impl ::core::convert::From<IComSecurityEvents> for ::windows::core::IUnknown {
    fn from(value: IComSecurityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComSecurityEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComSecurityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComSecurityEvents> for ::windows::core::IUnknown {
    fn from(value: &IComSecurityEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComSecurityEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComSecurityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComSecurityEvents {}
impl ::core::fmt::Debug for IComSecurityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComSecurityEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComSecurityEvents {
    type Vtable = IComSecurityEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130ac_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComSecurityEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAuthenticate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAuthenticateFail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAuthenticateFail: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs(::windows::core::IUnknown);
impl IComStaThreadPoolKnobs {
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinThreadCount)(::windows::core::Interface::as_raw(self), minthreads).ok()
    }
    pub unsafe fn GetMinThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMinThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxThreadCount)(::windows::core::Interface::as_raw(self), maxthreads).ok()
    }
    pub unsafe fn GetMaxThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActivityPerThread)(::windows::core::Interface::as_raw(self), activitiesperthread).ok()
    }
    pub unsafe fn GetActivityPerThread(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetActivityPerThread)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActivityRatio)(::windows::core::Interface::as_raw(self), activityratio).ok()
    }
    pub unsafe fn GetActivityRatio(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetActivityRatio)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn GetThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetQueueDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetQueueDepth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQueueDepth)(::windows::core::Interface::as_raw(self), dwqdepth).ok()
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs> for ::windows::core::IUnknown {
    fn from(value: IComStaThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComStaThreadPoolKnobs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComStaThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs> for ::windows::core::IUnknown {
    fn from(value: &IComStaThreadPoolKnobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComStaThreadPoolKnobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComStaThreadPoolKnobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComStaThreadPoolKnobs {}
impl ::core::fmt::Debug for IComStaThreadPoolKnobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComStaThreadPoolKnobs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComStaThreadPoolKnobs {
    type Vtable = IComStaThreadPoolKnobs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324b64fa_33b6_11d2_98b7_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetMinThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows::core::HRESULT,
    pub GetMinThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows::core::HRESULT,
    pub GetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows::core::HRESULT,
    pub SetActivityPerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows::core::HRESULT,
    pub GetActivityPerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows::core::HRESULT,
    pub SetActivityRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows::core::HRESULT,
    pub GetActivityRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows::core::HRESULT,
    pub GetThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows::core::HRESULT,
    pub GetQueueDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows::core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs2(::windows::core::IUnknown);
impl IComStaThreadPoolKnobs2 {
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMinThreadCount)(::windows::core::Interface::as_raw(self), minthreads).ok()
    }
    pub unsafe fn GetMinThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMinThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaxThreadCount)(::windows::core::Interface::as_raw(self), maxthreads).ok()
    }
    pub unsafe fn GetMaxThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMaxThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetActivityPerThread)(::windows::core::Interface::as_raw(self), activitiesperthread).ok()
    }
    pub unsafe fn GetActivityPerThread(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetActivityPerThread)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetActivityRatio)(::windows::core::Interface::as_raw(self), activityratio).ok()
    }
    pub unsafe fn GetActivityRatio(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetActivityRatio)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn GetThreadCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetThreadCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetQueueDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetQueueDepth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetQueueDepth)(::windows::core::Interface::as_raw(self), dwqdepth).ok()
    }
    pub unsafe fn GetMaxCPULoad(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxCPULoad)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxCPULoad(&self, pdwload: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxCPULoad)(::windows::core::Interface::as_raw(self), pdwload).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCPUMetricEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCPUMetricEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCPUMetricEnabled<'a, P0>(&self, bmetricenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetCPUMetricEnabled)(::windows::core::Interface::as_raw(self), bmetricenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreateThreadsAggressively(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCreateThreadsAggressively)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreateThreadsAggressively<'a, P0>(&self, bmetricenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetCreateThreadsAggressively)(::windows::core::Interface::as_raw(self), bmetricenabled.into()).ok()
    }
    pub unsafe fn GetMaxCSR(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxCSR)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxCSR(&self, dwcsr: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxCSR)(::windows::core::Interface::as_raw(self), dwcsr).ok()
    }
    pub unsafe fn GetWaitTimeForThreadCleanup(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetWaitTimeForThreadCleanup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWaitTimeForThreadCleanup)(::windows::core::Interface::as_raw(self), dwthreadcleanupwaittime).ok()
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs2> for ::windows::core::IUnknown {
    fn from(value: IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComStaThreadPoolKnobs2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs2> for ::windows::core::IUnknown {
    fn from(value: &IComStaThreadPoolKnobs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs2> for IComStaThreadPoolKnobs {
    fn from(value: IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComStaThreadPoolKnobs2> for &'a IComStaThreadPoolKnobs {
    fn from(value: &'a IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs2> for IComStaThreadPoolKnobs {
    fn from(value: &IComStaThreadPoolKnobs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComStaThreadPoolKnobs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComStaThreadPoolKnobs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComStaThreadPoolKnobs2 {}
impl ::core::fmt::Debug for IComStaThreadPoolKnobs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComStaThreadPoolKnobs2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComStaThreadPoolKnobs2 {
    type Vtable = IComStaThreadPoolKnobs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73707523_ff9a_4974_bf84_2108dc213740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs2_Vtbl {
    pub base__: IComStaThreadPoolKnobs_Vtbl,
    pub GetMaxCPULoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxCPULoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCPUMetricEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCPUMetricEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCPUMetricEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCPUMetricEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreateThreadsAggressively: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreateThreadsAggressively: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreateThreadsAggressively: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreateThreadsAggressively: usize,
    pub GetMaxCSR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxCSR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows::core::HRESULT,
    pub GetWaitTimeForThreadCleanup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows::core::HRESULT,
    pub SetWaitTimeForThreadCleanup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComThreadEvents(::windows::core::IUnknown);
impl IComThreadEvents {
    pub unsafe fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, dwthread, dwtheadcnt).ok()
    }
    pub unsafe fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadTerminate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, dwthread, dwtheadcnt).ok()
    }
    pub unsafe fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadBindToApartment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, aptid, dwactcnt, dwlowcnt).ok()
    }
    pub unsafe fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadUnBind)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, aptid, dwactcnt).ok()
    }
    pub unsafe fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadWorkEnque)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, msgworkid, queuelen).ok()
    }
    pub unsafe fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadWorkPrivate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, msgworkid).ok()
    }
    pub unsafe fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadWorkPublic)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, msgworkid, queuelen).ok()
    }
    pub unsafe fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadWorkRedirect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, msgworkid, queuelen, threadnum).ok()
    }
    pub unsafe fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadWorkReject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), threadid, msgworkid, queuelen).ok()
    }
    pub unsafe fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadAssignApartment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), aptid).ok()
    }
    pub unsafe fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadUnassignApartment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), aptid).ok()
    }
}
impl ::core::convert::From<IComThreadEvents> for ::windows::core::IUnknown {
    fn from(value: IComThreadEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComThreadEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComThreadEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComThreadEvents> for ::windows::core::IUnknown {
    fn from(value: &IComThreadEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComThreadEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComThreadEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadEvents {}
impl ::core::fmt::Debug for IComThreadEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComThreadEvents {
    type Vtable = IComThreadEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130a5_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnThreadStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT,
    pub OnThreadBindToApartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::HRESULT,
    pub OnThreadUnBind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::HRESULT,
    pub OnThreadWorkEnque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT,
    pub OnThreadWorkPrivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::HRESULT,
    pub OnThreadWorkPublic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT,
    pub OnThreadWorkRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::HRESULT,
    pub OnThreadWorkReject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT,
    pub OnThreadAssignApartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::HRESULT,
    pub OnThreadUnassignApartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComTrackingInfoCollection(::windows::core::IUnknown);
impl IComTrackingInfoCollection {
    pub unsafe fn Type(&self) -> ::windows::core::Result<TRACKING_COLL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TRACKING_COLL_TYPE>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Item(&self, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), ulindex, ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IComTrackingInfoCollection> for ::windows::core::IUnknown {
    fn from(value: IComTrackingInfoCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComTrackingInfoCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComTrackingInfoCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoCollection> for ::windows::core::IUnknown {
    fn from(value: &IComTrackingInfoCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComTrackingInfoCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoCollection {}
impl ::core::fmt::Debug for IComTrackingInfoCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComTrackingInfoCollection {
    type Vtable = IComTrackingInfoCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc266c677_c9ad_49ab_9fd9_d9661078588a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoCollection_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComTrackingInfoEvents(::windows::core::IUnknown);
impl IComTrackingInfoEvents {
    pub unsafe fn OnNewTrackingInfo<'a, P0>(&self, ptoplevelcollection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).OnNewTrackingInfo)(::windows::core::Interface::as_raw(self), ptoplevelcollection.into().abi()).ok()
    }
}
impl ::core::convert::From<IComTrackingInfoEvents> for ::windows::core::IUnknown {
    fn from(value: IComTrackingInfoEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComTrackingInfoEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComTrackingInfoEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoEvents> for ::windows::core::IUnknown {
    fn from(value: &IComTrackingInfoEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComTrackingInfoEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoEvents {}
impl ::core::fmt::Debug for IComTrackingInfoEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComTrackingInfoEvents {
    type Vtable = IComTrackingInfoEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e6cdcc9_fb25_4fd5_9cc5_c9f4b6559cec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnNewTrackingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComTrackingInfoObject(::windows::core::IUnknown);
impl IComTrackingInfoObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<'a, P0>(&self, szpropertyname: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), szpropertyname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<IComTrackingInfoObject> for ::windows::core::IUnknown {
    fn from(value: IComTrackingInfoObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComTrackingInfoObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComTrackingInfoObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoObject> for ::windows::core::IUnknown {
    fn from(value: &IComTrackingInfoObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComTrackingInfoObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoObject {}
impl ::core::fmt::Debug for IComTrackingInfoObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComTrackingInfoObject {
    type Vtable = IComTrackingInfoObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x116e42c5_d8b1_47bf_ab1e_c895ed3e2372);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szpropertyname: ::windows::core::PCWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComTrackingInfoProperties(::windows::core::IUnknown);
impl IComTrackingInfoProperties {
    pub unsafe fn PropCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PropCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPropName(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPropName)(::windows::core::Interface::as_raw(self), ulindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IComTrackingInfoProperties> for ::windows::core::IUnknown {
    fn from(value: IComTrackingInfoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComTrackingInfoProperties> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComTrackingInfoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoProperties> for ::windows::core::IUnknown {
    fn from(value: &IComTrackingInfoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComTrackingInfoProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoProperties {}
impl ::core::fmt::Debug for IComTrackingInfoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComTrackingInfoProperties {
    type Vtable = IComTrackingInfoProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x789b42be_6f6b_443a_898e_67abf390aa14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoProperties_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PropCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComTransaction2Events(::windows::core::IUnknown);
impl IComTransaction2Events {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTransactionStart2<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: P0, nisolationlevel: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnTransactionStart2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), ::core::mem::transmute(tsid), froot.into(), nisolationlevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTransactionPrepare2<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnTransactionPrepare2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), fvoteyes.into()).ok()
    }
    pub unsafe fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTransactionAbort2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
    pub unsafe fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTransactionCommit2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
}
impl ::core::convert::From<IComTransaction2Events> for ::windows::core::IUnknown {
    fn from(value: IComTransaction2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComTransaction2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComTransaction2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTransaction2Events> for ::windows::core::IUnknown {
    fn from(value: &IComTransaction2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComTransaction2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTransaction2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTransaction2Events {}
impl ::core::fmt::Debug for IComTransaction2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTransaction2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComTransaction2Events {
    type Vtable = IComTransaction2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa136f62a_2f94_4288_86e0_d8a1fa4c0299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransaction2Events_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionStart2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionStart2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionPrepare2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionPrepare2: usize,
    pub OnTransactionAbort2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnTransactionCommit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComTransactionEvents(::windows::core::IUnknown);
impl IComTransactionEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTransactionStart<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnTransactionStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), ::core::mem::transmute(tsid), froot.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTransactionPrepare<'a, P0>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnTransactionPrepare)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), fvoteyes.into()).ok()
    }
    pub unsafe fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTransactionAbort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
    pub unsafe fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTransactionCommit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
}
impl ::core::convert::From<IComTransactionEvents> for ::windows::core::IUnknown {
    fn from(value: IComTransactionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComTransactionEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComTransactionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTransactionEvents> for ::windows::core::IUnknown {
    fn from(value: &IComTransactionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComTransactionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTransactionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTransactionEvents {}
impl ::core::fmt::Debug for IComTransactionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTransactionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComTransactionEvents {
    type Vtable = IComTransactionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130a8_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransactionEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionPrepare: usize,
    pub OnTransactionAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnTransactionCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IComUserEvent(::windows::core::IUnknown);
impl IComUserEvent {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUserEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(pvarevent)).ok()
    }
}
impl ::core::convert::From<IComUserEvent> for ::windows::core::IUnknown {
    fn from(value: IComUserEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IComUserEvent> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComUserEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComUserEvent> for ::windows::core::IUnknown {
    fn from(value: &IComUserEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IComUserEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComUserEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComUserEvent {}
impl ::core::fmt::Debug for IComUserEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComUserEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComUserEvent {
    type Vtable = IComUserEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130a4_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComUserEvent_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnUserEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnUserEvent: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IContextProperties(::windows::core::IUnknown);
impl IContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, P0>(&self, name: P0, pproperty: *mut super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn EnumNames(&self) -> ::windows::core::Result<IEnumNames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNames>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, P0, P1>(&self, name: P0, property: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), name.into().abi(), property.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveProperty<'a, P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).RemoveProperty)(::windows::core::Interface::as_raw(self), name.into().abi()).ok()
    }
}
impl ::core::convert::From<IContextProperties> for ::windows::core::IUnknown {
    fn from(value: IContextProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContextProperties> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContextProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextProperties> for ::windows::core::IUnknown {
    fn from(value: &IContextProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IContextProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextProperties {}
impl ::core::fmt::Debug for IContextProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextProperties {
    type Vtable = IContextProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd396da85_bf8f_11d1_bbae_00c04fc2fa5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextProperties_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveProperty: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IContextSecurityPerimeter(::windows::core::IUnknown);
impl IContextSecurityPerimeter {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPerimeterFlag(&self, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPerimeterFlag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflag)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPerimeterFlag<'a, P0>(&self, fflag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPerimeterFlag)(::windows::core::Interface::as_raw(self), fflag.into()).ok()
    }
}
impl ::core::convert::From<IContextSecurityPerimeter> for ::windows::core::IUnknown {
    fn from(value: IContextSecurityPerimeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContextSecurityPerimeter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContextSecurityPerimeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextSecurityPerimeter> for ::windows::core::IUnknown {
    fn from(value: &IContextSecurityPerimeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IContextSecurityPerimeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextSecurityPerimeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextSecurityPerimeter {}
impl ::core::fmt::Debug for IContextSecurityPerimeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextSecurityPerimeter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextSecurityPerimeter {
    type Vtable = IContextSecurityPerimeter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7549a29_a7c4_42e1_8dc1_7e3d748dc24a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextSecurityPerimeter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPerimeterFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPerimeterFlag: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerimeterFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerimeterFlag: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IContextState(::windows::core::IUnknown);
impl IContextState {
    pub unsafe fn SetDeactivateOnReturn(&self, bdeactivate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeactivateOnReturn)(::windows::core::Interface::as_raw(self), bdeactivate).ok()
    }
    pub unsafe fn GetDeactivateOnReturn(&self, pbdeactivate: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeactivateOnReturn)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbdeactivate)).ok()
    }
    pub unsafe fn SetMyTransactionVote(&self, txvote: TransactionVote) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMyTransactionVote)(::windows::core::Interface::as_raw(self), txvote).ok()
    }
    pub unsafe fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMyTransactionVote)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptxvote)).ok()
    }
}
impl ::core::convert::From<IContextState> for ::windows::core::IUnknown {
    fn from(value: IContextState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IContextState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IContextState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextState> for ::windows::core::IUnknown {
    fn from(value: &IContextState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IContextState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextState {}
impl ::core::fmt::Debug for IContextState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextState {
    type Vtable = IContextState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c05e54b_a42a_11d2_afc4_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextState_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetDeactivateOnReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdeactivate: i16) -> ::windows::core::HRESULT,
    pub GetDeactivateOnReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdeactivate: *mut i16) -> ::windows::core::HRESULT,
    pub SetMyTransactionVote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows::core::HRESULT,
    pub GetMyTransactionVote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICreateWithLocalTransaction(::windows::core::IUnknown);
impl ICreateWithLocalTransaction {
    pub unsafe fn CreateInstanceWithSysTx<'a, P0>(&self, ptransaction: P0, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CreateInstanceWithSysTx)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(pobject)).ok()
    }
}
impl ::core::convert::From<ICreateWithLocalTransaction> for ::windows::core::IUnknown {
    fn from(value: ICreateWithLocalTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICreateWithLocalTransaction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICreateWithLocalTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateWithLocalTransaction> for ::windows::core::IUnknown {
    fn from(value: &ICreateWithLocalTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICreateWithLocalTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateWithLocalTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateWithLocalTransaction {}
impl ::core::fmt::Debug for ICreateWithLocalTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateWithLocalTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICreateWithLocalTransaction {
    type Vtable = ICreateWithLocalTransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x227ac7a8_8423_42ce_b7cf_03061ec9aaa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithLocalTransaction_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateInstanceWithSysTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICreateWithTipTransactionEx(::windows::core::IUnknown);
impl ICreateWithTipTransactionEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, P0, T>(&self, bstrtipurl: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), bstrtipurl.into().abi(), ::core::mem::transmute(rclsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateWithTipTransactionEx> for ::windows::core::IUnknown {
    fn from(value: ICreateWithTipTransactionEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICreateWithTipTransactionEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICreateWithTipTransactionEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateWithTipTransactionEx> for ::windows::core::IUnknown {
    fn from(value: &ICreateWithTipTransactionEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICreateWithTipTransactionEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateWithTipTransactionEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateWithTipTransactionEx {}
impl ::core::fmt::Debug for ICreateWithTipTransactionEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateWithTipTransactionEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICreateWithTipTransactionEx {
    type Vtable = ICreateWithTipTransactionEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x455acf59_5345_11d2_99cf_00c04f797bc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTipTransactionEx_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICreateWithTransactionEx(::windows::core::IUnknown);
impl ICreateWithTransactionEx {
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateInstance<'a, P0, T>(&self, ptransaction: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DistributedTransactionCoordinator::ITransaction>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), ::core::mem::transmute(rclsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateWithTransactionEx> for ::windows::core::IUnknown {
    fn from(value: ICreateWithTransactionEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICreateWithTransactionEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICreateWithTransactionEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateWithTransactionEx> for ::windows::core::IUnknown {
    fn from(value: &ICreateWithTransactionEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICreateWithTransactionEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateWithTransactionEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateWithTransactionEx {}
impl ::core::fmt::Debug for ICreateWithTransactionEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateWithTransactionEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICreateWithTransactionEx {
    type Vtable = ICreateWithTransactionEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x455acf57_5345_11d2_99cf_00c04f797bc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTransactionEx_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICrmCompensator(::windows::core::IUnknown);
impl ICrmCompensator {
    pub unsafe fn SetLogControl<'a, P0>(&self, plogcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ICrmLogControl>>,
    {
        (::windows::core::Interface::vtable(self).SetLogControl)(::windows::core::Interface::as_raw(self), plogcontrol.into().abi()).ok()
    }
    pub unsafe fn BeginPrepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPrepare)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PrepareRecord(&self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrepareRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(crmlogrec), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndPrepare(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EndPrepare)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginCommit<'a, P0>(&self, frecovery: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).BeginCommit)(::windows::core::Interface::as_raw(self), frecovery.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CommitRecord(&self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CommitRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(crmlogrec), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn EndCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginAbort<'a, P0>(&self, frecovery: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).BeginAbort)(::windows::core::Interface::as_raw(self), frecovery.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AbortRecord(&self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AbortRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(crmlogrec), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn EndAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICrmCompensator> for ::windows::core::IUnknown {
    fn from(value: ICrmCompensator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICrmCompensator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmCompensator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmCompensator> for ::windows::core::IUnknown {
    fn from(value: &ICrmCompensator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICrmCompensator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmCompensator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmCompensator {}
impl ::core::fmt::Debug for ICrmCompensator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmCompensator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICrmCompensator {
    type Vtable = ICrmCompensator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbc01830_8d3b_11d1_82ec_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetLogControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PrepareRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PrepareRecord: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndPrepare: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginCommit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CommitRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CommitRecord: usize,
    pub EndCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginAbort: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AbortRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AbortRecord: usize,
    pub EndAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICrmCompensatorVariants(::windows::core::IUnknown);
impl ICrmCompensatorVariants {
    pub unsafe fn SetLogControlVariants<'a, P0>(&self, plogcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ICrmLogControl>>,
    {
        (::windows::core::Interface::vtable(self).SetLogControlVariants)(::windows::core::Interface::as_raw(self), plogcontrol.into().abi()).ok()
    }
    pub unsafe fn BeginPrepareVariants(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPrepareVariants)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrepareRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrepareRecordVariants)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plogrecord), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn EndPrepareVariants(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EndPrepareVariants)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn BeginCommitVariants(&self, brecovery: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginCommitVariants)(::windows::core::Interface::as_raw(self), brecovery).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CommitRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CommitRecordVariants)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plogrecord), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn EndCommitVariants(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndCommitVariants)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginAbortVariants(&self, brecovery: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginAbortVariants)(::windows::core::Interface::as_raw(self), brecovery).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AbortRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AbortRecordVariants)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plogrecord), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn EndAbortVariants(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndAbortVariants)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICrmCompensatorVariants> for ::windows::core::IUnknown {
    fn from(value: ICrmCompensatorVariants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICrmCompensatorVariants> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmCompensatorVariants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmCompensatorVariants> for ::windows::core::IUnknown {
    fn from(value: &ICrmCompensatorVariants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICrmCompensatorVariants {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmCompensatorVariants {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmCompensatorVariants {}
impl ::core::fmt::Debug for ICrmCompensatorVariants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmCompensatorVariants").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICrmCompensatorVariants {
    type Vtable = ICrmCompensatorVariants_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0baf8e4_7804_11d1_82e9_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensatorVariants_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetLogControlVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginPrepareVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrepareRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrepareRecordVariants: usize,
    pub EndPrepareVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboktoprepare: *mut i16) -> ::windows::core::HRESULT,
    pub BeginCommitVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CommitRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CommitRecordVariants: usize,
    pub EndCommitVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginAbortVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AbortRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AbortRecordVariants: usize,
    pub EndAbortVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICrmFormatLogRecords(::windows::core::IUnknown);
impl ICrmFormatLogRecords {
    pub unsafe fn GetColumnCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetColumnCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetColumnHeaders(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetColumnHeaders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetColumn(&self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetColumn)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(crmlogrec), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetColumnVariants<'a, P0>(&self, logrecord: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetColumnVariants)(::windows::core::Interface::as_raw(self), logrecord.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICrmFormatLogRecords> for ::windows::core::IUnknown {
    fn from(value: ICrmFormatLogRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICrmFormatLogRecords> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmFormatLogRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmFormatLogRecords> for ::windows::core::IUnknown {
    fn from(value: &ICrmFormatLogRecords) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICrmFormatLogRecords {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmFormatLogRecords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmFormatLogRecords {}
impl ::core::fmt::Debug for ICrmFormatLogRecords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmFormatLogRecords").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICrmFormatLogRecords {
    type Vtable = ICrmFormatLogRecords_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c51d821_c98b_11d1_82fb_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmFormatLogRecords_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheaders: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnHeaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumn: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnVariants: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICrmLogControl(::windows::core::IUnknown);
impl ICrmLogControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransactionUOW(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionUOW)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RegisterCompensator<'a, P0, P1>(&self, lpcwstrprogidcompensator: P0, lpcwstrdescription: P1, lcrmregflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RegisterCompensator)(::windows::core::Interface::as_raw(self), lpcwstrprogidcompensator.into(), lpcwstrdescription.into(), lcrmregflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn WriteLogRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteLogRecordVariants)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plogrecord)).ok()
    }
    pub unsafe fn ForceLog(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ForceLog)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ForgetLogRecord(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ForgetLogRecord)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ForceTransactionToAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ForceTransactionToAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteLogRecord(&self, rgblob: &[super::Com::BLOB]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteLogRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgblob)), rgblob.len() as _).ok()
    }
}
impl ::core::convert::From<ICrmLogControl> for ::windows::core::IUnknown {
    fn from(value: ICrmLogControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICrmLogControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmLogControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmLogControl> for ::windows::core::IUnknown {
    fn from(value: &ICrmLogControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICrmLogControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmLogControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmLogControl {}
impl ::core::fmt::Debug for ICrmLogControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmLogControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICrmLogControl {
    type Vtable = ICrmLogControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e174b3_d26e_11d2_8f84_00805fc7bcd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmLogControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TransactionUOW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransactionUOW: usize,
    pub RegisterCompensator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: ::windows::core::PCWSTR, lpcwstrdescription: ::windows::core::PCWSTR, lcrmregflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub WriteLogRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    WriteLogRecordVariants: usize,
    pub ForceLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ForgetLogRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ForceTransactionToAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteLogRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteLogRecord: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICrmMonitor(::windows::core::IUnknown);
impl ICrmMonitor {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClerks(&self) -> ::windows::core::Result<ICrmMonitorClerks> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetClerks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICrmMonitorClerks>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn HoldClerk<'a, P0>(&self, index: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HoldClerk)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICrmMonitor> for ::windows::core::IUnknown {
    fn from(value: ICrmMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICrmMonitor> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmMonitor> for ::windows::core::IUnknown {
    fn from(value: &ICrmMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICrmMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmMonitor {}
impl ::core::fmt::Debug for ICrmMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICrmMonitor {
    type Vtable = ICrmMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70c8e443_c7ed_11d1_82fb_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitor_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClerks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclerks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClerks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub HoldClerk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    HoldClerk: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICrmMonitorClerks(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorClerks {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, P0>(&self, index: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ProgIdCompensator<'a, P0>(&self, index: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProgIdCompensator)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Description<'a, P0>(&self, index: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionUOW<'a, P0>(&self, index: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionUOW)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ActivityId<'a, P0>(&self, index: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ActivityId)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICrmMonitorClerks> for ::windows::core::IUnknown {
    fn from(value: ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICrmMonitorClerks> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICrmMonitorClerks> for ::windows::core::IUnknown {
    fn from(value: &ICrmMonitorClerks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICrmMonitorClerks> for super::Com::IDispatch {
    fn from(value: ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ICrmMonitorClerks> for &'a super::Com::IDispatch {
    fn from(value: &'a ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICrmMonitorClerks> for super::Com::IDispatch {
    fn from(value: &ICrmMonitorClerks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICrmMonitorClerks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICrmMonitorClerks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICrmMonitorClerks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICrmMonitorClerks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmMonitorClerks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICrmMonitorClerks {
    type Vtable = ICrmMonitorClerks_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70c8e442_c7ed_11d1_82fb_00a0c91eede9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorClerks_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ProgIdCompensator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ProgIdCompensator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Description: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionUOW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionUOW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ActivityId: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ICrmMonitorLogRecords(::windows::core::IUnknown);
impl ICrmMonitorLogRecords {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TransactionState(&self) -> ::windows::core::Result<CrmTransactionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CrmTransactionState>(result__)
    }
    pub unsafe fn StructuredRecords(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StructuredRecords)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogRecord)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pcrmlogrec)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetLogRecordVariants<'a, P0>(&self, indexnumber: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLogRecordVariants)(::windows::core::Interface::as_raw(self), indexnumber.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICrmMonitorLogRecords> for ::windows::core::IUnknown {
    fn from(value: ICrmMonitorLogRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICrmMonitorLogRecords> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICrmMonitorLogRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmMonitorLogRecords> for ::windows::core::IUnknown {
    fn from(value: &ICrmMonitorLogRecords) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICrmMonitorLogRecords {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmMonitorLogRecords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmMonitorLogRecords {}
impl ::core::fmt::Debug for ICrmMonitorLogRecords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmMonitorLogRecords").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICrmMonitorLogRecords {
    type Vtable = ICrmMonitorLogRecords_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70c8e441_c7ed_11d1_82fb_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorLogRecords_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub TransactionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows::core::HRESULT,
    pub StructuredRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetLogRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetLogRecord: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetLogRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetLogRecordVariants: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IDispenserDriver(::windows::core::IUnknown);
impl IDispenserDriver {
    pub unsafe fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateResource)(::windows::core::Interface::as_raw(self), restypid, ::core::mem::transmute(presid), ::core::mem::transmute(psecsfreebeforedestroy)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RateResource<'a, P0>(&self, restypid: usize, resid: usize, frequirestransactionenlistment: P0, prating: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RateResource)(::windows::core::Interface::as_raw(self), restypid, resid, frequirestransactionenlistment.into(), ::core::mem::transmute(prating)).ok()
    }
    pub unsafe fn EnlistResource(&self, resid: usize, transid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnlistResource)(::windows::core::Interface::as_raw(self), resid, transid).ok()
    }
    pub unsafe fn ResetResource(&self, resid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetResource)(::windows::core::Interface::as_raw(self), resid).ok()
    }
    pub unsafe fn DestroyResource(&self, resid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DestroyResource)(::windows::core::Interface::as_raw(self), resid).ok()
    }
    pub unsafe fn DestroyResourceS(&self, resid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DestroyResourceS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(resid)).ok()
    }
}
impl ::core::convert::From<IDispenserDriver> for ::windows::core::IUnknown {
    fn from(value: IDispenserDriver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDispenserDriver> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDispenserDriver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispenserDriver> for ::windows::core::IUnknown {
    fn from(value: &IDispenserDriver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDispenserDriver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispenserDriver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispenserDriver {}
impl ::core::fmt::Debug for IDispenserDriver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispenserDriver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDispenserDriver {
    type Vtable = IDispenserDriver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x208b3651_2b48_11cf_be10_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserDriver_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RateResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RateResource: usize,
    pub EnlistResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows::core::HRESULT,
    pub ResetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT,
    pub DestroyResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT,
    pub DestroyResourceS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IDispenserManager(::windows::core::IUnknown);
impl IDispenserManager {
    pub unsafe fn RegisterDispenser<'a, P0, P1>(&self, __midl__idispensermanager0000: P0, szdispensername: P1) -> ::windows::core::Result<IHolder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDispenserDriver>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RegisterDispenser)(::windows::core::Interface::as_raw(self), __midl__idispensermanager0000.into().abi(), szdispensername.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IHolder>(result__)
    }
    pub unsafe fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(__midl__idispensermanager0002), ::core::mem::transmute(__midl__idispensermanager0003)).ok()
    }
}
impl ::core::convert::From<IDispenserManager> for ::windows::core::IUnknown {
    fn from(value: IDispenserManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDispenserManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDispenserManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispenserManager> for ::windows::core::IUnknown {
    fn from(value: &IDispenserManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDispenserManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispenserManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispenserManager {}
impl ::core::fmt::Debug for IDispenserManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispenserManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDispenserManager {
    type Vtable = IDispenserManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cb31e10_2b5f_11cf_be10_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RegisterDispenser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: *mut ::core::ffi::c_void, szdispensername: ::windows::core::PCWSTR, __midl__idispensermanager0001: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IEnumNames(::windows::core::IUnknown);
impl IEnumNames {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(rgname), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNames>(result__)
    }
}
impl ::core::convert::From<IEnumNames> for ::windows::core::IUnknown {
    fn from(value: IEnumNames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumNames> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumNames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNames> for ::windows::core::IUnknown {
    fn from(value: &IEnumNames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumNames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNames {}
impl ::core::fmt::Debug for IEnumNames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNames {
    type Vtable = IEnumNames_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372af2_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNames_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEventServerTrace(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEventServerTrace {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTraceGuid<'a, P0, P1>(&self, bstrguidevent: P0, bstrguidfilter: P1, lpidfilter: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).StartTraceGuid)(::windows::core::Interface::as_raw(self), bstrguidevent.into().abi(), bstrguidfilter.into().abi(), lpidfilter).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StopTraceGuid<'a, P0, P1>(&self, bstrguidevent: P0, bstrguidfilter: P1, lpidfilter: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).StopTraceGuid)(::windows::core::Interface::as_raw(self), bstrguidevent.into().abi(), bstrguidfilter.into().abi(), lpidfilter).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumTraceGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plcntguids), ::core::mem::transmute(pbstrguidlist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventServerTrace> for ::windows::core::IUnknown {
    fn from(value: IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEventServerTrace> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventServerTrace> for ::windows::core::IUnknown {
    fn from(value: &IEventServerTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventServerTrace> for super::Com::IDispatch {
    fn from(value: IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEventServerTrace> for &'a super::Com::IDispatch {
    fn from(value: &'a IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventServerTrace> for super::Com::IDispatch {
    fn from(value: &IEventServerTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEventServerTrace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEventServerTrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEventServerTrace {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEventServerTrace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventServerTrace").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IEventServerTrace {
    type Vtable = IEventServerTrace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a9f12b8_80af_47ab_a579_35ea57725370);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventServerTrace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTraceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTraceGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StopTraceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StopTraceGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumTraceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumTraceGuid: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IGetAppTrackerData(::windows::core::IUnknown);
impl IGetAppTrackerData {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetApplicationProcesses(&self, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApplicationProcesses)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(partitionid), ::core::mem::transmute(applicationid), flags, ::core::mem::transmute(numapplicationprocesses), ::core::mem::transmute(applicationprocesses)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetApplicationProcessDetails(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApplicationProcessDetails)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), processid, flags, ::core::mem::transmute(summary), ::core::mem::transmute(statistics), ::core::mem::transmute(recycleinfo), ::core::mem::transmute(anycomponentshangmonitored)).ok()
    }
    pub unsafe fn GetApplicationsInProcess(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApplicationsInProcess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), processid, ::core::mem::transmute(partitionid), flags, ::core::mem::transmute(numapplicationsinprocess), ::core::mem::transmute(applications)).ok()
    }
    pub unsafe fn GetComponentsInProcess(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComponentsInProcess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), processid, ::core::mem::transmute(partitionid), ::core::mem::transmute(applicationid), flags, ::core::mem::transmute(numcomponentsinprocess), ::core::mem::transmute(components)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentDetails(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComponentDetails)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), processid, ::core::mem::transmute(clsid), flags, ::core::mem::transmute(summary), ::core::mem::transmute(statistics), ::core::mem::transmute(hangmonitorinfo)).ok()
    }
    pub unsafe fn GetTrackerDataAsCollectionObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTrackerDataAsCollectionObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetSuggestedPollingInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSuggestedPollingInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGetAppTrackerData> for ::windows::core::IUnknown {
    fn from(value: IGetAppTrackerData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGetAppTrackerData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGetAppTrackerData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetAppTrackerData> for ::windows::core::IUnknown {
    fn from(value: &IGetAppTrackerData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGetAppTrackerData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetAppTrackerData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetAppTrackerData {}
impl ::core::fmt::Debug for IGetAppTrackerData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetAppTrackerData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGetAppTrackerData {
    type Vtable = IGetAppTrackerData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507c3ac8_3e12_4cb0_9366_653d3e050638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetAppTrackerData_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationProcesses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationProcesses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationProcessDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationProcessDetails: usize,
    pub GetApplicationsInProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::HRESULT,
    pub GetComponentsInProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentDetails: usize,
    pub GetTrackerDataAsCollectionObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSuggestedPollingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IGetContextProperties(::windows::core::IUnknown);
impl IGetContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, P0>(&self, name: P0, pproperty: *mut super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn EnumNames(&self) -> ::windows::core::Result<IEnumNames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNames>(result__)
    }
}
impl ::core::convert::From<IGetContextProperties> for ::windows::core::IUnknown {
    fn from(value: IGetContextProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGetContextProperties> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGetContextProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetContextProperties> for ::windows::core::IUnknown {
    fn from(value: &IGetContextProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGetContextProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetContextProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetContextProperties {}
impl ::core::fmt::Debug for IGetContextProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetContextProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGetContextProperties {
    type Vtable = IGetContextProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372af4_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetContextProperties_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGetSecurityCallContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGetSecurityCallContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityCallContext(&self) -> ::windows::core::Result<ISecurityCallContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityCallContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISecurityCallContext>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGetSecurityCallContext> for ::windows::core::IUnknown {
    fn from(value: IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IGetSecurityCallContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGetSecurityCallContext> for ::windows::core::IUnknown {
    fn from(value: &IGetSecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGetSecurityCallContext> for super::Com::IDispatch {
    fn from(value: IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IGetSecurityCallContext> for &'a super::Com::IDispatch {
    fn from(value: &'a IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGetSecurityCallContext> for super::Com::IDispatch {
    fn from(value: &IGetSecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGetSecurityCallContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGetSecurityCallContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGetSecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGetSecurityCallContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetSecurityCallContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGetSecurityCallContext {
    type Vtable = IGetSecurityCallContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafc823f_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGetSecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityCallContext: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IHolder(::windows::core::IUnknown);
impl IHolder {
    pub unsafe fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocResource)(::windows::core::Interface::as_raw(self), __midl__iholder0000, ::core::mem::transmute(__midl__iholder0001)).ok()
    }
    pub unsafe fn FreeResource(&self, __midl__iholder0002: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeResource)(::windows::core::Interface::as_raw(self), __midl__iholder0002).ok()
    }
    pub unsafe fn TrackResource(&self, __midl__iholder0003: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TrackResource)(::windows::core::Interface::as_raw(self), __midl__iholder0003).ok()
    }
    pub unsafe fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TrackResourceS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0004)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UntrackResource<'a, P0>(&self, __midl__iholder0005: usize, __midl__iholder0006: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).UntrackResource)(::windows::core::Interface::as_raw(self), __midl__iholder0005, __midl__iholder0006.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UntrackResourceS<'a, P0>(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).UntrackResourceS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0007), __midl__iholder0008.into()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestDestroyResource)(::windows::core::Interface::as_raw(self), __midl__iholder0009).ok()
    }
}
impl ::core::convert::From<IHolder> for ::windows::core::IUnknown {
    fn from(value: IHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolder> for ::windows::core::IUnknown {
    fn from(value: &IHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolder {}
impl ::core::fmt::Debug for IHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHolder {
    type Vtable = IHolder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6a1850_2b45_11cf_be10_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AllocResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::HRESULT,
    pub FreeResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows::core::HRESULT,
    pub TrackResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows::core::HRESULT,
    pub TrackResourceS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UntrackResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UntrackResource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UntrackResourceS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UntrackResourceS: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestDestroyResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ILBEvents(::windows::core::IUnknown);
impl ILBEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetUp<'a, P0, P1>(&self, bstrservername: P0, bstrclsideng: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).TargetUp)(::windows::core::Interface::as_raw(self), bstrservername.into().abi(), bstrclsideng.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetDown<'a, P0, P1>(&self, bstrservername: P0, bstrclsideng: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).TargetDown)(::windows::core::Interface::as_raw(self), bstrservername.into().abi(), bstrclsideng.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EngineDefined<'a, P0, P1>(&self, bstrpropname: P0, varpropvalue: *const super::Com::VARIANT, bstrclsideng: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).EngineDefined)(::windows::core::Interface::as_raw(self), bstrpropname.into().abi(), ::core::mem::transmute(varpropvalue), bstrclsideng.into().abi()).ok()
    }
}
impl ::core::convert::From<ILBEvents> for ::windows::core::IUnknown {
    fn from(value: ILBEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILBEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILBEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILBEvents> for ::windows::core::IUnknown {
    fn from(value: &ILBEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILBEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILBEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILBEvents {}
impl ::core::fmt::Debug for ILBEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILBEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILBEvents {
    type Vtable = ILBEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683130b4_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILBEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetDown: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EngineDefined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EngineDefined: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IMTSActivity(::windows::core::IUnknown);
impl IMTSActivity {
    pub unsafe fn SynchronousCall<'a, P0>(&self, pcall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMTSCall>>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCall)(::windows::core::Interface::as_raw(self), pcall.into().abi()).ok()
    }
    pub unsafe fn AsyncCall<'a, P0>(&self, pcall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMTSCall>>,
    {
        (::windows::core::Interface::vtable(self).AsyncCall)(::windows::core::Interface::as_raw(self), pcall.into().abi()).ok()
    }
    pub unsafe fn Reserved1(&self) {
        (::windows::core::Interface::vtable(self).Reserved1)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn BindToCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindToCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnbindFromThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnbindFromThread)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMTSActivity> for ::windows::core::IUnknown {
    fn from(value: IMTSActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMTSActivity> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMTSActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMTSActivity> for ::windows::core::IUnknown {
    fn from(value: &IMTSActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMTSActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMTSActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMTSActivity {}
impl ::core::fmt::Debug for IMTSActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMTSActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMTSActivity {
    type Vtable = IMTSActivity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372af0_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSActivity_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SynchronousCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub BindToCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IMTSCall(::windows::core::IUnknown);
impl IMTSCall {
    pub unsafe fn OnCall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCall)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMTSCall> for ::windows::core::IUnknown {
    fn from(value: IMTSCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMTSCall> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMTSCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMTSCall> for ::windows::core::IUnknown {
    fn from(value: &IMTSCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMTSCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMTSCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMTSCall {}
impl ::core::fmt::Debug for IMTSCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMTSCall").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMTSCall {
    type Vtable = IMTSCall_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372aef_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSCall_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMTSLocator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMTSLocator {
    pub unsafe fn GetEventDispatcher(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEventDispatcher)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMTSLocator> for ::windows::core::IUnknown {
    fn from(value: IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMTSLocator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMTSLocator> for ::windows::core::IUnknown {
    fn from(value: &IMTSLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMTSLocator> for super::Com::IDispatch {
    fn from(value: IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMTSLocator> for &'a super::Com::IDispatch {
    fn from(value: &'a IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMTSLocator> for super::Com::IDispatch {
    fn from(value: &IMTSLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMTSLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMTSLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMTSLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMTSLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMTSLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMTSLocator {
    type Vtable = IMTSLocator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd19b8bfd_7f88_11d0_b16e_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMTSLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetEventDispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IManagedActivationEvents(::windows::core::IUnknown);
impl IManagedActivationEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateManagedStub<'a, P0, P1>(&self, pinfo: P0, fdist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IManagedObjectInfo>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CreateManagedStub)(::windows::core::Interface::as_raw(self), pinfo.into().abi(), fdist.into()).ok()
    }
    pub unsafe fn DestroyManagedStub<'a, P0>(&self, pinfo: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IManagedObjectInfo>>,
    {
        (::windows::core::Interface::vtable(self).DestroyManagedStub)(::windows::core::Interface::as_raw(self), pinfo.into().abi()).ok()
    }
}
impl ::core::convert::From<IManagedActivationEvents> for ::windows::core::IUnknown {
    fn from(value: IManagedActivationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IManagedActivationEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IManagedActivationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedActivationEvents> for ::windows::core::IUnknown {
    fn from(value: &IManagedActivationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IManagedActivationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedActivationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedActivationEvents {}
impl ::core::fmt::Debug for IManagedActivationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedActivationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IManagedActivationEvents {
    type Vtable = IManagedActivationEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5f325af_572f_46da_b8ab_827c3d95d99e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedActivationEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateManagedStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateManagedStub: usize,
    pub DestroyManagedStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IManagedObjectInfo(::windows::core::IUnknown);
impl IManagedObjectInfo {
    pub unsafe fn GetIUnknown(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetIUnknown)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetIObjectControl(&self) -> ::windows::core::Result<IObjectControl> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetIObjectControl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IObjectControl>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInPool<'a, P0, P1>(&self, binpool: P0, ppooledobj: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IManagedPooledObj>>,
    {
        (::windows::core::Interface::vtable(self).SetInPool)(::windows::core::Interface::as_raw(self), binpool.into(), ppooledobj.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWrapperStrength<'a, P0>(&self, bstrong: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetWrapperStrength)(::windows::core::Interface::as_raw(self), bstrong.into()).ok()
    }
}
impl ::core::convert::From<IManagedObjectInfo> for ::windows::core::IUnknown {
    fn from(value: IManagedObjectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IManagedObjectInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IManagedObjectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedObjectInfo> for ::windows::core::IUnknown {
    fn from(value: &IManagedObjectInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IManagedObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedObjectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedObjectInfo {}
impl ::core::fmt::Debug for IManagedObjectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedObjectInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IManagedObjectInfo {
    type Vtable = IManagedObjectInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1427c51a_4584_49d8_90a0_c50d8086cbe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedObjectInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetIUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIObjectControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctrl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInPool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWrapperStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWrapperStrength: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IManagedPoolAction(::windows::core::IUnknown);
impl IManagedPoolAction {
    pub unsafe fn LastRelease(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LastRelease)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IManagedPoolAction> for ::windows::core::IUnknown {
    fn from(value: IManagedPoolAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IManagedPoolAction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IManagedPoolAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedPoolAction> for ::windows::core::IUnknown {
    fn from(value: &IManagedPoolAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IManagedPoolAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedPoolAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedPoolAction {}
impl ::core::fmt::Debug for IManagedPoolAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedPoolAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IManagedPoolAction {
    type Vtable = IManagedPoolAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda91b74e_5388_4783_949d_c1cd5fb00506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPoolAction_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub LastRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IManagedPooledObj(::windows::core::IUnknown);
impl IManagedPooledObj {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHeld<'a, P0>(&self, m_bheld: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetHeld)(::windows::core::Interface::as_raw(self), m_bheld.into()).ok()
    }
}
impl ::core::convert::From<IManagedPooledObj> for ::windows::core::IUnknown {
    fn from(value: IManagedPooledObj) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IManagedPooledObj> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IManagedPooledObj) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedPooledObj> for ::windows::core::IUnknown {
    fn from(value: &IManagedPooledObj) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IManagedPooledObj {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedPooledObj {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedPooledObj {}
impl ::core::fmt::Debug for IManagedPooledObj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedPooledObj").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IManagedPooledObj {
    type Vtable = IManagedPooledObj_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5da4bea_1b42_4437_8926_b6a38860a770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPooledObj_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHeld: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHeld: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMessageMover(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMessageMover {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourcePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SourcePath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourcePath<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSourcePath)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDestPath<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDestPath)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn CommitBatchSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CommitBatchSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetCommitBatchSize(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCommitBatchSize)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MoveMessages(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MoveMessages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMessageMover> for ::windows::core::IUnknown {
    fn from(value: IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMessageMover> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMessageMover> for ::windows::core::IUnknown {
    fn from(value: &IMessageMover) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMessageMover> for super::Com::IDispatch {
    fn from(value: IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMessageMover> for &'a super::Com::IDispatch {
    fn from(value: &'a IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMessageMover> for super::Com::IDispatch {
    fn from(value: &IMessageMover) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMessageMover {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMessageMover {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMessageMover {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMessageMover {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageMover").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMessageMover {
    type Vtable = IMessageMover_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x588a085a_b795_11d1_8054_00c04fc340ee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMessageMover_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SourcePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourcePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourcePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourcePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDestPath: usize,
    pub CommitBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetCommitBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub MoveMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMtsEventInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventInfo {
    pub unsafe fn Names(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Names)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EventID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Value<'a, P0>(&self, skey: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Value)(::windows::core::Interface::as_raw(self), skey.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEventInfo> for ::windows::core::IUnknown {
    fn from(value: IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMtsEventInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEventInfo> for ::windows::core::IUnknown {
    fn from(value: &IMtsEventInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEventInfo> for super::Com::IDispatch {
    fn from(value: IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMtsEventInfo> for &'a super::Com::IDispatch {
    fn from(value: &'a IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEventInfo> for super::Com::IDispatch {
    fn from(value: &IMtsEventInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMtsEventInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMtsEventInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMtsEventInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMtsEventInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMtsEventInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMtsEventInfo {
    type Vtable = IMtsEventInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd56c3dc1_8482_11d0_b170_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEventInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Names: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sguideventid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventID: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Value: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMtsEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PackageName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PackageName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PackageGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PackageGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PostEvent(&self, vevent: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PostEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vevent)).ok()
    }
    pub unsafe fn FireEvents(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FireEvents)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetProcessID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProcessID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEvents> for ::windows::core::IUnknown {
    fn from(value: IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMtsEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEvents> for ::windows::core::IUnknown {
    fn from(value: &IMtsEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEvents> for super::Com::IDispatch {
    fn from(value: IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMtsEvents> for &'a super::Com::IDispatch {
    fn from(value: &'a IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEvents> for super::Com::IDispatch {
    fn from(value: &IMtsEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMtsEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMtsEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMtsEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMtsEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMtsEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMtsEvents {
    type Vtable = IMtsEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbacedf4d_74ab_11d0_b162_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PackageName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PackageGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PackageGuid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PostEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PostEvent: usize,
    pub FireEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub GetProcessID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMtsGrp(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMtsGrp {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsGrp> for ::windows::core::IUnknown {
    fn from(value: IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMtsGrp> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsGrp> for ::windows::core::IUnknown {
    fn from(value: &IMtsGrp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsGrp> for super::Com::IDispatch {
    fn from(value: IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMtsGrp> for &'a super::Com::IDispatch {
    fn from(value: &'a IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsGrp> for super::Com::IDispatch {
    fn from(value: &IMtsGrp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMtsGrp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMtsGrp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMtsGrp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMtsGrp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMtsGrp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMtsGrp {
    type Vtable = IMtsGrp_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b2e958c_0393_11d1_b1ab_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsGrp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjPool(::windows::core::IUnknown);
impl IObjPool {
    pub unsafe fn Reserved1(&self) {
        (::windows::core::Interface::vtable(self).Reserved1)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved2(&self) {
        (::windows::core::Interface::vtable(self).Reserved2)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved3(&self) {
        (::windows::core::Interface::vtable(self).Reserved3)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved4(&self) {
        (::windows::core::Interface::vtable(self).Reserved4)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn PutEndTx<'a, P0>(&self, pobj: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).PutEndTx)(::windows::core::Interface::as_raw(self), pobj.into().abi())
    }
    pub unsafe fn Reserved5(&self) {
        (::windows::core::Interface::vtable(self).Reserved5)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved6(&self) {
        (::windows::core::Interface::vtable(self).Reserved6)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IObjPool> for ::windows::core::IUnknown {
    fn from(value: IObjPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjPool> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjPool> for ::windows::core::IUnknown {
    fn from(value: &IObjPool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjPool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjPool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjPool {}
impl ::core::fmt::Debug for IObjPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjPool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjPool {
    type Vtable = IObjPool_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d8805a0_2ea7_11d1_b1cc_00aa00ba3258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjPool_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PutEndTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectConstruct(::windows::core::IUnknown);
impl IObjectConstruct {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct<'a, P0>(&self, pctorobj: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Construct)(::windows::core::Interface::as_raw(self), pctorobj.into().abi()).ok()
    }
}
impl ::core::convert::From<IObjectConstruct> for ::windows::core::IUnknown {
    fn from(value: IObjectConstruct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectConstruct> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectConstruct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectConstruct> for ::windows::core::IUnknown {
    fn from(value: &IObjectConstruct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectConstruct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectConstruct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectConstruct {}
impl ::core::fmt::Debug for IObjectConstruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectConstruct").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectConstruct {
    type Vtable = IObjectConstruct_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41c4f8b3_7439_11d2_98cb_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstruct_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctorobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IObjectConstructString(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructString {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConstructString(&self, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConstructString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IObjectConstructString> for ::windows::core::IUnknown {
    fn from(value: IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IObjectConstructString> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IObjectConstructString> for ::windows::core::IUnknown {
    fn from(value: &IObjectConstructString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IObjectConstructString> for super::Com::IDispatch {
    fn from(value: IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IObjectConstructString> for &'a super::Com::IDispatch {
    fn from(value: &'a IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IObjectConstructString> for super::Com::IDispatch {
    fn from(value: &IObjectConstructString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IObjectConstructString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IObjectConstructString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IObjectConstructString {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IObjectConstructString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectConstructString").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IObjectConstructString {
    type Vtable = IObjectConstructString_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41c4f8b2_7439_11d2_98cb_00c04f8ee1c4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstructString_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConstructString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConstructString: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectContext(::windows::core::IUnknown);
impl IObjectContext {
    pub unsafe fn CreateInstance(&self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn SetComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsInTransaction)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSecurityEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsSecurityEnabled)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCallerInRole<'a, P0>(&self, bstrrole: P0, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).IsCallerInRole)(::windows::core::Interface::as_raw(self), bstrrole.into().abi(), ::core::mem::transmute(pfisinrole)).ok()
    }
}
impl ::core::convert::From<IObjectContext> for ::windows::core::IUnknown {
    fn from(value: IObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContext> for ::windows::core::IUnknown {
    fn from(value: &IObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContext {}
impl ::core::fmt::Debug for IObjectContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectContext {
    type Vtable = IObjectContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372ae0_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContext_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInTransaction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSecurityEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCallerInRole: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectContextActivity(::windows::core::IUnknown);
impl IObjectContextActivity {
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActivityId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
}
impl ::core::convert::From<IObjectContextActivity> for ::windows::core::IUnknown {
    fn from(value: IObjectContextActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectContextActivity> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectContextActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextActivity> for ::windows::core::IUnknown {
    fn from(value: &IObjectContextActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectContextActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextActivity {}
impl ::core::fmt::Debug for IObjectContextActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectContextActivity {
    type Vtable = IObjectContextActivity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372afc_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextActivity_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectContextInfo(::windows::core::IUnknown);
impl IObjectContextInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsInTransaction)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetTransactionId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActivityId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetContextId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContextId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
}
impl ::core::convert::From<IObjectContextInfo> for ::windows::core::IUnknown {
    fn from(value: IObjectContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectContextInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo> for ::windows::core::IUnknown {
    fn from(value: &IObjectContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectContextInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextInfo {}
impl ::core::fmt::Debug for IObjectContextInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectContextInfo {
    type Vtable = IObjectContextInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75b52ddb_e8ed_11d1_93ad_00aa00ba3258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInTransaction: usize,
    pub GetTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectContextInfo2(::windows::core::IUnknown);
impl IObjectContextInfo2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsInTransaction)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetTransactionId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetActivityId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetContextId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetContextId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetPartitionId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPartitionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetApplicationId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApplicationId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetApplicationInstanceId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApplicationInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
}
impl ::core::convert::From<IObjectContextInfo2> for ::windows::core::IUnknown {
    fn from(value: IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectContextInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo2> for ::windows::core::IUnknown {
    fn from(value: &IObjectContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IObjectContextInfo2> for IObjectContextInfo {
    fn from(value: IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectContextInfo2> for &'a IObjectContextInfo {
    fn from(value: &'a IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo2> for IObjectContextInfo {
    fn from(value: &IObjectContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectContextInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextInfo2 {}
impl ::core::fmt::Debug for IObjectContextInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectContextInfo2 {
    type Vtable = IObjectContextInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594be71a_4bc4_438b_9197_cfd176248b09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo2_Vtbl {
    pub base__: IObjectContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectContextTip(::windows::core::IUnknown);
impl IObjectContextTip {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTipUrl(&self, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTipUrl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptipurl)).ok()
    }
}
impl ::core::convert::From<IObjectContextTip> for ::windows::core::IUnknown {
    fn from(value: IObjectContextTip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectContextTip> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectContextTip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextTip> for ::windows::core::IUnknown {
    fn from(value: &IObjectContextTip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectContextTip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextTip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextTip {}
impl ::core::fmt::Debug for IObjectContextTip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextTip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectContextTip {
    type Vtable = IObjectContextTip_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92fd41ca_bad9_11d2_9a2d_00c04f797bc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextTip_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTipUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTipUrl: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IObjectControl(::windows::core::IUnknown);
impl IObjectControl {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Deactivate(&self) {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanBePooled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).CanBePooled)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IObjectControl> for ::windows::core::IUnknown {
    fn from(value: IObjectControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IObjectControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IObjectControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectControl> for ::windows::core::IUnknown {
    fn from(value: &IObjectControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IObjectControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectControl {}
impl ::core::fmt::Debug for IObjectControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectControl {
    type Vtable = IObjectControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372aec_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub CanBePooled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanBePooled: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IPlaybackControl(::windows::core::IUnknown);
impl IPlaybackControl {
    pub unsafe fn FinalClientRetry(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinalClientRetry)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalServerRetry(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinalServerRetry)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPlaybackControl> for ::windows::core::IUnknown {
    fn from(value: IPlaybackControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPlaybackControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPlaybackControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlaybackControl> for ::windows::core::IUnknown {
    fn from(value: &IPlaybackControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPlaybackControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlaybackControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlaybackControl {}
impl ::core::fmt::Debug for IPlaybackControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlaybackControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPlaybackControl {
    type Vtable = IPlaybackControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372afd_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub FinalClientRetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FinalServerRetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPoolManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPoolManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShutdownPool<'a, P0>(&self, clsidorprogid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ShutdownPool)(::windows::core::Interface::as_raw(self), clsidorprogid.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPoolManager> for ::windows::core::IUnknown {
    fn from(value: IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPoolManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPoolManager> for ::windows::core::IUnknown {
    fn from(value: &IPoolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPoolManager> for super::Com::IDispatch {
    fn from(value: IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPoolManager> for &'a super::Com::IDispatch {
    fn from(value: &'a IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPoolManager> for super::Com::IDispatch {
    fn from(value: &IPoolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPoolManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPoolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPoolManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPoolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPoolManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPoolManager {
    type Vtable = IPoolManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a469861_5a91_43a0_99b6_d5e179bb0631);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPoolManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ShutdownPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShutdownPool: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IProcessInitializer(::windows::core::IUnknown);
impl IProcessInitializer {
    pub unsafe fn Startup<'a, P0>(&self, punkprocesscontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Startup)(::windows::core::Interface::as_raw(self), punkprocesscontrol.into().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IProcessInitializer> for ::windows::core::IUnknown {
    fn from(value: IProcessInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProcessInitializer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProcessInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessInitializer> for ::windows::core::IUnknown {
    fn from(value: &IProcessInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IProcessInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitializer {}
impl ::core::fmt::Debug for IProcessInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitializer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessInitializer {
    type Vtable = IProcessInitializer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1113f52d_dc7f_4943_aed6_88d04027e32a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitializer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Startup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISecurityCallContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallContext {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, name: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCallerInRole<'a, P0>(&self, bstrrole: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsCallerInRole)(::windows::core::Interface::as_raw(self), bstrrole.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsSecurityEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSecurityEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsUserInRole<'a, P0>(&self, puser: *const super::Com::VARIANT, bstrrole: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsUserInRole)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puser), bstrrole.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallContext> for ::windows::core::IUnknown {
    fn from(value: ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISecurityCallContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallContext> for ::windows::core::IUnknown {
    fn from(value: &ISecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallContext> for super::Com::IDispatch {
    fn from(value: ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISecurityCallContext> for &'a super::Com::IDispatch {
    fn from(value: &'a ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallContext> for super::Com::IDispatch {
    fn from(value: &ISecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISecurityCallContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISecurityCallContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISecurityCallContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityCallContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISecurityCallContext {
    type Vtable = ISecurityCallContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafc823e_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCallerInRole: usize,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUserInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUserInRole: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISecurityCallersColl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallersColl {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows::core::Result<ISecurityIdentityColl> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), lindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISecurityIdentityColl>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallersColl> for ::windows::core::IUnknown {
    fn from(value: ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISecurityCallersColl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallersColl> for ::windows::core::IUnknown {
    fn from(value: &ISecurityCallersColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallersColl> for super::Com::IDispatch {
    fn from(value: ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISecurityCallersColl> for &'a super::Com::IDispatch {
    fn from(value: &'a ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallersColl> for super::Com::IDispatch {
    fn from(value: &ISecurityCallersColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISecurityCallersColl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISecurityCallersColl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISecurityCallersColl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISecurityCallersColl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityCallersColl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISecurityCallersColl {
    type Vtable = ISecurityCallersColl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafc823d_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallersColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISecurityIdentityColl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityIdentityColl {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, name: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityIdentityColl> for ::windows::core::IUnknown {
    fn from(value: ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISecurityIdentityColl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityIdentityColl> for ::windows::core::IUnknown {
    fn from(value: &ISecurityIdentityColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityIdentityColl> for super::Com::IDispatch {
    fn from(value: ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISecurityIdentityColl> for &'a super::Com::IDispatch {
    fn from(value: &'a ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityIdentityColl> for super::Com::IDispatch {
    fn from(value: &ISecurityIdentityColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISecurityIdentityColl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISecurityIdentityColl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISecurityIdentityColl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISecurityIdentityColl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityIdentityColl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISecurityIdentityColl {
    type Vtable = ISecurityIdentityColl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafc823c_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityIdentityColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ISecurityProperty(::windows::core::IUnknown);
impl ISecurityProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDirectCreatorSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOriginalCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOriginalCreatorSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDirectCallerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOriginalCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOriginalCallerSID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseSID<'a, P0>(&self, psid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::PSID>,
    {
        (::windows::core::Interface::vtable(self).ReleaseSID)(::windows::core::Interface::as_raw(self), psid.into()).ok()
    }
}
impl ::core::convert::From<ISecurityProperty> for ::windows::core::IUnknown {
    fn from(value: ISecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISecurityProperty> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityProperty> for ::windows::core::IUnknown {
    fn from(value: &ISecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISecurityProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityProperty {}
impl ::core::fmt::Debug for ISecurityProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISecurityProperty {
    type Vtable = ISecurityProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372aea_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityProperty_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCreatorSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCreatorSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCreatorSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCreatorSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCallerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCallerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCallerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCallerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseSID: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ISelectCOMLBServer(::windows::core::IUnknown);
impl ISelectCOMLBServer {
    pub unsafe fn Init(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLBServer<'a, P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).GetLBServer)(::windows::core::Interface::as_raw(self), punk.into().abi()).ok()
    }
}
impl ::core::convert::From<ISelectCOMLBServer> for ::windows::core::IUnknown {
    fn from(value: ISelectCOMLBServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISelectCOMLBServer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISelectCOMLBServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectCOMLBServer> for ::windows::core::IUnknown {
    fn from(value: &ISelectCOMLBServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISelectCOMLBServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISelectCOMLBServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectCOMLBServer {}
impl ::core::fmt::Debug for ISelectCOMLBServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectCOMLBServer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISelectCOMLBServer {
    type Vtable = ISelectCOMLBServer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcf443f4_3f8a_4872_b9f0_369a796d12d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectCOMLBServer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLBServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ISendMethodEvents(::windows::core::IUnknown);
impl ISendMethodEvents {
    pub unsafe fn SendMethodCall(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMethodCall)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidentity), ::core::mem::transmute(riid), dwmeth).ok()
    }
    pub unsafe fn SendMethodReturn(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMethodReturn)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidentity), ::core::mem::transmute(riid), dwmeth, hrcall, hrserver).ok()
    }
}
impl ::core::convert::From<ISendMethodEvents> for ::windows::core::IUnknown {
    fn from(value: ISendMethodEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISendMethodEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISendMethodEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISendMethodEvents> for ::windows::core::IUnknown {
    fn from(value: &ISendMethodEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISendMethodEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISendMethodEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISendMethodEvents {}
impl ::core::fmt::Debug for ISendMethodEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISendMethodEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISendMethodEvents {
    type Vtable = ISendMethodEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2732fd59_b2b4_4d44_878c_8b8f09626008);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISendMethodEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SendMethodCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::HRESULT,
    pub SendMethodReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceActivity(::windows::core::IUnknown);
impl IServiceActivity {
    pub unsafe fn SynchronousCall<'a, P0>(&self, piservicecall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IServiceCall>>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCall)(::windows::core::Interface::as_raw(self), piservicecall.into().abi()).ok()
    }
    pub unsafe fn AsynchronousCall<'a, P0>(&self, piservicecall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IServiceCall>>,
    {
        (::windows::core::Interface::vtable(self).AsynchronousCall)(::windows::core::Interface::as_raw(self), piservicecall.into().abi()).ok()
    }
    pub unsafe fn BindToCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindToCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnbindFromThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnbindFromThread)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IServiceActivity> for ::windows::core::IUnknown {
    fn from(value: IServiceActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceActivity> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceActivity> for ::windows::core::IUnknown {
    fn from(value: &IServiceActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceActivity {}
impl ::core::fmt::Debug for IServiceActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceActivity {
    type Vtable = IServiceActivity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67532e0c_9e2f_4450_a354_035633944e17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceActivity_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SynchronousCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AsynchronousCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BindToCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceCall(::windows::core::IUnknown);
impl IServiceCall {
    pub unsafe fn OnCall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCall)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IServiceCall> for ::windows::core::IUnknown {
    fn from(value: IServiceCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceCall> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceCall> for ::windows::core::IUnknown {
    fn from(value: &IServiceCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceCall {}
impl ::core::fmt::Debug for IServiceCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceCall").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceCall {
    type Vtable = IServiceCall_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd3e2e12_42dd_40f4_a09a_95a50c58304b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceCall_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceComTIIntrinsicsConfig(::windows::core::IUnknown);
impl IServiceComTIIntrinsicsConfig {
    pub unsafe fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ComTIIntrinsicsConfig)(::windows::core::Interface::as_raw(self), comtiintrinsicsconfig).ok()
    }
}
impl ::core::convert::From<IServiceComTIIntrinsicsConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceComTIIntrinsicsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceComTIIntrinsicsConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceComTIIntrinsicsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceComTIIntrinsicsConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceComTIIntrinsicsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceComTIIntrinsicsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceComTIIntrinsicsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceComTIIntrinsicsConfig {}
impl ::core::fmt::Debug for IServiceComTIIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceComTIIntrinsicsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceComTIIntrinsicsConfig {
    type Vtable = IServiceComTIIntrinsicsConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e6831e_04e1_4ed4_9d0f_e8b168bafeaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceComTIIntrinsicsConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ComTIIntrinsicsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceIISIntrinsicsConfig(::windows::core::IUnknown);
impl IServiceIISIntrinsicsConfig {
    pub unsafe fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IISIntrinsicsConfig)(::windows::core::Interface::as_raw(self), iisintrinsicsconfig).ok()
    }
}
impl ::core::convert::From<IServiceIISIntrinsicsConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceIISIntrinsicsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceIISIntrinsicsConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceIISIntrinsicsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceIISIntrinsicsConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceIISIntrinsicsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceIISIntrinsicsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceIISIntrinsicsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceIISIntrinsicsConfig {}
impl ::core::fmt::Debug for IServiceIISIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceIISIntrinsicsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceIISIntrinsicsConfig {
    type Vtable = IServiceIISIntrinsicsConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a0cf920_d452_46f4_bc36_48118d54ea52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceIISIntrinsicsConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub IISIntrinsicsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceInheritanceConfig(::windows::core::IUnknown);
impl IServiceInheritanceConfig {
    pub unsafe fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ContainingContextTreatment)(::windows::core::Interface::as_raw(self), inheritanceconfig).ok()
    }
}
impl ::core::convert::From<IServiceInheritanceConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceInheritanceConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceInheritanceConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceInheritanceConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceInheritanceConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceInheritanceConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceInheritanceConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceInheritanceConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceInheritanceConfig {}
impl ::core::fmt::Debug for IServiceInheritanceConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceInheritanceConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceInheritanceConfig {
    type Vtable = IServiceInheritanceConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92186771_d3b4_4d77_a8ea_ee842d586f35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceInheritanceConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ContainingContextTreatment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServicePartitionConfig(::windows::core::IUnknown);
impl IServicePartitionConfig {
    pub unsafe fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PartitionConfig)(::windows::core::Interface::as_raw(self), partitionconfig).ok()
    }
    pub unsafe fn PartitionID(&self, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PartitionID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidpartitionid)).ok()
    }
}
impl ::core::convert::From<IServicePartitionConfig> for ::windows::core::IUnknown {
    fn from(value: IServicePartitionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServicePartitionConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServicePartitionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServicePartitionConfig> for ::windows::core::IUnknown {
    fn from(value: &IServicePartitionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServicePartitionConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServicePartitionConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServicePartitionConfig {}
impl ::core::fmt::Debug for IServicePartitionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServicePartitionConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServicePartitionConfig {
    type Vtable = IServicePartitionConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80182d03_5ea4_4831_ae97_55beffc2e590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePartitionConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PartitionConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows::core::HRESULT,
    pub PartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServicePool(::windows::core::IUnknown);
impl IServicePool {
    pub unsafe fn Initialize<'a, P0>(&self, ppoolconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ppoolconfig.into().abi()).ok()
    }
    pub unsafe fn GetObject(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IServicePool> for ::windows::core::IUnknown {
    fn from(value: IServicePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServicePool> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServicePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServicePool> for ::windows::core::IUnknown {
    fn from(value: &IServicePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServicePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServicePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServicePool {}
impl ::core::fmt::Debug for IServicePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServicePool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServicePool {
    type Vtable = IServicePool_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb302df81_ea45_451e_99a2_09f9fd1b1e13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePool_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServicePoolConfig(::windows::core::IUnknown);
impl IServicePoolConfig {
    pub unsafe fn SetMaxPoolSize(&self, dwmaxpool: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxPoolSize)(::windows::core::Interface::as_raw(self), dwmaxpool).ok()
    }
    pub unsafe fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MaxPoolSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwmaxpool)).ok()
    }
    pub unsafe fn SetMinPoolSize(&self, dwminpool: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinPoolSize)(::windows::core::Interface::as_raw(self), dwminpool).ok()
    }
    pub unsafe fn MinPoolSize(&self, pdwminpool: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MinPoolSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwminpool)).ok()
    }
    pub unsafe fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCreationTimeout)(::windows::core::Interface::as_raw(self), dwcreationtimeout).ok()
    }
    pub unsafe fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreationTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcreationtimeout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTransactionAffinity<'a, P0>(&self, ftxaffinity: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetTransactionAffinity)(::windows::core::Interface::as_raw(self), ftxaffinity.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransactionAffinity(&self, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransactionAffinity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pftxaffinity)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassFactory<'a, P0>(&self, pfactory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IClassFactory>>,
    {
        (::windows::core::Interface::vtable(self).SetClassFactory)(::windows::core::Interface::as_raw(self), pfactory.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassFactory(&self) -> ::windows::core::Result<super::Com::IClassFactory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ClassFactory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IClassFactory>(result__)
    }
}
impl ::core::convert::From<IServicePoolConfig> for ::windows::core::IUnknown {
    fn from(value: IServicePoolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServicePoolConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServicePoolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServicePoolConfig> for ::windows::core::IUnknown {
    fn from(value: &IServicePoolConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServicePoolConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServicePoolConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServicePoolConfig {}
impl ::core::fmt::Debug for IServicePoolConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServicePoolConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServicePoolConfig {
    type Vtable = IServicePoolConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9690656_5bca_470c_8451_250c1f43a33e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePoolConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetMaxPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows::core::HRESULT,
    pub MaxPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows::core::HRESULT,
    pub SetMinPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows::core::HRESULT,
    pub MinPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows::core::HRESULT,
    pub SetCreationTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows::core::HRESULT,
    pub CreationTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTransactionAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTransactionAffinity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransactionAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransactionAffinity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfactory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassFactory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassFactory: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceSxsConfig(::windows::core::IUnknown);
impl IServiceSxsConfig {
    pub unsafe fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SxsConfig)(::windows::core::Interface::as_raw(self), scsconfig).ok()
    }
    pub unsafe fn SxsName<'a, P0>(&self, szsxsname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SxsName)(::windows::core::Interface::as_raw(self), szsxsname.into()).ok()
    }
    pub unsafe fn SxsDirectory<'a, P0>(&self, szsxsdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SxsDirectory)(::windows::core::Interface::as_raw(self), szsxsdirectory.into()).ok()
    }
}
impl ::core::convert::From<IServiceSxsConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceSxsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceSxsConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceSxsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSxsConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceSxsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceSxsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceSxsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceSxsConfig {}
impl ::core::fmt::Debug for IServiceSxsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceSxsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceSxsConfig {
    type Vtable = IServiceSxsConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7cd7379_f3f2_4634_811b_703281d73e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSxsConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SxsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows::core::HRESULT,
    pub SxsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szsxsname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SxsDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szsxsdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceSynchronizationConfig(::windows::core::IUnknown);
impl IServiceSynchronizationConfig {
    pub unsafe fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConfigureSynchronization)(::windows::core::Interface::as_raw(self), synchconfig).ok()
    }
}
impl ::core::convert::From<IServiceSynchronizationConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceSynchronizationConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceSynchronizationConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceSynchronizationConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSynchronizationConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceSynchronizationConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceSynchronizationConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceSynchronizationConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceSynchronizationConfig {}
impl ::core::fmt::Debug for IServiceSynchronizationConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceSynchronizationConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceSynchronizationConfig {
    type Vtable = IServiceSynchronizationConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd880e81_6dce_4c58_af83_a208846c0030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSynchronizationConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ConfigureSynchronization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceSysTxnConfig(::windows::core::IUnknown);
impl IServiceSysTxnConfig {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ConfigureTransaction)(::windows::core::Interface::as_raw(self), transactionconfig).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.IsolationLevel)(::windows::core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.TransactionTimeout)(::windows::core::Interface::as_raw(self), ultimeoutsec).ok()
    }
    pub unsafe fn BringYourOwnTransaction<'a, P0>(&self, sztipurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.BringYourOwnTransaction)(::windows::core::Interface::as_raw(self), sztipurl.into()).ok()
    }
    pub unsafe fn NewTransactionDescription<'a, P0>(&self, sztxdesc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.NewTransactionDescription)(::windows::core::Interface::as_raw(self), sztxdesc.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn ConfigureBYOT<'a, P0>(&self, pitxbyot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DistributedTransactionCoordinator::ITransaction>>,
    {
        (::windows::core::Interface::vtable(self).base__.ConfigureBYOT)(::windows::core::Interface::as_raw(self), pitxbyot.into().abi()).ok()
    }
    pub unsafe fn ConfigureBYOTSysTxn<'a, P0>(&self, ptxproxy: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransactionProxy>>,
    {
        (::windows::core::Interface::vtable(self).ConfigureBYOTSysTxn)(::windows::core::Interface::as_raw(self), ptxproxy.into().abi()).ok()
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceSysTxnConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for IServiceTransactionConfigBase {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceSysTxnConfig> for &'a IServiceTransactionConfigBase {
    fn from(value: &'a IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for IServiceTransactionConfigBase {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for IServiceTransactionConfig {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceSysTxnConfig> for &'a IServiceTransactionConfig {
    fn from(value: &'a IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for IServiceTransactionConfig {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceSysTxnConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceSysTxnConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceSysTxnConfig {}
impl ::core::fmt::Debug for IServiceSysTxnConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceSysTxnConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceSysTxnConfig {
    type Vtable = IServiceSysTxnConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33caf1a1_fcb8_472b_b45e_967448ded6d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSysTxnConfig_Vtbl {
    pub base__: IServiceTransactionConfig_Vtbl,
    pub ConfigureBYOTSysTxn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptxproxy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceThreadPoolConfig(::windows::core::IUnknown);
impl IServiceThreadPoolConfig {
    pub unsafe fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectThreadPool)(::windows::core::Interface::as_raw(self), threadpool).ok()
    }
    pub unsafe fn SetBindingInfo(&self, binding: CSC_Binding) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBindingInfo)(::windows::core::Interface::as_raw(self), binding).ok()
    }
}
impl ::core::convert::From<IServiceThreadPoolConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceThreadPoolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceThreadPoolConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceThreadPoolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceThreadPoolConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceThreadPoolConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceThreadPoolConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceThreadPoolConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceThreadPoolConfig {}
impl ::core::fmt::Debug for IServiceThreadPoolConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceThreadPoolConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceThreadPoolConfig {
    type Vtable = IServiceThreadPoolConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x186d89bc_f277_4bcc_80d5_4df7b836ef4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceThreadPoolConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SelectThreadPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows::core::HRESULT,
    pub SetBindingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceTrackerConfig(::windows::core::IUnknown);
impl IServiceTrackerConfig {
    pub unsafe fn TrackerConfig<'a, P0, P1>(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: P0, sztrackerctxname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).TrackerConfig)(::windows::core::Interface::as_raw(self), trackerconfig, sztrackerappname.into(), sztrackerctxname.into()).ok()
    }
}
impl ::core::convert::From<IServiceTrackerConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceTrackerConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceTrackerConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceTrackerConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTrackerConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceTrackerConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceTrackerConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceTrackerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceTrackerConfig {}
impl ::core::fmt::Debug for IServiceTrackerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceTrackerConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceTrackerConfig {
    type Vtable = IServiceTrackerConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c3a3e1d_0ba6_4036_b76f_d0404db816c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTrackerConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub TrackerConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: ::windows::core::PCWSTR, sztrackerctxname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceTransactionConfig(::windows::core::IUnknown);
impl IServiceTransactionConfig {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ConfigureTransaction)(::windows::core::Interface::as_raw(self), transactionconfig).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsolationLevel)(::windows::core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TransactionTimeout)(::windows::core::Interface::as_raw(self), ultimeoutsec).ok()
    }
    pub unsafe fn BringYourOwnTransaction<'a, P0>(&self, sztipurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.BringYourOwnTransaction)(::windows::core::Interface::as_raw(self), sztipurl.into()).ok()
    }
    pub unsafe fn NewTransactionDescription<'a, P0>(&self, sztxdesc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.NewTransactionDescription)(::windows::core::Interface::as_raw(self), sztxdesc.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn ConfigureBYOT<'a, P0>(&self, pitxbyot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DistributedTransactionCoordinator::ITransaction>>,
    {
        (::windows::core::Interface::vtable(self).ConfigureBYOT)(::windows::core::Interface::as_raw(self), pitxbyot.into().abi()).ok()
    }
}
impl ::core::convert::From<IServiceTransactionConfig> for ::windows::core::IUnknown {
    fn from(value: IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceTransactionConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfig> for ::windows::core::IUnknown {
    fn from(value: &IServiceTransactionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IServiceTransactionConfig> for IServiceTransactionConfigBase {
    fn from(value: IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceTransactionConfig> for &'a IServiceTransactionConfigBase {
    fn from(value: &'a IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfig> for IServiceTransactionConfigBase {
    fn from(value: &IServiceTransactionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceTransactionConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceTransactionConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceTransactionConfig {}
impl ::core::fmt::Debug for IServiceTransactionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceTransactionConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceTransactionConfig {
    type Vtable = IServiceTransactionConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59f4c2a3_d3d7_4a31_b6e4_6ab3177c50b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfig_Vtbl {
    pub base__: IServiceTransactionConfigBase_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub ConfigureBYOT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitxbyot: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    ConfigureBYOT: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IServiceTransactionConfigBase(::windows::core::IUnknown);
impl IServiceTransactionConfigBase {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConfigureTransaction)(::windows::core::Interface::as_raw(self), transactionconfig).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsolationLevel)(::windows::core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransactionTimeout)(::windows::core::Interface::as_raw(self), ultimeoutsec).ok()
    }
    pub unsafe fn BringYourOwnTransaction<'a, P0>(&self, sztipurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).BringYourOwnTransaction)(::windows::core::Interface::as_raw(self), sztipurl.into()).ok()
    }
    pub unsafe fn NewTransactionDescription<'a, P0>(&self, sztxdesc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).NewTransactionDescription)(::windows::core::Interface::as_raw(self), sztxdesc.into()).ok()
    }
}
impl ::core::convert::From<IServiceTransactionConfigBase> for ::windows::core::IUnknown {
    fn from(value: IServiceTransactionConfigBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IServiceTransactionConfigBase> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IServiceTransactionConfigBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfigBase> for ::windows::core::IUnknown {
    fn from(value: &IServiceTransactionConfigBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IServiceTransactionConfigBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceTransactionConfigBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceTransactionConfigBase {}
impl ::core::fmt::Debug for IServiceTransactionConfigBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceTransactionConfigBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceTransactionConfigBase {
    type Vtable = IServiceTransactionConfigBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x772b3fbe_6ffd_42fb_b5f8_8f9b260f3810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfigBase_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ConfigureTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows::core::HRESULT,
    pub IsolationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows::core::HRESULT,
    pub BringYourOwnTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztipurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub NewTransactionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztxdesc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISharedProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISharedProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedProperty> for ::windows::core::IUnknown {
    fn from(value: ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISharedProperty> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedProperty> for ::windows::core::IUnknown {
    fn from(value: &ISharedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedProperty> for super::Com::IDispatch {
    fn from(value: ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISharedProperty> for &'a super::Com::IDispatch {
    fn from(value: &'a ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedProperty> for super::Com::IDispatch {
    fn from(value: &ISharedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISharedProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISharedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISharedProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISharedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISharedProperty {
    type Vtable = ISharedProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a005c01_a5de_11cf_9e66_00aa00a3f464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISharedPropertyGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroup {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyByPosition(&self, index: i32, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreatePropertyByPosition)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(fexists), ::core::mem::transmute(ppprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_PropertyByPosition(&self, index: i32) -> ::windows::core::Result<ISharedProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_PropertyByPosition)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISharedProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateProperty<'a, P0>(&self, name: P0, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).CreateProperty)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(fexists), ::core::mem::transmute(ppprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_Property<'a, P0>(&self, name: P0) -> ::windows::core::Result<ISharedProperty>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Property)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISharedProperty>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroup> for ::windows::core::IUnknown {
    fn from(value: ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISharedPropertyGroup> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroup> for ::windows::core::IUnknown {
    fn from(value: &ISharedPropertyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroup> for super::Com::IDispatch {
    fn from(value: ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISharedPropertyGroup> for &'a super::Com::IDispatch {
    fn from(value: &'a ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroup> for super::Com::IDispatch {
    fn from(value: &ISharedPropertyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISharedPropertyGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISharedPropertyGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISharedPropertyGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISharedPropertyGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedPropertyGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISharedPropertyGroup {
    type Vtable = ISharedPropertyGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a005c07_a5de_11cf_9e66_00aa00a3f464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyByPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut i16, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyByPosition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_PropertyByPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_PropertyByPosition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Property: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISharedPropertyGroupManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePropertyGroup<'a, P0>(&self, name: P0, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).CreatePropertyGroup)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(dwisomode), ::core::mem::transmute(dwrelmode), ::core::mem::transmute(fexists), ::core::mem::transmute(ppgroup)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_Group<'a, P0>(&self, name: P0) -> ::windows::core::Result<ISharedPropertyGroup>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Group)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISharedPropertyGroup>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroupManager> for ::windows::core::IUnknown {
    fn from(value: ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISharedPropertyGroupManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroupManager> for ::windows::core::IUnknown {
    fn from(value: &ISharedPropertyGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroupManager> for super::Com::IDispatch {
    fn from(value: ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISharedPropertyGroupManager> for &'a super::Com::IDispatch {
    fn from(value: &'a ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroupManager> for super::Com::IDispatch {
    fn from(value: &ISharedPropertyGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISharedPropertyGroupManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISharedPropertyGroupManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISharedPropertyGroupManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISharedPropertyGroupManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedPropertyGroupManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISharedPropertyGroupManager {
    type Vtable = ISharedPropertyGroupManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a005c0d_a5de_11cf_9e66_00aa00a3f464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroupManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePropertyGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePropertyGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Group: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ISystemAppEventData(::windows::core::IUnknown);
impl ISystemAppEventData {
    pub unsafe fn Startup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Startup)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDataChanged<'a, P0>(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: P0, dwreason: u32, u64tracehandle: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).OnDataChanged)(::windows::core::Interface::as_raw(self), dwpid, dwmask, dwnumbersinks, bstrdwmethodmask.into().abi(), dwreason, u64tracehandle).ok()
    }
}
impl ::core::convert::From<ISystemAppEventData> for ::windows::core::IUnknown {
    fn from(value: ISystemAppEventData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISystemAppEventData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISystemAppEventData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemAppEventData> for ::windows::core::IUnknown {
    fn from(value: &ISystemAppEventData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISystemAppEventData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemAppEventData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemAppEventData {}
impl ::core::fmt::Debug for ISystemAppEventData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemAppEventData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemAppEventData {
    type Vtable = ISystemAppEventData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6d48a3c_d5c5_49e7_8c74_99e4889ed52f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemAppEventData_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Startup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDataChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDataChanged: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct IThreadPoolKnobs(::windows::core::IUnknown);
impl IThreadPoolKnobs {
    pub unsafe fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaxThreads)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plcmaxthreads)).ok()
    }
    pub unsafe fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentThreads)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plccurrentthreads)).ok()
    }
    pub unsafe fn SetMaxThreads(&self, lcmaxthreads: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxThreads)(::windows::core::Interface::as_raw(self), lcmaxthreads).ok()
    }
    pub unsafe fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeleteDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmsecdeletedelay)).ok()
    }
    pub unsafe fn SetDeleteDelay(&self, msecdeletedelay: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeleteDelay)(::windows::core::Interface::as_raw(self), msecdeletedelay).ok()
    }
    pub unsafe fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaxQueuedRequests)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plcmaxqueuedrequests)).ok()
    }
    pub unsafe fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentQueuedRequests)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plccurrentqueuedrequests)).ok()
    }
    pub unsafe fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxQueuedRequests)(::windows::core::Interface::as_raw(self), lcmaxqueuedrequests).ok()
    }
    pub unsafe fn SetMinThreads(&self, lcminthreads: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinThreads)(::windows::core::Interface::as_raw(self), lcminthreads).ok()
    }
    pub unsafe fn SetQueueDepth(&self, lcqueuedepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQueueDepth)(::windows::core::Interface::as_raw(self), lcqueuedepth).ok()
    }
}
impl ::core::convert::From<IThreadPoolKnobs> for ::windows::core::IUnknown {
    fn from(value: IThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IThreadPoolKnobs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IThreadPoolKnobs> for ::windows::core::IUnknown {
    fn from(value: &IThreadPoolKnobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IThreadPoolKnobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IThreadPoolKnobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThreadPoolKnobs {}
impl ::core::fmt::Debug for IThreadPoolKnobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThreadPoolKnobs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IThreadPoolKnobs {
    type Vtable = IThreadPoolKnobs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51372af7_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolKnobs_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows::core::HRESULT,
    pub GetCurrentThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows::core::HRESULT,
    pub GetDeleteDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows::core::HRESULT,
    pub SetDeleteDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows::core::HRESULT,
    pub GetMaxQueuedRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows::core::HRESULT,
    pub GetCurrentQueuedRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxQueuedRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows::core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows::core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITransactionContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITransactionContext {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateInstance<'a, P0>(&self, pszprogid: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), pszprogid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITransactionContext> for ::windows::core::IUnknown {
    fn from(value: ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITransactionContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITransactionContext> for ::windows::core::IUnknown {
    fn from(value: &ITransactionContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITransactionContext> for super::Com::IDispatch {
    fn from(value: ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITransactionContext> for &'a super::Com::IDispatch {
    fn from(value: &'a ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITransactionContext> for super::Com::IDispatch {
    fn from(value: &ITransactionContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITransactionContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITransactionContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITransactionContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITransactionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITransactionContext {
    type Vtable = ITransactionContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7999fc21_d3c6_11cf_acab_00a024a55aef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateInstance: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ITransactionContextEx(::windows::core::IUnknown);
impl ITransactionContextEx {
    pub unsafe fn CreateInstance<T>(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rclsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionContextEx> for ::windows::core::IUnknown {
    fn from(value: ITransactionContextEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionContextEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionContextEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionContextEx> for ::windows::core::IUnknown {
    fn from(value: &ITransactionContextEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionContextEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionContextEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionContextEx {}
impl ::core::fmt::Debug for ITransactionContextEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionContextEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionContextEx {
    type Vtable = ITransactionContextEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7999fc22_d3c6_11cf_acab_00a024a55aef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContextEx_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ITransactionProperty(::windows::core::IUnknown);
impl ITransactionProperty {
    pub unsafe fn Reserved1(&self) {
        (::windows::core::Interface::vtable(self).Reserved1)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved2(&self) {
        (::windows::core::Interface::vtable(self).Reserved2)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved3(&self) {
        (::windows::core::Interface::vtable(self).Reserved3)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved4(&self) {
        (::windows::core::Interface::vtable(self).Reserved4)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved5(&self) {
        (::windows::core::Interface::vtable(self).Reserved5)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved6(&self) {
        (::windows::core::Interface::vtable(self).Reserved6)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved7(&self) {
        (::windows::core::Interface::vtable(self).Reserved7)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved8(&self) {
        (::windows::core::Interface::vtable(self).Reserved8)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved9(&self) {
        (::windows::core::Interface::vtable(self).Reserved9)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetTransactionResourcePool(&self) -> ::windows::core::Result<ITransactionResourcePool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransactionResourcePool)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionResourcePool>(result__)
    }
    pub unsafe fn Reserved10(&self) {
        (::windows::core::Interface::vtable(self).Reserved10)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved11(&self) {
        (::windows::core::Interface::vtable(self).Reserved11)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved12(&self) {
        (::windows::core::Interface::vtable(self).Reserved12)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved13(&self) {
        (::windows::core::Interface::vtable(self).Reserved13)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved14(&self) {
        (::windows::core::Interface::vtable(self).Reserved14)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved15(&self) {
        (::windows::core::Interface::vtable(self).Reserved15)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved16(&self) {
        (::windows::core::Interface::vtable(self).Reserved16)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved17(&self) {
        (::windows::core::Interface::vtable(self).Reserved17)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ITransactionProperty> for ::windows::core::IUnknown {
    fn from(value: ITransactionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionProperty> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionProperty> for ::windows::core::IUnknown {
    fn from(value: &ITransactionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionProperty {}
impl ::core::fmt::Debug for ITransactionProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionProperty {
    type Vtable = ITransactionProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x788ea814_87b1_11d1_bba6_00c04fc2fa5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProperty_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetTransactionResourcePool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptxpool: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reserved10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved14: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved15: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved17: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ITransactionProxy(::windows::core::IUnknown);
impl ITransactionProxy {
    pub unsafe fn Commit(&self, guid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn Promote(&self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Promote)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::DistributedTransactionCoordinator::ITransaction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateVoter<'a, P0>(&self, ptxasync: P0) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateVoter)(::windows::core::Interface::as_raw(self), ptxasync.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>(result__)
    }
    pub unsafe fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIsolationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(__midl__itransactionproxy0000)).ok()
    }
    pub unsafe fn GetIdentifier(&self, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdentifier)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstridentifier)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsReusable(&self, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsReusable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfisreusable)).ok()
    }
}
impl ::core::convert::From<ITransactionProxy> for ::windows::core::IUnknown {
    fn from(value: ITransactionProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionProxy> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionProxy> for ::windows::core::IUnknown {
    fn from(value: &ITransactionProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionProxy {}
impl ::core::fmt::Debug for ITransactionProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionProxy {
    type Vtable = ITransactionProxy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02558374_df2e_4dae_bd6b_1d5c994f9bdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProxy_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub Promote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    Promote: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateVoter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptxasync: *mut ::core::ffi::c_void, ppballot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateVoter: usize,
    pub GetIsolationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsReusable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsReusable: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ITransactionResourcePool(::windows::core::IUnknown);
impl ITransactionResourcePool {
    pub unsafe fn PutResource<'a, P0, P1>(&self, ppool: P0, punk: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IObjPool>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).PutResource)(::windows::core::Interface::as_raw(self), ppool.into().abi(), punk.into().abi()).ok()
    }
    pub unsafe fn GetResource<'a, P0>(&self, ppool: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IObjPool>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetResource)(::windows::core::Interface::as_raw(self), ppool.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ITransactionResourcePool> for ::windows::core::IUnknown {
    fn from(value: ITransactionResourcePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionResourcePool> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionResourcePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionResourcePool> for ::windows::core::IUnknown {
    fn from(value: &ITransactionResourcePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionResourcePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionResourcePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResourcePool {}
impl ::core::fmt::Debug for ITransactionResourcePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResourcePool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionResourcePool {
    type Vtable = ITransactionResourcePool_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5feb7c1_346a_11d1_b1cc_00aa00ba3258);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourcePool_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PutResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ITransactionStatus(::windows::core::IUnknown);
impl ITransactionStatus {
    pub unsafe fn SetTransactionStatus(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransactionStatus)(::windows::core::Interface::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn GetTransactionStatus(&self, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTransactionStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phrstatus)).ok()
    }
}
impl ::core::convert::From<ITransactionStatus> for ::windows::core::IUnknown {
    fn from(value: ITransactionStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionStatus> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionStatus> for ::windows::core::IUnknown {
    fn from(value: &ITransactionStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionStatus {}
impl ::core::fmt::Debug for ITransactionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionStatus {
    type Vtable = ITransactionStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61f589e8_3724_4898_a0a4_664ae9e1d1b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionStatus_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetTransactionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetTransactionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ITxProxyHolder(::windows::core::IUnknown);
impl ITxProxyHolder {
    pub unsafe fn GetIdentifier(&self, pguidltx: *mut ::windows::core::GUID) {
        (::windows::core::Interface::vtable(self).GetIdentifier)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidltx))
    }
}
impl ::core::convert::From<ITxProxyHolder> for ::windows::core::IUnknown {
    fn from(value: ITxProxyHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITxProxyHolder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITxProxyHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITxProxyHolder> for ::windows::core::IUnknown {
    fn from(value: &ITxProxyHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITxProxyHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITxProxyHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITxProxyHolder {}
impl ::core::fmt::Debug for ITxProxyHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITxProxyHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITxProxyHolder {
    type Vtable = ITxProxyHolder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d86f31_0139_41af_bcad_c7d50435fe9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITxProxyHolder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows::core::GUID),
}
pub const LBEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LockModes(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LockSetGet: LockModes = LockModes(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LockMethod: LockModes = LockModes(1i32);
impl ::core::marker::Copy for LockModes {}
impl ::core::clone::Clone for LockModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LockModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LockModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for LockModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockModes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn MTSCreateActivity(riid: *const ::windows::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MTSCreateActivity(riid: *const ::windows::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    MTSCreateActivity(::core::mem::transmute(riid), ::core::mem::transmute(ppobj)).ok()
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const MessageMover: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ObjectContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ObjectContext {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateInstance<'a, P0>(&self, bstrprogid: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), bstrprogid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsSecurityEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSecurityEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCallerInRole<'a, P0>(&self, bstrrole: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsCallerInRole)(::windows::core::Interface::as_raw(self), bstrrole.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, name: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> ::windows::core::Result<SecurityProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Security)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SecurityProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ContextInfo(&self) -> ::windows::core::Result<ContextInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ContextInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ContextInfo>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ObjectContext> for ::windows::core::IUnknown {
    fn from(value: ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ObjectContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ObjectContext> for ::windows::core::IUnknown {
    fn from(value: &ObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ObjectContext> for super::Com::IDispatch {
    fn from(value: ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ObjectContext> for &'a super::Com::IDispatch {
    fn from(value: &'a ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ObjectContext> for super::Com::IDispatch {
    fn from(value: &ObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ObjectContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ObjectContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ObjectContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ObjectContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ObjectContext {
    type Vtable = ObjectContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74c08646_cedb_11cf_8b49_00aa00b8a790);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ObjectContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateInstance: usize,
    pub SetComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCallerInRole: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ContextInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ContextInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
pub struct ObjectControl(::windows::core::IUnknown);
impl ObjectControl {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanBePooled(&self, pbpoolable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CanBePooled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbpoolable)).ok()
    }
}
impl ::core::convert::From<ObjectControl> for ::windows::core::IUnknown {
    fn from(value: ObjectControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ObjectControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ObjectControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectControl> for ::windows::core::IUnknown {
    fn from(value: &ObjectControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ObjectControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ObjectControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ObjectControl {}
impl ::core::fmt::Debug for ObjectControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ObjectControl {
    type Vtable = ObjectControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dc41850_0c31_11d0_8b79_00aa00b8a790);
}
#[repr(C)]
#[doc(hidden)]
pub struct ObjectControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanBePooled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpoolable: *mut i16) -> ::windows::core::HRESULT,
}
pub const PoolMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: ::windows::core::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
impl ::core::marker::Copy for RECYCLE_INFO {}
impl ::core::clone::Clone for RECYCLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECYCLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECYCLE_INFO").field("guidCombaseProcessIdentifier", &self.guidCombaseProcessIdentifier).field("ProcessStartTime", &self.ProcessStartTime).field("dwRecycleLifetimeLimit", &self.dwRecycleLifetimeLimit).field("dwRecycleMemoryLimit", &self.dwRecycleMemoryLimit).field("dwRecycleExpirationTimeout", &self.dwRecycleExpirationTimeout).finish()
    }
}
unsafe impl ::windows::core::Abi for RECYCLE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECYCLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECYCLE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECYCLE_INFO {}
impl ::core::default::Default for RECYCLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn RecycleSurrogate(lreasoncode: i32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RecycleSurrogate(lreasoncode: i32) -> ::windows::core::HRESULT;
    }
    RecycleSurrogate(lreasoncode).ok()
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ReleaseModes(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Standard: ReleaseModes = ReleaseModes(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Process: ReleaseModes = ReleaseModes(1i32);
impl ::core::marker::Copy for ReleaseModes {}
impl ::core::clone::Clone for ReleaseModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReleaseModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ReleaseModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReleaseModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReleaseModes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[inline]
pub unsafe fn SafeRef<'a, P0>(rid: *const ::windows::core::GUID, punk: P0) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SafeRef(rid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    }
    SafeRef(::core::mem::transmute(rid), punk.into().abi())
}
pub const SecurityCallContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct SecurityProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl SecurityProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectCallerName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDirectCallerName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectCreatorName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDirectCreatorName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOriginalCallerName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOriginalCallerName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOriginalCreatorName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOriginalCreatorName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SecurityProperty> for ::windows::core::IUnknown {
    fn from(value: SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a SecurityProperty> for &'a ::windows::core::IUnknown {
    fn from(value: &'a SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SecurityProperty> for ::windows::core::IUnknown {
    fn from(value: &SecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SecurityProperty> for super::Com::IDispatch {
    fn from(value: SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a SecurityProperty> for &'a super::Com::IDispatch {
    fn from(value: &'a SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SecurityProperty> for super::Com::IDispatch {
    fn from(value: &SecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SecurityProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SecurityProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SecurityProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SecurityProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for SecurityProperty {
    type Vtable = SecurityProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe74a7215_014d_11d1_a63c_00a0c911b4e0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SecurityProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCallerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCallerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCreatorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCreatorName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCallerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCallerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCreatorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCreatorName: usize,
}
pub const ServicePool: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRACKER_INIT_EVENT: &str = "Global\\COM+ Tracker Init Event";
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRACKER_STARTSTOP_EVENT: &str = "Global\\COM+ Tracker Push Event";
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACKING_COLL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(2i32);
impl ::core::marker::Copy for TRACKING_COLL_TYPE {}
impl ::core::clone::Clone for TRACKING_COLL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACKING_COLL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TRACKING_COLL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRACKING_COLL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKING_COLL_TYPE").field(&self.0).finish()
    }
}
pub const TrackerServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TransactionVote(pub i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxCommit: TransactionVote = TransactionVote(0i32);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxAbort: TransactionVote = TransactionVote(1i32);
impl ::core::marker::Copy for TransactionVote {}
impl ::core::clone::Clone for TransactionVote {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TransactionVote {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TransactionVote {
    type Abi = Self;
}
impl ::core::fmt::Debug for TransactionVote {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransactionVote").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
