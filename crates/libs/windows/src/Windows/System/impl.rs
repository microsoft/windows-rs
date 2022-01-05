#[cfg(feature = "implement_exclusive")]
pub trait IAppActivationResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoImpl: Sized {
    fn AppInfo(&self) -> ::windows::core::Result<super::ApplicationModel::AppInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfo2Impl: Sized {
    fn GetResourceGroups(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>>;
    fn CreateResourceGroupWatcher(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfo3Impl: Sized {
    fn LaunchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoStaticsImpl: Sized {
    fn RequestInfoAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoStatics2Impl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcher>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>;
    fn RequestInfoForPackageAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppUserModelId(&self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoWatcherEventArgsImpl: Sized {
    fn AppDiagnosticInfo(&self) -> ::windows::core::Result<AppDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExecutionStateChangeResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryReportImpl: Sized {
    fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn PeakPrivateCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalCommitLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryReport2Impl: Sized {
    fn ExpectedTotalCommitLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryUsageLimitChangingEventArgsImpl: Sized {
    fn OldLimit(&self) -> ::windows::core::Result<u64>;
    fn NewLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupBackgroundTaskReportImpl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Trigger(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsShared(&self) -> ::windows::core::Result<bool>;
    fn GetBackgroundTaskReports(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>;
    fn GetMemoryReport(&self) -> ::windows::core::Result<AppResourceGroupMemoryReport>;
    fn GetProcessDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>;
    fn GetStateReport(&self) -> ::windows::core::Result<AppResourceGroupStateReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfo2Impl: Sized {
    fn StartSuspendAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartResumeAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartTerminateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ExecutionStateChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveExecutionStateChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoWatcherEventArgsImpl: Sized {
    fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl: Sized {
    fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupMemoryReportImpl: Sized {
    fn CommitUsageLimit(&self) -> ::windows::core::Result<u64>;
    fn CommitUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel>;
    fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalCommitUsage(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupStateReportImpl: Sized {
    fn ExecutionState(&self) -> ::windows::core::Result<AppResourceGroupExecutionState>;
    fn EnergyQuotaState(&self) -> ::windows::core::Result<AppResourceGroupEnergyQuotaState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHostImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHost2Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHostFactoryImpl: Sized {
    fn CreateInstance(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerHost>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&self) -> ::windows::core::Result<User>;
    fn GetAppAddedHostsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>;
    fn SetAppAddedHostsAsync(&self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistration2Impl: Sized {
    fn GetAllHosts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>>;
    fn UpdateHosts(&self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerImpl: Sized {
    fn User(&self) -> ::windows::core::Result<User>;
    fn TryGetRegistration(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManager2Impl: Sized {
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
    fn GetForUser(&self, user: &::core::option::Option<User>) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerStatics2Impl: Sized {
    fn GetForPackage(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
    fn GetForPackageForUser(&self, packagefamilyname: &::windows::core::HSTRING, user: &::core::option::Option<User>) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDateTimeSettingsStaticsImpl: Sized {
    fn SetSystemDateTime(&self, utcdatetime: &super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueImpl: Sized {
    fn CreateTimer(&self) -> ::windows::core::Result<DispatcherQueueTimer>;
    fn TryEnqueue(&self, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn TryEnqueueWithPriority(&self, priority: DispatcherQueuePriority, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn ShutdownStarting(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownStarting(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShutdownCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueue2Impl: Sized {
    fn HasThreadAccess(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueControllerImpl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<DispatcherQueue>;
    fn ShutdownQueueAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueControllerStaticsImpl: Sized {
    fn CreateOnDedicatedThread(&self) -> ::windows::core::Result<DispatcherQueueController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueShutdownStartingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueStaticsImpl: Sized {
    fn GetForCurrentThread(&self) -> ::windows::core::Result<DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueTimerImpl: Sized {
    fn Interval(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetInterval(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsRunning(&self) -> ::windows::core::Result<bool>;
    fn IsRepeating(&self) -> ::windows::core::Result<bool>;
    fn SetIsRepeating(&self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Tick(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveTick(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderLauncherOptionsImpl: Sized {
    fn ItemsToSelect(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUserPropertiesStaticsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GuestHost(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrincipalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionInitiationProtocolUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUserPropertiesStatics2Impl: Sized {
    fn AgeEnforcementRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILaunchUriResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LaunchUriStatus>;
    fn Result(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptionsImpl: Sized {
    fn TreatAsUntrusted(&self) -> ::windows::core::Result<bool>;
    fn SetTreatAsUntrusted(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayApplicationPicker(&self) -> ::windows::core::Result<bool>;
    fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows::core::Result<()>;
    fn UI(&self) -> ::windows::core::Result<LauncherUIOptions>;
    fn PreferredApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreferredApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PreferredApplicationDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreferredApplicationDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions2Impl: Sized {
    fn TargetApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult>;
    fn SetNeighboringFilesQuery(&self, value: &::core::option::Option<super::Storage::Search::StorageFileQueryResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions3Impl: Sized {
    fn IgnoreAppUriHandlers(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions4Impl: Sized {
    fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows::core::Result<bool>;
    fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStaticsImpl: Sized {
    fn LaunchFileAsync(&self, file: &::core::option::Option<super::Storage::IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFileWithOptionsAsync(&self, file: &::core::option::Option<super::Storage::IStorageFile>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriWithOptionsAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStatics2Impl: Sized {
    fn LaunchUriForResultsAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriForResultsWithDataAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriWithDataAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn QueryUriSupportAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryUriSupportWithPackageFamilyNameAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryFileSupportAsync(&self, file: &::core::option::Option<super::Storage::StorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryFileSupportWithPackageFamilyNameAsync(&self, file: &::core::option::Option<super::Storage::StorageFile>, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn FindUriSchemeHandlersAsync(&self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn FindUriSchemeHandlersWithLaunchUriTypeAsync(&self, scheme: &::windows::core::HSTRING, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn FindFileHandlersAsync(&self, extension: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStatics3Impl: Sized {
    fn LaunchFolderAsync(&self, folder: &::core::option::Option<super::Storage::IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderWithOptionsAsync(&self, folder: &::core::option::Option<super::Storage::IStorageFolder>, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStatics4Impl: Sized {
    fn QueryAppUriSupportAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryAppUriSupportWithPackageFamilyNameAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn FindAppUriHandlersAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn LaunchUriForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriWithOptionsForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriWithDataForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriForResultsForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriForResultsWithDataForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStatics5Impl: Sized {
    fn LaunchFolderPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsAsync(&self, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathForUserAsync(&self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsForUserAsync(&self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherUIOptionsImpl: Sized {
    fn InvocationPoint(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>>;
    fn SetInvocationPoint(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn SelectionRect(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>>;
    fn SetSelectionRect(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn PreferredPlacement(&self) -> ::windows::core::Result<super::UI::Popups::Placement>;
    fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()>;
}
pub trait ILauncherViewOptionsImpl: Sized {
    fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStaticsImpl: Sized {
    fn AppMemoryUsage(&self) -> ::windows::core::Result<u64>;
    fn AppMemoryUsageLimit(&self) -> ::windows::core::Result<u64>;
    fn AppMemoryUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel>;
    fn AppMemoryUsageIncreased(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageIncreased(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AppMemoryUsageDecreased(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageDecreased(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AppMemoryUsageLimitChanging(&self, handler: &::core::option::Option<super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageLimitChanging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics2Impl: Sized {
    fn GetAppMemoryReport(&self) -> ::windows::core::Result<AppMemoryReport>;
    fn GetProcessMemoryReport(&self) -> ::windows::core::Result<ProcessMemoryReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics3Impl: Sized {
    fn TrySetAppMemoryUsageLimit(&self, value: u64) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics4Impl: Sized {
    fn ExpectedAppMemoryUsageLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessLauncherOptionsImpl: Sized {
    fn StandardInput(&self) -> ::windows::core::Result<super::Storage::Streams::IInputStream>;
    fn SetStandardInput(&self, value: &::core::option::Option<super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn StandardOutput(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream>;
    fn SetStandardOutput(&self, value: &::core::option::Option<super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<()>;
    fn StandardError(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream>;
    fn SetStandardError(&self, value: &::core::option::Option<super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetWorkingDirectory(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessLauncherResultImpl: Sized {
    fn ExitCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessLauncherStaticsImpl: Sized {
    fn RunToCompletionAsync(&self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
    fn RunToCompletionAsyncWithOptions(&self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING, options: &::core::option::Option<ProcessLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryReportImpl: Sized {
    fn PrivateWorkingSetUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalWorkingSetUsage(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtocolForResultsOperationImpl: Sized {
    fn ReportCompleted(&self, data: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteLauncherOptionsImpl: Sized {
    fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PreferredAppIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteLauncherStaticsImpl: Sized {
    fn LaunchUriAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithOptionsAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithDataAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShutdownManagerStaticsImpl: Sized {
    fn BeginShutdown(&self, shutdownkind: ShutdownKind, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CancelShutdown(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShutdownManagerStatics2Impl: Sized + IShutdownManagerStaticsImpl {
    fn IsPowerStateSupported(&self, powerstate: PowerState) -> ::windows::core::Result<bool>;
    fn EnterPowerState(&self, powerstate: PowerState) -> ::windows::core::Result<()>;
    fn EnterPowerStateWithTimeSpan(&self, powerstate: PowerState, wakeupafter: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeZoneSettingsStaticsImpl: Sized {
    fn CurrentTimeZoneDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedTimeZoneDisplayNames(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CanChangeTimeZone(&self) -> ::windows::core::Result<bool>;
    fn ChangeTimeZoneByDisplayName(&self, timezonedisplayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeZoneSettingsStatics2Impl: Sized {
    fn AutoUpdateTimeZoneAsync(&self, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserImpl: Sized {
    fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AuthenticationStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn Type(&self) -> ::windows::core::Result<UserType>;
    fn GetPropertyAsync(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetPropertiesAsync(&self, values: &::core::option::Option<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>;
    fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUser2Impl: Sized {
    fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserAuthenticationStatusChangeDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserAuthenticationStatusChangingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<UserAuthenticationStatusChangeDeferral>;
    fn User(&self) -> ::windows::core::Result<User>;
    fn NewStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn CurrentStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserChangedEventArgsImpl: Sized {
    fn User(&self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserChangedEventArgs2Impl: Sized {
    fn ChangedPropertyKinds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDeviceAssociationChangedEventArgsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewUser(&self) -> ::windows::core::Result<User>;
    fn OldUser(&self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDeviceAssociationStaticsImpl: Sized {
    fn FindUserFromDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
    fn UserDeviceAssociationChanged(&self, handler: &::core::option::Option<super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUserDeviceAssociationChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserPickerImpl: Sized {
    fn AllowGuestAccounts(&self) -> ::windows::core::Result<bool>;
    fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuggestedSelectedUser(&self) -> ::windows::core::Result<User>;
    fn SetSuggestedSelectedUser(&self, value: &::core::option::Option<User>) -> ::windows::core::Result<()>;
    fn PickSingleUserAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserPickerStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<UserWatcher>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByType(&self, r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByTypeAndStatus(&self, r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn GetFromId(&self, nonroamableid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserStatics2Impl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserWatcherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<UserWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Added(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationStatusChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStatusChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationStatusChanging(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStatusChanging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
