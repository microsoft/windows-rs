::windows_sys::core::windows_link ! ( "dxcore.dll" ,"system" fn DXCoreCreateAdapterFactory ( riid : *const :: windows_sys::core::GUID , ppvfactory : *mut *mut ::core::ffi::c_void ) -> :: windows_sys::core::HRESULT );
pub type IDXCoreAdapter = *mut ::core::ffi::c_void;
pub type IDXCoreAdapterFactory = *mut ::core::ffi::c_void;
pub type IDXCoreAdapterList = *mut ::core::ffi::c_void;
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const _FACDXCORE: u32 = 2176u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreAdapterPreference = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const Hardware: DXCoreAdapterPreference = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const MinimumPower: DXCoreAdapterPreference = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HighPerformance: DXCoreAdapterPreference = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreAdapterProperty = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const InstanceLuid: DXCoreAdapterProperty = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DriverVersion: DXCoreAdapterProperty = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DriverDescription: DXCoreAdapterProperty = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HardwareID: DXCoreAdapterProperty = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const KmdModelVersion: DXCoreAdapterProperty = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DedicatedSystemMemory: DXCoreAdapterProperty = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const SharedSystemMemory: DXCoreAdapterProperty = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AcgCompatible: DXCoreAdapterProperty = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsHardware: DXCoreAdapterProperty = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsIntegrated: DXCoreAdapterProperty = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsDetachable: DXCoreAdapterProperty = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HardwareIDParts: DXCoreAdapterProperty = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreAdapterState = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsDriverUpdateInProgress: DXCoreAdapterState = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterMemoryBudget: DXCoreAdapterState = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreNotificationType = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterListStale: DXCoreNotificationType = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterNoLongerValid: DXCoreNotificationType = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterBudgetChange: DXCoreNotificationType = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreSegmentGroup = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const Local: DXCoreSegmentGroup = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const NonLocal: DXCoreSegmentGroup = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
impl ::core::marker::Copy for DXCoreAdapterMemoryBudget {}
impl ::core::clone::Clone for DXCoreAdapterMemoryBudget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl ::core::marker::Copy for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::clone::Clone for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
impl ::core::marker::Copy for DXCoreHardwareID {}
impl ::core::clone::Clone for DXCoreHardwareID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
impl ::core::marker::Copy for DXCoreHardwareIDParts {}
impl ::core::clone::Clone for DXCoreHardwareIDParts {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::windows_sys::core::IUnknown, context: *const ::core::ffi::c_void) -> ()>;
