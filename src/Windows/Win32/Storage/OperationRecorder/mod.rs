#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_OperationRecorder`*"]
pub struct OPERATION_END_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_END_PARAMETERS_FLAGS,
}
impl OPERATION_END_PARAMETERS {}
impl ::core::default::Default for OPERATION_END_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OPERATION_END_PARAMETERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OPERATION_END_PARAMETERS").field("Version", &self.Version).field("OperationId", &self.OperationId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for OPERATION_END_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.OperationId == other.OperationId && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for OPERATION_END_PARAMETERS {}
unsafe impl ::windows::core::Abi for OPERATION_END_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OperationRecorder`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPERATION_END_PARAMETERS_FLAGS(pub u32);
pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = OPERATION_END_PARAMETERS_FLAGS(1u32);
impl ::core::convert::From<u32> for OPERATION_END_PARAMETERS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPERATION_END_PARAMETERS_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for OPERATION_END_PARAMETERS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for OPERATION_END_PARAMETERS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_OperationRecorder`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPERATION_START_FLAGS(pub u32);
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = OPERATION_START_FLAGS(1u32);
impl ::core::convert::From<u32> for OPERATION_START_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPERATION_START_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for OPERATION_START_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for OPERATION_START_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for OPERATION_START_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for OPERATION_START_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for OPERATION_START_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_OperationRecorder`*"]
pub struct OPERATION_START_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_START_FLAGS,
}
impl OPERATION_START_PARAMETERS {}
impl ::core::default::Default for OPERATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OPERATION_START_PARAMETERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OPERATION_START_PARAMETERS").field("Version", &self.Version).field("OperationId", &self.OperationId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for OPERATION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.OperationId == other.OperationId && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for OPERATION_START_PARAMETERS {}
unsafe impl ::windows::core::Abi for OPERATION_START_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_OperationRecorder`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Storage_OperationRecorder`, `Win32_Foundation`*"]
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
