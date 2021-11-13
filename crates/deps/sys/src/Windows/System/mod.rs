#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for AppActivationResult {}
impl ::core::clone::Clone for AppActivationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppDiagnosticInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppDiagnosticInfo {}
impl ::core::clone::Clone for AppDiagnosticInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppDiagnosticInfoWatcher {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppDiagnosticInfoWatcherEventArgs {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppDiagnosticInfoWatcherStatus {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExecutionStateChangeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExecutionStateChangeResult {}
impl ::core::clone::Clone for AppExecutionStateChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppMemoryReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppMemoryReport {}
impl ::core::clone::Clone for AppMemoryReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
    pub const OverLimit: Self = Self(3i32);
}
impl ::core::marker::Copy for AppMemoryUsageLevel {}
impl ::core::clone::Clone for AppMemoryUsageLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppMemoryUsageLimitChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppMemoryUsageLimitChangingEventArgs {}
impl ::core::clone::Clone for AppMemoryUsageLimitChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupBackgroundTaskReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupBackgroundTaskReport {}
impl ::core::clone::Clone for AppResourceGroupBackgroundTaskReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: Self = Self(0i32);
    pub const Over: Self = Self(1i32);
    pub const Under: Self = Self(2i32);
}
impl ::core::marker::Copy for AppResourceGroupEnergyQuotaState {}
impl ::core::clone::Clone for AppResourceGroupEnergyQuotaState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspending: Self = Self(2i32);
    pub const Suspended: Self = Self(3i32);
    pub const NotRunning: Self = Self(4i32);
}
impl ::core::marker::Copy for AppResourceGroupExecutionState {}
impl ::core::clone::Clone for AppResourceGroupExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupInfo {}
impl ::core::clone::Clone for AppResourceGroupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupInfoWatcher {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupInfoWatcherEventArgs {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppResourceGroupInfoWatcherStatus {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupMemoryReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupMemoryReport {}
impl ::core::clone::Clone for AppResourceGroupMemoryReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppResourceGroupStateReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppResourceGroupStateReport {}
impl ::core::clone::Clone for AppResourceGroupStateReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppUriHandlerHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppUriHandlerHost {}
impl ::core::clone::Clone for AppUriHandlerHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppUriHandlerRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppUriHandlerRegistration {}
impl ::core::clone::Clone for AppUriHandlerRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppUriHandlerRegistrationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppUriHandlerRegistrationManager {}
impl ::core::clone::Clone for AppUriHandlerRegistrationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoUpdateTimeZoneStatus {}
impl ::core::clone::Clone for AutoUpdateTimeZoneStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
    pub const Allowed: Self = Self(3i32);
}
impl ::core::marker::Copy for DiagnosticAccessStatus {}
impl ::core::clone::Clone for DiagnosticAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatcherQueue {}
impl ::core::clone::Clone for DispatcherQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherQueueController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatcherQueueController {}
impl ::core::clone::Clone for DispatcherQueueController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherQueueHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatcherQueueHandler {}
impl ::core::clone::Clone for DispatcherQueueHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherQueueShutdownStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatcherQueueShutdownStartingEventArgs {}
impl ::core::clone::Clone for DispatcherQueueShutdownStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherQueueTimer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatcherQueueTimer {}
impl ::core::clone::Clone for DispatcherQueueTimer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FolderLauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FolderLauncherOptions {}
impl ::core::clone::Clone for FolderLauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppActivationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppActivationResult {}
impl ::core::clone::Clone for IAppActivationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfo {}
impl ::core::clone::Clone for IAppDiagnosticInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfo2 {}
impl ::core::clone::Clone for IAppDiagnosticInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfo3 {}
impl ::core::clone::Clone for IAppDiagnosticInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfoStatics {}
impl ::core::clone::Clone for IAppDiagnosticInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfoStatics2 {}
impl ::core::clone::Clone for IAppDiagnosticInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfoWatcher {}
impl ::core::clone::Clone for IAppDiagnosticInfoWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDiagnosticInfoWatcherEventArgs {}
impl ::core::clone::Clone for IAppDiagnosticInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExecutionStateChangeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExecutionStateChangeResult {}
impl ::core::clone::Clone for IAppExecutionStateChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppMemoryReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppMemoryReport {}
impl ::core::clone::Clone for IAppMemoryReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppMemoryReport2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppMemoryReport2 {}
impl ::core::clone::Clone for IAppMemoryReport2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppMemoryUsageLimitChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppMemoryUsageLimitChangingEventArgs {}
impl ::core::clone::Clone for IAppMemoryUsageLimitChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupBackgroundTaskReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupBackgroundTaskReport {}
impl ::core::clone::Clone for IAppResourceGroupBackgroundTaskReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupInfo {}
impl ::core::clone::Clone for IAppResourceGroupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupInfo2 {}
impl ::core::clone::Clone for IAppResourceGroupInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupInfoWatcher {}
impl ::core::clone::Clone for IAppResourceGroupInfoWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupInfoWatcherEventArgs {}
impl ::core::clone::Clone for IAppResourceGroupInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
impl ::core::clone::Clone for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupMemoryReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupMemoryReport {}
impl ::core::clone::Clone for IAppResourceGroupMemoryReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppResourceGroupStateReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppResourceGroupStateReport {}
impl ::core::clone::Clone for IAppResourceGroupStateReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerHost {}
impl ::core::clone::Clone for IAppUriHandlerHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerHost2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerHost2 {}
impl ::core::clone::Clone for IAppUriHandlerHost2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerHostFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerHostFactory {}
impl ::core::clone::Clone for IAppUriHandlerHostFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerRegistration {}
impl ::core::clone::Clone for IAppUriHandlerRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerRegistration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerRegistration2 {}
impl ::core::clone::Clone for IAppUriHandlerRegistration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerRegistrationManager {}
impl ::core::clone::Clone for IAppUriHandlerRegistrationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerRegistrationManager2 {}
impl ::core::clone::Clone for IAppUriHandlerRegistrationManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerRegistrationManagerStatics {}
impl ::core::clone::Clone for IAppUriHandlerRegistrationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUriHandlerRegistrationManagerStatics2 {}
impl ::core::clone::Clone for IAppUriHandlerRegistrationManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDateTimeSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDateTimeSettingsStatics {}
impl ::core::clone::Clone for IDateTimeSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueue {}
impl ::core::clone::Clone for IDispatcherQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueue2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueue2 {}
impl ::core::clone::Clone for IDispatcherQueue2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueueController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueueController {}
impl ::core::clone::Clone for IDispatcherQueueController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueueControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueueControllerStatics {}
impl ::core::clone::Clone for IDispatcherQueueControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueueShutdownStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueueShutdownStartingEventArgs {}
impl ::core::clone::Clone for IDispatcherQueueShutdownStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueueStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueueStatics {}
impl ::core::clone::Clone for IDispatcherQueueStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherQueueTimer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherQueueTimer {}
impl ::core::clone::Clone for IDispatcherQueueTimer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFolderLauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFolderLauncherOptions {}
impl ::core::clone::Clone for IFolderLauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownUserPropertiesStatics {}
impl ::core::clone::Clone for IKnownUserPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownUserPropertiesStatics2 {}
impl ::core::clone::Clone for IKnownUserPropertiesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILaunchUriResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILaunchUriResult {}
impl ::core::clone::Clone for ILaunchUriResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherOptions {}
impl ::core::clone::Clone for ILauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherOptions2 {}
impl ::core::clone::Clone for ILauncherOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherOptions3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherOptions3 {}
impl ::core::clone::Clone for ILauncherOptions3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherOptions4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherOptions4 {}
impl ::core::clone::Clone for ILauncherOptions4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherStatics {}
impl ::core::clone::Clone for ILauncherStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherStatics2 {}
impl ::core::clone::Clone for ILauncherStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherStatics3 {}
impl ::core::clone::Clone for ILauncherStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherStatics4 {}
impl ::core::clone::Clone for ILauncherStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherStatics5 {}
impl ::core::clone::Clone for ILauncherStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherUIOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherUIOptions {}
impl ::core::clone::Clone for ILauncherUIOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILauncherViewOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILauncherViewOptions {}
impl ::core::clone::Clone for ILauncherViewOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryManagerStatics {}
impl ::core::clone::Clone for IMemoryManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryManagerStatics2 {}
impl ::core::clone::Clone for IMemoryManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryManagerStatics3 {}
impl ::core::clone::Clone for IMemoryManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMemoryManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMemoryManagerStatics4 {}
impl ::core::clone::Clone for IMemoryManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessLauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessLauncherOptions {}
impl ::core::clone::Clone for IProcessLauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessLauncherResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessLauncherResult {}
impl ::core::clone::Clone for IProcessLauncherResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessLauncherStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessLauncherStatics {}
impl ::core::clone::Clone for IProcessLauncherStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessMemoryReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessMemoryReport {}
impl ::core::clone::Clone for IProcessMemoryReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtocolForResultsOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtocolForResultsOperation {}
impl ::core::clone::Clone for IProtocolForResultsOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteLauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteLauncherOptions {}
impl ::core::clone::Clone for IRemoteLauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteLauncherStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteLauncherStatics {}
impl ::core::clone::Clone for IRemoteLauncherStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShutdownManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShutdownManagerStatics {}
impl ::core::clone::Clone for IShutdownManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShutdownManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShutdownManagerStatics2 {}
impl ::core::clone::Clone for IShutdownManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeZoneSettingsStatics {}
impl ::core::clone::Clone for ITimeZoneSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeZoneSettingsStatics2 {}
impl ::core::clone::Clone for ITimeZoneSettingsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUser {}
impl ::core::clone::Clone for IUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUser2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUser2 {}
impl ::core::clone::Clone for IUser2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangeDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserAuthenticationStatusChangeDeferral {}
impl ::core::clone::Clone for IUserAuthenticationStatusChangeDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserAuthenticationStatusChangingEventArgs {}
impl ::core::clone::Clone for IUserAuthenticationStatusChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserChangedEventArgs {}
impl ::core::clone::Clone for IUserChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserChangedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserChangedEventArgs2 {}
impl ::core::clone::Clone for IUserChangedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDeviceAssociationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDeviceAssociationChangedEventArgs {}
impl ::core::clone::Clone for IUserDeviceAssociationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDeviceAssociationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDeviceAssociationStatics {}
impl ::core::clone::Clone for IUserDeviceAssociationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserPicker {}
impl ::core::clone::Clone for IUserPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserPickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserPickerStatics {}
impl ::core::clone::Clone for IUserPickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserStatics {}
impl ::core::clone::Clone for IUserStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserStatics2 {}
impl ::core::clone::Clone for IUserStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserWatcher {}
impl ::core::clone::Clone for IUserWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const DeniedByPolicy: Self = Self(2i32);
    pub const FileTypeNotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchFileStatus {}
impl ::core::clone::Clone for LaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchQuerySupportStatus {}
impl ::core::clone::Clone for LaunchQuerySupportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: Self = Self(0i32);
    pub const UriForResults: Self = Self(1i32);
}
impl ::core::marker::Copy for LaunchQuerySupportType {}
impl ::core::clone::Clone for LaunchQuerySupportType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LaunchUriResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LaunchUriResult {}
impl ::core::clone::Clone for LaunchUriResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const ProtocolUnavailable: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LaunchUriStatus {}
impl ::core::clone::Clone for LaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LauncherOptions {}
impl ::core::clone::Clone for LauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LauncherUIOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LauncherUIOptions {}
impl ::core::clone::Clone for LauncherUIOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: Self = Self(0i32);
    pub const SleepS3: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerState {}
impl ::core::clone::Clone for PowerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessLauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessLauncherOptions {}
impl ::core::clone::Clone for ProcessLauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessLauncherResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessLauncherResult {}
impl ::core::clone::Clone for ProcessLauncherResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessMemoryReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessMemoryReport {}
impl ::core::clone::Clone for ProcessMemoryReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0i32);
    pub const Arm: Self = Self(5i32);
    pub const X64: Self = Self(9i32);
    pub const Neutral: Self = Self(11i32);
    pub const Arm64: Self = Self(12i32);
    pub const X86OnArm64: Self = Self(14i32);
    pub const Unknown: Self = Self(65535i32);
}
impl ::core::marker::Copy for ProcessorArchitecture {}
impl ::core::clone::Clone for ProcessorArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtocolForResultsOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtocolForResultsOperation {}
impl ::core::clone::Clone for ProtocolForResultsOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const ProtocolUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const ValueSetTooLarge: Self = Self(5i32);
    pub const DeniedByLocalSystem: Self = Self(6i32);
    pub const DeniedByRemoteSystem: Self = Self(7i32);
}
impl ::core::marker::Copy for RemoteLaunchUriStatus {}
impl ::core::clone::Clone for RemoteLaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteLauncherOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteLauncherOptions {}
impl ::core::clone::Clone for RemoteLauncherOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: Self = Self(0i32);
    pub const Restart: Self = Self(1i32);
}
impl ::core::marker::Copy for ShutdownKind {}
impl ::core::clone::Clone for ShutdownKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct User(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for User {}
impl ::core::clone::Clone for User {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: Self = Self(0i32);
    pub const Minor: Self = Self(1i32);
    pub const Adult: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAgeConsentGroup {}
impl ::core::clone::Clone for UserAgeConsentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: Self = Self(0i32);
    pub const Included: Self = Self(1i32);
    pub const NotIncluded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const Ambiguous: Self = Self(4i32);
}
impl ::core::marker::Copy for UserAgeConsentResult {}
impl ::core::clone::Clone for UserAgeConsentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const LocallyAuthenticated: Self = Self(1i32);
    pub const RemotelyAuthenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAuthenticationStatus {}
impl ::core::clone::Clone for UserAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserAuthenticationStatusChangeDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserAuthenticationStatusChangeDeferral {}
impl ::core::clone::Clone for UserAuthenticationStatusChangeDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserAuthenticationStatusChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserAuthenticationStatusChangingEventArgs {}
impl ::core::clone::Clone for UserAuthenticationStatusChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserChangedEventArgs {}
impl ::core::clone::Clone for UserChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDeviceAssociationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDeviceAssociationChangedEventArgs {}
impl ::core::clone::Clone for UserDeviceAssociationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserPicker {}
impl ::core::clone::Clone for UserPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: Self = Self(0i32);
    pub const Size208x208: Self = Self(1i32);
    pub const Size424x424: Self = Self(2i32);
    pub const Size1080x1080: Self = Self(3i32);
}
impl ::core::marker::Copy for UserPictureSize {}
impl ::core::clone::Clone for UserPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: Self = Self(0i32);
    pub const RemoteUser: Self = Self(1i32);
    pub const LocalGuest: Self = Self(2i32);
    pub const RemoteGuest: Self = Self(3i32);
    pub const SystemManaged: Self = Self(4i32);
}
impl ::core::marker::Copy for UserType {}
impl ::core::clone::Clone for UserType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserWatcher {}
impl ::core::clone::Clone for UserWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for UserWatcherStatus {}
impl ::core::clone::Clone for UserWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: Self = Self(0i32);
    pub const Picture: Self = Self(1i32);
}
impl ::core::marker::Copy for UserWatcherUpdateKind {}
impl ::core::clone::Clone for UserWatcherUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl ::core::marker::Copy for VirtualKey {}
impl ::core::clone::Clone for VirtualKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl ::core::marker::Copy for VirtualKeyModifiers {}
impl ::core::clone::Clone for VirtualKeyModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
