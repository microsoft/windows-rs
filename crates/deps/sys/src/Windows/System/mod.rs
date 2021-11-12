#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(0i32);
    pub const Started: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(1i32);
    pub const EnumerationCompleted: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(2i32);
    pub const Stopping: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(3i32);
    pub const Stopped: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(4i32);
    pub const Aborted: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(5i32);
}
#[repr(transparent)]
pub struct AppExecutionStateChangeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: AppMemoryUsageLevel = AppMemoryUsageLevel(0i32);
    pub const Medium: AppMemoryUsageLevel = AppMemoryUsageLevel(1i32);
    pub const High: AppMemoryUsageLevel = AppMemoryUsageLevel(2i32);
    pub const OverLimit: AppMemoryUsageLevel = AppMemoryUsageLevel(3i32);
}
#[repr(transparent)]
pub struct AppMemoryUsageLimitChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupBackgroundTaskReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: AppResourceGroupEnergyQuotaState = AppResourceGroupEnergyQuotaState(0i32);
    pub const Over: AppResourceGroupEnergyQuotaState = AppResourceGroupEnergyQuotaState(1i32);
    pub const Under: AppResourceGroupEnergyQuotaState = AppResourceGroupEnergyQuotaState(2i32);
}
#[repr(transparent)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: AppResourceGroupExecutionState = AppResourceGroupExecutionState(0i32);
    pub const Running: AppResourceGroupExecutionState = AppResourceGroupExecutionState(1i32);
    pub const Suspending: AppResourceGroupExecutionState = AppResourceGroupExecutionState(2i32);
    pub const Suspended: AppResourceGroupExecutionState = AppResourceGroupExecutionState(3i32);
    pub const NotRunning: AppResourceGroupExecutionState = AppResourceGroupExecutionState(4i32);
}
#[repr(transparent)]
pub struct AppResourceGroupInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(0i32);
    pub const Started: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(1i32);
    pub const EnumerationCompleted: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(2i32);
    pub const Stopping: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(3i32);
    pub const Stopped: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(4i32);
    pub const Aborted: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(5i32);
}
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
#[repr(transparent)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: AutoUpdateTimeZoneStatus = AutoUpdateTimeZoneStatus(0i32);
    pub const TimedOut: AutoUpdateTimeZoneStatus = AutoUpdateTimeZoneStatus(1i32);
    pub const Failed: AutoUpdateTimeZoneStatus = AutoUpdateTimeZoneStatus(2i32);
}
#[repr(transparent)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: DiagnosticAccessStatus = DiagnosticAccessStatus(0i32);
    pub const Denied: DiagnosticAccessStatus = DiagnosticAccessStatus(1i32);
    pub const Limited: DiagnosticAccessStatus = DiagnosticAccessStatus(2i32);
    pub const Allowed: DiagnosticAccessStatus = DiagnosticAccessStatus(3i32);
}
#[repr(transparent)]
pub struct DispatcherQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherQueueController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherQueueHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: DispatcherQueuePriority = DispatcherQueuePriority(-10i32);
    pub const Normal: DispatcherQueuePriority = DispatcherQueuePriority(0i32);
    pub const High: DispatcherQueuePriority = DispatcherQueuePriority(10i32);
}
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
#[repr(transparent)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: LaunchFileStatus = LaunchFileStatus(0i32);
    pub const AppUnavailable: LaunchFileStatus = LaunchFileStatus(1i32);
    pub const DeniedByPolicy: LaunchFileStatus = LaunchFileStatus(2i32);
    pub const FileTypeNotSupported: LaunchFileStatus = LaunchFileStatus(3i32);
    pub const Unknown: LaunchFileStatus = LaunchFileStatus(4i32);
}
#[repr(transparent)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: LaunchQuerySupportStatus = LaunchQuerySupportStatus(0i32);
    pub const AppNotInstalled: LaunchQuerySupportStatus = LaunchQuerySupportStatus(1i32);
    pub const AppUnavailable: LaunchQuerySupportStatus = LaunchQuerySupportStatus(2i32);
    pub const NotSupported: LaunchQuerySupportStatus = LaunchQuerySupportStatus(3i32);
    pub const Unknown: LaunchQuerySupportStatus = LaunchQuerySupportStatus(4i32);
}
#[repr(transparent)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: LaunchQuerySupportType = LaunchQuerySupportType(0i32);
    pub const UriForResults: LaunchQuerySupportType = LaunchQuerySupportType(1i32);
}
#[repr(transparent)]
pub struct LaunchUriResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: LaunchUriStatus = LaunchUriStatus(0i32);
    pub const AppUnavailable: LaunchUriStatus = LaunchUriStatus(1i32);
    pub const ProtocolUnavailable: LaunchUriStatus = LaunchUriStatus(2i32);
    pub const Unknown: LaunchUriStatus = LaunchUriStatus(3i32);
}
#[repr(transparent)]
pub struct LauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LauncherUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: PowerState = PowerState(0i32);
    pub const SleepS3: PowerState = PowerState(1i32);
}
#[repr(transparent)]
pub struct ProcessLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessLauncherResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessMemoryReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: ProcessorArchitecture = ProcessorArchitecture(0i32);
    pub const Arm: ProcessorArchitecture = ProcessorArchitecture(5i32);
    pub const X64: ProcessorArchitecture = ProcessorArchitecture(9i32);
    pub const Neutral: ProcessorArchitecture = ProcessorArchitecture(11i32);
    pub const Arm64: ProcessorArchitecture = ProcessorArchitecture(12i32);
    pub const X86OnArm64: ProcessorArchitecture = ProcessorArchitecture(14i32);
    pub const Unknown: ProcessorArchitecture = ProcessorArchitecture(65535i32);
}
#[repr(transparent)]
pub struct ProtocolForResultsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: RemoteLaunchUriStatus = RemoteLaunchUriStatus(0i32);
    pub const Success: RemoteLaunchUriStatus = RemoteLaunchUriStatus(1i32);
    pub const AppUnavailable: RemoteLaunchUriStatus = RemoteLaunchUriStatus(2i32);
    pub const ProtocolUnavailable: RemoteLaunchUriStatus = RemoteLaunchUriStatus(3i32);
    pub const RemoteSystemUnavailable: RemoteLaunchUriStatus = RemoteLaunchUriStatus(4i32);
    pub const ValueSetTooLarge: RemoteLaunchUriStatus = RemoteLaunchUriStatus(5i32);
    pub const DeniedByLocalSystem: RemoteLaunchUriStatus = RemoteLaunchUriStatus(6i32);
    pub const DeniedByRemoteSystem: RemoteLaunchUriStatus = RemoteLaunchUriStatus(7i32);
}
#[repr(transparent)]
pub struct RemoteLauncherOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: ShutdownKind = ShutdownKind(0i32);
    pub const Restart: ShutdownKind = ShutdownKind(1i32);
}
#[repr(C)]
pub struct SystemManagementContract(i32);
#[repr(transparent)]
pub struct User(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: UserAgeConsentGroup = UserAgeConsentGroup(0i32);
    pub const Minor: UserAgeConsentGroup = UserAgeConsentGroup(1i32);
    pub const Adult: UserAgeConsentGroup = UserAgeConsentGroup(2i32);
}
#[repr(transparent)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: UserAgeConsentResult = UserAgeConsentResult(0i32);
    pub const Included: UserAgeConsentResult = UserAgeConsentResult(1i32);
    pub const NotIncluded: UserAgeConsentResult = UserAgeConsentResult(2i32);
    pub const Unknown: UserAgeConsentResult = UserAgeConsentResult(3i32);
    pub const Ambiguous: UserAgeConsentResult = UserAgeConsentResult(4i32);
}
#[repr(transparent)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: UserAuthenticationStatus = UserAuthenticationStatus(0i32);
    pub const LocallyAuthenticated: UserAuthenticationStatus = UserAuthenticationStatus(1i32);
    pub const RemotelyAuthenticated: UserAuthenticationStatus = UserAuthenticationStatus(2i32);
}
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
#[repr(transparent)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: UserPictureSize = UserPictureSize(0i32);
    pub const Size208x208: UserPictureSize = UserPictureSize(1i32);
    pub const Size424x424: UserPictureSize = UserPictureSize(2i32);
    pub const Size1080x1080: UserPictureSize = UserPictureSize(3i32);
}
#[repr(transparent)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: UserType = UserType(0i32);
    pub const RemoteUser: UserType = UserType(1i32);
    pub const LocalGuest: UserType = UserType(2i32);
    pub const RemoteGuest: UserType = UserType(3i32);
    pub const SystemManaged: UserType = UserType(4i32);
}
#[repr(transparent)]
pub struct UserWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: UserWatcherStatus = UserWatcherStatus(0i32);
    pub const Started: UserWatcherStatus = UserWatcherStatus(1i32);
    pub const EnumerationCompleted: UserWatcherStatus = UserWatcherStatus(2i32);
    pub const Stopping: UserWatcherStatus = UserWatcherStatus(3i32);
    pub const Stopped: UserWatcherStatus = UserWatcherStatus(4i32);
    pub const Aborted: UserWatcherStatus = UserWatcherStatus(5i32);
}
#[repr(transparent)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: UserWatcherUpdateKind = UserWatcherUpdateKind(0i32);
    pub const Picture: UserWatcherUpdateKind = UserWatcherUpdateKind(1i32);
}
#[repr(transparent)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: VirtualKey = VirtualKey(0i32);
    pub const LeftButton: VirtualKey = VirtualKey(1i32);
    pub const RightButton: VirtualKey = VirtualKey(2i32);
    pub const Cancel: VirtualKey = VirtualKey(3i32);
    pub const MiddleButton: VirtualKey = VirtualKey(4i32);
    pub const XButton1: VirtualKey = VirtualKey(5i32);
    pub const XButton2: VirtualKey = VirtualKey(6i32);
    pub const Back: VirtualKey = VirtualKey(8i32);
    pub const Tab: VirtualKey = VirtualKey(9i32);
    pub const Clear: VirtualKey = VirtualKey(12i32);
    pub const Enter: VirtualKey = VirtualKey(13i32);
    pub const Shift: VirtualKey = VirtualKey(16i32);
    pub const Control: VirtualKey = VirtualKey(17i32);
    pub const Menu: VirtualKey = VirtualKey(18i32);
    pub const Pause: VirtualKey = VirtualKey(19i32);
    pub const CapitalLock: VirtualKey = VirtualKey(20i32);
    pub const Kana: VirtualKey = VirtualKey(21i32);
    pub const Hangul: VirtualKey = VirtualKey(21i32);
    pub const ImeOn: VirtualKey = VirtualKey(22i32);
    pub const Junja: VirtualKey = VirtualKey(23i32);
    pub const Final: VirtualKey = VirtualKey(24i32);
    pub const Hanja: VirtualKey = VirtualKey(25i32);
    pub const Kanji: VirtualKey = VirtualKey(25i32);
    pub const ImeOff: VirtualKey = VirtualKey(26i32);
    pub const Escape: VirtualKey = VirtualKey(27i32);
    pub const Convert: VirtualKey = VirtualKey(28i32);
    pub const NonConvert: VirtualKey = VirtualKey(29i32);
    pub const Accept: VirtualKey = VirtualKey(30i32);
    pub const ModeChange: VirtualKey = VirtualKey(31i32);
    pub const Space: VirtualKey = VirtualKey(32i32);
    pub const PageUp: VirtualKey = VirtualKey(33i32);
    pub const PageDown: VirtualKey = VirtualKey(34i32);
    pub const End: VirtualKey = VirtualKey(35i32);
    pub const Home: VirtualKey = VirtualKey(36i32);
    pub const Left: VirtualKey = VirtualKey(37i32);
    pub const Up: VirtualKey = VirtualKey(38i32);
    pub const Right: VirtualKey = VirtualKey(39i32);
    pub const Down: VirtualKey = VirtualKey(40i32);
    pub const Select: VirtualKey = VirtualKey(41i32);
    pub const Print: VirtualKey = VirtualKey(42i32);
    pub const Execute: VirtualKey = VirtualKey(43i32);
    pub const Snapshot: VirtualKey = VirtualKey(44i32);
    pub const Insert: VirtualKey = VirtualKey(45i32);
    pub const Delete: VirtualKey = VirtualKey(46i32);
    pub const Help: VirtualKey = VirtualKey(47i32);
    pub const Number0: VirtualKey = VirtualKey(48i32);
    pub const Number1: VirtualKey = VirtualKey(49i32);
    pub const Number2: VirtualKey = VirtualKey(50i32);
    pub const Number3: VirtualKey = VirtualKey(51i32);
    pub const Number4: VirtualKey = VirtualKey(52i32);
    pub const Number5: VirtualKey = VirtualKey(53i32);
    pub const Number6: VirtualKey = VirtualKey(54i32);
    pub const Number7: VirtualKey = VirtualKey(55i32);
    pub const Number8: VirtualKey = VirtualKey(56i32);
    pub const Number9: VirtualKey = VirtualKey(57i32);
    pub const A: VirtualKey = VirtualKey(65i32);
    pub const B: VirtualKey = VirtualKey(66i32);
    pub const C: VirtualKey = VirtualKey(67i32);
    pub const D: VirtualKey = VirtualKey(68i32);
    pub const E: VirtualKey = VirtualKey(69i32);
    pub const F: VirtualKey = VirtualKey(70i32);
    pub const G: VirtualKey = VirtualKey(71i32);
    pub const H: VirtualKey = VirtualKey(72i32);
    pub const I: VirtualKey = VirtualKey(73i32);
    pub const J: VirtualKey = VirtualKey(74i32);
    pub const K: VirtualKey = VirtualKey(75i32);
    pub const L: VirtualKey = VirtualKey(76i32);
    pub const M: VirtualKey = VirtualKey(77i32);
    pub const N: VirtualKey = VirtualKey(78i32);
    pub const O: VirtualKey = VirtualKey(79i32);
    pub const P: VirtualKey = VirtualKey(80i32);
    pub const Q: VirtualKey = VirtualKey(81i32);
    pub const R: VirtualKey = VirtualKey(82i32);
    pub const S: VirtualKey = VirtualKey(83i32);
    pub const T: VirtualKey = VirtualKey(84i32);
    pub const U: VirtualKey = VirtualKey(85i32);
    pub const V: VirtualKey = VirtualKey(86i32);
    pub const W: VirtualKey = VirtualKey(87i32);
    pub const X: VirtualKey = VirtualKey(88i32);
    pub const Y: VirtualKey = VirtualKey(89i32);
    pub const Z: VirtualKey = VirtualKey(90i32);
    pub const LeftWindows: VirtualKey = VirtualKey(91i32);
    pub const RightWindows: VirtualKey = VirtualKey(92i32);
    pub const Application: VirtualKey = VirtualKey(93i32);
    pub const Sleep: VirtualKey = VirtualKey(95i32);
    pub const NumberPad0: VirtualKey = VirtualKey(96i32);
    pub const NumberPad1: VirtualKey = VirtualKey(97i32);
    pub const NumberPad2: VirtualKey = VirtualKey(98i32);
    pub const NumberPad3: VirtualKey = VirtualKey(99i32);
    pub const NumberPad4: VirtualKey = VirtualKey(100i32);
    pub const NumberPad5: VirtualKey = VirtualKey(101i32);
    pub const NumberPad6: VirtualKey = VirtualKey(102i32);
    pub const NumberPad7: VirtualKey = VirtualKey(103i32);
    pub const NumberPad8: VirtualKey = VirtualKey(104i32);
    pub const NumberPad9: VirtualKey = VirtualKey(105i32);
    pub const Multiply: VirtualKey = VirtualKey(106i32);
    pub const Add: VirtualKey = VirtualKey(107i32);
    pub const Separator: VirtualKey = VirtualKey(108i32);
    pub const Subtract: VirtualKey = VirtualKey(109i32);
    pub const Decimal: VirtualKey = VirtualKey(110i32);
    pub const Divide: VirtualKey = VirtualKey(111i32);
    pub const F1: VirtualKey = VirtualKey(112i32);
    pub const F2: VirtualKey = VirtualKey(113i32);
    pub const F3: VirtualKey = VirtualKey(114i32);
    pub const F4: VirtualKey = VirtualKey(115i32);
    pub const F5: VirtualKey = VirtualKey(116i32);
    pub const F6: VirtualKey = VirtualKey(117i32);
    pub const F7: VirtualKey = VirtualKey(118i32);
    pub const F8: VirtualKey = VirtualKey(119i32);
    pub const F9: VirtualKey = VirtualKey(120i32);
    pub const F10: VirtualKey = VirtualKey(121i32);
    pub const F11: VirtualKey = VirtualKey(122i32);
    pub const F12: VirtualKey = VirtualKey(123i32);
    pub const F13: VirtualKey = VirtualKey(124i32);
    pub const F14: VirtualKey = VirtualKey(125i32);
    pub const F15: VirtualKey = VirtualKey(126i32);
    pub const F16: VirtualKey = VirtualKey(127i32);
    pub const F17: VirtualKey = VirtualKey(128i32);
    pub const F18: VirtualKey = VirtualKey(129i32);
    pub const F19: VirtualKey = VirtualKey(130i32);
    pub const F20: VirtualKey = VirtualKey(131i32);
    pub const F21: VirtualKey = VirtualKey(132i32);
    pub const F22: VirtualKey = VirtualKey(133i32);
    pub const F23: VirtualKey = VirtualKey(134i32);
    pub const F24: VirtualKey = VirtualKey(135i32);
    pub const NavigationView: VirtualKey = VirtualKey(136i32);
    pub const NavigationMenu: VirtualKey = VirtualKey(137i32);
    pub const NavigationUp: VirtualKey = VirtualKey(138i32);
    pub const NavigationDown: VirtualKey = VirtualKey(139i32);
    pub const NavigationLeft: VirtualKey = VirtualKey(140i32);
    pub const NavigationRight: VirtualKey = VirtualKey(141i32);
    pub const NavigationAccept: VirtualKey = VirtualKey(142i32);
    pub const NavigationCancel: VirtualKey = VirtualKey(143i32);
    pub const NumberKeyLock: VirtualKey = VirtualKey(144i32);
    pub const Scroll: VirtualKey = VirtualKey(145i32);
    pub const LeftShift: VirtualKey = VirtualKey(160i32);
    pub const RightShift: VirtualKey = VirtualKey(161i32);
    pub const LeftControl: VirtualKey = VirtualKey(162i32);
    pub const RightControl: VirtualKey = VirtualKey(163i32);
    pub const LeftMenu: VirtualKey = VirtualKey(164i32);
    pub const RightMenu: VirtualKey = VirtualKey(165i32);
    pub const GoBack: VirtualKey = VirtualKey(166i32);
    pub const GoForward: VirtualKey = VirtualKey(167i32);
    pub const Refresh: VirtualKey = VirtualKey(168i32);
    pub const Stop: VirtualKey = VirtualKey(169i32);
    pub const Search: VirtualKey = VirtualKey(170i32);
    pub const Favorites: VirtualKey = VirtualKey(171i32);
    pub const GoHome: VirtualKey = VirtualKey(172i32);
    pub const GamepadA: VirtualKey = VirtualKey(195i32);
    pub const GamepadB: VirtualKey = VirtualKey(196i32);
    pub const GamepadX: VirtualKey = VirtualKey(197i32);
    pub const GamepadY: VirtualKey = VirtualKey(198i32);
    pub const GamepadRightShoulder: VirtualKey = VirtualKey(199i32);
    pub const GamepadLeftShoulder: VirtualKey = VirtualKey(200i32);
    pub const GamepadLeftTrigger: VirtualKey = VirtualKey(201i32);
    pub const GamepadRightTrigger: VirtualKey = VirtualKey(202i32);
    pub const GamepadDPadUp: VirtualKey = VirtualKey(203i32);
    pub const GamepadDPadDown: VirtualKey = VirtualKey(204i32);
    pub const GamepadDPadLeft: VirtualKey = VirtualKey(205i32);
    pub const GamepadDPadRight: VirtualKey = VirtualKey(206i32);
    pub const GamepadMenu: VirtualKey = VirtualKey(207i32);
    pub const GamepadView: VirtualKey = VirtualKey(208i32);
    pub const GamepadLeftThumbstickButton: VirtualKey = VirtualKey(209i32);
    pub const GamepadRightThumbstickButton: VirtualKey = VirtualKey(210i32);
    pub const GamepadLeftThumbstickUp: VirtualKey = VirtualKey(211i32);
    pub const GamepadLeftThumbstickDown: VirtualKey = VirtualKey(212i32);
    pub const GamepadLeftThumbstickRight: VirtualKey = VirtualKey(213i32);
    pub const GamepadLeftThumbstickLeft: VirtualKey = VirtualKey(214i32);
    pub const GamepadRightThumbstickUp: VirtualKey = VirtualKey(215i32);
    pub const GamepadRightThumbstickDown: VirtualKey = VirtualKey(216i32);
    pub const GamepadRightThumbstickRight: VirtualKey = VirtualKey(217i32);
    pub const GamepadRightThumbstickLeft: VirtualKey = VirtualKey(218i32);
}
#[repr(transparent)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: VirtualKeyModifiers = VirtualKeyModifiers(0u32);
    pub const Control: VirtualKeyModifiers = VirtualKeyModifiers(1u32);
    pub const Menu: VirtualKeyModifiers = VirtualKeyModifiers(2u32);
    pub const Shift: VirtualKeyModifiers = VirtualKeyModifiers(4u32);
    pub const Windows: VirtualKeyModifiers = VirtualKeyModifiers(8u32);
}
