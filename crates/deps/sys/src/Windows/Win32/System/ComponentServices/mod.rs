#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(C)]
pub struct AppDomainHelper(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ApplicationProcessRecycleInfo(i32);
#[repr(C)]
pub struct ApplicationProcessStatistics(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ApplicationProcessSummary(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ApplicationSummary(i32);
#[repr(transparent)]
pub struct AutoSvcs_Error_Constants(pub u32);
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803138u32);
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803139u32);
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803140u32);
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803141u32);
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803142u32);
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803143u32);
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803148u32);
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803149u32);
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803150u32);
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803151u32);
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599296u32);
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599297u32);
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599298u32);
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599299u32);
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599300u32);
pub const comqcErrOutParam: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599301u32);
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599302u32);
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599303u32);
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599304u32);
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599376u32);
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599377u32);
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599378u32);
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599379u32);
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599380u32);
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599381u32);
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599382u32);
#[repr(C)]
pub struct ByotServerEx(i32);
#[repr(C)]
pub struct CAppData(i32);
#[repr(C)]
pub struct CAppStatistics(i32);
#[repr(C)]
pub struct CCLSIDData(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CCLSIDData2(i32);
#[repr(transparent)]
pub struct COMAdminAccessChecksLevelOptions(pub i32);
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(0i32);
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(1i32);
#[repr(transparent)]
pub struct COMAdminActivationOptions(pub i32);
pub const COMAdminActivationInproc: COMAdminActivationOptions = COMAdminActivationOptions(0i32);
pub const COMAdminActivationLocal: COMAdminActivationOptions = COMAdminActivationOptions(1i32);
#[repr(transparent)]
pub struct COMAdminApplicationExportOptions(pub i32);
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(0i32);
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(1i32);
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(2i32);
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(4i32);
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(16i32);
#[repr(transparent)]
pub struct COMAdminApplicationInstallOptions(pub i32);
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(0i32);
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(1i32);
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(2i32);
#[repr(transparent)]
pub struct COMAdminAuthenticationCapabilitiesOptions(pub i32);
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(0i32);
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(2i32);
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(32i32);
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(64i32);
#[repr(transparent)]
pub struct COMAdminAuthenticationLevelOptions(pub i32);
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(0i32);
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(1i32);
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(2i32);
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(3i32);
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(4i32);
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(5i32);
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(6i32);
#[repr(C)]
pub struct COMAdminCatalog(i32);
#[repr(C)]
pub struct COMAdminCatalogCollection(i32);
#[repr(C)]
pub struct COMAdminCatalogObject(i32);
#[repr(transparent)]
pub struct COMAdminComponentFlags(pub i32);
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = COMAdminComponentFlags(1i32);
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = COMAdminComponentFlags(2i32);
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = COMAdminComponentFlags(4i32);
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = COMAdminComponentFlags(8i32);
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = COMAdminComponentFlags(16i32);
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = COMAdminComponentFlags(32i32);
#[repr(transparent)]
pub struct COMAdminComponentType(pub i32);
pub const COMAdmin32BitComponent: COMAdminComponentType = COMAdminComponentType(1i32);
pub const COMAdmin64BitComponent: COMAdminComponentType = COMAdminComponentType(2i32);
#[repr(transparent)]
pub struct COMAdminErrorCodes(pub i32);
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = COMAdminErrorCodes(-2146368511i32);
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368510i32);
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146368509i32);
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368508i32);
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368505i32);
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368504i32);
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = COMAdminErrorCodes(-2146368503i32);
pub const COMAdminErrBadPath: COMAdminErrorCodes = COMAdminErrorCodes(-2146368502i32);
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368501i32);
pub const COMAdminErrRoleExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368500i32);
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = COMAdminErrorCodes(-2146368499i32);
pub const COMAdminErrNoUser: COMAdminErrorCodes = COMAdminErrorCodes(-2146368497i32);
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = COMAdminErrorCodes(-2146368496i32);
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368495i32);
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368494i32);
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = COMAdminErrorCodes(-2146368493i32);
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368492i32);
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368488i32);
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = COMAdminErrorCodes(-2146368487i32);
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = COMAdminErrorCodes(-2146368486i32);
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = COMAdminErrorCodes(-2146368485i32);
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368483i32);
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368482i32);
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368481i32);
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368477i32);
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368476i32);
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368475i32);
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = COMAdminErrorCodes(-2146368474i32);
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368473i32);
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368472i32);
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368471i32);
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368470i32);
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368469i32);
pub const COMAdminErrSession: COMAdminErrorCodes = COMAdminErrorCodes(-2146368468i32);
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = COMAdminErrorCodes(-2146368467i32);
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146368466i32);
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368464i32);
pub const COMAdminErrSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368461i32);
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = COMAdminErrorCodes(-2146368460i32);
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368459i32);
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368458i32);
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368457i32);
pub const COMAdminErrObjectExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368456i32);
pub const COMAdminErrComponentExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368455i32);
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = COMAdminErrorCodes(-2146368453i32);
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = COMAdminErrorCodes(-2146368452i32);
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = COMAdminErrorCodes(-2146368450i32);
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368449i32);
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368442i32);
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368441i32);
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = COMAdminErrorCodes(-2146368440i32);
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = COMAdminErrorCodes(-2146368439i32);
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = COMAdminErrorCodes(-2146367998i32);
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146367480i32);
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146367479i32);
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = COMAdminErrorCodes(-2146368438i32);
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368437i32);
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368436i32);
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = COMAdminErrorCodes(-2146368435i32);
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146367478i32);
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = COMAdminErrorCodes(-2146368434i32);
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = COMAdminErrorCodes(-2146368433i32);
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146368432i32);
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = COMAdminErrorCodes(-2146368425i32);
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = COMAdminErrorCodes(-2146368423i32);
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368421i32);
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = COMAdminErrorCodes(-2146368398i32);
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = COMAdminErrorCodes(-2146368397i32);
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = COMAdminErrorCodes(-2146368396i32);
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146368395i32);
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368384i32);
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368383i32);
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368382i32);
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368381i32);
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368380i32);
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368379i32);
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = COMAdminErrorCodes(-2146368378i32);
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367473i32);
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367471i32);
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367470i32);
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367469i32);
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = COMAdminErrorCodes(-2146367477i32);
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146367463i32);
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368431i32);
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = COMAdminErrorCodes(-2146367460i32);
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146367459i32);
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = COMAdminErrorCodes(-2146367458i32);
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = COMAdminErrorCodes(-2146367456i32);
#[repr(transparent)]
pub struct COMAdminFileFlags(pub i32);
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = COMAdminFileFlags(1i32);
pub const COMAdminFileFlagCOM: COMAdminFileFlags = COMAdminFileFlags(2i32);
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = COMAdminFileFlags(4i32);
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = COMAdminFileFlags(8i32);
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = COMAdminFileFlags(16i32);
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = COMAdminFileFlags(32i32);
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = COMAdminFileFlags(64i32);
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = COMAdminFileFlags(128i32);
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = COMAdminFileFlags(256i32);
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = COMAdminFileFlags(512i32);
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = COMAdminFileFlags(1024i32);
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = COMAdminFileFlags(2048i32);
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = COMAdminFileFlags(4096i32);
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = COMAdminFileFlags(8192i32);
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = COMAdminFileFlags(16384i32);
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = COMAdminFileFlags(32768i32);
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = COMAdminFileFlags(65536i32);
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = COMAdminFileFlags(131072i32);
pub const COMAdminFileFlagError: COMAdminFileFlags = COMAdminFileFlags(262144i32);
#[repr(transparent)]
pub struct COMAdminImpersonationLevelOptions(pub i32);
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(1i32);
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(2i32);
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(3i32);
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(4i32);
#[repr(transparent)]
pub struct COMAdminInUse(pub i32);
pub const COMAdminNotInUse: COMAdminInUse = COMAdminInUse(0i32);
pub const COMAdminInUseByCatalog: COMAdminInUse = COMAdminInUse(1i32);
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = COMAdminInUse(2i32);
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = COMAdminInUse(3i32);
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = COMAdminInUse(4i32);
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = COMAdminInUse(5i32);
#[repr(transparent)]
pub struct COMAdminOS(pub i32);
pub const COMAdminOSNotInitialized: COMAdminOS = COMAdminOS(0i32);
pub const COMAdminOSWindows3_1: COMAdminOS = COMAdminOS(1i32);
pub const COMAdminOSWindows9x: COMAdminOS = COMAdminOS(2i32);
pub const COMAdminOSWindows2000: COMAdminOS = COMAdminOS(3i32);
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = COMAdminOS(4i32);
pub const COMAdminOSWindows2000Unknown: COMAdminOS = COMAdminOS(5i32);
pub const COMAdminOSUnknown: COMAdminOS = COMAdminOS(6i32);
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = COMAdminOS(11i32);
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = COMAdminOS(12i32);
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = COMAdminOS(13i32);
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = COMAdminOS(14i32);
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = COMAdminOS(15i32);
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = COMAdminOS(16i32);
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = COMAdminOS(17i32);
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = COMAdminOS(18i32);
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = COMAdminOS(19i32);
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = COMAdminOS(20i32);
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = COMAdminOS(21i32);
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = COMAdminOS(22i32);
pub const COMAdminOSWindows7Personal: COMAdminOS = COMAdminOS(23i32);
pub const COMAdminOSWindows7Professional: COMAdminOS = COMAdminOS(24i32);
pub const COMAdminOSWindows7StandardServer: COMAdminOS = COMAdminOS(25i32);
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = COMAdminOS(26i32);
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = COMAdminOS(27i32);
pub const COMAdminOSWindows7WebServer: COMAdminOS = COMAdminOS(28i32);
pub const COMAdminOSWindows8Personal: COMAdminOS = COMAdminOS(29i32);
pub const COMAdminOSWindows8Professional: COMAdminOS = COMAdminOS(30i32);
pub const COMAdminOSWindows8StandardServer: COMAdminOS = COMAdminOS(31i32);
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = COMAdminOS(32i32);
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = COMAdminOS(33i32);
pub const COMAdminOSWindows8WebServer: COMAdminOS = COMAdminOS(34i32);
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = COMAdminOS(35i32);
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = COMAdminOS(36i32);
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = COMAdminOS(37i32);
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = COMAdminOS(38i32);
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = COMAdminOS(39i32);
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = COMAdminOS(40i32);
#[repr(transparent)]
pub struct COMAdminQCMessageAuthenticateOptions(pub i32);
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(0i32);
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(1i32);
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(2i32);
#[repr(transparent)]
pub struct COMAdminServiceOptions(pub i32);
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = COMAdminServiceOptions(1i32);
#[repr(transparent)]
pub struct COMAdminServiceStatusOptions(pub i32);
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(0i32);
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(1i32);
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(2i32);
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(3i32);
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(4i32);
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(5i32);
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(6i32);
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(7i32);
#[repr(transparent)]
pub struct COMAdminSynchronizationOptions(pub i32);
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(0i32);
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(1i32);
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(2i32);
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(3i32);
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(4i32);
#[repr(transparent)]
pub struct COMAdminThreadingModels(pub i32);
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = COMAdminThreadingModels(0i32);
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = COMAdminThreadingModels(1i32);
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = COMAdminThreadingModels(2i32);
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = COMAdminThreadingModels(3i32);
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = COMAdminThreadingModels(4i32);
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = COMAdminThreadingModels(5i32);
#[repr(transparent)]
pub struct COMAdminTransactionOptions(pub i32);
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = COMAdminTransactionOptions(0i32);
pub const COMAdminTransactionNone: COMAdminTransactionOptions = COMAdminTransactionOptions(1i32);
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = COMAdminTransactionOptions(2i32);
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = COMAdminTransactionOptions(3i32);
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = COMAdminTransactionOptions(4i32);
#[repr(transparent)]
pub struct COMAdminTxIsolationLevelOptions(pub i32);
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(0i32);
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(1i32);
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(2i32);
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(3i32);
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(4i32);
#[repr(C)]
pub struct COMEvents(i32);
#[repr(transparent)]
pub struct COMPLUS_APPTYPE(pub i32);
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = COMPLUS_APPTYPE(-1i32);
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = COMPLUS_APPTYPE(1i32);
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = COMPLUS_APPTYPE(0i32);
pub const APPTYPE_SWC: COMPLUS_APPTYPE = COMPLUS_APPTYPE(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMSVCSEVENTINFO(i32);
#[repr(C)]
pub struct CRMClerk(i32);
#[repr(transparent)]
pub struct CRMFLAGS(pub i32);
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = CRMFLAGS(1i32);
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = CRMFLAGS(2i32);
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = CRMFLAGS(4i32);
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = CRMFLAGS(8i32);
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = CRMFLAGS(16i32);
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = CRMFLAGS(32i32);
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = CRMFLAGS(64i32);
#[repr(transparent)]
pub struct CRMREGFLAGS(pub i32);
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = CRMREGFLAGS(1i32);
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = CRMREGFLAGS(2i32);
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = CRMREGFLAGS(4i32);
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = CRMREGFLAGS(7i32);
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = CRMREGFLAGS(16i32);
#[repr(C)]
pub struct CRMRecoveryClerk(i32);
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[repr(transparent)]
pub struct CSC_Binding(pub i32);
pub const CSC_NoBinding: CSC_Binding = CSC_Binding(0i32);
pub const CSC_BindToPoolThread: CSC_Binding = CSC_Binding(1i32);
#[repr(transparent)]
pub struct CSC_COMTIIntrinsicsConfig(pub i32);
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(0i32);
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(1i32);
#[repr(transparent)]
pub struct CSC_IISIntrinsicsConfig(pub i32);
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(0i32);
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(1i32);
#[repr(transparent)]
pub struct CSC_InheritanceConfig(pub i32);
pub const CSC_Inherit: CSC_InheritanceConfig = CSC_InheritanceConfig(0i32);
pub const CSC_Ignore: CSC_InheritanceConfig = CSC_InheritanceConfig(1i32);
#[repr(transparent)]
pub struct CSC_PartitionConfig(pub i32);
pub const CSC_NoPartition: CSC_PartitionConfig = CSC_PartitionConfig(0i32);
pub const CSC_InheritPartition: CSC_PartitionConfig = CSC_PartitionConfig(1i32);
pub const CSC_NewPartition: CSC_PartitionConfig = CSC_PartitionConfig(2i32);
#[repr(transparent)]
pub struct CSC_SxsConfig(pub i32);
pub const CSC_NoSxs: CSC_SxsConfig = CSC_SxsConfig(0i32);
pub const CSC_InheritSxs: CSC_SxsConfig = CSC_SxsConfig(1i32);
pub const CSC_NewSxs: CSC_SxsConfig = CSC_SxsConfig(2i32);
#[repr(transparent)]
pub struct CSC_SynchronizationConfig(pub i32);
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(0i32);
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = CSC_SynchronizationConfig(1i32);
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = CSC_SynchronizationConfig(2i32);
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(3i32);
#[repr(transparent)]
pub struct CSC_ThreadPool(pub i32);
pub const CSC_ThreadPoolNone: CSC_ThreadPool = CSC_ThreadPool(0i32);
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = CSC_ThreadPool(1i32);
pub const CSC_STAThreadPool: CSC_ThreadPool = CSC_ThreadPool(2i32);
pub const CSC_MTAThreadPool: CSC_ThreadPool = CSC_ThreadPool(3i32);
#[repr(transparent)]
pub struct CSC_TrackerConfig(pub i32);
pub const CSC_DontUseTracker: CSC_TrackerConfig = CSC_TrackerConfig(0i32);
pub const CSC_UseTracker: CSC_TrackerConfig = CSC_TrackerConfig(1i32);
#[repr(transparent)]
pub struct CSC_TransactionConfig(pub i32);
pub const CSC_NoTransaction: CSC_TransactionConfig = CSC_TransactionConfig(0i32);
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = CSC_TransactionConfig(1i32);
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = CSC_TransactionConfig(2i32);
pub const CSC_NewTransaction: CSC_TransactionConfig = CSC_TransactionConfig(3i32);
#[repr(C)]
pub struct CServiceConfig(i32);
#[repr(C)]
pub struct ClrAssemblyLocator(i32);
#[repr(C)]
pub struct CoMTSLocator(i32);
#[repr(C)]
pub struct ComServiceEvents(i32);
#[repr(C)]
pub struct ComSystemAppEventData(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ComponentHangMonitorInfo(i32);
#[repr(C)]
pub struct ComponentStatistics(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ComponentSummary(i32);
#[repr(transparent)]
pub struct ContextInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContextInfo2(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct CrmLogRecordRead(i32);
#[repr(transparent)]
pub struct CrmTransactionState(pub i32);
pub const TxState_Active: CrmTransactionState = CrmTransactionState(0i32);
pub const TxState_Committed: CrmTransactionState = CrmTransactionState(1i32);
pub const TxState_Aborted: CrmTransactionState = CrmTransactionState(2i32);
pub const TxState_Indoubt: CrmTransactionState = CrmTransactionState(3i32);
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[repr(transparent)]
pub struct DUMPTYPE(pub i32);
pub const DUMPTYPE_FULL: DUMPTYPE = DUMPTYPE(0i32);
pub const DUMPTYPE_MINI: DUMPTYPE = DUMPTYPE(1i32);
pub const DUMPTYPE_NONE: DUMPTYPE = DUMPTYPE(2i32);
#[repr(C)]
pub struct DispenserManager(i32);
#[repr(C)]
pub struct Dummy30040732(i32);
#[repr(C)]
pub struct EventServer(i32);
pub const GUID_STRING_SIZE: u32 = 40u32;
#[repr(transparent)]
pub struct GetAppTrackerDataFlags(pub i32);
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(1i32);
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = GetAppTrackerDataFlags(2i32);
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = GetAppTrackerDataFlags(4i32);
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(8i32);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(16i32);
#[repr(C)]
pub struct GetSecurityCallContextAppObject(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HANG_INFO(i32);
#[repr(transparent)]
pub struct IAppDomainHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssemblyLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncErrorNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICOMAdminCatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICOMAdminCatalog2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICOMLBArguments(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICatalogCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICatalogObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICheckSxsConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComActivityEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComApp2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComAppEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComCRMEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComExceptionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComIdentityEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComInstance2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComInstanceEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComLTxEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComMethod2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComMethodEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComMtaThreadPoolKnobs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComObjectConstruction2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComObjectConstructionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComObjectEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComObjectPool2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComObjectPoolEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComObjectPoolEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComQCEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComResourceEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComSecurityEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComThreadEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComTrackingInfoCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComTrackingInfoEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComTrackingInfoObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComTrackingInfoProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComTransaction2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComTransactionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComUserEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextSecurityPerimeter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateWithLocalTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateWithTipTransactionEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateWithTransactionEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmCompensator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmCompensatorVariants(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmFormatLogRecords(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmLogControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmMonitorClerks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrmMonitorLogRecords(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispenserDriver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispenserManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventServerTrace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetAppTrackerData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetContextProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetSecurityCallContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILBEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMTSActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMTSCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMTSLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManagedActivationEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManagedObjectInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManagedPoolAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManagedPooledObj(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageMover(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMtsEventInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMtsEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMtsGrp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectConstruct(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectConstructString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectContextActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectContextInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectContextInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectContextTip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPoolManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessInitializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityCallContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityCallersColl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityIdentityColl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectCOMLBServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISendMethodEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceComTIIntrinsicsConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceIISIntrinsicsConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceInheritanceConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServicePartitionConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServicePool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServicePoolConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceSxsConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceSynchronizationConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceSysTxnConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceThreadPoolConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceTrackerConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceTransactionConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceTransactionConfigBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPropertyGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedPropertyGroupManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemAppEventData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThreadPoolKnobs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionContextEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionProxy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionResourcePool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITxProxyHolder(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LBEvents(i32);
#[repr(transparent)]
pub struct LockModes(pub i32);
pub const LockSetGet: LockModes = LockModes(0i32);
pub const LockMethod: LockModes = LockModes(1i32);
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
#[repr(C)]
pub struct MessageMover(i32);
#[repr(C)]
pub struct MtsGrp(i32);
#[repr(transparent)]
pub struct ObjectContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ObjectControl(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PoolMgr(i32);
#[repr(C)]
pub struct RECYCLE_INFO(i32);
#[repr(transparent)]
pub struct ReleaseModes(pub i32);
pub const Standard: ReleaseModes = ReleaseModes(0i32);
pub const Process: ReleaseModes = ReleaseModes(1i32);
#[repr(C)]
pub struct SecurityCallContext(i32);
#[repr(C)]
pub struct SecurityCallers(i32);
#[repr(C)]
pub struct SecurityIdentity(i32);
#[repr(transparent)]
pub struct SecurityProperty(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ServicePool(i32);
#[repr(C)]
pub struct ServicePoolConfig(i32);
#[repr(C)]
pub struct SharedProperty(i32);
#[repr(C)]
pub struct SharedPropertyGroup(i32);
#[repr(C)]
pub struct SharedPropertyGroupManager(i32);
#[repr(transparent)]
pub struct TRACKING_COLL_TYPE(pub i32);
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(0i32);
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(1i32);
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(2i32);
#[repr(C)]
pub struct TrackerServer(i32);
#[repr(C)]
pub struct TransactionContext(i32);
#[repr(C)]
pub struct TransactionContextEx(i32);
#[repr(transparent)]
pub struct TransactionVote(pub i32);
pub const TxCommit: TransactionVote = TransactionVote(0i32);
pub const TxAbort: TransactionVote = TransactionVote(1i32);
