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
pub const Hardware: u32 = 0u32;
pub const MinimumPower: u32 = 1u32;
pub const HighPerformance: u32 = 2u32;
pub const InstanceLuid: u32 = 0u32;
pub const DriverVersion: u32 = 1u32;
pub const DriverDescription: u32 = 2u32;
pub const HardwareID: u32 = 3u32;
pub const KmdModelVersion: u32 = 4u32;
pub const ComputePreemptionGranularity: u32 = 5u32;
pub const GraphicsPreemptionGranularity: u32 = 6u32;
pub const DedicatedAdapterMemory: u32 = 7u32;
pub const DedicatedSystemMemory: u32 = 8u32;
pub const SharedSystemMemory: u32 = 9u32;
pub const AcgCompatible: u32 = 10u32;
pub const IsHardware: u32 = 11u32;
pub const IsIntegrated: u32 = 12u32;
pub const IsDetachable: u32 = 13u32;
pub const HardwareIDParts: u32 = 14u32;
pub const IsDriverUpdateInProgress: u32 = 0u32;
pub const AdapterMemoryBudget: u32 = 1u32;
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
pub const AdapterListStale: u32 = 0u32;
pub const AdapterNoLongerValid: u32 = 1u32;
pub const AdapterBudgetChange: u32 = 2u32;
pub const AdapterHardwareContentProtectionTeardown: u32 = 3u32;
pub const Local: u32 = 0u32;
pub const NonLocal: u32 = 1u32;
#[repr(transparent)]
pub struct IDXCoreAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXCoreAdapter {}
impl ::core::clone::Clone for IDXCoreAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXCoreAdapterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXCoreAdapterFactory {}
impl ::core::clone::Clone for IDXCoreAdapterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDXCoreAdapterList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDXCoreAdapterList {}
impl ::core::clone::Clone for IDXCoreAdapterList {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::windows_sys::core::IUnknown, context: *const ::core::ffi::c_void);
pub const _FACDXCORE: u32 = 2176u32;
