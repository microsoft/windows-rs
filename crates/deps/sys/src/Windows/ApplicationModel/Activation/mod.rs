#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
}
impl ::core::marker::Copy for ActivationKind {}
impl ::core::clone::Clone for ActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationExecutionState {}
impl ::core::clone::Clone for ApplicationExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerPreviewActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CachedFileUpdaterActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandLineActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandLineActivationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactMapActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactMessageActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPanelActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPostActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactVideoCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialReceiverActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileOpenPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileOpenPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileSavePickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileSavePickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivatedEventArgsWithUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICachedFileUpdaterActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandLineActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandLineActivationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactMapActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactMessageActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPanelActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPostActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactVideoCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactsProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContinuationActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePairingActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialReceiverActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerReturnedActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrelaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DWorkflowActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtocolActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtocolForResultsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRestrictedLaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareTargetActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplashScreen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartupTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileActivatedInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewSwitcherProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletActionActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenComponentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerReturnedActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DWorkflowActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtocolActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtocolForResultsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RestrictedLaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareTargetActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplashScreen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StartupTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileActivatedInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletActionActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAuthenticationBrokerContinuationEventArgs(pub *mut ::core::ffi::c_void);
