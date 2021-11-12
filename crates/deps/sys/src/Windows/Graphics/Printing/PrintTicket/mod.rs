#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintTicketCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTicketCapabilities {}
impl ::core::clone::Clone for IPrintTicketCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTicketFeature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTicketFeature {}
impl ::core::clone::Clone for IPrintTicketFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTicketOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTicketOption {}
impl ::core::clone::Clone for IPrintTicketOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTicketParameterDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTicketParameterDefinition {}
impl ::core::clone::Clone for IPrintTicketParameterDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTicketParameterInitializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTicketParameterInitializer {}
impl ::core::clone::Clone for IPrintTicketParameterInitializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintTicketValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintTicketValue {}
impl ::core::clone::Clone for IPrintTicketValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkflowPrintTicket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkflowPrintTicket {}
impl ::core::clone::Clone for IWorkflowPrintTicket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkflowPrintTicketValidationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkflowPrintTicketValidationResult {}
impl ::core::clone::Clone for IWorkflowPrintTicketValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTicketCapabilities {}
impl ::core::clone::Clone for PrintTicketCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketFeature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTicketFeature {}
impl ::core::clone::Clone for PrintTicketFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketFeatureSelectionType(pub i32);
impl PrintTicketFeatureSelectionType {
    pub const PickOne: Self = Self(0i32);
    pub const PickMany: Self = Self(1i32);
}
impl ::core::marker::Copy for PrintTicketFeatureSelectionType {}
impl ::core::clone::Clone for PrintTicketFeatureSelectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTicketOption {}
impl ::core::clone::Clone for PrintTicketOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketParameterDataType(pub i32);
impl PrintTicketParameterDataType {
    pub const Integer: Self = Self(0i32);
    pub const NumericString: Self = Self(1i32);
    pub const String: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintTicketParameterDataType {}
impl ::core::clone::Clone for PrintTicketParameterDataType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketParameterDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTicketParameterDefinition {}
impl ::core::clone::Clone for PrintTicketParameterDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketParameterInitializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTicketParameterInitializer {}
impl ::core::clone::Clone for PrintTicketParameterInitializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintTicketValue {}
impl ::core::clone::Clone for PrintTicketValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintTicketValueType(pub i32);
impl PrintTicketValueType {
    pub const Integer: Self = Self(0i32);
    pub const String: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintTicketValueType {}
impl ::core::clone::Clone for PrintTicketValueType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkflowPrintTicket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WorkflowPrintTicket {}
impl ::core::clone::Clone for WorkflowPrintTicket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WorkflowPrintTicketValidationResult {}
impl ::core::clone::Clone for WorkflowPrintTicketValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
