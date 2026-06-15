#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
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
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HRESULT = i32;
pub const IID_ID3D12DeviceChild: GUID = GUID::from_u128(0x905db94b_a00c_4140_9df5_2b64ca9ea357);
#[repr(C)]
pub struct ID3D12DeviceChild_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
}
pub const IID_ID3D12Object: GUID = GUID::from_u128(0xc4fec28f_7966_4e95_9f94_f431cb56c3b8);
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
pub const IID_ID3D12Pageable: GUID = GUID::from_u128(0x63ee58fb_1268_4835_86da_f008ce62f0d6);
#[repr(C)]
pub struct ID3D12Pageable_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
pub const IID_ID3D12Resource: GUID = GUID::from_u128(0x696442be_a72e_4059_bc79_5b5c98040fad);
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
pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
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
