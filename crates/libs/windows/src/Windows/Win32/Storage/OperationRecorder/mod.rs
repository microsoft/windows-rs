#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "advapi32.dll""system" fn OperationEnd ( operationendparams : *const OPERATION_END_PARAMETERS ) -> super::super::Foundation:: BOOL );
    OperationEnd(operationendparams)
}
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "advapi32.dll""system" fn OperationStart ( operationstartparams : *const OPERATION_START_PARAMETERS ) -> super::super::Foundation:: BOOL );
    OperationStart(operationstartparams)
}
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPERATION_END_PARAMETERS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`*"]
pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = OPERATION_END_PARAMETERS_FLAGS(1u32);
impl ::core::marker::Copy for OPERATION_END_PARAMETERS_FLAGS {}
impl ::core::clone::Clone for OPERATION_END_PARAMETERS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OPERATION_END_PARAMETERS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPERATION_START_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`*"]
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = OPERATION_START_FLAGS(1u32);
impl ::core::marker::Copy for OPERATION_START_FLAGS {}
impl ::core::clone::Clone for OPERATION_START_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OPERATION_START_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`*"]
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
unsafe impl ::windows::core::Abi for OPERATION_END_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_OperationRecorder\"`*"]
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
unsafe impl ::windows::core::Abi for OPERATION_START_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
