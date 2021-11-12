#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ActivatedEventsContract(i32);
#[repr(C)]
pub struct ActivationCameraSettingsContract(i32);
#[repr(C)]
pub struct ActivationKind(i32);
#[repr(C)]
pub struct ApplicationExecutionState(i32);
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
