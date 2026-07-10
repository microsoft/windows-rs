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
pub const COMAdminCatalog: windows_core::GUID = windows_core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: windows_core::GUID = windows_core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCollectionApplicationCluster: windows_core::PCSTR = windows_core::s!("ApplicationCluster");
pub const COMAdminCollectionApplications: windows_core::PCSTR = windows_core::s!("Applications");
pub const COMAdminCollectionComponents: windows_core::PCSTR = windows_core::s!("Components");
pub const COMAdminCollectionComputerList: windows_core::PCSTR = windows_core::s!("ComputerList");
pub const COMAdminCollectionDCOMProtocols: windows_core::PCSTR = windows_core::s!("DCOMProtocols");
pub const COMAdminCollectionErrorInfo: windows_core::PCSTR = windows_core::s!("ErrorInfo");
pub const COMAdminCollectionInprocServers: windows_core::PCSTR = windows_core::s!("InprocServers");
pub const COMAdminCollectionInterfacesForComponent: windows_core::PCSTR = windows_core::s!("InterfacesForComponent");
pub const COMAdminCollectionLocalComputer: windows_core::PCSTR = windows_core::s!("LocalComputer");
pub const COMAdminCollectionMethodsForInterface: windows_core::PCSTR = windows_core::s!("MethodsForInterface");
pub const COMAdminCollectionPartitions: windows_core::PCSTR = windows_core::s!("Partitions");
pub const COMAdminCollectionPropertyInfo: windows_core::PCSTR = windows_core::s!("PropertyInfo");
pub const COMAdminCollectionRelatedCollectionInfo: windows_core::PCSTR = windows_core::s!("RelatedCollectionInfo");
pub const COMAdminCollectionRoles: windows_core::PCSTR = windows_core::s!("Roles");
pub const COMAdminCollectionRolesForComponent: windows_core::PCSTR = windows_core::s!("RolesForComponent");
pub const COMAdminCollectionRolesForInterface: windows_core::PCSTR = windows_core::s!("RolesForInterface");
pub const COMAdminCollectionRolesForMethod: windows_core::PCSTR = windows_core::s!("RolesForMethod");
pub const COMAdminCollectionRoot: windows_core::PCSTR = windows_core::s!("Root");
pub const COMAdminCollectionUsersInRole: windows_core::PCSTR = windows_core::s!("UsersInRole");
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
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICOMAdminCatalog, ICOMAdminCatalog_Vtbl, 0xdd662187_dfc2_11d1_a2cf_00805fc79235);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICOMAdminCatalog {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICOMAdminCatalog, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICOMAdminCatalog {
    pub unsafe fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Connect(&self, bstrcatalogservername: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcatalogservername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCollectionByQuery(&self, bstrcollname: &windows_core::BSTR, ppsavarquery: *const *const super::oaidl::SAFEARRAY) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionByQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), ppsavarquery, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ImportComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrclsidorprogid)) }
    }
    pub unsafe fn InstallComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrdll), core::mem::transmute_copy(bstrtlb), core::mem::transmute_copy(bstrpsdll)) }
    }
    pub unsafe fn ShutdownApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutdownApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname)) }
    }
    pub unsafe fn ExportApplication(&self, bstrapplidorname: &windows_core::BSTR, bstrapplicationfile: &windows_core::BSTR, loptions: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrapplicationfile), loptions) }
    }
    pub unsafe fn InstallApplication(&self, bstrapplicationfile: &windows_core::BSTR, bstrdestinationdirectory: &windows_core::BSTR, loptions: Option<i32>, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationfile), core::mem::transmute_copy(bstrdestinationdirectory), loptions.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute_copy(bstruserid), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrrsn)) }
    }
    pub unsafe fn StopRouter(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopRouter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RefreshRouter(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshRouter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn StartRouter(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartRouter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved1(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved2(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InstallMultipleComponents(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *const *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallMultipleComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), ppsavarfilenames, ppsavarclsids) }
    }
    pub unsafe fn GetMultipleComponentsInfo(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *mut *mut super::oaidl::SAFEARRAY, ppsavarclassnames: *mut *mut super::oaidl::SAFEARRAY, ppsavarfileflags: *mut *mut super::oaidl::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMultipleComponentsInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), ppsavarfilenames, ppsavarclsids as _, ppsavarclassnames as _, ppsavarfileflags as _, ppsavarcomponentflags as _) }
    }
    pub unsafe fn RefreshComponents(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshComponents)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackupREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackupREGDB)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupfilepath)) }
    }
    pub unsafe fn RestoreREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreREGDB)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupfilepath)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn QueryApplicationFile(&self, bstrapplicationfile: &windows_core::BSTR, pbstrapplicationname: *mut windows_core::BSTR, pbstrapplicationdescription: *mut windows_core::BSTR, pbhasusers: *mut super::wtypes::VARIANT_BOOL, pbisproxy: *mut super::wtypes::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryApplicationFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationfile), core::mem::transmute(pbstrapplicationname), core::mem::transmute(pbstrapplicationdescription), pbhasusers as _, pbisproxy as _, ppsavarfilenames as _) }
    }
    pub unsafe fn StartApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname)) }
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceCheck)(windows_core::Interface::as_raw(self), lservice, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstallMultipleEventClasses(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *const *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallMultipleEventClasses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), ppsavarfilenames, ppsavarclsids) }
    }
    pub unsafe fn InstallEventClass(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallEventClass)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrdll), core::mem::transmute_copy(bstrtlb), core::mem::transmute_copy(bstrpsdll)) }
    }
    pub unsafe fn GetEventClassesForIID(&self, bstriid: &windows_core::BSTR, ppsavarclsids: *mut *mut super::oaidl::SAFEARRAY, ppsavarprogids: *mut *mut super::oaidl::SAFEARRAY, ppsavardescriptions: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEventClassesForIID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstriid), ppsavarclsids as _, ppsavarprogids as _, ppsavardescriptions as _) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCollectionByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::oaidl::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExportApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub InstallApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallMultipleComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::oaidl::SAFEARRAY, *const *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub GetMultipleComponentsInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub RefreshComponents: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackupREGDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestoreREGDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub QueryApplicationFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL, *mut super::wtypes::VARIANT_BOOL, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    QueryApplicationFile: usize,
    pub StartApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceCheck: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub InstallMultipleEventClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::oaidl::SAFEARRAY, *const *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub InstallEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventClassesForIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICOMAdminCatalog_Impl: super::oaidl::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Connect(&self, bstrcatalogservername: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn GetCollectionByQuery(&self, bstrcollname: &windows_core::BSTR, ppsavarquery: *const *const super::oaidl::SAFEARRAY) -> windows_core::Result<super::oaidl::IDispatch>;
    fn ImportComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ShutdownApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExportApplication(&self, bstrapplidorname: &windows_core::BSTR, bstrapplicationfile: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()>;
    fn InstallApplication(&self, bstrapplicationfile: &windows_core::BSTR, bstrdestinationdirectory: &windows_core::BSTR, loptions: i32, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopRouter(&self) -> windows_core::Result<()>;
    fn RefreshRouter(&self) -> windows_core::Result<()>;
    fn StartRouter(&self) -> windows_core::Result<()>;
    fn Reserved1(&self) -> windows_core::Result<()>;
    fn Reserved2(&self) -> windows_core::Result<()>;
    fn InstallMultipleComponents(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *const *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn GetMultipleComponentsInfo(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *mut *mut super::oaidl::SAFEARRAY, ppsavarclassnames: *mut *mut super::oaidl::SAFEARRAY, ppsavarfileflags: *mut *mut super::oaidl::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn RefreshComponents(&self) -> windows_core::Result<()>;
    fn BackupREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RestoreREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryApplicationFile(&self, bstrapplicationfile: &windows_core::BSTR, pbstrapplicationname: *mut windows_core::BSTR, pbstrapplicationdescription: *mut windows_core::BSTR, pbhasusers: *mut super::wtypes::VARIANT_BOOL, pbisproxy: *mut super::wtypes::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn StartApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceCheck(&self, lservice: i32) -> windows_core::Result<i32>;
    fn InstallMultipleEventClasses(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *const *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn InstallEventClass(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetEventClassesForIID(&self, bstriid: &windows_core::BSTR, ppsavarclsids: *mut *mut super::oaidl::SAFEARRAY, ppsavarprogids: *mut *mut super::oaidl::SAFEARRAY, ppsavardescriptions: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICOMAdminCatalog_Vtbl {
    pub const fn new<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCollection<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::GetCollection(this, core::mem::transmute(&bstrcollname)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcatalogservername: *mut core::ffi::c_void, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::Connect(this, core::mem::transmute(&bstrcatalogservername)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::MajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::MinorVersion(this) {
                    Ok(ok__) => {
                        plminorversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, ppsavarquery: *const *const super::oaidl::SAFEARRAY, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::GetCollectionByQuery(this, core::mem::transmute(&bstrcollname), core::mem::transmute_copy(&ppsavarquery)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrclsidorprogid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::ImportComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrclsidorprogid)).into()
            }
        }
        unsafe extern "system" fn InstallComponent<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrdll: *mut core::ffi::c_void, bstrtlb: *mut core::ffi::c_void, bstrpsdll: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
            }
        }
        unsafe extern "system" fn ShutdownApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::ShutdownApplication(this, core::mem::transmute(&bstrapplidorname)).into()
            }
        }
        unsafe extern "system" fn ExportApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::ExportApplication(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&loptions)).into()
            }
        }
        unsafe extern "system" fn InstallApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, bstrdestinationdirectory: *mut core::ffi::c_void, loptions: i32, bstruserid: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrrsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallApplication(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute(&bstrdestinationdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
            }
        }
        unsafe extern "system" fn StopRouter<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::StopRouter(this).into()
            }
        }
        unsafe extern "system" fn RefreshRouter<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::RefreshRouter(this).into()
            }
        }
        unsafe extern "system" fn StartRouter<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::StartRouter(this).into()
            }
        }
        unsafe extern "system" fn Reserved1<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::Reserved1(this).into()
            }
        }
        unsafe extern "system" fn Reserved2<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::Reserved2(this).into()
            }
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *const *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallMultipleComponents(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
            }
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *mut *mut super::oaidl::SAFEARRAY, ppsavarclassnames: *mut *mut super::oaidl::SAFEARRAY, ppsavarfileflags: *mut *mut super::oaidl::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::GetMultipleComponentsInfo(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarclassnames), core::mem::transmute_copy(&ppsavarfileflags), core::mem::transmute_copy(&ppsavarcomponentflags)).into()
            }
        }
        unsafe extern "system" fn RefreshComponents<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::RefreshComponents(this).into()
            }
        }
        unsafe extern "system" fn BackupREGDB<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::BackupREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
            }
        }
        unsafe extern "system" fn RestoreREGDB<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::RestoreREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
            }
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, pbstrapplicationname: *mut *mut core::ffi::c_void, pbstrapplicationdescription: *mut *mut core::ffi::c_void, pbhasusers: *mut super::wtypes::VARIANT_BOOL, pbisproxy: *mut super::wtypes::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::QueryApplicationFile(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&pbstrapplicationname), core::mem::transmute_copy(&pbstrapplicationdescription), core::mem::transmute_copy(&pbhasusers), core::mem::transmute_copy(&pbisproxy), core::mem::transmute_copy(&ppsavarfilenames)).into()
            }
        }
        unsafe extern "system" fn StartApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::StartApplication(this, core::mem::transmute(&bstrapplidorname)).into()
            }
        }
        unsafe extern "system" fn ServiceCheck<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::ServiceCheck(this, core::mem::transmute_copy(&lservice)) {
                    Ok(ok__) => {
                        plstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, ppsavarfilenames: *const *const super::oaidl::SAFEARRAY, ppsavarclsids: *const *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallMultipleEventClasses(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
            }
        }
        unsafe extern "system" fn InstallEventClass<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrdll: *mut core::ffi::c_void, bstrtlb: *mut core::ffi::c_void, bstrpsdll: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallEventClass(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
            }
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstriid: *mut core::ffi::c_void, ppsavarclsids: *mut *mut super::oaidl::SAFEARRAY, ppsavarprogids: *mut *mut super::oaidl::SAFEARRAY, ppsavardescriptions: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::GetEventClassesForIID(this, core::mem::transmute(&bstriid), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarprogids), core::mem::transmute_copy(&ppsavardescriptions)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetCollection: GetCollection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Identity, OFFSET>,
            ImportComponent: ImportComponent::<Identity, OFFSET>,
            InstallComponent: InstallComponent::<Identity, OFFSET>,
            ShutdownApplication: ShutdownApplication::<Identity, OFFSET>,
            ExportApplication: ExportApplication::<Identity, OFFSET>,
            InstallApplication: InstallApplication::<Identity, OFFSET>,
            StopRouter: StopRouter::<Identity, OFFSET>,
            RefreshRouter: RefreshRouter::<Identity, OFFSET>,
            StartRouter: StartRouter::<Identity, OFFSET>,
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Identity, OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Identity, OFFSET>,
            RefreshComponents: RefreshComponents::<Identity, OFFSET>,
            BackupREGDB: BackupREGDB::<Identity, OFFSET>,
            RestoreREGDB: RestoreREGDB::<Identity, OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Identity, OFFSET>,
            StartApplication: StartApplication::<Identity, OFFSET>,
            ServiceCheck: ServiceCheck::<Identity, OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Identity, OFFSET>,
            InstallEventClass: InstallEventClass::<Identity, OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICOMAdminCatalog {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICOMAdminCatalog2, ICOMAdminCatalog2_Vtbl, 0x790c6e0b_9194_4cc9_9426_a48a63185696);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICOMAdminCatalog2 {
    type Target = ICOMAdminCatalog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICOMAdminCatalog2, windows_core::IUnknown, super::oaidl::IDispatch, ICOMAdminCatalog);
#[cfg(feature = "oaidl")]
impl ICOMAdminCatalog2 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCollectionByQuery2(&self, bstrcollectionname: &windows_core::BSTR, pvarquerystrings: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionByQuery2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollectionname), core::mem::transmute(pvarquerystrings), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicationInstanceIDFromProcessID)(windows_core::Interface::as_raw(self), lprocessid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutdownApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PauseApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResumeApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT, lreasoncode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecycleApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid), lreasoncode) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreApplicationInstancesPaused)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &windows_core::BSTR, bstrdirectory: &windows_core::BSTR, lmaximages: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpApplicationInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationinstanceid), core::mem::transmute_copy(bstrdirectory), lmaximages, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsApplicationInstanceDumpSupported(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsApplicationInstanceDumpSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CreateServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR, bstrservicename: &windows_core::BSTR, bstrstarttype: &windows_core::BSTR, bstrerrorcontrol: &windows_core::BSTR, bstrdependencies: &windows_core::BSTR, bstrrunas: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bdesktopok: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateServiceForApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute_copy(bstrservicename), core::mem::transmute_copy(bstrstarttype), core::mem::transmute_copy(bstrerrorcontrol), core::mem::transmute_copy(bstrdependencies), core::mem::transmute_copy(bstrrunas), core::mem::transmute_copy(bstrpassword), bdesktopok) }
    }
    pub unsafe fn DeleteServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteServiceForApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname)) }
    }
    pub unsafe fn GetPartitionID(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartitionID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPartitionName(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartitionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCurrentPartition(&self, bstrpartitionidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPartition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpartitionidorname)) }
    }
    pub unsafe fn CurrentPartitionID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPartitionID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentPartitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPartitionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GlobalPartitionID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GlobalPartitionID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FlushPartitionCache(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlushPartitionCache)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CopyApplications(&self, bstrsourcepartitionidorname: &windows_core::BSTR, pvarapplicationid: *const super::oaidl::VARIANT, bstrdestinationpartitionidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyApplications)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsourcepartitionidorname), core::mem::transmute(pvarapplicationid), core::mem::transmute_copy(bstrdestinationpartitionidorname)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CopyComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsourceapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute_copy(bstrdestinationapplicationidorname)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn MoveComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsourceapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute_copy(bstrdestinationapplicationidorname)) }
    }
    pub unsafe fn AliasComponent(&self, bstrsrcapplicationidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR, bstrdestapplicationidorname: &windows_core::BSTR, bstrnewprogid: &windows_core::BSTR, bstrnewclsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AliasComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsrcapplicationidorname), core::mem::transmute_copy(bstrclsidorprogid), core::mem::transmute_copy(bstrdestapplicationidorname), core::mem::transmute_copy(bstrnewprogid), core::mem::transmute_copy(bstrnewclsid)) }
    }
    pub unsafe fn IsSafeToDelete(&self, bstrdllname: &windows_core::BSTR) -> windows_core::Result<COMAdminInUse> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSafeToDelete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdllname), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportUnconfiguredComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute(pvarclsidorprogid), pvarcomponenttype.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PromoteUnconfiguredComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute(pvarclsidorprogid), pvarcomponenttype.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute(pvarclsidorprogid), pvarcomponenttype.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Is64BitCatalogServer(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Is64BitCatalogServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExportPartition(&self, bstrpartitionidorname: &windows_core::BSTR, bstrpartitionfilename: &windows_core::BSTR, loptions: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportPartition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpartitionidorname), core::mem::transmute_copy(bstrpartitionfilename), loptions) }
    }
    pub unsafe fn InstallPartition(&self, bstrfilename: &windows_core::BSTR, bstrdestdirectory: &windows_core::BSTR, loptions: i32, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallPartition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), core::mem::transmute_copy(bstrdestdirectory), loptions, core::mem::transmute_copy(bstruserid), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrrsn)) }
    }
    pub unsafe fn QueryApplicationFile2(&self, bstrapplicationfile: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryApplicationFile2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationfile), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetComponentVersionCount(&self, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentVersionCount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclsidorprogid), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog2_Vtbl {
    pub base__: ICOMAdminCatalog_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetCollectionByQuery2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetCollectionByQuery2: usize,
    pub GetApplicationInstanceIDFromProcessID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ShutdownApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ShutdownApplicationInstances: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub PauseApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    PauseApplicationInstances: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ResumeApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ResumeApplicationInstances: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub RecycleApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    RecycleApplicationInstances: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AreApplicationInstancesPaused: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AreApplicationInstancesPaused: usize,
    pub DumpApplicationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsApplicationInstanceDumpSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsApplicationInstanceDumpSupported: usize,
    #[cfg(feature = "wtypes")]
    pub CreateServiceForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CreateServiceForApplication: usize,
    pub DeleteServiceForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCurrentPartition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentPartitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GlobalPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushPartitionCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub CopyApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    CopyApplications: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub CopyComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    CopyComponents: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub MoveComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    MoveComponents: usize,
    pub AliasComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSafeToDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut COMAdminInUse) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ImportUnconfiguredComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ImportUnconfiguredComponents: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub PromoteUnconfiguredComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    PromoteUnconfiguredComponents: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ImportComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ImportComponents: usize,
    #[cfg(feature = "wtypes")]
    pub Is64BitCatalogServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Is64BitCatalogServer: usize,
    pub ExportPartition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub InstallPartition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryApplicationFile2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentVersionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICOMAdminCatalog2_Impl: ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(&self, bstrcollectionname: &windows_core::BSTR, pvarquerystrings: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT, lreasoncode: i32) -> windows_core::Result<()>;
    fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &windows_core::BSTR, bstrdirectory: &windows_core::BSTR, lmaximages: i32) -> windows_core::Result<windows_core::BSTR>;
    fn IsApplicationInstanceDumpSupported(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CreateServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR, bstrservicename: &windows_core::BSTR, bstrstarttype: &windows_core::BSTR, bstrerrorcontrol: &windows_core::BSTR, bstrdependencies: &windows_core::BSTR, bstrrunas: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bdesktopok: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPartitionID(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetPartitionName(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentPartition(&self, bstrpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CurrentPartitionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentPartitionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GlobalPartitionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FlushPartitionCache(&self) -> windows_core::Result<()>;
    fn CopyApplications(&self, bstrsourcepartitionidorname: &windows_core::BSTR, pvarapplicationid: *const super::oaidl::VARIANT, bstrdestinationpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MoveComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AliasComponent(&self, bstrsrcapplicationidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR, bstrdestapplicationidorname: &windows_core::BSTR, bstrnewprogid: &windows_core::BSTR, bstrnewclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSafeToDelete(&self, bstrdllname: &windows_core::BSTR) -> windows_core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ImportComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Is64BitCatalogServer(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn ExportPartition(&self, bstrpartitionidorname: &windows_core::BSTR, bstrpartitionfilename: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()>;
    fn InstallPartition(&self, bstrfilename: &windows_core::BSTR, bstrdestdirectory: &windows_core::BSTR, loptions: i32, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryApplicationFile2(&self, bstrapplicationfile: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn GetComponentVersionCount(&self, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICOMAdminCatalog2_Vtbl {
    pub const fn new<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollectionname: *mut core::ffi::c_void, pvarquerystrings: *const super::oaidl::VARIANT, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetCollectionByQuery2(this, core::mem::transmute(&bstrcollectionname), core::mem::transmute_copy(&pvarquerystrings)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetApplicationInstanceIDFromProcessID(this, core::mem::transmute_copy(&lprocessid)) {
                    Ok(ok__) => {
                        pbstrapplicationinstanceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ShutdownApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
            }
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::PauseApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
            }
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ResumeApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
            }
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::oaidl::VARIANT, lreasoncode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::RecycleApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid), core::mem::transmute_copy(&lreasoncode)).into()
            }
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::oaidl::VARIANT, pvarboolpaused: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::AreApplicationInstancesPaused(this, core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                    Ok(ok__) => {
                        pvarboolpaused.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationinstanceid: *mut core::ffi::c_void, bstrdirectory: *mut core::ffi::c_void, lmaximages: i32, pbstrdumpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::DumpApplicationInstance(this, core::mem::transmute(&bstrapplicationinstanceid), core::mem::transmute(&bstrdirectory), core::mem::transmute_copy(&lmaximages)) {
                    Ok(ok__) => {
                        pbstrdumpfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbooldumpsupported: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::IsApplicationInstanceDumpSupported(this) {
                    Ok(ok__) => {
                        pvarbooldumpsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, bstrservicename: *mut core::ffi::c_void, bstrstarttype: *mut core::ffi::c_void, bstrerrorcontrol: *mut core::ffi::c_void, bstrdependencies: *mut core::ffi::c_void, bstrrunas: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bdesktopok: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::CreateServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute(&bstrservicename), core::mem::transmute(&bstrstarttype), core::mem::transmute(&bstrerrorcontrol), core::mem::transmute(&bstrdependencies), core::mem::transmute(&bstrrunas), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&bdesktopok)).into()
            }
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::DeleteServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname)).into()
            }
        }
        unsafe extern "system" fn GetPartitionID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pbstrpartitionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetPartitionID(this, core::mem::transmute(&bstrapplicationidorname)) {
                    Ok(ok__) => {
                        pbstrpartitionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartitionName<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pbstrpartitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetPartitionName(this, core::mem::transmute(&bstrapplicationidorname)) {
                    Ok(ok__) => {
                        pbstrpartitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::SetCurrentPartition(this, core::mem::transmute(&bstrpartitionidorname)).into()
            }
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::CurrentPartitionID(this) {
                    Ok(ok__) => {
                        pbstrpartitionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::CurrentPartitionName(this) {
                    Ok(ok__) => {
                        pbstrpartitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrglobalpartitionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GlobalPartitionID(this) {
                    Ok(ok__) => {
                        pbstrglobalpartitionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::FlushPartitionCache(this).into()
            }
        }
        unsafe extern "system" fn CopyApplications<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourcepartitionidorname: *mut core::ffi::c_void, pvarapplicationid: *const super::oaidl::VARIANT, bstrdestinationpartitionidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::CopyApplications(this, core::mem::transmute(&bstrsourcepartitionidorname), core::mem::transmute_copy(&pvarapplicationid), core::mem::transmute(&bstrdestinationpartitionidorname)).into()
            }
        }
        unsafe extern "system" fn CopyComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::oaidl::VARIANT, bstrdestinationapplicationidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::CopyComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
            }
        }
        unsafe extern "system" fn MoveComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::oaidl::VARIANT, bstrdestinationapplicationidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::MoveComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
            }
        }
        unsafe extern "system" fn AliasComponent<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsrcapplicationidorname: *mut core::ffi::c_void, bstrclsidorprogid: *mut core::ffi::c_void, bstrdestapplicationidorname: *mut core::ffi::c_void, bstrnewprogid: *mut core::ffi::c_void, bstrnewclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::AliasComponent(this, core::mem::transmute(&bstrsrcapplicationidorname), core::mem::transmute(&bstrclsidorprogid), core::mem::transmute(&bstrdestapplicationidorname), core::mem::transmute(&bstrnewprogid), core::mem::transmute(&bstrnewclsid)).into()
            }
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllname: *mut core::ffi::c_void, pcomadmininuse: *mut COMAdminInUse) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::IsSafeToDelete(this, core::mem::transmute(&bstrdllname)) {
                    Ok(ok__) => {
                        pcomadmininuse.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ImportUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
            }
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::PromoteUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
            }
        }
        unsafe extern "system" fn ImportComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::oaidl::VARIANT, pvarcomponenttype: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ImportComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
            }
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbis64bit: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::Is64BitCatalogServer(this) {
                    Ok(ok__) => {
                        pbis64bit.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportPartition<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: *mut core::ffi::c_void, bstrpartitionfilename: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ExportPartition(this, core::mem::transmute(&bstrpartitionidorname), core::mem::transmute(&bstrpartitionfilename), core::mem::transmute_copy(&loptions)).into()
            }
        }
        unsafe extern "system" fn InstallPartition<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, bstrdestdirectory: *mut core::ffi::c_void, loptions: i32, bstruserid: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrrsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::InstallPartition(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrdestdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
            }
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, ppfilesforimport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::QueryApplicationFile2(this, core::mem::transmute(&bstrapplicationfile)) {
                    Ok(ok__) => {
                        ppfilesforimport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsidorprogid: *mut core::ffi::c_void, plversioncount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetComponentVersionCount(this, core::mem::transmute(&bstrclsidorprogid)) {
                    Ok(ok__) => {
                        plversioncount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICOMAdminCatalog_Vtbl::new::<Identity, OFFSET>(),
            GetCollectionByQuery2: GetCollectionByQuery2::<Identity, OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Identity, OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Identity, OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Identity, OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Identity, OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Identity, OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Identity, OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Identity, OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Identity, OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Identity, OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Identity, OFFSET>,
            GetPartitionID: GetPartitionID::<Identity, OFFSET>,
            GetPartitionName: GetPartitionName::<Identity, OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Identity, OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Identity, OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Identity, OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Identity, OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Identity, OFFSET>,
            CopyApplications: CopyApplications::<Identity, OFFSET>,
            CopyComponents: CopyComponents::<Identity, OFFSET>,
            MoveComponents: MoveComponents::<Identity, OFFSET>,
            AliasComponent: AliasComponent::<Identity, OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Identity, OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Identity, OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Identity, OFFSET>,
            ImportComponents: ImportComponents::<Identity, OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Identity, OFFSET>,
            ExportPartition: ExportPartition::<Identity, OFFSET>,
            InstallPartition: InstallPartition::<Identity, OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Identity, OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICOMAdminCatalog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICOMAdminCatalog2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICatalogCollection, ICatalogCollection_Vtbl, 0x6eb22872_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICatalogCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICatalogCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICatalogCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, lindex: i32) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Remove(&self, lindex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lindex) }
    }
    pub unsafe fn Add(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Populate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Populate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCollection(&self, bstrcollname: &windows_core::BSTR, varobjectkey: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), core::mem::transmute_copy(varobjectkey), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Name(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AddEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RemoveEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoveEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUtilInterface(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUtilInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DataStoreMajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataStoreMajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataStoreMinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataStoreMinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PopulateByKey(&self, psakeys: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PopulateByKey)(windows_core::Interface::as_raw(self), psakeys) }
    }
    pub unsafe fn PopulateByQuery(&self, bstrquerystring: &windows_core::BSTR, lquerytype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PopulateByQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrquerystring), lquerytype) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Populate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetCollection: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Name: usize,
    #[cfg(feature = "wtypes")]
    pub AddEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AddEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub RemoveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RemoveEnabled: usize,
    pub GetUtilInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataStoreMajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataStoreMinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PopulateByKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub PopulateByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICatalogCollection_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, lindex: i32) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn Add(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Populate(&self) -> windows_core::Result<()>;
    fn SaveChanges(&self) -> windows_core::Result<i32>;
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR, varobjectkey: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Name(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn AddEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn RemoveEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn GetUtilInterface(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn DataStoreMajorVersion(&self) -> windows_core::Result<i32>;
    fn DataStoreMinorVersion(&self) -> windows_core::Result<i32>;
    fn PopulateByKey(&self, psakeys: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn PopulateByQuery(&self, bstrquerystring: &windows_core::BSTR, lquerytype: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICatalogCollection_Vtbl {
    pub const fn new<Identity: ICatalogCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppcatalogobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plobjectcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        plobjectcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Add(this) {
                    Ok(ok__) => {
                        ppcatalogobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Populate<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::Populate(this).into()
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchanges: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::SaveChanges(this) {
                    Ok(ok__) => {
                        pcchanges.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCollection<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, varobjectkey: super::oaidl::VARIANT, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::GetCollection(this, core::mem::transmute(&bstrcollname), core::mem::transmute(&varobjectkey)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarnamel: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Name(this) {
                    Ok(ok__) => {
                        pvarnamel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEnabled<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::AddEnabled(this) {
                    Ok(ok__) => {
                        pvarbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEnabled<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::RemoveEnabled(this) {
                    Ok(ok__) => {
                        pvarbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUtilInterface<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::GetUtilInterface(this) {
                    Ok(ok__) => {
                        ppidispatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::DataStoreMajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversionl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::DataStoreMinorVersion(this) {
                    Ok(ok__) => {
                        plminorversionl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PopulateByKey<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psakeys: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::PopulateByKey(this, core::mem::transmute_copy(&psakeys)).into()
            }
        }
        unsafe extern "system" fn PopulateByQuery<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquerystring: *mut core::ffi::c_void, lquerytype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::PopulateByQuery(this, core::mem::transmute(&bstrquerystring), core::mem::transmute_copy(&lquerytype)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Populate: Populate::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            GetCollection: GetCollection::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            AddEnabled: AddEnabled::<Identity, OFFSET>,
            RemoveEnabled: RemoveEnabled::<Identity, OFFSET>,
            GetUtilInterface: GetUtilInterface::<Identity, OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Identity, OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Identity, OFFSET>,
            PopulateByKey: PopulateByKey::<Identity, OFFSET>,
            PopulateByQuery: PopulateByQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICatalogCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICatalogObject, ICatalogObject_Vtbl, 0x6eb22871_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICatalogObject {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICatalogObject, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICatalogObject {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Value(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, bstrpropname: &windows_core::BSTR, val: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), core::mem::transmute_copy(val)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Key(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Name(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsPropertyReadOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPropertyReadOnly)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Valid(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Valid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsPropertyWriteOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPropertyWriteOnly)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogObject_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Value: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Key: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Name: usize,
    #[cfg(feature = "wtypes")]
    pub IsPropertyReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsPropertyReadOnly: usize,
    #[cfg(feature = "wtypes")]
    pub Valid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Valid: usize,
    #[cfg(feature = "wtypes")]
    pub IsPropertyWriteOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsPropertyWriteOnly: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICatalogObject_Impl: super::oaidl::IDispatch_Impl {
    fn Value(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetValue(&self, bstrpropname: &windows_core::BSTR, val: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Key(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Name(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn IsPropertyReadOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Valid(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn IsPropertyWriteOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICatalogObject_Vtbl {
    pub const fn new<Identity: ICatalogObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pvarretval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Value(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pvarretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, val: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogObject_Impl::SetValue(this, core::mem::transmute(&bstrpropname), core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn Key<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Key(this) {
                    Ok(ok__) => {
                        pvarretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Name(this) {
                    Ok(ok__) => {
                        pvarretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pbretval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::IsPropertyReadOnly(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pbretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Valid<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbretval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Valid(this) {
                    Ok(ok__) => {
                        pbretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pbretval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::IsPropertyWriteOnly(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pbretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Key: Key::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Identity, OFFSET>,
            Valid: Valid::<Identity, OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogObject as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICatalogObject {}
