#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_OperationRecorder'*"]
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
impl ::core::fmt::Debug for OPERATION_END_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPERATION_END_PARAMETERS").field("Version", &self.Version).field("OperationId", &self.OperationId).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for OPERATION_END_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OPERATION_END_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPERATION_END_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for OPERATION_END_PARAMETERS {}
impl ::core::default::Default for OPERATION_END_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_OperationRecorder'*"]
pub type OPERATION_END_PARAMETERS_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_OperationRecorder'*"]
pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_OperationRecorder'*"]
pub type OPERATION_START_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_OperationRecorder'*"]
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_OperationRecorder'*"]
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
impl ::core::fmt::Debug for OPERATION_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPERATION_START_PARAMETERS").field("Version", &self.Version).field("OperationId", &self.OperationId).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for OPERATION_START_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OPERATION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPERATION_START_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for OPERATION_START_PARAMETERS {}
impl ::core::default::Default for OPERATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_OperationRecorder', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OperationEnd(::core::mem::transmute(operationendparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_OperationRecorder', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OperationStart(::core::mem::transmute(operationstartparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
