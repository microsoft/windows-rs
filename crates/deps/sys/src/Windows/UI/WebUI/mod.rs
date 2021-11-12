#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_WebUI_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivatedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivatedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivatedOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundActivatedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HtmlPrintDocumentSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivatedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivatedEventArgsDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivatedOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHtmlPrintDocumentSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INewWebUIViewCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIActivationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIActivationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIActivationStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIActivationStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstanceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUINavigatedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUINavigatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUINavigatedOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUIViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LeavingBackgroundEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigatedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NewWebUIViewCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PrintContent(i32);
#[repr(transparent)]
pub struct ResumingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIBackgroundTaskInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICameraSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICommandLineActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactMapActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactMessageActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactPanelActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactPostActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIContactVideoCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIDeviceActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIDevicePairingActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIDialReceiverActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIFileActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIFileOpenPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIFileOpenPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIFileSavePickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIFileSavePickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIFolderPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUILaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUILockScreenActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUILockScreenCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUILockScreenComponentActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUINavigatedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUINavigatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUINavigatedOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIPhoneCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIProtocolActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIProtocolForResultsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUISearchActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIShareTargetActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIStartupTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIToastNotificationActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIVoiceCommandActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIWalletActionActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIWebAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(pub *mut ::core::ffi::c_void);
