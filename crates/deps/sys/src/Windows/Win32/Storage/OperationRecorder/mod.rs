#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct OPERATION_END_PARAMETERS(i32);
#[repr(transparent)]
pub struct OPERATION_END_PARAMETERS_FLAGS(pub u32);
pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = OPERATION_END_PARAMETERS_FLAGS(1u32);
#[repr(transparent)]
pub struct OPERATION_START_FLAGS(pub u32);
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = OPERATION_START_FLAGS(1u32);
#[repr(C)]
pub struct OPERATION_START_PARAMETERS(i32);
