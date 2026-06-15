#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2);
    pub const Completed: Self = Self(1);
    pub const Error: Self = Self(3);
    pub const Started: Self = Self(0);
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
pub const IID_IAsyncAction: GUID = GUID::from_u128(0x5a648006_843a_4da9_865b_9d26e5dfad7b);
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: IInspectable_Vtbl,
    SetCompleted: usize,
    Completed: usize,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
}
pub const IID_IAsyncInfo: GUID = GUID::from_u128(0x00000036_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AsyncStatus) -> HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HRESULT) -> HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
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
pub const IID_IInspectable: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
#[repr(C)]
pub struct IInspectable_Vtbl {
    pub base: IUnknown_Vtbl,
    pub GetIids: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        count: *mut u32,
        values: *mut *mut GUID,
    ) -> HRESULT,
    pub GetRuntimeClassName: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        value: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub GetTrustLevel:
        unsafe extern "system" fn(this: *mut core::ffi::c_void, value: *mut i32) -> HRESULT,
}
