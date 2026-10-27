windows_core::imp::define_interface!(IMemoryBufferByteAccess, IMemoryBufferByteAccess_Vtbl, 0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d);
windows_core::imp::interface_hierarchy!(IMemoryBufferByteAccess, windows_core::IUnknown);
impl IMemoryBufferByteAccess {
    #[cfg(feature = "shared")]
    pub unsafe fn GetBuffer(&self, value: *mut *mut super::BYTE, capacity: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), value as _, capacity as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBuffer: usize,
}
#[cfg(feature = "shared")]
pub trait IMemoryBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, value: *mut *mut super::BYTE, capacity: *mut super::UINT32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMemoryBufferByteAccess_Vtbl {
    pub const fn new<Identity: IMemoryBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IMemoryBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::BYTE, capacity: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMemoryBufferByteAccess_Impl::GetBuffer(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&capacity)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBufferByteAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMemoryBufferByteAccess {}
