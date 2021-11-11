#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ExtensionsContract();
    fn IPrint3DWorkflow();
    fn IPrint3DWorkflow2();
    fn IPrint3DWorkflowPrintRequestedEventArgs();
    fn IPrint3DWorkflowPrinterChangedEventArgs();
    fn IPrintExtensionContextStatic();
    fn IPrintNotificationEventDetails();
    fn IPrintTaskConfiguration();
    fn IPrintTaskConfigurationSaveRequest();
    fn IPrintTaskConfigurationSaveRequestedDeferral();
    fn IPrintTaskConfigurationSaveRequestedEventArgs();
    fn Print3DWorkflow();
    fn Print3DWorkflowDetail();
    fn Print3DWorkflowPrintRequestedEventArgs();
    fn Print3DWorkflowPrinterChangedEventArgs();
    fn Print3DWorkflowStatus();
    fn PrintExtensionContext();
    fn PrintNotificationEventDetails();
    fn PrintTaskConfiguration();
    fn PrintTaskConfigurationSaveRequest();
    fn PrintTaskConfigurationSaveRequestedDeferral();
    fn PrintTaskConfigurationSaveRequestedEventArgs();
}
