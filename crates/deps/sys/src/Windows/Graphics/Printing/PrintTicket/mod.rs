#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintTicketCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTicketFeature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTicketOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTicketParameterDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTicketParameterInitializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTicketValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWorkflowPrintTicket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWorkflowPrintTicketValidationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketFeature(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PrintTicketFeatureSelectionType(i32);
#[repr(transparent)]
pub struct PrintTicketOption(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PrintTicketParameterDataType(i32);
#[repr(transparent)]
pub struct PrintTicketParameterDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketParameterInitializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketValue(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PrintTicketValueType(i32);
#[repr(transparent)]
pub struct WorkflowPrintTicket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationResult(pub *mut ::core::ffi::c_void);
