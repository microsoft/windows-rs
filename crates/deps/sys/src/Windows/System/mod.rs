#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "System_Display")]
pub mod Display;
#[cfg(feature = "System_Implementation")]
pub mod Implementation;
#[cfg(feature = "System_Inventory")]
pub mod Inventory;
#[cfg(feature = "System_Power")]
pub mod Power;
#[cfg(feature = "System_Preview")]
pub mod Preview;
#[cfg(feature = "System_Profile")]
pub mod Profile;
#[cfg(feature = "System_RemoteDesktop")]
pub mod RemoteDesktop;
#[cfg(feature = "System_RemoteSystems")]
pub mod RemoteSystems;
#[cfg(feature = "System_Threading")]
pub mod Threading;
#[cfg(feature = "System_Update")]
pub mod Update;
#[cfg(feature = "System_UserProfile")]
pub mod UserProfile;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppActivationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppDiagnosticInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppDiagnosticInfoWatcherStatus(i32);
#[repr(transparent)]
pub struct AppExecutionStateChangeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppMemoryUsageLevel(i32);
#[repr(transparent)]
pub struct AppMemoryUsageLimitChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupBackgroundTaskReport(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppResourceGroupEnergyQuotaState(i32);
#[repr(C)]
pub struct AppResourceGroupExecutionState(i32);
#[repr(transparent)]
pub struct AppResourceGroupInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppResourceGroupInfoWatcherStatus(i32);
#[repr(transparent)]
pub struct AppResourceGroupMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupStateReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppUriHandlerHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppUriHandlerRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppUriHandlerRegistrationManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AutoUpdateTimeZoneStatus(i32);
#[repr(C)]
pub struct DiagnosticAccessStatus(i32);
#[repr(transparent)]
pub struct DispatcherQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherQueueController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherQueueHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DispatcherQueuePriority(i32);
#[repr(transparent)]
pub struct DispatcherQueueShutdownStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherQueueTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppActivationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExecutionStateChangeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppMemoryReport2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppMemoryUsageLimitChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupBackgroundTaskReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppResourceGroupStateReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerHost2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerHostFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerRegistration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueue2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueueController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueueControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueueShutdownStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherQueueTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchUriResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherOptions3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherOptions4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILauncherViewOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessLauncherResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessLauncherStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtocolForResultsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteLauncherStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShutdownManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShutdownManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangeDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDeviceAssociationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDeviceAssociationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LaunchFileStatus(i32);
#[repr(C)]
pub struct LaunchQuerySupportStatus(i32);
#[repr(C)]
pub struct LaunchQuerySupportType(i32);
#[repr(transparent)]
pub struct LaunchUriResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LaunchUriStatus(i32);
#[repr(transparent)]
pub struct LauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LauncherUIOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PowerState(i32);
#[repr(transparent)]
pub struct ProcessLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessLauncherResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ProcessorArchitecture(i32);
#[repr(transparent)]
pub struct ProtocolForResultsOperation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RemoteLaunchUriStatus(i32);
#[repr(transparent)]
pub struct RemoteLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ShutdownKind(i32);
#[repr(C)]
pub struct SystemManagementContract(i32);
#[repr(transparent)]
pub struct User(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserAgeConsentGroup(i32);
#[repr(C)]
pub struct UserAgeConsentResult(i32);
#[repr(C)]
pub struct UserAuthenticationStatus(i32);
#[repr(transparent)]
pub struct UserAuthenticationStatusChangeDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserAuthenticationStatusChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDeviceAssociationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserPicker(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserPictureSize(i32);
#[repr(C)]
pub struct UserType(i32);
#[repr(transparent)]
pub struct UserWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserWatcherStatus(i32);
#[repr(C)]
pub struct UserWatcherUpdateKind(i32);
#[repr(C)]
pub struct VirtualKey(i32);
#[repr(C)]
pub struct VirtualKeyModifiers(i32);
