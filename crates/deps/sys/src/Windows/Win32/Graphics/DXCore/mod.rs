#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct DXCoreAdapterPreference(i32);
#[repr(C)]
pub struct DXCoreAdapterProperty(i32);
#[repr(C)]
pub struct DXCoreAdapterState(i32);
#[repr(C)]
pub struct DXCoreHardwareID(i32);
#[repr(C)]
pub struct DXCoreHardwareIDParts(i32);
#[repr(C)]
pub struct DXCoreNotificationType(i32);
#[repr(C)]
pub struct DXCoreSegmentGroup(i32);
#[repr(transparent)]
pub struct IDXCoreAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDXCoreAdapterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDXCoreAdapterList(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PFN_DXCORE_NOTIFICATION_CALLBACK(i32);
pub const _FACDXCORE: u32 = 2176u32;
