#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrint3DWorkflow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrint3DWorkflow {}
impl ::core::clone::Clone for IPrint3DWorkflow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrint3DWorkflow2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrint3DWorkflow2 {}
impl ::core::clone::Clone for IPrint3DWorkflow2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrint3DWorkflowPrintRequestedEventArgs {}
impl ::core::clone::Clone for IPrint3DWorkflowPrintRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrint3DWorkflowPrinterChangedEventArgs {}
impl ::core::clone::Clone for IPrint3DWorkflowPrinterChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintExtensionContextStatic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintExtensionContextStatic {}
impl ::core::clone::Clone for IPrintExtensionContextStatic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintNotificationEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintNotificationEventDetails {}
impl ::core::clone::Clone for IPrintNotificationEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskConfiguration {}
impl ::core::clone::Clone for IPrintTaskConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskConfigurationSaveRequest {}
impl ::core::clone::Clone for IPrintTaskConfigurationSaveRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskConfigurationSaveRequestedDeferral {}
impl ::core::clone::Clone for IPrintTaskConfigurationSaveRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTaskConfigurationSaveRequestedEventArgs {}
impl ::core::clone::Clone for IPrintTaskConfigurationSaveRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Print3DWorkflow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Print3DWorkflow {}
impl ::core::clone::Clone for Print3DWorkflow {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for Print3DWorkflowDetail {}
impl ::core::clone::Clone for Print3DWorkflowDetail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Print3DWorkflowPrintRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Print3DWorkflowPrintRequestedEventArgs {}
impl ::core::clone::Clone for Print3DWorkflowPrintRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Print3DWorkflowPrinterChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Print3DWorkflowPrinterChangedEventArgs {}
impl ::core::clone::Clone for Print3DWorkflowPrinterChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Print3DWorkflowStatus(pub i32);
impl Print3DWorkflowStatus {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl ::core::marker::Copy for Print3DWorkflowStatus {}
impl ::core::clone::Clone for Print3DWorkflowStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintNotificationEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintNotificationEventDetails {}
impl ::core::clone::Clone for PrintNotificationEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskConfiguration {}
impl ::core::clone::Clone for PrintTaskConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskConfigurationSaveRequest {}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskConfigurationSaveRequestedDeferral {}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTaskConfigurationSaveRequestedEventArgs {}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
