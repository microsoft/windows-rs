pub trait IAssemblyCacheImpl: Sized {
    fn UninstallAssembly();
    fn QueryAssemblyInfo();
    fn CreateAssemblyCacheItem();
    fn Reserved();
    fn InstallAssembly();
}
pub trait IAssemblyCacheItemImpl: Sized {
    fn CreateStream();
    fn Commit();
    fn AbortItem();
}
pub trait IAssemblyNameImpl: Sized {
    fn SetProperty();
    fn GetProperty();
    fn Finalize();
    fn GetDisplayName();
    fn Reserved();
    fn GetName();
    fn GetVersion();
    fn IsEqual();
    fn Clone();
}
pub trait IEnumMsmDependencyImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumMsmErrorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumMsmStringImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmDependenciesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmDependencyImpl: Sized + IDispatchImpl {
    fn Module();
    fn Language();
    fn Version();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmErrorImpl: Sized + IDispatchImpl {
    fn Type();
    fn Path();
    fn Language();
    fn DatabaseTable();
    fn DatabaseKeys();
    fn ModuleTable();
    fn ModuleKeys();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmErrorsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmGetFilesImpl: Sized + IDispatchImpl {
    fn ModuleFiles();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmMergeImpl: Sized + IDispatchImpl {
    fn OpenDatabase();
    fn OpenModule();
    fn CloseDatabase();
    fn CloseModule();
    fn OpenLog();
    fn CloseLog();
    fn Log();
    fn Errors();
    fn Dependencies();
    fn Merge();
    fn Connect();
    fn ExtractCAB();
    fn ExtractFiles();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmStringsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
pub trait IPMApplicationInfoImpl: Sized {
    fn ProductID();
    fn InstanceID();
    fn OfferID();
    fn DefaultTask();
    fn AppTitle();
    fn IconPath();
    fn NotificationState();
    fn AppInstallType();
    fn State();
    fn IsRevoked();
    fn UpdateAvailable();
    fn InstallDate();
    fn IsUninstallable();
    fn IsThemable();
    fn IsTrial();
    fn InstallPath();
    fn DataRoot();
    fn Genre();
    fn Publisher();
    fn Author();
    fn Description();
    fn Version();
    fn InvocationInfo();
    fn AppPlatMajorVersion();
    fn AppPlatMinorVersion();
    fn PublisherID();
    fn IsMultiCore();
    fn SID();
    fn AppPlatMajorVersionLightUp();
    fn AppPlatMinorVersionLightUp();
    fn set_UpdateAvailable();
    fn set_NotificationState();
    fn set_IconPath();
    fn set_UninstallableState();
    fn IsPinableOnKidZone();
    fn IsOriginallyPreInstalled();
    fn IsInstallOnSD();
    fn IsOptoutOnSD();
    fn IsOptoutBackupRestore();
    fn set_EnterpriseDisabled();
    fn set_EnterpriseUninstallable();
    fn EnterpriseDisabled();
    fn EnterpriseUninstallable();
    fn IsVisibleOnAppList();
    fn IsInboxApp();
    fn StorageID();
    fn StartAppBlob();
    fn IsMovable();
    fn DeploymentAppEnumerationHubFilter();
    fn ModifiedDate();
    fn IsOriginallyRestored();
    fn ShouldDeferMdilBind();
    fn IsFullyPreInstall();
    fn set_IsMdilMaintenanceNeeded();
    fn set_Title();
}
pub trait IPMApplicationInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMBackgroundServiceAgentInfoImpl: Sized {
    fn ProductID();
    fn TaskID();
    fn BSAID();
    fn BGSpecifier();
    fn BGName();
    fn BGSource();
    fn BGType();
    fn IsPeriodic();
    fn IsScheduled();
    fn IsScheduleAllowed();
    fn Description();
    fn IsLaunchOnBoot();
    fn set_IsScheduled();
    fn set_IsScheduleAllowed();
}
pub trait IPMBackgroundServiceAgentInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMBackgroundWorkerInfoImpl: Sized {
    fn ProductID();
    fn TaskID();
    fn BGName();
    fn MaxStartupLatency();
    fn ExpectedRuntime();
    fn IsBootWorker();
}
pub trait IPMBackgroundWorkerInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMDeploymentManagerImpl: Sized {
    fn ReportDownloadBegin();
    fn ReportDownloadProgress();
    fn ReportDownloadComplete();
    fn BeginInstall();
    fn BeginUpdate();
    fn BeginDeployPackage();
    fn BeginUpdateDeployedPackageLegacy();
    fn BeginUninstall();
    fn BeginEnterpriseAppInstall();
    fn BeginEnterpriseAppUpdate();
    fn BeginUpdateLicense();
    fn GetLicenseChallenge();
    fn GetLicenseChallengeByProductID();
    fn GetLicenseChallengeByProductID2();
    fn RevokeLicense();
    fn RebindMdilBinaries();
    fn RebindAllMdilBinaries();
    fn RegenerateXbf();
    fn GenerateXbfForCurrentLocale();
    fn BeginProvision();
    fn BeginDeprovision();
    fn ReindexSQLCEDatabases();
    fn SetApplicationsNeedMaintenance();
    fn UpdateChamberProfile();
    fn EnterprisePolicyIsApplicationAllowed();
    fn BeginUpdateDeployedPackage();
    fn ReportRestoreCancelled();
    fn ResolveResourceString();
    fn UpdateCapabilitiesForModernApps();
    fn ReportDownloadStatusUpdate();
    fn BeginUninstallWithOptions();
    fn BindDeferredMdilBinaries();
    fn GenerateXamlLightupXbfForCurrentLocale();
    fn AddLicenseForAppx();
    fn FixJunctionsForAppsOnSDCard();
}
pub trait IPMEnumerationManagerImpl: Sized {
    fn AllApplications();
    fn AllTiles();
    fn AllTasks();
    fn AllExtensions();
    fn AllBackgroundServiceAgents();
    fn AllBackgroundWorkers();
    fn ApplicationInfo();
    fn TileInfo();
    fn TaskInfo();
    fn TaskInfoEx();
    fn BackgroundServiceAgentInfo();
    fn AllLiveTileJobs();
    fn LiveTileJob();
    fn ApplicationInfoExternal();
    fn FileHandlerGenericLogo();
    fn ApplicationInfoFromAccessClaims();
    fn StartTileEnumeratorBlob();
    fn StartAppEnumeratorBlob();
}
pub trait IPMExtensionCachedFileUpdaterInfoImpl: Sized {
    fn SupportsUpdates();
}
pub trait IPMExtensionContractInfoImpl: Sized {
    fn InvocationInfo();
}
pub trait IPMExtensionFileExtensionInfoImpl: Sized {
    fn Name();
    fn DisplayName();
    fn Logo();
    fn ContentType();
    fn FileType();
    fn InvocationInfo();
    fn AllFileTypes();
}
pub trait IPMExtensionFileOpenPickerInfoImpl: Sized {
    fn AllFileTypes();
    fn SupportsAllFileTypes();
}
pub trait IPMExtensionFileSavePickerInfoImpl: Sized {
    fn AllFileTypes();
    fn SupportsAllFileTypes();
}
pub trait IPMExtensionInfoImpl: Sized {
    fn SupplierPID();
    fn SupplierTaskID();
    fn Title();
    fn IconPath();
    fn ExtraFile();
    fn InvocationInfo();
}
pub trait IPMExtensionInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMExtensionProtocolInfoImpl: Sized {
    fn Protocol();
    fn InvocationInfo();
}
pub trait IPMExtensionShareTargetInfoImpl: Sized {
    fn AllFileTypes();
    fn AllDataFormats();
    fn SupportsAllFileTypes();
}
pub trait IPMLiveTileJobInfoImpl: Sized {
    fn ProductID();
    fn TileID();
    fn NextSchedule();
    fn set_NextSchedule();
    fn StartSchedule();
    fn set_StartSchedule();
    fn IntervalDuration();
    fn set_IntervalDuration();
    fn RunForever();
    fn set_RunForever();
    fn MaxRunCount();
    fn set_MaxRunCount();
    fn RunCount();
    fn set_RunCount();
    fn RecurrenceType();
    fn set_RecurrenceType();
    fn TileXML();
    fn set_TileXML();
    fn UrlXML();
    fn set_UrlXML();
    fn AttemptCount();
    fn set_AttemptCount();
    fn DownloadState();
    fn set_DownloadState();
}
pub trait IPMLiveTileJobInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMTaskInfoImpl: Sized {
    fn ProductID();
    fn TaskID();
    fn NavigationPage();
    fn TaskTransition();
    fn RuntimeType();
    fn ActivationPolicy();
    fn TaskType();
    fn InvocationInfo();
    fn ImagePath();
    fn ImageParams();
    fn InstallRootFolder();
    fn DataRootFolder();
    fn IsSingleInstanceHost();
    fn IsInteropEnabled();
    fn ApplicationState();
    fn InstallType();
    fn Version();
    fn BitsPerPixel();
    fn SuppressesDehydration();
    fn BackgroundExecutionAbilities();
    fn IsOptedForExtendedMem();
}
pub trait IPMTaskInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMTileInfoImpl: Sized {
    fn ProductID();
    fn TileID();
    fn TemplateType();
    fn HubPinnedState();
    fn HubPosition();
    fn IsNotified();
    fn IsDefault();
    fn TaskID();
    fn TileType();
    fn IsThemable();
    fn PropertyById();
    fn InvocationInfo();
    fn PropertyEnum();
    fn HubTileSize();
    fn set_HubPosition();
    fn set_NotifiedState();
    fn set_HubPinnedState();
    fn set_HubTileSize();
    fn set_InvocationInfo();
    fn StartTileBlob();
    fn IsRestoring();
    fn IsAutoRestoreDisabled();
    fn set_IsRestoring();
    fn set_IsAutoRestoreDisabled();
}
pub trait IPMTileInfoEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMTilePropertyEnumeratorImpl: Sized {
    fn Next();
}
pub trait IPMTilePropertyInfoImpl: Sized {
    fn PropertyID();
    fn PropertyValue();
    fn set_Property();
}
pub trait IValidateImpl: Sized {
    fn OpenDatabase();
    fn OpenCUB();
    fn CloseDatabase();
    fn CloseCUB();
    fn SetDisplay();
    fn SetStatus();
    fn Validate();
}
