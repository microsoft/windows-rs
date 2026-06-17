#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HRESULT = i32;
pub const IID_IPersist: GUID = GUID {
    data1: 0x0000010c,
    data2: 0x0000,
    data3: 0x0000,
    data4: [192, 0, 0, 0, 0, 0, 0, 70],
};
#[repr(C)]
pub struct IPersist_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GUID) -> HRESULT,
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
