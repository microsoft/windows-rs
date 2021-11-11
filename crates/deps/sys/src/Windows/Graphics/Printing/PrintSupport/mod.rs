#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPrintSupportExtensionSession();
    fn IPrintSupportExtensionTriggerDetails();
    fn IPrintSupportPrintDeviceCapabilitiesChangedEventArgs();
    fn IPrintSupportPrintTicketValidationRequestedEventArgs();
    fn IPrintSupportSessionInfo();
    fn IPrintSupportSettingsActivatedEventArgs();
    fn IPrintSupportSettingsUISession();
    fn PrintSupportExtensionSession();
    fn PrintSupportExtensionTriggerDetails();
    fn PrintSupportPrintDeviceCapabilitiesChangedEventArgs();
    fn PrintSupportPrintTicketValidationRequestedEventArgs();
    fn PrintSupportSessionInfo();
    fn PrintSupportSettingsActivatedEventArgs();
    fn PrintSupportSettingsUISession();
    fn SettingsLaunchKind();
    fn WorkflowPrintTicketValidationStatus();
}
