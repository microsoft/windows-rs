windows_core::imp::define_interface!(IInner, IInner_Vtbl);
#[repr(C)]
pub struct IInner_Vtbl {
    Value: usize,
}
windows_core::imp::define_interface!(IOuter, IOuter_Vtbl);
impl IOuter {
    pub unsafe fn GetInner(&self) -> Option<IInner> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInner)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IOuter_Vtbl {
    pub GetInner: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<IInner>,
}
