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
impl ::core::marker::Copy for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::clone::Clone for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::clone::Clone for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundActivatedEventArgs {}
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerPreviewActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerPreviewActivatedEventArgs {}
impl ::core::clone::Clone for BarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CachedFileUpdaterActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CachedFileUpdaterActivatedEventArgs {}
impl ::core::clone::Clone for CachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraSettingsActivatedEventArgs {}
impl ::core::clone::Clone for CameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandLineActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommandLineActivatedEventArgs {}
impl ::core::clone::Clone for CommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandLineActivationOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommandLineActivationOperation {}
impl ::core::clone::Clone for CommandLineActivationOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactCallActivatedEventArgs {}
impl ::core::clone::Clone for ContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactMapActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactMapActivatedEventArgs {}
impl ::core::clone::Clone for ContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactMessageActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactMessageActivatedEventArgs {}
impl ::core::clone::Clone for ContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPanelActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPanelActivatedEventArgs {}
impl ::core::clone::Clone for ContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPickerActivatedEventArgs {}
impl ::core::clone::Clone for ContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPostActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPostActivatedEventArgs {}
impl ::core::clone::Clone for ContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactVideoCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactVideoCallActivatedEventArgs {}
impl ::core::clone::Clone for ContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceActivatedEventArgs {}
impl ::core::clone::Clone for DeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DevicePairingActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DevicePairingActivatedEventArgs {}
impl ::core::clone::Clone for DevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialReceiverActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialReceiverActivatedEventArgs {}
impl ::core::clone::Clone for DialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileActivatedEventArgs {}
impl ::core::clone::Clone for FileActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileOpenPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileOpenPickerActivatedEventArgs {}
impl ::core::clone::Clone for FileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileOpenPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileOpenPickerContinuationEventArgs {}
impl ::core::clone::Clone for FileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileSavePickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileSavePickerActivatedEventArgs {}
impl ::core::clone::Clone for FileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileSavePickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileSavePickerContinuationEventArgs {}
impl ::core::clone::Clone for FileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FolderPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FolderPickerContinuationEventArgs {}
impl ::core::clone::Clone for FolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivatedEventArgs {}
impl ::core::clone::Clone for IActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivatedEventArgsWithUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivatedEventArgsWithUser {}
impl ::core::clone::Clone for IActivatedEventArgsWithUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationViewActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationViewActivatedEventArgs {}
impl ::core::clone::Clone for IApplicationViewActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderActivatedEventArgs {}
impl ::core::clone::Clone for IAppointmentsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::clone::Clone for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::clone::Clone for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundActivatedEventArgs {}
impl ::core::clone::Clone for IBackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerPreviewActivatedEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileUpdaterActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileUpdaterActivatedEventArgs {}
impl ::core::clone::Clone for ICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraSettingsActivatedEventArgs {}
impl ::core::clone::Clone for ICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandLineActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandLineActivatedEventArgs {}
impl ::core::clone::Clone for ICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandLineActivationOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandLineActivationOperation {}
impl ::core::clone::Clone for ICommandLineActivationOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactActivatedEventArgs {}
impl ::core::clone::Clone for IContactActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactCallActivatedEventArgs {}
impl ::core::clone::Clone for IContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactMapActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactMapActivatedEventArgs {}
impl ::core::clone::Clone for IContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactMessageActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactMessageActivatedEventArgs {}
impl ::core::clone::Clone for IContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPanelActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPanelActivatedEventArgs {}
impl ::core::clone::Clone for IContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPickerActivatedEventArgs {}
impl ::core::clone::Clone for IContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPostActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPostActivatedEventArgs {}
impl ::core::clone::Clone for IContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactVideoCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactVideoCallActivatedEventArgs {}
impl ::core::clone::Clone for IContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactsProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactsProviderActivatedEventArgs {}
impl ::core::clone::Clone for IContactsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContinuationActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContinuationActivatedEventArgs {}
impl ::core::clone::Clone for IContinuationActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceActivatedEventArgs {}
impl ::core::clone::Clone for IDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePairingActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePairingActivatedEventArgs {}
impl ::core::clone::Clone for IDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialReceiverActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialReceiverActivatedEventArgs {}
impl ::core::clone::Clone for IDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileActivatedEventArgs {}
impl ::core::clone::Clone for IFileActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileActivatedEventArgsWithCallerPackageFamilyName {}
impl ::core::clone::Clone for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::core::clone::Clone for IFileActivatedEventArgsWithNeighboringFiles {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerActivatedEventArgs {}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerActivatedEventArgs2 {}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerContinuationEventArgs {}
impl ::core::clone::Clone for IFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePickerActivatedEventArgs {}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePickerActivatedEventArgs2 {}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePickerContinuationEventArgs {}
impl ::core::clone::Clone for IFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFolderPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFolderPickerContinuationEventArgs {}
impl ::core::clone::Clone for IFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILaunchActivatedEventArgs {}
impl ::core::clone::Clone for ILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILaunchActivatedEventArgs2 {}
impl ::core::clone::Clone for ILaunchActivatedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenActivatedEventArgs {}
impl ::core::clone::Clone for ILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenCallActivatedEventArgs {}
impl ::core::clone::Clone for ILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallActivatedEventArgs {}
impl ::core::clone::Clone for IPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerReturnedActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerReturnedActivatedEventArgs {}
impl ::core::clone::Clone for IPickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrelaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrelaunchActivatedEventArgs {}
impl ::core::clone::Clone for IPrelaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrint3DWorkflowActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrint3DWorkflowActivatedEventArgs {}
impl ::core::clone::Clone for IPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskSettingsActivatedEventArgs {}
impl ::core::clone::Clone for IPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtocolActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtocolActivatedEventArgs {}
impl ::core::clone::Clone for IProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
impl ::core::clone::Clone for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtocolForResultsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtocolForResultsActivatedEventArgs {}
impl ::core::clone::Clone for IProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRestrictedLaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRestrictedLaunchActivatedEventArgs {}
impl ::core::clone::Clone for IRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchActivatedEventArgs {}
impl ::core::clone::Clone for ISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchActivatedEventArgsWithLinguisticDetails {}
impl ::core::clone::Clone for ISearchActivatedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareTargetActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareTargetActivatedEventArgs {}
impl ::core::clone::Clone for IShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplashScreen(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplashScreen {}
impl ::core::clone::Clone for ISplashScreen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStartupTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStartupTaskActivatedEventArgs {}
impl ::core::clone::Clone for IStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileActivatedInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileActivatedInfo {}
impl ::core::clone::Clone for ITileActivatedInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationActivatedEventArgs {}
impl ::core::clone::Clone for IToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataAccountProviderActivatedEventArgs {}
impl ::core::clone::Clone for IUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IViewSwitcherProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IViewSwitcherProvider {}
impl ::core::clone::Clone for IViewSwitcherProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandActivatedEventArgs {}
impl ::core::clone::Clone for IVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWalletActionActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWalletActionActivatedEventArgs {}
impl ::core::clone::Clone for IWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderActivatedEventArgs {}
impl ::core::clone::Clone for IWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::core::clone::Clone for IWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LaunchActivatedEventArgs {}
impl ::core::clone::Clone for LaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenActivatedEventArgs {}
impl ::core::clone::Clone for LockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenCallActivatedEventArgs {}
impl ::core::clone::Clone for LockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenComponentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenComponentActivatedEventArgs {}
impl ::core::clone::Clone for LockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallActivatedEventArgs {}
impl ::core::clone::Clone for PhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerReturnedActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerReturnedActivatedEventArgs {}
impl ::core::clone::Clone for PickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Print3DWorkflowActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Print3DWorkflowActivatedEventArgs {}
impl ::core::clone::Clone for Print3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskSettingsActivatedEventArgs {}
impl ::core::clone::Clone for PrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtocolActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtocolActivatedEventArgs {}
impl ::core::clone::Clone for ProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtocolForResultsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtocolForResultsActivatedEventArgs {}
impl ::core::clone::Clone for ProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RestrictedLaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RestrictedLaunchActivatedEventArgs {}
impl ::core::clone::Clone for RestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchActivatedEventArgs {}
impl ::core::clone::Clone for SearchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareTargetActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareTargetActivatedEventArgs {}
impl ::core::clone::Clone for ShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplashScreen(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplashScreen {}
impl ::core::clone::Clone for SplashScreen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StartupTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StartupTaskActivatedEventArgs {}
impl ::core::clone::Clone for StartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileActivatedInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileActivatedInfo {}
impl ::core::clone::Clone for TileActivatedInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationActivatedEventArgs {}
impl ::core::clone::Clone for ToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataAccountProviderActivatedEventArgs {}
impl ::core::clone::Clone for UserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandActivatedEventArgs {}
impl ::core::clone::Clone for VoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WalletActionActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WalletActionActivatedEventArgs {}
impl ::core::clone::Clone for WalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderActivatedEventArgs {}
impl ::core::clone::Clone for WebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAuthenticationBrokerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAuthenticationBrokerContinuationEventArgs {}
impl ::core::clone::Clone for WebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
