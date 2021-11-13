#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintSupportExtensionSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportExtensionSession {}
impl ::core::clone::Clone for IPrintSupportExtensionSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintSupportExtensionTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportExtensionTriggerDetails {}
impl ::core::clone::Clone for IPrintSupportExtensionTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
impl ::core::clone::Clone for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportPrintTicketValidationRequestedEventArgs {}
impl ::core::clone::Clone for IPrintSupportPrintTicketValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintSupportSessionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportSessionInfo {}
impl ::core::clone::Clone for IPrintSupportSessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintSupportSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportSettingsActivatedEventArgs {}
impl ::core::clone::Clone for IPrintSupportSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintSupportSettingsUISession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintSupportSettingsUISession {}
impl ::core::clone::Clone for IPrintSupportSettingsUISession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportExtensionSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportExtensionSession {}
impl ::core::clone::Clone for PrintSupportExtensionSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportExtensionTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportExtensionTriggerDetails {}
impl ::core::clone::Clone for PrintSupportExtensionTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
impl ::core::clone::Clone for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportPrintTicketValidationRequestedEventArgs {}
impl ::core::clone::Clone for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportSessionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportSessionInfo {}
impl ::core::clone::Clone for PrintSupportSessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportSettingsActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportSettingsActivatedEventArgs {}
impl ::core::clone::Clone for PrintSupportSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintSupportSettingsUISession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintSupportSettingsUISession {}
impl ::core::clone::Clone for PrintSupportSettingsUISession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: Self = Self(0i32);
    pub const UserDefaultPrintTicket: Self = Self(1i32);
}
impl ::core::marker::Copy for SettingsLaunchKind {}
impl ::core::clone::Clone for SettingsLaunchKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: Self = Self(0i32);
    pub const Conflicting: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
}
impl ::core::marker::Copy for WorkflowPrintTicketValidationStatus {}
impl ::core::clone::Clone for WorkflowPrintTicketValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
