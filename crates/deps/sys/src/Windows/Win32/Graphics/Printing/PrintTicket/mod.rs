#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn PTCloseProvider();
    fn PTConvertDevModeToPrintTicket();
    fn PTConvertPrintTicketToDevMode();
    fn PTGetPrintCapabilities();
    fn PTGetPrintDeviceCapabilities();
    fn PTGetPrintDeviceResources();
    fn PTMergeAndValidatePrintTicket();
    fn PTOpenProvider();
    fn PTOpenProviderEx();
    fn PTQuerySchemaVersionSupport();
    fn PTReleaseMemory();
}
