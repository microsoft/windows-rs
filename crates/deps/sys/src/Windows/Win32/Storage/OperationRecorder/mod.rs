#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct OPERATION_END_PARAMETERS(i32);
#[repr(C)]
pub struct OPERATION_END_PARAMETERS_FLAGS(i32);
#[repr(C)]
pub struct OPERATION_START_FLAGS(i32);
#[repr(C)]
pub struct OPERATION_START_PARAMETERS(i32);
