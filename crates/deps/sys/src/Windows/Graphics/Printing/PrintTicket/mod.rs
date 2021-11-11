#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPrintTicketCapabilities();
    fn IPrintTicketFeature();
    fn IPrintTicketOption();
    fn IPrintTicketParameterDefinition();
    fn IPrintTicketParameterInitializer();
    fn IPrintTicketValue();
    fn IWorkflowPrintTicket();
    fn IWorkflowPrintTicketValidationResult();
    fn PrintTicketCapabilities();
    fn PrintTicketFeature();
    fn PrintTicketFeatureSelectionType();
    fn PrintTicketOption();
    fn PrintTicketParameterDataType();
    fn PrintTicketParameterDefinition();
    fn PrintTicketParameterInitializer();
    fn PrintTicketValue();
    fn PrintTicketValueType();
    fn WorkflowPrintTicket();
    fn WorkflowPrintTicketValidationResult();
}
