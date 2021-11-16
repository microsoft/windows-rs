#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn DXCoreCreateAdapterFactory(riid: *const ::windows_sys::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2353497707,
    data2: 30083,
    data3: 17677,
    data4: [240, 240, 107, 173, 168, 149, 175, 75],
};
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 613296128,
    data2: 42899,
    data3: 18212,
    data4: [171, 170, 35, 166, 222, 27, 224, 144],
};
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 211734093, data2: 12142, data3: 20225, data4: [140, 150, 232, 158, 51, 27, 71, 177] };
#[repr(C)]
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
pub type DXCoreAdapterPreference = u32;
pub const Hardware: DXCoreAdapterPreference = 0u32;
pub const MinimumPower: DXCoreAdapterPreference = 1u32;
pub const HighPerformance: DXCoreAdapterPreference = 2u32;
pub type DXCoreAdapterProperty = u32;
pub const InstanceLuid: DXCoreAdapterProperty = 0u32;
pub const DriverVersion: DXCoreAdapterProperty = 1u32;
pub const DriverDescription: DXCoreAdapterProperty = 2u32;
pub const HardwareID: DXCoreAdapterProperty = 3u32;
pub const KmdModelVersion: DXCoreAdapterProperty = 4u32;
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = 5u32;
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = 6u32;
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = 7u32;
pub const DedicatedSystemMemory: DXCoreAdapterProperty = 8u32;
pub const SharedSystemMemory: DXCoreAdapterProperty = 9u32;
pub const AcgCompatible: DXCoreAdapterProperty = 10u32;
pub const IsHardware: DXCoreAdapterProperty = 11u32;
pub const IsIntegrated: DXCoreAdapterProperty = 12u32;
pub const IsDetachable: DXCoreAdapterProperty = 13u32;
pub const HardwareIDParts: DXCoreAdapterProperty = 14u32;
pub type DXCoreAdapterState = u32;
pub const IsDriverUpdateInProgress: DXCoreAdapterState = 0u32;
pub const AdapterMemoryBudget: DXCoreAdapterState = 1u32;
#[repr(C)]
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
pub type DXCoreNotificationType = u32;
pub const AdapterListStale: DXCoreNotificationType = 0u32;
pub const AdapterNoLongerValid: DXCoreNotificationType = 1u32;
pub const AdapterBudgetChange: DXCoreNotificationType = 2u32;
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = 3u32;
pub type DXCoreSegmentGroup = u32;
pub const Local: DXCoreSegmentGroup = 0u32;
pub const NonLocal: DXCoreSegmentGroup = 1u32;
pub type IDXCoreAdapter = *mut ::core::ffi::c_void;
pub type IDXCoreAdapterFactory = *mut ::core::ffi::c_void;
pub type IDXCoreAdapterList = *mut ::core::ffi::c_void;
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::windows_sys::core::IUnknown, context: *const ::core::ffi::c_void);
pub const _FACDXCORE: u32 = 2176u32;
