windows_link::link!("comsvcs.dll" "system" fn CoCreateActivity(piunknown : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "system" fn CoEnterServiceDomain(pconfigobject : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_link::link!("ole32.dll" "system" fn CoGetDefaultContext(apttype : super::Com::APTTYPE, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "system" fn CoLeaveServiceDomain(punkstatus : *mut core::ffi::c_void));
windows_link::link!("mtxdm.dll" "C" fn GetDispenserManager(param0 : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "system" fn GetManagedExtensions(dwexts : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "system" fn MTSCreateActivity(riid : *const windows_sys::core::GUID, ppobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "C" fn RecycleSurrogate(lreasoncode : i32) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "C" fn SafeRef(rid : *const windows_sys::core::GUID, punk : *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPDATA {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: APPSTATISTICS,
}
impl Default for APPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APPSTATISTICS {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = 0;
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = 1;
pub const APPTYPE_SWC: COMPLUS_APPTYPE = 2;
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = -1;
pub const AppDomainHelper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: windows_sys::core::BOOL,
    pub IsRecycled: windows_sys::core::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: windows_sys::core::BOOL,
    pub HasAutomaticLifetimeRecycling: windows_sys::core::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: windows_sys::core::GUID,
    pub ApplicationIdPrimaryApplication: windows_sys::core::GUID,
    pub ApplicationInstanceId: windows_sys::core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: windows_sys::core::PWSTR,
    pub IsService: windows_sys::core::BOOL,
    pub IsPaused: windows_sys::core::BOOL,
    pub IsRecycled: windows_sys::core::BOOL,
}
impl Default for ApplicationProcessSummary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub ApplicationId: windows_sys::core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: windows_sys::core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
impl Default for ApplicationSummary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AutoSvcs_Error_Constants = u32;
pub const ByotServerEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLSIDDATA {
    pub m_clsid: windows_sys::core::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLSIDDATA2 {
    pub m_clsid: windows_sys::core::GUID,
    pub m_appid: windows_sys::core::GUID,
    pub m_partid: windows_sys::core::GUID,
    pub m_pwszAppName: windows_sys::core::PWSTR,
    pub m_pwszCtxName: windows_sys::core::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl Default for CLSIDDATA2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COMAdmin32BitComponent: COMAdminComponentType = 1;
pub const COMAdmin64BitComponent: COMAdminComponentType = 2;
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = 1;
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = 0;
pub type COMAdminAccessChecksLevelOptions = i32;
pub const COMAdminActivationInproc: COMAdminActivationOptions = 0;
pub const COMAdminActivationLocal: COMAdminActivationOptions = 1;
pub type COMAdminActivationOptions = i32;
pub type COMAdminApplicationExportOptions = i32;
pub type COMAdminApplicationInstallOptions = i32;
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = 3;
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = 64;
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = 0;
pub type COMAdminAuthenticationCapabilitiesOptions = i32;
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = 2;
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = 32;
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = 2;
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = 0;
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = 5;
pub type COMAdminAuthenticationLevelOptions = i32;
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = 1;
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = 4;
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = 6;
pub const COMAdminCatalog: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = 16;
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = 2;
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = 8;
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = 32;
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = 4;
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = 1;
pub type COMAdminComponentFlags = i32;
pub type COMAdminComponentType = i32;
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = -2146368508;
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = -2146368481;
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = -2146368504;
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = -2146368503;
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = -2146368505;
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = -2146367478;
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = -2146368501;
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = -2146368442;
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = -2146368493;
pub const COMAdminErrBadPath: COMAdminErrorCodes = -2146368502;
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = -2146368482;
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = -2146368494;
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = -2146368432;
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = -2146368488;
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = -2146368438;
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = -2146368436;
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = -2146368437;
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = -2146368435;
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = -2146367456;
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = -2146368499;
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = -2146367473;
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = -2146367471;
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = -2146368382;
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = -2146368379;
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = -2146368378;
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = -2146368381;
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = -2146368380;
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = -2146368459;
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = -2146368472;
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = -2146368473;
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = -2146368476;
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = -2146368474;
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = -2146368475;
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = -2146368460;
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = -2146368471;
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = -2146368466;
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = -2146367459;
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = -2146368467;
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = -2146367458;
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = -2146367460;
pub const COMAdminErrComponentExists: COMAdminErrorCodes = -2146368455;
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = -2146368483;
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = -2146368486;
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = -2146368425;
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = -2146368434;
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = -2146368421;
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = -2146367477;
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = -2146368496;
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = -2146368509;
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = -2146368433;
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = -2146368383;
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = -2146368384;
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = -2146368495;
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = -2146368485;
pub const COMAdminErrNoUser: COMAdminErrorCodes = -2146368497;
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = -2146368470;
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = -2146368469;
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = -2146368450;
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = -2146367479;
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = -2146368511;
pub const COMAdminErrObjectExists: COMAdminErrorCodes = -2146368456;
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = -2146368510;
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = -2146368449;
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = -2146367480;
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = -2146368423;
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = -2146367463;
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = -2146367469;
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = -2146367470;
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = -2146368452;
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = -2146368457;
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = -2146367998;
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = -2146368453;
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = -2146368395;
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = -2146368398;
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = -2146368397;
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = -2146368396;
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = -2146368464;
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = -2146368477;
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = -2146368487;
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = -2146368439;
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = -2146368441;
pub const COMAdminErrRoleExists: COMAdminErrorCodes = -2146368500;
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = -2146368458;
pub const COMAdminErrSession: COMAdminErrorCodes = -2146368468;
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = -2146368431;
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = -2146368440;
pub const COMAdminErrSystemApp: COMAdminErrorCodes = -2146368461;
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = -2146368492;
pub type COMAdminErrorCodes = i32;
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = 2;
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = 4;
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = 16;
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = 0;
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = 1;
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = 512;
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = 1024;
pub const COMAdminFileFlagCOM: COMAdminFileFlags = 2;
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = 4096;
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = 8;
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = 4;
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = 16;
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = 32768;
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = 256;
pub const COMAdminFileFlagError: COMAdminFileFlags = 262144;
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = 2048;
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = 1;
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = 16384;
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = 65536;
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = 8192;
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = 131072;
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = 32;
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = 64;
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = 128;
pub type COMAdminFileFlags = i32;
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = 1;
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = 4;
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = 2;
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = 3;
pub type COMAdminImpersonationLevelOptions = i32;
pub type COMAdminInUse = i32;
pub const COMAdminInUseByCatalog: COMAdminInUse = 1;
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = 5;
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = 3;
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = 4;
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = 2;
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = 2;
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = 0;
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = 1;
pub const COMAdminNotInUse: COMAdminInUse = 0;
pub type COMAdminOS = i32;
pub const COMAdminOSNotInitialized: COMAdminOS = 0;
pub const COMAdminOSUnknown: COMAdminOS = 6;
pub const COMAdminOSWindows2000: COMAdminOS = 3;
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = 4;
pub const COMAdminOSWindows2000Unknown: COMAdminOS = 5;
pub const COMAdminOSWindows3_1: COMAdminOS = 1;
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = 27;
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = 26;
pub const COMAdminOSWindows7Personal: COMAdminOS = 23;
pub const COMAdminOSWindows7Professional: COMAdminOS = 24;
pub const COMAdminOSWindows7StandardServer: COMAdminOS = 25;
pub const COMAdminOSWindows7WebServer: COMAdminOS = 28;
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = 33;
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = 32;
pub const COMAdminOSWindows8Personal: COMAdminOS = 29;
pub const COMAdminOSWindows8Professional: COMAdminOS = 30;
pub const COMAdminOSWindows8StandardServer: COMAdminOS = 31;
pub const COMAdminOSWindows8WebServer: COMAdminOS = 34;
pub const COMAdminOSWindows9x: COMAdminOS = 2;
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = 39;
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = 38;
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = 35;
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = 36;
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = 37;
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = 40;
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = 21;
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = 20;
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = 17;
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = 18;
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = 19;
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = 22;
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = 15;
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = 14;
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = 13;
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = 16;
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = 11;
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = 12;
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = 1;
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = 2;
pub type COMAdminQCMessageAuthenticateOptions = i32;
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = 0;
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = 4;
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = 1;
pub type COMAdminServiceOptions = i32;
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = 5;
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = 6;
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = 3;
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = 1;
pub type COMAdminServiceStatusOptions = i32;
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = 2;
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = 0;
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = 7;
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = 0;
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = 1;
pub type COMAdminSynchronizationOptions = i32;
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = 3;
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = 4;
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = 2;
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = 0;
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = 3;
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = 1;
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = 2;
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = 4;
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = 5;
pub type COMAdminThreadingModels = i32;
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = 0;
pub const COMAdminTransactionNone: COMAdminTransactionOptions = 1;
pub type COMAdminTransactionOptions = i32;
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = 3;
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = 4;
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = 2;
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = 0;
pub type COMAdminTxIsolationLevelOptions = i32;
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = 2;
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = 1;
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = 3;
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = 4;
pub const COMEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
pub type COMPLUS_APPTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: windows_sys::core::GUID,
    pub sMachineName: windows_sys::core::PWSTR,
}
impl Default for COMSVCSEVENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRMClerk: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
pub type CRMFLAGS = i32;
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = 1;
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = 64;
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = 8;
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = 4;
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = 2;
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = 16;
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = 32;
pub type CRMREGFLAGS = i32;
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = 4;
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = 7;
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = 2;
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = 16;
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = 1;
pub const CRMRecoveryClerk: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294;
pub const CRR_CALL_LIMIT: u32 = 4294967293;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292;
pub const CRR_NO_REASON_SUPPLIED: u32 = 0;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291;
pub const CSC_BindToPoolThread: CSC_Binding = 1;
pub type CSC_Binding = i32;
pub type CSC_COMTIIntrinsicsConfig = i32;
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = 2;
pub const CSC_DontUseTracker: CSC_TrackerConfig = 0;
pub type CSC_IISIntrinsicsConfig = i32;
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = 1;
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = 1;
pub const CSC_Ignore: CSC_InheritanceConfig = 1;
pub const CSC_Inherit: CSC_InheritanceConfig = 0;
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 1;
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = 1;
pub const CSC_InheritPartition: CSC_PartitionConfig = 1;
pub const CSC_InheritSxs: CSC_SxsConfig = 1;
pub type CSC_InheritanceConfig = i32;
pub const CSC_MTAThreadPool: CSC_ThreadPool = 3;
pub const CSC_NewPartition: CSC_PartitionConfig = 2;
pub const CSC_NewSxs: CSC_SxsConfig = 2;
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = 3;
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = 2;
pub const CSC_NewTransaction: CSC_TransactionConfig = 3;
pub const CSC_NoBinding: CSC_Binding = 0;
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 0;
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = 0;
pub const CSC_NoPartition: CSC_PartitionConfig = 0;
pub const CSC_NoSxs: CSC_SxsConfig = 0;
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = 0;
pub const CSC_NoTransaction: CSC_TransactionConfig = 0;
pub type CSC_PartitionConfig = i32;
pub const CSC_STAThreadPool: CSC_ThreadPool = 2;
pub type CSC_SxsConfig = i32;
pub type CSC_SynchronizationConfig = i32;
pub type CSC_ThreadPool = i32;
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = 1;
pub const CSC_ThreadPoolNone: CSC_ThreadPool = 0;
pub type CSC_TrackerConfig = i32;
pub type CSC_TransactionConfig = i32;
pub const CSC_UseTracker: CSC_TrackerConfig = 1;
pub const CServiceConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
pub const CoMTSLocator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: windows_sys::core::BOOL,
    pub TerminateOnHang: windows_sys::core::BOOL,
    pub AvgCallThresholdInMs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComponentSummary {
    pub ApplicationInstanceId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub ApplicationId: windows_sys::core::GUID,
    pub Clsid: windows_sys::core::GUID,
    pub ClassName: windows_sys::core::PWSTR,
    pub ApplicationName: windows_sys::core::PWSTR,
}
impl Default for ComponentSummary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
pub type CrmTransactionState = i32;
pub const DATA_NOT_AVAILABLE: u32 = 4294967295;
pub type DUMPTYPE = i32;
pub const DUMPTYPE_FULL: DUMPTYPE = 0;
pub const DUMPTYPE_MINI: DUMPTYPE = 1;
pub const DUMPTYPE_NONE: DUMPTYPE = 2;
pub const DispenserManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub const EventServer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = 16;
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = 8;
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = 2;
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = 1;
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = 4;
pub const GUID_STRING_SIZE: u32 = 40;
pub type GetAppTrackerDataFlags = i32;
pub const GetSecurityCallContextAppObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: windows_sys::core::BOOL,
    pub fTerminateOnHang: windows_sys::core::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
pub const LBEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
pub const LockMethod: LockModes = 1;
pub type LockModes = i32;
pub const LockSetGet: LockModes = 0;
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392;
pub const MessageMover: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
pub const PoolMgr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
pub const Process: ReleaseModes = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: windows_sys::core::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
pub type ReleaseModes = i32;
pub const SecurityCallContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
pub const ServicePool: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
pub const Standard: ReleaseModes = 0;
pub const TRACKER_INIT_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("Global\\COM+ Tracker Init Event");
pub const TRACKER_STARTSTOP_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("Global\\COM+ Tracker Push Event");
pub type TRACKING_COLL_TYPE = i32;
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = 1;
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = 2;
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = 0;
pub const TrackerServer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
pub type TransactionVote = i32;
pub const TxAbort: TransactionVote = 1;
pub const TxCommit: TransactionVote = 0;
pub const TxState_Aborted: CrmTransactionState = 2;
pub const TxState_Active: CrmTransactionState = 0;
pub const TxState_Committed: CrmTransactionState = 1;
pub const TxState_Indoubt: CrmTransactionState = 3;
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = 2148599296;
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = 2148599297;
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = 2148599299;
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = 2148599298;
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = 2148599382;
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = 2148599376;
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = 2148599304;
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = 2148599380;
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = 2148599381;
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = 2148599379;
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = 2148599377;
pub const comqcErrOutParam: AutoSvcs_Error_Constants = 2148599301;
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = 2148599303;
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = 2148599300;
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = 2148599302;
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = 2148599378;
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = 2147803138;
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = 2147803139;
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = 2147803140;
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = 2147803149;
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = 2147803141;
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = 2147803143;
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = 2147803148;
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = 2147803142;
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = 2147803151;
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = 2147803150;
