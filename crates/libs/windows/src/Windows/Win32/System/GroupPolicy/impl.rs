pub trait IGPEInformationImpl: Sized {
    fn GetName();
    fn GetDisplayName();
    fn GetRegistryKey();
    fn GetDSPath();
    fn GetFileSysPath();
    fn GetOptions();
    fn GetType();
    fn GetHint();
    fn PolicyChanged();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMImpl: Sized + IDispatchImpl {
    fn GetDomain();
    fn GetBackupDir();
    fn GetSitesContainer();
    fn GetRSOP();
    fn CreatePermission();
    fn CreateSearchCriteria();
    fn CreateTrustee();
    fn GetClientSideExtensions();
    fn GetConstants();
    fn GetMigrationTable();
    fn CreateMigrationTable();
    fn InitializeReporting();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPM2Impl: Sized + IGPMImpl + IDispatchImpl {
    fn GetBackupDirEx();
    fn InitializeReportingEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMAsyncCancelImpl: Sized + IDispatchImpl {
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMAsyncProgressImpl: Sized + IDispatchImpl {
    fn Status();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackupImpl: Sized + IDispatchImpl {
    fn ID();
    fn GPOID();
    fn GPODomain();
    fn GPODisplayName();
    fn Timestamp();
    fn Comment();
    fn BackupDir();
    fn Delete();
    fn GenerateReport();
    fn GenerateReportToFile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackupCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackupDirImpl: Sized + IDispatchImpl {
    fn BackupDirectory();
    fn GetBackup();
    fn SearchBackups();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackupDirExImpl: Sized + IDispatchImpl {
    fn BackupDir();
    fn BackupType();
    fn GetBackup();
    fn SearchBackups();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMCSECollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMClientSideExtensionImpl: Sized + IDispatchImpl {
    fn ID();
    fn DisplayName();
    fn IsUserEnabled();
    fn IsComputerEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMConstantsImpl: Sized + IDispatchImpl {
    fn PermGPOApply();
    fn PermGPORead();
    fn PermGPOEdit();
    fn PermGPOEditSecurityAndDelete();
    fn PermGPOCustom();
    fn PermWMIFilterEdit();
    fn PermWMIFilterFullControl();
    fn PermWMIFilterCustom();
    fn PermSOMLink();
    fn PermSOMLogging();
    fn PermSOMPlanning();
    fn PermSOMGPOCreate();
    fn PermSOMWMICreate();
    fn PermSOMWMIFullControl();
    fn SearchPropertyGPOPermissions();
    fn SearchPropertyGPOEffectivePermissions();
    fn SearchPropertyGPODisplayName();
    fn SearchPropertyGPOWMIFilter();
    fn SearchPropertyGPOID();
    fn SearchPropertyGPOComputerExtensions();
    fn SearchPropertyGPOUserExtensions();
    fn SearchPropertySOMLinks();
    fn SearchPropertyGPODomain();
    fn SearchPropertyBackupMostRecent();
    fn SearchOpEquals();
    fn SearchOpContains();
    fn SearchOpNotContains();
    fn SearchOpNotEquals();
    fn UsePDC();
    fn UseAnyDC();
    fn DoNotUseW2KDC();
    fn SOMSite();
    fn SOMDomain();
    fn SOMOU();
    fn SecurityFlags();
    fn DoNotValidateDC();
    fn ReportHTML();
    fn ReportXML();
    fn RSOPModeUnknown();
    fn RSOPModePlanning();
    fn RSOPModeLogging();
    fn EntryTypeUser();
    fn EntryTypeComputer();
    fn EntryTypeLocalGroup();
    fn EntryTypeGlobalGroup();
    fn EntryTypeUniversalGroup();
    fn EntryTypeUNCPath();
    fn EntryTypeUnknown();
    fn DestinationOptionSameAsSource();
    fn DestinationOptionNone();
    fn DestinationOptionByRelativeName();
    fn DestinationOptionSet();
    fn MigrationTableOnly();
    fn ProcessSecurity();
    fn RsopLoggingNoComputer();
    fn RsopLoggingNoUser();
    fn RsopPlanningAssumeSlowLink();
    fn RsopPlanningLoopbackOption();
    fn RsopPlanningAssumeUserWQLFilterTrue();
    fn RsopPlanningAssumeCompWQLFilterTrue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMConstants2Impl: Sized + IGPMConstantsImpl + IDispatchImpl {
    fn BackupTypeGPO();
    fn BackupTypeStarterGPO();
    fn StarterGPOTypeSystem();
    fn StarterGPOTypeCustom();
    fn SearchPropertyStarterGPOPermissions();
    fn SearchPropertyStarterGPOEffectivePermissions();
    fn SearchPropertyStarterGPODisplayName();
    fn SearchPropertyStarterGPOID();
    fn SearchPropertyStarterGPODomain();
    fn PermStarterGPORead();
    fn PermStarterGPOEdit();
    fn PermStarterGPOFullControl();
    fn PermStarterGPOCustom();
    fn ReportLegacy();
    fn ReportComments();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMDomainImpl: Sized + IDispatchImpl {
    fn DomainController();
    fn Domain();
    fn CreateGPO();
    fn GetGPO();
    fn SearchGPOs();
    fn RestoreGPO();
    fn GetSOM();
    fn SearchSOMs();
    fn GetWMIFilter();
    fn SearchWMIFilters();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMDomain2Impl: Sized + IGPMDomainImpl + IDispatchImpl {
    fn CreateStarterGPO();
    fn CreateGPOFromStarterGPO();
    fn GetStarterGPO();
    fn SearchStarterGPOs();
    fn LoadStarterGPO();
    fn RestoreStarterGPO();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMDomain3Impl: Sized + IGPMDomain2Impl + IGPMDomainImpl + IDispatchImpl {
    fn GenerateReport();
    fn InfrastructureDC();
    fn SetInfrastructureDC();
    fn SetInfrastructureFlags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPOImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn SetDisplayName();
    fn Path();
    fn ID();
    fn DomainName();
    fn CreationTime();
    fn ModificationTime();
    fn UserDSVersionNumber();
    fn ComputerDSVersionNumber();
    fn UserSysvolVersionNumber();
    fn ComputerSysvolVersionNumber();
    fn GetWMIFilter();
    fn SetWMIFilter();
    fn SetUserEnabled();
    fn SetComputerEnabled();
    fn IsUserEnabled();
    fn IsComputerEnabled();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
    fn Delete();
    fn Backup();
    fn Import();
    fn GenerateReport();
    fn GenerateReportToFile();
    fn CopyTo();
    fn SetSecurityDescriptor();
    fn GetSecurityDescriptor();
    fn IsACLConsistent();
    fn MakeACLConsistent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPO2Impl: Sized + IGPMGPOImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPO3Impl: Sized + IGPMGPO2Impl + IGPMGPOImpl + IDispatchImpl {
    fn InfrastructureDC();
    fn SetInfrastructureDC();
    fn SetInfrastructureFlags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPOCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPOLinkImpl: Sized + IDispatchImpl {
    fn GPOID();
    fn GPODomain();
    fn Enabled();
    fn SetEnabled();
    fn Enforced();
    fn SetEnforced();
    fn SOMLinkOrder();
    fn SOM();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPOLinksCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMMapEntryImpl: Sized + IDispatchImpl {
    fn Source();
    fn Destination();
    fn DestinationOption();
    fn EntryType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMMapEntryCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMMigrationTableImpl: Sized + IDispatchImpl {
    fn Save();
    fn Add();
    fn AddEntry();
    fn GetEntry();
    fn DeleteEntry();
    fn UpdateDestination();
    fn Validate();
    fn GetEntries();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMPermissionImpl: Sized + IDispatchImpl {
    fn Inherited();
    fn Inheritable();
    fn Denied();
    fn Permission();
    fn Trustee();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMRSOPImpl: Sized + IDispatchImpl {
    fn Mode();
    fn Namespace();
    fn SetLoggingComputer();
    fn LoggingComputer();
    fn SetLoggingUser();
    fn LoggingUser();
    fn SetLoggingFlags();
    fn LoggingFlags();
    fn SetPlanningFlags();
    fn PlanningFlags();
    fn SetPlanningDomainController();
    fn PlanningDomainController();
    fn SetPlanningSiteName();
    fn PlanningSiteName();
    fn SetPlanningUser();
    fn PlanningUser();
    fn SetPlanningUserSOM();
    fn PlanningUserSOM();
    fn SetPlanningUserWMIFilters();
    fn PlanningUserWMIFilters();
    fn SetPlanningUserSecurityGroups();
    fn PlanningUserSecurityGroups();
    fn SetPlanningComputer();
    fn PlanningComputer();
    fn SetPlanningComputerSOM();
    fn PlanningComputerSOM();
    fn SetPlanningComputerWMIFilters();
    fn PlanningComputerWMIFilters();
    fn SetPlanningComputerSecurityGroups();
    fn PlanningComputerSecurityGroups();
    fn LoggingEnumerateUsers();
    fn CreateQueryResults();
    fn ReleaseQueryResults();
    fn GenerateReport();
    fn GenerateReportToFile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMResultImpl: Sized + IDispatchImpl {
    fn Status();
    fn Result();
    fn OverallStatus();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSOMImpl: Sized + IDispatchImpl {
    fn GPOInheritanceBlocked();
    fn SetGPOInheritanceBlocked();
    fn Name();
    fn Path();
    fn CreateGPOLink();
    fn Type();
    fn GetGPOLinks();
    fn GetInheritedGPOLinks();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSOMCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSearchCriteriaImpl: Sized + IDispatchImpl {
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSecurityInfoImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn RemoveTrustee();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSitesContainerImpl: Sized + IDispatchImpl {
    fn DomainController();
    fn Domain();
    fn Forest();
    fn GetSite();
    fn SearchSites();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStarterGPOImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn SetDisplayName();
    fn Description();
    fn SetDescription();
    fn Author();
    fn Product();
    fn CreationTime();
    fn ID();
    fn ModifiedTime();
    fn Type();
    fn ComputerVersion();
    fn UserVersion();
    fn StarterGPOVersion();
    fn Delete();
    fn Save();
    fn Backup();
    fn CopyTo();
    fn GenerateReport();
    fn GenerateReportToFile();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStarterGPOBackupImpl: Sized + IDispatchImpl {
    fn BackupDir();
    fn Comment();
    fn DisplayName();
    fn Domain();
    fn StarterGPOID();
    fn ID();
    fn Timestamp();
    fn Type();
    fn Delete();
    fn GenerateReport();
    fn GenerateReportToFile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStarterGPOBackupCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStarterGPOCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStatusMessageImpl: Sized + IDispatchImpl {
    fn ObjectPath();
    fn ErrorCode();
    fn ExtensionName();
    fn SettingsName();
    fn OperationCode();
    fn Message();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStatusMsgCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMTrusteeImpl: Sized + IDispatchImpl {
    fn TrusteeSid();
    fn TrusteeName();
    fn TrusteeDomain();
    fn TrusteeDSPath();
    fn TrusteeType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMWMIFilterImpl: Sized + IDispatchImpl {
    fn Path();
    fn SetName();
    fn Name();
    fn SetDescription();
    fn Description();
    fn GetQueryList();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMWMIFilterCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
pub trait IGroupPolicyObjectImpl: Sized {
    fn New();
    fn OpenDSGPO();
    fn OpenLocalMachineGPO();
    fn OpenRemoteMachineGPO();
    fn Save();
    fn Delete();
    fn GetName();
    fn GetDisplayName();
    fn SetDisplayName();
    fn GetPath();
    fn GetDSPath();
    fn GetFileSysPath();
    fn GetRegistryKey();
    fn GetOptions();
    fn SetOptions();
    fn GetType();
    fn GetMachineName();
    fn GetPropertySheetPages();
}
pub trait IRSOPInformationImpl: Sized {
    fn GetNamespace();
    fn GetFlags();
    fn GetEventLogEntryText();
}
