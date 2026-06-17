#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HRESULT = i32;
pub const IID_IAsyncAction: GUID = GUID {
    data1: 0x5a648006,
    data2: 0x843a,
    data3: 0x4da9,
    data4: [134, 91, 157, 38, 229, 223, 173, 123],
};
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: IInspectable_Vtbl,
    SetCompleted: usize,
    Completed: usize,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
}
pub const IID_IAsyncInfo: GUID = GUID {
    data1: 0x00000036,
    data2: 0x0000,
    data3: 0x0000,
    data4: [192, 0, 0, 0, 0, 0, 0, 70],
};
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
    Status: usize,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HRESULT) -> HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
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
pub const IID_IInspectable: GUID = GUID {
    data1: 0xaf86e2e0,
    data2: 0xb12d,
    data3: 0x4c6a,
    data4: [156, 90, 215, 170, 101, 16, 30, 144],
};
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
