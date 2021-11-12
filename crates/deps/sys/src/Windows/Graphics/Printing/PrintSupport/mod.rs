#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPrintSupportExtensionSession(pub *mut ::core::ffi::c_void);
pub struct IPrintSupportExtensionTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintSupportSessionInfo(pub *mut ::core::ffi::c_void);
pub struct IPrintSupportSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintSupportSettingsUISession(pub *mut ::core::ffi::c_void);
pub struct PrintSupportExtensionSession(i32);
pub struct PrintSupportExtensionTriggerDetails(i32);
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(i32);
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(i32);
pub struct PrintSupportSessionInfo(i32);
pub struct PrintSupportSettingsActivatedEventArgs(i32);
pub struct PrintSupportSettingsUISession(i32);
pub struct SettingsLaunchKind(i32);
pub struct WorkflowPrintTicketValidationStatus(i32);
