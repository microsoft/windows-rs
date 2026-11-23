windows_link::link!("fixture.dll" "C" fn Return() -> *mut core::ffi::c_void);
windows_link::link!("fixture.dll" "C" fn Take(widget : *mut core::ffi::c_void));
pub type Callback =
    Option<unsafe extern "C" fn(widget: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Functions {
    pub callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void>,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Holder {
    pub widgets: [*mut core::ffi::c_void; 2],
    pub interface: *mut *mut core::ffi::c_void,
    pub generic: *mut core::ffi::c_void,
    pub handler: *mut core::ffi::c_void,
}
impl Default for Holder {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IBase_Vtbl {
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
#[repr(C)]
pub struct IDerived_Vtbl {
    pub base__: IBase_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
