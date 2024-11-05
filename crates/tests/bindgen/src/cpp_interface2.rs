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
windows_core::imp::define_interface!(
    IPersistFile,
    IPersistFile_Vtbl,
    0x0000010b_0000_0000_c000_000000000046
);
impl IPersistFile {
    #[inline]
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT where {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
    #[inline]
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveCompleted)(
            windows_core::Interface::as_raw(self),
            pszfilename.param().abi(),
        )
        .ok()
    }
    #[inline]
    pub unsafe fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR> where {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurFile)(
            windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
}
#[repr(C)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Load: usize,
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
}
