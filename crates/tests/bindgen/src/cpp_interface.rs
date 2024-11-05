#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IPersist,
    IPersist_Vtbl,
    0x0000010c_0000_0000_c000_000000000046
);
impl IPersist {
    #[inline]
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> where {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassID)(
            windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
}
#[repr(C)]
pub struct IPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
}
