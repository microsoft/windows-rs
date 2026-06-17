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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HRESULT = i32;
pub const IID_ID3D12DeviceChild: GUID = GUID {
    data1: 0x905db94b,
    data2: 0xa00c,
    data3: 0x4140,
    data4: [157, 245, 43, 100, 202, 158, 163, 87],
};
#[repr(C)]
pub struct ID3D12DeviceChild_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
}
pub const IID_ID3D12Object: GUID = GUID {
    data1: 0xc4fec28f,
    data2: 0x7966,
    data3: 0x4e95,
    data4: [159, 148, 244, 49, 203, 86, 195, 184],
};
#[repr(C)]
pub struct ID3D12Object_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        u32,
        *const core::ffi::c_void,
    ) -> HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *mut core::ffi::c_void,
    ) -> HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, PCWSTR) -> HRESULT,
}
pub const IID_ID3D12Pageable: GUID = GUID {
    data1: 0x63ee58fb,
    data2: 0x1268,
    data3: 0x4835,
    data4: [134, 218, 240, 8, 206, 98, 240, 214],
};
#[repr(C)]
pub struct ID3D12Pageable_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
pub const IID_ID3D12Resource: GUID = GUID {
    data1: 0x696442be,
    data2: 0xa72e,
    data3: 0x4059,
    data4: [188, 121, 91, 92, 152, 4, 15, 173],
};
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
pub const IID_IUnknown: GUID = GUID {
    data1: 0x00000000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [192, 0, 0, 0, 0, 0, 0, 70],
};
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}
pub type PCWSTR = *const u16;
