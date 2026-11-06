windows_core::imp::define_interface!(Interface, Interface_Vtbl);
impl Interface {
    pub unsafe fn OutFixed(&self, buffer: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutFixed)(
                windows_core::Interface::as_raw(self),
                buffer as _,
            )
        }
    }
    pub unsafe fn OutLen(&self, count: u32, buffer: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer as _,
            )
        }
    }
    pub unsafe fn OutOpt(&self, count: u32, buffer: Option<*mut u32>) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutOpt)(
                windows_core::Interface::as_raw(self),
                count,
                buffer.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn OutBytes(&self, size: u32, buffer: *mut u8) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutBytes)(
                windows_core::Interface::as_raw(self),
                size,
                buffer as _,
            )
        }
    }
    pub unsafe fn OutInterfaces(
        &self,
        count: u32,
        values: *mut Option<windows_core::IInspectable>,
    ) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).OutInterfaces)(
                windows_core::Interface::as_raw(self),
                count,
                values as _,
            )
        }
    }
    pub unsafe fn InOutLen(&self, count: u32, buffer: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InOutLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer as _,
            )
        }
    }
    pub unsafe fn SharedCount(&self, count: u32, input: *const u32, output: *mut u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).SharedCount)(
                windows_core::Interface::as_raw(self),
                count,
                input,
                output as _,
            )
        }
    }
    pub unsafe fn InLen(&self, count: u32, buffer: *const u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer,
            )
        }
    }
    pub unsafe fn InSignedLen(&self, count: i32, buffer: *const u32) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InSignedLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer,
            )
        }
    }
    pub unsafe fn InOptionalSignedLen(&self, count: i32, buffer: Option<*const u32>) -> i32 {
        unsafe {
            (windows_core::Interface::vtable(self).InOptionalSignedLen)(
                windows_core::Interface::as_raw(self),
                count,
                buffer.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub OutFixed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> i32,
    pub OutLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub OutOpt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub OutBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> i32,
    pub OutInterfaces:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> i32,
    pub InOutLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32,
    pub SharedCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut u32) -> i32,
    pub InLen: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> i32,
    pub InSignedLen: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u32) -> i32,
    pub InOptionalSignedLen:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u32) -> i32,
}
windows_core::imp::define_interface!(Object, Object_Vtbl);
#[repr(C)]
pub struct Object_Vtbl {}
