#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPrintTicketCapabilities(pub *mut ::core::ffi::c_void);
pub struct IPrintTicketFeature(pub *mut ::core::ffi::c_void);
pub struct IPrintTicketOption(pub *mut ::core::ffi::c_void);
pub struct IPrintTicketParameterDefinition(pub *mut ::core::ffi::c_void);
pub struct IPrintTicketParameterInitializer(pub *mut ::core::ffi::c_void);
pub struct IPrintTicketValue(pub *mut ::core::ffi::c_void);
pub struct IWorkflowPrintTicket(pub *mut ::core::ffi::c_void);
pub struct IWorkflowPrintTicketValidationResult(pub *mut ::core::ffi::c_void);
pub struct PrintTicketCapabilities(i32);
pub struct PrintTicketFeature(i32);
pub struct PrintTicketFeatureSelectionType(i32);
pub struct PrintTicketOption(i32);
pub struct PrintTicketParameterDataType(i32);
pub struct PrintTicketParameterDefinition(i32);
pub struct PrintTicketParameterInitializer(i32);
pub struct PrintTicketValue(i32);
pub struct PrintTicketValueType(i32);
pub struct WorkflowPrintTicket(i32);
pub struct WorkflowPrintTicketValidationResult(i32);
