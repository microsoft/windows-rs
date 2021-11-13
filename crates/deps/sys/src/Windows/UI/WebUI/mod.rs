#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_WebUI_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivatedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivatedDeferral {}
impl ::core::clone::Clone for ActivatedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivatedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivatedEventHandler {}
impl ::core::clone::Clone for ActivatedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivatedOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivatedOperation {}
impl ::core::clone::Clone for ActivatedOperation {
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
pub struct BackgroundActivatedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundActivatedEventHandler {}
impl ::core::clone::Clone for BackgroundActivatedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnteredBackgroundEventArgs {}
impl ::core::clone::Clone for EnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnteredBackgroundEventHandler {}
impl ::core::clone::Clone for EnteredBackgroundEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HtmlPrintDocumentSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HtmlPrintDocumentSource {}
impl ::core::clone::Clone for HtmlPrintDocumentSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivatedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivatedDeferral {}
impl ::core::clone::Clone for IActivatedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivatedEventArgsDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivatedEventArgsDeferral {}
impl ::core::clone::Clone for IActivatedEventArgsDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivatedOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivatedOperation {}
impl ::core::clone::Clone for IActivatedOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHtmlPrintDocumentSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHtmlPrintDocumentSource {}
impl ::core::clone::Clone for IHtmlPrintDocumentSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INewWebUIViewCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INewWebUIViewCreatedEventArgs {}
impl ::core::clone::Clone for INewWebUIViewCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIActivationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIActivationStatics {}
impl ::core::clone::Clone for IWebUIActivationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIActivationStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIActivationStatics2 {}
impl ::core::clone::Clone for IWebUIActivationStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIActivationStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIActivationStatics3 {}
impl ::core::clone::Clone for IWebUIActivationStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIActivationStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIActivationStatics4 {}
impl ::core::clone::Clone for IWebUIActivationStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIBackgroundTaskInstance {}
impl ::core::clone::Clone for IWebUIBackgroundTaskInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstanceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIBackgroundTaskInstanceStatics {}
impl ::core::clone::Clone for IWebUIBackgroundTaskInstanceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUINavigatedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUINavigatedDeferral {}
impl ::core::clone::Clone for IWebUINavigatedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUINavigatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUINavigatedEventArgs {}
impl ::core::clone::Clone for IWebUINavigatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUINavigatedOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUINavigatedOperation {}
impl ::core::clone::Clone for IWebUINavigatedOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIView {}
impl ::core::clone::Clone for IWebUIView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUIViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUIViewStatics {}
impl ::core::clone::Clone for IWebUIViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LeavingBackgroundEventArgs {}
impl ::core::clone::Clone for LeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LeavingBackgroundEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LeavingBackgroundEventHandler {}
impl ::core::clone::Clone for LeavingBackgroundEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigatedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigatedEventHandler {}
impl ::core::clone::Clone for NavigatedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NewWebUIViewCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NewWebUIViewCreatedEventArgs {}
impl ::core::clone::Clone for NewWebUIViewCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: Self = Self(0i32);
    pub const CurrentPage: Self = Self(1i32);
    pub const CustomPageRange: Self = Self(2i32);
    pub const CurrentSelection: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintContent {}
impl ::core::clone::Clone for PrintContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResumingEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResumingEventHandler {}
impl ::core::clone::Clone for ResumingEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingDeferral {}
impl ::core::clone::Clone for SuspendingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingEventArgs {}
impl ::core::clone::Clone for SuspendingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingEventHandler {}
impl ::core::clone::Clone for SuspendingEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingOperation {}
impl ::core::clone::Clone for SuspendingOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::clone::Clone for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::clone::Clone for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::clone::Clone for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIBackgroundTaskInstanceRuntimeClass {}
impl ::core::clone::Clone for WebUIBackgroundTaskInstanceRuntimeClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIBarcodeScannerPreviewActivatedEventArgs {}
impl ::core::clone::Clone for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICachedFileUpdaterActivatedEventArgs {}
impl ::core::clone::Clone for WebUICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICameraSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICameraSettingsActivatedEventArgs {}
impl ::core::clone::Clone for WebUICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandLineActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandLineActivatedEventArgs {}
impl ::core::clone::Clone for WebUICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactCallActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactMapActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactMapActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactMessageActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactMessageActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactPanelActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactPanelActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactPickerActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactPostActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactPostActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIContactVideoCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIContactVideoCallActivatedEventArgs {}
impl ::core::clone::Clone for WebUIContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIDeviceActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIDeviceActivatedEventArgs {}
impl ::core::clone::Clone for WebUIDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIDevicePairingActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIDevicePairingActivatedEventArgs {}
impl ::core::clone::Clone for WebUIDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIDialReceiverActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIDialReceiverActivatedEventArgs {}
impl ::core::clone::Clone for WebUIDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIFileActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIFileActivatedEventArgs {}
impl ::core::clone::Clone for WebUIFileActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIFileOpenPickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIFileOpenPickerActivatedEventArgs {}
impl ::core::clone::Clone for WebUIFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIFileOpenPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIFileOpenPickerContinuationEventArgs {}
impl ::core::clone::Clone for WebUIFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIFileSavePickerActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIFileSavePickerActivatedEventArgs {}
impl ::core::clone::Clone for WebUIFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIFileSavePickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIFileSavePickerContinuationEventArgs {}
impl ::core::clone::Clone for WebUIFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIFolderPickerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIFolderPickerContinuationEventArgs {}
impl ::core::clone::Clone for WebUIFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUILaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUILaunchActivatedEventArgs {}
impl ::core::clone::Clone for WebUILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUILockScreenActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUILockScreenActivatedEventArgs {}
impl ::core::clone::Clone for WebUILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUILockScreenCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUILockScreenCallActivatedEventArgs {}
impl ::core::clone::Clone for WebUILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUILockScreenComponentActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUILockScreenComponentActivatedEventArgs {}
impl ::core::clone::Clone for WebUILockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUINavigatedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUINavigatedDeferral {}
impl ::core::clone::Clone for WebUINavigatedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUINavigatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUINavigatedEventArgs {}
impl ::core::clone::Clone for WebUINavigatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUINavigatedOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUINavigatedOperation {}
impl ::core::clone::Clone for WebUINavigatedOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIPhoneCallActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIPhoneCallActivatedEventArgs {}
impl ::core::clone::Clone for WebUIPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIPrint3DWorkflowActivatedEventArgs {}
impl ::core::clone::Clone for WebUIPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIPrintTaskSettingsActivatedEventArgs {}
impl ::core::clone::Clone for WebUIPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {}
impl ::core::clone::Clone for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIProtocolActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIProtocolActivatedEventArgs {}
impl ::core::clone::Clone for WebUIProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIProtocolForResultsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIProtocolForResultsActivatedEventArgs {}
impl ::core::clone::Clone for WebUIProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIRestrictedLaunchActivatedEventArgs {}
impl ::core::clone::Clone for WebUIRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUISearchActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUISearchActivatedEventArgs {}
impl ::core::clone::Clone for WebUISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIShareTargetActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIShareTargetActivatedEventArgs {}
impl ::core::clone::Clone for WebUIShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIStartupTaskActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIStartupTaskActivatedEventArgs {}
impl ::core::clone::Clone for WebUIStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIToastNotificationActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIToastNotificationActivatedEventArgs {}
impl ::core::clone::Clone for WebUIToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIUserDataAccountProviderActivatedEventArgs {}
impl ::core::clone::Clone for WebUIUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIView {}
impl ::core::clone::Clone for WebUIView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIVoiceCommandActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIVoiceCommandActivatedEventArgs {}
impl ::core::clone::Clone for WebUIVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIWalletActionActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIWalletActionActivatedEventArgs {}
impl ::core::clone::Clone for WebUIWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIWebAccountProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIWebAccountProviderActivatedEventArgs {}
impl ::core::clone::Clone for WebUIWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUIWebAuthenticationBrokerContinuationEventArgs {}
impl ::core::clone::Clone for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
