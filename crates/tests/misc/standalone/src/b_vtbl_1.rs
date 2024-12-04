#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

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
pub const IID_IDispatch: GUID = GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
    GetTypeInfo: usize,
    pub GetIDsOfNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *const PCWSTR,
        u32,
        u32,
        *mut i32,
    ) -> HRESULT,
    Invoke: usize,
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
