#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesImpl: Sized + IDispatchImpl {
    fn DetectNow();
    fn Pause();
    fn Resume();
    fn ShowSettingsDialog();
    fn Settings();
    fn ServiceEnabled();
    fn EnableService();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdates2Impl: Sized + IAutomaticUpdatesImpl + IDispatchImpl {
    fn Results();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesResultsImpl: Sized + IDispatchImpl {
    fn LastSearchSuccessDate();
    fn LastInstallationSuccessDate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettingsImpl: Sized + IDispatchImpl {
    fn NotificationLevel();
    fn SetNotificationLevel();
    fn ReadOnly();
    fn Required();
    fn ScheduledInstallationDay();
    fn SetScheduledInstallationDay();
    fn ScheduledInstallationTime();
    fn SetScheduledInstallationTime();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings2Impl: Sized + IAutomaticUpdatesSettingsImpl + IDispatchImpl {
    fn IncludeRecommendedUpdates();
    fn SetIncludeRecommendedUpdates();
    fn CheckPermission();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings3Impl: Sized + IAutomaticUpdatesSettings2Impl + IAutomaticUpdatesSettingsImpl + IDispatchImpl {
    fn NonAdministratorsElevated();
    fn SetNonAdministratorsElevated();
    fn FeaturedUpdatesEnabled();
    fn SetFeaturedUpdatesEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICategoryImpl: Sized + IDispatchImpl {
    fn Name();
    fn CategoryID();
    fn Children();
    fn Description();
    fn Image();
    fn Order();
    fn Parent();
    fn Type();
    fn Updates();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICategoryCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
pub trait IDownloadCompletedCallbackImpl: Sized {
    fn Invoke();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallbackArgsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadJobImpl: Sized + IDispatchImpl {
    fn AsyncState();
    fn IsCompleted();
    fn Updates();
    fn CleanUp();
    fn GetProgress();
    fn RequestAbort();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressImpl: Sized + IDispatchImpl {
    fn CurrentUpdateBytesDownloaded();
    fn CurrentUpdateBytesToDownload();
    fn CurrentUpdateIndex();
    fn PercentComplete();
    fn TotalBytesDownloaded();
    fn TotalBytesToDownload();
    fn GetUpdateResult();
    fn CurrentUpdateDownloadPhase();
    fn CurrentUpdatePercentComplete();
}
pub trait IDownloadProgressChangedCallbackImpl: Sized {
    fn Invoke();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallbackArgsImpl: Sized + IDispatchImpl {
    fn Progress();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn ResultCode();
    fn GetUpdateResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImageInformationImpl: Sized + IDispatchImpl {
    fn AltText();
    fn Height();
    fn Source();
    fn Width();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationAgentImpl: Sized + IDispatchImpl {
    fn RecordInstallationResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationBehaviorImpl: Sized + IDispatchImpl {
    fn CanRequestUserInput();
    fn Impact();
    fn RebootBehavior();
    fn RequiresNetworkConnectivity();
}
pub trait IInstallationCompletedCallbackImpl: Sized {
    fn Invoke();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallbackArgsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationJobImpl: Sized + IDispatchImpl {
    fn AsyncState();
    fn IsCompleted();
    fn Updates();
    fn CleanUp();
    fn GetProgress();
    fn RequestAbort();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressImpl: Sized + IDispatchImpl {
    fn CurrentUpdateIndex();
    fn CurrentUpdatePercentComplete();
    fn PercentComplete();
    fn GetUpdateResult();
}
pub trait IInstallationProgressChangedCallbackImpl: Sized {
    fn Invoke();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallbackArgsImpl: Sized + IDispatchImpl {
    fn Progress();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn RebootRequired();
    fn ResultCode();
    fn GetUpdateResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInvalidProductLicenseExceptionImpl: Sized + IUpdateExceptionImpl + IDispatchImpl {
    fn Product();
}
pub trait ISearchCompletedCallbackImpl: Sized {
    fn Invoke();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallbackArgsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchJobImpl: Sized + IDispatchImpl {
    fn AsyncState();
    fn IsCompleted();
    fn CleanUp();
    fn RequestAbort();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchResultImpl: Sized + IDispatchImpl {
    fn ResultCode();
    fn RootCategories();
    fn Updates();
    fn Warnings();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStringCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn SetItem();
    fn _NewEnum();
    fn Count();
    fn ReadOnly();
    fn Add();
    fn Clear();
    fn Copy();
    fn Insert();
    fn RemoveAt();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISystemInformationImpl: Sized + IDispatchImpl {
    fn OemHardwareSupportLink();
    fn RebootRequired();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateImpl: Sized + IDispatchImpl {
    fn Title();
    fn AutoSelectOnWebSites();
    fn BundledUpdates();
    fn CanRequireSource();
    fn Categories();
    fn Deadline();
    fn DeltaCompressedContentAvailable();
    fn DeltaCompressedContentPreferred();
    fn Description();
    fn EulaAccepted();
    fn EulaText();
    fn HandlerID();
    fn Identity();
    fn Image();
    fn InstallationBehavior();
    fn IsBeta();
    fn IsDownloaded();
    fn IsHidden();
    fn SetIsHidden();
    fn IsInstalled();
    fn IsMandatory();
    fn IsUninstallable();
    fn Languages();
    fn LastDeploymentChangeTime();
    fn MaxDownloadSize();
    fn MinDownloadSize();
    fn MoreInfoUrls();
    fn MsrcSeverity();
    fn RecommendedCpuSpeed();
    fn RecommendedHardDiskSpace();
    fn RecommendedMemory();
    fn ReleaseNotes();
    fn SecurityBulletinIDs();
    fn SupersededUpdateIDs();
    fn SupportUrl();
    fn Type();
    fn UninstallationNotes();
    fn UninstallationBehavior();
    fn UninstallationSteps();
    fn KBArticleIDs();
    fn AcceptEula();
    fn DeploymentAction();
    fn CopyFromCache();
    fn DownloadPriority();
    fn DownloadContents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate2Impl: Sized + IUpdateImpl + IDispatchImpl {
    fn RebootRequired();
    fn IsPresent();
    fn CveIDs();
    fn CopyToCache();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate3Impl: Sized + IUpdate2Impl + IUpdateImpl + IDispatchImpl {
    fn BrowseOnly();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate4Impl: Sized + IUpdate3Impl + IUpdate2Impl + IUpdateImpl + IDispatchImpl {
    fn PerUser();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate5Impl: Sized + IUpdate4Impl + IUpdate3Impl + IUpdate2Impl + IUpdateImpl + IDispatchImpl {
    fn AutoSelection();
    fn AutoDownload();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn SetItem();
    fn _NewEnum();
    fn Count();
    fn ReadOnly();
    fn Add();
    fn Clear();
    fn Copy();
    fn Insert();
    fn RemoveAt();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContentImpl: Sized + IDispatchImpl {
    fn DownloadUrl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContent2Impl: Sized + IUpdateDownloadContentImpl + IDispatchImpl {
    fn IsDeltaCompressedContent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContentCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn ResultCode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloaderImpl: Sized + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn IsForced();
    fn SetIsForced();
    fn Priority();
    fn SetPriority();
    fn Updates();
    fn SetUpdates();
    fn BeginDownload();
    fn Download();
    fn EndDownload();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateExceptionImpl: Sized + IDispatchImpl {
    fn Message();
    fn HResult();
    fn Context();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateExceptionCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntryImpl: Sized + IDispatchImpl {
    fn Operation();
    fn ResultCode();
    fn HResult();
    fn Date();
    fn UpdateIdentity();
    fn Title();
    fn Description();
    fn UnmappedResultCode();
    fn ClientApplicationID();
    fn ServerSelection();
    fn ServiceID();
    fn UninstallationSteps();
    fn UninstallationNotes();
    fn SupportUrl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntry2Impl: Sized + IUpdateHistoryEntryImpl + IDispatchImpl {
    fn Categories();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntryCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateIdentityImpl: Sized + IDispatchImpl {
    fn RevisionNumber();
    fn UpdateID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstallationResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn RebootRequired();
    fn ResultCode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstallerImpl: Sized + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn IsForced();
    fn SetIsForced();
    fn ParentHwnd();
    fn SetParentHwnd();
    fn SetParentWindow();
    fn ParentWindow();
    fn Updates();
    fn SetUpdates();
    fn BeginInstall();
    fn BeginUninstall();
    fn EndInstall();
    fn EndUninstall();
    fn Install();
    fn RunWizard();
    fn IsBusy();
    fn Uninstall();
    fn AllowSourcePrompts();
    fn SetAllowSourcePrompts();
    fn RebootRequiredBeforeInstallation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller2Impl: Sized + IUpdateInstallerImpl + IDispatchImpl {
    fn ForceQuiet();
    fn SetForceQuiet();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller3Impl: Sized + IUpdateInstaller2Impl + IUpdateInstallerImpl + IDispatchImpl {
    fn AttemptCloseAppsIfNecessary();
    fn SetAttemptCloseAppsIfNecessary();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller4Impl: Sized + IUpdateInstaller3Impl + IUpdateInstaller2Impl + IUpdateInstallerImpl + IDispatchImpl {
    fn Commit();
}
pub trait IUpdateLockdownImpl: Sized {
    fn LockDown();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcherImpl: Sized + IDispatchImpl {
    fn CanAutomaticallyUpgradeService();
    fn SetCanAutomaticallyUpgradeService();
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn IncludePotentiallySupersededUpdates();
    fn SetIncludePotentiallySupersededUpdates();
    fn ServerSelection();
    fn SetServerSelection();
    fn BeginSearch();
    fn EndSearch();
    fn EscapeString();
    fn QueryHistory();
    fn Search();
    fn Online();
    fn SetOnline();
    fn GetTotalHistoryCount();
    fn ServiceID();
    fn SetServiceID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher2Impl: Sized + IUpdateSearcherImpl + IDispatchImpl {
    fn IgnoreDownloadPriority();
    fn SetIgnoreDownloadPriority();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher3Impl: Sized + IUpdateSearcher2Impl + IUpdateSearcherImpl + IDispatchImpl {
    fn SearchScope();
    fn SetSearchScope();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceImpl: Sized + IDispatchImpl {
    fn Name();
    fn ContentValidationCert();
    fn ExpirationDate();
    fn IsManaged();
    fn IsRegisteredWithAU();
    fn IssueDate();
    fn OffersWindowsUpdates();
    fn RedirectUrls();
    fn ServiceID();
    fn IsScanPackageService();
    fn CanRegisterWithAU();
    fn ServiceUrl();
    fn SetupPrefix();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateService2Impl: Sized + IUpdateServiceImpl + IDispatchImpl {
    fn IsDefaultAUService();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceManagerImpl: Sized + IDispatchImpl {
    fn Services();
    fn AddService();
    fn RegisterServiceWithAU();
    fn RemoveService();
    fn UnregisterServiceWithAU();
    fn AddScanPackageService();
    fn SetOption();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceManager2Impl: Sized + IUpdateServiceManagerImpl + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn QueryServiceRegistration();
    fn AddService2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceRegistrationImpl: Sized + IDispatchImpl {
    fn RegistrationState();
    fn ServiceID();
    fn IsPendingRegistrationWithAU();
    fn Service();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSessionImpl: Sized + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn ReadOnly();
    fn WebProxy();
    fn SetWebProxy();
    fn CreateUpdateSearcher();
    fn CreateUpdateDownloader();
    fn CreateUpdateInstaller();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession2Impl: Sized + IUpdateSessionImpl + IDispatchImpl {
    fn UserLocale();
    fn SetUserLocale();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession3Impl: Sized + IUpdateSession2Impl + IUpdateSessionImpl + IDispatchImpl {
    fn CreateUpdateServiceManager();
    fn QueryHistory();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebProxyImpl: Sized + IDispatchImpl {
    fn Address();
    fn SetAddress();
    fn BypassList();
    fn SetBypassList();
    fn BypassProxyOnLocal();
    fn SetBypassProxyOnLocal();
    fn ReadOnly();
    fn UserName();
    fn SetUserName();
    fn SetPassword();
    fn PromptForCredentials();
    fn PromptForCredentialsFromHwnd();
    fn AutoDetect();
    fn SetAutoDetect();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateImpl: Sized + IUpdateImpl + IDispatchImpl {
    fn DriverClass();
    fn DriverHardwareID();
    fn DriverManufacturer();
    fn DriverModel();
    fn DriverProvider();
    fn DriverVerDate();
    fn DeviceProblemNumber();
    fn DeviceStatus();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate2Impl: Sized + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn RebootRequired();
    fn IsPresent();
    fn CveIDs();
    fn CopyToCache();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate3Impl: Sized + IWindowsDriverUpdate2Impl + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn BrowseOnly();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate4Impl: Sized + IWindowsDriverUpdate3Impl + IWindowsDriverUpdate2Impl + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn WindowsDriverUpdateEntries();
    fn PerUser();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate5Impl: Sized + IWindowsDriverUpdate4Impl + IWindowsDriverUpdate3Impl + IWindowsDriverUpdate2Impl + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn AutoSelection();
    fn AutoDownload();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateEntryImpl: Sized + IDispatchImpl {
    fn DriverClass();
    fn DriverHardwareID();
    fn DriverManufacturer();
    fn DriverModel();
    fn DriverProvider();
    fn DriverVerDate();
    fn DeviceProblemNumber();
    fn DeviceStatus();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateEntryCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsUpdateAgentInfoImpl: Sized + IDispatchImpl {
    fn GetInfo();
}
