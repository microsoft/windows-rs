#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct Print3DWorkflowDetail(pub i32);
impl Print3DWorkflowDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
#[repr(transparent)]
pub struct Print3DWorkflowPrintRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DWorkflowPrinterChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DWorkflowStatus(pub i32);
impl Print3DWorkflowStatus {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
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
