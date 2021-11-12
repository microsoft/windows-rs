#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ExtensionsContract(i32);
#[repr(transparent)]
pub struct IPrint3DWorkflow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DWorkflow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintExtensionContextStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintNotificationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DWorkflow(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Print3DWorkflowDetail(i32);
#[repr(transparent)]
pub struct Print3DWorkflowPrintRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DWorkflowPrinterChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Print3DWorkflowStatus(i32);
#[repr(transparent)]
pub struct PrintExtensionContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintNotificationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedEventArgs(pub *mut ::core::ffi::c_void);
