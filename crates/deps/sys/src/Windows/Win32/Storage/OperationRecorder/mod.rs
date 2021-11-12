#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct OPERATION_END_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_END_PARAMETERS_FLAGS,
}
impl ::core::marker::Copy for OPERATION_END_PARAMETERS {}
impl ::core::clone::Clone for OPERATION_END_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OPERATION_END_PARAMETERS_FLAGS(pub u32);
pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = OPERATION_END_PARAMETERS_FLAGS(1u32);
impl ::core::marker::Copy for OPERATION_END_PARAMETERS_FLAGS {}
impl ::core::clone::Clone for OPERATION_END_PARAMETERS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OPERATION_START_FLAGS(pub u32);
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = OPERATION_START_FLAGS(1u32);
impl ::core::marker::Copy for OPERATION_START_FLAGS {}
impl ::core::clone::Clone for OPERATION_START_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OPERATION_START_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_START_FLAGS,
}
impl ::core::marker::Copy for OPERATION_START_PARAMETERS {}
impl ::core::clone::Clone for OPERATION_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
