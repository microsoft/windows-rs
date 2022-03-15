#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn CreateDXGIFactory<T: ::windows::core::Interface>() -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDXGIFactory(riid: *const ::windows::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateDXGIFactory(&<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn CreateDXGIFactory1<T: ::windows::core::Interface>() -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDXGIFactory1(riid: *const ::windows::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateDXGIFactory1(&<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn CreateDXGIFactory2<T: ::windows::core::Interface>(flags: u32) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDXGIFactory2(flags: u32, riid: *const ::windows::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateDXGIFactory2(::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DXGIDeclareAdapterRemovalSupport() -> ::windows::core::HRESULT;
        }
        DXGIDeclareAdapterRemovalSupport().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[inline]
pub unsafe fn DXGIGetDebugInterface1<T: ::windows::core::Interface>(flags: u32) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DXGIGetDebugInterface1(flags: u32, riid: *const ::windows::core::GUID, pdebug: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        DXGIGetDebugInterface1(::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_ADAPTER_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_ADAPTER_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC1").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_ADAPTER_DESC1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_ADAPTER_DESC1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC2")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_ADAPTER_DESC2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_ADAPTER_DESC2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC3")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_ADAPTER_DESC3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_ADAPTER_DESC3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_ADAPTER_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(2u32);
impl ::core::marker::Copy for DXGI_ADAPTER_FLAG {}
impl ::core::clone::Clone for DXGI_ADAPTER_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_ADAPTER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_ADAPTER_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_ADAPTER_FLAG3(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4294967295u32);
impl ::core::marker::Copy for DXGI_ADAPTER_FLAG3 {}
impl ::core::clone::Clone for DXGI_ADAPTER_FLAG3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_ADAPTER_FLAG3 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_ADAPTER_FLAG3 {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG3").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(4i32);
impl ::core::marker::Copy for DXGI_COMPUTE_PREEMPTION_GRANULARITY {}
impl ::core::clone::Clone for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_COMPUTE_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_DEBUG_RLO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(7u32);
impl ::core::marker::Copy for DXGI_DEBUG_RLO_FLAGS {}
impl ::core::clone::Clone for DXGI_DEBUG_RLO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_DEBUG_RLO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_DEBUG_RLO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_DEBUG_RLO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_DEBUG_RLO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl ::core::marker::Copy for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::clone::Clone for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_DECODE_SWAP_CHAIN_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_DECODE_SWAP_CHAIN_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl ::core::marker::Copy for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::clone::Clone for DXGI_DISPLAY_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DISPLAY_COLOR_SPACE").field("PrimaryCoordinates", &self.PrimaryCoordinates).field("WhitePoints", &self.WhitePoints).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_DISPLAY_COLOR_SPACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_DISPLAY_COLOR_SPACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270485i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ACCESS_LOST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270490i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270474i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270477i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270476i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270475i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270486i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_HUNG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270522i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270523i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_RESET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270521i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270496i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270479i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270517i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270516i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270480i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_INVALID_CALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270527i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270491i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270525i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270484i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270495i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270478i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_CURRENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270482i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270494i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270526i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270493i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270492i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270487i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270483i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270488i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270524i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270489i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005270518i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = DXGI_FEATURE(0i32);
impl ::core::marker::Copy for DXGI_FEATURE {}
impl ::core::clone::Clone for DXGI_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_FEATURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_FRAME_PRESENTATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(3i32);
impl ::core::marker::Copy for DXGI_FRAME_PRESENTATION_MODE {}
impl ::core::clone::Clone for DXGI_FRAME_PRESENTATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_FRAME_PRESENTATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_FRAME_PRESENTATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_FRAME_PRESENTATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FRAME_PRESENTATION_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_FRAME_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_FRAME_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS {}
impl ::core::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS_MEDIA").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).field("CompositionMode", &self.CompositionMode).field("ApprovedPresentDuration", &self.ApprovedPresentDuration).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_FRAME_STATISTICS_MEDIA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_FRAME_STATISTICS_MEDIA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_GPU_PREFERENCE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(2i32);
impl ::core::marker::Copy for DXGI_GPU_PREFERENCE {}
impl ::core::clone::Clone for DXGI_GPU_PREFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_GPU_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_GPU_PREFERENCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_GPU_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GPU_PREFERENCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(4i32);
impl ::core::marker::Copy for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {}
impl ::core::clone::Clone for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GRAPHICS_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(4u32);
impl ::core::marker::Copy for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {}
impl ::core::clone::Clone for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10 {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10").field("RedPrimary", &self.RedPrimary).field("GreenPrimary", &self.GreenPrimary).field("BluePrimary", &self.BluePrimary).field("WhitePoint", &self.WhitePoint).field("MaxMasteringLuminance", &self.MaxMasteringLuminance).field("MinMasteringLuminance", &self.MinMasteringLuminance).field("MaxContentLightLevel", &self.MaxContentLightLevel).field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_HDR_METADATA_HDR10 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_HDR_METADATA_HDR10>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10PLUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10PLUS").field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_HDR_METADATA_HDR10PLUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_HDR_METADATA_HDR10PLUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_HDR_METADATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(2i32);
impl ::core::marker::Copy for DXGI_HDR_METADATA_TYPE {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_HDR_METADATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_HDR_METADATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HDR_METADATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_INFO_QUEUE_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_INFO_QUEUE_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_INFO_QUEUE_FILTER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_INFO_QUEUE_FILTER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows::core::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_MESSAGE").field("Producer", &self.Producer).field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_INFO_QUEUE_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_INFO_QUEUE_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(10i32);
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(4i32);
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl ::core::marker::Copy for DXGI_MAPPED_RECT {}
impl ::core::clone::Clone for DXGI_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MAPPED_RECT").field("Pitch", &self.Pitch).field("pBits", &self.pBits).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_MAPPED_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_MAPPED_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_MAPPED_RECT {}
impl ::core::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_DISCARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_WRITE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for DXGI_MATRIX_3X2_F {}
impl ::core::clone::Clone for DXGI_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_MATRIX_3X2_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MATRIX_3X2_F").field("_11", &self._11).field("_12", &self._12).field("_21", &self._21).field("_22", &self._22).field("_31", &self._31).field("_32", &self._32).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_MATRIX_3X2_F {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_MATRIX_3X2_F>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_MATRIX_3X2_F {}
impl ::core::default::Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_MEMORY_SEGMENT_GROUP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(1i32);
impl ::core::marker::Copy for DXGI_MEMORY_SEGMENT_GROUP {}
impl ::core::clone::Clone for DXGI_MEMORY_SEGMENT_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_MEMORY_SEGMENT_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_MEMORY_SEGMENT_GROUP {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_MEMORY_SEGMENT_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MEMORY_SEGMENT_GROUP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub Format: Common::DXGI_FORMAT,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Stereo: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_MODE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MODE_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Stereo", &self.Stereo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for DXGI_MODE_DESC1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_MODE_DESC1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(4i32);
impl ::core::marker::Copy for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {}
impl ::core::clone::Clone for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_VALID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_Message_Id(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: DXGI_Message_Id = DXGI_Message_Id(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: DXGI_Message_Id = DXGI_Message_Id(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: DXGI_Message_Id = DXGI_Message_Id(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: DXGI_Message_Id = DXGI_Message_Id(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: DXGI_Message_Id = DXGI_Message_Id(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: DXGI_Message_Id = DXGI_Message_Id(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: DXGI_Message_Id = DXGI_Message_Id(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: DXGI_Message_Id = DXGI_Message_Id(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: DXGI_Message_Id = DXGI_Message_Id(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: DXGI_Message_Id = DXGI_Message_Id(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: DXGI_Message_Id = DXGI_Message_Id(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: DXGI_Message_Id = DXGI_Message_Id(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: DXGI_Message_Id = DXGI_Message_Id(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: DXGI_Message_Id = DXGI_Message_Id(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: DXGI_Message_Id = DXGI_Message_Id(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: DXGI_Message_Id = DXGI_Message_Id(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: DXGI_Message_Id = DXGI_Message_Id(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: DXGI_Message_Id = DXGI_Message_Id(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: DXGI_Message_Id = DXGI_Message_Id(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: DXGI_Message_Id = DXGI_Message_Id(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: DXGI_Message_Id = DXGI_Message_Id(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: DXGI_Message_Id = DXGI_Message_Id(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: DXGI_Message_Id = DXGI_Message_Id(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: DXGI_Message_Id = DXGI_Message_Id(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: DXGI_Message_Id = DXGI_Message_Id(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: DXGI_Message_Id = DXGI_Message_Id(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: DXGI_Message_Id = DXGI_Message_Id(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: DXGI_Message_Id = DXGI_Message_Id(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: DXGI_Message_Id = DXGI_Message_Id(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: DXGI_Message_Id = DXGI_Message_Id(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: DXGI_Message_Id = DXGI_Message_Id(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: DXGI_Message_Id = DXGI_Message_Id(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: DXGI_Message_Id = DXGI_Message_Id(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight: DXGI_Message_Id = DXGI_Message_Id(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: DXGI_Message_Id = DXGI_Message_Id(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: DXGI_Message_Id = DXGI_Message_Id(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: DXGI_Message_Id = DXGI_Message_Id(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: DXGI_Message_Id = DXGI_Message_Id(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary: DXGI_Message_Id = DXGI_Message_Id(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: DXGI_Message_Id = DXGI_Message_Id(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: DXGI_Message_Id = DXGI_Message_Id(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: DXGI_Message_Id = DXGI_Message_Id(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: DXGI_Message_Id = DXGI_Message_Id(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: DXGI_Message_Id = DXGI_Message_Id(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: DXGI_Message_Id = DXGI_Message_Id(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL: DXGI_Message_Id = DXGI_Message_Id(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero: DXGI_Message_Id = DXGI_Message_Id(71i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration: DXGI_Message_Id = DXGI_Message_Id(72i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering: DXGI_Message_Id = DXGI_Message_Id(73i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: DXGI_Message_Id = DXGI_Message_Id(74i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination: DXGI_Message_Id = DXGI_Message_Id(75i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(76i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice: DXGI_Message_Id = DXGI_Message_Id(77i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: DXGI_Message_Id = DXGI_Message_Id(78i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: DXGI_Message_Id = DXGI_Message_Id(79i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput: DXGI_Message_Id = DXGI_Message_Id(80i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: DXGI_Message_Id = DXGI_Message_Id(81i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows: DXGI_Message_Id = DXGI_Message_Id(82i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(83i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(84i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(86i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: DXGI_Message_Id = DXGI_Message_Id(87i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: DXGI_Message_Id = DXGI_Message_Id(88i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: DXGI_Message_Id = DXGI_Message_Id(89i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: DXGI_Message_Id = DXGI_Message_Id(91i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: DXGI_Message_Id = DXGI_Message_Id(92i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: DXGI_Message_Id = DXGI_Message_Id(93i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(94i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(95i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(96i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(97i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: DXGI_Message_Id = DXGI_Message_Id(98i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: DXGI_Message_Id = DXGI_Message_Id(99i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(100i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(101i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(102i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(103i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(104i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation: DXGI_Message_Id = DXGI_Message_Id(105i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: DXGI_Message_Id = DXGI_Message_Id(106i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: DXGI_Message_Id = DXGI_Message_Id(107i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: DXGI_Message_Id = DXGI_Message_Id(108i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: DXGI_Message_Id = DXGI_Message_Id(109i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(110i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset: DXGI_Message_Id = DXGI_Message_Id(111i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: DXGI_Message_Id = DXGI_Message_Id(112i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(113i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: DXGI_Message_Id = DXGI_Message_Id(114i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined: DXGI_Message_Id = DXGI_Message_Id(115i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: DXGI_Message_Id = DXGI_Message_Id(116i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition: DXGI_Message_Id = DXGI_Message_Id(117i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(118i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(119i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput: DXGI_Message_Id = DXGI_Message_Id(120i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: DXGI_Message_Id = DXGI_Message_Id(121i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: DXGI_Message_Id = DXGI_Message_Id(122i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: DXGI_Message_Id = DXGI_Message_Id(123i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(124i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(125i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(126i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(127i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: DXGI_Message_Id = DXGI_Message_Id(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(129i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay: DXGI_Message_Id = DXGI_Message_Id(130i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential: DXGI_Message_Id = DXGI_Message_Id(131i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: DXGI_Message_Id = DXGI_Message_Id(132i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity: DXGI_Message_Id = DXGI_Message_Id(133i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(134i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported: DXGI_Message_Id = DXGI_Message_Id(135i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(136i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(137i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(138i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(139i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(140i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(141i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: DXGI_Message_Id = DXGI_Message_Id(142i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(143i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(144i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: DXGI_Message_Id = DXGI_Message_Id(145i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly: DXGI_Message_Id = DXGI_Message_Id(146i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(147i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: DXGI_Message_Id = DXGI_Message_Id(148i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(149i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(150i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: DXGI_Message_Id = DXGI_Message_Id(151i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(152i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(153i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: DXGI_Message_Id = DXGI_Message_Id(154i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: DXGI_Message_Id = DXGI_Message_Id(155i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(156i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(157i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(158i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: DXGI_Message_Id = DXGI_Message_Id(159i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: DXGI_Message_Id = DXGI_Message_Id(160i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: DXGI_Message_Id = DXGI_Message_Id(161i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: DXGI_Message_Id = DXGI_Message_Id(162i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: DXGI_Message_Id = DXGI_Message_Id(163i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(164i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: DXGI_Message_Id = DXGI_Message_Id(165i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(166i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: DXGI_Message_Id = DXGI_Message_Id(167i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: DXGI_Message_Id = DXGI_Message_Id(168i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: DXGI_Message_Id = DXGI_Message_Id(169i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: DXGI_Message_Id = DXGI_Message_Id(170i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: DXGI_Message_Id = DXGI_Message_Id(171i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: DXGI_Message_Id = DXGI_Message_Id(172i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle: DXGI_Message_Id = DXGI_Message_Id(173i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: DXGI_Message_Id = DXGI_Message_Id(174i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer: DXGI_Message_Id = DXGI_Message_Id(175i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: DXGI_Message_Id = DXGI_Message_Id(176i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence: DXGI_Message_Id = DXGI_Message_Id(177i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: DXGI_Message_Id = DXGI_Message_Id(178i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: DXGI_Message_Id = DXGI_Message_Id(179i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: DXGI_Message_Id = DXGI_Message_Id(180i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard: DXGI_Message_Id = DXGI_Message_Id(181i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: DXGI_Message_Id = DXGI_Message_Id(182i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: DXGI_Message_Id = DXGI_Message_Id(183i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(184i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: DXGI_Message_Id = DXGI_Message_Id(185i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(186i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(187i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: DXGI_Message_Id = DXGI_Message_Id(188i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(189i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: DXGI_Message_Id = DXGI_Message_Id(190i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(191i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(192i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(193i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(194i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(195i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(196i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(197i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: DXGI_Message_Id = DXGI_Message_Id(198i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(199i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: DXGI_Message_Id = DXGI_Message_Id(200i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: DXGI_Message_Id = DXGI_Message_Id(201i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: DXGI_Message_Id = DXGI_Message_Id(202i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: DXGI_Message_Id = DXGI_Message_Id(203i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: DXGI_Message_Id = DXGI_Message_Id(204i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: DXGI_Message_Id = DXGI_Message_Id(205i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(206i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: DXGI_Message_Id = DXGI_Message_Id(207i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(208i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(209i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(210i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen: DXGI_Message_Id = DXGI_Message_Id(211i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: DXGI_Message_Id = DXGI_Message_Id(212i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: DXGI_Message_Id = DXGI_Message_Id(213i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(214i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(215i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: DXGI_Message_Id = DXGI_Message_Id(216i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(217i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: DXGI_Message_Id = DXGI_Message_Id(218i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: DXGI_Message_Id = DXGI_Message_Id(219i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(220i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(221i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(222i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: DXGI_Message_Id = DXGI_Message_Id(223i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(224i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(225i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: DXGI_Message_Id = DXGI_Message_Id(226i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale: DXGI_Message_Id = DXGI_Message_Id(227i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(228i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(229i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: DXGI_Message_Id = DXGI_Message_Id(230i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(231i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: DXGI_Message_Id = DXGI_Message_Id(232i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: DXGI_Message_Id = DXGI_Message_Id(233i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: DXGI_Message_Id = DXGI_Message_Id(234i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(235i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: DXGI_Message_Id = DXGI_Message_Id(236i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: DXGI_Message_Id = DXGI_Message_Id(237i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(238i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(239i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(240i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(241i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(242i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(243i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(244i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(245i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: DXGI_Message_Id = DXGI_Message_Id(246i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(247i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(248i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(249i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(250i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: DXGI_Message_Id = DXGI_Message_Id(251i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: DXGI_Message_Id = DXGI_Message_Id(252i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: DXGI_Message_Id = DXGI_Message_Id(253i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: DXGI_Message_Id = DXGI_Message_Id(254i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(255i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: DXGI_Message_Id = DXGI_Message_Id(257i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard: DXGI_Message_Id = DXGI_Message_Id(258i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(259i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(260i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(261i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: DXGI_Message_Id = DXGI_Message_Id(262i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(263i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(264i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(265i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(266i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: DXGI_Message_Id = DXGI_Message_Id(267i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: DXGI_Message_Id = DXGI_Message_Id(268i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: DXGI_Message_Id = DXGI_Message_Id(269i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(270i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(271i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: DXGI_Message_Id = DXGI_Message_Id(272i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: DXGI_Message_Id = DXGI_Message_Id(273i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: DXGI_Message_Id = DXGI_Message_Id(274i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: DXGI_Message_Id = DXGI_Message_Id(275i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(276i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(277i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(278i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(279i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(280i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: DXGI_Message_Id = DXGI_Message_Id(281i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: DXGI_Message_Id = DXGI_Message_Id(282i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: DXGI_Message_Id = DXGI_Message_Id(283i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: DXGI_Message_Id = DXGI_Message_Id(284i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: DXGI_Message_Id = DXGI_Message_Id(285i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(286i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: DXGI_Message_Id = DXGI_Message_Id(287i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(288i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: DXGI_Message_Id = DXGI_Message_Id(289i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(290i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(291i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: DXGI_Message_Id = DXGI_Message_Id(292i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(293i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: DXGI_Message_Id = DXGI_Message_Id(294i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: DXGI_Message_Id = DXGI_Message_Id(295i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: DXGI_Message_Id = DXGI_Message_Id(296i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr: DXGI_Message_Id = DXGI_Message_Id(297i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: DXGI_Message_Id = DXGI_Message_Id(298i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: DXGI_Message_Id = DXGI_Message_Id(1000i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1001i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1002i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1003i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1004i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1005i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1006i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: DXGI_Message_Id = DXGI_Message_Id(1007i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(1008i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: DXGI_Message_Id = DXGI_Message_Id(1009i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(1010i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: DXGI_Message_Id = DXGI_Message_Id(1011i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: DXGI_Message_Id = DXGI_Message_Id(1012i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: DXGI_Message_Id = DXGI_Message_Id(1013i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: DXGI_Message_Id = DXGI_Message_Id(1014i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: DXGI_Message_Id = DXGI_Message_Id(1015i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: DXGI_Message_Id = DXGI_Message_Id(1016i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: DXGI_Message_Id = DXGI_Message_Id(1017i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: DXGI_Message_Id = DXGI_Message_Id(1018i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: DXGI_Message_Id = DXGI_Message_Id(1019i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: DXGI_Message_Id = DXGI_Message_Id(1020i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: DXGI_Message_Id = DXGI_Message_Id(1021i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1022i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1023i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: DXGI_Message_Id = DXGI_Message_Id(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor: DXGI_Message_Id = DXGI_Message_Id(1025i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering: DXGI_Message_Id = DXGI_Message_Id(1026i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1027i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: DXGI_Message_Id = DXGI_Message_Id(1028i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: DXGI_Message_Id = DXGI_Message_Id(1029i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1030i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1031i32);
impl ::core::marker::Copy for DXGI_Message_Id {}
impl ::core::clone::Clone for DXGI_Message_Id {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_Message_Id {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_Message_Id {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_Message_Id {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_Message_Id").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_OFFER_RESOURCE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = DXGI_OFFER_RESOURCE_FLAGS(1i32);
impl ::core::marker::Copy for DXGI_OFFER_RESOURCE_FLAGS {}
impl ::core::clone::Clone for DXGI_OFFER_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OFFER_RESOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_OFFER_RESOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_OFFER_RESOURCE_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(3i32);
impl ::core::marker::Copy for DXGI_OFFER_RESOURCE_PRIORITY {}
impl ::core::clone::Clone for DXGI_OFFER_RESOURCE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OFFER_RESOURCE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_OFFER_RESOURCE_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: Common::DXGI_MODE_DESC,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_OUTDUPL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_DESC").field("ModeDesc", &self.ModeDesc).field("Rotation", &self.Rotation).field("DesktopImageInSystemMemory", &self.DesktopImageInSystemMemory).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTDUPL_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_OUTDUPL_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = DXGI_OUTDUPL_FLAG(1i32);
impl ::core::marker::Copy for DXGI_OUTDUPL_FLAG {}
impl ::core::clone::Clone for DXGI_OUTDUPL_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OUTDUPL_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_FLAG").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::super::Foundation::BOOL,
    pub ProtectedContentMaskedOut: super::super::Foundation::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
            .field("LastPresentTime", &self.LastPresentTime)
            .field("LastMouseUpdateTime", &self.LastMouseUpdateTime)
            .field("AccumulatedFrames", &self.AccumulatedFrames)
            .field("RectsCoalesced", &self.RectsCoalesced)
            .field("ProtectedContentMaskedOut", &self.ProtectedContentMaskedOut)
            .field("PointerPosition", &self.PointerPosition)
            .field("TotalMetadataBufferSize", &self.TotalMetadataBufferSize)
            .field("PointerShapeBufferSize", &self.PointerShapeBufferSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_FRAME_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTDUPL_FRAME_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_MOVE_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_MOVE_RECT").field("SourcePoint", &self.SourcePoint).field("DestinationRect", &self.DestinationRect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_MOVE_RECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTDUPL_MOVE_RECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_POSITION").field("Position", &self.Position).field("Visible", &self.Visible).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_POINTER_POSITION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTDUPL_POINTER_POSITION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO").field("Type", &self.Type).field("Width", &self.Width).field("Height", &self.Height).field("Pitch", &self.Pitch).field("HotSpot", &self.HotSpot).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTDUPL_POINTER_SHAPE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(4i32);
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {}
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_POINTER_SHAPE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC").field("DeviceName", &self.DeviceName).field("DesktopCoordinates", &self.DesktopCoordinates).field("AttachedToDesktop", &self.AttachedToDesktop).field("Rotation", &self.Rotation).field("Monitor", &self.Monitor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DXGI_OUTPUT_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTPUT_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: Common::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC1")
            .field("DeviceName", &self.DeviceName)
            .field("DesktopCoordinates", &self.DesktopCoordinates)
            .field("AttachedToDesktop", &self.AttachedToDesktop)
            .field("Rotation", &self.Rotation)
            .field("Monitor", &self.Monitor)
            .field("BitsPerColor", &self.BitsPerColor)
            .field("ColorSpace", &self.ColorSpace)
            .field("RedPrimary", &self.RedPrimary)
            .field("GreenPrimary", &self.GreenPrimary)
            .field("BluePrimary", &self.BluePrimary)
            .field("WhitePoint", &self.WhitePoint)
            .field("MinLuminance", &self.MinLuminance)
            .field("MaxLuminance", &self.MaxLuminance)
            .field("MaxFullFrameLuminance", &self.MaxFullFrameLuminance)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DXGI_OUTPUT_DESC1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_OUTPUT_DESC1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(1i32);
impl ::core::marker::Copy for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {}
impl ::core::clone::Clone for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_OVERLAY_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(2i32);
impl ::core::marker::Copy for DXGI_OVERLAY_SUPPORT_FLAG {}
impl ::core::clone::Clone for DXGI_OVERLAY_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_OVERLAY_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_OVERLAY_SUPPORT_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_OVERLAY_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_PRESENT_PARAMETERS").field("DirtyRectsCount", &self.DirtyRectsCount).field("pDirtyRects", &self.pDirtyRects).field("pScrollRect", &self.pScrollRect).field("pScrollOffset", &self.pScrollOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_PRESENT_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_PRESENT_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_TEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl ::core::marker::Copy for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::clone::Clone for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO").field("Budget", &self.Budget).field("CurrentUsage", &self.CurrentUsage).field("AvailableForReservation", &self.AvailableForReservation).field("CurrentReservation", &self.CurrentReservation).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_QUERY_VIDEO_MEMORY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_QUERY_VIDEO_MEMORY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(2i32);
impl ::core::marker::Copy for DXGI_RECLAIM_RESOURCE_RESULTS {}
impl ::core::clone::Clone for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_RECLAIM_RESOURCE_RESULTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RECLAIM_RESOURCE_RESULTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_RESIDENCY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = DXGI_RESIDENCY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = DXGI_RESIDENCY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = DXGI_RESIDENCY(3i32);
impl ::core::marker::Copy for DXGI_RESIDENCY {}
impl ::core::clone::Clone for DXGI_RESIDENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_RESIDENCY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_RESIDENCY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_RESIDENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESIDENCY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_RESOURCE_PRIORITY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(671088640u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_LOW: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(1342177280u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(2013265920u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_HIGH: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(2684354560u32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(3355443200u32);
impl ::core::marker::Copy for DXGI_RESOURCE_PRIORITY {}
impl ::core::clone::Clone for DXGI_RESOURCE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_RESOURCE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_RESOURCE_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DXGI_RGBA {}
impl ::core::clone::Clone for DXGI_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_RGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RGBA").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_RGBA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_RGBA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_RGBA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXGI_RGBA {}
impl ::core::default::Default for DXGI_RGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_SCALING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = DXGI_SCALING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_NONE: DXGI_SCALING = DXGI_SCALING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = DXGI_SCALING(2i32);
impl ::core::marker::Copy for DXGI_SCALING {}
impl ::core::clone::Clone for DXGI_SCALING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SCALING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_SCALING {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SCALING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_SHARED_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SHARED_RESOURCE").field("Handle", &self.Handle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DXGI_SHARED_RESOURCE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_SHARED_RESOURCE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SURFACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SURFACE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for DXGI_SURFACE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_SURFACE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(2i32);
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {}
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: Common::DXGI_MODE_DESC,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC").field("BufferDesc", &self.BufferDesc).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("OutputWindow", &self.OutputWindow).field("Windowed", &self.Windowed).field("SwapEffect", &self.SwapEffect).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for DXGI_SWAP_CHAIN_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_SWAP_CHAIN_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: super::super::Foundation::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("Stereo", &self.Stereo).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("Scaling", &self.Scaling).field("SwapEffect", &self.SwapEffect).field("AlphaMode", &self.AlphaMode).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for DXGI_SWAP_CHAIN_DESC1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_SWAP_CHAIN_DESC1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_SWAP_CHAIN_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4096i32);
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FLAG {}
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SWAP_CHAIN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_SWAP_CHAIN_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_FLAG").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Windowed: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC").field("RefreshRate", &self.RefreshRate).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Windowed", &self.Windowed).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXGI_SWAP_EFFECT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(4i32);
impl ::core::marker::Copy for DXGI_SWAP_EFFECT {}
impl ::core::clone::Clone for DXGI_SWAP_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_SWAP_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_SWAP_EFFECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_SWAP_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_EFFECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_READ_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_SHARED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter(::windows::core::IUnknown);
impl IDXGIAdapter {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumOutputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(output), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC> {
        let mut result__: DXGI_ADAPTER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckInterfaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfacename), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
}
impl ::core::convert::From<IDXGIAdapter> for ::windows::core::IUnknown {
    fn from(value: IDXGIAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter> for ::windows::core::IUnknown {
    fn from(value: &IDXGIAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter> for IDXGIObject {
    fn from(value: IDXGIAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter> for IDXGIObject {
    fn from(value: &IDXGIAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter {}
impl ::core::fmt::Debug for IDXGIAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter {
    type Vtable = IDXGIAdapter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
    pub CheckInterfaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacename: *const ::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter1(::windows::core::IUnknown);
impl IDXGIAdapter1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumOutputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(output), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC> {
        let mut result__: DXGI_ADAPTER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CheckInterfaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfacename), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: DXGI_ADAPTER_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
}
impl ::core::convert::From<IDXGIAdapter1> for ::windows::core::IUnknown {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter1> for ::windows::core::IUnknown {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIAdapter1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter1> for IDXGIObject {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter1> for IDXGIObject {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter1> for IDXGIAdapter {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter1> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIAdapter1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter1 {}
impl ::core::fmt::Debug for IDXGIAdapter1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter1 {
    type Vtable = IDXGIAdapter1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29038f61_3839_4626_91fd_086879011a05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_Vtbl {
    pub base: IDXGIAdapter_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter2(::windows::core::IUnknown);
impl IDXGIAdapter2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumOutputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(output), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC> {
        let mut result__: DXGI_ADAPTER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CheckInterfaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfacename), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: DXGI_ADAPTER_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC2> {
        let mut result__: DXGI_ADAPTER_DESC2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC2>(result__)
    }
}
impl ::core::convert::From<IDXGIAdapter2> for ::windows::core::IUnknown {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter2> for ::windows::core::IUnknown {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter2> for IDXGIObject {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter2> for IDXGIObject {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter2> for IDXGIAdapter {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter2> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter2> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter2> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter1> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIAdapter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter2 {}
impl ::core::fmt::Debug for IDXGIAdapter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter2 {
    type Vtable = IDXGIAdapter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_Vtbl {
    pub base: IDXGIAdapter1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc2: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter3(::windows::core::IUnknown);
impl IDXGIAdapter3 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.EnumOutputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(output), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC> {
        let mut result__: DXGI_ADAPTER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CheckInterfaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfacename), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: DXGI_ADAPTER_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC2> {
        let mut result__: DXGI_ADAPTER_DESC2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterHardwareContentProtectionTeardownStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterHardwareContentProtectionTeardownStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP) -> ::windows::core::Result<DXGI_QUERY_VIDEO_MEMORY_INFO> {
        let mut result__: DXGI_QUERY_VIDEO_MEMORY_INFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryVideoMemoryInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(nodeindex), ::core::mem::transmute(memorysegmentgroup), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_QUERY_VIDEO_MEMORY_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVideoMemoryReservation)(::core::mem::transmute_copy(self), ::core::mem::transmute(nodeindex), ::core::mem::transmute(memorysegmentgroup), ::core::mem::transmute(reservation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterVideoMemoryBudgetChangeNotificationEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterVideoMemoryBudgetChangeNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
}
impl ::core::convert::From<IDXGIAdapter3> for ::windows::core::IUnknown {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter3> for ::windows::core::IUnknown {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter3> for IDXGIObject {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter3> for IDXGIObject {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter3> for IDXGIAdapter {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter3> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter3> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter3> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter1> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter3> for IDXGIAdapter2 {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter3> for IDXGIAdapter2 {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter2> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter2> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIAdapter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter3 {}
impl ::core::fmt::Debug for IDXGIAdapter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter3 {
    type Vtable = IDXGIAdapter3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x645967a4_1392_4310_a798_8053ce3e93fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_Vtbl {
    pub base: IDXGIAdapter2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterHardwareContentProtectionTeardownStatusEvent: usize,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterVideoMemoryBudgetChangeNotificationEvent: usize,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIAdapter4(::windows::core::IUnknown);
impl IDXGIAdapter4 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.EnumOutputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(output), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC> {
        let mut result__: DXGI_ADAPTER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.CheckInterfaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfacename), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: DXGI_ADAPTER_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC2> {
        let mut result__: DXGI_ADAPTER_DESC2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisterHardwareContentProtectionTeardownStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.UnregisterHardwareContentProtectionTeardownStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP) -> ::windows::core::Result<DXGI_QUERY_VIDEO_MEMORY_INFO> {
        let mut result__: DXGI_QUERY_VIDEO_MEMORY_INFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.QueryVideoMemoryInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(nodeindex), ::core::mem::transmute(memorysegmentgroup), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_QUERY_VIDEO_MEMORY_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVideoMemoryReservation)(::core::mem::transmute_copy(self), ::core::mem::transmute(nodeindex), ::core::mem::transmute(memorysegmentgroup), ::core::mem::transmute(reservation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisterVideoMemoryBudgetChangeNotificationEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.UnregisterVideoMemoryBudgetChangeNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc3(&self) -> ::windows::core::Result<DXGI_ADAPTER_DESC3> {
        let mut result__: DXGI_ADAPTER_DESC3 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_ADAPTER_DESC3>(result__)
    }
}
impl ::core::convert::From<IDXGIAdapter4> for ::windows::core::IUnknown {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter4> for ::windows::core::IUnknown {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter4> for IDXGIObject {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter4> for IDXGIObject {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter4> for IDXGIAdapter {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter4> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter4> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter4> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter1> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter4> for IDXGIAdapter2 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter4> for IDXGIAdapter2 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter2> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter2> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIAdapter4> for IDXGIAdapter3 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIAdapter4> for IDXGIAdapter3 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter3> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIAdapter3> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIAdapter3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIAdapter4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter4 {}
impl ::core::fmt::Debug for IDXGIAdapter4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIAdapter4 {
    type Vtable = IDXGIAdapter4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_Vtbl {
    pub base: IDXGIAdapter3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc3: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDebug(::windows::core::IUnknown);
impl IDXGIDebug {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReportLiveObjects<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, apiid: Param0, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportLiveObjects)(::core::mem::transmute_copy(self), apiid.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IDXGIDebug> for ::windows::core::IUnknown {
    fn from(value: IDXGIDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDebug> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDebug {}
impl ::core::fmt::Debug for IDXGIDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDebug {
    type Vtable = IDXGIDebug_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x119e7452_de9e_40fe_8806_88f90c12b441);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ReportLiveObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDebug1(::windows::core::IUnknown);
impl IDXGIDebug1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReportLiveObjects<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, apiid: Param0, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ReportLiveObjects)(::core::mem::transmute_copy(self), apiid.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        (::windows::core::Interface::vtable(self).EnableLeakTrackingForThread)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        (::windows::core::Interface::vtable(self).DisableLeakTrackingForThread)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsLeakTrackingEnabledForThread)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDXGIDebug1> for ::windows::core::IUnknown {
    fn from(value: IDXGIDebug1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDebug1> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDebug1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDebug1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDebug1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDebug1> for IDXGIDebug {
    fn from(value: IDXGIDebug1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDebug1> for IDXGIDebug {
    fn from(value: &IDXGIDebug1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDebug> for IDXGIDebug1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDebug> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDebug> for &'a IDXGIDebug1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDebug> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDebug1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDebug1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDebug1 {}
impl ::core::fmt::Debug for IDXGIDebug1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDebug1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDebug1 {
    type Vtable = IDXGIDebug1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_Vtbl {
    pub base: IDXGIDebug_Vtbl,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLeakTrackingEnabledForThread: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDecodeSwapChain(::windows::core::IUnknown);
impl IDXGIDecodeSwapChain {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PresentBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffertopresent), ::core::mem::transmute(syncinterval), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSourceRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDestSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDestSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorSpace)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetColorSpace)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDXGIDecodeSwapChain> for ::windows::core::IUnknown {
    fn from(value: IDXGIDecodeSwapChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDecodeSwapChain> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDecodeSwapChain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDecodeSwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDecodeSwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDecodeSwapChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDecodeSwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDecodeSwapChain {}
impl ::core::fmt::Debug for IDXGIDecodeSwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDecodeSwapChain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDecodeSwapChain {
    type Vtable = IDXGIDecodeSwapChain_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2633066b_4514_4c7a_8fd8_12ea98059d18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub PresentBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourceRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetRect: usize,
    pub SetDestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSourceRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetRect: usize,
    pub GetDestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice(::windows::core::IUnknown);
impl IDXGIDevice {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAdapter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ppsurface.len() as _, ::core::mem::transmute(usage), ::core::mem::transmute(psharedresource), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsurface))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryResourceResidency<'a, const PARAM2: usize>(&self, ppresources: &[::core::option::Option<::windows::core::IUnknown>; PARAM2], presidencystatus: &mut [DXGI_RESIDENCY; PARAM2]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryResourceResidency)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presidencystatus)), PARAM2 as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IDXGIDevice> for ::windows::core::IUnknown {
    fn from(value: IDXGIDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice> for IDXGIObject {
    fn from(value: IDXGIDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice> for IDXGIObject {
    fn from(value: &IDXGIDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice {}
impl ::core::fmt::Debug for IDXGIDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice {
    type Vtable = IDXGIDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice1(::windows::core::IUnknown);
impl IDXGIDevice1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAdapter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ppsurface.len() as _, ::core::mem::transmute(usage), ::core::mem::transmute(psharedresource), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsurface))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryResourceResidency<'a, const PARAM2: usize>(&self, ppresources: &[::core::option::Option<::windows::core::IUnknown>; PARAM2], presidencystatus: &mut [DXGI_RESIDENCY; PARAM2]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.QueryResourceResidency)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presidencystatus)), PARAM2 as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDXGIDevice1> for ::windows::core::IUnknown {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice1> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice1> for IDXGIObject {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice1> for IDXGIObject {
    fn from(value: &IDXGIDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice1> for IDXGIDevice {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice1> for IDXGIDevice {
    fn from(value: &IDXGIDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDevice1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice1 {}
impl ::core::fmt::Debug for IDXGIDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice1 {
    type Vtable = IDXGIDevice1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77db970f_6276_48ba_ba28_070143b4392c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice2(::windows::core::IUnknown);
impl IDXGIDevice2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetAdapter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ppsurface.len() as _, ::core::mem::transmute(usage), ::core::mem::transmute(psharedresource), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsurface))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryResourceResidency<'a, const PARAM2: usize>(&self, ppresources: &[::core::option::Option<::windows::core::IUnknown>; PARAM2], presidencystatus: &mut [DXGI_RESIDENCY; PARAM2]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.QueryResourceResidency)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presidencystatus)), PARAM2 as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OfferResources)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, ppresources: &[::core::option::Option<IDXGIResource>]) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReclaimResources)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnqueueSetEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDXGIDevice2> for ::windows::core::IUnknown {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice2> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice2> for IDXGIObject {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice2> for IDXGIObject {
    fn from(value: &IDXGIDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice2> for IDXGIDevice {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice2> for IDXGIDevice {
    fn from(value: &IDXGIDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice2> for IDXGIDevice1 {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice2> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice1> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice1> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice2 {}
impl ::core::fmt::Debug for IDXGIDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice2 {
    type Vtable = IDXGIDevice2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05008617_fbfd_4051_a790_144884b4f6a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_Vtbl {
    pub base: IDXGIDevice1_Vtbl,
    pub OfferResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReclaimResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReclaimResources: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnqueueSetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnqueueSetEvent: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice3(::windows::core::IUnknown);
impl IDXGIDevice3 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetAdapter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ppsurface.len() as _, ::core::mem::transmute(usage), ::core::mem::transmute(psharedresource), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsurface))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryResourceResidency<'a, const PARAM2: usize>(&self, ppresources: &[::core::option::Option<::windows::core::IUnknown>; PARAM2], presidencystatus: &mut [DXGI_RESIDENCY; PARAM2]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.QueryResourceResidency)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presidencystatus)), PARAM2 as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OfferResources)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, ppresources: &[::core::option::Option<IDXGIResource>]) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReclaimResources)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EnqueueSetEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Trim(&self) {
        (::windows::core::Interface::vtable(self).Trim)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IDXGIDevice3> for ::windows::core::IUnknown {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice3> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice3> for IDXGIObject {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice3> for IDXGIObject {
    fn from(value: &IDXGIDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice3> for IDXGIDevice {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice3> for IDXGIDevice {
    fn from(value: &IDXGIDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice3> for IDXGIDevice1 {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice3> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice1> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice1> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice3> for IDXGIDevice2 {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice3> for IDXGIDevice2 {
    fn from(value: &IDXGIDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice2> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice2> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice3 {}
impl ::core::fmt::Debug for IDXGIDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice3 {
    type Vtable = IDXGIDevice3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6007896c_3244_4afd_bf18_a6d3beda5023);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_Vtbl {
    pub base: IDXGIDevice2_Vtbl,
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDevice4(::windows::core::IUnknown);
impl IDXGIDevice4 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetAdapter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ppsurface.len() as _, ::core::mem::transmute(usage), ::core::mem::transmute(psharedresource), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsurface))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn QueryResourceResidency<'a, const PARAM2: usize>(&self, ppresources: &[::core::option::Option<::windows::core::IUnknown>; PARAM2], presidencystatus: &mut [DXGI_RESIDENCY; PARAM2]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.QueryResourceResidency)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presidencystatus)), PARAM2 as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetGPUThreadPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn OfferResources(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.OfferResources)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, ppresources: &[::core::option::Option<IDXGIResource>]) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ReclaimResources)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.EnqueueSetEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Trim(&self) {
        (::windows::core::Interface::vtable(self).base.Trim)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn OfferResources1(&self, ppresources: &[::core::option::Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OfferResources1)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(priority), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReclaimResources1(&self, ppresources: &[::core::option::Option<IDXGIResource>]) -> ::windows::core::Result<DXGI_RECLAIM_RESOURCE_RESULTS> {
        let mut result__: DXGI_RECLAIM_RESOURCE_RESULTS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReclaimResources1)(::core::mem::transmute_copy(self), ppresources.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_RECLAIM_RESOURCE_RESULTS>(result__)
    }
}
impl ::core::convert::From<IDXGIDevice4> for ::windows::core::IUnknown {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice4> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDevice4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice4> for IDXGIObject {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice4> for IDXGIObject {
    fn from(value: &IDXGIDevice4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice4> for IDXGIDevice {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice4> for IDXGIDevice {
    fn from(value: &IDXGIDevice4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice4> for IDXGIDevice1 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice4> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice1> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice1> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice4> for IDXGIDevice2 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice4> for IDXGIDevice2 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice2> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice2> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDevice4> for IDXGIDevice3 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDevice4> for IDXGIDevice3 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice3> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDevice3> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDevice3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDevice4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice4 {}
impl ::core::fmt::Debug for IDXGIDevice4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDevice4 {
    type Vtable = IDXGIDevice4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_Vtbl {
    pub base: IDXGIDevice3_Vtbl,
    pub OfferResources1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDeviceSubObject(::windows::core::IUnknown);
impl IDXGIDeviceSubObject {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDXGIDeviceSubObject> for ::windows::core::IUnknown {
    fn from(value: IDXGIDeviceSubObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDeviceSubObject> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDeviceSubObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIDeviceSubObject> for IDXGIObject {
    fn from(value: IDXGIDeviceSubObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDeviceSubObject> for IDXGIObject {
    fn from(value: &IDXGIDeviceSubObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDeviceSubObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDeviceSubObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDeviceSubObject {}
impl ::core::fmt::Debug for IDXGIDeviceSubObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDeviceSubObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDeviceSubObject {
    type Vtable = IDXGIDeviceSubObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIDisplayControl(::windows::core::IUnknown);
impl IDXGIDisplayControl {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStereoEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, enabled: Param0) {
        (::windows::core::Interface::vtable(self).SetStereoEnabled)(::core::mem::transmute_copy(self), enabled.into_param().abi())
    }
}
impl ::core::convert::From<IDXGIDisplayControl> for ::windows::core::IUnknown {
    fn from(value: IDXGIDisplayControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIDisplayControl> for ::windows::core::IUnknown {
    fn from(value: &IDXGIDisplayControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIDisplayControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIDisplayControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIDisplayControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIDisplayControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDisplayControl {}
impl ::core::fmt::Debug for IDXGIDisplayControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDisplayControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIDisplayControl {
    type Vtable = IDXGIDisplayControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStereoEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStereoEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory(::windows::core::IUnknown);
impl IDXGIFactory {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
}
impl ::core::convert::From<IDXGIFactory> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory> for IDXGIObject {
    fn from(value: IDXGIFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory> for IDXGIObject {
    fn from(value: &IDXGIFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory {}
impl ::core::fmt::Debug for IDXGIFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory {
    type Vtable = IDXGIFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MakeWindowAssociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MakeWindowAssociation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindowAssociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindowAssociation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSoftwareAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HINSTANCE, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSoftwareAdapter: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory1(::windows::core::IUnknown);
impl IDXGIFactory1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsCurrent)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDXGIFactory1> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory1> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory1> for IDXGIObject {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory1> for IDXGIObject {
    fn from(value: &IDXGIFactory1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory1> for IDXGIFactory {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory1> for IDXGIFactory {
    fn from(value: &IDXGIFactory1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory1 {}
impl ::core::fmt::Debug for IDXGIFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory1 {
    type Vtable = IDXGIFactory1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x770aae78_f26f_4dba_a829_253c83d1b387);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base: IDXGIFactory_Vtbl,
    pub EnumAdapters1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCurrent: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory2(::windows::core::IUnknown);
impl IDXGIFactory2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.IsCurrent)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsWindowedStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hwnd: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Param4) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSwapChainForHwnd)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(pfullscreendesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pwindow: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSwapChainForCoreWindow)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), pwindow.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSharedResourceAdapterLuid)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterStereoStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterStereoStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterStereoStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterOcclusionStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterOcclusionStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).UnregisterOcclusionStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param2) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSwapChainForComposition)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
}
impl ::core::convert::From<IDXGIFactory2> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory2> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory2> for IDXGIObject {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory2> for IDXGIObject {
    fn from(value: &IDXGIFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory2> for IDXGIFactory {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory2> for IDXGIFactory {
    fn from(value: &IDXGIFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory2> for IDXGIFactory1 {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory2> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory2 {}
impl ::core::fmt::Debug for IDXGIFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory2 {
    type Vtable = IDXGIFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base: IDXGIFactory1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWindowedStereoEnabled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForHwnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForCoreWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSharedResourceAdapterLuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterStereoStatusWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterStereoStatusEvent: usize,
    pub UnregisterStereoStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterOcclusionStatusWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterOcclusionStatusEvent: usize,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForComposition: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory3(::windows::core::IUnknown);
impl IDXGIFactory3 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.IsCurrent)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.IsWindowedStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hwnd: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Param4) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSwapChainForHwnd)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(pfullscreendesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pwindow: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSwapChainForCoreWindow)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), pwindow.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSharedResourceAdapterLuid)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisterStereoStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisterStereoStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.UnregisterStereoStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisterOcclusionStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisterOcclusionStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.UnregisterOcclusionStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param2) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSwapChainForComposition)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetCreationFlags)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDXGIFactory3> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory3> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory3> for IDXGIObject {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory3> for IDXGIObject {
    fn from(value: &IDXGIFactory3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory3> for IDXGIFactory {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory3> for IDXGIFactory {
    fn from(value: &IDXGIFactory3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory3> for IDXGIFactory1 {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory3> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory3> for IDXGIFactory2 {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory3> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory3 {}
impl ::core::fmt::Debug for IDXGIFactory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory3 {
    type Vtable = IDXGIFactory3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25483823_cd46_4c7d_86ca_47aa95b837bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_Vtbl {
    pub base: IDXGIFactory2_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory4(::windows::core::IUnknown);
impl IDXGIFactory4 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.IsCurrent)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.IsWindowedStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hwnd: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Param4) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateSwapChainForHwnd)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(pfullscreendesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pwindow: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateSwapChainForCoreWindow)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), pwindow.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSharedResourceAdapterLuid)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.RegisterStereoStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.RegisterStereoStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.UnregisterStereoStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.RegisterOcclusionStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.RegisterOcclusionStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.UnregisterOcclusionStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param2) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateSwapChainForComposition)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetCreationFlags)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LUID>, T: ::windows::core::Interface>(&self, adapterluid: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).EnumAdapterByLuid)(::core::mem::transmute_copy(self), adapterluid.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumWarpAdapter<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).EnumWarpAdapter)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDXGIFactory4> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory4> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory4> for IDXGIObject {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory4> for IDXGIObject {
    fn from(value: &IDXGIFactory4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory4> for IDXGIFactory {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory4> for IDXGIFactory {
    fn from(value: &IDXGIFactory4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory4> for IDXGIFactory1 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory4> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory4> for IDXGIFactory2 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory4> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory4> for IDXGIFactory3 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory4> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory4 {}
impl ::core::fmt::Debug for IDXGIFactory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory4 {
    type Vtable = IDXGIFactory4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_Vtbl {
    pub base: IDXGIFactory3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumAdapterByLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumAdapterByLuid: usize,
    pub EnumWarpAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory5(::windows::core::IUnknown);
impl IDXGIFactory5 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.IsCurrent)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.IsWindowedStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hwnd: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Param4) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CreateSwapChainForHwnd)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(pfullscreendesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pwindow: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CreateSwapChainForCoreWindow)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), pwindow.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetSharedResourceAdapterLuid)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.RegisterStereoStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.RegisterStereoStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.base.UnregisterStereoStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.RegisterOcclusionStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.RegisterOcclusionStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.base.UnregisterOcclusionStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param2) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CreateSwapChainForComposition)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetCreationFlags)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LUID>, T: ::windows::core::Interface>(&self, adapterluid: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.EnumAdapterByLuid)(::core::mem::transmute_copy(self), adapterluid.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumWarpAdapter<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.EnumWarpAdapter)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckFeatureSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(pfeaturesupportdata), ::core::mem::transmute(featuresupportdatasize)).ok()
    }
}
impl ::core::convert::From<IDXGIFactory5> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory5> for IDXGIObject {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for IDXGIObject {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory5> for IDXGIFactory {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for IDXGIFactory {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory5> for IDXGIFactory1 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory5> for IDXGIFactory2 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory5> for IDXGIFactory3 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory5> for IDXGIFactory4 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory5> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory4> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory4> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory5 {}
impl ::core::fmt::Debug for IDXGIFactory5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory5 {
    type Vtable = IDXGIFactory5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_Vtbl {
    pub base: IDXGIFactory4_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory6(::windows::core::IUnknown);
impl IDXGIFactory6 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.base.IsCurrent)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.IsWindowedStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hwnd: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Param4) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.CreateSwapChainForHwnd)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(pfullscreendesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pwindow: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.CreateSwapChainForCoreWindow)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), pwindow.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetSharedResourceAdapterLuid)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.RegisterStereoStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.RegisterStereoStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.base.base.UnregisterStereoStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.RegisterOcclusionStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.RegisterOcclusionStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.base.base.UnregisterOcclusionStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param2) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.CreateSwapChainForComposition)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.GetCreationFlags)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LUID>, T: ::windows::core::Interface>(&self, adapterluid: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.EnumAdapterByLuid)(::core::mem::transmute_copy(self), adapterluid.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumWarpAdapter<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.EnumWarpAdapter)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CheckFeatureSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(pfeaturesupportdata), ::core::mem::transmute(featuresupportdatasize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapterByGpuPreference<T: ::windows::core::Interface>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).EnumAdapterByGpuPreference)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(gpupreference), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDXGIFactory6> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIObject {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIObject {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIFactory {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIFactory {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIFactory1 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIFactory2 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIFactory3 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIFactory4 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory4> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory4> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory6> for IDXGIFactory5 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory6> for IDXGIFactory5 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory5> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory5> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory5> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory5> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory6 {}
impl ::core::fmt::Debug for IDXGIFactory6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory6 {
    type Vtable = IDXGIFactory6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_Vtbl {
    pub base: IDXGIFactory5_Vtbl,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactory7(::windows::core::IUnknown);
impl IDXGIFactory7 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.EnumAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.MakeWindowAssociation)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.GetWindowAssociation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.CreateSwapChain)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, module: Param0) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.CreateSoftwareAdapter)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.EnumAdapters1)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIAdapter1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.base.base.IsCurrent)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.base.IsWindowedStereoEnabled)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hwnd: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: Param4) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.CreateSwapChainForHwnd)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(pfullscreendesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pwindow: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.CreateSwapChainForCoreWindow)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), pwindow.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetSharedResourceAdapterLuid)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::LUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.RegisterStereoStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.RegisterStereoStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.UnregisterStereoStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, windowhandle: Param0, wmsg: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.RegisterOcclusionStatusWindow)(::core::mem::transmute_copy(self), windowhandle.into_param().abi(), ::core::mem::transmute(wmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.RegisterOcclusionStatusEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.UnregisterOcclusionStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param2) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.CreateSwapChainForComposition)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.GetCreationFlags)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LUID>, T: ::windows::core::Interface>(&self, adapterluid: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.EnumAdapterByLuid)(::core::mem::transmute_copy(self), adapterluid.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumWarpAdapter<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.EnumWarpAdapter)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CheckFeatureSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(pfeaturesupportdata), ::core::mem::transmute(featuresupportdatasize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EnumAdapterByGpuPreference<T: ::windows::core::Interface>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.EnumAdapterByGpuPreference)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapter), ::core::mem::transmute(gpupreference), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterAdaptersChangedEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterAdaptersChangedEvent)(::core::mem::transmute_copy(self), hevent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterAdaptersChangedEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<IDXGIFactory7> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIObject {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIObject {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory1 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory2 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory3 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory4 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory4> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory4> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory5 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory5 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory5> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory5> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory5> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory5> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIFactory7> for IDXGIFactory6 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactory7> for IDXGIFactory6 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory6> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory6> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIFactory6> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIFactory6> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactory7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory7 {}
impl ::core::fmt::Debug for IDXGIFactory7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactory7 {
    type Vtable = IDXGIFactory7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_Vtbl {
    pub base: IDXGIFactory6_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterAdaptersChangedEvent: usize,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIFactoryMedia(::windows::core::IUnknown);
impl IDXGIFactoryMedia {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hsurface: Param1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: Param3) -> ::windows::core::Result<IDXGISwapChain1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSwapChainForCompositionSurfaceHandle)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hsurface.into_param().abi(), ::core::mem::transmute(pdesc), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISwapChain1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, IDXGIResource>, Param4: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, pdevice: Param0, hsurface: Param1, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: Param3, prestricttooutput: Param4) -> ::windows::core::Result<IDXGIDecodeSwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDecodeSwapChainForCompositionSurfaceHandle)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), hsurface.into_param().abi(), ::core::mem::transmute(pdesc), pyuvdecodebuffers.into_param().abi(), prestricttooutput.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIDecodeSwapChain>(result__)
    }
}
impl ::core::convert::From<IDXGIFactoryMedia> for ::windows::core::IUnknown {
    fn from(value: IDXGIFactoryMedia) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIFactoryMedia> for ::windows::core::IUnknown {
    fn from(value: &IDXGIFactoryMedia) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIFactoryMedia {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIFactoryMedia {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIFactoryMedia {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIFactoryMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactoryMedia {}
impl ::core::fmt::Debug for IDXGIFactoryMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactoryMedia").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIFactoryMedia {
    type Vtable = IDXGIFactoryMedia_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::windows::core::RawPtr, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDecodeSwapChainForCompositionSurfaceHandle: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIInfoQueue(::windows::core::IUnknown);
impl IDXGIInfoQueue {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMessageCountLimit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, messagecountlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageCountLimit)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(messagecountlimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ClearStoredMessages<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) {
        (::windows::core::Interface::vtable(self).ClearStoredMessages)(::core::mem::transmute_copy(self), producer.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMessage)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(messageindex), ::core::mem::transmute(pmessage), ::core::mem::transmute(pmessagebytelength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilters)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetNumStoredMessages<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumStoredMessages)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMessageCountLimit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetMessageCountLimit)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetNumMessagesAllowedByStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetNumMessagesDeniedByStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn AddStorageFilterEntries<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStorageFilterEntries)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(pfilter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(pfilter), ::core::mem::transmute(pfilterbytelength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ClearStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) {
        (::windows::core::Interface::vtable(self).ClearStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushEmptyStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushEmptyStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushDenyAllStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushDenyAllStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushCopyOfStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushCopyOfStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(pfilter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PopStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) {
        (::windows::core::Interface::vtable(self).PopStorageFilter)(::core::mem::transmute_copy(self), producer.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetStorageFilterStackSize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetStorageFilterStackSize)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn AddRetrievalFilterEntries<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRetrievalFilterEntries)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(pfilter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(pfilter), ::core::mem::transmute(pfilterbytelength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ClearRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) {
        (::windows::core::Interface::vtable(self).ClearRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushEmptyRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushEmptyRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushDenyAllRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushDenyAllRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushCopyOfRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushCopyOfRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PushRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(pfilter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn PopRetrievalFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) {
        (::windows::core::Interface::vtable(self).PopRetrievalFilter)(::core::mem::transmute_copy(self), producer.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetRetrievalFilterStackSize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetRetrievalFilterStackSize)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn AddMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(&self, producer: Param0, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddMessage)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(category), ::core::mem::transmute(severity), ::core::mem::transmute(id), pdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn AddApplicationMessage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddApplicationMessage)(::core::mem::transmute_copy(self), ::core::mem::transmute(severity), pdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, producer: Param0, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBreakOnCategory)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(category), benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, producer: Param0, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBreakOnSeverity)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(severity), benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, producer: Param0, id: i32, benable: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBreakOnID)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(id), benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetBreakOnCategory)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(category)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetBreakOnSeverity)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(severity)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0, id: i32) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetBreakOnID)(::core::mem::transmute_copy(self), producer.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, producer: Param0, bmute: Param1) {
        (::windows::core::Interface::vtable(self).SetMuteDebugOutput)(::core::mem::transmute_copy(self), producer.into_param().abi(), bmute.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, producer: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetMuteDebugOutput)(::core::mem::transmute_copy(self), producer.into_param().abi()))
    }
}
impl ::core::convert::From<IDXGIInfoQueue> for ::windows::core::IUnknown {
    fn from(value: IDXGIInfoQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIInfoQueue> for ::windows::core::IUnknown {
    fn from(value: &IDXGIInfoQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIInfoQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIInfoQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIInfoQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIInfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIInfoQueue {}
impl ::core::fmt::Debug for IDXGIInfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIInfoQueue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIInfoQueue {
    type Vtable = IDXGIInfoQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd67441c7_672a_476f_9e82_cd55b44949ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMuteDebugOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMuteDebugOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIKeyedMutex(::windows::core::IUnknown);
impl IDXGIKeyedMutex {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireSync)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseSync(&self, key: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseSync)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
}
impl ::core::convert::From<IDXGIKeyedMutex> for ::windows::core::IUnknown {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIKeyedMutex> for ::windows::core::IUnknown {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIKeyedMutex> for IDXGIObject {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIKeyedMutex> for IDXGIObject {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIKeyedMutex> for IDXGIDeviceSubObject {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIKeyedMutex> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIKeyedMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIKeyedMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIKeyedMutex {}
impl ::core::fmt::Debug for IDXGIKeyedMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIKeyedMutex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIKeyedMutex {
    type Vtable = IDXGIKeyedMutex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d8e1289_d7b3_465f_8126_250e349af85d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_Vtbl {
    pub base: IDXGIDeviceSubObject_Vtbl,
    pub AcquireSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIObject(::windows::core::IUnknown);
impl IDXGIObject {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDXGIObject> for ::windows::core::IUnknown {
    fn from(value: IDXGIObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIObject> for ::windows::core::IUnknown {
    fn from(value: &IDXGIObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIObject {}
impl ::core::fmt::Debug for IDXGIObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIObject {
    type Vtable = IDXGIObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput(::windows::core::IUnknown);
impl IDXGIOutput {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
}
impl ::core::convert::From<IDXGIOutput> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput> for IDXGIObject {
    fn from(value: IDXGIOutput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput> for IDXGIObject {
    fn from(value: &IDXGIOutput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput {}
impl ::core::fmt::Debug for IDXGIOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput {
    type Vtable = IDXGIOutput_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae02eedb_c735_4690_8d52_5a8dc20213aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_Vtbl {
    pub base: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode: usize,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TakeOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TakeOwnership: usize,
    pub ReleaseOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetGammaControlCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetGammaControlCapabilities: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetGammaControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetGammaControl: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControl: usize,
    pub SetDisplaySurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscanoutsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput1(::windows::core::IUnknown);
impl IDXGIOutput1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base.ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayModeList1)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindClosestMatchingMode1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData1<'a, Param0: ::windows::core::IntoParam<'a, IDXGIResource>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplaySurfaceData1)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DuplicateOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DuplicateOutput)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
}
impl ::core::convert::From<IDXGIOutput1> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput1> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput1> for IDXGIObject {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput1> for IDXGIObject {
    fn from(value: &IDXGIOutput1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput1> for IDXGIOutput {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput1> for IDXGIOutput {
    fn from(value: &IDXGIOutput1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput1 {}
impl ::core::fmt::Debug for IDXGIOutput1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput1 {
    type Vtable = IDXGIOutput1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00cddea8_939b_4b83_a340_a685226666cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_Vtbl {
    pub base: IDXGIOutput_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDisplayModeList1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDisplayModeList1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub FindClosestMatchingMode1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    FindClosestMatchingMode1: usize,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput2(::windows::core::IUnknown);
impl IDXGIOutput2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base.base.ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDisplayModeList1)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FindClosestMatchingMode1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData1<'a, Param0: ::windows::core::IntoParam<'a, IDXGIResource>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDisplaySurfaceData1)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DuplicateOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DuplicateOutput)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).SupportsOverlays)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDXGIOutput2> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput2> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput2> for IDXGIObject {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput2> for IDXGIObject {
    fn from(value: &IDXGIOutput2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput2> for IDXGIOutput {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput2> for IDXGIOutput {
    fn from(value: &IDXGIOutput2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput2> for IDXGIOutput1 {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput2> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput2 {}
impl ::core::fmt::Debug for IDXGIOutput2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput2 {
    type Vtable = IDXGIOutput2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x595e39d1_2724_4663_99b1_da969de28364);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_Vtbl {
    pub base: IDXGIOutput1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsOverlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsOverlays: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput3(::windows::core::IUnknown);
impl IDXGIOutput3 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base.base.base.ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDisplayModeList1)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FindClosestMatchingMode1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData1<'a, Param0: ::windows::core::IntoParam<'a, IDXGIResource>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDisplaySurfaceData1)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DuplicateOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.DuplicateOutput)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.SupportsOverlays)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckOverlaySupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDXGIOutput3> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput3> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput3> for IDXGIObject {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput3> for IDXGIObject {
    fn from(value: &IDXGIOutput3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput3> for IDXGIOutput {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput3> for IDXGIOutput {
    fn from(value: &IDXGIOutput3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput3> for IDXGIOutput1 {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput3> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput3> for IDXGIOutput2 {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput3> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput3 {}
impl ::core::fmt::Debug for IDXGIOutput3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput3 {
    type Vtable = IDXGIOutput3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_Vtbl {
    pub base: IDXGIOutput2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlaySupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlaySupport: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput4(::windows::core::IUnknown);
impl IDXGIOutput4 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base.base.base.base.ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetDisplayModeList1)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.FindClosestMatchingMode1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData1<'a, Param0: ::windows::core::IntoParam<'a, IDXGIResource>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetDisplaySurfaceData1)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DuplicateOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.DuplicateOutput)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.SupportsOverlays)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CheckOverlaySupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckOverlayColorSpaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(colorspace), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDXGIOutput4> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput4> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput4> for IDXGIObject {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput4> for IDXGIObject {
    fn from(value: &IDXGIOutput4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput4> for IDXGIOutput {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput4> for IDXGIOutput {
    fn from(value: &IDXGIOutput4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput4> for IDXGIOutput1 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput4> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput4> for IDXGIOutput2 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput4> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput4> for IDXGIOutput3 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput4> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput3> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput3> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput4 {}
impl ::core::fmt::Debug for IDXGIOutput4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput4 {
    type Vtable = IDXGIOutput4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc7dca35_2196_414d_9f53_617884032a60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_Vtbl {
    pub base: IDXGIOutput3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlayColorSpaceSupport: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput5(::windows::core::IUnknown);
impl IDXGIOutput5 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDisplayModeList1)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.FindClosestMatchingMode1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData1<'a, Param0: ::windows::core::IntoParam<'a, IDXGIResource>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDisplaySurfaceData1)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DuplicateOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.DuplicateOutput)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.SupportsOverlays)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CheckOverlaySupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CheckOverlayColorSpaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(colorspace), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DuplicateOutput1)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(flags), psupportedformats.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psupportedformats)), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
}
impl ::core::convert::From<IDXGIOutput5> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput5> for IDXGIObject {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for IDXGIObject {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput5> for IDXGIOutput {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for IDXGIOutput {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput5> for IDXGIOutput1 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput5> for IDXGIOutput2 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput5> for IDXGIOutput3 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput3> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput3> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput5> for IDXGIOutput4 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput5> for IDXGIOutput4 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput4> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput4> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput5 {}
impl ::core::fmt::Debug for IDXGIOutput5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput5 {
    type Vtable = IDXGIOutput5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80a07424_ab52_42eb_833c_0c42fd282d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_Vtbl {
    pub base: IDXGIOutput4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DuplicateOutput1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DuplicateOutput1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutput6(::windows::core::IUnknown);
impl IDXGIOutput6 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC> {
        let mut result__: DXGI_OUTPUT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetDisplayModeList)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.FindClosestMatchingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.WaitForVBlank)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdevice: Param0, exclusive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.TakeOwnership)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), exclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.ReleaseOwnership)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: Common::DXGI_GAMMA_CONTROL_CAPABILITIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetGammaControlCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(parray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL> {
        let mut result__: Common::DXGI_GAMMA_CONTROL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetGammaControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_GAMMA_CONTROL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pscanoutsurface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetDisplaySurface)(::core::mem::transmute_copy(self), pscanoutsurface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData<'a, Param0: ::windows::core::IntoParam<'a, IDXGISurface>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetDisplaySurfaceData)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetDisplayModeList1)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), ::core::mem::transmute(flags), ::core::mem::transmute(pnummodes), ::core::mem::transmute(pdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.FindClosestMatchingMode1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmodetomatch), ::core::mem::transmute(pclosestmatch), pconcerneddevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDisplaySurfaceData1<'a, Param0: ::windows::core::IntoParam<'a, IDXGIResource>>(&self, pdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetDisplaySurfaceData1)(::core::mem::transmute_copy(self), pdestination.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn DuplicateOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.DuplicateOutput)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.base.SupportsOverlays)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CheckOverlaySupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumformat), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CheckOverlayColorSpaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(colorspace), pconcerneddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pdevice: Param0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows::core::Result<IDXGIOutputDuplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DuplicateOutput1)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(flags), psupportedformats.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psupportedformats)), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_OUTPUT_DESC1> {
        let mut result__: DXGI_OUTPUT_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_OUTPUT_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckHardwareCompositionSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDXGIOutput6> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIObject {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIObject {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIOutput {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIOutput {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIOutput1 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIOutput2 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIOutput3 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput3> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput3> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIOutput4 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIOutput4 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput4> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput4> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutput6> for IDXGIOutput5 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutput6> for IDXGIOutput5 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput5> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput5> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIOutput5> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIOutput5> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutput6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput6 {}
impl ::core::fmt::Debug for IDXGIOutput6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutput6 {
    type Vtable = IDXGIOutput6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x068346e8_aaec_4b84_add7_137f513f77a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_Vtbl {
    pub base: IDXGIOutput5_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIOutputDuplication(::windows::core::IUnknown);
impl IDXGIOutputDuplication {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireNextFrame)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeoutinmilliseconds), ::core::mem::transmute(pframeinfo), ::core::mem::transmute(ppdesktopresource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameDirtyRects)(::core::mem::transmute_copy(self), ::core::mem::transmute(dirtyrectsbuffersize), ::core::mem::transmute(pdirtyrectsbuffer), ::core::mem::transmute(pdirtyrectsbuffersizerequired)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameMoveRects)(::core::mem::transmute_copy(self), ::core::mem::transmute(moverectsbuffersize), ::core::mem::transmute(pmoverectbuffer), ::core::mem::transmute(pmoverectsbuffersizerequired)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFramePointerShape)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointershapebuffersize), ::core::mem::transmute(ppointershapebuffer), ::core::mem::transmute(ppointershapebuffersizerequired), ::core::mem::transmute(ppointershapeinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn MapDesktopSurface(&self) -> ::windows::core::Result<DXGI_MAPPED_RECT> {
        let mut result__: DXGI_MAPPED_RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MapDesktopSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_MAPPED_RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn UnMapDesktopSurface(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnMapDesktopSurface)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn ReleaseFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseFrame)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDXGIOutputDuplication> for ::windows::core::IUnknown {
    fn from(value: IDXGIOutputDuplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutputDuplication> for ::windows::core::IUnknown {
    fn from(value: &IDXGIOutputDuplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIOutputDuplication> for IDXGIObject {
    fn from(value: IDXGIOutputDuplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIOutputDuplication> for IDXGIObject {
    fn from(value: &IDXGIOutputDuplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIOutputDuplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIOutputDuplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutputDuplication {}
impl ::core::fmt::Debug for IDXGIOutputDuplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutputDuplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIOutputDuplication {
    type Vtable = IDXGIOutputDuplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x191cfac3_a341_470d_b26e_a864f428319c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_Vtbl {
    pub base: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireNextFrame: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameDirtyRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameDirtyRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameMoveRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameMoveRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFramePointerShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFramePointerShape: usize,
    pub MapDesktopSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIResource(::windows::core::IUnknown);
impl IDXGIResource {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSharedHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetUsage(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUsage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEvictionPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEvictionPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDXGIResource> for ::windows::core::IUnknown {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource> for ::windows::core::IUnknown {
    fn from(value: &IDXGIResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIResource> for IDXGIObject {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource> for IDXGIObject {
    fn from(value: &IDXGIResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIResource {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIResource {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIResource> for IDXGIDeviceSubObject {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIResource {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGIResource {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIResource {}
impl ::core::fmt::Debug for IDXGIResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIResource {
    type Vtable = IDXGIResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_Vtbl {
    pub base: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSharedHandle: usize,
    pub GetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusage: *mut u32) -> ::windows::core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGIResource1(::windows::core::IUnknown);
impl IDXGIResource1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSharedHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetUsage(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetUsage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEvictionPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetEvictionPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CreateSubresourceSurface(&self, index: u32) -> ::windows::core::Result<IDXGISurface2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSubresourceSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IDXGISurface2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSharedHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(pattributes), ::core::mem::transmute(dwaccess), lpname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IDXGIResource1> for ::windows::core::IUnknown {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource1> for ::windows::core::IUnknown {
    fn from(value: &IDXGIResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIResource1> for IDXGIObject {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource1> for IDXGIObject {
    fn from(value: &IDXGIResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIResource1> for IDXGIDeviceSubObject {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGIResource1> for IDXGIResource {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGIResource1> for IDXGIResource {
    fn from(value: &IDXGIResource1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIResource> for IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIResource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIResource> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIResource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGIResource1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGIResource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIResource1 {}
impl ::core::fmt::Debug for IDXGIResource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIResource1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGIResource1 {
    type Vtable = IDXGIResource1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30961379_4609_4a41_998e_54fe567ee0c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_Vtbl {
    pub base: IDXGIResource_Vtbl,
    pub CreateSubresourceSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows::core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    CreateSharedHandle: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISurface(::windows::core::IUnknown);
impl IDXGISurface {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SURFACE_DESC> {
        let mut result__: DXGI_SURFACE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SURFACE_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Map)(::core::mem::transmute_copy(self), ::core::mem::transmute(plockedrect), ::core::mem::transmute(mapflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unmap)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDXGISurface> for ::windows::core::IUnknown {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface> for ::windows::core::IUnknown {
    fn from(value: &IDXGISurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface> for IDXGIObject {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface> for IDXGIObject {
    fn from(value: &IDXGISurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface {}
impl ::core::fmt::Debug for IDXGISurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISurface {
    type Vtable = IDXGISurface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISurface1(::windows::core::IUnknown);
impl IDXGISurface1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SURFACE_DESC> {
        let mut result__: DXGI_SURFACE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SURFACE_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Map)(::core::mem::transmute_copy(self), ::core::mem::transmute(plockedrect), ::core::mem::transmute(mapflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Unmap)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, discard: Param0) -> ::windows::core::Result<super::Gdi::HDC> {
        let mut result__: super::Gdi::HDC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDC)(::core::mem::transmute_copy(self), discard.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Gdi::HDC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdirtyrect)).ok()
    }
}
impl ::core::convert::From<IDXGISurface1> for ::windows::core::IUnknown {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface1> for ::windows::core::IUnknown {
    fn from(value: &IDXGISurface1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface1> for IDXGIObject {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface1> for IDXGIObject {
    fn from(value: &IDXGISurface1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface1> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface1> for IDXGISurface {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface1> for IDXGISurface {
    fn from(value: &IDXGISurface1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISurface> for IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISurface> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISurface> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISurface> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISurface1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISurface1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface1 {}
impl ::core::fmt::Debug for IDXGISurface1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISurface1 {
    type Vtable = IDXGISurface1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_Vtbl {
    pub base: IDXGISurface_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseDC: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISurface2(::windows::core::IUnknown);
impl IDXGISurface2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SURFACE_DESC> {
        let mut result__: DXGI_SURFACE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SURFACE_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Map)(::core::mem::transmute_copy(self), ::core::mem::transmute(plockedrect), ::core::mem::transmute(mapflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Unmap)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, discard: Param0) -> ::windows::core::Result<super::Gdi::HDC> {
        let mut result__: super::Gdi::HDC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDC)(::core::mem::transmute_copy(self), discard.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Gdi::HDC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ReleaseDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdirtyrect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetResource(&self, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppparentresource), ::core::mem::transmute(psubresourceindex)).ok()
    }
}
impl ::core::convert::From<IDXGISurface2> for ::windows::core::IUnknown {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface2> for ::windows::core::IUnknown {
    fn from(value: &IDXGISurface2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface2> for IDXGIObject {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface2> for IDXGIObject {
    fn from(value: &IDXGISurface2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface2> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface2> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface2> for IDXGISurface {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface2> for IDXGISurface {
    fn from(value: &IDXGISurface2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISurface> for IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISurface> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISurface> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISurface> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISurface2> for IDXGISurface1 {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISurface2> for IDXGISurface1 {
    fn from(value: &IDXGISurface2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISurface1> for IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISurface1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISurface1> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISurface1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISurface2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISurface2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface2 {}
impl ::core::fmt::Debug for IDXGISurface2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISurface2 {
    type Vtable = IDXGISurface2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_Vtbl {
    pub base: IDXGISurface1_Vtbl,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain(::windows::core::IUnknown);
impl IDXGISwapChain {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBuffer<T: ::windows::core::Interface>(&self, buffer: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, fullscreen: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFullscreenState)(::core::mem::transmute_copy(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFullscreenState)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfullscreen), ::core::mem::transmute(pptarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffercount), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(newformat), ::core::mem::transmute(swapchainflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewtargetparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContainingOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastPresentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDXGISwapChain> for ::windows::core::IUnknown {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain> for ::windows::core::IUnknown {
    fn from(value: &IDXGISwapChain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain> for IDXGIObject {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain> for IDXGIObject {
    fn from(value: &IDXGISwapChain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISwapChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain {}
impl ::core::fmt::Debug for IDXGISwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain {
    type Vtable = IDXGISwapChain_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFullscreenState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFullscreenState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullscreenState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullscreenState: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeTarget: usize,
    pub GetContainingOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain1(::windows::core::IUnknown);
impl IDXGISwapChain1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBuffer<T: ::windows::core::Interface>(&self, buffer: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, fullscreen: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFullscreenState)(::core::mem::transmute_copy(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFullscreenState)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfullscreen), ::core::mem::transmute(pptarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResizeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffercount), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(newformat), ::core::mem::transmute(swapchainflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResizeTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewtargetparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetContainingOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLastPresentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: DXGI_SWAP_CHAIN_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_FULLSCREEN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFullscreenDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHwnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCoreWindow<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetCoreWindow)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Present1)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(presentflags), ::core::mem::transmute(ppresentparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsTemporaryMonoSupported)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRestrictToOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__: DXGI_RGBA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_RGBA>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__: Common::DXGI_MODE_ROTATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_MODE_ROTATION>(result__)
    }
}
impl ::core::convert::From<IDXGISwapChain1> for ::windows::core::IUnknown {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain1> for ::windows::core::IUnknown {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain1> for IDXGIObject {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain1> for IDXGIObject {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain1> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain1> for IDXGISwapChain {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain1> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISwapChain1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain1 {}
impl ::core::fmt::Debug for IDXGISwapChain1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain1 {
    type Vtable = IDXGISwapChain1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base: IDXGISwapChain_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetFullscreenDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetFullscreenDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwnd: usize,
    pub GetCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Present1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Present1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTemporaryMonoSupported: usize,
    pub GetRestrictToOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT,
    pub GetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotation: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain2(::windows::core::IUnknown);
impl IDXGISwapChain2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBuffer<T: ::windows::core::Interface>(&self, buffer: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, fullscreen: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetFullscreenState)(::core::mem::transmute_copy(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetFullscreenState)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfullscreen), ::core::mem::transmute(pptarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ResizeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffercount), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(newformat), ::core::mem::transmute(swapchainflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ResizeTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewtargetparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetContainingOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetLastPresentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: DXGI_SWAP_CHAIN_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_FULLSCREEN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFullscreenDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetHwnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCoreWindow<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetCoreWindow)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Present1)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(presentflags), ::core::mem::transmute(ppresentparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.IsTemporaryMonoSupported)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRestrictToOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__: DXGI_RGBA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_RGBA>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__: Common::DXGI_MODE_ROTATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_MODE_ROTATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSourceSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSourceSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetFrameLatencyWaitableObject)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMatrixTransform(&self) -> ::windows::core::Result<DXGI_MATRIX_3X2_F> {
        let mut result__: DXGI_MATRIX_3X2_F = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_MATRIX_3X2_F>(result__)
    }
}
impl ::core::convert::From<IDXGISwapChain2> for ::windows::core::IUnknown {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain2> for ::windows::core::IUnknown {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain2> for IDXGIObject {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain2> for IDXGIObject {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain2> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain2> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain2> for IDXGISwapChain {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain2> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain2> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain2> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain1> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISwapChain2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain2 {}
impl ::core::fmt::Debug for IDXGISwapChain2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain2 {
    type Vtable = IDXGISwapChain2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8be2ac4_199f_4946_b331_79599fb98de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base: IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameLatencyWaitableObject: usize,
    pub SetMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain3(::windows::core::IUnknown);
impl IDXGISwapChain3 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBuffer<T: ::windows::core::Interface>(&self, buffer: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, fullscreen: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetFullscreenState)(::core::mem::transmute_copy(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetFullscreenState)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfullscreen), ::core::mem::transmute(pptarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.ResizeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffercount), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(newformat), ::core::mem::transmute(swapchainflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.ResizeTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewtargetparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetContainingOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetLastPresentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: DXGI_SWAP_CHAIN_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_FULLSCREEN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetFullscreenDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetHwnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCoreWindow<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetCoreWindow)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Present1)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(presentflags), ::core::mem::transmute(ppresentparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.IsTemporaryMonoSupported)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetRestrictToOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__: DXGI_RGBA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_RGBA>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__: Common::DXGI_MODE_ROTATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_MODE_ROTATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSourceSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSourceSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetFrameLatencyWaitableObject)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMatrixTransform(&self) -> ::windows::core::Result<DXGI_MATRIX_3X2_F> {
        let mut result__: DXGI_MATRIX_3X2_F = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_MATRIX_3X2_F>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetCurrentBackBufferIndex)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckColorSpaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorSpace1)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1<'a, const PARAM0: usize>(&self, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: &[u32; PARAM0], pppresentqueue: &[::core::option::Option<::windows::core::IUnknown>; PARAM0]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeBuffers1)(::core::mem::transmute_copy(self), PARAM0 as _, ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(swapchainflags), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcreationnodemask)), ::core::mem::transmute(::windows::core::as_ptr_or_null(pppresentqueue))).ok()
    }
}
impl ::core::convert::From<IDXGISwapChain3> for ::windows::core::IUnknown {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain3> for ::windows::core::IUnknown {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain3> for IDXGIObject {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain3> for IDXGIObject {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain3> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain3> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain3> for IDXGISwapChain {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain3> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain3> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain3> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain1> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain3> for IDXGISwapChain2 {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain3> for IDXGISwapChain2 {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain2> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain2> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISwapChain3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain3 {}
impl ::core::fmt::Debug for IDXGISwapChain3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain3 {
    type Vtable = IDXGISwapChain3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_Vtbl {
    pub base: IDXGISwapChain2_Vtbl,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckColorSpaceSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckColorSpaceSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChain4(::windows::core::IUnknown);
impl IDXGISwapChain4 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, name: *const ::windows::core::GUID, punknown: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.SetPrivateDataInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), punknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetPrivateData)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetParent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.base.GetParent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetDevice)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.Present)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBuffer<T: ::windows::core::Interface>(&self, buffer: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, IDXGIOutput>>(&self, fullscreen: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetFullscreenState)(::core::mem::transmute_copy(self), fullscreen.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetFullscreenState)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfullscreen), ::core::mem::transmute(pptarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.ResizeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffercount), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(newformat), ::core::mem::transmute(swapchainflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.ResizeTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewtargetparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetContainingOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: DXGI_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetLastPresentCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: DXGI_SWAP_CHAIN_DESC1 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDesc1)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: DXGI_SWAP_CHAIN_FULLSCREEN_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetFullscreenDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetHwnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCoreWindow<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.base.GetCoreWindow)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Present1)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncinterval), ::core::mem::transmute(presentflags), ::core::mem::transmute(ppresentparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.base.IsTemporaryMonoSupported)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetRestrictToOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDXGIOutput>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcolor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__: DXGI_RGBA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetBackgroundColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_RGBA>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__: Common::DXGI_MODE_ROTATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<Common::DXGI_MODE_ROTATION>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetSourceSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetSourceSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxlatency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMaximumFrameLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetFrameLatencyWaitableObject)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetMatrixTransform(&self) -> ::windows::core::Result<DXGI_MATRIX_3X2_F> {
        let mut result__: DXGI_MATRIX_3X2_F = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_MATRIX_3X2_F>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetCurrentBackBufferIndex)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CheckColorSpaceSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetColorSpace1)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1<'a, const PARAM0: usize>(&self, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: &[u32; PARAM0], pppresentqueue: &[::core::option::Option<::windows::core::IUnknown>; PARAM0]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResizeBuffers1)(::core::mem::transmute_copy(self), PARAM0 as _, ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(swapchainflags), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcreationnodemask)), ::core::mem::transmute(::windows::core::as_ptr_or_null(pppresentqueue))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, pmetadata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHDRMetaData)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), pmetadata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pmetadata))).ok()
    }
}
impl ::core::convert::From<IDXGISwapChain4> for ::windows::core::IUnknown {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for ::windows::core::IUnknown {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain4> for IDXGIObject {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for IDXGIObject {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain4> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGIDeviceSubObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain4> for IDXGISwapChain {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain4> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain1> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain4> for IDXGISwapChain2 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for IDXGISwapChain2 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain2> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain2> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDXGISwapChain4> for IDXGISwapChain3 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChain4> for IDXGISwapChain3 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain3> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDXGISwapChain3> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::core::Param<'a, IDXGISwapChain3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISwapChain4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain4 {}
impl ::core::fmt::Debug for IDXGISwapChain4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChain4 {
    type Vtable = IDXGISwapChain4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_Vtbl {
    pub base: IDXGISwapChain3_Vtbl,
    pub SetHDRMetaData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGISwapChainMedia(::windows::core::IUnknown);
impl IDXGISwapChainMedia {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn GetFrameStatisticsMedia(&self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS_MEDIA> {
        let mut result__: DXGI_FRAME_STATISTICS_MEDIA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameStatisticsMedia)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DXGI_FRAME_STATISTICS_MEDIA>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPresentDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckPresentDurationSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(desiredpresentduration), ::core::mem::transmute(pclosestsmallerpresentduration), ::core::mem::transmute(pclosestlargerpresentduration)).ok()
    }
}
impl ::core::convert::From<IDXGISwapChainMedia> for ::windows::core::IUnknown {
    fn from(value: IDXGISwapChainMedia) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGISwapChainMedia> for ::windows::core::IUnknown {
    fn from(value: &IDXGISwapChainMedia) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGISwapChainMedia {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGISwapChainMedia {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGISwapChainMedia {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChainMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChainMedia {}
impl ::core::fmt::Debug for IDXGISwapChainMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChainMedia").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGISwapChainMedia {
    type Vtable = IDXGISwapChainMedia_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
#[repr(transparent)]
pub struct IDXGraphicsAnalysis(::windows::core::IUnknown);
impl IDXGraphicsAnalysis {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn BeginCapture(&self) {
        (::windows::core::Interface::vtable(self).BeginCapture)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub unsafe fn EndCapture(&self) {
        (::windows::core::Interface::vtable(self).EndCapture)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IDXGraphicsAnalysis> for ::windows::core::IUnknown {
    fn from(value: IDXGraphicsAnalysis) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXGraphicsAnalysis> for ::windows::core::IUnknown {
    fn from(value: &IDXGraphicsAnalysis) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXGraphicsAnalysis {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXGraphicsAnalysis {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXGraphicsAnalysis {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXGraphicsAnalysis {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGraphicsAnalysis {}
impl ::core::fmt::Debug for IDXGraphicsAnalysis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGraphicsAnalysis").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDXGraphicsAnalysis {
    type Vtable = IDXGraphicsAnalysis_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f251514_9d4d_4902_9d60_18988ab7d4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGraphicsAnalysis_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub BeginCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub EndCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
