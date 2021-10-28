#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub unsafe fn CreateDXGIFactory<T: ::windows::runtime::Interface>() -> ::windows::runtime::Result<T>
{
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDXGIFactory(
                riid: *const ::windows::runtime::GUID,
                ppfactory: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CreateDXGIFactory(
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateDXGIFactory1<T: ::windows::runtime::Interface>() -> ::windows::runtime::Result<T>
{
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDXGIFactory1(
                riid: *const ::windows::runtime::GUID,
                ppfactory: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CreateDXGIFactory1(
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateDXGIFactory2<T: ::windows::runtime::Interface>(
    flags: u32,
) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDXGIFactory2(
                flags: u32,
                riid: *const ::windows::runtime::GUID,
                ppfactory: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CreateDXGIFactory2(
            ::std::mem::transmute(flags),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DXGIDeclareAdapterRemovalSupport() -> ::windows::runtime::HRESULT;
        }
        DXGIDeclareAdapterRemovalSupport().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DXGIGetDebugInterface1<T: ::windows::runtime::Interface>(
    flags: u32,
) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DXGIGetDebugInterface1(
                flags: u32,
                riid: *const ::windows::runtime::GUID,
                pdebug: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        DXGIGetDebugInterface1(
            ::std::mem::transmute(flags),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::System::SystemServices::LUID,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DXGI_ADAPTER_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::System::SystemServices::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC1")
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
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DXGI_ADAPTER_DESC1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::System::SystemServices::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC2")
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
            .field(
                "GraphicsPreemptionGranularity",
                &self.GraphicsPreemptionGranularity,
            )
            .field(
                "ComputePreemptionGranularity",
                &self.ComputePreemptionGranularity,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
            && self.Flags == other.Flags
            && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity
            && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DXGI_ADAPTER_DESC2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::System::SystemServices::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC3")
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
            .field(
                "GraphicsPreemptionGranularity",
                &self.GraphicsPreemptionGranularity,
            )
            .field(
                "ComputePreemptionGranularity",
                &self.ComputePreemptionGranularity,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
            && self.Flags == other.Flags
            && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity
            && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DXGI_ADAPTER_DESC3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_ADAPTER_FLAG(pub u32);
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(0u32);
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(1u32);
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(2u32);
impl ::std::convert::From<u32> for DXGI_ADAPTER_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_ADAPTER_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_ADAPTER_FLAG3(pub u32);
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(0u32);
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(1u32);
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(2u32);
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4u32);
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 =
    DXGI_ADAPTER_FLAG3(8u32);
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 =
    DXGI_ADAPTER_FLAG3(16u32);
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 =
    DXGI_ADAPTER_FLAG3(32u32);
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4294967295u32);
impl ::std::convert::From<u32> for DXGI_ADAPTER_FLAG3 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_ADAPTER_FLAG3 {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_ALPHA_MODE(pub u32);
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(0u32);
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(1u32);
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(2u32);
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(3u32);
pub const DXGI_ALPHA_MODE_FORCE_DWORD: DXGI_ALPHA_MODE = DXGI_ALPHA_MODE(4294967295u32);
impl ::std::convert::From<u32> for DXGI_ALPHA_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_ALPHA_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_ALPHA_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_ALPHA_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_ALPHA_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_ALPHA_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_ALPHA_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_COLOR_SPACE_TYPE(pub i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(0i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(1i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(2i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(3i32);
pub const DXGI_COLOR_SPACE_RESERVED: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(4i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(5i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(6i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(7i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(8i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(9i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(10i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(11i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(12i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(13i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(14i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(15i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(16i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(17i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(18i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(19i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(20i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(21i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(22i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(23i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(24i32);
pub const DXGI_COLOR_SPACE_CUSTOM: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(-1i32);
impl ::std::convert::From<i32> for DXGI_COLOR_SPACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_COLOR_SPACE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(pub i32);
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY =
    DXGI_COMPUTE_PREEMPTION_GRANULARITY(0i32);
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY =
    DXGI_COMPUTE_PREEMPTION_GRANULARITY(1i32);
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY =
    DXGI_COMPUTE_PREEMPTION_GRANULARITY(2i32);
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY =
    DXGI_COMPUTE_PREEMPTION_GRANULARITY(3i32);
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY =
    DXGI_COMPUTE_PREEMPTION_GRANULARITY(4i32);
impl ::std::convert::From<i32> for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1u32;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15u32;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0u32;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2u32;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3u32;
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_ALL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3834307203,
    55936,
    18699,
    [135, 230, 67, 233, 169, 207, 218, 8],
);
pub const DXGI_DEBUG_APP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    114126337,
    16921,
    20157,
    [135, 9, 39, 237, 35, 54, 12, 98],
);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    902682620,
    5042,
    16925,
    [165, 215, 126, 68, 81, 40, 125, 100],
);
pub const DXGI_DEBUG_DXGI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    634247844,
    45510,
    18401,
    [172, 62, 152, 135, 91, 90, 46, 42],
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_DEBUG_RLO_FLAGS(pub u32);
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(1u32);
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(2u32);
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(4u32);
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(7u32);
impl ::std::convert::From<u32> for DXGI_DEBUG_RLO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_DEBUG_RLO_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::std::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC")
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
unsafe impl ::windows::runtime::Abi for DXGI_DECODE_SWAP_CHAIN_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl DXGI_DISPLAY_COLOR_SPACE {}
impl ::std::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_DISPLAY_COLOR_SPACE")
            .field("PrimaryCoordinates", &self.PrimaryCoordinates)
            .field("WhitePoints", &self.WhitePoints)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryCoordinates == other.PrimaryCoordinates && self.WhitePoints == other.WhitePoints
    }
}
impl ::std::cmp::Eq for DXGI_DISPLAY_COLOR_SPACE {}
unsafe impl ::windows::runtime::Abi for DXGI_DISPLAY_COLOR_SPACE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
pub const DXGI_ERROR_ACCESS_DENIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270485i32 as _);
pub const DXGI_ERROR_ACCESS_LOST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270490i32 as _);
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270474i32 as _);
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270477i32 as _);
pub const DXGI_ERROR_CACHE_FULL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270476i32 as _);
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270475i32 as _);
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270486i32 as _);
pub const DXGI_ERROR_DEVICE_HUNG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270522i32 as _);
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270523i32 as _);
pub const DXGI_ERROR_DEVICE_RESET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270521i32 as _);
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270496i32 as _);
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270479i32 as _);
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270517i32 as _);
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270516i32 as _);
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270480i32 as _);
pub const DXGI_ERROR_INVALID_CALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270527i32 as _);
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270491i32 as _);
pub const DXGI_ERROR_MORE_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270525i32 as _);
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270484i32 as _);
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270495i32 as _);
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270478i32 as _);
pub const DXGI_ERROR_NOT_CURRENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270482i32 as _);
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270494i32 as _);
pub const DXGI_ERROR_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270526i32 as _);
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270493i32 as _);
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270492i32 as _);
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270487i32 as _);
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270483i32 as _);
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270488i32 as _);
pub const DXGI_ERROR_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270524i32 as _);
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270489i32 as _);
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2005270518i32 as _);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_FEATURE(pub i32);
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = DXGI_FEATURE(0i32);
impl ::std::convert::From<i32> for DXGI_FEATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_FEATURE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_FORMAT(pub u32);
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = DXGI_FORMAT(0u32);
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(1u32);
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(2u32);
pub const DXGI_FORMAT_R32G32B32A32_UINT: DXGI_FORMAT = DXGI_FORMAT(3u32);
pub const DXGI_FORMAT_R32G32B32A32_SINT: DXGI_FORMAT = DXGI_FORMAT(4u32);
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(5u32);
pub const DXGI_FORMAT_R32G32B32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(6u32);
pub const DXGI_FORMAT_R32G32B32_UINT: DXGI_FORMAT = DXGI_FORMAT(7u32);
pub const DXGI_FORMAT_R32G32B32_SINT: DXGI_FORMAT = DXGI_FORMAT(8u32);
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(9u32);
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(10u32);
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DXGI_FORMAT = DXGI_FORMAT(11u32);
pub const DXGI_FORMAT_R16G16B16A16_UINT: DXGI_FORMAT = DXGI_FORMAT(12u32);
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DXGI_FORMAT = DXGI_FORMAT(13u32);
pub const DXGI_FORMAT_R16G16B16A16_SINT: DXGI_FORMAT = DXGI_FORMAT(14u32);
pub const DXGI_FORMAT_R32G32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(15u32);
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(16u32);
pub const DXGI_FORMAT_R32G32_UINT: DXGI_FORMAT = DXGI_FORMAT(17u32);
pub const DXGI_FORMAT_R32G32_SINT: DXGI_FORMAT = DXGI_FORMAT(18u32);
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(19u32);
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DXGI_FORMAT = DXGI_FORMAT(20u32);
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(21u32);
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DXGI_FORMAT = DXGI_FORMAT(22u32);
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(23u32);
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DXGI_FORMAT = DXGI_FORMAT(24u32);
pub const DXGI_FORMAT_R10G10B10A2_UINT: DXGI_FORMAT = DXGI_FORMAT(25u32);
pub const DXGI_FORMAT_R11G11B10_FLOAT: DXGI_FORMAT = DXGI_FORMAT(26u32);
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(27u32);
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(28u32);
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(29u32);
pub const DXGI_FORMAT_R8G8B8A8_UINT: DXGI_FORMAT = DXGI_FORMAT(30u32);
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DXGI_FORMAT = DXGI_FORMAT(31u32);
pub const DXGI_FORMAT_R8G8B8A8_SINT: DXGI_FORMAT = DXGI_FORMAT(32u32);
pub const DXGI_FORMAT_R16G16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(33u32);
pub const DXGI_FORMAT_R16G16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(34u32);
pub const DXGI_FORMAT_R16G16_UNORM: DXGI_FORMAT = DXGI_FORMAT(35u32);
pub const DXGI_FORMAT_R16G16_UINT: DXGI_FORMAT = DXGI_FORMAT(36u32);
pub const DXGI_FORMAT_R16G16_SNORM: DXGI_FORMAT = DXGI_FORMAT(37u32);
pub const DXGI_FORMAT_R16G16_SINT: DXGI_FORMAT = DXGI_FORMAT(38u32);
pub const DXGI_FORMAT_R32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(39u32);
pub const DXGI_FORMAT_D32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(40u32);
pub const DXGI_FORMAT_R32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(41u32);
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = DXGI_FORMAT(42u32);
pub const DXGI_FORMAT_R32_SINT: DXGI_FORMAT = DXGI_FORMAT(43u32);
pub const DXGI_FORMAT_R24G8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(44u32);
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DXGI_FORMAT = DXGI_FORMAT(45u32);
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(46u32);
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DXGI_FORMAT = DXGI_FORMAT(47u32);
pub const DXGI_FORMAT_R8G8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(48u32);
pub const DXGI_FORMAT_R8G8_UNORM: DXGI_FORMAT = DXGI_FORMAT(49u32);
pub const DXGI_FORMAT_R8G8_UINT: DXGI_FORMAT = DXGI_FORMAT(50u32);
pub const DXGI_FORMAT_R8G8_SNORM: DXGI_FORMAT = DXGI_FORMAT(51u32);
pub const DXGI_FORMAT_R8G8_SINT: DXGI_FORMAT = DXGI_FORMAT(52u32);
pub const DXGI_FORMAT_R16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(53u32);
pub const DXGI_FORMAT_R16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(54u32);
pub const DXGI_FORMAT_D16_UNORM: DXGI_FORMAT = DXGI_FORMAT(55u32);
pub const DXGI_FORMAT_R16_UNORM: DXGI_FORMAT = DXGI_FORMAT(56u32);
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = DXGI_FORMAT(57u32);
pub const DXGI_FORMAT_R16_SNORM: DXGI_FORMAT = DXGI_FORMAT(58u32);
pub const DXGI_FORMAT_R16_SINT: DXGI_FORMAT = DXGI_FORMAT(59u32);
pub const DXGI_FORMAT_R8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(60u32);
pub const DXGI_FORMAT_R8_UNORM: DXGI_FORMAT = DXGI_FORMAT(61u32);
pub const DXGI_FORMAT_R8_UINT: DXGI_FORMAT = DXGI_FORMAT(62u32);
pub const DXGI_FORMAT_R8_SNORM: DXGI_FORMAT = DXGI_FORMAT(63u32);
pub const DXGI_FORMAT_R8_SINT: DXGI_FORMAT = DXGI_FORMAT(64u32);
pub const DXGI_FORMAT_A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(65u32);
pub const DXGI_FORMAT_R1_UNORM: DXGI_FORMAT = DXGI_FORMAT(66u32);
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DXGI_FORMAT = DXGI_FORMAT(67u32);
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DXGI_FORMAT = DXGI_FORMAT(68u32);
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DXGI_FORMAT = DXGI_FORMAT(69u32);
pub const DXGI_FORMAT_BC1_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(70u32);
pub const DXGI_FORMAT_BC1_UNORM: DXGI_FORMAT = DXGI_FORMAT(71u32);
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(72u32);
pub const DXGI_FORMAT_BC2_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(73u32);
pub const DXGI_FORMAT_BC2_UNORM: DXGI_FORMAT = DXGI_FORMAT(74u32);
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(75u32);
pub const DXGI_FORMAT_BC3_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(76u32);
pub const DXGI_FORMAT_BC3_UNORM: DXGI_FORMAT = DXGI_FORMAT(77u32);
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(78u32);
pub const DXGI_FORMAT_BC4_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(79u32);
pub const DXGI_FORMAT_BC4_UNORM: DXGI_FORMAT = DXGI_FORMAT(80u32);
pub const DXGI_FORMAT_BC4_SNORM: DXGI_FORMAT = DXGI_FORMAT(81u32);
pub const DXGI_FORMAT_BC5_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(82u32);
pub const DXGI_FORMAT_BC5_UNORM: DXGI_FORMAT = DXGI_FORMAT(83u32);
pub const DXGI_FORMAT_BC5_SNORM: DXGI_FORMAT = DXGI_FORMAT(84u32);
pub const DXGI_FORMAT_B5G6R5_UNORM: DXGI_FORMAT = DXGI_FORMAT(85u32);
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DXGI_FORMAT = DXGI_FORMAT(86u32);
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(87u32);
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DXGI_FORMAT = DXGI_FORMAT(88u32);
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DXGI_FORMAT = DXGI_FORMAT(89u32);
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(90u32);
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(91u32);
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(92u32);
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(93u32);
pub const DXGI_FORMAT_BC6H_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(94u32);
pub const DXGI_FORMAT_BC6H_UF16: DXGI_FORMAT = DXGI_FORMAT(95u32);
pub const DXGI_FORMAT_BC6H_SF16: DXGI_FORMAT = DXGI_FORMAT(96u32);
pub const DXGI_FORMAT_BC7_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(97u32);
pub const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT = DXGI_FORMAT(98u32);
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(99u32);
pub const DXGI_FORMAT_AYUV: DXGI_FORMAT = DXGI_FORMAT(100u32);
pub const DXGI_FORMAT_Y410: DXGI_FORMAT = DXGI_FORMAT(101u32);
pub const DXGI_FORMAT_Y416: DXGI_FORMAT = DXGI_FORMAT(102u32);
pub const DXGI_FORMAT_NV12: DXGI_FORMAT = DXGI_FORMAT(103u32);
pub const DXGI_FORMAT_P010: DXGI_FORMAT = DXGI_FORMAT(104u32);
pub const DXGI_FORMAT_P016: DXGI_FORMAT = DXGI_FORMAT(105u32);
pub const DXGI_FORMAT_420_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(106u32);
pub const DXGI_FORMAT_YUY2: DXGI_FORMAT = DXGI_FORMAT(107u32);
pub const DXGI_FORMAT_Y210: DXGI_FORMAT = DXGI_FORMAT(108u32);
pub const DXGI_FORMAT_Y216: DXGI_FORMAT = DXGI_FORMAT(109u32);
pub const DXGI_FORMAT_NV11: DXGI_FORMAT = DXGI_FORMAT(110u32);
pub const DXGI_FORMAT_AI44: DXGI_FORMAT = DXGI_FORMAT(111u32);
pub const DXGI_FORMAT_IA44: DXGI_FORMAT = DXGI_FORMAT(112u32);
pub const DXGI_FORMAT_P8: DXGI_FORMAT = DXGI_FORMAT(113u32);
pub const DXGI_FORMAT_A8P8: DXGI_FORMAT = DXGI_FORMAT(114u32);
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DXGI_FORMAT = DXGI_FORMAT(115u32);
pub const DXGI_FORMAT_P208: DXGI_FORMAT = DXGI_FORMAT(130u32);
pub const DXGI_FORMAT_V208: DXGI_FORMAT = DXGI_FORMAT(131u32);
pub const DXGI_FORMAT_V408: DXGI_FORMAT = DXGI_FORMAT(132u32);
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(189u32);
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(190u32);
pub const DXGI_FORMAT_FORCE_UINT: DXGI_FORMAT = DXGI_FORMAT(4294967295u32);
impl ::std::convert::From<u32> for DXGI_FORMAT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_FORMAT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_FORMAT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_FORMAT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_FORMAT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_FORMAT_DEFINED: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_FRAME_PRESENTATION_MODE(pub i32);
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE =
    DXGI_FRAME_PRESENTATION_MODE(0i32);
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE =
    DXGI_FRAME_PRESENTATION_MODE(1i32);
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE =
    DXGI_FRAME_PRESENTATION_MODE(2i32);
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE =
    DXGI_FRAME_PRESENTATION_MODE(3i32);
