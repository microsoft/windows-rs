#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_OperationRecorder`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_OperationRecorder`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL;
}
pub struct OPERATION_END_PARAMETERS(i32);
pub struct OPERATION_END_PARAMETERS_FLAGS(i32);
pub struct OPERATION_START_FLAGS(i32);
pub struct OPERATION_START_PARAMETERS(i32);
