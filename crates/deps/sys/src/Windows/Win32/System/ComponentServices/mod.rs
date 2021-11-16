#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn CoCreateActivity(piunknown: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoEnterServiceDomain(pconfigobject: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoLeaveServiceDomain(punkstatus: ::windows_sys::core::IUnknown);
    pub fn GetDispenserManager(param0: *mut IDispenserManager) -> ::windows_sys::core::HRESULT;
    pub fn GetManagedExtensions(dwexts: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn MTSCreateActivity(riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn RecycleSurrogate(lreasoncode: i32) -> ::windows_sys::core::HRESULT;
    pub fn SafeRef(rid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown) -> *mut ::core::ffi::c_void;
}
pub const AppDomainHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4012177033,
    data2: 5368,
    data3: 19858,
    data4: [180, 175, 215, 177, 240, 231, 15, 212],
};
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: ::windows_sys::core::GUID,
    pub ApplicationIdPrimaryApplication: ::windows_sys::core::GUID,
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: super::super::Foundation::PWSTR,
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: super::super::Foundation::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationSummary {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutoSvcs_Error_Constants = u32;
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = 2147803138u32;
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = 2147803139u32;
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = 2147803140u32;
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = 2147803141u32;
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = 2147803142u32;
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = 2147803143u32;
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = 2147803148u32;
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = 2147803149u32;
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = 2147803150u32;
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = 2147803151u32;
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = 2148599296u32;
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = 2148599297u32;
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = 2148599298u32;
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = 2148599299u32;
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = 2148599300u32;
pub const comqcErrOutParam: AutoSvcs_Error_Constants = 2148599301u32;
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = 2148599302u32;
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = 2148599303u32;
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = 2148599304u32;
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = 2148599376u32;
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = 2148599377u32;
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = 2148599378u32;
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = 2148599379u32;
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = 2148599380u32;
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = 2148599381u32;
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = 2148599382u32;
pub const ByotServerEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674858, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct CCLSIDData {
    pub m_clsid: ::windows_sys::core::GUID,
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CCLSIDData2 {
    pub m_clsid: ::windows_sys::core::GUID,
    pub m_appid: ::windows_sys::core::GUID,
    pub m_partid: ::windows_sys::core::GUID,
    pub m_pwszAppName: super::super::Foundation::PWSTR,
    pub m_pwszCtxName: super::super::Foundation::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CCLSIDData2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CCLSIDData2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type COMAdminAccessChecksLevelOptions = i32;
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = 0i32;
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = 1i32;
pub type COMAdminActivationOptions = i32;
pub const COMAdminActivationInproc: COMAdminActivationOptions = 0i32;
pub const COMAdminActivationLocal: COMAdminActivationOptions = 1i32;
pub type COMAdminApplicationExportOptions = i32;
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = 0i32;
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = 1i32;
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = 2i32;
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = 4i32;
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = 16i32;
pub type COMAdminApplicationInstallOptions = i32;
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = 0i32;
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = 1i32;
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = 2i32;
pub type COMAdminAuthenticationCapabilitiesOptions = i32;
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = 0i32;
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = 2i32;
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = 32i32;
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = 64i32;
pub type COMAdminAuthenticationLevelOptions = i32;
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = 0i32;
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = 1i32;
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = 2i32;
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = 3i32;
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = 4i32;
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = 5i32;
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = 6i32;
pub const COMAdminCatalog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128818452, data2: 57272, data3: 4561, data4: [162, 207, 0, 128, 95, 199, 146, 53] };
pub const COMAdminCatalogCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128818454, data2: 57272, data3: 4561, data4: [162, 207, 0, 128, 95, 199, 146, 53] };
pub const COMAdminCatalogObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128818453, data2: 57272, data3: 4561, data4: [162, 207, 0, 128, 95, 199, 146, 53] };
pub type COMAdminComponentFlags = i32;
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = 1i32;
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = 2i32;
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = 4i32;
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = 8i32;
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = 16i32;
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = 32i32;
pub type COMAdminComponentType = i32;
pub const COMAdmin32BitComponent: COMAdminComponentType = 1i32;
pub const COMAdmin64BitComponent: COMAdminComponentType = 2i32;
pub type COMAdminErrorCodes = i32;
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = -2146368511i32;
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = -2146368510i32;
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = -2146368509i32;
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = -2146368508i32;
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = -2146368505i32;
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = -2146368504i32;
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = -2146368503i32;
pub const COMAdminErrBadPath: COMAdminErrorCodes = -2146368502i32;
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = -2146368501i32;
pub const COMAdminErrRoleExists: COMAdminErrorCodes = -2146368500i32;
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = -2146368499i32;
pub const COMAdminErrNoUser: COMAdminErrorCodes = -2146368497i32;
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = -2146368496i32;
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = -2146368495i32;
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = -2146368494i32;
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = -2146368493i32;
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = -2146368492i32;
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = -2146368488i32;
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = -2146368487i32;
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = -2146368486i32;
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = -2146368485i32;
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = -2146368483i32;
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = -2146368482i32;
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = -2146368481i32;
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = -2146368477i32;
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = -2146368476i32;
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = -2146368475i32;
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = -2146368474i32;
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = -2146368473i32;
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = -2146368472i32;
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = -2146368471i32;
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = -2146368470i32;
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = -2146368469i32;
pub const COMAdminErrSession: COMAdminErrorCodes = -2146368468i32;
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = -2146368467i32;
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = -2146368466i32;
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = -2146368464i32;
pub const COMAdminErrSystemApp: COMAdminErrorCodes = -2146368461i32;
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = -2146368460i32;
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = -2146368459i32;
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = -2146368458i32;
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = -2146368457i32;
pub const COMAdminErrObjectExists: COMAdminErrorCodes = -2146368456i32;
pub const COMAdminErrComponentExists: COMAdminErrorCodes = -2146368455i32;
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = -2146368453i32;
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = -2146368452i32;
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = -2146368450i32;
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = -2146368449i32;
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = -2146368442i32;
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = -2146368441i32;
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = -2146368440i32;
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = -2146368439i32;
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = -2146367998i32;
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = -2146367480i32;
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = -2146367479i32;
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = -2146368438i32;
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = -2146368437i32;
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = -2146368436i32;
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = -2146368435i32;
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = -2146367478i32;
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = -2146368434i32;
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = -2146368433i32;
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = -2146368432i32;
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = -2146368425i32;
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = -2146368423i32;
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = -2146368421i32;
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = -2146368398i32;
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = -2146368397i32;
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = -2146368396i32;
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = -2146368395i32;
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = -2146368384i32;
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = -2146368383i32;
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = -2146368382i32;
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = -2146368381i32;
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = -2146368380i32;
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = -2146368379i32;
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = -2146368378i32;
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = -2146367473i32;
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = -2146367471i32;
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = -2146367470i32;
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = -2146367469i32;
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = -2146367477i32;
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = -2146367463i32;
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = -2146368431i32;
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = -2146367460i32;
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = -2146367459i32;
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = -2146367458i32;
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = -2146367456i32;
pub type COMAdminFileFlags = i32;
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = 1i32;
pub const COMAdminFileFlagCOM: COMAdminFileFlags = 2i32;
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = 4i32;
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = 8i32;
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = 16i32;
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = 32i32;
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = 64i32;
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = 128i32;
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = 256i32;
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = 512i32;
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = 1024i32;
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = 2048i32;
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = 4096i32;
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = 8192i32;
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = 16384i32;
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = 32768i32;
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = 65536i32;
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = 131072i32;
pub const COMAdminFileFlagError: COMAdminFileFlags = 262144i32;
pub type COMAdminImpersonationLevelOptions = i32;
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = 1i32;
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = 2i32;
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = 3i32;
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = 4i32;
pub type COMAdminInUse = i32;
pub const COMAdminNotInUse: COMAdminInUse = 0i32;
pub const COMAdminInUseByCatalog: COMAdminInUse = 1i32;
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = 2i32;
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = 3i32;
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = 4i32;
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = 5i32;
pub type COMAdminOS = i32;
pub const COMAdminOSNotInitialized: COMAdminOS = 0i32;
pub const COMAdminOSWindows3_1: COMAdminOS = 1i32;
pub const COMAdminOSWindows9x: COMAdminOS = 2i32;
pub const COMAdminOSWindows2000: COMAdminOS = 3i32;
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = 4i32;
pub const COMAdminOSWindows2000Unknown: COMAdminOS = 5i32;
pub const COMAdminOSUnknown: COMAdminOS = 6i32;
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = 11i32;
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = 12i32;
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = 13i32;
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = 14i32;
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = 15i32;
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = 16i32;
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = 17i32;
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = 18i32;
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = 19i32;
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = 20i32;
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = 21i32;
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = 22i32;
pub const COMAdminOSWindows7Personal: COMAdminOS = 23i32;
pub const COMAdminOSWindows7Professional: COMAdminOS = 24i32;
pub const COMAdminOSWindows7StandardServer: COMAdminOS = 25i32;
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = 26i32;
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = 27i32;
pub const COMAdminOSWindows7WebServer: COMAdminOS = 28i32;
pub const COMAdminOSWindows8Personal: COMAdminOS = 29i32;
pub const COMAdminOSWindows8Professional: COMAdminOS = 30i32;
pub const COMAdminOSWindows8StandardServer: COMAdminOS = 31i32;
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = 32i32;
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = 33i32;
pub const COMAdminOSWindows8WebServer: COMAdminOS = 34i32;
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = 35i32;
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = 36i32;
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = 37i32;
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = 38i32;
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = 39i32;
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = 40i32;
pub type COMAdminQCMessageAuthenticateOptions = i32;
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = 0i32;
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = 1i32;
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = 2i32;
pub type COMAdminServiceOptions = i32;
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = 1i32;
pub type COMAdminServiceStatusOptions = i32;
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = 0i32;
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = 1i32;
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = 2i32;
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = 3i32;
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = 4i32;
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = 5i32;
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = 6i32;
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = 7i32;
pub type COMAdminSynchronizationOptions = i32;
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = 0i32;
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = 1i32;
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = 2i32;
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = 3i32;
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = 4i32;
pub type COMAdminThreadingModels = i32;
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = 0i32;
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = 1i32;
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = 2i32;
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = 3i32;
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = 4i32;
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = 5i32;
pub type COMAdminTransactionOptions = i32;
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = 0i32;
pub const COMAdminTransactionNone: COMAdminTransactionOptions = 1i32;
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = 2i32;
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = 3i32;
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = 4i32;
pub type COMAdminTxIsolationLevelOptions = i32;
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = 0i32;
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = 1i32;
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = 2i32;
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = 3i32;
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = 4i32;
pub const COMEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674859, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub type COMPLUS_APPTYPE = i32;
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = -1i32;
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = 1i32;
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = 0i32;
pub const APPTYPE_SWC: COMPLUS_APPTYPE = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: ::windows_sys::core::GUID,
    pub sMachineName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMSVCSEVENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMSVCSEVENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CRMClerk: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674877, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub type CRMFLAGS = i32;
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = 1i32;
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = 2i32;
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = 4i32;
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = 8i32;
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = 16i32;
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = 32i32;
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = 64i32;
pub type CRMREGFLAGS = i32;
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = 1i32;
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = 2i32;
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = 4i32;
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = 7i32;
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = 16i32;
pub const CRMRecoveryClerk: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674878, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
pub type CSC_Binding = i32;
pub const CSC_NoBinding: CSC_Binding = 0i32;
pub const CSC_BindToPoolThread: CSC_Binding = 1i32;
pub type CSC_COMTIIntrinsicsConfig = i32;
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 0i32;
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 1i32;
pub type CSC_IISIntrinsicsConfig = i32;
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = 0i32;
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = 1i32;
pub type CSC_InheritanceConfig = i32;
pub const CSC_Inherit: CSC_InheritanceConfig = 0i32;
pub const CSC_Ignore: CSC_InheritanceConfig = 1i32;
pub type CSC_PartitionConfig = i32;
pub const CSC_NoPartition: CSC_PartitionConfig = 0i32;
pub const CSC_InheritPartition: CSC_PartitionConfig = 1i32;
pub const CSC_NewPartition: CSC_PartitionConfig = 2i32;
pub type CSC_SxsConfig = i32;
pub const CSC_NoSxs: CSC_SxsConfig = 0i32;
pub const CSC_InheritSxs: CSC_SxsConfig = 1i32;
pub const CSC_NewSxs: CSC_SxsConfig = 2i32;
pub type CSC_SynchronizationConfig = i32;
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = 0i32;
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = 1i32;
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = 2i32;
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = 3i32;
pub type CSC_ThreadPool = i32;
pub const CSC_ThreadPoolNone: CSC_ThreadPool = 0i32;
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = 1i32;
pub const CSC_STAThreadPool: CSC_ThreadPool = 2i32;
pub const CSC_MTAThreadPool: CSC_ThreadPool = 3i32;
pub type CSC_TrackerConfig = i32;
pub const CSC_DontUseTracker: CSC_TrackerConfig = 0i32;
pub const CSC_UseTracker: CSC_TrackerConfig = 1i32;
pub type CSC_TransactionConfig = i32;
pub const CSC_NoTransaction: CSC_TransactionConfig = 0i32;
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = 1i32;
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = 2i32;
pub const CSC_NewTransaction: CSC_TransactionConfig = 3i32;
pub const CServiceConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674888, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ClrAssemblyLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166713781, data2: 9818, data3: 19317, data4: [188, 5, 155, 234, 70, 48, 207, 24] };
pub const CoMTSLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674860, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ComServiceEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674883, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ComSystemAppEventData: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674886, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ComponentSummary {
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub Clsid: ::windows_sys::core::GUID,
    pub ClassName: super::super::Foundation::PWSTR,
    pub ApplicationName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ComponentSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ComponentSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContextInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContextInfo {}
impl ::core::clone::Clone for ContextInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContextInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContextInfo2 {}
impl ::core::clone::Clone for ContextInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub type CrmTransactionState = i32;
pub const TxState_Active: CrmTransactionState = 0i32;
pub const TxState_Committed: CrmTransactionState = 1i32;
pub const TxState_Aborted: CrmTransactionState = 2i32;
pub const TxState_Indoubt: CrmTransactionState = 3i32;
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
pub type DUMPTYPE = i32;
pub const DUMPTYPE_FULL: DUMPTYPE = 0i32;
pub const DUMPTYPE_MINI: DUMPTYPE = 1i32;
pub const DUMPTYPE_NONE: DUMPTYPE = 2i32;
pub const DispenserManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674880, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const Dummy30040732: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674857, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const EventServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674620, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const GUID_STRING_SIZE: u32 = 40u32;
pub type GetAppTrackerDataFlags = i32;
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = 1i32;
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = 2i32;
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = 4i32;
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = 8i32;
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = 16i32;
pub const GetSecurityCallContextAppObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674856, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
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
#[repr(transparent)]
pub struct IAppDomainHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDomainHelper {}
impl ::core::clone::Clone for IAppDomainHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAssemblyLocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAssemblyLocator {}
impl ::core::clone::Clone for IAssemblyLocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsyncErrorNotify(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsyncErrorNotify {}
impl ::core::clone::Clone for IAsyncErrorNotify {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICOMAdminCatalog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICOMAdminCatalog {}
impl ::core::clone::Clone for ICOMAdminCatalog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICOMAdminCatalog2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICOMAdminCatalog2 {}
impl ::core::clone::Clone for ICOMAdminCatalog2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICOMLBArguments(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICOMLBArguments {}
impl ::core::clone::Clone for ICOMLBArguments {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICatalogCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICatalogCollection {}
impl ::core::clone::Clone for ICatalogCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICatalogObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICatalogObject {}
impl ::core::clone::Clone for ICatalogObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICheckSxsConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICheckSxsConfig {}
impl ::core::clone::Clone for ICheckSxsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComActivityEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComActivityEvents {}
impl ::core::clone::Clone for IComActivityEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComApp2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComApp2Events {}
impl ::core::clone::Clone for IComApp2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComAppEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComAppEvents {}
impl ::core::clone::Clone for IComAppEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComCRMEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComCRMEvents {}
impl ::core::clone::Clone for IComCRMEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComExceptionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComExceptionEvents {}
impl ::core::clone::Clone for IComExceptionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComIdentityEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComIdentityEvents {}
impl ::core::clone::Clone for IComIdentityEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComInstance2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComInstance2Events {}
impl ::core::clone::Clone for IComInstance2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComInstanceEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComInstanceEvents {}
impl ::core::clone::Clone for IComInstanceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComLTxEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComLTxEvents {}
impl ::core::clone::Clone for IComLTxEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComMethod2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComMethod2Events {}
impl ::core::clone::Clone for IComMethod2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComMethodEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComMethodEvents {}
impl ::core::clone::Clone for IComMethodEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComMtaThreadPoolKnobs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComMtaThreadPoolKnobs {}
impl ::core::clone::Clone for IComMtaThreadPoolKnobs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComObjectConstruction2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComObjectConstruction2Events {}
impl ::core::clone::Clone for IComObjectConstruction2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComObjectConstructionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComObjectConstructionEvents {}
impl ::core::clone::Clone for IComObjectConstructionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComObjectEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComObjectEvents {}
impl ::core::clone::Clone for IComObjectEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComObjectPool2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComObjectPool2Events {}
impl ::core::clone::Clone for IComObjectPool2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComObjectPoolEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComObjectPoolEvents {}
impl ::core::clone::Clone for IComObjectPoolEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComObjectPoolEvents2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComObjectPoolEvents2 {}
impl ::core::clone::Clone for IComObjectPoolEvents2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComQCEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComQCEvents {}
impl ::core::clone::Clone for IComQCEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComResourceEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComResourceEvents {}
impl ::core::clone::Clone for IComResourceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComSecurityEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComSecurityEvents {}
impl ::core::clone::Clone for IComSecurityEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComStaThreadPoolKnobs {}
impl ::core::clone::Clone for IComStaThreadPoolKnobs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComStaThreadPoolKnobs2 {}
impl ::core::clone::Clone for IComStaThreadPoolKnobs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComThreadEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComThreadEvents {}
impl ::core::clone::Clone for IComThreadEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComTrackingInfoCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComTrackingInfoCollection {}
impl ::core::clone::Clone for IComTrackingInfoCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComTrackingInfoEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComTrackingInfoEvents {}
impl ::core::clone::Clone for IComTrackingInfoEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComTrackingInfoObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComTrackingInfoObject {}
impl ::core::clone::Clone for IComTrackingInfoObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComTrackingInfoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComTrackingInfoProperties {}
impl ::core::clone::Clone for IComTrackingInfoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComTransaction2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComTransaction2Events {}
impl ::core::clone::Clone for IComTransaction2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComTransactionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComTransactionEvents {}
impl ::core::clone::Clone for IComTransactionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComUserEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComUserEvent {}
impl ::core::clone::Clone for IComUserEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextProperties {}
impl ::core::clone::Clone for IContextProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextSecurityPerimeter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextSecurityPerimeter {}
impl ::core::clone::Clone for IContextSecurityPerimeter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextState {}
impl ::core::clone::Clone for IContextState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateWithLocalTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateWithLocalTransaction {}
impl ::core::clone::Clone for ICreateWithLocalTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateWithTipTransactionEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateWithTipTransactionEx {}
impl ::core::clone::Clone for ICreateWithTipTransactionEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateWithTransactionEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateWithTransactionEx {}
impl ::core::clone::Clone for ICreateWithTransactionEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmCompensator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmCompensator {}
impl ::core::clone::Clone for ICrmCompensator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmCompensatorVariants(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmCompensatorVariants {}
impl ::core::clone::Clone for ICrmCompensatorVariants {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmFormatLogRecords(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmFormatLogRecords {}
impl ::core::clone::Clone for ICrmFormatLogRecords {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmLogControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmLogControl {}
impl ::core::clone::Clone for ICrmLogControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmMonitor {}
impl ::core::clone::Clone for ICrmMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmMonitorClerks(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmMonitorClerks {}
impl ::core::clone::Clone for ICrmMonitorClerks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrmMonitorLogRecords(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrmMonitorLogRecords {}
impl ::core::clone::Clone for ICrmMonitorLogRecords {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispenserDriver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispenserDriver {}
impl ::core::clone::Clone for IDispenserDriver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispenserManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispenserManager {}
impl ::core::clone::Clone for IDispenserManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNames {}
impl ::core::clone::Clone for IEnumNames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEventServerTrace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEventServerTrace {}
impl ::core::clone::Clone for IEventServerTrace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetAppTrackerData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetAppTrackerData {}
impl ::core::clone::Clone for IGetAppTrackerData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetContextProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetContextProperties {}
impl ::core::clone::Clone for IGetContextProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetSecurityCallContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetSecurityCallContext {}
impl ::core::clone::Clone for IGetSecurityCallContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolder {}
impl ::core::clone::Clone for IHolder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILBEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILBEvents {}
impl ::core::clone::Clone for ILBEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMTSActivity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMTSActivity {}
impl ::core::clone::Clone for IMTSActivity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMTSCall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMTSCall {}
impl ::core::clone::Clone for IMTSCall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMTSLocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMTSLocator {}
impl ::core::clone::Clone for IMTSLocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManagedActivationEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManagedActivationEvents {}
impl ::core::clone::Clone for IManagedActivationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManagedObjectInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManagedObjectInfo {}
impl ::core::clone::Clone for IManagedObjectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManagedPoolAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManagedPoolAction {}
impl ::core::clone::Clone for IManagedPoolAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManagedPooledObj(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManagedPooledObj {}
impl ::core::clone::Clone for IManagedPooledObj {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageMover(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageMover {}
impl ::core::clone::Clone for IMessageMover {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMtsEventInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMtsEventInfo {}
impl ::core::clone::Clone for IMtsEventInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMtsEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMtsEvents {}
impl ::core::clone::Clone for IMtsEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMtsGrp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMtsGrp {}
impl ::core::clone::Clone for IMtsGrp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjPool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjPool {}
impl ::core::clone::Clone for IObjPool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectConstruct(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectConstruct {}
impl ::core::clone::Clone for IObjectConstruct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectConstructString(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectConstructString {}
impl ::core::clone::Clone for IObjectConstructString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectContext {}
impl ::core::clone::Clone for IObjectContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectContextActivity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectContextActivity {}
impl ::core::clone::Clone for IObjectContextActivity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectContextInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectContextInfo {}
impl ::core::clone::Clone for IObjectContextInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectContextInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectContextInfo2 {}
impl ::core::clone::Clone for IObjectContextInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectContextTip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectContextTip {}
impl ::core::clone::Clone for IObjectContextTip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectControl {}
impl ::core::clone::Clone for IObjectControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackControl {}
impl ::core::clone::Clone for IPlaybackControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPoolManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPoolManager {}
impl ::core::clone::Clone for IPoolManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessInitializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessInitializer {}
impl ::core::clone::Clone for IProcessInitializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecurityCallContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecurityCallContext {}
impl ::core::clone::Clone for ISecurityCallContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecurityCallersColl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecurityCallersColl {}
impl ::core::clone::Clone for ISecurityCallersColl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecurityIdentityColl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecurityIdentityColl {}
impl ::core::clone::Clone for ISecurityIdentityColl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecurityProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecurityProperty {}
impl ::core::clone::Clone for ISecurityProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectCOMLBServer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectCOMLBServer {}
impl ::core::clone::Clone for ISelectCOMLBServer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISendMethodEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISendMethodEvents {}
impl ::core::clone::Clone for ISendMethodEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceActivity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceActivity {}
impl ::core::clone::Clone for IServiceActivity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceCall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceCall {}
impl ::core::clone::Clone for IServiceCall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceComTIIntrinsicsConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceComTIIntrinsicsConfig {}
impl ::core::clone::Clone for IServiceComTIIntrinsicsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceIISIntrinsicsConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceIISIntrinsicsConfig {}
impl ::core::clone::Clone for IServiceIISIntrinsicsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceInheritanceConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceInheritanceConfig {}
impl ::core::clone::Clone for IServiceInheritanceConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServicePartitionConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServicePartitionConfig {}
impl ::core::clone::Clone for IServicePartitionConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServicePool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServicePool {}
impl ::core::clone::Clone for IServicePool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServicePoolConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServicePoolConfig {}
impl ::core::clone::Clone for IServicePoolConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceSxsConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceSxsConfig {}
impl ::core::clone::Clone for IServiceSxsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceSynchronizationConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceSynchronizationConfig {}
impl ::core::clone::Clone for IServiceSynchronizationConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceSysTxnConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceSysTxnConfig {}
impl ::core::clone::Clone for IServiceSysTxnConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceThreadPoolConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceThreadPoolConfig {}
impl ::core::clone::Clone for IServiceThreadPoolConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceTrackerConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceTrackerConfig {}
impl ::core::clone::Clone for IServiceTrackerConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceTransactionConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceTransactionConfig {}
impl ::core::clone::Clone for IServiceTransactionConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceTransactionConfigBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceTransactionConfigBase {}
impl ::core::clone::Clone for IServiceTransactionConfigBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISharedProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISharedProperty {}
impl ::core::clone::Clone for ISharedProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISharedPropertyGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISharedPropertyGroup {}
impl ::core::clone::Clone for ISharedPropertyGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISharedPropertyGroupManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISharedPropertyGroupManager {}
impl ::core::clone::Clone for ISharedPropertyGroupManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemAppEventData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemAppEventData {}
impl ::core::clone::Clone for ISystemAppEventData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThreadPoolKnobs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThreadPoolKnobs {}
impl ::core::clone::Clone for IThreadPoolKnobs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionContext {}
impl ::core::clone::Clone for ITransactionContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionContextEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionContextEx {}
impl ::core::clone::Clone for ITransactionContextEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionProperty {}
impl ::core::clone::Clone for ITransactionProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionProxy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionProxy {}
impl ::core::clone::Clone for ITransactionProxy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionResourcePool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionResourcePool {}
impl ::core::clone::Clone for ITransactionResourcePool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionStatus {}
impl ::core::clone::Clone for ITransactionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITxProxyHolder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITxProxyHolder {}
impl ::core::clone::Clone for ITxProxyHolder {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LBEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674881, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub type LockModes = i32;
pub const LockSetGet: LockModes = 0i32;
pub const LockMethod: LockModes = 1i32;
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const MessageMover: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674879, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const MtsGrp: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1261344141, data2: 915, data3: 4561, data4: [177, 171, 0, 170, 0, 186, 50, 88] };
#[repr(transparent)]
pub struct ObjectContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ObjectContext {}
impl ::core::clone::Clone for ObjectContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ObjectControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ObjectControl {}
impl ::core::clone::Clone for ObjectControl {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PoolMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674613, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: ::windows_sys::core::GUID,
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
pub type ReleaseModes = i32;
pub const Standard: ReleaseModes = 0i32;
pub const Process: ReleaseModes = 1i32;
pub const SecurityCallContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674855, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const SecurityCallers: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674854, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const SecurityIdentity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674853, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(transparent)]
pub struct SecurityProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SecurityProperty {}
impl ::core::clone::Clone for SecurityProperty {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ServicePool: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674889, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ServicePoolConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674890, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const SharedProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704666629, data2: 42462, data3: 4559, data4: [158, 102, 0, 170, 0, 163, 244, 100] };
pub const SharedPropertyGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704666635, data2: 42462, data3: 4559, data4: [158, 102, 0, 170, 0, 163, 244, 100] };
pub const SharedPropertyGroupManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704666641, data2: 42462, data3: 4559, data4: [158, 102, 0, 170, 0, 163, 244, 100] };
pub type TRACKING_COLL_TYPE = i32;
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = 0i32;
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = 1i32;
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = 2i32;
pub const TrackerServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674617, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const TransactionContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2040134693, data2: 54214, data3: 4559, data4: [172, 171, 0, 160, 36, 165, 90, 239] };
pub const TransactionContextEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1555457648, data2: 54228, data3: 4559, data4: [172, 171, 0, 160, 36, 165, 90, 239] };
pub type TransactionVote = i32;
pub const TxCommit: TransactionVote = 0i32;
pub const TxAbort: TransactionVote = 1i32;
