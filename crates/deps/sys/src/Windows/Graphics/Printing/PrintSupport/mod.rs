#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintSupportExtensionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSupportExtensionTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSupportSessionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSupportSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSupportSettingsUISession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportExtensionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportExtensionTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportSessionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSupportSettingsUISession(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SettingsLaunchKind(i32);
#[repr(C)]
pub struct WorkflowPrintTicketValidationStatus(i32);