impl ::std::convert::From<i32> for DXGI_FRAME_PRESENTATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_FRAME_PRESENTATION_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl DXGI_FRAME_STATISTICS {}
impl ::std::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_FRAME_STATISTICS")
            .field("PresentCount", &self.PresentCount)
            .field("PresentRefreshCount", &self.PresentRefreshCount)
            .field("SyncRefreshCount", &self.SyncRefreshCount)
            .field("SyncQPCTime", &self.SyncQPCTime)
            .field("SyncGPUTime", &self.SyncGPUTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount
            && self.PresentRefreshCount == other.PresentRefreshCount
            && self.SyncRefreshCount == other.SyncRefreshCount
            && self.SyncQPCTime == other.SyncQPCTime
            && self.SyncGPUTime == other.SyncGPUTime
    }
}
impl ::std::cmp::Eq for DXGI_FRAME_STATISTICS {}
unsafe impl ::windows::runtime::Abi for DXGI_FRAME_STATISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl DXGI_FRAME_STATISTICS_MEDIA {}
impl ::std::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_FRAME_STATISTICS_MEDIA")
            .field("PresentCount", &self.PresentCount)
            .field("PresentRefreshCount", &self.PresentRefreshCount)
            .field("SyncRefreshCount", &self.SyncRefreshCount)
            .field("SyncQPCTime", &self.SyncQPCTime)
            .field("SyncGPUTime", &self.SyncGPUTime)
            .field("CompositionMode", &self.CompositionMode)
            .field("ApprovedPresentDuration", &self.ApprovedPresentDuration)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount
            && self.PresentRefreshCount == other.PresentRefreshCount
            && self.SyncRefreshCount == other.SyncRefreshCount
            && self.SyncQPCTime == other.SyncQPCTime
            && self.SyncGPUTime == other.SyncGPUTime
            && self.CompositionMode == other.CompositionMode
            && self.ApprovedPresentDuration == other.ApprovedPresentDuration
    }
}
impl ::std::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
unsafe impl ::windows::runtime::Abi for DXGI_FRAME_STATISTICS_MEDIA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl DXGI_GAMMA_CONTROL {}
impl ::std::default::Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_GAMMA_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL")
            .field("Scale", &self.Scale)
            .field("Offset", &self.Offset)
            .field("GammaCurve", &self.GammaCurve)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_GAMMA_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale
            && self.Offset == other.Offset
            && self.GammaCurve == other.GammaCurve
    }
}
impl ::std::cmp::Eq for DXGI_GAMMA_CONTROL {}
unsafe impl ::windows::runtime::Abi for DXGI_GAMMA_CONTROL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: super::super::Foundation::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_GAMMA_CONTROL_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL_CAPABILITIES")
            .field("ScaleAndOffsetSupported", &self.ScaleAndOffsetSupported)
            .field("MaxConvertedValue", &self.MaxConvertedValue)
            .field("MinConvertedValue", &self.MinConvertedValue)
            .field("NumGammaControlPoints", &self.NumGammaControlPoints)
            .field("ControlPointPositions", &self.ControlPointPositions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.ScaleAndOffsetSupported == other.ScaleAndOffsetSupported
            && self.MaxConvertedValue == other.MaxConvertedValue
            && self.MinConvertedValue == other.MinConvertedValue
            && self.NumGammaControlPoints == other.NumGammaControlPoints
            && self.ControlPointPositions == other.ControlPointPositions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_GAMMA_CONTROL_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_GAMMA_CONTROL_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_GPU_PREFERENCE(pub i32);
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(0i32);
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(1i32);
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(2i32);
impl ::std::convert::From<i32> for DXGI_GPU_PREFERENCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_GPU_PREFERENCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(pub i32);
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY =
    DXGI_GRAPHICS_PREEMPTION_GRANULARITY(0i32);
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY =
    DXGI_GRAPHICS_PREEMPTION_GRANULARITY(1i32);
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY =
    DXGI_GRAPHICS_PREEMPTION_GRANULARITY(2i32);
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY =
    DXGI_GRAPHICS_PREEMPTION_GRANULARITY(3i32);
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY =
    DXGI_GRAPHICS_PREEMPTION_GRANULARITY(4i32);
impl ::std::convert::From<i32> for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(pub u32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN:
    DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(1u32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS =
    DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(2u32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED:
    DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(4u32);
impl ::std::convert::From<u32> for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl DXGI_HDR_METADATA_HDR10 {}
impl ::std::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_HDR_METADATA_HDR10")
            .field("RedPrimary", &self.RedPrimary)
            .field("GreenPrimary", &self.GreenPrimary)
            .field("BluePrimary", &self.BluePrimary)
            .field("WhitePoint", &self.WhitePoint)
            .field("MaxMasteringLuminance", &self.MaxMasteringLuminance)
            .field("MinMasteringLuminance", &self.MinMasteringLuminance)
            .field("MaxContentLightLevel", &self.MaxContentLightLevel)
            .field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimary == other.RedPrimary
            && self.GreenPrimary == other.GreenPrimary
            && self.BluePrimary == other.BluePrimary
            && self.WhitePoint == other.WhitePoint
            && self.MaxMasteringLuminance == other.MaxMasteringLuminance
            && self.MinMasteringLuminance == other.MinMasteringLuminance
            && self.MaxContentLightLevel == other.MaxContentLightLevel
            && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::std::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
unsafe impl ::windows::runtime::Abi for DXGI_HDR_METADATA_HDR10 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl DXGI_HDR_METADATA_HDR10PLUS {}
impl ::std::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_HDR_METADATA_HDR10PLUS")
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::std::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
unsafe impl ::windows::runtime::Abi for DXGI_HDR_METADATA_HDR10PLUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_HDR_METADATA_TYPE(pub i32);
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(0i32);
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(1i32);
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(2i32);
impl ::std::convert::From<i32> for DXGI_HDR_METADATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_HDR_METADATA_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl DXGI_INFO_QUEUE_FILTER {}
impl ::std::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_FILTER")
            .field("AllowList", &self.AllowList)
            .field("DenyList", &self.DenyList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
unsafe impl ::windows::runtime::Abi for DXGI_INFO_QUEUE_FILTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::std::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC")
            .field("NumCategories", &self.NumCategories)
            .field("pCategoryList", &self.pCategoryList)
            .field("NumSeverities", &self.NumSeverities)
            .field("pSeverityList", &self.pSeverityList)
            .field("NumIDs", &self.NumIDs)
            .field("pIDList", &self.pIDList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories
            && self.pCategoryList == other.pCategoryList
            && self.NumSeverities == other.NumSeverities
            && self.pSeverityList == other.pSeverityList
            && self.NumIDs == other.NumIDs
            && self.pIDList == other.pIDList
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
unsafe impl ::windows::runtime::Abi for DXGI_INFO_QUEUE_FILTER_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows::runtime::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl DXGI_INFO_QUEUE_MESSAGE {}
impl ::std::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_MESSAGE")
            .field("Producer", &self.Producer)
            .field("Category", &self.Category)
            .field("Severity", &self.Severity)
            .field("ID", &self.ID)
            .field("pDescription", &self.pDescription)
            .field("DescriptionByteLength", &self.DescriptionByteLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Producer == other.Producer
            && self.Category == other.Category
            && self.Severity == other.Severity
            && self.ID == other.ID
            && self.pDescription == other.pDescription
            && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
unsafe impl ::windows::runtime::Abi for DXGI_INFO_QUEUE_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(pub i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(0i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(1i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(2i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(3i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(4i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(5i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(6i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(7i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(8i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(9i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY =
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY(10i32);
impl ::std::convert::From<i32> for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(pub i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY =
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY(0i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY =
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY(1i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY =
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY(2i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY =
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY(3i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY =
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY(4i32);
impl ::std::convert::From<i32> for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 16],
    pub CodeValues: [u8; 162],
}
impl DXGI_JPEG_AC_HUFFMAN_TABLE {}
impl ::std::default::Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_AC_HUFFMAN_TABLE")
            .field("CodeCounts", &self.CodeCounts)
            .field("CodeValues", &self.CodeValues)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_AC_HUFFMAN_TABLE {}
unsafe impl ::windows::runtime::Abi for DXGI_JPEG_AC_HUFFMAN_TABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 12],
    pub CodeValues: [u8; 12],
}
impl DXGI_JPEG_DC_HUFFMAN_TABLE {}
impl ::std::default::Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_DC_HUFFMAN_TABLE")
            .field("CodeCounts", &self.CodeCounts)
            .field("CodeValues", &self.CodeValues)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_DC_HUFFMAN_TABLE {}
unsafe impl ::windows::runtime::Abi for DXGI_JPEG_DC_HUFFMAN_TABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl DXGI_JPEG_QUANTIZATION_TABLE {}
impl ::std::default::Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_QUANTIZATION_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_QUANTIZATION_TABLE")
            .field("Elements", &self.Elements)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_QUANTIZATION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Elements == other.Elements
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_QUANTIZATION_TABLE {}
unsafe impl ::windows::runtime::Abi for DXGI_JPEG_QUANTIZATION_TABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl DXGI_MAPPED_RECT {}
impl ::std::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MAPPED_RECT")
            .field("Pitch", &self.Pitch)
            .field("pBits", &self.pBits)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Pitch == other.Pitch && self.pBits == other.pBits
    }
}
impl ::std::cmp::Eq for DXGI_MAPPED_RECT {}
unsafe impl ::windows::runtime::Abi for DXGI_MAPPED_RECT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_MAP_DISCARD: u32 = 4u32;
pub const DXGI_MAP_READ: u32 = 1u32;
pub const DXGI_MAP_WRITE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl DXGI_MATRIX_3X2_F {}
impl ::std::default::Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_MATRIX_3X2_F {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MATRIX_3X2_F")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11
            && self._12 == other._12
            && self._21 == other._21
            && self._22 == other._22
            && self._31 == other._31
            && self._32 == other._32
    }
}
impl ::std::cmp::Eq for DXGI_MATRIX_3X2_F {}
unsafe impl ::windows::runtime::Abi for DXGI_MATRIX_3X2_F {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MEMORY_SEGMENT_GROUP(pub i32);
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP =
    DXGI_MEMORY_SEGMENT_GROUP(0i32);
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP =
    DXGI_MEMORY_SEGMENT_GROUP(1i32);
impl ::std::convert::From<i32> for DXGI_MEMORY_SEGMENT_GROUP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_MEMORY_SEGMENT_GROUP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
impl DXGI_MODE_DESC {}
impl ::std::default::Default for DXGI_MODE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_MODE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC")
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .field("RefreshRate", &self.RefreshRate)
            .field("Format", &self.Format)
            .field("ScanlineOrdering", &self.ScanlineOrdering)
            .field("Scaling", &self.Scaling)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.RefreshRate == other.RefreshRate
            && self.Format == other.Format
            && self.ScanlineOrdering == other.ScanlineOrdering
            && self.Scaling == other.Scaling
    }
}
impl ::std::cmp::Eq for DXGI_MODE_DESC {}
unsafe impl ::windows::runtime::Abi for DXGI_MODE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Stereo: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_MODE_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC1")
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .field("RefreshRate", &self.RefreshRate)
            .field("Format", &self.Format)
            .field("ScanlineOrdering", &self.ScanlineOrdering)
            .field("Scaling", &self.Scaling)
            .field("Stereo", &self.Stereo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.RefreshRate == other.RefreshRate
            && self.Format == other.Format
            && self.ScanlineOrdering == other.ScanlineOrdering
            && self.Scaling == other.Scaling
            && self.Stereo == other.Stereo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_MODE_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_MODE_DESC1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MODE_ROTATION(pub i32);
pub const DXGI_MODE_ROTATION_UNSPECIFIED: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(0i32);
pub const DXGI_MODE_ROTATION_IDENTITY: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(1i32);
pub const DXGI_MODE_ROTATION_ROTATE90: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(2i32);
pub const DXGI_MODE_ROTATION_ROTATE180: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(3i32);
pub const DXGI_MODE_ROTATION_ROTATE270: DXGI_MODE_ROTATION = DXGI_MODE_ROTATION(4i32);
impl ::std::convert::From<i32> for DXGI_MODE_ROTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_MODE_ROTATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MODE_SCALING(pub i32);
pub const DXGI_MODE_SCALING_UNSPECIFIED: DXGI_MODE_SCALING = DXGI_MODE_SCALING(0i32);
pub const DXGI_MODE_SCALING_CENTERED: DXGI_MODE_SCALING = DXGI_MODE_SCALING(1i32);
pub const DXGI_MODE_SCALING_STRETCHED: DXGI_MODE_SCALING = DXGI_MODE_SCALING(2i32);
impl ::std::convert::From<i32> for DXGI_MODE_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_MODE_SCALING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MODE_SCANLINE_ORDER(pub i32);
pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: DXGI_MODE_SCANLINE_ORDER =
    DXGI_MODE_SCANLINE_ORDER(0i32);
pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: DXGI_MODE_SCANLINE_ORDER =
    DXGI_MODE_SCANLINE_ORDER(1i32);
pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER =
    DXGI_MODE_SCANLINE_ORDER(2i32);
pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER =
    DXGI_MODE_SCANLINE_ORDER(3i32);
impl ::std::convert::From<i32> for DXGI_MODE_SCANLINE_ORDER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_MODE_SCANLINE_ORDER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(pub i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS =
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(1i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS =
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(2i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS =
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(4i32);
impl ::std::convert::From<i32> for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
pub const DXGI_MWA_VALID: u32 = 7u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_Message_Id(pub i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: DXGI_Message_Id =
    DXGI_Message_Id(0i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: DXGI_Message_Id =
    DXGI_Message_Id(1i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: DXGI_Message_Id =
    DXGI_Message_Id(2i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: DXGI_Message_Id =
    DXGI_Message_Id(3i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: DXGI_Message_Id =
    DXGI_Message_Id(4i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: DXGI_Message_Id = DXGI_Message_Id(5i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: DXGI_Message_Id = DXGI_Message_Id(6i32);
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: DXGI_Message_Id =
    DXGI_Message_Id(7i32);
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(8i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(9i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: DXGI_Message_Id =
    DXGI_Message_Id(10i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: DXGI_Message_Id =
    DXGI_Message_Id(11i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: DXGI_Message_Id = DXGI_Message_Id(12i32);
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(13i32);
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: DXGI_Message_Id = DXGI_Message_Id(14i32);
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: DXGI_Message_Id =
    DXGI_Message_Id(15i32);
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: DXGI_Message_Id =
    DXGI_Message_Id(16i32);
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: DXGI_Message_Id =
    DXGI_Message_Id(17i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: DXGI_Message_Id =
    DXGI_Message_Id(18i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: DXGI_Message_Id =
    DXGI_Message_Id(19i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: DXGI_Message_Id =
    DXGI_Message_Id(20i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: DXGI_Message_Id =
    DXGI_Message_Id(21i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: DXGI_Message_Id =
    DXGI_Message_Id(22i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: DXGI_Message_Id =
    DXGI_Message_Id(23i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(24i32);
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(25i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(26i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: DXGI_Message_Id =
    DXGI_Message_Id(27i32);
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(28i32);
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(29i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(30i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(31i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: DXGI_Message_Id =
    DXGI_Message_Id(32i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: DXGI_Message_Id =
    DXGI_Message_Id(33i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: DXGI_Message_Id =
    DXGI_Message_Id(34i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: DXGI_Message_Id =
    DXGI_Message_Id(35i32);
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(36i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: DXGI_Message_Id =
    DXGI_Message_Id(37i32);
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: DXGI_Message_Id = DXGI_Message_Id(38i32);
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: DXGI_Message_Id = DXGI_Message_Id(39i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: DXGI_Message_Id = DXGI_Message_Id(40i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: DXGI_Message_Id =
    DXGI_Message_Id(41i32);
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: DXGI_Message_Id = DXGI_Message_Id(42i32);
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: DXGI_Message_Id =
    DXGI_Message_Id(43i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: DXGI_Message_Id =
    DXGI_Message_Id(44i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(45i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight:
    DXGI_Message_Id = DXGI_Message_Id(46i32);
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: DXGI_Message_Id =
    DXGI_Message_Id(47i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(48i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: DXGI_Message_Id =
    DXGI_Message_Id(49i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(50i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: DXGI_Message_Id =
    DXGI_Message_Id(51i32);
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: DXGI_Message_Id =
    DXGI_Message_Id(52i32);
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(53i32);
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: DXGI_Message_Id =
    DXGI_Message_Id(54i32);
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: DXGI_Message_Id =
    DXGI_Message_Id(55i32);
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: DXGI_Message_Id =
    DXGI_Message_Id(56i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary:
    DXGI_Message_Id = DXGI_Message_Id(57i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: DXGI_Message_Id =
    DXGI_Message_Id(58i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: DXGI_Message_Id =
    DXGI_Message_Id(59i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(60i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(61i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(62i32);
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: DXGI_Message_Id =
    DXGI_Message_Id(63i32);
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(64i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: DXGI_Message_Id =
    DXGI_Message_Id(65i32);
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: DXGI_Message_Id =
    DXGI_Message_Id(66i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: DXGI_Message_Id =
    DXGI_Message_Id(67i32);
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(68i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice:
    DXGI_Message_Id = DXGI_Message_Id(69i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL:
    DXGI_Message_Id = DXGI_Message_Id(70i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero:
    DXGI_Message_Id = DXGI_Message_Id(71i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration:
    DXGI_Message_Id = DXGI_Message_Id(72i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering:
    DXGI_Message_Id = DXGI_Message_Id(73i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: DXGI_Message_Id =
    DXGI_Message_Id(74i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination : DXGI_Message_Id = DXGI_Message_Id ( 75i32 ) ;
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: DXGI_Message_Id =
    DXGI_Message_Id(76i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice:
    DXGI_Message_Id = DXGI_Message_Id(77i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: DXGI_Message_Id =
    DXGI_Message_Id(78i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: DXGI_Message_Id =
    DXGI_Message_Id(79i32);
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput:
    DXGI_Message_Id = DXGI_Message_Id(80i32);
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(81i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows:
    DXGI_Message_Id = DXGI_Message_Id(82i32);
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(83i32);
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(84i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags:
    DXGI_Message_Id = DXGI_Message_Id(85i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags:
    DXGI_Message_Id = DXGI_Message_Id(86i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: DXGI_Message_Id = DXGI_Message_Id(87i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: DXGI_Message_Id =
    DXGI_Message_Id(88i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: DXGI_Message_Id =
    DXGI_Message_Id(89i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(90i32);
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: DXGI_Message_Id = DXGI_Message_Id(91i32);
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: DXGI_Message_Id = DXGI_Message_Id(92i32);
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: DXGI_Message_Id =
    DXGI_Message_Id(93i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(94i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(95i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(96i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: DXGI_Message_Id =
    DXGI_Message_Id(97i32);
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: DXGI_Message_Id =
    DXGI_Message_Id(98i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: DXGI_Message_Id =
    DXGI_Message_Id(99i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: DXGI_Message_Id =
    DXGI_Message_Id(100i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: DXGI_Message_Id =
    DXGI_Message_Id(101i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential:
    DXGI_Message_Id = DXGI_Message_Id(102i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: DXGI_Message_Id =
    DXGI_Message_Id(103i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: DXGI_Message_Id =
    DXGI_Message_Id(104i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation:
    DXGI_Message_Id = DXGI_Message_Id(105i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: DXGI_Message_Id =
    DXGI_Message_Id(106i32);
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: DXGI_Message_Id =
    DXGI_Message_Id(107i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: DXGI_Message_Id =
    DXGI_Message_Id(108i32);
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: DXGI_Message_Id =
    DXGI_Message_Id(109i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: DXGI_Message_Id =
    DXGI_Message_Id(110i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset:
    DXGI_Message_Id = DXGI_Message_Id(111i32);
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: DXGI_Message_Id = DXGI_Message_Id(112i32);
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: DXGI_Message_Id =
    DXGI_Message_Id(113i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: DXGI_Message_Id =
    DXGI_Message_Id(114i32);
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined:
    DXGI_Message_Id = DXGI_Message_Id(115i32);
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: DXGI_Message_Id =
    DXGI_Message_Id(116i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition:
    DXGI_Message_Id = DXGI_Message_Id(117i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory:
    DXGI_Message_Id = DXGI_Message_Id(118i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter:
    DXGI_Message_Id = DXGI_Message_Id(119i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput:
    DXGI_Message_Id = DXGI_Message_Id(120i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: DXGI_Message_Id =
    DXGI_Message_Id(121i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: DXGI_Message_Id =
    DXGI_Message_Id(122i32);
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: DXGI_Message_Id =
    DXGI_Message_Id(123i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: DXGI_Message_Id =
    DXGI_Message_Id(124i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: DXGI_Message_Id =
    DXGI_Message_Id(125i32);
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: DXGI_Message_Id =
    DXGI_Message_Id(126i32);
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: DXGI_Message_Id =
    DXGI_Message_Id(127i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: DXGI_Message_Id =
    DXGI_Message_Id(128i32);
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: DXGI_Message_Id =
    DXGI_Message_Id(129i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay:
    DXGI_Message_Id = DXGI_Message_Id(130i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential:
    DXGI_Message_Id = DXGI_Message_Id(131i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: DXGI_Message_Id =
    DXGI_Message_Id(132i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity : DXGI_Message_Id = DXGI_Message_Id ( 133i32 ) ;
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: DXGI_Message_Id =
    DXGI_Message_Id(134i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported:
    DXGI_Message_Id = DXGI_Message_Id(135i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter:
    DXGI_Message_Id = DXGI_Message_Id(136i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7:
    DXGI_Message_Id = DXGI_Message_Id(137i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain:
    DXGI_Message_Id = DXGI_Message_Id(138i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: DXGI_Message_Id =
    DXGI_Message_Id(139i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: DXGI_Message_Id =
    DXGI_Message_Id(140i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: DXGI_Message_Id =
    DXGI_Message_Id(141i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: DXGI_Message_Id =
    DXGI_Message_Id(142i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(143i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(144i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: DXGI_Message_Id =
    DXGI_Message_Id(145i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly:
    DXGI_Message_Id = DXGI_Message_Id(146i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(147i32);
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: DXGI_Message_Id =
    DXGI_Message_Id(148i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(149i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(150i32);
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(151i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(152i32);
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(153i32);
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: DXGI_Message_Id =
    DXGI_Message_Id(154i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: DXGI_Message_Id =
    DXGI_Message_Id(155i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: DXGI_Message_Id =
    DXGI_Message_Id(156i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: DXGI_Message_Id =
    DXGI_Message_Id(157i32);
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: DXGI_Message_Id =
    DXGI_Message_Id(158i32);
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: DXGI_Message_Id = DXGI_Message_Id(159i32);
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: DXGI_Message_Id =
    DXGI_Message_Id(160i32);
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: DXGI_Message_Id =
    DXGI_Message_Id(161i32);
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: DXGI_Message_Id =
    DXGI_Message_Id(162i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: DXGI_Message_Id =
    DXGI_Message_Id(163i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7:
    DXGI_Message_Id = DXGI_Message_Id(164i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(165i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: DXGI_Message_Id =
    DXGI_Message_Id(166i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: DXGI_Message_Id =
    DXGI_Message_Id(167i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: DXGI_Message_Id = DXGI_Message_Id(168i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(169i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: DXGI_Message_Id =
    DXGI_Message_Id(170i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: DXGI_Message_Id = DXGI_Message_Id(171i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: DXGI_Message_Id =
    DXGI_Message_Id(172i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle:
    DXGI_Message_Id = DXGI_Message_Id(173i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: DXGI_Message_Id = DXGI_Message_Id(174i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer:
    DXGI_Message_Id = DXGI_Message_Id(175i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: DXGI_Message_Id =
    DXGI_Message_Id(176i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence:
    DXGI_Message_Id = DXGI_Message_Id(177i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: DXGI_Message_Id =
    DXGI_Message_Id(178i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(179i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: DXGI_Message_Id =
    DXGI_Message_Id(180i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard:
    DXGI_Message_Id = DXGI_Message_Id(181i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: DXGI_Message_Id =
    DXGI_Message_Id(182i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: DXGI_Message_Id =
    DXGI_Message_Id(183i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: DXGI_Message_Id =
    DXGI_Message_Id(184i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: DXGI_Message_Id =
    DXGI_Message_Id(185i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: DXGI_Message_Id =
    DXGI_Message_Id(186i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: DXGI_Message_Id =
    DXGI_Message_Id(187i32);
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: DXGI_Message_Id =
    DXGI_Message_Id(188i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: DXGI_Message_Id =
    DXGI_Message_Id(189i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: DXGI_Message_Id =
    DXGI_Message_Id(190i32);
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(191i32);
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(192i32);
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(193i32);
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(194i32);
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(195i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: DXGI_Message_Id =
    DXGI_Message_Id(196i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: DXGI_Message_Id =
    DXGI_Message_Id(197i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: DXGI_Message_Id =
    DXGI_Message_Id(198i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: DXGI_Message_Id =
    DXGI_Message_Id(199i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: DXGI_Message_Id =
    DXGI_Message_Id(200i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: DXGI_Message_Id =
    DXGI_Message_Id(201i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: DXGI_Message_Id =
    DXGI_Message_Id(202i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: DXGI_Message_Id =
    DXGI_Message_Id(203i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: DXGI_Message_Id =
    DXGI_Message_Id(204i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: DXGI_Message_Id =
    DXGI_Message_Id(205i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: DXGI_Message_Id =
    DXGI_Message_Id(206i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: DXGI_Message_Id =
    DXGI_Message_Id(207i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents : DXGI_Message_Id = DXGI_Message_Id ( 208i32 ) ;
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents:
    DXGI_Message_Id = DXGI_Message_Id(209i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly:
    DXGI_Message_Id = DXGI_Message_Id(210i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen:
    DXGI_Message_Id = DXGI_Message_Id(211i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: DXGI_Message_Id =
    DXGI_Message_Id(212i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: DXGI_Message_Id =
    DXGI_Message_Id(213i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: DXGI_Message_Id =
    DXGI_Message_Id(214i32);
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id =
    DXGI_Message_Id(215i32);
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: DXGI_Message_Id =
    DXGI_Message_Id(216i32);
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id =
    DXGI_Message_Id(217i32);
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: DXGI_Message_Id =
    DXGI_Message_Id(218i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: DXGI_Message_Id =
    DXGI_Message_Id(219i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter:
    DXGI_Message_Id = DXGI_Message_Id(220i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: DXGI_Message_Id =
    DXGI_Message_Id(221i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: DXGI_Message_Id =
    DXGI_Message_Id(222i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: DXGI_Message_Id =
    DXGI_Message_Id(223i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id =
    DXGI_Message_Id(224i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id =
    DXGI_Message_Id(225i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: DXGI_Message_Id =
    DXGI_Message_Id(226i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale:
    DXGI_Message_Id = DXGI_Message_Id(227i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id =
    DXGI_Message_Id(228i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id =
    DXGI_Message_Id(229i32);
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: DXGI_Message_Id = DXGI_Message_Id(230i32);
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(231i32);
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: DXGI_Message_Id = DXGI_Message_Id(232i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: DXGI_Message_Id = DXGI_Message_Id(233i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: DXGI_Message_Id =
    DXGI_Message_Id(234i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(235i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: DXGI_Message_Id =
    DXGI_Message_Id(236i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: DXGI_Message_Id =
    DXGI_Message_Id(237i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(238i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: DXGI_Message_Id =
    DXGI_Message_Id(239i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: DXGI_Message_Id =
    DXGI_Message_Id(240i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: DXGI_Message_Id =
    DXGI_Message_Id(241i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: DXGI_Message_Id =
    DXGI_Message_Id(242i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: DXGI_Message_Id =
    DXGI_Message_Id(243i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: DXGI_Message_Id =
    DXGI_Message_Id(244i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: DXGI_Message_Id =
    DXGI_Message_Id(245i32);
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: DXGI_Message_Id = DXGI_Message_Id(246i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(247i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(248i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(249i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(250i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: DXGI_Message_Id =
    DXGI_Message_Id(251i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: DXGI_Message_Id =
    DXGI_Message_Id(252i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: DXGI_Message_Id =
    DXGI_Message_Id(253i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: DXGI_Message_Id =
    DXGI_Message_Id(254i32);
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: DXGI_Message_Id =
    DXGI_Message_Id(255i32);
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice:
    DXGI_Message_Id = DXGI_Message_Id(256i32);
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: DXGI_Message_Id =
    DXGI_Message_Id(257i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard:
    DXGI_Message_Id = DXGI_Message_Id(258i32);
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: DXGI_Message_Id =
    DXGI_Message_Id(259i32);
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice : DXGI_Message_Id = DXGI_Message_Id ( 260i32 ) ;
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: DXGI_Message_Id =
    DXGI_Message_Id(261i32);
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: DXGI_Message_Id =
    DXGI_Message_Id(262i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: DXGI_Message_Id =
    DXGI_Message_Id(263i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(264i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: DXGI_Message_Id =
    DXGI_Message_Id(265i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: DXGI_Message_Id =
    DXGI_Message_Id(266i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: DXGI_Message_Id =
    DXGI_Message_Id(267i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: DXGI_Message_Id =
    DXGI_Message_Id(268i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: DXGI_Message_Id =
    DXGI_Message_Id(269i32);
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: DXGI_Message_Id =
    DXGI_Message_Id(270i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: DXGI_Message_Id =
    DXGI_Message_Id(271i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: DXGI_Message_Id =
    DXGI_Message_Id(272i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: DXGI_Message_Id =
    DXGI_Message_Id(273i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: DXGI_Message_Id =
    DXGI_Message_Id(274i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: DXGI_Message_Id = DXGI_Message_Id(275i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: DXGI_Message_Id =
    DXGI_Message_Id(276i32);
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: DXGI_Message_Id =
    DXGI_Message_Id(277i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: DXGI_Message_Id =
    DXGI_Message_Id(278i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: DXGI_Message_Id =
    DXGI_Message_Id(279i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: DXGI_Message_Id =
    DXGI_Message_Id(280i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: DXGI_Message_Id =
    DXGI_Message_Id(281i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: DXGI_Message_Id =
    DXGI_Message_Id(282i32);
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: DXGI_Message_Id =
    DXGI_Message_Id(283i32);
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: DXGI_Message_Id =
    DXGI_Message_Id(284i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: DXGI_Message_Id =
    DXGI_Message_Id(285i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: DXGI_Message_Id =
    DXGI_Message_Id(286i32);
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: DXGI_Message_Id =
    DXGI_Message_Id(287i32);
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: DXGI_Message_Id =
    DXGI_Message_Id(288i32);
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: DXGI_Message_Id =
    DXGI_Message_Id(289i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: DXGI_Message_Id =
    DXGI_Message_Id(290i32);
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: DXGI_Message_Id =
    DXGI_Message_Id(291i32);
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: DXGI_Message_Id =
    DXGI_Message_Id(292i32);
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: DXGI_Message_Id =
    DXGI_Message_Id(293i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: DXGI_Message_Id =
    DXGI_Message_Id(294i32);
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: DXGI_Message_Id =
    DXGI_Message_Id(295i32);
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: DXGI_Message_Id =
    DXGI_Message_Id(296i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr:
    DXGI_Message_Id = DXGI_Message_Id(297i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: DXGI_Message_Id =
    DXGI_Message_Id(298i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: DXGI_Message_Id =
    DXGI_Message_Id(1000i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: DXGI_Message_Id =
    DXGI_Message_Id(1001i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: DXGI_Message_Id =
    DXGI_Message_Id(1002i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: DXGI_Message_Id =
    DXGI_Message_Id(1003i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: DXGI_Message_Id =
    DXGI_Message_Id(1004i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: DXGI_Message_Id =
    DXGI_Message_Id(1005i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: DXGI_Message_Id =
    DXGI_Message_Id(1006i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: DXGI_Message_Id =
    DXGI_Message_Id(1007i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: DXGI_Message_Id =
    DXGI_Message_Id(1008i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: DXGI_Message_Id =
    DXGI_Message_Id(1009i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: DXGI_Message_Id =
    DXGI_Message_Id(1010i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: DXGI_Message_Id =
    DXGI_Message_Id(1011i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: DXGI_Message_Id =
    DXGI_Message_Id(1012i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: DXGI_Message_Id =
    DXGI_Message_Id(1013i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: DXGI_Message_Id =
    DXGI_Message_Id(1014i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: DXGI_Message_Id =
    DXGI_Message_Id(1015i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: DXGI_Message_Id =
    DXGI_Message_Id(1016i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: DXGI_Message_Id =
    DXGI_Message_Id(1017i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: DXGI_Message_Id =
    DXGI_Message_Id(1018i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: DXGI_Message_Id =
    DXGI_Message_Id(1019i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: DXGI_Message_Id =
    DXGI_Message_Id(1020i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: DXGI_Message_Id =
    DXGI_Message_Id(1021i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp : DXGI_Message_Id = DXGI_Message_Id ( 1022i32 ) ;
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: DXGI_Message_Id =
    DXGI_Message_Id(1023i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: DXGI_Message_Id =
    DXGI_Message_Id(1024i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor:
    DXGI_Message_Id = DXGI_Message_Id(1025i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering:
    DXGI_Message_Id = DXGI_Message_Id(1026i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: DXGI_Message_Id =
    DXGI_Message_Id(1027i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: DXGI_Message_Id =
    DXGI_Message_Id(1028i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: DXGI_Message_Id =
    DXGI_Message_Id(1029i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id =
    DXGI_Message_Id(1030i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id =
    DXGI_Message_Id(1031i32);
impl ::std::convert::From<i32> for DXGI_Message_Id {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_Message_Id {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OFFER_RESOURCE_FLAGS(pub i32);
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS =
    DXGI_OFFER_RESOURCE_FLAGS(1i32);
impl ::std::convert::From<i32> for DXGI_OFFER_RESOURCE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_OFFER_RESOURCE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OFFER_RESOURCE_PRIORITY(pub i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY =
    DXGI_OFFER_RESOURCE_PRIORITY(1i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY =
    DXGI_OFFER_RESOURCE_PRIORITY(2i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY =
    DXGI_OFFER_RESOURCE_PRIORITY(3i32);
impl ::std::convert::From<i32> for DXGI_OFFER_RESOURCE_PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_OFFER_RESOURCE_PRIORITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: DXGI_MODE_DESC,
    pub Rotation: DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_OUTDUPL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_DESC")
            .field("ModeDesc", &self.ModeDesc)
            .field("Rotation", &self.Rotation)
            .field(
                "DesktopImageInSystemMemory",
                &self.DesktopImageInSystemMemory,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ModeDesc == other.ModeDesc
            && self.Rotation == other.Rotation
            && self.DesktopImageInSystemMemory == other.DesktopImageInSystemMemory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_OUTDUPL_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OUTDUPL_FLAG(pub i32);
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = DXGI_OUTDUPL_FLAG(1i32);
impl ::std::convert::From<i32> for DXGI_OUTDUPL_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
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
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastPresentTime == other.LastPresentTime
            && self.LastMouseUpdateTime == other.LastMouseUpdateTime
            && self.AccumulatedFrames == other.AccumulatedFrames
            && self.RectsCoalesced == other.RectsCoalesced
            && self.ProtectedContentMaskedOut == other.ProtectedContentMaskedOut
            && self.PointerPosition == other.PointerPosition
            && self.TotalMetadataBufferSize == other.TotalMetadataBufferSize
            && self.PointerShapeBufferSize == other.PointerShapeBufferSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_FRAME_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_MOVE_RECT")
            .field("SourcePoint", &self.SourcePoint)
            .field("DestinationRect", &self.DestinationRect)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.SourcePoint == other.SourcePoint && self.DestinationRect == other.DestinationRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_MOVE_RECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_POINTER_POSITION")
            .field("Position", &self.Position)
            .field("Visible", &self.Visible)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Visible == other.Visible
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_POINTER_POSITION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO")
            .field("Type", &self.Type)
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .field("Pitch", &self.Pitch)
            .field("HotSpot", &self.HotSpot)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Width == other.Width
            && self.Height == other.Height
            && self.Pitch == other.Pitch
            && self.HotSpot == other.HotSpot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(pub i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE =
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE(1i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE =
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE(2i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE =
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE(4i32);
impl ::std::convert::From<i32> for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTPUT_DESC")
            .field("DeviceName", &self.DeviceName)
            .field("DesktopCoordinates", &self.DesktopCoordinates)
            .field("AttachedToDesktop", &self.AttachedToDesktop)
            .field("Rotation", &self.Rotation)
            .field("Monitor", &self.Monitor)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName
            && self.DesktopCoordinates == other.DesktopCoordinates
            && self.AttachedToDesktop == other.AttachedToDesktop
            && self.Rotation == other.Rotation
            && self.Monitor == other.Monitor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DXGI_OUTPUT_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTPUT_DESC1")
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName
            && self.DesktopCoordinates == other.DesktopCoordinates
            && self.AttachedToDesktop == other.AttachedToDesktop
            && self.Rotation == other.Rotation
            && self.Monitor == other.Monitor
            && self.BitsPerColor == other.BitsPerColor
            && self.ColorSpace == other.ColorSpace
            && self.RedPrimary == other.RedPrimary
            && self.GreenPrimary == other.GreenPrimary
            && self.BluePrimary == other.BluePrimary
            && self.WhitePoint == other.WhitePoint
            && self.MinLuminance == other.MinLuminance
            && self.MaxLuminance == other.MaxLuminance
            && self.MaxFullFrameLuminance == other.MaxFullFrameLuminance
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DXGI_OUTPUT_DESC1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(pub i32);
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG =
    DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(1i32);
impl ::std::convert::From<i32> for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OVERLAY_SUPPORT_FLAG(pub i32);
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG =
    DXGI_OVERLAY_SUPPORT_FLAG(1i32);
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG =
    DXGI_OVERLAY_SUPPORT_FLAG(2i32);
impl ::std::convert::From<i32> for DXGI_OVERLAY_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_OVERLAY_SUPPORT_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_PRESENT_PARAMETERS")
            .field("DirtyRectsCount", &self.DirtyRectsCount)
            .field("pDirtyRects", &self.pDirtyRects)
            .field("pScrollRect", &self.pScrollRect)
            .field("pScrollOffset", &self.pScrollOffset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.DirtyRectsCount == other.DirtyRectsCount
            && self.pDirtyRects == other.pDirtyRects
            && self.pScrollRect == other.pScrollRect
            && self.pScrollOffset == other.pScrollOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_PRESENT_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
pub const DXGI_PRESENT_TEST: u32 = 1u32;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::std::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO")
            .field("Budget", &self.Budget)
            .field("CurrentUsage", &self.CurrentUsage)
            .field("AvailableForReservation", &self.AvailableForReservation)
            .field("CurrentReservation", &self.CurrentReservation)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Budget == other.Budget
            && self.CurrentUsage == other.CurrentUsage
            && self.AvailableForReservation == other.AvailableForReservation
            && self.CurrentReservation == other.CurrentReservation
    }
}
impl ::std::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
unsafe impl ::windows::runtime::Abi for DXGI_QUERY_VIDEO_MEMORY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl DXGI_RATIONAL {}
impl ::std::default::Default for DXGI_RATIONAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_RATIONAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RATIONAL")
            .field("Numerator", &self.Numerator)
            .field("Denominator", &self.Denominator)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::std::cmp::Eq for DXGI_RATIONAL {}
unsafe impl ::windows::runtime::Abi for DXGI_RATIONAL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(pub i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS =
    DXGI_RECLAIM_RESOURCE_RESULTS(0i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS =
    DXGI_RECLAIM_RESOURCE_RESULTS(1i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS =
    DXGI_RECLAIM_RESOURCE_RESULTS(2i32);
impl ::std::convert::From<i32> for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_RECLAIM_RESOURCE_RESULTS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_RESIDENCY(pub i32);
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = DXGI_RESIDENCY(1i32);
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = DXGI_RESIDENCY(2i32);
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = DXGI_RESIDENCY(3i32);
impl ::std::convert::From<i32> for DXGI_RESIDENCY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_RESIDENCY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_RESOURCE_PRIORITY(pub u32);
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DXGI_RESOURCE_PRIORITY =
    DXGI_RESOURCE_PRIORITY(671088640u32);
pub const DXGI_RESOURCE_PRIORITY_LOW: DXGI_RESOURCE_PRIORITY =
    DXGI_RESOURCE_PRIORITY(1342177280u32);
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DXGI_RESOURCE_PRIORITY =
    DXGI_RESOURCE_PRIORITY(2013265920u32);
pub const DXGI_RESOURCE_PRIORITY_HIGH: DXGI_RESOURCE_PRIORITY =
    DXGI_RESOURCE_PRIORITY(2684354560u32);
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DXGI_RESOURCE_PRIORITY =
    DXGI_RESOURCE_PRIORITY(3355443200u32);
impl ::std::convert::From<u32> for DXGI_RESOURCE_PRIORITY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_RESOURCE_PRIORITY {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DXGI_RESOURCE_PRIORITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_RESOURCE_PRIORITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXGI_RESOURCE_PRIORITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_RESOURCE_PRIORITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXGI_RESOURCE_PRIORITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
impl DXGI_RGB {}
impl ::std::default::Default for DXGI_RGB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_RGB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RGB")
            .field("Red", &self.Red)
            .field("Green", &self.Green)
            .field("Blue", &self.Blue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::std::cmp::Eq for DXGI_RGB {}
unsafe impl ::windows::runtime::Abi for DXGI_RGB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl DXGI_RGBA {}
impl ::std::default::Default for DXGI_RGBA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_RGBA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RGBA")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("a", &self.a)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::std::cmp::Eq for DXGI_RGBA {}
unsafe impl ::windows::runtime::Abi for DXGI_RGBA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
impl DXGI_SAMPLE_DESC {}
impl ::std::default::Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_SAMPLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SAMPLE_DESC")
            .field("Count", &self.Count)
            .field("Quality", &self.Quality)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::std::cmp::Eq for DXGI_SAMPLE_DESC {}
unsafe impl ::windows::runtime::Abi for DXGI_SAMPLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SCALING(pub i32);
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = DXGI_SCALING(0i32);
pub const DXGI_SCALING_NONE: DXGI_SCALING = DXGI_SCALING(1i32);
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = DXGI_SCALING(2i32);
impl ::std::convert::From<i32> for DXGI_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_SCALING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SHARED_RESOURCE")
            .field("Handle", &self.Handle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_SHARED_RESOURCE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
impl DXGI_SURFACE_DESC {}
impl ::std::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SURFACE_DESC")
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .field("Format", &self.Format)
            .field("SampleDesc", &self.SampleDesc)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.Format == other.Format
            && self.SampleDesc == other.SampleDesc
    }
}
impl ::std::cmp::Eq for DXGI_SURFACE_DESC {}
unsafe impl ::windows::runtime::Abi for DXGI_SURFACE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(pub i32);
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT:
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(1i32);
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT:
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(2i32);
impl ::std::convert::From<i32> for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_SWAP_CHAIN_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_DESC")
            .field("BufferDesc", &self.BufferDesc)
            .field("SampleDesc", &self.SampleDesc)
            .field("BufferUsage", &self.BufferUsage)
            .field("BufferCount", &self.BufferCount)
            .field("OutputWindow", &self.OutputWindow)
            .field("Windowed", &self.Windowed)
            .field("SwapEffect", &self.SwapEffect)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferDesc == other.BufferDesc
            && self.SampleDesc == other.SampleDesc
            && self.BufferUsage == other.BufferUsage
            && self.BufferCount == other.BufferCount
            && self.OutputWindow == other.OutputWindow
            && self.Windowed == other.Windowed
            && self.SwapEffect == other.SwapEffect
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_SWAP_CHAIN_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub Stereo: super::super::Foundation::BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_DESC1")
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .field("Format", &self.Format)
            .field("Stereo", &self.Stereo)
            .field("SampleDesc", &self.SampleDesc)
            .field("BufferUsage", &self.BufferUsage)
            .field("BufferCount", &self.BufferCount)
            .field("Scaling", &self.Scaling)
            .field("SwapEffect", &self.SwapEffect)
            .field("AlphaMode", &self.AlphaMode)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.Format == other.Format
            && self.Stereo == other.Stereo
            && self.SampleDesc == other.SampleDesc
            && self.BufferUsage == other.BufferUsage
            && self.BufferCount == other.BufferCount
            && self.Scaling == other.Scaling
            && self.SwapEffect == other.SwapEffect
            && self.AlphaMode == other.AlphaMode
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_SWAP_CHAIN_DESC1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SWAP_CHAIN_FLAG(pub i32);
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1i32);
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2i32);
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG =
    DXGI_SWAP_CHAIN_FLAG(8i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG =
    DXGI_SWAP_CHAIN_FLAG(16i32);
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(32i32);
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG =
    DXGI_SWAP_CHAIN_FLAG(64i32);
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG =
    DXGI_SWAP_CHAIN_FLAG(128i32);
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG =
    DXGI_SWAP_CHAIN_FLAG(256i32);
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(512i32);
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1024i32);
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2048i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG =
    DXGI_SWAP_CHAIN_FLAG(4096i32);
impl ::std::convert::From<i32> for DXGI_SWAP_CHAIN_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_SWAP_CHAIN_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC")
            .field("RefreshRate", &self.RefreshRate)
            .field("ScanlineOrdering", &self.ScanlineOrdering)
            .field("Scaling", &self.Scaling)
            .field("Windowed", &self.Windowed)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.RefreshRate == other.RefreshRate
            && self.ScanlineOrdering == other.ScanlineOrdering
            && self.Scaling == other.Scaling
            && self.Windowed == other.Windowed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SWAP_EFFECT(pub i32);
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(0i32);
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(1i32);
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(3i32);
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(4i32);
impl ::std::convert::From<i32> for DXGI_SWAP_EFFECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXGI_SWAP_EFFECT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64u32;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512u32;
pub const DXGI_USAGE_READ_ONLY: u32 = 256u32;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32u32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16u32;
pub const DXGI_USAGE_SHARED: u32 = 128u32;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter(::windows::runtime::IUnknown);
impl IDXGIAdapter {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(output),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC> {
        let mut result__: <DXGI_ADAPTER_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interfacename),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIAdapter {
    type Vtable = IDXGIAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        605153249,
        4780,
        19663,
        [189, 20, 151, 152, 232, 83, 77, 192],
    );
}
impl ::std::convert::From<IDXGIAdapter> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIAdapter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIAdapter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter> for IDXGIObject {
    fn from(value: IDXGIAdapter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter> for IDXGIObject {
    fn from(value: &IDXGIAdapter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interfacename: *const ::windows::runtime::GUID,
        pumdversion: *mut i64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter1(::windows::runtime::IUnknown);
impl IDXGIAdapter1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(output),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC> {
        let mut result__: <DXGI_ADAPTER_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interfacename),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: <DXGI_ADAPTER_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIAdapter1 {
    type Vtable = IDXGIAdapter1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        688099169,
        14393,
        17958,
        [145, 253, 8, 104, 121, 1, 26, 5],
    );
}
impl ::std::convert::From<IDXGIAdapter1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIAdapter1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter1> for IDXGIAdapter {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter1> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for &IDXGIAdapter1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter1> for IDXGIObject {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter1> for IDXGIObject {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIAdapter1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interfacename: *const ::windows::runtime::GUID,
        pumdversion: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter2(::windows::runtime::IUnknown);
impl IDXGIAdapter2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(output),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC> {
        let mut result__: <DXGI_ADAPTER_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interfacename),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: <DXGI_ADAPTER_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc2(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC2> {
        let mut result__: <DXGI_ADAPTER_DESC2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIAdapter2 {
    type Vtable = IDXGIAdapter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        178368010,
        64014,
        19332,
        [134, 68, 224, 95, 248, 229, 172, 181],
    );
}
impl ::std::convert::From<IDXGIAdapter2> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter2> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter1> for &IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter2> for IDXGIAdapter {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for &IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter2> for IDXGIObject {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for IDXGIObject {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIAdapter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interfacename: *const ::windows::runtime::GUID,
        pumdversion: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter3(::windows::runtime::IUnknown);
impl IDXGIAdapter3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(output),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC> {
        let mut result__: <DXGI_ADAPTER_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interfacename),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: <DXGI_ADAPTER_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc2(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC2> {
        let mut result__: <DXGI_ADAPTER_DESC2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    pub unsafe fn QueryVideoMemoryInfo(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
    ) -> ::windows::runtime::Result<DXGI_QUERY_VIDEO_MEMORY_INFO> {
        let mut result__: <DXGI_QUERY_VIDEO_MEMORY_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            &mut result__,
        )
        .from_abi::<DXGI_QUERY_VIDEO_MEMORY_INFO>(result__)
    }
    pub unsafe fn SetVideoMemoryReservation(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            ::std::mem::transmute(reservation),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIAdapter3 {
    type Vtable = IDXGIAdapter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1683580836,
        5010,
        17168,
        [167, 152, 128, 83, 206, 62, 147, 253],
    );
}
impl ::std::convert::From<IDXGIAdapter3> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIAdapter2 {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIAdapter2 {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter2> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter2> for &IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter1> for &IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIAdapter {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for &IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIObject {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIObject {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIAdapter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interfacename: *const ::windows::runtime::GUID,
        pumdversion: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter4(::windows::runtime::IUnknown);
impl IDXGIAdapter4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(output),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC> {
        let mut result__: <DXGI_ADAPTER_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC>(result__)
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interfacename),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC1> {
        let mut result__: <DXGI_ADAPTER_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC1>(result__)
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc2(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC2> {
        let mut result__: <DXGI_ADAPTER_DESC2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    pub unsafe fn QueryVideoMemoryInfo(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
    ) -> ::windows::runtime::Result<DXGI_QUERY_VIDEO_MEMORY_INFO> {
        let mut result__: <DXGI_QUERY_VIDEO_MEMORY_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            &mut result__,
        )
        .from_abi::<DXGI_QUERY_VIDEO_MEMORY_INFO>(result__)
    }
    pub unsafe fn SetVideoMemoryReservation(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            ::std::mem::transmute(reservation),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetDesc3(&self) -> ::windows::runtime::Result<DXGI_ADAPTER_DESC3> {
        let mut result__: <DXGI_ADAPTER_DESC3 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_ADAPTER_DESC3>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIAdapter4 {
    type Vtable = IDXGIAdapter4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1015912913,
        20415,
        16769,
        [168, 44, 175, 102, 191, 123, 210, 78],
    );
}
impl ::std::convert::From<IDXGIAdapter4> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter3 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter3 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter3> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter3> for &IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter2 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter2 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter2> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter2> for &IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter1> for &IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIAdapter> for &IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIAdapter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIObject {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIObject {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIAdapter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interfacename: *const ::windows::runtime::GUID,
        pumdversion: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDebug(::windows::runtime::IUnknown);
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        apiid: Param0,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            apiid.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDebug {
    type Vtable = IDXGIDebug_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        295597138,
        56990,
        16638,
        [136, 6, 136, 249, 12, 18, 180, 65],
    );
}
impl ::std::convert::From<IDXGIDebug> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDebug) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDebug> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDebug) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDebug {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDebug {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        apiid: ::windows::runtime::GUID,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDebug1(::windows::runtime::IUnknown);
impl IDXGIDebug1 {
    pub unsafe fn ReportLiveObjects<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        apiid: Param0,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            apiid.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDebug1 {
    type Vtable = IDXGIDebug1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3315621644,
        5874,
        19167,
        [159, 77, 168, 196, 213, 138, 197, 80],
    );
}
impl ::std::convert::From<IDXGIDebug1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDebug1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDebug1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDebug1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDebug1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDebug1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDebug1> for IDXGIDebug {
    fn from(value: IDXGIDebug1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDebug1> for IDXGIDebug {
    fn from(value: &IDXGIDebug1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDebug> for IDXGIDebug1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDebug> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDebug>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDebug> for &IDXGIDebug1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDebug> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDebug>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        apiid: ::windows::runtime::GUID,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDecodeSwapChain(::windows::runtime::IUnknown);
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(
        &self,
        buffertopresent: u32,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffertopresent),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRect(
        &self,
        prect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(prect),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTargetRect(
        &self,
        prect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(prect),
        )
        .ok()
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSourceRect(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetRect(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::RECT>(result__)
    }
    pub unsafe fn GetDestSize(
        &self,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
        .ok()
    }
    pub unsafe fn SetColorSpace(
        &self,
        colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(colorspace),
        )
        .ok()
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDecodeSwapChain {
    type Vtable = IDXGIDecodeSwapChain_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        640878187,
        17684,
        19578,
        [143, 216, 18, 234, 152, 5, 157, 24],
    );
}
impl ::std::convert::From<IDXGIDecodeSwapChain> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDecodeSwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDecodeSwapChain> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDecodeSwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDecodeSwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDecodeSwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffertopresent: u32,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        prect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        prect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        prect: *mut super::super::Foundation::RECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        prect: *mut super::super::Foundation::RECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice(::windows::runtime::IUnknown);
impl IDXGIDevice {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
        .ok()
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const ::std::option::Option<::windows::runtime::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
        .ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDevice {
    type Vtable = IDXGIDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1424783354,
        4983,
        17638,
        [140, 50, 136, 253, 95, 68, 200, 76],
    );
}
impl ::std::convert::From<IDXGIDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDevice> for IDXGIObject {
    fn from(value: IDXGIDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice> for IDXGIObject {
    fn from(value: &IDXGIDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        padapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppresources: *const ::windows::runtime::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        priority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice1(::windows::runtime::IUnknown);
impl IDXGIDevice1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
        .ok()
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const ::std::option::Option<::windows::runtime::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
        .ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDevice1 {
    type Vtable = IDXGIDevice1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2010879759,
        25206,
        18618,
        [186, 40, 7, 1, 67, 180, 57, 44],
    );
}
impl ::std::convert::From<IDXGIDevice1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDevice1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDevice1> for IDXGIDevice {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice1> for IDXGIDevice {
    fn from(value: &IDXGIDevice1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for &IDXGIDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice1> for IDXGIObject {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice1> for IDXGIObject {
    fn from(value: &IDXGIDevice1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        padapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppresources: *const ::windows::runtime::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        priority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice2(::windows::runtime::IUnknown);
impl IDXGIDevice2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
        .ok()
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const ::std::option::Option<::windows::runtime::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
        .ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn OfferResources(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDevice2 {
    type Vtable = IDXGIDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        83920407,
        64509,
        16465,
        [167, 144, 20, 72, 132, 180, 246, 169],
    );
}
impl ::std::convert::From<IDXGIDevice2> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDevice2> for IDXGIDevice1 {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice1> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice1> for &IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice2> for IDXGIDevice {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for IDXGIDevice {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for &IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice2> for IDXGIObject {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for IDXGIObject {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        padapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppresources: *const ::windows::runtime::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        priority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        pdiscarded: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice3(::windows::runtime::IUnknown);
impl IDXGIDevice3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
        .ok()
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const ::std::option::Option<::windows::runtime::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
        .ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn OfferResources(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Trim(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDevice3 {
    type Vtable = IDXGIDevice3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1611106668,
        12868,
        19197,
        [191, 24, 166, 211, 190, 218, 80, 35],
    );
}
impl ::std::convert::From<IDXGIDevice3> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIDevice2 {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIDevice2 {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice2> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice2> for &IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIDevice1 {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice1> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice1> for &IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIDevice {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIDevice {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for &IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIObject {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIObject {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        padapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppresources: *const ::windows::runtime::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        priority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        pdiscarded: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice4(::windows::runtime::IUnknown);
impl IDXGIDevice4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
        .ok()
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const ::std::option::Option<::windows::runtime::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
        .ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn OfferResources(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Trim(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn OfferResources1(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn ReclaimResources1(
        &self,
        numresources: u32,
        ppresources: *const ::std::option::Option<IDXGIResource>,
    ) -> ::windows::runtime::Result<DXGI_RECLAIM_RESOURCE_RESULTS> {
        let mut result__: <DXGI_RECLAIM_RESOURCE_RESULTS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            &mut result__,
        )
        .from_abi::<DXGI_RECLAIM_RESOURCE_RESULTS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDevice4 {
    type Vtable = IDXGIDevice4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2511665503,
        55514,
        19620,
        [158, 230, 59, 118, 213, 150, 138, 16],
    );
}
impl ::std::convert::From<IDXGIDevice4> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice3 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice3 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice3> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice3> for &IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice2 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice2 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice2> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice2> for &IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice1 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice1> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice1> for &IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDevice> for &IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIObject {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIObject {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIDevice4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        padapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppresources: *const ::windows::runtime::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        priority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        pdiscarded: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numresources: u32,
        ppresources: *const ::windows::runtime::RawPtr,
        presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDeviceSubObject(::windows::runtime::IUnknown);
impl IDXGIDeviceSubObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDeviceSubObject {
    type Vtable = IDXGIDeviceSubObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1027474297,
        63966,
        19800,
        [187, 108, 24, 214, 41, 146, 241, 166],
    );
}
impl ::std::convert::From<IDXGIDeviceSubObject> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDeviceSubObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDeviceSubObject> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDeviceSubObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIDeviceSubObject> for IDXGIObject {
    fn from(value: IDXGIDeviceSubObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDeviceSubObject> for IDXGIObject {
    fn from(value: &IDXGIDeviceSubObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDisplayControl(::windows::runtime::IUnknown);
impl IDXGIDisplayControl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStereoEnabled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        enabled: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            enabled.into_param().abi(),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIDisplayControl {
    type Vtable = IDXGIDisplayControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3936206618,
        51342,
        17542,
        [133, 74, 152, 170, 1, 56, 243, 12],
    );
}
impl ::std::convert::From<IDXGIDisplayControl> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIDisplayControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDisplayControl> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIDisplayControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIDisplayControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIDisplayControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enabled: super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory(::windows::runtime::IUnknown);
impl IDXGIFactory {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory {
    type Vtable = IDXGIFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2071029484,
        8647,
        17582,
        [178, 26, 201, 174, 50, 26, 227, 105],
    );
}
impl ::std::convert::From<IDXGIFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory> for IDXGIObject {
    fn from(value: IDXGIFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory> for IDXGIObject {
    fn from(value: &IDXGIFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory1(::windows::runtime::IUnknown);
impl IDXGIFactory1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory1 {
    type Vtable = IDXGIFactory1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1997188728,
        62063,
        19898,
        [168, 41, 37, 60, 131, 209, 179, 135],
    );
}
impl ::std::convert::From<IDXGIFactory1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory1> for IDXGIFactory {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory1> for IDXGIFactory {
    fn from(value: &IDXGIFactory1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory1> for IDXGIObject {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory1> for IDXGIObject {
    fn from(value: &IDXGIFactory1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory2(::windows::runtime::IUnknown);
impl IDXGIFactory2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hwnd: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pwindow: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hresource: Param0,
    ) -> ::windows::runtime::Result<super::super::System::SystemServices::LUID> {
        let mut result__ : < super::super::System::SystemServices:: LUID as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            hresource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::SystemServices::LUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param2,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory2 {
    type Vtable = IDXGIFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1355299356,
        57458,
        19528,
        [135, 176, 54, 48, 250, 54, 166, 208],
    );
}
impl ::std::convert::From<IDXGIFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory2> for IDXGIFactory1 {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for &IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory2> for IDXGIFactory {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for IDXGIFactory {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory2> for IDXGIObject {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for IDXGIObject {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pwindow: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresource: super::super::Foundation::HANDLE,
        pluid: *mut super::super::System::SystemServices::LUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory3(::windows::runtime::IUnknown);
impl IDXGIFactory3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hwnd: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pwindow: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hresource: Param0,
    ) -> ::windows::runtime::Result<super::super::System::SystemServices::LUID> {
        let mut result__ : < super::super::System::SystemServices:: LUID as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            hresource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::SystemServices::LUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param2,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory3 {
    type Vtable = IDXGIFactory3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        625489955,
        52550,
        19581,
        [134, 202, 71, 170, 149, 184, 55, 189],
    );
}
impl ::std::convert::From<IDXGIFactory3> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIFactory2 {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for &IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIFactory1 {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for &IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIFactory {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIFactory {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIObject {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIObject {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pwindow: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresource: super::super::Foundation::HANDLE,
        pluid: *mut super::super::System::SystemServices::LUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory4(::windows::runtime::IUnknown);
impl IDXGIFactory4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hwnd: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pwindow: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hresource: Param0,
    ) -> ::windows::runtime::Result<super::super::System::SystemServices::LUID> {
        let mut result__ : < super::super::System::SystemServices:: LUID as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            hresource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::SystemServices::LUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param2,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn EnumAdapterByLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        adapterluid: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            adapterluid.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumWarpAdapter<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory4 {
    type Vtable = IDXGIFactory4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        466020866,
        61238,
        17999,
        [191, 12, 33, 202, 57, 229, 22, 138],
    );
}
impl ::std::convert::From<IDXGIFactory4> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory3 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for &IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory2 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for &IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory1 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for &IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIObject {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIObject {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pwindow: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresource: super::super::Foundation::HANDLE,
        pluid: *mut super::super::System::SystemServices::LUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapterluid: super::super::System::SystemServices::LUID,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory5(::windows::runtime::IUnknown);
impl IDXGIFactory5 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hwnd: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pwindow: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hresource: Param0,
    ) -> ::windows::runtime::Result<super::super::System::SystemServices::LUID> {
        let mut result__ : < super::super::System::SystemServices:: LUID as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            hresource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::SystemServices::LUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param2,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn EnumAdapterByLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        adapterluid: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            adapterluid.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumWarpAdapter<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(pfeaturesupportdata),
            ::std::mem::transmute(featuresupportdatasize),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory5 {
    type Vtable = IDXGIFactory5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1983046133,
        61029,
        19914,
        [135, 253, 132, 205, 117, 248, 131, 141],
    );
}
impl ::std::convert::From<IDXGIFactory5> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory4 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory4> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory4> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory3 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory2 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory1 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIObject {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIObject {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pwindow: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresource: super::super::Foundation::HANDLE,
        pluid: *mut super::super::System::SystemServices::LUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapterluid: super::super::System::SystemServices::LUID,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory6(::windows::runtime::IUnknown);
impl IDXGIFactory6 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hwnd: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pwindow: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hresource: Param0,
    ) -> ::windows::runtime::Result<super::super::System::SystemServices::LUID> {
        let mut result__ : < super::super::System::SystemServices:: LUID as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            hresource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::SystemServices::LUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param2,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn EnumAdapterByLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        adapterluid: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            adapterluid.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumWarpAdapter<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(pfeaturesupportdata),
            ::std::mem::transmute(featuresupportdatasize),
        )
        .ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T: ::windows::runtime::Interface>(
        &self,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(gpupreference),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory6 {
    type Vtable = IDXGIFactory6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3249957199,
        65289,
        17577,
        [176, 60, 119, 144, 10, 10, 29, 23],
    );
}
impl ::std::convert::From<IDXGIFactory6> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory5 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory5 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory5> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory5> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory4 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory4> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory4> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory3 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory2 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory1 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIObject {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIObject {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pwindow: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresource: super::super::Foundation::HANDLE,
        pluid: *mut super::super::System::SystemServices::LUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapterluid: super::super::System::SystemServices::LUID,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory7(::windows::runtime::IUnknown);
impl IDXGIFactory7 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::Result<IDXGISwapChain> {
        let mut result__: <IDXGISwapChain as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    >(
        &self,
        module: Param0,
    ) -> ::windows::runtime::Result<IDXGIAdapter> {
        let mut result__: <IDXGIAdapter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            module.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter>(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::runtime::Result<IDXGIAdapter1> {
        let mut result__: <IDXGIAdapter1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            &mut result__,
        )
        .from_abi::<IDXGIAdapter1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hwnd: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pwindow: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hresource: Param0,
    ) -> ::windows::runtime::Result<super::super::System::SystemServices::LUID> {
        let mut result__ : < super::super::System::SystemServices:: LUID as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            hresource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::SystemServices::LUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        windowhandle: Param0,
        wmsg: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param2,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn EnumAdapterByLuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        adapterluid: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            adapterluid.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn EnumWarpAdapter<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(pfeaturesupportdata),
            ::std::mem::transmute(featuresupportdatasize),
        )
        .ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T: ::windows::runtime::Interface>(
        &self,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(gpupreference),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterAdaptersChangedEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        hevent: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            hevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(
        &self,
        dwcookie: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwcookie),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactory7 {
    type Vtable = IDXGIFactory7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2761322221,
        30427,
        17626,
        [132, 193, 238, 154, 122, 251, 32, 168],
    );
}
impl ::std::convert::From<IDXGIFactory7> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory6 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory6 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory6> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory6>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory6> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory6>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory5 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory5 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory5> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory5> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory4 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory4> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory4> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory3 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory3> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory2 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory2> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory1 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory1> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIFactory> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIObject {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIObject {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIFactory7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwindowhandle: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: super::super::Foundation::HINSTANCE,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pwindow: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresource: super::super::Foundation::HANDLE,
        pluid: *mut super::super::System::SystemServices::LUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowhandle: super::super::Foundation::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapterluid: super::super::System::SystemServices::LUID,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::runtime::GUID,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hevent: super::super::Foundation::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwcookie: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactoryMedia(::windows::runtime::IUnknown);
impl IDXGIFactoryMedia {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hsurface: Param1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: Param3,
    ) -> ::windows::runtime::Result<IDXGISwapChain1> {
        let mut result__: <IDXGISwapChain1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hsurface.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGISwapChain1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
        Param3: ::windows::runtime::IntoParam<'a, IDXGIResource>,
        Param4: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: Param0,
        hsurface: Param1,
        pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC,
        pyuvdecodebuffers: Param3,
        prestricttooutput: Param4,
    ) -> ::windows::runtime::Result<IDXGIDecodeSwapChain> {
        let mut result__: <IDXGIDecodeSwapChain as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            hsurface.into_param().abi(),
            ::std::mem::transmute(pdesc),
            pyuvdecodebuffers.into_param().abi(),
            prestricttooutput.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIDecodeSwapChain>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIFactoryMedia {
    type Vtable = IDXGIFactoryMedia_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1105711602,
        42385,
        20347,
        [162, 229, 250, 156, 132, 62, 28, 18],
    );
}
impl ::std::convert::From<IDXGIFactoryMedia> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIFactoryMedia) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactoryMedia> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIFactoryMedia) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIFactoryMedia {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIFactoryMedia {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hsurface: super::super::Foundation::HANDLE,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        hsurface: super::super::Foundation::HANDLE,
        pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC,
        pyuvdecodebuffers: ::windows::runtime::RawPtr,
        prestricttooutput: ::windows::runtime::RawPtr,
        ppswapchain: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIInfoQueue(::windows::runtime::IUnknown);
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        messagecountlimit: u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(messagecountlimit),
        )
        .ok()
    }
    pub unsafe fn ClearStoredMessages<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        messageindex: u64,
        pmessage: *mut DXGI_INFO_QUEUE_MESSAGE,
        pmessagebytelength: *mut usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(messageindex),
            ::std::mem::transmute(pmessage),
            ::std::mem::transmute(pmessagebytelength),
        )
        .ok()
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetNumStoredMessages<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetMessageCountLimit<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn AddStorageFilterEntries<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
        )
        .ok()
    }
    pub unsafe fn GetStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pfilterbytelength),
        )
        .ok()
    }
    pub unsafe fn ClearStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn PushEmptyStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PushDenyAllStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PushCopyOfStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PushStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
        )
        .ok()
    }
    pub unsafe fn PopStorageFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetStorageFilterStackSize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn AddRetrievalFilterEntries<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
        )
        .ok()
    }
    pub unsafe fn GetRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pfilterbytelength),
        )
        .ok()
    }
    pub unsafe fn ClearRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn PushEmptyRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PushDenyAllRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn PushRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
        )
        .ok()
    }
    pub unsafe fn PopRetrievalFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    pub unsafe fn GetRetrievalFilterStackSize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    >(
        &self,
        producer: Param0,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        id: i32,
        pdescription: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
            ::std::mem::transmute(severity),
            ::std::mem::transmute(id),
            pdescription.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddApplicationMessage<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    >(
        &self,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        pdescription: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(severity),
            pdescription.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        producer: Param0,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        benable: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
            benable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        producer: Param0,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        benable: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(severity),
            benable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        producer: Param0,
        id: i32,
        benable: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(id),
            benable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(severity),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
        id: i32,
    ) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        producer: Param0,
        bmute: Param1,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
            bmute.into_param().abi(),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        producer: Param0,
    ) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            producer.into_param().abi(),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIInfoQueue {
    type Vtable = IDXGIInfoQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3597943239,
        26410,
        18287,
        [158, 130, 205, 85, 180, 73, 73, 206],
    );
}
impl ::std::convert::From<IDXGIInfoQueue> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIInfoQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIInfoQueue> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIInfoQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIInfoQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIInfoQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        messagecountlimit: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        messageindex: u64,
        pmessage: *mut DXGI_INFO_QUEUE_MESSAGE,
        pmessagebytelength: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        pfilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        id: i32,
        pdescription: super::super::Foundation::PSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        pdescription: super::super::Foundation::PSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        benable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        benable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        id: i32,
        benable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        id: i32,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
        bmute: super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        producer: ::windows::runtime::GUID,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIKeyedMutex(::windows::runtime::IUnknown);
impl IDXGIKeyedMutex {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn AcquireSync(
        &self,
        key: u64,
        dwmilliseconds: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(key),
            ::std::mem::transmute(dwmilliseconds),
        )
        .ok()
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(key),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIKeyedMutex {
    type Vtable = IDXGIKeyedMutex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2643333769,
        55219,
        18015,
        [129, 38, 37, 14, 52, 154, 248, 93],
    );
}
impl ::std::convert::From<IDXGIKeyedMutex> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIKeyedMutex> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIKeyedMutex> for IDXGIDeviceSubObject {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIKeyedMutex> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIKeyedMutex> for IDXGIObject {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIKeyedMutex> for IDXGIObject {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        key: u64,
        dwmilliseconds: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        key: u64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIObject(::windows::runtime::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIObject {
    type Vtable = IDXGIObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2931961784,
        30451,
        17977,
        [155, 224, 40, 235, 67, 166, 122, 46],
    );
}
impl ::std::convert::From<IDXGIObject> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIObject> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput(::windows::runtime::IUnknown);
impl IDXGIOutput {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput {
    type Vtable = IDXGIOutput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2919427803,
        50997,
        18064,
        [141, 82, 90, 141, 194, 2, 19, 170],
    );
}
impl ::std::convert::From<IDXGIOutput> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput> for IDXGIObject {
    fn from(value: IDXGIOutput) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput> for IDXGIObject {
    fn from(value: &IDXGIOutput) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput1(::windows::runtime::IUnknown);
impl IDXGIOutput1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGIResource>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DuplicateOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput1 {
    type Vtable = IDXGIOutput1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        13491880,
        37787,
        19331,
        [163, 64, 166, 133, 34, 102, 102, 204],
    );
}
impl ::std::convert::From<IDXGIOutput1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput1> for IDXGIOutput {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput1> for IDXGIOutput {
    fn from(value: &IDXGIOutput1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for &IDXGIOutput1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput1> for IDXGIObject {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput1> for IDXGIObject {
    fn from(value: &IDXGIOutput1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput2(::windows::runtime::IUnknown);
impl IDXGIOutput2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGIResource>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DuplicateOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput2 {
    type Vtable = IDXGIOutput2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1499347409,
        10020,
        18019,
        [153, 177, 218, 150, 157, 226, 131, 100],
    );
}
impl ::std::convert::From<IDXGIOutput2> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput2> for IDXGIOutput1 {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for &IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput2> for IDXGIOutput {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for IDXGIOutput {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for &IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput2> for IDXGIObject {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for IDXGIObject {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput3(::windows::runtime::IUnknown);
impl IDXGIOutput3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGIResource>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DuplicateOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CheckOverlaySupport<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput3 {
    type Vtable = IDXGIOutput3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2322313985,
        32382,
        16884,
        [168, 224, 91, 50, 247, 249, 155, 24],
    );
}
impl ::std::convert::From<IDXGIOutput3> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIOutput2 {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for &IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIOutput1 {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for &IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIOutput {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIOutput {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for &IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIObject {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIObject {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput4(::windows::runtime::IUnknown);
impl IDXGIOutput4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGIResource>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DuplicateOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CheckOverlaySupport<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(colorspace),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput4 {
    type Vtable = IDXGIOutput4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3699231285,
        8598,
        16717,
        [159, 83, 97, 120, 132, 3, 42, 96],
    );
}
impl ::std::convert::From<IDXGIOutput4> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput3 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput3> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput3> for &IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput2 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for &IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput1 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for &IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for &IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIObject {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIObject {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput5(::windows::runtime::IUnknown);
impl IDXGIOutput5 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGIResource>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DuplicateOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CheckOverlaySupport<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(colorspace),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn DuplicateOutput1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(supportedformatscount),
            ::std::mem::transmute(psupportedformats),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput5 {
    type Vtable = IDXGIOutput5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2157999140,
        43858,
        17131,
        [131, 60, 12, 66, 253, 40, 45, 152],
    );
}
impl ::std::convert::From<IDXGIOutput5> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput4 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput4 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput4> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput4> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput3 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput3> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput3> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput2 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput1 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIObject {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIObject {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput6(::windows::runtime::IUnknown);
impl IDXGIOutput6 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC> {
        let mut result__: <DXGI_OUTPUT_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC>(result__)
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pdevice: Param0,
        exclusive: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGammaControlCapabilities(
        &self,
    ) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL_CAPABILITIES> {
        let mut result__: <DXGI_GAMMA_CONTROL_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL_CAPABILITIES>(result__)
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(parray),
        )
        .ok()
    }
    pub unsafe fn GetGammaControl(&self) -> ::windows::runtime::Result<DXGI_GAMMA_CONTROL> {
        let mut result__: <DXGI_GAMMA_CONTROL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_GAMMA_CONTROL>(result__)
    }
    pub unsafe fn SetDisplaySurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pscanoutsurface.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGISurface>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDXGIResource>,
    >(
        &self,
        pdestination: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            pdestination.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DuplicateOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CheckOverlaySupport<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: Param2,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(colorspace),
            pconcerneddevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn DuplicateOutput1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pdevice: Param0,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
    ) -> ::windows::runtime::Result<IDXGIOutputDuplication> {
        let mut result__: <IDXGIOutputDuplication as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(supportedformatscount),
            ::std::mem::transmute(psupportedformats),
            &mut result__,
        )
        .from_abi::<IDXGIOutputDuplication>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_OUTPUT_DESC1> {
        let mut result__: <DXGI_OUTPUT_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_OUTPUT_DESC1>(result__)
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutput6 {
    type Vtable = IDXGIOutput6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        109266664,
        43756,
        19332,
        [173, 215, 19, 127, 81, 63, 119, 161],
    );
}
impl ::std::convert::From<IDXGIOutput6> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput5 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput5 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput5> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput5> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput5>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput4 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput4 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput4> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput4> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput3 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput3> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput3> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput2 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput2> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput1 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput1> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIOutput> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIOutput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIObject {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIObject {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutput6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        exclusive: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscanoutsurface: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevice: ::windows::runtime::RawPtr,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
        ppoutputduplication: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutputDuplication(::windows::runtime::IUnknown);
impl IDXGIOutputDuplication {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdesc),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireNextFrame(
        &self,
        timeoutinmilliseconds: u32,
        pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO,
        ppdesktopresource: *mut ::std::option::Option<IDXGIResource>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(timeoutinmilliseconds),
            ::std::mem::transmute(pframeinfo),
            ::std::mem::transmute(ppdesktopresource),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameDirtyRects(
        &self,
        dirtyrectsbuffersize: u32,
        pdirtyrectsbuffer: *mut super::super::Foundation::RECT,
        pdirtyrectsbuffersizerequired: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dirtyrectsbuffersize),
            ::std::mem::transmute(pdirtyrectsbuffer),
            ::std::mem::transmute(pdirtyrectsbuffersizerequired),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameMoveRects(
        &self,
        moverectsbuffersize: u32,
        pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        pmoverectsbuffersizerequired: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(moverectsbuffersize),
            ::std::mem::transmute(pmoverectbuffer),
            ::std::mem::transmute(pmoverectsbuffersizerequired),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFramePointerShape(
        &self,
        pointershapebuffersize: u32,
        ppointershapebuffer: *mut ::std::ffi::c_void,
        ppointershapebuffersizerequired: *mut u32,
        ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pointershapebuffersize),
            ::std::mem::transmute(ppointershapebuffer),
            ::std::mem::transmute(ppointershapebuffersizerequired),
            ::std::mem::transmute(ppointershapeinfo),
        )
        .ok()
    }
    pub unsafe fn MapDesktopSurface(&self) -> ::windows::runtime::Result<DXGI_MAPPED_RECT> {
        let mut result__: <DXGI_MAPPED_RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MAPPED_RECT>(result__)
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ReleaseFrame(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIOutputDuplication {
    type Vtable = IDXGIOutputDuplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        421329603,
        41793,
        18189,
        [178, 110, 168, 100, 244, 40, 49, 156],
    );
}
impl ::std::convert::From<IDXGIOutputDuplication> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIOutputDuplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutputDuplication> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIOutputDuplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDXGIOutputDuplication
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDXGIOutputDuplication
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIOutputDuplication> for IDXGIObject {
    fn from(value: IDXGIOutputDuplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutputDuplication> for IDXGIObject {
    fn from(value: &IDXGIOutputDuplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdesc: *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timeoutinmilliseconds: u32,
        pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO,
        ppdesktopresource: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dirtyrectsbuffersize: u32,
        pdirtyrectsbuffer: *mut super::super::Foundation::RECT,
        pdirtyrectsbuffersizerequired: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        moverectsbuffersize: u32,
        pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        pmoverectsbuffersizerequired: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pointershapebuffersize: u32,
        ppointershapebuffer: *mut ::std::ffi::c_void,
        ppointershapebuffersizerequired: *mut u32,
        ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIResource(::windows::runtime::IUnknown);
impl IDXGIResource {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetEvictionPriority(
        &self,
        evictionpriority: DXGI_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(evictionpriority),
        )
        .ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIResource {
    type Vtable = IDXGIResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        56572596,
        18478,
        20048,
        [180, 31, 138, 127, 139, 216, 150, 11],
    );
}
impl ::std::convert::From<IDXGIResource> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIResource> for IDXGIDeviceSubObject {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGIResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource> for IDXGIObject {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource> for IDXGIObject {
    fn from(value: &IDXGIResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psharedhandle: *mut super::super::Foundation::HANDLE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pusage: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        evictionpriority: DXGI_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pevictionpriority: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIResource1(::windows::runtime::IUnknown);
impl IDXGIResource1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetEvictionPriority(
        &self,
        evictionpriority: DXGI_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(evictionpriority),
        )
        .ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CreateSubresourceSurface(
        &self,
        index: u32,
    ) -> ::windows::runtime::Result<IDXGISurface2> {
        let mut result__: <IDXGISurface2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<IDXGISurface2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        dwaccess: u32,
        lpname: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pattributes),
            ::std::mem::transmute(dwaccess),
            lpname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGIResource1 {
    type Vtable = IDXGIResource1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        815141753,
        17929,
        19009,
        [153, 142, 84, 254, 86, 126, 224, 193],
    );
}
impl ::std::convert::From<IDXGIResource1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGIResource1> for IDXGIResource {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for IDXGIResource {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIResource> for IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIResource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIResource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIResource> for &IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIResource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIResource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource1> for IDXGIDeviceSubObject {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource1> for IDXGIObject {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for IDXGIObject {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGIResource1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psharedhandle: *mut super::super::Foundation::HANDLE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pusage: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        evictionpriority: DXGI_RESOURCE_PRIORITY,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pevictionpriority: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ppsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        dwaccess: u32,
        lpname: super::super::Foundation::PWSTR,
        phandle: *mut super::super::Foundation::HANDLE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISurface(::windows::runtime::IUnknown);
impl IDXGISurface {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SURFACE_DESC> {
        let mut result__: <DXGI_SURFACE_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SURFACE_DESC>(result__)
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plockedrect),
            ::std::mem::transmute(mapflags),
        )
        .ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISurface {
    type Vtable = IDXGISurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3405559148,
        27331,
        18569,
        [191, 71, 158, 35, 187, 210, 96, 236],
    );
}
impl ::std::convert::From<IDXGISurface> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISurface> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface> for IDXGIObject {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface> for IDXGIObject {
    fn from(value: &IDXGISurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISurface1(::windows::runtime::IUnknown);
impl IDXGISurface1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SURFACE_DESC> {
        let mut result__: <DXGI_SURFACE_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SURFACE_DESC>(result__)
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plockedrect),
            ::std::mem::transmute(mapflags),
        )
        .ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        discard: Param0,
    ) -> ::windows::runtime::Result<super::Gdi::HDC> {
        let mut result__: <super::Gdi::HDC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            discard.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::Gdi::HDC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(
        &self,
        pdirtyrect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdirtyrect),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISurface1 {
    type Vtable = IDXGISurface1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1256599698,
        25383,
        19483,
        [128, 174, 191, 225, 46, 163, 43, 134],
    );
}
impl ::std::convert::From<IDXGISurface1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISurface1> for IDXGISurface {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for IDXGISurface {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISurface> for IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISurface> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISurface>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISurface> for &IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISurface> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISurface>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface1> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface1> for IDXGIObject {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for IDXGIObject {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISurface1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        discard: super::super::Foundation::BOOL,
        phdc: *mut super::Gdi::HDC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdirtyrect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISurface2(::windows::runtime::IUnknown);
impl IDXGISurface2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SURFACE_DESC> {
        let mut result__: <DXGI_SURFACE_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SURFACE_DESC>(result__)
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plockedrect),
            ::std::mem::transmute(mapflags),
        )
        .ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        discard: Param0,
    ) -> ::windows::runtime::Result<super::Gdi::HDC> {
        let mut result__: <super::Gdi::HDC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            discard.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::Gdi::HDC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(
        &self,
        pdirtyrect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdirtyrect),
        )
        .ok()
    }
    pub unsafe fn GetResource(
        &self,
        riid: *const ::windows::runtime::GUID,
        ppparentresource: *mut *mut ::std::ffi::c_void,
        psubresourceindex: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparentresource),
            ::std::mem::transmute(psubresourceindex),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISurface2 {
    type Vtable = IDXGISurface2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2879690461,
        46615,
        19640,
        [168, 102, 188, 68, 215, 235, 31, 162],
    );
}
impl ::std::convert::From<IDXGISurface2> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGISurface1 {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGISurface1 {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISurface1> for IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISurface1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISurface1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISurface1> for &IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISurface1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISurface1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGISurface {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGISurface {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISurface> for IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISurface> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISurface>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISurface> for &IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISurface> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISurface>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGIObject {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGIObject {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISurface2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        discard: super::super::Foundation::BOOL,
        phdc: *mut super::Gdi::HDC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdirtyrect: *const super::super::Foundation::RECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparentresource: *mut *mut ::std::ffi::c_void,
        psubresourceindex: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain(::windows::runtime::IUnknown);
impl IDXGISwapChain {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn GetBuffer<T: ::windows::runtime::Interface>(
        &self,
        buffer: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffer),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param1: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: Param0,
        ptarget: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
        .ok()
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
        .ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISwapChain {
    type Vtable = IDXGISwapChain_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        822949536,
        53991,
        19466,
        [170, 4, 106, 157, 35, 184, 136, 106],
    );
}
impl ::std::convert::From<IDXGISwapChain> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain> for IDXGIObject {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain> for IDXGIObject {
    fn from(value: &IDXGISwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISwapChain {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer: u32,
        riid: *const ::windows::runtime::GUID,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fullscreen: super::super::Foundation::BOOL,
        ptarget: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain1(::windows::runtime::IUnknown);
impl IDXGISwapChain1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn GetBuffer<T: ::windows::runtime::Interface>(
        &self,
        buffer: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffer),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param1: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: Param0,
        ptarget: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
        .ok()
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
        .ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenDesc(
        &self,
    ) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_FULLSCREEN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn GetCoreWindow<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn SetBackgroundColor(
        &self,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolor),
        )
        .ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::runtime::Result<DXGI_RGBA> {
        let mut result__: <DXGI_RGBA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_RGBA>(result__)
    }
    pub unsafe fn SetRotation(
        &self,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rotation),
        )
        .ok()
    }
    pub unsafe fn GetRotation(&self) -> ::windows::runtime::Result<DXGI_MODE_ROTATION> {
        let mut result__: <DXGI_MODE_ROTATION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MODE_ROTATION>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISwapChain1 {
    type Vtable = IDXGISwapChain1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2030716407,
        3394,
        18550,
        [152, 58, 10, 85, 207, 230, 244, 170],
    );
}
impl ::std::convert::From<IDXGISwapChain1> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain1> for IDXGISwapChain {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for &IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain1> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain1> for IDXGIObject {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for IDXGIObject {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISwapChain1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer: u32,
        riid: *const ::windows::runtime::GUID,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fullscreen: super::super::Foundation::BOOL,
        ptarget: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        refiid: *const ::windows::runtime::GUID,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pprestricttooutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain2(::windows::runtime::IUnknown);
impl IDXGISwapChain2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn GetBuffer<T: ::windows::runtime::Interface>(
        &self,
        buffer: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffer),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param1: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: Param0,
        ptarget: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
        .ok()
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
        .ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenDesc(
        &self,
    ) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_FULLSCREEN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn GetCoreWindow<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn SetBackgroundColor(
        &self,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolor),
        )
        .ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::runtime::Result<DXGI_RGBA> {
        let mut result__: <DXGI_RGBA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_RGBA>(result__)
    }
    pub unsafe fn SetRotation(
        &self,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rotation),
        )
        .ok()
    }
    pub unsafe fn GetRotation(&self) -> ::windows::runtime::Result<DXGI_MODE_ROTATION> {
        let mut result__: <DXGI_MODE_ROTATION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MODE_ROTATION>(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
        .ok()
    }
    pub unsafe fn GetSourceSize(
        &self,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
        .ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmatrix),
        )
        .ok()
    }
    pub unsafe fn GetMatrixTransform(&self) -> ::windows::runtime::Result<DXGI_MATRIX_3X2_F> {
        let mut result__: <DXGI_MATRIX_3X2_F as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MATRIX_3X2_F>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISwapChain2 {
    type Vtable = IDXGISwapChain2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2831035076,
        6559,
        18758,
        [179, 49, 121, 89, 159, 185, 141, 231],
    );
}
impl ::std::convert::From<IDXGISwapChain2> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain1> for &IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGISwapChain {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for &IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGIObject {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGIObject {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISwapChain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer: u32,
        riid: *const ::windows::runtime::GUID,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fullscreen: super::super::Foundation::BOOL,
        ptarget: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        refiid: *const ::windows::runtime::GUID,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pprestricttooutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain3(::windows::runtime::IUnknown);
impl IDXGISwapChain3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn GetBuffer<T: ::windows::runtime::Interface>(
        &self,
        buffer: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffer),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param1: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: Param0,
        ptarget: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
        .ok()
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
        .ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenDesc(
        &self,
    ) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_FULLSCREEN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn GetCoreWindow<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn SetBackgroundColor(
        &self,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolor),
        )
        .ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::runtime::Result<DXGI_RGBA> {
        let mut result__: <DXGI_RGBA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_RGBA>(result__)
    }
    pub unsafe fn SetRotation(
        &self,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rotation),
        )
        .ok()
    }
    pub unsafe fn GetRotation(&self) -> ::windows::runtime::Result<DXGI_MODE_ROTATION> {
        let mut result__: <DXGI_MODE_ROTATION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MODE_ROTATION>(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
        .ok()
    }
    pub unsafe fn GetSourceSize(
        &self,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
        .ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmatrix),
        )
        .ok()
    }
    pub unsafe fn GetMatrixTransform(&self) -> ::windows::runtime::Result<DXGI_MATRIX_3X2_F> {
        let mut result__: <DXGI_MATRIX_3X2_F as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MATRIX_3X2_F>(result__)
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CheckColorSpaceSupport(
        &self,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(colorspace),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetColorSpace1(
        &self,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(colorspace),
        )
        .ok()
    }
    pub unsafe fn ResizeBuffers1(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *const ::std::option::Option<::windows::runtime::IUnknown>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(format),
            ::std::mem::transmute(swapchainflags),
            ::std::mem::transmute(pcreationnodemask),
            ::std::mem::transmute(pppresentqueue),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISwapChain3 {
    type Vtable = IDXGISwapChain3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2497289179,
        61944,
        19120,
        [178, 54, 125, 160, 23, 14, 218, 177],
    );
}
impl ::std::convert::From<IDXGISwapChain3> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGISwapChain2 {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGISwapChain2 {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain2> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain2> for &IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain1> for &IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGISwapChain {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for &IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGIObject {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGIObject {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISwapChain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer: u32,
        riid: *const ::windows::runtime::GUID,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fullscreen: super::super::Foundation::BOOL,
        ptarget: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        refiid: *const ::windows::runtime::GUID,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pprestricttooutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pcolorspacesupport: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *const ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain4(::windows::runtime::IUnknown);
impl IDXGISwapChain4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        name: *const ::windows::runtime::GUID,
        punknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
        .ok()
    }
    pub unsafe fn GetParent<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn GetBuffer<T: ::windows::runtime::Interface>(
        &self,
        buffer: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffer),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param1: ::windows::runtime::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: Param0,
        ptarget: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC>(result__)
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
        .ok()
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
        .ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS> {
        let mut result__: <DXGI_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut result__: <DXGI_SWAP_CHAIN_DESC1 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_DESC1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenDesc(
        &self,
    ) -> ::windows::runtime::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        let mut result__: <DXGI_SWAP_CHAIN_FULLSCREEN_DESC as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn GetCoreWindow<T: ::windows::runtime::Interface>(
        &self,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::runtime::Result<IDXGIOutput> {
        let mut result__: <IDXGIOutput as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDXGIOutput>(result__)
    }
    pub unsafe fn SetBackgroundColor(
        &self,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolor),
        )
        .ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::runtime::Result<DXGI_RGBA> {
        let mut result__: <DXGI_RGBA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_RGBA>(result__)
    }
    pub unsafe fn SetRotation(
        &self,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rotation),
        )
        .ok()
    }
    pub unsafe fn GetRotation(&self) -> ::windows::runtime::Result<DXGI_MODE_ROTATION> {
        let mut result__: <DXGI_MODE_ROTATION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MODE_ROTATION>(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
        .ok()
    }
    pub unsafe fn GetSourceSize(
        &self,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
        .ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(maxlatency),
        )
        .ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmatrix),
        )
        .ok()
    }
    pub unsafe fn GetMatrixTransform(&self) -> ::windows::runtime::Result<DXGI_MATRIX_3X2_F> {
        let mut result__: <DXGI_MATRIX_3X2_F as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_MATRIX_3X2_F>(result__)
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CheckColorSpaceSupport(
        &self,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(colorspace),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetColorSpace1(
        &self,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(colorspace),
        )
        .ok()
    }
    pub unsafe fn ResizeBuffers1(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *const ::std::option::Option<::windows::runtime::IUnknown>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(format),
            ::std::mem::transmute(swapchainflags),
            ::std::mem::transmute(pcreationnodemask),
            ::std::mem::transmute(pppresentqueue),
        )
        .ok()
    }
    pub unsafe fn SetHDRMetaData(
        &self,
        r#type: DXGI_HDR_METADATA_TYPE,
        size: u32,
        pmetadata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(size),
            ::std::mem::transmute(pmetadata),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISwapChain4 {
    type Vtable = IDXGISwapChain4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1029201242,
        48458,
        18590,
        [177, 244, 61, 188, 182, 69, 47, 251],
    );
}
impl ::std::convert::From<IDXGISwapChain4> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain3 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain3 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain3> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain3> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain2 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain2 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain2> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain2> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain1> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain1> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGISwapChain> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGISwapChain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIDeviceSubObject> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIDeviceSubObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGIObject {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGIObject {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDXGIObject> for &IDXGISwapChain4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDXGIObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *const ::windows::runtime::GUID,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffer: u32,
        riid: *const ::windows::runtime::GUID,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fullscreen: super::super::Foundation::BOOL,
        ptarget: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfullscreen: *mut super::super::Foundation::BOOL,
        pptarget: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppoutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        refiid: *const ::windows::runtime::GUID,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pprestricttooutput: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlatency: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pcolorspacesupport: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *const ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: DXGI_HDR_METADATA_TYPE,
        size: u32,
        pmetadata: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChainMedia(::windows::runtime::IUnknown);
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(
        &self,
    ) -> ::windows::runtime::Result<DXGI_FRAME_STATISTICS_MEDIA> {
        let mut result__: <DXGI_FRAME_STATISTICS_MEDIA as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DXGI_FRAME_STATISTICS_MEDIA>(result__)
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(duration),
        )
        .ok()
    }
    pub unsafe fn CheckPresentDurationSupport(
        &self,
        desiredpresentduration: u32,
        pclosestsmallerpresentduration: *mut u32,
        pclosestlargerpresentduration: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(desiredpresentduration),
            ::std::mem::transmute(pclosestsmallerpresentduration),
            ::std::mem::transmute(pclosestlargerpresentduration),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDXGISwapChainMedia {
    type Vtable = IDXGISwapChainMedia_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3717576971,
        61535,
        20330,
        [189, 101, 37, 191, 178, 100, 189, 132],
    );
}
impl ::std::convert::From<IDXGISwapChainMedia> for ::windows::runtime::IUnknown {
    fn from(value: IDXGISwapChainMedia) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChainMedia> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGISwapChainMedia) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGISwapChainMedia {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGISwapChainMedia {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS_MEDIA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        duration: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        desiredpresentduration: u32,
        pclosestsmallerpresentduration: *mut u32,
        pclosestlargerpresentduration: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGraphicsAnalysis(::windows::runtime::IUnknown);
impl IDXGraphicsAnalysis {
    pub unsafe fn BeginCapture(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn EndCapture(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IDXGraphicsAnalysis {
    type Vtable = IDXGraphicsAnalysis_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2670007572,
        40269,
        18690,
        [157, 96, 24, 152, 138, 183, 212, 181],
    );
}
impl ::std::convert::From<IDXGraphicsAnalysis> for ::windows::runtime::IUnknown {
    fn from(value: IDXGraphicsAnalysis) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGraphicsAnalysis> for ::windows::runtime::IUnknown {
    fn from(value: &IDXGraphicsAnalysis) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDXGraphicsAnalysis {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDXGraphicsAnalysis {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGraphicsAnalysis_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
pub const _FACDXGI: u32 = 2170u32;
