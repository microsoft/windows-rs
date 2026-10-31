#[inline]
pub unsafe fn Bool() -> windows_core::BOOL {
    windows_core::link!("test.dll" "system" fn Bool() -> windows_core::BOOL);
    unsafe { Bool() }
}
#[must_use]
#[inline]
pub unsafe fn Count() -> u32 {
    windows_core::link!("test.dll" "system" fn Count() -> u32);
    unsafe { Count() }
}
#[inline]
pub unsafe fn Hresult() -> windows_core::HRESULT {
    windows_core::link!("test.dll" "system" fn Hresult() -> windows_core::HRESULT);
    unsafe { Hresult() }
}
#[inline]
pub unsafe fn Status() -> windows_core::NTSTATUS {
    windows_core::link!("test.dll" "system" fn Status() -> windows_core::NTSTATUS);
    unsafe { Status() }
}
windows_core::imp::define_interface!(Value, Value_Vtbl, 0x10000000_0000_0000_0000_000000000001);
windows_core::imp::interface_hierarchy!(Value, windows_core::IUnknown);
impl Value {
    #[must_use]
    pub unsafe fn Count(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Get(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct Value_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait Value_Impl: windows_core::IUnknownImpl {
    fn Count(&self) -> u32;
    fn Get(&self) -> windows_core::Result<u32>;
}
impl Value_Vtbl {
    pub const fn new<Identity: Value_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: Value_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Value_Impl::Count(this)
            }
        }
        unsafe extern "system" fn Get<Identity: Value_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Value_Impl::Get(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Value as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for Value {}
