#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: *mut core::ffi::c_void,
}
impl Default for D3D12_RESOURCE_UAV_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IID_ID3D12DeviceChild: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x905db94b_a00c_4140_9df5_2b64ca9ea357);
#[repr(C)]
pub struct ID3D12DeviceChild_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_sys::core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_sys::core::HRESULT,
}
pub const IID_ID3D12Object: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0xc4fec28f_7966_4e95_9f94_f431cb56c3b8);
#[repr(C)]
pub struct ID3D12Object_Vtbl {
    pub base__: windows_sys::core::IUnknown_Vtbl,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_sys::core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_sys::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_sys::core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_sys::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_sys::core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_sys::core::PCWSTR,
    ) -> windows_sys::core::HRESULT,
}
pub const IID_ID3D12Pageable: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x63ee58fb_1268_4835_86da_f008ce62f0d6);
#[repr(C)]
pub struct ID3D12Pageable_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
pub const IID_ID3D12Resource: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x696442be_a72e_4059_bc79_5b5c98040fad);
#[repr(C)]
pub struct ID3D12Resource_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    Map: usize,
    Unmap: usize,
    GetDesc: usize,
    pub GetGPUVirtualAddress: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    WriteToSubresource: usize,
    ReadFromSubresource: usize,
    GetHeapProperties: usize,
}
