#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct PrintTicketFeatureSelectionType(pub i32);
impl PrintTicketFeatureSelectionType {
    pub const PickOne: PrintTicketFeatureSelectionType = PrintTicketFeatureSelectionType(0i32);
    pub const PickMany: PrintTicketFeatureSelectionType = PrintTicketFeatureSelectionType(1i32);
}
#[repr(transparent)]
pub struct PrintTicketOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketParameterDataType(pub i32);
impl PrintTicketParameterDataType {
    pub const Integer: PrintTicketParameterDataType = PrintTicketParameterDataType(0i32);
    pub const NumericString: PrintTicketParameterDataType = PrintTicketParameterDataType(1i32);
    pub const String: PrintTicketParameterDataType = PrintTicketParameterDataType(2i32);
}
#[repr(transparent)]
pub struct PrintTicketParameterDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketParameterInitializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTicketValueType(pub i32);
impl PrintTicketValueType {
    pub const Integer: PrintTicketValueType = PrintTicketValueType(0i32);
    pub const String: PrintTicketValueType = PrintTicketValueType(1i32);
    pub const Unknown: PrintTicketValueType = PrintTicketValueType(2i32);
}
#[repr(transparent)]
pub struct WorkflowPrintTicket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationResult(pub *mut ::core::ffi::c_void);
