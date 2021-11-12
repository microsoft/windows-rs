#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ActivatedEventsContract(i32);
#[repr(C)]
pub struct ActivationCameraSettingsContract(i32);
#[repr(transparent)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: ActivationKind = ActivationKind(0i32);
    pub const Search: ActivationKind = ActivationKind(1i32);
    pub const ShareTarget: ActivationKind = ActivationKind(2i32);
    pub const File: ActivationKind = ActivationKind(3i32);
    pub const Protocol: ActivationKind = ActivationKind(4i32);
    pub const FileOpenPicker: ActivationKind = ActivationKind(5i32);
    pub const FileSavePicker: ActivationKind = ActivationKind(6i32);
    pub const CachedFileUpdater: ActivationKind = ActivationKind(7i32);
    pub const ContactPicker: ActivationKind = ActivationKind(8i32);
    pub const Device: ActivationKind = ActivationKind(9i32);
    pub const PrintTaskSettings: ActivationKind = ActivationKind(10i32);
    pub const CameraSettings: ActivationKind = ActivationKind(11i32);
    pub const RestrictedLaunch: ActivationKind = ActivationKind(12i32);
    pub const AppointmentsProvider: ActivationKind = ActivationKind(13i32);
    pub const Contact: ActivationKind = ActivationKind(14i32);
    pub const LockScreenCall: ActivationKind = ActivationKind(15i32);
    pub const VoiceCommand: ActivationKind = ActivationKind(16i32);
    pub const LockScreen: ActivationKind = ActivationKind(17i32);
    pub const PickerReturned: ActivationKind = ActivationKind(1000i32);
    pub const WalletAction: ActivationKind = ActivationKind(1001i32);
    pub const PickFileContinuation: ActivationKind = ActivationKind(1002i32);
    pub const PickSaveFileContinuation: ActivationKind = ActivationKind(1003i32);
    pub const PickFolderContinuation: ActivationKind = ActivationKind(1004i32);
    pub const WebAuthenticationBrokerContinuation: ActivationKind = ActivationKind(1005i32);
    pub const WebAccountProvider: ActivationKind = ActivationKind(1006i32);
    pub const ComponentUI: ActivationKind = ActivationKind(1007i32);
    pub const ProtocolForResults: ActivationKind = ActivationKind(1009i32);
    pub const ToastNotification: ActivationKind = ActivationKind(1010i32);
    pub const Print3DWorkflow: ActivationKind = ActivationKind(1011i32);
    pub const DialReceiver: ActivationKind = ActivationKind(1012i32);
    pub const DevicePairing: ActivationKind = ActivationKind(1013i32);
    pub const UserDataAccountsProvider: ActivationKind = ActivationKind(1014i32);
    pub const FilePickerExperience: ActivationKind = ActivationKind(1015i32);
    pub const LockScreenComponent: ActivationKind = ActivationKind(1016i32);
    pub const ContactPanel: ActivationKind = ActivationKind(1017i32);
    pub const PrintWorkflowForegroundTask: ActivationKind = ActivationKind(1018i32);
    pub const GameUIProvider: ActivationKind = ActivationKind(1019i32);
    pub const StartupTask: ActivationKind = ActivationKind(1020i32);
    pub const CommandLineLaunch: ActivationKind = ActivationKind(1021i32);
    pub const BarcodeScannerProvider: ActivationKind = ActivationKind(1022i32);
    pub const PrintSupportJobUI: ActivationKind = ActivationKind(1023i32);
    pub const PrintSupportSettingsUI: ActivationKind = ActivationKind(1024i32);
    pub const PhoneCallActivation: ActivationKind = ActivationKind(1025i32);
    pub const VpnForeground: ActivationKind = ActivationKind(1026i32);
}
#[repr(transparent)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: ApplicationExecutionState = ApplicationExecutionState(0i32);
    pub const Running: ApplicationExecutionState = ApplicationExecutionState(1i32);
    pub const Suspended: ApplicationExecutionState = ApplicationExecutionState(2i32);
    pub const Terminated: ApplicationExecutionState = ApplicationExecutionState(3i32);
    pub const ClosedByUser: ApplicationExecutionState = ApplicationExecutionState(4i32);
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
#[repr(C)]
pub struct ContactActivatedEventsContract(i32);
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
#[repr(C)]
pub struct WebUISearchActivatedEventsContract(i32);
