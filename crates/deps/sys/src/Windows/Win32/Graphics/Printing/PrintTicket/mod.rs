#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EDefaultDevmodeType();
    fn EPrintTicketScope();
    fn E_DELTA_PRINTTICKET_FORMAT();
    fn E_PRINTCAPABILITIES_FORMAT();
    fn E_PRINTDEVICECAPABILITIES_FORMAT();
    fn E_PRINTTICKET_FORMAT();
    fn PRINTTICKET_ISTREAM_APIS();
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
    fn S_PT_CONFLICT_RESOLVED();
    fn S_PT_NO_CONFLICT();
}
