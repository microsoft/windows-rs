#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn DXCoreCreateAdapterFactory(riid: *const ::windows_sys::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2353497707,
    data2: 30083,
    data3: 17677,
    data4: [240, 240, 107, 173, 168, 149, 175, 75],
};
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 613296128,
    data2: 42899,
    data3: 18212,
    data4: [171, 170, 35, 166, 222, 27, 224, 144],
};
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 211734093, data2: 12142, data3: 20225, data4: [140, 150, 232, 158, 51, 27, 71, 177] };
#[repr(C)]
pub struct DXCoreAdapterMemoryBudget(i32);
#[repr(C)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup(i32);
#[repr(transparent)]
pub struct DXCoreAdapterPreference(pub u32);
pub const Hardware: DXCoreAdapterPreference = DXCoreAdapterPreference(0u32);
pub const MinimumPower: DXCoreAdapterPreference = DXCoreAdapterPreference(1u32);
pub const HighPerformance: DXCoreAdapterPreference = DXCoreAdapterPreference(2u32);
#[repr(transparent)]
pub struct DXCoreAdapterProperty(pub u32);
pub const InstanceLuid: DXCoreAdapterProperty = DXCoreAdapterProperty(0u32);
pub const DriverVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(1u32);
pub const DriverDescription: DXCoreAdapterProperty = DXCoreAdapterProperty(2u32);
pub const HardwareID: DXCoreAdapterProperty = DXCoreAdapterProperty(3u32);
pub const KmdModelVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(4u32);
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(5u32);
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(6u32);
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(7u32);
pub const DedicatedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(8u32);
pub const SharedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(9u32);
pub const AcgCompatible: DXCoreAdapterProperty = DXCoreAdapterProperty(10u32);
pub const IsHardware: DXCoreAdapterProperty = DXCoreAdapterProperty(11u32);
pub const IsIntegrated: DXCoreAdapterProperty = DXCoreAdapterProperty(12u32);
pub const IsDetachable: DXCoreAdapterProperty = DXCoreAdapterProperty(13u32);
pub const HardwareIDParts: DXCoreAdapterProperty = DXCoreAdapterProperty(14u32);
#[repr(transparent)]
pub struct DXCoreAdapterState(pub u32);
pub const IsDriverUpdateInProgress: DXCoreAdapterState = DXCoreAdapterState(0u32);
pub const AdapterMemoryBudget: DXCoreAdapterState = DXCoreAdapterState(1u32);
#[repr(C)]
pub struct DXCoreHardwareID(i32);
#[repr(C)]
pub struct DXCoreHardwareIDParts(i32);
#[repr(transparent)]
pub struct DXCoreNotificationType(pub u32);
pub const AdapterListStale: DXCoreNotificationType = DXCoreNotificationType(0u32);
pub const AdapterNoLongerValid: DXCoreNotificationType = DXCoreNotificationType(1u32);
pub const AdapterBudgetChange: DXCoreNotificationType = DXCoreNotificationType(2u32);
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = DXCoreNotificationType(3u32);
#[repr(transparent)]
pub struct DXCoreSegmentGroup(pub u32);
pub const Local: DXCoreSegmentGroup = DXCoreSegmentGroup(0u32);
pub const NonLocal: DXCoreSegmentGroup = DXCoreSegmentGroup(1u32);
#[repr(transparent)]
pub struct IDXCoreAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDXCoreAdapterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDXCoreAdapterList(pub *mut ::core::ffi::c_void);
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::windows_sys::core::IUnknown, context: *const ::core::ffi::c_void);
pub const _FACDXCORE: u32 = 2176u32;
